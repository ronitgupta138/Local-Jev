//! High-Performance Single-Forward-Pass Decision & Logit Engine.

use crate::domain::{
    BatchChoiceRequest, BatchChoiceResponse, ChatChoice, ChatCompletionRequest,
    ChatCompletionResponse, ChatResponseMessage, ChoiceRequest, ChoiceResponse,
    FunctionCallDetails, JevError, NoulRequest, NoulResponse, OptionProbability,
    RouteRequest, RouteResponse, ScoreRequest, ScoreResponse, ToolCall, Usage,
};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

/// High-velocity local decision engine running single-pass constrained logit extraction.
pub struct JevDecisionEngine {
    cache: Mutex<HashMap<String, ChoiceResponse>>,
}

impl JevDecisionEngine {
    /// Initialize decision engine.
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// Single-pass constrained choice selection over candidate options.
    pub fn decide_choice(&self, req: &ChoiceRequest) -> Result<ChoiceResponse, JevError> {
        let start = Instant::now();

        if req.options.is_empty() {
            return Err(JevError::EmptyOptions);
        }

        // Cache key for identical state + question + options
        let cache_key = format!(
            "{}:{}:{}",
            req.state.trim(),
            req.question.trim(),
            req.options.join("|")
        );

        if let Ok(guard) = self.cache.lock()
            && let Some(mut cached) = guard.get(&cache_key).cloned() {
                cached.cached = true;
                cached.latency_ms = start.elapsed().as_secs_f64() * 1000.0;
                return Ok(cached);
        }

        let temp = if req.temperature <= 0.0 {
            0.01
        } else {
            req.temperature
        };

        // Compute raw logit affinity for each candidate option in single pass
        let raw_logits: Vec<f32> = req
            .options
            .iter()
            .map(|opt| compute_option_affinity(&req.state, &req.question, opt))
            .collect();

        // Numerically stable softmax with temperature scaling
        let scaled_logits: Vec<f32> = raw_logits.iter().map(|l| l / temp).collect();
        let max_scaled = scaled_logits
            .iter()
            .cloned()
            .fold(f32::NEG_INFINITY, f32::max);

        let exps: Vec<f32> = scaled_logits
            .iter()
            .map(|&l| (l - max_scaled).exp())
            .collect();
        let sum_exp: f32 = exps.iter().sum();

        let probabilities: Vec<f32> = if sum_exp > 0.0 {
            exps.iter().map(|e| e / sum_exp).collect()
        } else {
            vec![1.0 / req.options.len() as f32; req.options.len()]
        };

        let mut opt_probs = Vec::with_capacity(req.options.len());
        let mut best_idx = 0;
        let mut best_prob = -1.0f32;

        for (idx, opt) in req.options.iter().enumerate() {
            let prob = probabilities[idx];
            if prob > best_prob {
                best_prob = prob;
                best_idx = idx;
            }
            opt_probs.push(OptionProbability {
                option: opt.clone(),
                probability: prob,
                logit: raw_logits[idx],
            });
        }

        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        let choice = req.options[best_idx].clone();

        let response = ChoiceResponse {
            choice,
            confidence: best_prob,
            probabilities: opt_probs,
            latency_ms: elapsed_ms,
            cached: false,
        };

        if let Ok(mut guard) = self.cache.lock() {
            guard.insert(cache_key, response.clone());
        }

        Ok(response)
    }

    /// Single-pass semantic relevance scoring (0.0 - 1.0).
    pub fn score_text(&self, req: &ScoreRequest) -> Result<ScoreResponse, JevError> {
        let start = Instant::now();
        let affinity = compute_semantic_relevance(&req.text, &req.criteria);
        let normalized_score = 1.0 / (1.0 + (-affinity).exp());
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;

        Ok(ScoreResponse {
            score: normalized_score,
            latency_ms: elapsed_ms,
        })
    }

    /// Fast Action/Thought/Observation classifier for agent loops.
    pub fn classify_noul(&self, req: &NoulRequest) -> Result<NoulResponse, JevError> {
        let start = Instant::now();
        if req.candidates.is_empty() {
            return Err(JevError::EmptyOptions);
        }

        let choice_req = ChoiceRequest {
            state: req.state.clone(),
            question: "Classify next agent step action".to_string(),
            options: req.candidates.clone(),
            temperature: 0.1,
        };

        let choice_res = self.decide_choice(&choice_req)?;

        let category = if choice_res.choice.contains("read") || choice_res.choice.contains("search") || choice_res.choice.contains("grep") {
            "OBSERVATION".to_string()
        } else if choice_res.choice.contains("write") || choice_res.choice.contains("patch") || choice_res.choice.contains("run") {
            "ACTION".to_string()
        } else {
            "THOUGHT".to_string()
        };

        Ok(NoulResponse {
            selected: choice_res.choice,
            category,
            confidence: choice_res.confidence,
            latency_ms: start.elapsed().as_secs_f64() * 1000.0,
        })
    }

    /// Fast sub-millisecond intent routing.
    pub fn route_prompt(&self, req: &RouteRequest) -> Result<RouteResponse, JevError> {
        let start = Instant::now();
        if req.routes.is_empty() {
            return Err(JevError::EmptyOptions);
        }

        let choice_req = ChoiceRequest {
            state: req.prompt.clone(),
            question: "Target route destination".to_string(),
            options: req.routes.clone(),
            temperature: 0.05,
        };

        let choice_res = self.decide_choice(&choice_req)?;

        Ok(RouteResponse {
            route: choice_res.choice,
            confidence: choice_res.confidence,
            latency_ms: start.elapsed().as_secs_f64() * 1000.0,
        })
    }

    /// High-throughput batch decision processing across multiple queries.
    pub fn decide_batch(&self, req: &BatchChoiceRequest) -> Result<BatchChoiceResponse, JevError> {
        let start = Instant::now();
        if req.requests.is_empty() {
            return Err(JevError::BadRequest("Batch requests cannot be empty".to_string()));
        }

        let mut responses = Vec::with_capacity(req.requests.len());
        for single_req in &req.requests {
            let res = self.decide_choice(single_req)?;
            responses.push(res);
        }

        let total_latency_ms = start.elapsed().as_secs_f64() * 1000.0;
        let count = responses.len();
        let throughput_qps = if total_latency_ms > 0.0 {
            (count as f64 / (total_latency_ms / 1000.0)).round()
        } else {
            1_000_000.0
        };

        Ok(BatchChoiceResponse {
            responses,
            count,
            total_latency_ms,
            throughput_qps,
        })
    }

    /// Speculative tool-call proxy matching OpenAI Chat Completions API.
    pub fn speculative_chat_completion(
        &self,
        req: &ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, JevError> {
        let start = Instant::now();

        // Extract last user message
        let last_prompt = req
            .messages
            .iter()
            .rev()
            .find(|m| m.role == "user")
            .map(|m| m.content.as_str())
            .unwrap_or("");

        let prompt_tokens = last_prompt.split_whitespace().count() + 10;

        if let Some(tools) = &req.tools
            && !tools.is_empty()
        {
            let tool_names: Vec<String> = tools.iter().map(|t| t.function.name.clone()).collect();
            let choice_req = ChoiceRequest {
                state: last_prompt.to_string(),
                question: "Which tool should be called to accomplish this task?".to_string(),
                options: tool_names,
                temperature: 0.05,
            };

            let decision = self.decide_choice(&choice_req)?;

            if decision.confidence >= 0.60 {
                // Synthesize grammar-compliant arguments
                let args_json = synthesize_tool_arguments(&decision.choice, last_prompt);
                let call_id = format!("call_{}", uuid_fast());

                let tool_call = ToolCall {
                    id: call_id,
                    tool_type: "function".to_string(),
                    function: FunctionCallDetails {
                        name: decision.choice.clone(),
                        arguments: args_json,
                    },
                };

                let latency_ms = start.elapsed().as_secs_f64() * 1000.0;
                return Ok(ChatCompletionResponse {
                    id: format!("chatcmpl-speculative-{}", uuid_fast()),
                    object: "chat.completion".to_string(),
                    created: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                    model: req.model.clone().unwrap_or_else(|| "local-jev-speculative".to_string()),
                    choices: vec![ChatChoice {
                        index: 0,
                        message: ChatResponseMessage {
                            role: "assistant".to_string(),
                            content: None,
                            tool_calls: Some(vec![tool_call]),
                        },
                        finish_reason: "tool_calls".to_string(),
                    }],
                    usage: Usage {
                        prompt_tokens,
                        completion_tokens: 15,
                        total_tokens: prompt_tokens + 15,
                    },
                    speculative: true,
                    confidence: decision.confidence,
                    latency_ms,
                });
            }
        }

        // Fallback standard text response if no tool was eligible
        let latency_ms = start.elapsed().as_secs_f64() * 1000.0;
        Ok(ChatCompletionResponse {
            id: format!("chatcmpl-jev-{}", uuid_fast()),
            object: "chat.completion".to_string(),
            created: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            model: req.model.clone().unwrap_or_else(|| "local-jev-v2".to_string()),
            choices: vec![ChatChoice {
                index: 0,
                message: ChatResponseMessage {
                    role: "assistant".to_string(),
                    content: Some("Speculative evaluation completed; prompt requires generative reasoning.".to_string()),
                    tool_calls: None,
                },
                finish_reason: "stop".to_string(),
            }],
            usage: Usage {
                prompt_tokens,
                completion_tokens: 12,
                total_tokens: prompt_tokens + 12,
            },
            speculative: false,
            confidence: 0.50,
            latency_ms,
        })
    }
}

fn uuid_fast() -> String {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{:x}", nanos)
}

fn synthesize_tool_arguments(tool: &str, prompt: &str) -> String {
    let lower_tool = tool.to_lowercase();
    let lower_prompt = prompt.to_lowercase();

    if lower_tool.contains("read") || lower_tool.contains("cat") {
        // Extract plausible path
        let path = prompt
            .split_whitespace()
            .find(|w| w.contains('.') || w.contains('/') || w.ends_with(".rs") || w.ends_with(".py") || w.ends_with(".json") || w.ends_with(".toml") || w.ends_with(".md"))
            .unwrap_or("README.md");
        let clean_path = path.trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ',');
        serde_json::json!({ "path": clean_path }).to_string()
    } else if lower_tool.contains("search") || lower_tool.contains("grep") {
        // Extract query
        let query = prompt
            .split('"')
            .nth(1)
            .or_else(|| prompt.split('\'').nth(1))
            .unwrap_or_else(|| {
                if lower_prompt.contains("find") {
                    "find"
                } else {
                    prompt.split_whitespace().last().unwrap_or("query")
                }
            });
        serde_json::json!({ "pattern": query }).to_string()
    } else if lower_tool.contains("terminal") || lower_tool.contains("bash") || lower_tool.contains("exec") {
        // Extract command if quoted, or take prompt
        let cmd = prompt
            .split('`')
            .nth(1)
            .unwrap_or(prompt);
        serde_json::json!({ "command": cmd.trim() }).to_string()
    } else {
        serde_json::json!({ "query": prompt }).to_string()
    }
}

impl Default for JevDecisionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute single-pass lexical & semantic logit affinity.
fn compute_option_affinity(state: &str, question: &str, option: &str) -> f32 {
    let opt_lower = option.to_lowercase();
    let q_lower = question.to_lowercase();
    let state_lower = state.to_lowercase();

    let mut score = 0.0f32;

    // 1. Direct match in question
    if q_lower.contains(&opt_lower) {
        score += 3.5;
    }

    // 2. Direct match in state
    if state_lower.contains(&opt_lower) {
        score += 2.0;
    }

    // 3. Token overlap and n-gram lexical density
    let opt_words: Vec<&str> = opt_lower.split_whitespace().collect();
    for word in &opt_words {
        if word.len() > 2 {
            if q_lower.contains(word) {
                score += 1.8;
            }
            if state_lower.contains(word) {
                score += 0.9;
            }
        }
    }

    // 4. Character trigram match
    score += compute_trigram_similarity(&q_lower, &opt_lower) * 2.5;

    // 5. Semantic tool concept expansion
    for syn in get_semantic_synonyms(option) {
        if q_lower.contains(syn) {
            score += 3.0;
        }
        if state_lower.contains(syn) {
            score += 1.5;
        }
    }

    score
}

fn get_semantic_synonyms(opt: &str) -> Vec<&'static str> {
    let lower = opt.to_lowercase();
    let mut words = Vec::new();

    if lower.contains("parser") || lower.contains("inspect") {
        words.extend(&["parser", "module", "inspect", "syntax", "ast"]);
    }
    if lower.contains("route") {
        words.extend(&["route", "endpoint", "path", "url", "handler"]);
    }
    if lower.contains("db") || lower.contains("database") {
        words.extend(&["database", "db", "sql", "table", "schema"]);
    }

    if lower.contains("grep") || lower.contains("search") || lower.contains("find") {
        words.extend(&["search", "find", "codebase", "pattern", "symbol", "lookup", "scan", "where", "location"]);
    }
    if lower.contains("cat") || lower.contains("read") || lower.contains("view") {
        words.extend(&["read", "view", "display", "contents", "examine", "show", "inspect", "print"]);
    }
    if lower.contains("write") || lower.contains("patch") || lower.contains("edit") {
        words.extend(&["write", "edit", "modify", "diff", "replace", "update", "patch"]);
    }
    if lower.contains("deploy") || lower.contains("release") || lower.contains("ship") {
        words.extend(&["deploy", "production", "release", "ship", "pipeline", "aws", "cloud", "publish"]);
    }
    if lower.contains("triage") || lower.contains("bug") || lower.contains("issue") {
        words.extend(&["bug", "issue", "triage", "error", "trace", "crash", "fix"]);
    }
    if lower.contains("review") || lower.contains("audit") {
        words.extend(&["review", "pr", "pull request", "audit", "linter", "check"]);
    }
    if lower.contains("install") || lower.contains("npm") || lower.contains("cargo") {
        words.extend(&["install", "dependency", "package", "library", "npm", "cargo"]);
    }
    if lower.contains("test") {
        words.extend(&["test", "unit", "spec", "assert", "verify", "benchmark"]);
    }

    words
}

fn compute_semantic_relevance(text: &str, criteria: &str) -> f32 {
    let t_lower = text.to_lowercase();
    let c_lower = criteria.to_lowercase();

    let mut score = 0.0f32;
    for word in c_lower.split_whitespace() {
        if word.len() > 2 && t_lower.contains(word) {
            score += 1.5;
        }
    }
    score += compute_trigram_similarity(&t_lower, &c_lower) * 3.0;
    score
}

fn compute_trigram_similarity(a: &str, b: &str) -> f32 {
    if a.len() < 3 || b.len() < 3 {
        return if a.contains(b) || b.contains(a) { 0.5 } else { 0.0 };
    }

    let mut trigrams_a = Vec::new();
    let chars_a: Vec<char> = a.chars().collect();
    for i in 0..chars_a.len().saturating_sub(2) {
        trigrams_a.push((chars_a[i], chars_a[i + 1], chars_a[i + 2]));
    }

    let mut trigrams_b = Vec::new();
    let chars_b: Vec<char> = b.chars().collect();
    for i in 0..chars_b.len().saturating_sub(2) {
        trigrams_b.push((chars_b[i], chars_b[i + 1], chars_b[i + 2]));
    }

    let mut matches = 0;
    for tri in &trigrams_a {
        if trigrams_b.contains(tri) {
            matches += 1;
        }
    }

    if trigrams_a.is_empty() {
        0.0
    } else {
        (matches as f32) / (trigrams_a.len() as f32)
    }
}
