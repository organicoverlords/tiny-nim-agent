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
        "/api/smoke/dry-run" => match run_smoke_for_workspace(workspace) {
            Ok(response) => json_response(200, smoke_json(&response)),
            Err(error) => json_response(500, format!("{{\"ok\":false,\"error\":\"{}\"}}", escape_json(&format!("{error:?}")))),
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
    r#"<!doctype html>
<html>
<head><meta charset="utf-8"><title>tiny-nim-agent</title></head>
<body>
<main>
  <h1>tiny-nim-agent</h1>
  <p>Dry-run smoke calls the real agent loop and returns proof data.</p>
  <button id="run">Run smoke</button>
  <pre id="out">idle</pre>
</main>
<script>
document.getElementById('run').onclick = async () => {
  const res = await fetch('/api/smoke/dry-run', { method: 'POST' });
  document.getElementById('out').textContent = JSON.stringify(await res.json(), null, 2);
};
</script>
</body>
</html>"#.to_string()
}

fn json_response(status: u16, body: String) -> HttpResponse {
    HttpResponse { status, content_type: "application/json", body }
}

fn html_response(body: String) -> HttpResponse {
    HttpResponse { status: 200, content_type: "text/html; charset=utf-8", body }
}

fn text_response(status: u16, body: &str) -> HttpResponse {
    HttpResponse { status, content_type: "text/plain; charset=utf-8", body: body.to_string() }
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
        assert!(response.body.contains("/api/smoke/dry-run"));
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
}
