use crate::{ManageProfiles, ToggleEndpointSelector, ToggleProfileSelector};
use agent::{
    Thread,
    agent_profile::{AgentProfile, AvailableProfiles},
};
use agent_settings::{AgentDockPosition, AgentProfileId, AgentSettings, builtin_profiles};
use anyhow::Context as AnyhowResultContext;
use fs::Fs;
use gpui::{Action, Empty, Entity, FocusHandle, Subscription, Task, prelude::*};
use language_model::{
    ConfiguredModel, LanguageModel, LanguageModelEndpoint, LanguageModelId, LanguageModelRegistry,
};
use log::{debug, info};
use settings::{Settings as _, SettingsStore, update_settings_file};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use ui::{
    Chip, ContextMenu, ContextMenuEntry, ContextMenuItem, DocumentationSide, IconButtonShape,
    PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*,
};
use util::{ResultExt, size::format_file_size};

pub struct EndpointSelector {
    endpoints: HashMap<LanguageModelId, Arc<Vec<LanguageModelEndpoint>>>,
    /// None means default (not specified)
    selected_endpoint_idx: usize,
    current_model: Option<LanguageModelId>,

    fs: Arc<dyn Fs>,
    thread: Entity<Thread>,
    menu_handle: PopoverMenuHandle<ContextMenu>,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
    get_endpoint_task: Option<Task<()>>,
}

fn get_model(
    thread: &Entity<Thread>,
    cx: &mut Context<EndpointSelector>,
) -> Option<Arc<dyn LanguageModel>> {
    thread
        .read(cx)
        .configured_model()
        .or_else(|| {
            let model_registry = LanguageModelRegistry::read_global(cx);
            model_registry.default_model()
        })
        .map(|m| m.model.clone())
}

impl EndpointSelector {
    pub fn new(
        fs: Arc<dyn Fs>,
        thread: Entity<Thread>,
        focus_handle: FocusHandle,
        cx: &mut Context<Self>,
    ) -> Self {
        let model_change_subscription = cx.observe_global::<SettingsStore>(move |this, cx| {
            // 1. Get current model
            let (Some(new_model), previous_model) = ({
                let curr = get_model(&this.thread, cx);
                let prev_model = this.current_model.clone();

                let _curr_model = curr.clone().map(|x| x.id());
                info!("Model changed from {prev_model:?} to {_curr_model:?}");

                (curr, prev_model)
            }) else {
                return;
            };
            if Some(new_model.id()) == previous_model {
                return;
            }
            this.current_model = Some(new_model.id());

            if this.endpoints.contains_key(&new_model.id()) == false {
                let task = cx.spawn(async move |this, cx| {
                    // 2. Get endpoints
                    let Some(endpoints) = new_model
                        .endpoints(cx)
                        .await
                        .context("Getting OpenRouter model endpoints")
                        .log_err()
                    else {
                        return;
                    };

                    // 3. Write to cache
                    let model_id = new_model.id();
                    info!("Done fetching endpoints for {model_id:?}");
                    this.update(cx, |this, cx| {
                        this.endpoints
                            .insert(new_model.id().clone(), Arc::new(endpoints));
                        cx.notify();
                    })
                    .log_err();
                });
                this.get_endpoint_task.replace(task);
            }
        });

        Self {
            endpoints: HashMap::new(),
            current_model: get_model(&thread, cx).map(|m| m.id()),
            selected_endpoint_idx: 0,
            fs,
            thread,
            focus_handle,
            menu_handle: PopoverMenuHandle::default(),
            _subscriptions: vec![model_change_subscription],
            get_endpoint_task: None,
        }
    }

    fn build_endpoints_menu(
        &self,
        endpoints: Arc<Vec<LanguageModelEndpoint>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let entity = cx.entity().clone();
        ContextMenu::build(window, cx, |mut menu, window, cx| {
            for (idx, endpoint) in endpoints.iter().enumerate() {
                menu = menu.item(self.build_menu_entry(endpoint, idx, entity.clone()));
            }

            menu
        })
    }

    fn build_menu_entry(
        &self,
        endpoint: &LanguageModelEndpoint,
        idx: usize,
        ent: Entity<Self>,
    ) -> ContextMenuItem {
        // todo: documentation aside?

        let handler = move |_window: &mut Window, cx: &mut App| {
            let _ = ent.update(cx, |this, cx| {
                this.selected_endpoint_idx = idx;
                cx.notify();
            });
        };

        match endpoint.clone() {
            LanguageModelEndpoint::Default => self.build_default_entry(handler, idx),
            LanguageModelEndpoint::Specified {
                name,
                context_length,
                quantization,
                availability,
            } => self.build_endpoint_entry(
                handler,
                name,
                idx,
                context_length,
                quantization,
                availability,
            ),
        }
    }

    fn build_default_entry(
        &self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
        idx: usize,
    ) -> ContextMenuItem {
        let selected = self.selected_endpoint_idx;
        ContextMenuItem::custom_entry(
            move |_window, _cx| {
                h_flex()
                    .gap_1()
                    .child(Label::new("Default"))
                    .child(
                        Label::new("Zed will not request for any specific provider")
                            .size(LabelSize::Small),
                    )
                    .when(idx == selected, |p| {
                        p.child(div().ml_auto().child(Icon::new(IconName::Check)))
                    })
                    .into_any_element()
            },
            handler,
            None,
        )
    }

    fn build_endpoint_entry(
        &self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
        name: String,
        idx: usize,
        context_length: Option<u64>,
        quantization: Option<String>,
        availability: Option<f32>,
    ) -> ContextMenuItem {
        let name = SharedString::new(name);
        let quantization = quantization.clone();
        let selected = self.selected_endpoint_idx;

        ContextMenuItem::custom_entry(
            move |_window, _cx| {
                h_flex()
                    .gap_1()
                    .child(Label::new(name.clone()))
                    .when_some(context_length, |s, ctx_len| {
                        s.child(Chip::new(format_file_size(ctx_len, false)))
                    })
                    .when_some(quantization.clone(), |s, quant| s.child(Chip::new(quant)))
                    .when_some(availability, |s, avail| {
                        s.child(Chip::new(format!("{:.2}% up", avail)))
                    })
                    .into_any_element()
            },
            handler,
            None,
        )
    }

    pub fn menu_handle(&self) -> PopoverMenuHandle<ContextMenu> {
        self.menu_handle.clone()
    }
}

impl Render for EndpointSelector {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(configured_model) = self.thread.read(cx).configured_model().or_else(|| {
            let model_registry = LanguageModelRegistry::read_global(cx);
            model_registry.default_model()
        }) else {
            return Empty.into_any_element();
        };

        let model = configured_model.model;
        let selected_endpoint = {
            let endpoint_name: Option<String> = (|| {
                Some(
                    self.endpoints
                        .get(&model.id())?
                        .get(self.selected_endpoint_idx)?
                        .name()
                        .to_string(),
                )
            })();

            endpoint_name.unwrap_or("Default".to_string())
        };

        if model.supports_different_endpoints() {
            match self.endpoints.get(&model.id()) {
                Some(endpoints) => {
                    let this = cx.entity().clone();
                    let trigger_button = Button::new("endpoint-selector-model", selected_endpoint)
                        .label_size(LabelSize::Small)
                        .color(Color::Muted)
                        .icon(IconName::ChevronDown)
                        .icon_size(IconSize::XSmall)
                        .icon_position(IconPosition::End)
                        .icon_color(Color::Muted);
                    let endpoints = endpoints.clone();

                    PopoverMenu::new("endpoint-selector")
                        .trigger_with_tooltip(trigger_button, {
                            let focus_handle = self.focus_handle.clone();
                            move |window, cx| {
                                Tooltip::for_action_in(
                                    "Select model provider",
                                    &ToggleEndpointSelector,
                                    &focus_handle,
                                    window,
                                    cx,
                                )
                            }
                        })
                        .anchor(gpui::Corner::BottomRight)
                        .with_handle(self.menu_handle.clone())
                        .menu(move |window, cx| {
                            let endpoints = endpoints.clone();
                            Some(this.update(cx, move |this, cx| {
                                this.build_endpoints_menu(endpoints, window, cx)
                            }))
                        })
                        .into_any_element()
                }
                None => Button::new("loading-endpoints", "Loading providers...")
                    .disabled(true)
                    .label_size(LabelSize::Small)
                    .color(Color::Muted)
                    .tooltip(Tooltip::text(
                        "Loading available model providers from OpenRouter",
                    ))
                    .into_any_element(),
            }
        } else {
            Empty.into_any_element()
        }
    }
}
