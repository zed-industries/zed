use gpui::{
    Action, App, AppContext as _, Entity, EventEmitter, FocusHandle, Focusable,
    KeyBindingContextPredicate, KeyContext, Keystroke, MouseButton, Render, Subscription, actions,
};
use itertools::Itertools;
use serde_json::json;
use settings::get_key_equivalents;
use ui::{Button, ButtonStyle};
use ui::{
    ButtonCommon, Clickable, Context, FluentBuilder, InteractiveElement, Label, LabelCommon,
    LabelSize, ParentElement, SharedString, StatefulInteractiveElement, Styled, Window, div,
    h_flex, px, v_flex,
};
use workspace::{Item, SplitDirection, Workspace};

actions!(debug, [OpenKeyContextView]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &OpenKeyContextView, window, cx| {
            let key_context_view = cx.new(|cx| KeyContextView::new(window, cx));
            workspace.split_item(
                SplitDirection::Right,
                Box::new(key_context_view),
                window,
                cx,
            )
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
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let sub1 = cx.observe_keystrokes(|this, e, _, cx| {
            let mut pending = this.pending_keystrokes.take().unwrap_or_default();
            pending.push(e.keystroke.clone());
            let mut possibilities = cx.all_bindings_for_input(&pending);
            possibilities.reverse();
            this.last_keystrokes = Some(
                json!(pending.iter().map(|p| p.unparse()).join(" "))
                    .to_string()
                    .into(),
            );
            this.context_stack = e.context_stack.clone();
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
            cx.notify();
        });
        let sub2 = cx.observe_pending_input(window, |this, window, cx| {
            this.pending_keystrokes = window
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

impl Focusable for KeyContextView {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
impl KeyContextView {
    fn set_context_stack(&mut self, stack: Vec<KeyContext>, cx: &mut Context<Self>) {
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

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Keyboard Context".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>>
    where
        Self: Sized,
    {
        Some(cx.new(|cx| KeyContextView::new(window, cx)))
    }
}

impl Render for KeyContextView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        use itertools::Itertools;
        let key_equivalents = get_key_equivalents(cx.keyboard_layout().id());
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
                cx.listener(|this, _, window, cx| {
                    this.last_keystrokes.take();
                    this.set_context_stack(window.context_stack(), cx);
                }),
            )
            .on_mouse_up_out(
                MouseButton::Right,
                cx.listener(|_, _, window, cx| {
                    cx.defer_in(window, |this, window, cx| {
                        this.last_keystrokes.take();
                        this.set_context_stack(window.context_stack(), cx);
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
                        Button::new("open_documentation", "Open Documentation")
                            .style(ButtonStyle::Filled)
                            .on_click(|_, _, cx| cx.open_url("https://zed.dev/docs/key-bindings")),
                    )
                    .child(
                        Button::new("view_default_keymap", "View default keymap")
                            .style(ButtonStyle::Filled)
                            .key_binding(ui::KeyBinding::for_action(
                                &zed_actions::OpenDefaultKeymap,
                                window,
                                cx
                            ))
                            .on_click(|_, window, cx| {
                                window.dispatch_action(zed_actions::OpenDefaultKeymap.boxed_clone(), cx);
                            }),
                    )
                    .child(
                        Button::new("edit_your_keymap", "Edit your keymap")
                            .style(ButtonStyle::Filled)
                            .key_binding(ui::KeyBinding::for_action(&zed_actions::OpenKeymap, window, cx))
                            .on_click(|_, window, cx| {
                                window.dispatch_action(zed_actions::OpenKeymap.boxed_clone(), cx);
                            }),
                    ),
            )
            .child(
                Label::new("Current Context Stack")
                    .size(LabelSize::Large)
                    .mt_8(),
            )
            .children({
                self.context_stack.iter().enumerate().map(|(i, context)| {
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
