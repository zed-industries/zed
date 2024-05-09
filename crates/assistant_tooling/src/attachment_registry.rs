use crate::ProjectContext;
use anyhow::{anyhow, Result};
use collections::HashMap;
use futures::future::join_all;
use gpui::{AnyView, Render, Task, View, WindowContext};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::value::RawValue;
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

pub trait AttachmentOutput {
    fn generate(&self, project: &mut ProjectContext, cx: &mut WindowContext) -> String;
}

pub trait LanguageModelAttachment {
    type Output: DeserializeOwned + Serialize + 'static;
    type View: Render + AttachmentOutput;

    fn name(&self) -> Arc<str>;
    fn run(&self, cx: &mut WindowContext) -> Task<Result<Self::Output>>;
    fn view(&self, output: Result<Self::Output>, cx: &mut WindowContext) -> View<Self::View>;
}

/// A collected attachment from running an attachment tool
pub struct UserAttachment {
    pub view: AnyView,
    name: Arc<str>,
    serialized_output: Result<Box<RawValue>, String>,
    generate_fn: fn(AnyView, &mut ProjectContext, cx: &mut WindowContext) -> String,
}

#[derive(Serialize, Deserialize)]
pub struct SavedUserAttachment {
    name: Arc<str>,
    serialized_output: Result<Box<RawValue>, String>,
}

/// Internal representation of an attachment tool to allow us to treat them dynamically
struct RegisteredAttachment {
    name: Arc<str>,
    enabled: AtomicBool,
    call: Box<dyn Fn(&mut WindowContext) -> Task<Result<UserAttachment>>>,
    deserialize: Box<dyn Fn(&SavedUserAttachment, &mut WindowContext) -> Result<UserAttachment>>,
}

impl AttachmentRegistry {
    pub fn new() -> Self {
        Self {
            registered_attachments: HashMap::default(),
        }
    }

    pub fn register<A: LanguageModelAttachment + 'static>(&mut self, attachment: A) {
        let attachment = Arc::new(attachment);

        let call = Box::new({
            let attachment = attachment.clone();
            move |cx: &mut WindowContext| {
                let result = attachment.run(cx);
                let attachment = attachment.clone();
                cx.spawn(move |mut cx| async move {
                    let result: Result<A::Output> = result.await;
                    let serialized_output =
                        result
                            .as_ref()
                            .map_err(ToString::to_string)
                            .and_then(|output| {
                                Ok(RawValue::from_string(
                                    serde_json::to_string(output).map_err(|e| e.to_string())?,
                                )
                                .unwrap())
                            });

                    let view = cx.update(|cx| attachment.view(result, cx))?;

                    Ok(UserAttachment {
                        name: attachment.name(),
                        view: view.into(),
                        generate_fn: generate::<A>,
                        serialized_output,
                    })
                })
            }
        });

        let deserialize = Box::new({
            let attachment = attachment.clone();
            move |saved_attachment: &SavedUserAttachment, cx: &mut WindowContext| {
                let serialized_output = saved_attachment.serialized_output.clone();
                let output = match &serialized_output {
                    Ok(serialized_output) => {
                        Ok(serde_json::from_str::<A::Output>(serialized_output.get())?)
                    }
                    Err(error) => Err(anyhow!("{error}")),
                };
                let view = attachment.view(output, cx).into();

                Ok(UserAttachment {
                    name: saved_attachment.name.clone(),
                    view,
                    serialized_output,
                    generate_fn: generate::<A>,
                })
            }
        });

        self.registered_attachments.insert(
            TypeId::of::<A>(),
            RegisteredAttachment {
                name: attachment.name(),
                call,
                deserialize,
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

    pub fn serialize_user_attachment(
        &self,
        user_attachment: &UserAttachment,
    ) -> SavedUserAttachment {
        SavedUserAttachment {
            name: user_attachment.name.clone(),
            serialized_output: user_attachment.serialized_output.clone(),
        }
    }

    pub fn deserialize_user_attachment(
        &self,
        saved_user_attachment: SavedUserAttachment,
        cx: &mut WindowContext,
    ) -> Result<UserAttachment> {
        if let Some(registered_attachment) = self
            .registered_attachments
            .values()
            .find(|attachment| attachment.name == saved_user_attachment.name)
        {
            (registered_attachment.deserialize)(&saved_user_attachment, cx)
        } else {
            Err(anyhow!(
                "no attachment tool for name {}",
                saved_user_attachment.name
            ))
        }
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
