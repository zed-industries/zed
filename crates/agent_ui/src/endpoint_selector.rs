use crate::ToggleEndpointSelector;
use agent::Thread;
use anyhow::Context as AnyhowResultContext;
use gpui::{Empty, Entity, FocusHandle, Subscription, Task, prelude::*};
use language_model::LanguageModel;
use language_model::{LanguageModelEndpoint, LanguageModelId, LanguageModelRegistry};
use settings::SettingsStore;
use std::{collections::HashMap, sync::Arc};
use ui::{Chip, ContextMenu, ContextMenuItem, PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*};
use util::{ResultExt, size::format_file_size};

pub struct EndpointSelector {
    endpoints: HashMap<LanguageModelId, Arc<Vec<LanguageModelEndpoint>>>,
    /// None means default (not specified)
    selected_endpoint_idx: usize,
    current_model: Option<LanguageModelId>,

    thread: Entity<Thread>,
    menu_handle: PopoverMenuHandle<ContextMenu>,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
    get_endpoint_task: Option<Task<Option<()>>>,
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
    pub fn new(thread: Entity<Thread>, focus_handle: FocusHandle, cx: &mut Context<Self>) -> Self {
        let model_change_subscription =
            cx.observe_global::<SettingsStore>(move |this, cx| this.on_model_change(cx));

        Self {
            endpoints: HashMap::new(),
            current_model: get_model(&thread, cx).map(|m| m.id()),
            selected_endpoint_idx: 0,
            thread,
            focus_handle,
            menu_handle: PopoverMenuHandle::default(),
            _subscriptions: vec![model_change_subscription],
            get_endpoint_task: None,
        }
    }

    /// Build the selector menu
    fn build_endpoints_menu(
        &self,
        endpoints: Arc<Vec<LanguageModelEndpoint>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let entity = cx.entity().clone();
        ContextMenu::build(window, cx, |mut menu, _window, _cx| {
            for (idx, endpoint) in endpoints.iter().enumerate() {
                menu = menu.item(self.build_menu_entry(endpoint, idx, entity.clone()));
            }

            menu
        })
    }

    /// Build an entry of the provider menu,
    /// dispatching to [`build_default_entry`] or [`build_endpoint_entry`]
    fn build_menu_entry(
        &self,
        endpoint: &LanguageModelEndpoint,
        idx: usize,
        ent: Entity<Self>,
    ) -> ContextMenuItem {
        // todo: documentation aside?

        let handler = {
            let endpoint2 = endpoint.clone();
            let thread2 = self.thread.clone();
            move |_window: &mut Window, cx: &mut App| {
                ent.update(cx, |this, cx| {
                    this.selected_endpoint_idx = idx;
                    cx.notify();
                });

                thread2.update(cx, |this, _cx| {
                    this.set_endpoint(endpoint2.clone());
                })
            }
        };

        let endpoint = endpoint.clone();
        match endpoint {
            LanguageModelEndpoint::Default => self.build_default_entry(handler, idx),
            LanguageModelEndpoint::Specified {
                name,
                context_length,
                quantization,
                throughput,
                latency,
                input_price,
                output_price,
                ..
            } => {
                let a = self.build_endpoint_entry(
                    handler,
                    name,
                    idx,
                    context_length,
                    quantization,
                    throughput,
                    latency,
                    input_price,
                    output_price,
                );
                return a;
            }
        }
    }

    fn build_default_entry(
        &self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
        idx: usize,
    ) -> ContextMenuItem {
        let selected = self.selected_endpoint_idx == idx;
        ContextMenuItem::custom_entry(
            move |_window, _cx| {
                h_flex()
                    .gap_1()
                    .w_full()
                    .child(
                        v_flex().gap_0p5().child(Label::new("Default")).child(
                            Label::new("Zed will not request for any specific provider")
                                .size(LabelSize::Small),
                        ),
                    )
                    .when(selected, Self::check_tick)
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
        throughput: Option<f32>,
        latency: Option<f32>,
        input_price: Option<f32>,
        output_price: Option<f32>,
    ) -> ContextMenuItem {
        let name = SharedString::new(name);
        let quantization = quantization.clone();
        let selected = self.selected_endpoint_idx == idx;
        ContextMenuItem::custom_entry(
            move |_window, _cx| {
                h_flex()
                    .gap_1()
                    .w_full()
                    .child(
                        v_flex().gap_0p5().child(Label::new(name.clone())).child(
                            h_flex()
                                .gap_1()
                                .when_some(throughput, |s, throughput| {
                                    s.child(Chip::new(format!("{:.0} tok/s", throughput)))
                                })
                                .when_some(latency, |s, latency| {
                                    s.child(Chip::new(format!("{:.1} s", latency)))
                                })
                                .when_some(input_price, |s, ip| {
                                    s.child(Chip::new(format!("in ${:.2}", ip)))
                                })
                                .when_some(output_price, |s, op| {
                                    s.child(Chip::new(format!("out ${:.2}", op)))
                                })
                                .when_some(context_length, |s, ctx_len| {
                                    let ctx_len_as_filesize = format_file_size(ctx_len, false);
                                    let string_length = ctx_len_as_filesize.len();
                                    // Strip "iB" from "KiB"/"MiB"
                                    let ctx_len = &ctx_len_as_filesize[..string_length - 2];
                                    s.child(Chip::new(format!("ctx {ctx_len}")))
                                })
                                .when_some(quantization.clone(), |s, quant| {
                                    s.child(Chip::new(quant))
                                }),
                        ),
                    )
                    .when(selected, Self::check_tick)
                    .into_any_element()
            },
            handler,
            None,
        )
    }

    /// Add a tick to the rightmost
    fn check_tick<T: ParentElement>(parent: T) -> T {
        parent.child(
            h_flex().w_full().justify_end().child(
                Icon::new(IconName::Check)
                    .size(IconSize::Small)
                    .color(Color::Accent),
            ),
        )
    }

    fn on_model_change(&mut self, cx: &mut Context<Self>) {
        let (Some(new_model), previous_model) =
            (get_model(&self.thread, cx), self.current_model.clone())
        else {
            return;
        };
        if Some(new_model.id()) == previous_model {
            return;
        }
        self.current_model = Some(new_model.id());
        self.selected_endpoint_idx = 0;

        if self.endpoints.contains_key(&new_model.id()) == false {
            let task = cx.spawn(async move |this, cx| {
                let endpoints = new_model
                    .endpoints(cx)
                    .await
                    .context("Getting OpenRouter model endpoints")
                    .log_err()?;

                this.update(cx, |this, cx| {
                    this.endpoints
                        .insert(new_model.id().clone(), Arc::new(endpoints));
                    cx.notify();
                })
                .log_err()?;

                Some(())
            });
            self.get_endpoint_task.replace(task);
        }
    }

    pub fn menu_handle(&self) -> PopoverMenuHandle<ContextMenu> {
        self.menu_handle.clone()
    }
}

impl Render for EndpointSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(model) = get_model(&self.thread, cx) else {
            return Empty.into_any_element();
        };

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
                None => {
                    // Sometimes after startup, the settings change subscriber only
                    // receives some model changes from `None` to `None`, but there's
                    // actually a configured model.
                    if matches!(self.get_endpoint_task, None) {
                        self.on_model_change(cx);
                    }

                    Button::new("loading-endpoints", "Loading providers...")
                        .disabled(true)
                        .label_size(LabelSize::Small)
                        .color(Color::Muted)
                        .tooltip(Tooltip::text(
                            "Loading available model providers from OpenRouter",
                        ))
                        .into_any_element()
                }
            }
        } else {
            Empty.into_any_element()
        }
    }
}
