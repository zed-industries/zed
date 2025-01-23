use gpui::{
    actions, Action, AppContext, EventEmitter, FocusHandle, FocusableView,
    KeyBindingContextPredicate, KeyContext, Keystroke, MouseButton, Render, Subscription,
};
use itertools::Itertools;
use serde_json::json;
use settings::get_key_equivalents;
use ui::{
    div, h_flex, px, v_flex, ButtonCommon, Clickable, FluentBuilder, InteractiveElement, Label,
    LabelCommon, LabelSize, ParentElement, SharedString, StatefulInteractiveElement, Styled,
    ViewContext, VisualContext, WindowContext,
};
use ui::{Button, ButtonStyle};
use workspace::Item;
use workspace::Workspace;

actions!(debug, [OpenKeyContextView]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &OpenKeyContextView, cx| {
            let key_context_view = cx.new_view(KeyContextView::new);
            workspace.add_item_to_active_pane(Box::new(key_context_view), None, true, cx)
        });
    })
    .detach();
}

struct KeyContextView {
    pending_keystrokes: Option<Vec<Keystroke>>,
    last_keystrokes: Option<SharedString>,
    last_possibilities: Vec<(SharedString, SharedString, Option<bool>)>,
    context_stack: Vec<KeyContext>,
    focus_handle: FocusHandle,
    _subscriptions: [Subscription; 2],
}

impl KeyContextView {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let sub1 = cx.observe_keystrokes(|this, e, cx| {
            let mut pending = this.pending_keystrokes.take().unwrap_or_default();
            pending.push(e.keystroke.clone());
            let mut possibilities = cx.all_bindings_for_input(&pending);
            possibilities.reverse();
            this.context_stack = cx.context_stack();
            this.last_keystrokes = Some(
                json!(pending.iter().map(|p| p.unparse()).join(" "))
                    .to_string()
                    .into(),
            );
            this.last_possibilities = possibilities
                .into_iter()
                .map(|binding| {
                    let match_state = if let Some(predicate) = binding.predicate() {
                        if this.matches(&predicate) {
                            if this.action_matches(&e.action, binding.action()) {
                                Some(true)
                            } else {
                                Some(false)
                            }
                        } else {
                            None
                        }
                    } else {
                        if this.action_matches(&e.action, binding.action()) {
                            Some(true)
                        } else {
                            Some(false)
                        }
                    };
                    let predicate = if let Some(predicate) = binding.predicate() {
                        format!("{}", predicate)
                    } else {
                        "".to_string()
                    };
                    let mut name = binding.action().name();
                    if name == "zed::NoAction" {
                        name = "(null)"
                    }

                    (
                        name.to_owned().into(),
                        json!(predicate).to_string().into(),
                        match_state,
                    )
                })
                .collect();
        });
        let sub2 = cx.observe_pending_input(|this, cx| {
            this.pending_keystrokes = cx
                .pending_input_keystrokes()
                .map(|k| k.iter().cloned().collect());
            if this.pending_keystrokes.is_some() {
                this.last_keystrokes.take();
            }
            cx.notify();
        });

        Self {
            context_stack: Vec::new(),
            pending_keystrokes: None,
            last_keystrokes: None,
            last_possibilities: Vec::new(),
            focus_handle: cx.focus_handle(),
            _subscriptions: [sub1, sub2],
        }
    }
}

impl EventEmitter<()> for KeyContextView {}

impl FocusableView for KeyContextView {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
impl KeyContextView {
    fn set_context_stack(&mut self, stack: Vec<KeyContext>, cx: &mut ViewContext<Self>) {
        self.context_stack = stack;
        cx.notify()
    }

    fn matches(&self, predicate: &KeyBindingContextPredicate) -> bool {
        let mut stack = self.context_stack.clone();
        while !stack.is_empty() {
            if predicate.eval(&stack) {
                return true;
            }
            stack.pop();
        }
        false
    }

    fn action_matches(&self, a: &Option<Box<dyn Action>>, b: &dyn Action) -> bool {
        if let Some(last_action) = a {
            last_action.partial_eq(b)
        } else {
            b.name() == "zed::NoAction"
        }
    }
}

impl Item for KeyContextView {
    type Event = ();

    fn to_item_events(_: &Self::Event, _: impl FnMut(workspace::item::ItemEvent)) {}

    fn tab_content_text(&self, _cx: &WindowContext) -> Option<SharedString> {
        Some("Keyboard Context".into())
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        cx: &mut ViewContext<Self>,
    ) -> Option<gpui::View<Self>>
    where
        Self: Sized,
    {
        Some(cx.new_view(Self::new))
    }
}

impl Render for KeyContextView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl ui::IntoElement {
        use itertools::Itertools;
        let key_equivalents = get_key_equivalents(cx.keyboard_layout());
        v_flex()
            .id("key-context-view")
            .overflow_scroll()
            .size_full()
            .max_h_full()
            .pt_4()
            .pl_4()
            .track_focus(&self.focus_handle)
            .key_context("KeyContextView")
            .on_mouse_up_out(
                MouseButton::Left,
                cx.listener(|this, _, cx| {
                    this.last_keystrokes.take();
                    this.set_context_stack(cx.context_stack(), cx);
                }),
            )
            .on_mouse_up_out(
                MouseButton::Right,
                cx.listener(|_, _, cx| {
                    cx.defer(|this, cx| {
                        this.last_keystrokes.take();
                        this.set_context_stack(cx.context_stack(), cx);
                    });
                }),
            )
            .child(Label::new("Keyboard Context").size(LabelSize::Large))
            .child(Label::new("This view lets you determine the current context stack for creating custom key bindings in Zed. When a keyboard shortcut is triggered, it also shows all the possible contexts it could have triggered in, and which one matched."))
            .child(
                h_flex()
                    .mt_4()
                    .gap_4()
                    .child(
                        Button::new("default", "Open Documentation")
                            .style(ButtonStyle::Filled)
                            .on_click(|_, cx| cx.open_url("https://zed.dev/docs/key-bindings")),
                    )
                    .child(
                        Button::new("default", "View default keymap")
                            .style(ButtonStyle::Filled)
                            .key_binding(ui::KeyBinding::for_action(
                                &zed_actions::OpenDefaultKeymap,
                                cx,
                            ))
                            .on_click(|_, cx| {
                                cx.dispatch_action(workspace::SplitRight.boxed_clone());
                                cx.dispatch_action(zed_actions::OpenDefaultKeymap.boxed_clone());
                            }),
                    )
                    .child(
                        Button::new("default", "Edit your keymap")
                            .style(ButtonStyle::Filled)
                            .key_binding(ui::KeyBinding::for_action(&zed_actions::OpenKeymap, cx))
                            .on_click(|_, cx| {
                                cx.dispatch_action(workspace::SplitRight.boxed_clone());
                                cx.dispatch_action(zed_actions::OpenKeymap.boxed_clone());
                            }),
                    ),
            )
            .child(
                Label::new("Current Context Stack")
                    .size(LabelSize::Large)
                    .mt_8(),
            )
            .children({
                cx.context_stack().iter().enumerate().map(|(i, context)| {
                    let primary = context.primary().map(|e| e.key.clone()).unwrap_or_default();
                    let secondary = context
                        .secondary()
                        .map(|e| {
                            if let Some(value) = e.value.as_ref() {
                                format!("{}={}", e.key, value)
                            } else {
                                e.key.to_string()
                            }
                        })
                        .join(" ");
                    Label::new(format!("{} {}", primary, secondary)).ml(px(12. * (i + 1) as f32))
                })
            })
            .child(Label::new("Last Keystroke").mt_4().size(LabelSize::Large))
            .when_some(self.pending_keystrokes.as_ref(), |el, keystrokes| {
                el.child(
                    Label::new(format!(
                        "Waiting for more input: {}",
                        keystrokes.iter().map(|k| k.unparse()).join(" ")
                    ))
                    .ml(px(12.)),
                )
            })
            .when_some(self.last_keystrokes.as_ref(), |el, keystrokes| {
                el.child(Label::new(format!("Typed: {}", keystrokes)).ml_4())
                    .children(
                        self.last_possibilities
                            .iter()
                            .map(|(name, predicate, state)| {
                                let (text, color) = match state {
                                    Some(true) => ("(match)", ui::Color::Success),
                                    Some(false) => ("(low precedence)", ui::Color::Hint),
                                    None => ("(no match)", ui::Color::Error),
                                };
                                h_flex()
                                    .gap_2()
                                    .ml_8()
                                    .child(div().min_w(px(200.)).child(Label::new(name.clone())))
                                    .child(Label::new(predicate.clone()))
                                    .child(Label::new(text).color(color))
                            }),
                    )
            })
            .when_some(key_equivalents, |el, key_equivalents| {
                el.child(Label::new("Key Equivalents").mt_4().size(LabelSize::Large))
                    .child(Label::new("Shortcuts defined using some characters have been remapped so that shortcuts can be typed without holding option."))
                    .children(
                        key_equivalents
                            .iter()
                            .sorted()
                            .map(|(key, equivalent)| {
                                Label::new(format!("cmd-{} => cmd-{}", key, equivalent)).ml_8()
                            }),
                    )
            })
    }
}
