use local_jev::mcp::McpServer;
use std::io::Cursor;

#[test]
fn test_mcp_initialize_and_tool_list() {
    let mcp = McpServer::new();
    let input = "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{}}\n\
                 {\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/list\",\"params\":{}}\n";

    let mut output = Vec::new();
    mcp.run_loop(Cursor::new(input), &mut output).unwrap();

    let output_str = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = output_str.trim().split('\n').collect();
    assert_eq!(lines.len(), 2);

    let init_res: serde_json::Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(init_res["result"]["serverInfo"]["name"], "local-jev");

    let tools_res: serde_json::Value = serde_json::from_str(lines[1]).unwrap();
    let tools = tools_res["result"]["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 4);
}

#[test]
fn test_mcp_call_choice_tool() {
    let mcp = McpServer::new();
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 10,
        "method": "tools/call",
        "params": {
            "name": "jev_choice",
            "arguments": {
                "state": "Compiler threw unhandled error in parser module",
                "question": "Which action should be selected?",
                "options": ["inspect parser.rs", "delete repository", "ping server"],
                "temperature": 0.1
            }
        }
    });

    let mut output = Vec::new();
    mcp.run_loop(Cursor::new(serde_json::to_string(&req).unwrap() + "\n"), &mut output).unwrap();

    let res: serde_json::Value = serde_json::from_slice(&output).unwrap();
    let content_text = res["result"]["content"][0]["text"].as_str().unwrap();
    let choice_payload: serde_json::Value = serde_json::from_str(content_text).unwrap();
    assert_eq!(choice_payload["choice"], "inspect parser.rs");
    assert!(choice_payload["confidence"].as_f64().unwrap() > 0.8);
}
