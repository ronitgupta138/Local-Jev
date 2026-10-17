use local_jev::domain::{
    ChatCompletionRequest, ChatMessage, FunctionDefinition, ToolDefinition,
};
use local_jev::engine::JevDecisionEngine;

#[test]
fn test_speculative_proxy_tool_interception() {
    let engine = JevDecisionEngine::new();

    let tools = vec![
        ToolDefinition {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "read_file".to_string(),
                description: Some("Read file contents".to_string()),
                parameters: None,
            },
        },
        ToolDefinition {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "terminal".to_string(),
                description: Some("Execute terminal command".to_string()),
                parameters: None,
            },
        },
        ToolDefinition {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "search_files".to_string(),
                description: Some("Search files".to_string()),
                parameters: None,
            },
        },
    ];

    let req = ChatCompletionRequest {
        model: Some("claude-3-5-sonnet".to_string()),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are an agentic coding assistant.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: "Please read the file src/main.rs to verify entrypoint".to_string(),
            },
        ],
        tools: Some(tools),
        temperature: Some(0.0),
    };

    let res = engine
        .speculative_chat_completion(&req)
        .expect("Speculative completion should succeed");

    assert!(res.speculative);
    assert!(res.confidence >= 0.60);
    assert_eq!(res.choices[0].finish_reason, "tool_calls");

    let tool_calls = res.choices[0]
        .message
        .tool_calls
        .as_ref()
        .expect("Tool calls must exist");
    assert_eq!(tool_calls.len(), 1);
    assert_eq!(tool_calls[0].function.name, "read_file");

    let args: serde_json::Value =
        serde_json::from_str(&tool_calls[0].function.arguments).expect("Arguments must be valid JSON");
    assert_eq!(args["path"], "src/main.rs");
}

#[test]
fn test_speculative_proxy_fallback_without_tools() {
    let engine = JevDecisionEngine::new();

    let req = ChatCompletionRequest {
        model: Some("gpt-4o".to_string()),
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: "What is the meaning of existence?".to_string(),
        }],
        tools: None,
        temperature: Some(0.7),
    };

    let res = engine
        .speculative_chat_completion(&req)
        .expect("Completion should succeed");

    assert!(!res.speculative);
    assert_eq!(res.choices[0].finish_reason, "stop");
    assert!(res.choices[0].message.tool_calls.is_none());
    assert!(res.choices[0].message.content.is_some());
}
