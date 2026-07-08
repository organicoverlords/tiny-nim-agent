#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ModelMessage {
    pub role: MessageRole,
    pub content: String,
}

impl ModelMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::System,
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ToolSchema {
    pub name: String,
    pub required_args: Vec<String>,
}

impl ToolSchema {
    pub fn new(name: impl Into<String>, required_args: Vec<String>) -> Self {
        Self {
            name: name.into(),
            required_args,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ToolCall {
    pub name: String,
    pub args_json: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NormalizedModelResponse {
    pub text: String,
    pub tool_calls: Vec<ToolCall>,
}

impl NormalizedModelResponse {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            tool_calls: Vec::new(),
        }
    }

    pub fn with_tool_call(call: ToolCall) -> Self {
        Self {
            text: String::new(),
            tool_calls: vec![call],
        }
    }

    pub fn has_tool_calls(&self) -> bool {
        !self.tool_calls.is_empty()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ContractError {
    EmptyResponse,
    UnknownTool { name: String },
    InvalidArguments { name: String },
    RequiredToolMissing,
}

pub fn validate_response(
    response: NormalizedModelResponse,
    tools: &[ToolSchema],
    require_tool_call: bool,
) -> Result<NormalizedModelResponse, ContractError> {
    if response.text.trim().is_empty() && response.tool_calls.is_empty() {
        return Err(ContractError::EmptyResponse);
    }

    if require_tool_call && response.tool_calls.is_empty() {
        return Err(ContractError::RequiredToolMissing);
    }

    for call in &response.tool_calls {
        let schema = tools
            .iter()
            .find(|tool| tool.name == call.name)
            .ok_or_else(|| ContractError::UnknownTool {
                name: call.name.clone(),
            })?;
        for required in &schema.required_args {
            let quoted = format!("\"{}\"", required);
            if !call.args_json.contains(&quoted) {
                return Err(ContractError::InvalidArguments {
                    name: call.name.clone(),
                });
            }
        }
    }

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_file_schema() -> ToolSchema {
        ToolSchema::new("read_file", vec!["path".to_string()])
    }

    #[test]
    fn accepts_text_when_tool_not_required() {
        let response = NormalizedModelResponse::text("hello");
        let validated = validate_response(response.clone(), &[], false).unwrap();
        assert_eq!(validated, response);
    }

    #[test]
    fn rejects_empty_response() {
        let err = validate_response(NormalizedModelResponse::text(""), &[], false).unwrap_err();
        assert_eq!(err, ContractError::EmptyResponse);
    }

    #[test]
    fn rejects_missing_required_tool_call() {
        let response = NormalizedModelResponse::text("I will inspect it.");
        let err = validate_response(response, &[read_file_schema()], true).unwrap_err();
        assert_eq!(err, ContractError::RequiredToolMissing);
    }

    #[test]
    fn accepts_known_tool_with_required_arg() {
        let response = NormalizedModelResponse::with_tool_call(ToolCall {
            name: "read_file".to_string(),
            args_json: "{\"path\":\"README.md\"}".to_string(),
        });
        assert!(validate_response(response, &[read_file_schema()], true).is_ok());
    }

    #[test]
    fn rejects_unknown_tool() {
        let response = NormalizedModelResponse::with_tool_call(ToolCall {
            name: "magic".to_string(),
            args_json: "{}".to_string(),
        });
        let err = validate_response(response, &[read_file_schema()], true).unwrap_err();
        assert_eq!(err, ContractError::UnknownTool { name: "magic".into() });
    }

    #[test]
    fn rejects_missing_required_argument() {
        let response = NormalizedModelResponse::with_tool_call(ToolCall {
            name: "read_file".to_string(),
            args_json: "{}".to_string(),
        });
        let err = validate_response(response, &[read_file_schema()], true).unwrap_err();
        assert_eq!(err, ContractError::InvalidArguments { name: "read_file".into() });
    }
}
