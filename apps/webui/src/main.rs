use std::path::PathBuf;
use tiny_webui::http::run_server;

fn main() -> std::io::Result<()> {
    let bind = std::env::var("TINY_WEBUI_BIND").unwrap_or_else(|_| "127.0.0.1:8787".to_string());
    let workspace = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or(std::env::current_dir()?);
    println!("tiny-webui listening on {bind}");
    println!("workspace: {}", workspace.display());
    run_server(&bind, workspace)
}
