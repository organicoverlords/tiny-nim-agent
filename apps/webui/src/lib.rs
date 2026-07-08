#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WebUiTarget {
    pub ui_style_reference: &'static str,
    pub agent_reference: &'static str,
}

impl WebUiTarget {
    pub fn chatgpt_with_opencode_loop() -> Self {
        Self {
            ui_style_reference: "ChatGPT-like chat surface",
            agent_reference: "OpenCode-like coding loop",
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "{} with {}",
            self.ui_style_reference, self.agent_reference
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_names_reference_behavior() {
        let target = WebUiTarget::chatgpt_with_opencode_loop();
        assert!(target.summary().contains("ChatGPT-like"));
        assert!(target.summary().contains("OpenCode-like"));
    }
}
