//! Native Model Context Protocol (MCP) JSON-RPC 2.0 stdio server for local-jev.

use crate::domain::{ChoiceRequest, JevError, NoulRequest, RouteRequest, ScoreRequest};
use crate::engine::JevDecisionEngine;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{BufRead, Write};

#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: &'static str,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

pub struct McpServer {
    engine: JevDecisionEngine,
}

impl McpServer {
    pub fn new() -> Self {
        Self {
            engine: JevDecisionEngine::new(),
        }
    }

    pub fn run_loop<R: BufRead, W: Write>(&self, reader: R, mut writer: W) -> Result<(), JevError> {
        for line_res in reader.lines() {
            let line = line_res?;
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(trimmed) {
                let id = req.id.clone().unwrap_or(Value::Null);
                let is_notification = req.id.is_none();
                let resp_val = self.handle_request(&req);

                if !is_notification {
                    let resp = match resp_val {
                        Ok(res) => JsonRpcResponse {
                            jsonrpc: "2.0",
                            id,
                            result: Some(res),
                            error: None,
                        },
                        Err(err) => JsonRpcResponse {
                            jsonrpc: "2.0",
                            id,
                            result: None,
                            error: Some(json!({
                                "code": -32000,
                                "message": err.to_string()
                            })),
                        },
                    };

                    let out = serde_json::to_string(&resp)?;
                    writer.write_all(out.as_bytes())?;
                    writer.write_all(b"\n")?;
                    writer.flush()?;
                }
            }
        }
        Ok(())
    }

    pub fn handle_request(&self, req: &JsonRpcRequest) -> Result<Value, JevError> {
        match req.method.as_str() {
            "initialize" => Ok(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": { "tools": {} },
                "serverInfo": {
                    "name": "local-jev",
                    "version": "0.2.0"
                }
            })),

            "notifications/initialized" => Ok(json!({})),
            "ping" => Ok(json!({})),

            "tools/list" => Ok(json!({
                "tools": self.list_tools()
            })),

            "tools/call" => {
                let name = req.params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let args = req.params.get("arguments").cloned().unwrap_or(json!({}));
                let output = self.call_tool(name, &args)?;
                Ok(json!({
                    "content": [
                        {
                            "type": "text",
                            "text": serde_json::to_string_pretty(&output)?
                        }
                    ]
                }))
            }

            other => Err(JevError::BadRequest(format!("Unsupported method: {}", other))),
        }
    }

    fn list_tools(&self) -> Vec<Value> {
        vec![
            json!({
                "name": "jev_choice",
                "description": "Select the best option from candidates with single-pass logit probabilities (<18ms).",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "state": { "type": "string", "description": "Contextual state or prompt" },
                        "question": { "type": "string", "description": "Decision question" },
                        "options": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Candidate options"
                        },
                        "temperature": { "type": "number", "description": "Temperature scaling (default: 0.1)" }
                    },
                    "required": ["state", "question", "options"]
                }
            }),
            json!({
                "name": "jev_score",
                "description": "Compute semantic relevance score (0.0 to 1.0) against specified criteria.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "text": { "type": "string", "description": "Text to evaluate" },
                        "criteria": { "type": "string", "description": "Criteria to score against" }
                    },
                    "required": ["text", "criteria"]
                }
            }),
            json!({
                "name": "jev_noul",
                "description": "Classify next agent step into ACTION, OBSERVATION, or THOUGHT.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "state": { "type": "string", "description": "Current agent context" },
                        "candidates": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Candidate step actions"
                        }
                    },
                    "required": ["state", "candidates"]
                }
            }),
            json!({
                "name": "jev_route",
                "description": "Route prompt to target destination route with calibrated confidence.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "prompt": { "type": "string", "description": "User prompt" },
                        "routes": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Candidate routes"
                        }
                    },
                    "required": ["prompt", "routes"]
                }
            })
        ]
    }

    pub fn call_tool(&self, name: &str, args: &Value) -> Result<Value, JevError> {
        match name {
            "jev_choice" => {
                let state = args.get("state").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let question = args.get("question").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let options = args
                    .get("options")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|x| x.as_str()).map(|s| s.to_string()).collect())
                    .unwrap_or_default();
                let temperature = args.get("temperature").and_then(|v| v.as_f64()).map(|f| f as f32).unwrap_or(0.1);

                let req = ChoiceRequest {
                    state,
                    question,
                    options,
                    temperature,
                };
                let res = self.engine.decide_choice(&req)?;
                Ok(serde_json::to_value(&res)?)
            }

            "jev_score" => {
                let text = args.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let criteria = args.get("criteria").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let req = ScoreRequest { text, criteria };
                let res = self.engine.score_text(&req)?;
                Ok(serde_json::to_value(&res)?)
            }

            "jev_noul" => {
                let state = args.get("state").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let candidates = args
                    .get("candidates")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|x| x.as_str()).map(|s| s.to_string()).collect())
                    .unwrap_or_default();
                let req = NoulRequest { state, candidates };
                let res = self.engine.classify_noul(&req)?;
                Ok(serde_json::to_value(&res)?)
            }

            "jev_route" => {
                let prompt = args.get("prompt").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let routes = args
                    .get("routes")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|x| x.as_str()).map(|s| s.to_string()).collect())
                    .unwrap_or_default();
                let req = RouteRequest { prompt, routes };
                let res = self.engine.route_prompt(&req)?;
                Ok(serde_json::to_value(&res)?)
            }

            unknown => Err(JevError::BadRequest(format!("Unknown tool: {}", unknown))),
        }
    }
}

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}
