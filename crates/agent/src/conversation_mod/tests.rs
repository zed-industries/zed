#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::future::Future;
    use std::pin::Pin;
    use std::cell::RefCell;
    use std::collections::HashMap;
    
    use anyhow::{anyhow, Result};
    use assistant_tool::{ActionLog, Tool, ToolWorkingSet};
    use gpui::{Context, Entity, TestAppContext};
    use language_model::{
        ConfiguredModel, LanguageModel, LanguageModelCompletionEvent, LanguageModelId, 
        LanguageModelProviderId, LanguageModelRequest, LanguageModelToolResult, 
        LanguageModelToolResultContent, Role, StopReason, TokenUsage,
        LanguageModelRegistry, GlobalLanguageModelRegistry,
    };
    use language_model::test::TestLanguageModel;
    use serde::{Deserialize, Serialize};
    
    use crate::context::LoadedContext;
    use crate::conversation_mod::{
        Conversation, ConversationController, ConversationEvent, ConversationId, MessageSegment,
        ToolService, DefaultToolService, CompletionService, CompletionEvent, DefaultCompletionService,
        ContextService, DefaultContextService,
    };
    use crate::thread::Thread;
    use futures::Stream;
    use pretty_assertions::assert_eq;
    use project::Project;
    use prompt_store::PromptBuilder;
    use crate::context::{ContextLoadResult};
    use crate::thread_store::SharedProjectContext;
    
    /// Mock GlobalLanguageModelRegistry for testing
    struct MockGlobalLanguageModelRegistry {
        mock_models: Vec<Arc<dyn LanguageModel>>,
    }

    impl Default for MockGlobalLanguageModelRegistry {
        fn default() -> Self {
            Self {
                mock_models: Vec::new(),
            }
        }
    }

    impl gpui::Global for MockGlobalLanguageModelRegistry {}

    /// Mock implementation of a language model for testing
    struct MockLanguageModel {
        id: LanguageModelId,
        provider_id: LanguageModelProviderId,
        supports_tools: bool,
        complete_fn: RefCell<Box<dyn FnMut(&LanguageModelRequest) -> Pin<Box<dyn Future<Output = Result<Box<dyn Stream<Item = Result<LanguageModelCompletionEvent>> + Unpin + Send>>> + Send>> + Send>>,
    }

    impl MockLanguageModel {
        fn new(
            id: impl Into<String>,
            provider: impl Into<String>,
            supports_tools: bool,
            complete_fn: impl FnMut(&LanguageModelRequest) -> Pin<Box<dyn Future<Output = Result<Box<dyn Stream<Item = Result<LanguageModelCompletionEvent>> + Unpin + Send>>> + Send>> + Send + 'static,
        ) -> Arc<Self> {
            Arc::new(Self {
                id: LanguageModelId::from(id.into()),
                provider_id: LanguageModelProviderId::from(provider.into()),
                supports_tools,
                complete_fn: RefCell::new(Box::new(complete_fn)),
            })
        }
    }

    impl LanguageModel for MockLanguageModel {
        fn id(&self) -> &LanguageModelId {
            &self.id
        }
        
        fn name(&self) -> &str {
            self.id.0.as_ref()
        }
        
        fn provider_id(&self) -> &LanguageModelProviderId {
            &self.provider_id
        }
        
        fn capabilities(&self) -> language_model::LanguageModelCapabilities {
            language_model::LanguageModelCapabilities {
                supports_tools: self.supports_tools,
                supports_thinking: true,
                supports_tool_choice: true,
                ..Default::default()
            }
        }
        
        fn complete(
            &self,
            request: &LanguageModelRequest,
        ) -> impl std::future::Future<Output = Result<Box<dyn futures::Stream<Item = Result<LanguageModelCompletionEvent>> + Unpin + Send>>> + Send {
            (self.complete_fn.borrow_mut())(request)
        }
        
        fn count_tokens(&self, _request: &LanguageModelRequest) -> impl std::future::Future<Output = Result<TokenUsage>> + Send {
            async move {
                Ok(TokenUsage {
                    prompt_tokens: 100,
                    completion_tokens: 50,
                    total_tokens: 150,
                })
            }
        }
    }

    /// Implementation of LanguageModel trait for MockGlobalLanguageModelRegistry
    impl language_model::LanguageModelRegistry for MockGlobalLanguageModelRegistry {
        fn models(&self) -> impl Iterator<Item = &language_model::ModelInfo> {
            // We don't use this for our tests
            [].iter()
        }
        
        fn get_model(&self, _provider_id: &LanguageModelProviderId, model_id: &LanguageModelId) -> Option<Arc<dyn LanguageModel>> {
            self.mock_models.iter()
                .find(|m| m.id() == model_id)
                .cloned()
        }
        
        fn default_model(&self) -> Option<ConfiguredModel> {
            self.mock_models.first().map(|model| ConfiguredModel {
                provider_id: model.provider_id().clone(),
                model_id: model.id().clone(),
                model: model.clone(),
            })
        }
        
        // Other methods can be implemented as needed
    }

    fn setup_test() -> TestAppContext {
        let mut cx = TestAppContext::new();
        
        // Initialize test registry
        cx.update(|cx| {
            cx.set_global(MockGlobalLanguageModelRegistry::default());
        });
        
        cx
    }
    
    #[gpui::test]
    async fn test_conversation_controller_basics(cx: &mut TestAppContext) -> Result<()> {
        // Set up test environment
        let project = cx.update(|cx| cx.new(|cx| project::Project::test(cx)))?;
        let tools = cx.update(|cx| cx.new(|cx| ToolWorkingSet::new(cx)))?;
        let action_log = cx.update(|cx| cx.new(|cx| ActionLog::new(cx)))?;
        let prompt_builder = Arc::new(prompt_store::PromptBuilder::default());
        let project_context = Default::default();
        
        // Create controller
        let controller = cx.update(|cx| {
            cx.new(|cx| {
                ConversationController::new(
                    None,
                    project.clone(),
                    tools.clone(),
                    action_log.clone(),
                    prompt_builder.clone(),
                    project_context.clone(),
                    cx,
                )
            })
        })?;
        
        // Test inserting a user message
        let user_message_id = cx.update(|cx| {
            controller.update(cx, |controller, cx| {
                controller.insert_user_message("Hello, assistant", LoadedContext::default(), vec![], cx)
            })
        })?;
        
        // Verify conversation state
        cx.update(|cx| {
            let conversation = controller.read(cx).conversation();
            assert_eq!(conversation.messages().len(), 1);
            let message = conversation.message(user_message_id).unwrap();
            assert_eq!(message.role, Role::User);
            
            let segments = &message.segments;
            assert_eq!(segments.len(), 1);
            if let MessageSegment::Text(text) = &segments[0] {
                assert_eq!(text, "Hello, assistant");
            } else {
                panic!("Expected Text segment");
            }
        })?;
        
        // Create test model
        let model = Arc::new(TestLanguageModel::default());
        
        // Set configured model
        let provider_id = LanguageModelProviderId::from("test_provider");
        let model_id = LanguageModelId::from("test_model");
        
        cx.update(|cx| {
            controller.update(cx, |controller, cx| {
                controller.set_configured_model(Some(ConfiguredModel {
                    provider_id: provider_id.clone(),
                    model_id: model_id.clone(),
                }));
            });
        })?;
        
        // Collect events to verify correct operation
        let mut received_events = Vec::new();
        cx.update(|cx| {
            cx.subscribe(&controller, move |_, _, event, _| {
                received_events.push(event.clone());
            });
        })?;
        
        // Send to model
        cx.update(|cx| {
            controller.update(cx, |controller, cx| {
                controller.send_to_model(model.clone(), None, cx);
            });
        })?;
        
        // Run some ticks to allow async operations to complete
        cx.run_until_parked();
        
        // Verify that controller state is updated
        cx.update(|cx| {
            let conversation = controller.read(cx).conversation();
            assert_eq!(conversation.messages().len(), 2); // User message + assistant message
            
            // Ensure we received the expected events
            assert!(received_events.iter().any(|e| matches!(e, ConversationEvent::MessageAdded(_))));
            
            // If the TestLanguageModel produced any text, we should have received it
            assert!(received_events.iter().any(|e| 
                matches!(e, ConversationEvent::StreamedText { .. }) ||
                matches!(e, ConversationEvent::Stopped(..))
            ));
        })?;
        
        Ok(())
    }
    
    #[gpui::test]
    async fn test_conversation_model(cx: &mut TestAppContext) -> Result<()> {
        // Create a conversation
        let conversation = Conversation::new(Some(ConversationId::from("test_conversation")));
        
        // Add a user message
        let user_message_id = conversation.insert_user_message(
            "Hello, assistant",
            LoadedContext::default(),
            vec![],
        );
        
        // Add an assistant message
        let assistant_message_id = conversation.insert_assistant_message(vec![
            MessageSegment::Text("Hello, user!".to_string()),
        ]);
        
        // Verify state
        assert_eq!(conversation.messages().len(), 2);
        assert_eq!(conversation.id().to_string(), "test_conversation");
        
        // Test message access
        let user_message = conversation.message(user_message_id).unwrap();
        assert_eq!(user_message.role, Role::User);
        
        let assistant_message = conversation.message(assistant_message_id).unwrap();
        assert_eq!(assistant_message.role, Role::Assistant);
        
        // Test editing a message
        let edited = conversation.edit_message(
            assistant_message_id,
            Role::Assistant,
            vec![MessageSegment::Text("Updated response".to_string())],
            None,
        );
        
        assert!(edited);
        
        let updated_message = conversation.message(assistant_message_id).unwrap();
        if let MessageSegment::Text(text) = &updated_message.segments[0] {
            assert_eq!(text, "Updated response");
        } else {
            panic!("Expected Text segment");
        }
        
        // Test deletion
        let deleted = conversation.delete_message(user_message_id);
        assert!(deleted);
        assert_eq!(conversation.messages().len(), 1);
        
        // Test conversation text
        let conversation = Conversation::new(None);
        conversation.insert_user_message("Hello", LoadedContext::default(), vec![]);
        conversation.insert_assistant_message(vec![MessageSegment::Text("Hi there".to_string())]);
        
        let text = conversation.text();
        assert!(text.contains("# User:"));
        assert!(text.contains("Hello"));
        assert!(text.contains("# Assistant:"));
        assert!(text.contains("Hi there"));
        
        Ok(())
    }

    #[gpui::test]
    async fn test_conversation_with_tools(cx: &mut TestAppContext) -> Result<()> {
        // Setup test environment
        let project = cx.update(|cx| cx.new(|cx| project::Project::test(cx)))?;
        let tools = cx.update(|cx| cx.new(|cx| {
            let mut tool_set = ToolWorkingSet::new(cx);
            // We could add test tools to the set here if needed
            tool_set
        }))?;
        let action_log = cx.update(|cx| cx.new(|cx| ActionLog::new(cx)))?;
        let prompt_builder = Arc::new(prompt_store::PromptBuilder::default());
        let project_context = Default::default();
        
        // Create controller
        let controller = cx.update(|cx| {
            cx.new(|cx| {
                ConversationController::new(
                    None,
                    project.clone(),
                    tools.clone(),
                    action_log.clone(),
                    prompt_builder.clone(),
                    project_context.clone(),
                    cx,
                )
            })
        })?;
        
        // Create a model that will output tool calls
        struct ToolCallingModel {
            tool_name: String,
            tool_args: serde_json::Value,
        }
        
        impl LanguageModel for ToolCallingModel {
            fn id(&self) -> LanguageModelId {
                LanguageModelId::from("test_model")
            }
            
            fn name(&self) -> String {
                "Test Tool Model".to_string()
            }
            
            fn provider_id(&self) -> LanguageModelProviderId {
                LanguageModelProviderId::from("test")
            }
            
            async fn complete(&self, _request: &LanguageModelRequest) -> Result<Box<dyn futures::Stream<Item = Result<LanguageModelCompletionEvent, String>> + Unpin + Send>> {
                let tool_name = self.tool_name.clone();
                let tool_args = self.tool_args.clone();
                
                // Create a stream that emits a tool call
                Ok(Box::new(futures::stream::iter(vec![
                    Ok(LanguageModelCompletionEvent::ContentBlock {
                        content: "I'll help with that".to_string(),
                        is_final: false,
                        end_turn: false,
                    }),
                    Ok(LanguageModelCompletionEvent::ToolCall {
                        tool_name: tool_name.into(),
                        tool_call_id: "test_tool_call_id".into(),
                        arguments: tool_args,
                        is_final: true,
                    }),
                    Ok(LanguageModelCompletionEvent::StreamEnd {
                        stop_reason: StopReason::EndContent,
                    }),
                ])))
            }
            
            async fn count_tokens(&self, _request: &LanguageModelRequest) -> Result<language_model::TokenUsage> {
                Ok(language_model::TokenUsage {
                    prompt_tokens: 10,
                    completion_tokens: 10,
                    total_tokens: 20,
                })
            }
        }
        
        let tool_args = serde_json::json!({
            "arg1": "value1",
            "arg2": "value2",
        });
        
        let model = Arc::new(ToolCallingModel {
            tool_name: "test_tool".to_string(),
            tool_args: tool_args.clone(),
        });
        
        // Insert a user message
        let user_message_id = cx.update(|cx| {
            controller.update(cx, |controller, cx| {
                controller.insert_user_message("Use a tool", LoadedContext::default(), vec![], cx)
            })
        })?;
        
        // Collect events
        let mut received_events = Vec::new();
        cx.update(|cx| {
            cx.subscribe(&controller, move |_, _, event, _| {
                received_events.push(event.clone());
            });
        })?;
        
        // Send to model
        cx.update(|cx| {
            controller.update(cx, |controller, cx| {
                controller.send_to_model(model.clone(), None, cx);
            });
        })?;
        
        // Run some ticks to allow async operations to complete
        cx.run_until_parked();
        
        // Verify tool call event
        let tool_call_event = received_events.iter().find(|e| matches!(e, ConversationEvent::ToolCall { .. }));
        assert!(tool_call_event.is_some(), "No tool call event found");
        
        if let ConversationEvent::ToolCall { tool_name, input, .. } = tool_call_event.unwrap() {
            assert_eq!(tool_name.to_string(), "test_tool");
            assert_eq!(input, &tool_args);
        }
        
        Ok(())
    }
    
    #[gpui::test]
    async fn test_completion_service(cx: &mut TestAppContext) -> Result<()> {
        // Create a test conversation
        let conversation = Conversation::new(None);
        let msg_id = conversation.insert_user_message("Hello", LoadedContext::default(), vec![]);
        
        // Create a test language model that returns predefined responses
        struct TestCompletionModel;
        
        impl LanguageModel for TestCompletionModel {
            fn id(&self) -> LanguageModelId {
                LanguageModelId::from("test_model")
            }
            
            fn name(&self) -> String {
                "Test Completion Model".to_string()
            }
            
            fn provider_id(&self) -> LanguageModelProviderId {
                LanguageModelProviderId::from("test")
            }
            
            async fn complete(&self, _request: &LanguageModelRequest) -> Result<Box<dyn futures::Stream<Item = Result<LanguageModelCompletionEvent, String>> + Unpin + Send>> {
                // Return a stream with text chunks and thinking
                Ok(Box::new(futures::stream::iter(vec![
                    Ok(LanguageModelCompletionEvent::ContentBlock {
                        content: "Hello".to_string(),
                        is_final: false,
                        end_turn: false,
                    }),
                    Ok(LanguageModelCompletionEvent::ThinkingBlock {
                        content: "I'm thinking...".to_string(),
                        is_final: false,
                        end_turn: false,
                        signature: Some("reasoning".to_string()),
                    }),
                    Ok(LanguageModelCompletionEvent::ContentBlock {
                        content: " world!".to_string(),
                        is_final: true,
                        end_turn: true,
                    }),
                    Ok(LanguageModelCompletionEvent::StreamEnd {
                        stop_reason: StopReason::EndContent,
                    }),
                ])))
            }
            
            async fn count_tokens(&self, _request: &LanguageModelRequest) -> Result<language_model::TokenUsage> {
                Ok(language_model::TokenUsage {
                    prompt_tokens: 5,
                    completion_tokens: 5,
                    total_tokens: 10,
                })
            }
        }
        
        let model = Arc::new(TestCompletionModel);
        
        // Create completion service
        let completion_service = DefaultCompletionService::new();
        
        // Stream the completion
        let mut completion_stream = completion_service
            .stream_completion(
                &conversation,
                model.clone(),
                vec![], // No tools
                msg_id,
                None, // No window
                &mut cx.to_async(),
            )
            .await;
        
        // Collect events
        let mut events = Vec::new();
        while let Some(event) = completion_stream.next().await {
            events.push(event);
        }
        
        // Verify events
        assert_eq!(events.len(), 4);
        
        // Check events
        assert!(matches!(events[0].as_ref().unwrap(), CompletionEvent::TextChunk(text) if text == "Hello"));
        assert!(matches!(events[1].as_ref().unwrap(), CompletionEvent::ThinkingChunk { text, signature } 
            if text == "I'm thinking..." && signature.as_ref().unwrap() == "reasoning"));
        assert!(matches!(events[2].as_ref().unwrap(), CompletionEvent::TextChunk(text) if text == " world!"));
        assert!(matches!(events[3].as_ref().unwrap(), CompletionEvent::Stopped(Ok(_))));
        
        // Test request preparation
        let request = completion_service.prepare_request(&conversation, model.clone(), vec![]);
        assert_eq!(request.thread_id, conversation.id().to_string());
        assert_eq!(request.messages.len(), 1);
        
        // Test token usage calculation
        let token_usage = completion_service.calculate_token_usage(&request, &model).await?;
        assert_eq!(token_usage.total_tokens, 10);
        
        Ok(())
    }
    
    #[gpui::test]
    async fn test_tool_service(cx: &mut TestAppContext) -> Result<()> {
        // Define a test tool
        #[derive(Clone, Debug, Serialize, Deserialize)]
        struct TestToolInput {
            message: String,
        }
        
        #[derive(Clone, Debug, Serialize, Deserialize)]
        struct TestToolOutput {
            response: String,
        }
        
        struct TestTool;
        
        impl Tool for TestTool {
            fn name(&self) -> &str {
                "test_tool"
            }
            
            fn description(&self) -> &str {
                "A test tool that echoes back the input message"
            }
            
            fn schema(&self) -> serde_json::Value {
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "message": {
                            "type": "string",
                            "description": "Message to echo"
                        }
                    },
                    "required": ["message"]
                })
            }
            
            async fn execute(
                &self,
                input: &serde_json::Value,
                _request: &LanguageModelRequest,
                _window: Option<&gpui::WindowHandle>,
                _cx: &AsyncApp,
            ) -> Result<serde_json::Value> {
                // Parse input
                let input: TestToolInput = serde_json::from_value(input.clone())?;
                
                // Return response
                let output = TestToolOutput {
                    response: format!("Echo: {}", input.message),
                };
                
                Ok(serde_json::to_value(output)?)
            }
        }
        
        // Setup tool working set
        let tools = cx.update(|cx| {
            cx.new(|cx| {
                let mut tool_set = ToolWorkingSet::new(cx);
                // Add our test tool to the set
                tool_set.register_tool(Arc::new(TestTool));
                tool_set
            })
        })?;
        
        // Create tool service
        let tool_service = DefaultToolService::new(tools.clone());
        
        // Run the tool
        let tool_result = tool_service.run_tool(
            "test_id".into(),
            "test_tool".into(),
            serde_json::json!({"message": "Hello from test"}),
            msg_id, // Placeholder message ID
            Arc::new(LanguageModelRequest::default()),
            Arc::new(TestLanguageModel::default()),
            None,
            &mut cx.to_async(),
        ).await?;
        
        // Verify result
        assert_eq!(tool_result.id, "test_id");
        assert_eq!(tool_result.name, "test_tool");
        
        if let LanguageModelToolResultContent::Object(json) = &tool_result.content {
            let output: TestToolOutput = serde_json::from_value(json.clone())?;
            assert_eq!(output.response, "Echo: Hello from test");
        } else {
            panic!("Expected Object content");
        }
        
        // Test hallucinated tool
        let hallucinated_result = tool_service.handle_hallucinated_tool(
            "fake_id".into(),
            "non_existent_tool".into(),
            None,
            &mut cx.to_async(),
        ).await?;
        
        // Verify error result
        assert_eq!(hallucinated_result.id, "fake_id");
        assert_eq!(hallucinated_result.name, "non_existent_tool");
        
        if let LanguageModelToolResultContent::Error(error) = &hallucinated_result.content {
            assert!(error.contains("does not exist"));
        } else {
            panic!("Expected Error content");
        }
        
        Ok(())
    }
    
    #[gpui::test]
    async fn test_error_handling(cx: &mut TestAppContext) -> Result<()> {
        // Create a test conversation
        let conversation = Conversation::new(None);
        let msg_id = conversation.insert_user_message("Hello", LoadedContext::default(), vec![]);
        
        // Create a model that returns an error
        struct ErrorModel;
        
        impl LanguageModel for ErrorModel {
            fn id(&self) -> LanguageModelId {
                LanguageModelId::from("error_model")
            }
            
            fn name(&self) -> String {
                "Error Model".to_string()
            }
            
            fn provider_id(&self) -> LanguageModelProviderId {
                LanguageModelProviderId::from("test")
            }
            
            async fn complete(&self, _request: &LanguageModelRequest) -> Result<Box<dyn futures::Stream<Item = Result<LanguageModelCompletionEvent, String>> + Unpin + Send>> {
                Ok(Box::new(futures::stream::iter(vec![
                    Err("Test error message".to_string()),
                ])))
            }
            
            async fn count_tokens(&self, _request: &LanguageModelRequest) -> Result<language_model::TokenUsage> {
                Err(anyhow!("Token counting error"))
            }
        }
        
        let model = Arc::new(ErrorModel);
        
        // Create completion service
        let completion_service = DefaultCompletionService::new();
        
        // Stream the completion
        let mut completion_stream = completion_service
            .stream_completion(
                &conversation,
                model.clone(),
                vec![], // No tools
                msg_id,
                None, // No window
                &mut cx.to_async(),
            )
            .await;
        
        // Get the first event
        let first_event = completion_stream.next().await.unwrap();
        
        // Verify it's an error
        assert!(first_event.is_err());
        if let Err(error) = first_event {
            match error {
                crate::conversation_mod::CompletionError::Message { header, message } => {
                    assert_eq!(header, "Error");
                    assert_eq!(message, "Test error message");
                },
                _ => panic!("Expected Message error"),
            }
        }
        
        Ok(())
    }

    #[gpui::test]
    async fn test_conversation_adapter(cx: &mut TestAppContext) -> Result<()> {
        // Set up test environment
        let project = cx.update(|cx| cx.new(|cx| project::Project::test(cx)))?;
        let tools = cx.update(|cx| cx.new(|cx| ToolWorkingSet::new(cx)))?;
        let action_log = cx.update(|cx| cx.new(|cx| ActionLog::new(cx)))?;
        let prompt_builder = Arc::new(prompt_store::PromptBuilder::default());
        let project_context = Default::default();
        
        // Create an adapter
        let adapter = cx.update(|cx| {
            ConversationAdapter::new(
                None,
                project.clone(),
                tools.clone(),
                action_log.clone(),
                prompt_builder.clone(),
                project_context.clone(),
                cx,
            )
        });
        
        // Add a message
        cx.update(|cx| {
            adapter.insert_user_message("Hello, assistant", 
                Default::default(), vec![], cx);
        });
        
        // Get the thread
        let thread = cx.update(|cx| adapter.to_thread(cx));
        
        // Verify thread state
        assert_eq!(thread.messages.len(), 1);
        assert_eq!(thread.messages[0].segments.len(), 1);
        match &thread.messages[0].segments[0] {
            crate::thread::MessageSegment::Text(text) => {
                assert_eq!(text, "Hello, assistant");
            }
            _ => panic!("Expected Text segment"),
        }
        
        // Test round trip
        let adapter2 = cx.update(|cx| ConversationAdapter::from_thread(&thread, cx))?;
        let thread2 = cx.update(|cx| adapter2.to_thread(cx));
        
        // Verify round trip conversion works
        assert_eq!(thread2.messages.len(), 1);
        assert_eq!(thread2.messages[0].segments.len(), 1);
        match &thread2.messages[0].segments[0] {
            crate::thread::MessageSegment::Text(text) => {
                assert_eq!(text, "Hello, assistant");
            }
            _ => panic!("Expected Text segment"),
        }
        
        Ok(())
    }

    /// Test continuous thinking capability with fallback mechanism
    #[test]
    fn test_continuous_thinking_with_fallback() {
        let mut cx = setup_test();
        
        // Create controller
        let controller = setup_controller(&mut cx);
        
        // Register mock models in the registry
        let primary_model = MockLanguageModel::new(
            "primary_model",
            "provider1",
            true,
            |_request| {
                Box::pin(async {
                    Err(anyhow!("Primary model failed"))
                })
            }
        );
        
        let fallback_model = MockLanguageModel::new(
            "fallback_model",
            "provider2",
            true,
            |_request| {
                use futures::stream;
                Box::pin(async {
                    Ok(Box::new(stream::iter(vec![
                        Ok(LanguageModelCompletionEvent::ThinkingBlock {
                            content: "Analyzing the conversation for a different approach...".into(),
                            is_final: false,
                            end_turn: false,
                            signature: Some("continuous_thinking".to_string()),
                        }),
                        Ok(LanguageModelCompletionEvent::ContentBlock {
                            content: "Let me try a completely different approach to your problem.".into(),
                            is_final: true,
                            finish_details: None,
                        }),
                        Ok(LanguageModelCompletionEvent::StreamEnd {
                            stop_reason: StopReason::EndContent,
                        }),
                    ])) as Box<dyn Stream<Item = Result<LanguageModelCompletionEvent>> + Unpin + Send>)
                })
            }
        );
        
        cx.update(|cx| {
            cx.set_global(MockGlobalLanguageModelRegistry {
                mock_models: vec![primary_model.clone(), fallback_model.clone()],
            });
        });
        
        // Insert user message
        controller.update(&mut cx, |controller, cx| {
            controller.insert_user_message(
                "I need help with a complex problem",
                ContextLoadResult::Loaded(LoadedContext::default()),
                Vec::new(),
                cx,
            );
        });
        
        // Start continuous thinking
        controller.update(&mut cx, |controller, cx| {
            controller.perform_continuous_thinking(None, cx);
        });
        
        // Run until completion
        cx.run_until_parked();
        
        // Verify results
        controller.read(&mut cx, |controller| {
            let messages = controller.conversation().messages().collect::<Vec<_>>();
            
            // Should have 2 messages (user + assistant)
            assert_eq!(messages.len(), 2, "Expected 2 messages");
            
            // Check that fallback worked
            let assistant_message = &messages[1];
            
            // Should have at least one segment (thinking)
            assert!(!assistant_message.segments.is_empty(), "Expected at least one segment in assistant message");
            
            // Check thinking segment
            for segment in &assistant_message.segments {
                match segment {
                    MessageSegment::Thinking { text, signature } => {
                        assert!(text.contains("Analyzing"), "Expected thinking content to be present");
                        assert_eq!(signature.as_ref().unwrap(), "continuous_thinking", "Expected 'continuous_thinking' signature");
                        return; // Found the thinking segment
                    }
                    _ => {}
                }
            }
            
            panic!("No thinking segment found");
        });
    }

    #[test]
    fn test_conversation_creation_and_serialization() {
        use crate::conversation_mod::conversation::{Conversation, ConversationId, Message, MessageId};
        use language_model::{Role, MessageContent};
        use std::collections::HashMap;
        
        // Create a simple conversation
        let conversation_id = ConversationId::random();
        let mut conversation = Conversation::new(conversation_id);
        
        // Add some messages
        let system_message = Message::new(
            MessageId::random(),
            Role::System,
            MessageContent::Text("You are a helpful assistant".into()),
        );
        
        let user_message = Message::new(
            MessageId::random(),
            Role::User,
            MessageContent::Text("Hello, how are you?".into()),
        );
        
        let assistant_message = Message::new(
            MessageId::random(),
            Role::Assistant,
            MessageContent::Text("I'm doing well, thanks for asking!".into()),
        );
        
        conversation.add_message(system_message);
        conversation.add_message(user_message);
        conversation.add_message(assistant_message);
        
        // Test serialization
        let serialized = conversation.serialize();
        
        // Basic assertions
        assert!(!serialized.is_empty(), "Serialized conversation should not be empty");
        assert!(serialized.contains("You are a helpful assistant"), "Serialized conversation should contain system prompt");
        assert!(serialized.contains("Hello, how are you?"), "Serialized conversation should contain user message");
        assert!(serialized.contains("I'm doing well, thanks for asking!"), "Serialized conversation should contain assistant response");
    }
} 