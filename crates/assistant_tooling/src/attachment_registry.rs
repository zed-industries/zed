use crate::{ProjectContext, ToolOutput};
use anyhow::{anyhow, Result};
use collections::HashMap;
use futures::future::join_all;
use gpui::{AnyView, Render, Task, View, WindowContext};
use std::{
    any::TypeId,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};
use util::ResultExt as _;

pub struct AttachmentRegistry {
    registered_attachments: HashMap<TypeId, RegisteredAttachment>,
}

pub trait LanguageModelAttachment {
    type Output: 'static;
    type View: Render + ToolOutput;

    fn run(&self, cx: &mut WindowContext) -> Task<Result<Self::Output>>;

    fn view(output: Result<Self::Output>, cx: &mut WindowContext) -> View<Self::View>;
}

/// A collected attachment from running an attachment tool
pub struct UserAttachment {
    pub view: AnyView,
    generate_fn: fn(AnyView, &mut ProjectContext, cx: &mut WindowContext) -> String,
}

/// Internal representation of an attachment tool to allow us to treat them dynamically
struct RegisteredAttachment {
    enabled: AtomicBool,
    call: Box<dyn Fn(&mut WindowContext) -> Task<Result<UserAttachment>>>,
}

impl AttachmentRegistry {
    pub fn new() -> Self {
        Self {
            registered_attachments: HashMap::default(),
        }
    }

    pub fn register<A: LanguageModelAttachment + 'static>(&mut self, attachment: A) {
        let call = Box::new(move |cx: &mut WindowContext| {
            let result = attachment.run(cx);

            cx.spawn(move |mut cx| async move {
                let result: Result<A::Output> = result.await;
                let view = cx.update(|cx| A::view(result, cx))?;

                Ok(UserAttachment {
                    view: view.into(),
                    generate_fn: generate::<A>,
                })
            })
        });

        self.registered_attachments.insert(
            TypeId::of::<A>(),
            RegisteredAttachment {
                call,
                enabled: AtomicBool::new(true),
            },
        );
        return;

        fn generate<T: LanguageModelAttachment>(
            view: AnyView,
            project: &mut ProjectContext,
            cx: &mut WindowContext,
        ) -> String {
            view.downcast::<T::View>()
                .unwrap()
                .update(cx, |view, cx| T::View::generate(view, project, cx))
        }
    }

    pub fn set_attachment_tool_enabled<A: LanguageModelAttachment + 'static>(
        &self,
        is_enabled: bool,
    ) {
        if let Some(attachment) = self.registered_attachments.get(&TypeId::of::<A>()) {
            attachment.enabled.store(is_enabled, SeqCst);
        }
    }

    pub fn is_attachment_tool_enabled<A: LanguageModelAttachment + 'static>(&self) -> bool {
        if let Some(attachment) = self.registered_attachments.get(&TypeId::of::<A>()) {
            attachment.enabled.load(SeqCst)
        } else {
            false
        }
    }

    pub fn call<A: LanguageModelAttachment + 'static>(
        &self,
        cx: &mut WindowContext,
    ) -> Task<Result<UserAttachment>> {
        let Some(attachment) = self.registered_attachments.get(&TypeId::of::<A>()) else {
            return Task::ready(Err(anyhow!("no attachment tool")));
        };

        (attachment.call)(cx)
    }

    pub fn call_all_attachment_tools(
        self: Arc<Self>,
        cx: &mut WindowContext<'_>,
    ) -> Task<Result<Vec<UserAttachment>>> {
        let this = self.clone();
        cx.spawn(|mut cx| async move {
            let attachment_tasks = cx.update(|cx| {
                let mut tasks = Vec::new();
                for attachment in this
                    .registered_attachments
                    .values()
                    .filter(|attachment| attachment.enabled.load(SeqCst))
                {
                    tasks.push((attachment.call)(cx))
                }

                tasks
            })?;

            let attachments = join_all(attachment_tasks.into_iter()).await;

            Ok(attachments
                .into_iter()
                .filter_map(|attachment| attachment.log_err())
                .collect())
        })
    }
}

impl UserAttachment {
    pub fn generate(&self, output: &mut ProjectContext, cx: &mut WindowContext) -> Option<String> {
        let result = (self.generate_fn)(self.view.clone(), output, cx);
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}
