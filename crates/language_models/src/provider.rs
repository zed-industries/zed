use collections::HashMap;
use http_client::CustomHeaders;
use http_client::http::{HeaderName, HeaderValue};
use language_model::LanguageModelRequest;
use settings::settings_content::{CustomHeaderSourceContent, CustomHeaderValueContent};
use std::sync::Arc;

pub mod anthropic;
pub mod anthropic_compatible;
pub mod api_compatible;
pub mod bedrock;
pub mod cloud;
pub mod copilot_chat;
pub mod deepseek;
pub mod google;
pub mod llama_cpp;
pub mod lmstudio;
pub mod mistral;
pub mod ollama;
pub mod open_ai;
pub mod open_ai_compatible;
pub mod open_router;
pub mod openai_subscribed;
pub mod opencode;

pub mod vercel_ai_gateway;
pub mod x_ai;

const COMMON_RESERVED_HEADER_NAMES: &[&str] = &["Authorization", "Content-Type", "Accept"];

/// User-configured custom headers for a language model provider: static
/// `(name, value)` pairs plus dynamic definitions resolved per request.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct CustomHeaderDefinitions {
    definitions: Arc<[CustomHeaderDefinition]>,
}

impl CustomHeaderDefinitions {
    /// Resolves the headers for a single outgoing request, substituting any
    /// dynamic values (such as the agent thread ID) from `request`.
    pub(crate) fn resolve(&self, request: &LanguageModelRequest) -> CustomHeaders {
        self.resolve_agent_thread_id(request.thread_id.as_deref())
    }

    /// Resolves the static headers only; dynamic values are omitted. Used for
    /// requests without a thread context, such as model discovery.
    pub(crate) fn resolve_static(&self) -> CustomHeaders {
        self.resolve_agent_thread_id(None)
    }

    fn resolve_agent_thread_id(&self, agent_thread_id: Option<&str>) -> CustomHeaders {
        let headers = self
            .definitions
            .iter()
            .filter_map(|definition| match definition {
                CustomHeaderDefinition::Static(name, value) => {
                    Some((name.clone(), value.clone()))
                }
                CustomHeaderDefinition::AgentThreadId(name) => {
                    let thread_id = agent_thread_id?;
                    match HeaderValue::from_str(thread_id) {
                        Ok(value) => Some((name.clone(), value)),
                        Err(error) => {
                            log::warn!(
                                "ignoring custom header `{name}` from agent thread ID: invalid \
                                 header value ({error})"
                            );
                            None
                        }
                    }
                }
            })
            .collect();
        CustomHeaders::new(headers)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum CustomHeaderDefinition {
    Static(HeaderName, HeaderValue),
    AgentThreadId(HeaderName),
}

/// Validate the user-supplied custom-headers map once at settings load time,
/// dropping reserved or malformed entries (each with a `log::warn!`) and
/// returning definitions ready to be resolved onto outgoing requests.
pub(crate) fn resolve_custom_headers(
    provider_name: &str,
    settings: &HashMap<String, CustomHeaderValueContent>,
    reserved_header_names: &[&str],
) -> CustomHeaderDefinitions {
    let definitions: Vec<_> = settings
        .iter()
        .filter_map(|(name, value)| {
            if COMMON_RESERVED_HEADER_NAMES
                .iter()
                .chain(reserved_header_names)
                .any(|reserved| reserved.eq_ignore_ascii_case(name))
            {
                log::warn!(
                    "ignoring custom {provider_name} header `{name}`: managed by Zed and cannot be overridden"
                );
                return None;
            }
            let header_name = match name.parse::<HeaderName>() {
                Ok(header_name) => header_name,
                Err(err) => {
                    log::warn!("ignoring custom {provider_name} header `{name}`: invalid header name ({err})");
                    return None;
                }
            };
            match value {
                CustomHeaderValueContent::Static(value) => {
                    let header_value = match HeaderValue::from_str(value) {
                        Ok(header_value) => header_value,
                        Err(err) => {
                            log::warn!(
                                "ignoring custom {provider_name} header `{name}`: invalid header value ({err})"
                            );
                            return None;
                        }
                    };
                    Some(CustomHeaderDefinition::Static(header_name, header_value))
                }
                CustomHeaderValueContent::Dynamic { source } => match source {
                    CustomHeaderSourceContent::AgentThreadId => Some(
                        CustomHeaderDefinition::AgentThreadId(header_name),
                    ),
                },
            }
        })
        .collect();
    CustomHeaderDefinitions {
        definitions: definitions.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map(pairs: &[(&str, &str)]) -> HashMap<String, CustomHeaderValueContent> {
        pairs
            .iter()
            .map(|(key, value)| {
                (
                    (*key).to_string(),
                    CustomHeaderValueContent::Static((*value).to_string()),
                )
            })
            .collect()
    }

    fn names(headers: &CustomHeaders) -> Vec<String> {
        let mut names: Vec<String> = headers
            .iter()
            .map(|(name, _)| name.as_str().to_owned())
            .collect();
        names.sort();
        names
    }

    #[test]
    fn drops_common_and_provider_reserved_headers() {
        let settings = map(&[
            ("Authorization", "Bearer leak"),
            ("Content-Type", "text/plain"),
            ("Accept", "text/plain"),
            ("X-Api-Key", "leak"),
            ("X-Allowed", "yes"),
        ]);
        let merged = resolve_custom_headers("Test", &settings, &["X-Api-Key"]);
        assert_eq!(names(&merged.resolve_static()), vec!["x-allowed".to_string()]);
    }

    #[test]
    fn reserved_header_match_is_case_insensitive() {
        let settings = map(&[
            ("authorization", "Bearer leak"),
            ("CONTENT-TYPE", "text/plain"),
            ("x-api-key", "leak"),
            ("X-Allowed", "yes"),
        ]);
        let merged = resolve_custom_headers("Test", &settings, &["X-Api-Key"]);
        assert_eq!(names(&merged.resolve_static()), vec!["x-allowed".to_string()]);
    }

    #[test]
    fn headers_with_reserved_prefix_are_kept() {
        let settings = map(&[("Authorization-Forwarded", "ok"), ("X-Api-Key-Trace", "ok")]);
        let merged = resolve_custom_headers("Test", &settings, &["X-Api-Key"]);
        assert_eq!(
            names(&merged.resolve_static()),
            vec![
                "authorization-forwarded".to_string(),
                "x-api-key-trace".to_string(),
            ]
        );
    }

    #[test]
    fn drops_invalid_header_name_and_value() {
        let settings = map(&[
            ("Bad Name", "ok"),
            ("X-Bad-Value", "line1\nline2"),
            ("X-Allowed", "yes"),
        ]);
        let merged = resolve_custom_headers("Test", &settings, &[]);
        assert_eq!(names(&merged.resolve_static()), vec!["x-allowed".to_string()]);
    }

    #[test]
    fn resolves_agent_thread_id_from_request() {
        let settings = HashMap::from([(
            "X-Agent-Thread".to_string(),
            CustomHeaderValueContent::Dynamic {
                source: CustomHeaderSourceContent::AgentThreadId,
            },
        )]);
        let definitions =
            resolve_custom_headers("Test", &settings, &[]);
        assert!(definitions.resolve_static().is_empty());

        let request = LanguageModelRequest {
            thread_id: Some("thread-uuid".into()),
            ..Default::default()
        };
        let headers = definitions.resolve(&request);
        let mut iter = headers.iter();
        let (name, value) = iter.next().expect("expected agent thread header");
        assert_eq!(name.as_str(), "x-agent-thread");
        assert_eq!(value.to_str().unwrap(), "thread-uuid");
        assert!(iter.next().is_none());
    }

    #[test]
    fn agent_thread_id_header_omitted_without_thread() {
        let settings = HashMap::from([(
            "X-Agent-Thread".to_string(),
            CustomHeaderValueContent::Dynamic {
                source: CustomHeaderSourceContent::AgentThreadId,
            },
        )]);
        let definitions = resolve_custom_headers("Test", &settings, &[]);
        let request = LanguageModelRequest::default();
        assert!(definitions.resolve(&request).is_empty());
    }
}
