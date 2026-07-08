use std::collections::HashMap;
use std::fmt;

const DEFAULT_NIM_BASE_URL: &str = "https://integrate.api.nvidia.com/v1";
const DEFAULT_NIM_MODEL: &str = "openai/gpt-oss-120b";

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ModelId(String);

impl ModelId {
    pub fn new(value: impl Into<String>) -> Result<Self, ConfigError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ConfigError::EmptyModel);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct NimApiKey {
    value: String,
}

impl NimApiKey {
    pub fn new(value: impl Into<String>) -> Result<Self, ConfigError> {
        let value = value.into();
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ConfigError::MissingNimKey);
        }
        Ok(Self {
            value: trimmed.to_string(),
        })
    }

    pub fn redacted(&self) -> String {
        let chars: Vec<char> = self.value.chars().collect();
        let start = chars.len().saturating_sub(4);
        let last_four: String = chars[start..].iter().collect();
        format!("***{} ({} chars)", last_four, chars.len())
    }

    pub fn expose_for_http_client(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for NimApiKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NimApiKey")
            .field("redacted", &self.redacted())
            .finish()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NimConfig {
    pub api_key: NimApiKey,
    pub base_url: String,
    pub model_order: Vec<ModelId>,
}

impl NimConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_pairs(std::env::vars())
    }

    pub fn from_pairs<I, K, V>(pairs: I) -> Result<Self, ConfigError>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let mut values = HashMap::new();
        for (key, value) in pairs {
            values.insert(key.into(), value.into());
        }

        let api_key = values
            .get("NIM_KEY")
            .ok_or(ConfigError::MissingNimKey)
            .and_then(|value| NimApiKey::new(value.clone()))?;

        let base_url = values
            .get("NIM_BASE_URL")
            .cloned()
            .unwrap_or_else(|| DEFAULT_NIM_BASE_URL.to_string());

        let raw_models = values
            .get("NIM_MODEL_ORDER")
            .or_else(|| values.get("NIM_MODEL"))
            .cloned()
            .unwrap_or_else(|| DEFAULT_NIM_MODEL.to_string());

        let model_order = parse_models(&raw_models)?;

        Ok(Self {
            api_key,
            base_url,
            model_order,
        })
    }
}

fn parse_models(raw: &str) -> Result<Vec<ModelId>, ConfigError> {
    let models: Result<Vec<_>, _> = raw
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(|item| ModelId::new(item.to_string()))
        .collect();
    let models = models?;
    if models.is_empty() {
        return Err(ConfigError::EmptyModelOrder);
    }
    Ok(models)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ConfigError {
    MissingNimKey,
    EmptyModel,
    EmptyModelOrder,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ProviderFailure {
    RateLimited,
    ServerError,
    Timeout,
    EmptyResponse,
    UnavailableModel,
    MalformedEnvelope,
    InvalidModelOutput,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ToolFailure {
    NonZeroExit,
    FileNotFound,
    PermissionDenied,
    PathBlocked,
    VerifierRejected,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FailureClass {
    Provider(ProviderFailure),
    Tool(ToolFailure),
}

impl FailureClass {
    pub fn allows_model_fallback(&self) -> bool {
        matches!(self, Self::Provider(_))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RouteAttempt {
    pub model: ModelId,
    pub outcome: RouteOutcome,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RouteOutcome {
    Started,
    Succeeded,
    Failed(FailureClass),
    SkippedCooldown,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CooldownBook {
    until_epoch_second: HashMap<ModelId, u64>,
}

impl CooldownBook {
    pub fn new() -> Self {
        Self {
            until_epoch_second: HashMap::new(),
        }
    }

    pub fn mark(&mut self, model: ModelId, until_epoch_second: u64) {
        self.until_epoch_second.insert(model, until_epoch_second);
    }

    pub fn is_available(&self, model: &ModelId, now_epoch_second: u64) -> bool {
        self.until_epoch_second
            .get(model)
            .map(|until| *until <= now_epoch_second)
            .unwrap_or(true)
    }
}

impl Default for CooldownBook {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RoutePolicy {
    order: Vec<ModelId>,
    cooldowns: CooldownBook,
}

impl RoutePolicy {
    pub fn new(order: Vec<ModelId>) -> Result<Self, ConfigError> {
        if order.is_empty() {
            return Err(ConfigError::EmptyModelOrder);
        }
        Ok(Self {
            order,
            cooldowns: CooldownBook::new(),
        })
    }

    pub fn next_model(&self, now_epoch_second: u64) -> Option<ModelId> {
        self.order
            .iter()
            .find(|model| self.cooldowns.is_available(model, now_epoch_second))
            .cloned()
    }

    pub fn record_failure(
        &mut self,
        model: ModelId,
        failure: FailureClass,
        now_epoch_second: u64,
        cooldown_seconds: u64,
    ) -> RouteAttempt {
        if failure.allows_model_fallback() {
            self.cooldowns
                .mark(model.clone(), now_epoch_second.saturating_add(cooldown_seconds));
        }
        RouteAttempt {
            model,
            outcome: RouteOutcome::Failed(failure),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_requires_nim_key() {
        let err = NimConfig::from_pairs([("NIM_MODEL_ORDER", "model-a")]).unwrap_err();
        assert_eq!(err, ConfigError::MissingNimKey);
    }

    #[test]
    fn config_reads_nim_key_without_exposing_it() {
        let config = NimConfig::from_pairs([
            ("NIM_KEY", "secret-value-1234"),
            ("NIM_MODEL_ORDER", "model-a, model-b"),
        ])
        .unwrap();

        assert_eq!(config.base_url, DEFAULT_NIM_BASE_URL);
        assert_eq!(config.model_order[0].as_str(), "model-a");
        assert_eq!(config.model_order[1].as_str(), "model-b");
        assert_eq!(config.api_key.redacted(), "***1234 (17 chars)");
        assert!(!format!("{:?}", config.api_key).contains("secret-value"));
    }

    #[test]
    fn provider_failures_allow_fallback() {
        let failure = FailureClass::Provider(ProviderFailure::Timeout);
        assert!(failure.allows_model_fallback());
    }

    #[test]
    fn tool_failures_do_not_allow_fallback() {
        let failure = FailureClass::Tool(ToolFailure::NonZeroExit);
        assert!(!failure.allows_model_fallback());
    }

    #[test]
    fn cooldown_skips_failed_provider_model() {
        let first = ModelId::new("model-a").unwrap();
        let second = ModelId::new("model-b").unwrap();
        let mut policy = RoutePolicy::new(vec![first.clone(), second.clone()]).unwrap();

        let attempt = policy.record_failure(
            first,
            FailureClass::Provider(ProviderFailure::RateLimited),
            100,
            30,
        );

        assert!(matches!(attempt.outcome, RouteOutcome::Failed(_)));
        assert_eq!(policy.next_model(101), Some(second));
    }

    #[test]
    fn tool_failure_keeps_same_model_available() {
        let first = ModelId::new("model-a").unwrap();
        let mut policy = RoutePolicy::new(vec![first.clone()]).unwrap();

        policy.record_failure(
            first.clone(),
            FailureClass::Tool(ToolFailure::VerifierRejected),
            100,
            30,
        );

        assert_eq!(policy.next_model(101), Some(first));
    }
}
