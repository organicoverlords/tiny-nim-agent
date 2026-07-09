use crate::SmokeRunResponse;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SmokeEvent {
    pub kind: &'static str,
    pub data: String,
}

pub fn smoke_events(response: &SmokeRunResponse) -> Vec<SmokeEvent> {
    let mut events = Vec::new();
    events.push(SmokeEvent {
        kind: "run_started",
        data: format!("run_id={}", response.run_id),
    });
    for result in &response.tool_results {
        events.push(SmokeEvent {
            kind: "tool_result",
            data: format!("tool={} ok={}", result.tool, result.ok),
        });
    }
    for key in &response.evidence {
        events.push(SmokeEvent {
            kind: "evidence",
            data: key.clone(),
        });
    }
    events.push(SmokeEvent {
        kind: "final_state",
        data: format!("state={} ok={}", response.state, response.ok),
    });
    events
}

pub fn smoke_event_stream(response: &SmokeRunResponse) -> String {
    smoke_events(response)
        .into_iter()
        .map(|event| format!("event: {}\ndata: {}\n\n", event.kind, clean_sse_data(&event.data)))
        .collect::<String>()
}

pub fn smoke_cards_html(response: &SmokeRunResponse) -> String {
    let tools = response
        .tool_results
        .iter()
        .map(|result| {
            format!(
                "<li class=\"tool-card\"><strong>{}</strong><span>{}</span></li>",
                escape_html(&result.tool),
                if result.ok { "ok" } else { "failed" }
            )
        })
        .collect::<String>();
    let evidence = response
        .evidence
        .iter()
        .map(|key| format!("<li>{}</li>", escape_html(key)))
        .collect::<String>();
    format!(
        "<section class=\"run-card\"><h2>Run {}</h2><p>State: {} · ok: {}</p><h3>Tools</h3><ul>{}</ul><h3>Evidence</h3><ul>{}</ul></section>",
        escape_html(&response.run_id),
        escape_html(&response.state),
        response.ok,
        tools,
        evidence
    )
}

fn clean_sse_data(value: &str) -> String {
    value.replace('\n', " ").replace('\r', " ")
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SmokeRunResponse, ToolResultView};

    fn response() -> SmokeRunResponse {
        SmokeRunResponse {
            run_id: "run-1".to_string(),
            state: "final".to_string(),
            ok: true,
            evidence: vec!["file_written".to_string(), "file_deleted".to_string()],
            tool_results: vec![
                ToolResultView { tool: "write_file".to_string(), ok: true },
                ToolResultView { tool: "delete_file".to_string(), ok: true },
            ],
        }
    }

    #[test]
    fn event_stream_contains_ordered_events() {
        let stream = smoke_event_stream(&response());
        assert!(stream.contains("event: run_started"));
        assert!(stream.contains("event: tool_result"));
        assert!(stream.contains("data: tool=write_file ok=true"));
        assert!(stream.contains("event: final_state"));
    }

    #[test]
    fn cards_html_contains_tools_and_evidence() {
        let html = smoke_cards_html(&response());
        assert!(html.contains("class=\"tool-card\""));
        assert!(html.contains("write_file"));
        assert!(html.contains("file_deleted"));
    }
}
