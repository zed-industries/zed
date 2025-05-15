use std::sync::Arc;

use anyhow::{Result, anyhow};
use gpui::{AsyncApp, Context, Entity, Task, WindowHandle};
use language::{Buffer, Language};

use crate::context::{
    AgentContext, AgentContextHandle, AgentContextKey, ContextLoadResult, ContextSummary, 
    LoadedContext, load_context,
};
use crate::context_store::ContextStore;
use crate::conversation::{MessageCrease, MessageId};

/// Service for handling contexts in conversations
pub trait ContextService {
    /// Load context for a conversation message
    fn load_context(
        &self,
        keys: Vec<AgentContextKey>,
        window: Option<WindowHandle>,
        cx: &mut AsyncApp,
    ) -> Task<ContextLoadResult>;
    
    /// Create context from buffers
    fn create_context_from_buffer(
        &self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Result<AgentContext>;
    
    /// Create a message crease from a context
    fn create_crease_from_context(
        &self,
        context: &AgentContext,
        range: std::ops::Range<usize>,
        cx: &App,
    ) -> MessageCrease;
    
    /// Get a summary of available contexts
    fn get_context_summaries(&self, cx: &App) -> Vec<ContextSummary>;
    
    /// Add a context to the store
    fn add_context(&mut self, context: AgentContext, cx: &mut Context<Self>) -> Result<()>;
    
    /// Remove a context from the store
    fn remove_context(&mut self, key: &AgentContextKey, cx: &mut Context<Self>) -> Result<()>;
}

/// Default implementation of the ContextService
pub struct DefaultContextService {
    context_store: Entity<ContextStore>,
}

impl DefaultContextService {
    pub fn new(context_store: Entity<ContextStore>) -> Self {
        Self { context_store }
    }
}

impl ContextService for DefaultContextService {
    fn load_context(
        &self,
        keys: Vec<AgentContextKey>,
        window: Option<WindowHandle>,
        cx: &mut AsyncApp,
    ) -> Task<ContextLoadResult> {
        let context_store = self.context_store.clone();
        
        cx.spawn(|cx| async move {
            let mut contexts = Vec::new();
            
            for key in keys {
                if let Some(ctx) = context_store.read(&cx).get(&key) {
                    contexts.push(ctx);
                }
            }
            
            load_context(&contexts, window, &cx).await
        })
    }
    
    fn create_context_from_buffer(
        &self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Result<AgentContext> {
        let buffer_text = buffer.read(cx).text();
        
        // Create a context from the buffer text
        // This is a simplified version - in a real implementation,
        // we would extract more information from the buffer
        let language = match buffer.read(cx).language() {
            Some(lang) => lang.name().to_string(),
            None => "plain_text".to_string(),
        };
        
        let path = match buffer.read(cx).file() {
            Some(file) => file.path().to_string_lossy().to_string(),
            None => "unnamed".to_string(),
        };
        
        let context = AgentContext {
            key: AgentContextKey::Buffer(buffer.entity_id()),
            title: path.clone(),
            language,
            path: Some(path),
            text: buffer_text.to_string(),
            metadata: Default::default(),
        };
        
        Ok(context)
    }
    
    fn create_crease_from_context(
        &self,
        context: &AgentContext,
        range: std::ops::Range<usize>,
        cx: &App,
    ) -> MessageCrease {
        // Create a crease for the message editor
        MessageCrease {
            range,
            metadata: editor::display_map::CreaseMetadata {
                // Set the metadata based on the context
                title: Some(context.title.clone()),
                language: Some(context.language.clone()),
                path: context.path.clone(),
                ..Default::default()
            },
            context: Some(AgentContextHandle::new(
                context.clone(),
                None,
                vec![self.context_store.clone()],
            )),
        }
    }
    
    fn get_context_summaries(&self, cx: &App) -> Vec<ContextSummary> {
        self.context_store.read(cx).all_context_summaries(cx)
    }
    
    fn add_context(&mut self, context: AgentContext, cx: &mut Context<Self>) -> Result<()> {
        self.context_store.update(cx, |store, cx| {
            store.add(context, cx);
        });
        
        Ok(())
    }
    
    fn remove_context(&mut self, key: &AgentContextKey, cx: &mut Context<Self>) -> Result<()> {
        self.context_store.update(cx, |store, cx| {
            store.remove(key, cx);
        });
        
        Ok(())
    }
}

/// Bridge between old and new context implementations
pub struct ContextAdapter {
    service: Arc<dyn ContextService>,
}

impl ContextAdapter {
    pub fn new(service: Arc<dyn ContextService>) -> Self {
        Self { service }
    }
    
    pub fn load_context_for_conversation(
        &self,
        keys: Vec<AgentContextKey>,
        window: Option<WindowHandle>,
        cx: &mut AsyncApp,
    ) -> Task<ContextLoadResult> {
        self.service.load_context(keys, window, cx)
    }
} 