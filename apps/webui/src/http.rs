use crate::smoke_artifact::{artifact_response_json, write_smoke_proof_artifact};
use crate::smoke_events::{smoke_cards_html, smoke_event_stream};
use crate::smoke_report::{smoke_ledger_json, smoke_report_html};
use crate::{run_smoke_for_workspace, SmokeRunResponse};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HttpResponse {
    pub status: u16,
    pub content_type: &'static str,
    pub body: String,
}

pub fn handle_request_line(request_line: &str, workspace: &Path) -> HttpResponse {
    match request_line.split_whitespace().next() {
        Some("GET") | Some("POST") => route(request_line, workspace),
        _ => text_response(405, "method not allowed"),
    }
}

pub fn run_server(bind: &str, workspace: PathBuf) -> std::io::Result<()> {
    let listener = TcpListener::bind(bind)?;
    for stream in listener.incoming() {
        let stream = stream?;
        respond(stream, &workspace)?;
    }
    Ok(())
}

fn route(request_line: &str, workspace: &Path) -> HttpResponse {
    let path = request_line
        .split_whitespace()
        .nth(1)
        .unwrap_or("/");
    match path {
        "/" => html_response(index_html()),
        "/smoke/cards" => smoke_html(workspace, smoke_cards_html),
        "/smoke/report" => smoke_html(workspace, smoke_report_html),
        "/api/smoke/dry-run" => smoke_json_route(workspace, smoke_json),
        "/api/smoke/ledger" => smoke_json_route(workspace, smoke_ledger_json),
        "/api/smoke/proof-file" => smoke_proof_file_route(workspace),
        "/api/smoke/events" => match run_smoke_for_workspace(workspace) {
            Ok(response) => event_response(smoke_event_stream(&response)),
            Err(error) => error_event(error),
        },
        _ => text_response(404, "not found"),
    }
}

fn respond(mut stream: TcpStream, workspace: &Path) -> std::io::Result<()> {
    let mut buffer = [0_u8; 4096];
    let count = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..count]);
    let request_line = request.lines().next().unwrap_or("");
    let response = handle_request_line(request_line, workspace);
    let wire = format!(
        "HTTP/1.1 {} OK\r\ncontent-type: {}\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
        response.status,
        response.content_type,
        response.body.len(),
        response.body
    );
    stream.write_all(wire.as_bytes())
}

fn smoke_html(workspace: &Path, render: fn(&SmokeRunResponse) -> String) -> HttpResponse {
    match run_smoke_for_workspace(workspace) {
        Ok(response) => html_response(page_shell(&render(&response))),
        Err(error) => html_response(page_shell(&format!("<pre>{}</pre>", escape_json(&format!("{error:?}"))))),
    }
}

fn smoke_json_route(workspace: &Path, render: fn(&SmokeRunResponse) -> String) -> HttpResponse {
    match run_smoke_for_workspace(workspace) {
        Ok(response) => json_response(200, render(&response)),
        Err(error) => error_json(error),
    }
}

fn smoke_proof_file_route(workspace: &Path) -> HttpResponse {
    match run_smoke_for_workspace(workspace) {
        Ok(response) => match write_smoke_proof_artifact(workspace, &response) {
            Ok(artifact) => json_response(200, artifact_response_json(&artifact)),
            Err(error) => json_response(500, format!("{{\"ok\":false,\"error\":{}}}", quoted(&format!("{error:?}")))),
        },
        Err(error) => error_json(error),
    }
}

fn smoke_json(response: &SmokeRunResponse) -> String {
    let evidence = response
        .evidence
        .iter()
        .map(|item| quoted(item))
        .collect::<Vec<_>>()
        .join(",");
    let tool_results = response
        .tool_results
        .iter()
        .map(|result| {
            format!(
                "{{\"tool\":{},\"ok\":{}}}",
                quoted(&result.tool),
                result.ok
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"run_id\":{},\"state\":{},\"ok\":{},\"evidence\":[{}],\"tool_results\":[{}]}}",
        quoted(&response.run_id),
        quoted(&response.state),
        response.ok,
        evidence,
        tool_results
    )
}

fn index_html() -> String {
    page_shell(
        r#"<section class="chat-shell">
  <h1>tiny-nim-agent</h1>
  <p>Run the real smoke path and inspect proof as JSON, ledger, proof file, cards, report, or events.</p>
  <button id="run">Run smoke</button>
  <button id="proof">Write proof file</button>
  <p><a href="/smoke/cards">Open smoke tool cards</a></p>
  <p><a href="/smoke/report">Open verified smoke report</a></p>
  <pre id="out">idle</pre>
</section>
<script>
document.getElementById('run').onclick = async () => {
  const res = await fetch('/api/smoke/ledger', { method: 'POST' });
  document.getElementById('out').textContent = JSON.stringify(await res.json(), null, 2);
};
document.getElementById('proof').onclick = async () => {
  const res = await fetch('/api/smoke/proof-file', { method: 'POST' });
  document.getElementById('out').textContent = JSON.stringify(await res.json(), null, 2);
};
</script>"#,
    )
}

fn page_shell(body: &str) -> String {
    format!(
        r#"<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <title>tiny-nim-agent</title>
  <style>
    body {{ font-family: system-ui, sans-serif; max-width: 860px; margin: 40px auto; background: #111; color: #eee; }}
    a {{ color: #8ab4ff; }}
    button {{ padding: 10px 14px; border-radius: 10px; }}
    pre, .run-card {{ background: #1d1d1d; border: 1px solid #333; border-radius: 14px; padding: 16px; }}
    .tool-card {{ display: flex; justify-content: space-between; margin: 8px 0; padding: 10px; background: #242424; border-radius: 10px; }}
  </style>
</head>
<body>{body}</body>
</html>"#
    )
}

fn json_response(status: u16, body: String) -> HttpResponse {
    HttpResponse { status, content_type: "application/json", body }
}

fn event_response(body: String) -> HttpResponse {
    HttpResponse { status: 200, content_type: "text/event-stream", body }
}

fn html_response(body: String) -> HttpResponse {
    HttpResponse { status: 200, content_type: "text/html; charset=utf-8", body }
}

fn text_response(status: u16, body: &str) -> HttpResponse {
    HttpResponse { status, content_type: "text/plain; charset=utf-8", body: body.to_string() }
}

fn error_json(error: crate::WebUiError) -> HttpResponse {
    json_response(500, format!("{{\"ok\":false,\"error\":{}}}", quoted(&format!("{error:?}"))))
}

fn error_event(error: crate::WebUiError) -> HttpResponse {
    event_response(format!("event: error\ndata: {}\n\n", escape_json(&format!("{error:?}"))))
}

fn quoted(value: &str) -> String {
    format!("\"{}\"", escape_json(value))
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn git_workspace() -> PathBuf {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("tiny-webui-http-{nonce}"));
        fs::create_dir_all(&root).unwrap();
        let status = Command::new("git").args(["init"]).current_dir(&root).status().unwrap();
        assert!(status.success());
        root
    }

    #[test]
    fn index_page_exposes_smoke_button() {
        let response = handle_request_line("GET / HTTP/1.1", Path::new("."));
        assert_eq!(response.status, 200);
        assert!(response.body.contains("Run smoke"));
        assert!(response.body.contains("/api/smoke/ledger"));
        assert!(response.body.contains("/api/smoke/proof-file"));
        assert!(response.body.contains("/smoke/cards"));
        assert!(response.body.contains("/smoke/report"));
    }

    #[test]
    fn smoke_endpoint_returns_real_proof_json() {
        let root = git_workspace();
        let response = handle_request_line("POST /api/smoke/dry-run HTTP/1.1", &root);
        assert_eq!(response.status, 200);
        assert!(response.body.contains("\"ok\":true"));
        assert!(response.body.contains("\"state\":\"final\""));
        assert!(response.body.contains("file_written"));
        assert!(response.body.contains("git_status_checked"));
        assert!(!root.join("agent-smoke.txt").exists());
    }

    #[test]
    fn smoke_ledger_route_returns_verified_final_answer() {
        let root = git_workspace();
        let response = handle_request_line("POST /api/smoke/ledger HTTP/1.1", &root);
        assert_eq!(response.status, 200);
        assert!(response.body.contains("\"verified\":true"));
        assert!(response.body.contains("\"final_answer\""));
        assert!(response.body.contains("created agent-smoke.txt"));
        assert!(!root.join("agent-smoke.txt").exists());
    }

    #[test]
    fn smoke_proof_file_route_writes_run_artifact() {
        let root = git_workspace();
        let response = handle_request_line("POST /api/smoke/proof-file HTTP/1.1", &root);
        let path = root.join(".tiny-nim-agent/proofs/dry-run-first-smoke.json");
        let proof = fs::read_to_string(path).unwrap();

        assert_eq!(response.status, 200);
        assert!(response.body.contains("\"proof_path\""));
        assert!(response.body.contains("\"proof_json\""));
        assert!(proof.contains("\"verified\":true"));
        assert!(proof.contains("\"run_id\":\"dry-run-first-smoke\""));
        assert!(!root.join("agent-smoke.txt").exists());
    }

    #[test]
    fn smoke_report_page_renders_verified_report() {
        let root = git_workspace();
        let response = handle_request_line("GET /smoke/report HTTP/1.1", &root);
        assert_eq!(response.status, 200);
        assert!(response.body.contains("Final report"));
        assert!(response.body.contains("Verified"));
        assert!(response.body.contains("agent-smoke.txt"));
    }

    #[test]
    fn smoke_events_endpoint_returns_event_stream() {
        let root = git_workspace();
        let response = handle_request_line("GET /api/smoke/events HTTP/1.1", &root);
        assert_eq!(response.status, 200);
        assert_eq!(response.content_type, "text/event-stream");
        assert!(response.body.contains("event: tool_result"));
        assert!(response.body.contains("event: final_state"));
    }

    #[test]
    fn smoke_cards_page_renders_tool_cards() {
        let root = git_workspace();
        let response = handle_request_line("GET /smoke/cards HTTP/1.1", &root);
        assert_eq!(response.status, 200);
        assert!(response.body.contains("class=\"tool-card\""));
        assert!(response.body.contains("git_status"));
        assert!(!root.join("agent-smoke.txt").exists());
    }
}
