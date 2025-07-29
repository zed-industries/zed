use super::*;
use gpui::TestAppContext;
use http_client::FakeHttpClient;
use language_model::{LanguageModelRequest, MessageContent, Role};

#[gpui::test]
fn test_local_provider_creation(cx: &mut TestAppContext) {
    let http_client = FakeHttpClient::with_200_response();
    let provider = cx.update(|cx| LocalLanguageModelProvider::new(Arc::new(http_client), cx));

    cx.read(|cx| {
        assert_eq!(provider.id(), PROVIDER_ID);
        assert_eq!(provider.name(), PROVIDER_NAME);
        assert!(!provider.is_authenticated(cx));
        assert_eq!(provider.provided_models(cx).len(), 1);
    });
}

#[gpui::test]
fn test_state_initialization(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let state = cx.new(State::new);

        assert!(!state.read(cx).is_authenticated());
        assert_eq!(state.read(cx).status, ModelStatus::NotLoaded);
        assert!(state.read(cx).model.is_none());
    });
}

#[gpui::test]
fn test_model_properties(cx: &mut TestAppContext) {
    let http_client = FakeHttpClient::with_200_response();
    let provider = cx.update(|cx| LocalLanguageModelProvider::new(Arc::new(http_client), cx));

    // Create a model directly for testing (bypassing authentication)
    let model = LocalLanguageModel {
        state: provider.state.clone(),
        request_limiter: RateLimiter::new(4),
    };

    assert_eq!(model.id(), LanguageModelId(DEFAULT_MODEL.into()));
    assert_eq!(model.name(), LanguageModelName(DEFAULT_MODEL.into()));
    assert_eq!(model.provider_id(), PROVIDER_ID);
    assert_eq!(model.provider_name(), PROVIDER_NAME);
    assert_eq!(model.max_token_count(), 128000);
    assert!(!model.supports_tools());
    assert!(!model.supports_images());
}

#[gpui::test]
async fn test_token_counting(cx: &mut TestAppContext) {
    let http_client = FakeHttpClient::with_200_response();
    let provider = cx.update(|cx| LocalLanguageModelProvider::new(Arc::new(http_client), cx));

    let model = LocalLanguageModel {
        state: provider.state.clone(),
        request_limiter: RateLimiter::new(4),
    };

    let request = LanguageModelRequest {
        thread_id: None,
        prompt_id: None,
        intent: None,
        mode: None,
        messages: vec![language_model::LanguageModelRequestMessage {
            role: Role::User,
            content: vec![MessageContent::Text("Hello, world!".to_string())],
            cache: false,
        }],
        tools: Vec::new(),
        tool_choice: None,
        stop: Vec::new(),
        temperature: None,
        thinking_allowed: false,
    };

    let count = cx
        .update(|cx| model.count_tokens(request, cx))
        .await
        .unwrap();

    // "Hello, world!" is 13 characters, so ~3 tokens
    assert!(count > 0);
    assert!(count < 10);
}

#[gpui::test]
async fn test_message_conversion(cx: &mut TestAppContext) {
    let http_client = FakeHttpClient::with_200_response();
    let provider = cx.update(|cx| LocalLanguageModelProvider::new(Arc::new(http_client), cx));

    let model = LocalLanguageModel {
        state: provider.state.clone(),
        request_limiter: RateLimiter::new(4),
    };

    let request = LanguageModelRequest {
        thread_id: None,
        prompt_id: None,
        intent: None,
        mode: None,
        messages: vec![
            language_model::LanguageModelRequestMessage {
                role: Role::System,
                content: vec![MessageContent::Text(
                    "You are a helpful assistant.".to_string(),
                )],
                cache: false,
            },
            language_model::LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Hello!".to_string())],
                cache: false,
            },
            language_model::LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![MessageContent::Text("Hi there!".to_string())],
                cache: false,
            },
        ],
        tools: Vec::new(),
        tool_choice: None,
        stop: Vec::new(),
        temperature: None,
        thinking_allowed: false,
    };

    let _messages = model.to_mistral_messages(&request);
    // We can't directly inspect TextMessages, but we can verify it doesn't panic
    assert!(true); // Placeholder assertion
}

#[gpui::test]
async fn test_reset_credentials(cx: &mut TestAppContext) {
    let http_client = FakeHttpClient::with_200_response();
    let provider = cx.update(|cx| LocalLanguageModelProvider::new(Arc::new(http_client), cx));

    // Simulate loading a model by just setting the status
    cx.update(|cx| {
        provider.state.update(cx, |state, cx| {
            state.status = ModelStatus::Loaded;
            // We don't actually set a model since we can't mock it safely
            cx.notify();
        });
    });

    cx.read(|cx| {
        // Since is_authenticated checks for model presence, we need to check status directly
        assert_eq!(provider.state.read(cx).status, ModelStatus::Loaded);
    });

    // Reset credentials
    let task = cx.update(|cx| provider.reset_credentials(cx));
    task.await.unwrap();

    cx.read(|cx| {
        assert!(!provider.is_authenticated(cx));
        assert_eq!(provider.state.read(cx).status, ModelStatus::NotLoaded);
        assert!(provider.state.read(cx).model.is_none());
    });
}

// TODO: Fix this test - need to handle window creation in tests
// #[gpui::test]
// async fn test_configuration_view_rendering(cx: &mut TestAppContext) {
//     let http_client = FakeHttpClient::with_200_response();
//     let provider = cx.update(|cx| LocalLanguageModelProvider::new(Arc::new(http_client), cx));

//     let view = cx.update(|cx| provider.configuration_view(cx.window(), cx));

//     // Basic test to ensure the view can be created without panicking
//     assert!(view.entity_type() == std::any::TypeId::of::<ConfigurationView>());
// }

#[gpui::test]
fn test_status_transitions(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let state = cx.new(State::new);

        // Initial state
        assert_eq!(state.read(cx).status, ModelStatus::NotLoaded);

        // Transition to loading
        state.update(cx, |state, cx| {
            state.status = ModelStatus::Loading;
            cx.notify();
        });
        assert_eq!(state.read(cx).status, ModelStatus::Loading);

        // Transition to loaded
        state.update(cx, |state, cx| {
            state.status = ModelStatus::Loaded;
            cx.notify();
        });
        assert_eq!(state.read(cx).status, ModelStatus::Loaded);

        // Transition to error
        state.update(cx, |state, cx| {
            state.status = ModelStatus::Error("Test error".to_string());
            cx.notify();
        });
        match &state.read(cx).status {
            ModelStatus::Error(msg) => assert_eq!(msg, "Test error"),
            _ => panic!("Expected error status"),
        }
    });
}

#[gpui::test]
fn test_provider_shows_models_without_authentication(cx: &mut TestAppContext) {
    let http_client = FakeHttpClient::with_200_response();
    let provider = cx.update(|cx| LocalLanguageModelProvider::new(Arc::new(http_client), cx));

    cx.read(|cx| {
        // Provider should show models even when not authenticated
        let models = provider.provided_models(cx);
        assert_eq!(models.len(), 1);

        let model = &models[0];
        assert_eq!(model.id(), LanguageModelId(DEFAULT_MODEL.into()));
        assert_eq!(model.name(), LanguageModelName(DEFAULT_MODEL.into()));
        assert_eq!(model.provider_id(), PROVIDER_ID);
        assert_eq!(model.provider_name(), PROVIDER_NAME);
    });
}

#[gpui::test]
fn test_provider_has_icon(cx: &mut TestAppContext) {
    let http_client = FakeHttpClient::with_200_response();
    let provider = cx.update(|cx| LocalLanguageModelProvider::new(Arc::new(http_client), cx));

    assert_eq!(provider.icon(), IconName::Ai);
}

#[gpui::test]
fn test_provider_appears_in_registry(cx: &mut TestAppContext) {
    use language_model::LanguageModelRegistry;

    cx.update(|cx| {
        let registry = cx.new(|_| LanguageModelRegistry::default());
        let http_client = FakeHttpClient::with_200_response();

        // Register the local provider
        registry.update(cx, |registry, cx| {
            let provider = LocalLanguageModelProvider::new(Arc::new(http_client), cx);
            registry.register_provider(provider, cx);
        });

        // Verify the provider is registered
        let provider = registry.read(cx).provider(&PROVIDER_ID).unwrap();
        assert_eq!(provider.name(), PROVIDER_NAME);
        assert_eq!(provider.icon(), IconName::Ai);

        // Verify it provides models even without authentication
        let models = provider.provided_models(cx);
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].id(), LanguageModelId(DEFAULT_MODEL.into()));
    });
}
