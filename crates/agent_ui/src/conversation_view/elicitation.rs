use acp_thread::{Elicitation, ElicitationEntryId, ElicitationStatus};
use agent_client_protocol::schema::v1 as acp;
use collections::{HashMap, HashSet};
use component::{Component, ComponentScope, example_group_with_title, single_example};
use editor::{Editor, EditorMode, SelectionEffects};
use futures::channel::oneshot;
use gpui::{
    AnyElement, App, Div, ElementId, Empty, Entity, FocusHandle, Focusable, MouseButton, Role,
    SharedString, TextStyleRefinement, Window, div,
};
use language::{Buffer, Capability, language_settings::SoftWrap};
use multi_buffer::{MultiBuffer, MultiBufferOffset};
use std::cell::Cell;
use std::collections::BTreeMap;
use std::rc::Rc;
use ui::{
    Button, Checkbox, ChoiceCard, Color, ContextMenu, Icon, IconName, IconSize, Label, LabelSize,
    ToggleState, prelude::*,
};

#[derive(Clone)]
struct ElicitationOption {
    value: String,
    label: SharedString,
    description: Option<SharedString>,
}

enum ElicitationFieldState {
    Text(Entity<Editor>),
    Boolean {
        value: bool,
        focus_handle: FocusHandle,
    },
    SingleSelect {
        value: Option<String>,
    },
    MultiSelect(HashSet<String>),
}

#[derive(PartialEq, Eq)]
enum ElicitationFieldValue {
    Text(String),
    Boolean(bool),
    SingleSelect { value: Option<String> },
    MultiSelect(HashSet<String>),
}

#[derive(PartialEq, Eq)]
pub(crate) struct ElicitationFormSubmission {
    fields: HashMap<String, ElicitationFieldValue>,
}

pub(crate) struct ElicitationFormState {
    fields: HashMap<String, ElicitationFieldState>,
    option_views: HashMap<String, Vec<ElicitationOptionView>>,
    field_errors: HashMap<String, SharedString>,
    is_submitting: bool,
}

impl ElicitationFormState {
    pub(crate) fn new(schema: &acp::ElicitationSchema, window: &mut Window, cx: &mut App) -> Self {
        let required = schema.required.as_deref().unwrap_or_default();
        let mut fields = HashMap::default();
        let mut option_views = HashMap::default();

        for (name, property) in &schema.properties {
            let is_required = required.iter().any(|required| required == name);
            let field = match property {
                acp::ElicitationPropertySchema::String(schema) => {
                    let options = single_select_options(schema);
                    if options.is_empty() {
                        let editor = cx.new(|cx| {
                            let mut editor = Editor::single_line(window, cx);
                            if let Some(default) = &schema.default {
                                editor.set_text(default.clone(), window, cx);
                            }
                            editor
                        });
                        ElicitationFieldState::Text(editor)
                    } else {
                        let value = single_select_default_value(schema, &options).or_else(|| {
                            is_required
                                .then(|| options.first().map(|option| option.value.clone()))
                                .flatten()
                        });
                        option_views.insert(
                            name.clone(),
                            options
                                .into_iter()
                                .map(|option| ElicitationOptionView::new(option, window, cx))
                                .collect(),
                        );
                        ElicitationFieldState::SingleSelect { value }
                    }
                }
                acp::ElicitationPropertySchema::Number(schema) => {
                    let editor = cx.new(|cx| {
                        let mut editor = Editor::single_line(window, cx);
                        if let Some(default) = schema.default {
                            editor.set_text(default.to_string(), window, cx);
                        }
                        editor
                    });
                    ElicitationFieldState::Text(editor)
                }
                acp::ElicitationPropertySchema::Integer(schema) => {
                    let editor = cx.new(|cx| {
                        let mut editor = Editor::single_line(window, cx);
                        if let Some(default) = schema.default {
                            editor.set_text(default.to_string(), window, cx);
                        }
                        editor
                    });
                    ElicitationFieldState::Text(editor)
                }
                acp::ElicitationPropertySchema::Boolean(schema) => ElicitationFieldState::Boolean {
                    value: schema.default.unwrap_or(false),
                    focus_handle: cx.focus_handle(),
                },
                acp::ElicitationPropertySchema::Array(schema) => {
                    option_views.insert(
                        name.clone(),
                        multi_select_options(schema)
                            .into_iter()
                            .map(|option| ElicitationOptionView::new(option, window, cx))
                            .collect(),
                    );
                    ElicitationFieldState::MultiSelect(
                        schema
                            .default
                            .clone()
                            .unwrap_or_default()
                            .into_iter()
                            .collect(),
                    )
                }
                _ => continue,
            };
            fields.insert(name.clone(), field);
        }

        Self {
            fields,
            option_views,
            field_errors: HashMap::default(),
            is_submitting: false,
        }
    }

    fn snapshot(&self, cx: &App) -> ElicitationFormSubmission {
        ElicitationFormSubmission {
            fields: self
                .fields
                .iter()
                .map(|(name, field)| {
                    let value = match field {
                        ElicitationFieldState::Text(editor) => {
                            ElicitationFieldValue::Text(editor.read(cx).text(cx))
                        }
                        ElicitationFieldState::Boolean { value, .. } => {
                            ElicitationFieldValue::Boolean(*value)
                        }
                        ElicitationFieldState::SingleSelect { value } => {
                            ElicitationFieldValue::SingleSelect {
                                value: value.clone(),
                            }
                        }
                        ElicitationFieldState::MultiSelect(values) => {
                            ElicitationFieldValue::MultiSelect(values.clone())
                        }
                    };
                    (name.clone(), value)
                })
                .collect(),
        }
    }

    pub(crate) fn begin_submission(&mut self, cx: &App) -> Option<ElicitationFormSubmission> {
        if self.is_submitting {
            return None;
        }
        self.is_submitting = true;
        Some(self.snapshot(cx))
    }

    pub(crate) fn validation_matches_current_values(
        &mut self,
        submission: &ElicitationFormSubmission,
        cx: &App,
    ) -> bool {
        let is_current = self.snapshot(cx) == *submission;
        if !is_current {
            self.is_submitting = false;
        }
        is_current
    }

    #[cfg(test)]
    pub(crate) fn collect(
        &self,
        schema: &acp::ElicitationSchema,
        cx: &App,
    ) -> Result<BTreeMap<String, acp::ElicitationContentValue>, HashMap<String, SharedString>> {
        self.snapshot(cx).validate(schema)
    }

    pub(crate) fn set_errors(&mut self, errors: HashMap<String, SharedString>) {
        self.field_errors = errors;
        self.is_submitting = false;
    }

    pub(crate) fn set_field_error(
        &mut self,
        field_name: impl Into<String>,
        error: impl Into<SharedString>,
    ) {
        self.field_errors.insert(field_name.into(), error.into());
    }

    pub(crate) fn set_boolean(&mut self, field_name: &str, value: bool) {
        if let Some(ElicitationFieldState::Boolean { value: field, .. }) =
            self.fields.get_mut(field_name)
        {
            *field = value;
            self.field_errors.remove(field_name);
        }
    }

    pub(crate) fn set_single_select(&mut self, field_name: &str, value: String) {
        if let Some(ElicitationFieldState::SingleSelect { value: selected }) =
            self.fields.get_mut(field_name)
        {
            *selected = Some(value);
            self.field_errors.remove(field_name);
        }
    }

    pub(crate) fn set_multi_select(&mut self, field_name: &str, value: String, selected: bool) {
        if let Some(ElicitationFieldState::MultiSelect(values)) = self.fields.get_mut(field_name) {
            if selected {
                values.insert(value);
            } else {
                values.remove(&value);
            }
            self.field_errors.remove(field_name);
        }
    }
}

impl ElicitationFormSubmission {
    pub(crate) fn validate(
        &self,
        schema: &acp::ElicitationSchema,
    ) -> Result<BTreeMap<String, acp::ElicitationContentValue>, HashMap<String, SharedString>> {
        let required = schema.required.as_deref().unwrap_or_default();
        let mut content = BTreeMap::new();
        let mut errors = HashMap::default();

        for (name, property) in &schema.properties {
            let is_required = required.iter().any(|required| required == name);
            let Some(field) = self.fields.get(name) else {
                continue;
            };

            let field_content = match (property, field) {
                (
                    acp::ElicitationPropertySchema::String(schema),
                    ElicitationFieldValue::Text(value),
                ) => {
                    if value.is_empty() {
                        if is_required {
                            Err(format!("{} is required", property_title(name, property)).into())
                        } else {
                            Ok(None)
                        }
                    } else {
                        validate_string_value(property_title(name, property), schema, value)
                            .map(|()| Some(value.clone().into()))
                    }
                }
                (
                    acp::ElicitationPropertySchema::String(schema),
                    ElicitationFieldValue::SingleSelect { value },
                ) => {
                    if let Some(value) = value {
                        validate_single_select_value(property_title(name, property), schema, value)
                            .and_then(|()| {
                                validate_string_value(property_title(name, property), schema, value)
                            })
                            .map(|()| Some(value.clone().into()))
                    } else if is_required {
                        Err(format!("{} is required", property_title(name, property)).into())
                    } else {
                        Ok(None)
                    }
                }
                (
                    acp::ElicitationPropertySchema::Number(schema),
                    ElicitationFieldValue::Text(value),
                ) => {
                    let value = value.trim();
                    if value.is_empty() {
                        if is_required {
                            Err(format!("{} is required", property_title(name, property)).into())
                        } else {
                            Ok(None)
                        }
                    } else {
                        validate_number_value(property_title(name, property), schema, value)
                            .map(|parsed| Some(parsed.into()))
                    }
                }
                (
                    acp::ElicitationPropertySchema::Integer(schema),
                    ElicitationFieldValue::Text(value),
                ) => {
                    let value = value.trim();
                    if value.is_empty() {
                        if is_required {
                            Err(format!("{} is required", property_title(name, property)).into())
                        } else {
                            Ok(None)
                        }
                    } else {
                        validate_integer_value(property_title(name, property), schema, value)
                            .map(|parsed| Some(parsed.into()))
                    }
                }
                (
                    acp::ElicitationPropertySchema::Boolean(schema),
                    ElicitationFieldValue::Boolean(value),
                ) => {
                    if is_required || *value || schema.default.is_some() {
                        Ok(Some((*value).into()))
                    } else {
                        Ok(None)
                    }
                }
                (
                    acp::ElicitationPropertySchema::Array(schema),
                    ElicitationFieldValue::MultiSelect(selected),
                ) => {
                    let mut values = multi_select_options(schema)
                        .into_iter()
                        .filter_map(|option| {
                            selected.contains(&option.value).then_some(option.value)
                        })
                        .collect::<Vec<_>>();
                    values.sort();
                    if values.is_empty() && !is_required {
                        Ok(None)
                    } else if schema
                        .min_items
                        .is_some_and(|min_items| values.len() < min_items as usize)
                    {
                        Err(
                            format!("{} needs more selections", property_title(name, property))
                                .into(),
                        )
                    } else if schema
                        .max_items
                        .is_some_and(|max_items| values.len() > max_items as usize)
                    {
                        Err(
                            format!("{} has too many selections", property_title(name, property))
                                .into(),
                        )
                    } else {
                        Ok(Some(values.into()))
                    }
                }
                _ => Ok(None),
            };

            match field_content {
                Ok(Some(value)) => {
                    content.insert(name.clone(), value);
                }
                Ok(None) => {}
                Err(error) => {
                    errors.insert(name.clone(), error);
                }
            }
        }

        if errors.is_empty() {
            Ok(content)
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{
        ClipboardItem, KeyUpEvent, Keystroke, Modifiers, PlatformInput, TestAppContext,
        VisualTestContext, point, size,
    };
    use std::cell::RefCell;

    struct TestElicitationView {
        elicitation: Elicitation,
        form_state: ElicitationFormState,
        events: Rc<RefCell<Vec<String>>>,
    }

    impl TestElicitationView {
        fn new(schema: acp::ElicitationSchema, window: &mut Window, cx: &mut App) -> Self {
            Self {
                form_state: ElicitationFormState::new(&schema, window, cx),
                elicitation: Elicitation {
                    id: ElicitationEntryId("keyboard-test".into()),
                    request: acp::CreateElicitationRequest::new(
                        acp::ElicitationFormMode::new(preview_request_scope(0), schema),
                        "Choose an answer.",
                    ),
                    status: pending_status(),
                },
                events: Rc::default(),
            }
        }

        fn editor(&self, field_name: &str) -> Entity<Editor> {
            match self.form_state.fields.get(field_name) {
                Some(ElicitationFieldState::Text(editor)) => editor.clone(),
                _ => panic!("expected a text field named {field_name}"),
            }
        }
    }

    impl Render for TestElicitationView {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let response_handler = |response: &'static str| -> RespondHandler {
                let events = self.events.clone();
                let expected_id = self.elicitation.id.clone();
                Rc::new(move |id, _, _| {
                    assert_eq!(id, expected_id);
                    events.borrow_mut().push(response.into());
                })
            };
            let handlers = ElicitationCardHandlers {
                on_submit: response_handler("submit"),
                on_decline: response_handler("decline"),
                on_cancel: response_handler("cancel"),
                on_dismiss_url: response_handler("dismiss"),
                on_open_url: {
                    let events = self.events.clone();
                    let expected_id = self.elicitation.id.clone();
                    Rc::new(move |id, url, _, _| {
                        assert_eq!(id, expected_id);
                        events.borrow_mut().push(format!("open: {url}"));
                    })
                },
                on_boolean_change: {
                    let events = self.events.clone();
                    Rc::new(move |_, name, value, _| {
                        events.borrow_mut().push(format!("{name}: {value}"));
                    })
                },
                on_single_select_change: {
                    let events = self.events.clone();
                    Rc::new(move |_, name, value, _| {
                        events.borrow_mut().push(format!("{name}: {value}"));
                    })
                },
                on_multi_select_change: {
                    let events = self.events.clone();
                    Rc::new(move |_, name, value, selected, _| {
                        events
                            .borrow_mut()
                            .push(format!("{name}: {value} = {selected}"));
                    })
                },
            };

            div()
                .key_context("AcpThread")
                .on_action(|_: &crate::CycleModeSelector, _, _| {
                    panic!("form navigation must not cycle the agent mode");
                })
                .on_action(|_: &zed_actions::agent::Chat, _, _| {
                    panic!("form submission must not send a chat message");
                })
                .child(
                    ElicitationCard::new(
                        0,
                        &self.elicitation,
                        "Claude Code".into(),
                        Some(&self.form_state),
                        handlers,
                    )
                    .render(cx),
                )
        }
    }

    fn init_keyboard_test(cx: &mut TestAppContext) {
        crate::conversation_view::tests::init_test(cx);
        cx.update(|cx| {
            let bindings = settings::KeymapFile::load_asset_allow_partial_failure(
                "keymaps/default-linux.json",
                cx,
            )
            .expect("default keymap should load");
            cx.bind_keys(bindings);
        });
    }

    fn press_keys(cx: &mut VisualTestContext, keys: &str) {
        for key in keys.split_whitespace() {
            cx.simulate_keystrokes(key);
            cx.update(|window, cx| {
                // Click handlers activate buttons on key release, not key press.
                window.dispatch_event(
                    PlatformInput::KeyUp(KeyUpEvent {
                        keystroke: Keystroke::parse(key).expect("valid test keystroke"),
                    }),
                    cx,
                );
            });
            cx.run_until_parked();
        }
    }

    #[gpui::test]
    fn form_text_fields_submit_on_enter(cx: &mut TestAppContext) {
        init_keyboard_test(cx);
        let (view, cx) = cx.add_window_view(|window, cx| {
            TestElicitationView::new(
                acp::ElicitationSchema::new()
                    .string("other", true)
                    .property("count", acp::IntegerPropertySchema::new(), true)
                    .property("amount", acp::NumberPropertySchema::new(), true),
                window,
                cx,
            )
        });
        let events = view.read_with(cx, |view, _| view.events.clone());

        for (field_name, value) in [
            ("other", "  My answer  "),
            ("count", "2"),
            ("amount", "1.5"),
        ] {
            let editor = view.read_with(cx, |view, _| view.editor(field_name));
            cx.update(|window, cx| window.focus(&editor.focus_handle(cx), cx));
            cx.simulate_input(value);
            press_keys(cx, "enter");
            assert_eq!(editor.read_with(cx, |editor, cx| editor.text(cx)), value);
        }
        assert_eq!(*events.borrow(), ["submit", "submit", "submit"]);

        view.update(cx, |view, cx| {
            assert!(view.form_state.begin_submission(cx).is_some());
            cx.notify();
        });
        press_keys(cx, "enter");
        assert_eq!(events.borrow().len(), 3);
    }

    #[gpui::test]
    fn form_tab_navigation_reaches_fields_and_actions(cx: &mut TestAppContext) {
        init_keyboard_test(cx);
        let (view, cx) = cx.add_window_view(|window, cx| {
            TestElicitationView::new(
                acp::ElicitationSchema::new()
                    .string("first", false)
                    .string("other", false),
                window,
                cx,
            )
        });
        let (first, other, events) = view.read_with(cx, |view, _| {
            (
                view.editor("first"),
                view.editor("other"),
                view.events.clone(),
            )
        });
        cx.update(|window, cx| window.focus(&first.focus_handle(cx), cx));

        press_keys(cx, "tab");
        cx.update(|window, cx| assert!(other.focus_handle(cx).is_focused(window)));
        press_keys(cx, "shift-tab");
        cx.update(|window, cx| assert!(first.focus_handle(cx).is_focused(window)));

        press_keys(cx, "tab tab");
        let submit_focus =
            cx.update(|window, cx| window.focused(cx).expect("Submit should be focused"));
        press_keys(cx, "shift-tab");
        cx.update(|window, cx| assert!(other.focus_handle(cx).is_focused(window)));
        press_keys(cx, "tab");
        cx.update(|window, _| assert!(submit_focus.is_focused(window)));
        press_keys(cx, "enter tab enter tab space");
        assert_eq!(*events.borrow(), ["submit", "decline", "cancel"]);

        press_keys(cx, "shift-tab space");
        assert_eq!(*events.borrow(), ["submit", "decline", "cancel", "decline"]);
        let decline_focus =
            cx.update(|window, cx| window.focused(cx).expect("Decline should be focused"));

        cx.update(|window, cx| window.focus(&submit_focus, cx));
        view.update(cx, |view, cx| {
            assert!(view.form_state.begin_submission(cx).is_some());
            cx.notify();
        });
        press_keys(cx, "tab");
        cx.update(|window, _| assert!(decline_focus.is_focused(window)));
        press_keys(cx, "shift-tab");
        cx.update(|window, cx| assert!(other.focus_handle(cx).is_focused(window)));
        press_keys(cx, "tab");
        cx.update(|window, _| assert!(decline_focus.is_focused(window)));
        press_keys(cx, "shift-tab");
        cx.update(|window, cx| assert!(other.focus_handle(cx).is_focused(window)));

        view.update(cx, |view, cx| {
            view.form_state.set_errors(HashMap::default());
            cx.notify();
        });
        press_keys(cx, "tab");
        cx.update(|window, _| assert!(submit_focus.is_focused(window)));
    }

    #[gpui::test]
    fn form_choices_are_keyboard_accessible(cx: &mut TestAppContext) {
        init_keyboard_test(cx);
        let (view, cx) = cx.add_window_view(|window, cx| {
            TestElicitationView::new(
                acp::ElicitationSchema::new()
                    .string("answer", false)
                    .property("boolean", acp::BooleanPropertySchema::new(), false)
                    .property(
                        "choice",
                        acp::StringPropertySchema::new()
                            .enum_values(vec!["first".into(), "second".into()]),
                        false,
                    )
                    .property(
                        "multiple",
                        acp::MultiSelectPropertySchema::new(vec!["first".into(), "second".into()]),
                        false,
                    ),
                window,
                cx,
            )
        });
        let (editor, events) =
            view.read_with(cx, |view, _| (view.editor("answer"), view.events.clone()));
        cx.update(|window, cx| window.focus(&editor.focus_handle(cx), cx));
        press_keys(
            cx,
            "tab space tab enter tab space tab space tab enter tab enter",
        );
        assert_eq!(
            *events.borrow(),
            [
                "boolean: true",
                "choice: first",
                "choice: second",
                "multiple: first = true",
                "multiple: second = true",
                "submit",
            ]
        );
    }

    #[gpui::test]
    fn form_option_text_selection_preserves_answer_controls(cx: &mut TestAppContext) {
        init_keyboard_test(cx);
        let label = "https://example.com/path **literal** `code` café 中🙂 with enough text to wrap across several lines in a narrow panel";
        let description = "First line\r\n\r\n  Second line with &amp; and <literal> text";
        for multiple in [false, true] {
            let options = vec![
                acp::EnumOption::new("before", "Before"),
                acp::EnumOption::new("value", label).description(description),
                acp::EnumOption::new("after", "After"),
            ];
            let property = if multiple {
                acp::ElicitationPropertySchema::from(acp::MultiSelectPropertySchema::titled(
                    options,
                ))
            } else {
                acp::ElicitationPropertySchema::from(
                    acp::StringPropertySchema::new().one_of(options),
                )
            };
            let (view, cx) = cx.add_window_view(|window, cx| {
                TestElicitationView::new(
                    acp::ElicitationSchema::new().property("choice", property, false),
                    window,
                    cx,
                )
            });
            cx.simulate_resize(size(px(320.), px(800.)));
            cx.run_until_parked();
            let (events, focus, next_focus) = view.read_with(cx, |view, _| {
                (
                    view.events.clone(),
                    view.form_state.option_views["choice"][1]
                        .focus_handle
                        .clone(),
                    view.form_state.option_views["choice"][2]
                        .focus_handle
                        .clone(),
                )
            });
            for (selector, expected) in [
                ("elicitation-option-label-value", label),
                (
                    "elicitation-option-description-value",
                    "First line\n\n  Second line with &amp; and <literal> text",
                ),
            ] {
                let bounds = cx
                    .debug_bounds(selector)
                    .expect("option text should be rendered");
                let start = bounds.origin + point(px(1.), px(1.));
                let end = bounds.bottom_right() - point(px(1.), px(1.));
                for (start, end) in [(start, end), (end, start)] {
                    cx.update(|_, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string("unchanged".to_string()))
                    });
                    cx.simulate_mouse_down(start, MouseButton::Left, Modifiers::default());
                    cx.simulate_mouse_move(end, MouseButton::Left, Modifiers::default());
                    cx.simulate_mouse_up(end, MouseButton::Left, Modifiers::default());
                    view.update(cx, |_, cx| cx.notify());
                    cx.simulate_keystrokes("ctrl-c");
                    assert_eq!(
                        cx.read_from_clipboard()
                            .and_then(|item| item.text())
                            .as_deref(),
                        Some(expected)
                    );
                    assert_eq!(*events.borrow(), Vec::<String>::new());
                    press_keys(cx, "shift-tab");
                    cx.update(|window, _| assert!(focus.is_focused(window)));
                    press_keys(cx, "tab");
                    cx.update(|window, _| assert!(next_focus.is_focused(window)));
                }
                for turn in [end, point(px(1.), px(1.))] {
                    cx.simulate_mouse_down(start, MouseButton::Left, Modifiers::default());
                    cx.simulate_mouse_move(turn, MouseButton::Left, Modifiers::default());
                    cx.simulate_mouse_move(start, MouseButton::Left, Modifiers::default());
                    cx.simulate_mouse_up(start, MouseButton::Left, Modifiers::default());
                    assert_eq!(*events.borrow(), Vec::<String>::new());
                }
            }
            let bounds = cx
                .debug_bounds("elicitation-option-label-value")
                .expect("option text should be rendered");
            cx.simulate_click(bounds.origin + point(px(1.), px(1.)), Modifiers::default());
            cx.update(|window, _| assert!(focus.is_focused(window)));
            press_keys(cx, "space");
            let expected = if multiple {
                "choice: value = true"
            } else {
                "choice: value"
            };
            assert_eq!(*events.borrow(), [expected, expected]);
            let editor = view.read_with(cx, |view, _| {
                view.form_state.option_views["choice"][1]
                    .label
                    .editor
                    .clone()
            });
            editor.update_in(cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                    selections.select_ranges([MultiBufferOffset(0)..MultiBufferOffset(5)]);
                });
                let selection = editor.selections.newest_anchor().clone();
                let target = editor.snapshot(window, cx).max_point();
                editor.move_selection_on_drop(&selection, target, false, window, cx);
                assert_eq!(editor.text(cx), label);
            });
        }
    }

    #[gpui::test]
    fn form_request_text_has_selection_copy_menu(cx: &mut TestAppContext) {
        init_keyboard_test(cx);
        let (view, cx) = cx.add_window_view(|window, cx| {
            let mut schema = acp::ElicitationSchema::new().property(
                "boolean",
                acp::BooleanPropertySchema::new()
                    .title("Remember café 中🙂")
                    .description("Keep **literal** text."),
                false,
            );
            schema.title = Some("Connection".to_string());
            schema.description = Some("Review the request.".to_string());
            TestElicitationView::new(schema, window, cx)
        });
        cx.simulate_resize(size(px(320.), px(800.)));
        cx.run_until_parked();
        let events = view.read_with(cx, |view, _| view.events.clone());
        let question = cx.debug_bounds("elicitation-question").expect("question");
        cx.update(|_, cx| {
            cx.write_to_clipboard(ClipboardItem::new_string("unchanged".to_string()))
        });
        cx.simulate_mouse_down(question.center(), MouseButton::Right, Modifiers::default());
        cx.simulate_mouse_up(question.center(), MouseButton::Right, Modifiers::default());
        let copy = cx.debug_bounds("MENU_ITEM-Copy").expect("Copy menu entry");
        cx.simulate_click(copy.center(), Modifiers::default());
        assert_eq!(
            cx.read_from_clipboard()
                .and_then(|item| item.text())
                .as_deref(),
            Some("unchanged")
        );
        press_keys(cx, "escape");

        for (selector, expected) in [
            ("elicitation-question", "Choose an answer."),
            ("elicitation-title", "Connection"),
            ("elicitation-description", "Review the request."),
            ("elicitation-field-title-boolean", "Remember café 中🙂"),
            (
                "elicitation-field-description-boolean",
                "Keep **literal** text.",
            ),
        ] {
            select_text(cx, selector);
            view.update(cx, |_, cx| cx.notify());
            copy_selection_from_menu(cx, selector, "KEY_BINDING-c", expected);
        }
        assert_eq!(*events.borrow(), Vec::<String>::new());

        view.update(cx, |view, cx| {
            view.elicitation.request.message = "Updated question?".to_string();
            view.form_state
                .set_field_error("boolean", "Check this value.");
            cx.notify();
        });
        cx.update(|_, cx| {
            cx.bind_keys([gpui::KeyBinding::new(
                "ctrl-shift-x",
                editor::actions::Copy,
                Some("Editor"),
            )]);
        });
        cx.run_until_parked();
        select_text(cx, "elicitation-question");
        copy_selection_from_menu(
            cx,
            "elicitation-question",
            "KEY_BINDING-x",
            "Updated question?",
        );
        select_text(cx, "elicitation-field-error-boolean");
        cx.simulate_keystrokes("ctrl-shift-x");
        assert_eq!(
            cx.read_from_clipboard()
                .and_then(|item| item.text())
                .as_deref(),
            Some("Check this value.")
        );
        view.update(cx, |view, cx| {
            view.form_state.set_field_error("boolean", "Changed error.");
            cx.notify();
        });
        cx.run_until_parked();
        select_text(cx, "elicitation-field-error-boolean");
        cx.simulate_keystrokes("ctrl-shift-x");
        assert_eq!(
            cx.read_from_clipboard()
                .and_then(|item| item.text())
                .as_deref(),
            Some("Changed error.")
        );

        let title = cx
            .debug_bounds("elicitation-field-title-boolean")
            .expect("boolean title");
        cx.simulate_click(title.origin + point(px(1.), px(1.)), Modifiers::default());
        press_keys(cx, "space");
        assert_eq!(*events.borrow(), ["boolean: true", "boolean: true"]);
        select_text(cx, "elicitation-field-description-boolean");
        press_keys(cx, "shift-tab space tab enter");
        assert_eq!(
            *events.borrow(),
            ["boolean: true", "boolean: true", "boolean: true", "submit"]
        );
    }

    #[gpui::test]
    fn tab_navigation_between_form_and_url_cards(cx: &mut TestAppContext) {
        struct Cards {
            form: Entity<TestElicitationView>,
            url: Entity<TestElicitationView>,
        }

        impl Render for Cards {
            fn render(
                &mut self,
                _window: &mut Window,
                _cx: &mut Context<Self>,
            ) -> impl IntoElement {
                v_flex().child(self.form.clone()).child(self.url.clone())
            }
        }

        init_keyboard_test(cx);
        let (cards, cx) = cx.add_window_view(|window, cx| Cards {
            form: cx.new(|cx| {
                TestElicitationView::new(
                    acp::ElicitationSchema::new().string("other", false),
                    window,
                    cx,
                )
            }),
            url: cx.new(|cx| {
                let mut view = TestElicitationView::new(acp::ElicitationSchema::new(), window, cx);
                view.elicitation.id = ElicitationEntryId("url-test".into());
                view.elicitation.request = acp::CreateElicitationRequest::new(
                    acp::ElicitationUrlMode::new(
                        preview_request_scope(1),
                        acp::ElicitationId::new("url-test"),
                        "https://example.com/authorize",
                    ),
                    "Authorize access.",
                );
                view
            }),
        });
        let (form, url) = cards.read_with(cx, |cards, _| (cards.form.clone(), cards.url.clone()));
        let (editor, form_events) =
            form.read_with(cx, |form, _| (form.editor("other"), form.events.clone()));
        let url_events = url.read_with(cx, |url, _| url.events.clone());
        cx.update(|window, cx| window.focus(&editor.focus_handle(cx), cx));

        press_keys(cx, "tab tab tab enter");
        assert_eq!(*form_events.borrow(), ["cancel"]);
        press_keys(cx, "tab enter");
        assert_eq!(
            *url_events.borrow(),
            ["open: https://example.com/authorize", "submit"]
        );

        press_keys(cx, "shift-tab enter");
        assert_eq!(*form_events.borrow(), ["cancel", "cancel"]);
        press_keys(cx, "shift-tab shift-tab enter");
        assert_eq!(*form_events.borrow(), ["cancel", "cancel", "submit"]);
        press_keys(cx, "shift-tab");
        cx.update(|window, cx| assert!(editor.focus_handle(cx).is_focused(window)));

        url.update(cx, |url, cx| {
            url.elicitation.status = ElicitationStatus::Accepted;
            cx.notify();
        });
        press_keys(cx, "tab tab tab tab enter tab enter");
        assert_eq!(
            *url_events.borrow(),
            [
                "open: https://example.com/authorize",
                "submit",
                "open: https://example.com/authorize",
                "dismiss",
            ]
        );
        assert_eq!(*form_events.borrow(), ["cancel", "cancel", "submit"]);
    }

    #[test]
    fn string_validation_rejects_email_format_mismatch() {
        let schema = acp::StringPropertySchema::email();

        validate_string_value("Email".into(), &schema, "user@example.com")
            .expect("valid email should be accepted");
        assert_eq!(
            validate_string_value("Email".into(), &schema, "not-an-email")
                .expect_err("invalid email should be rejected")
                .to_string(),
            "Email must be an email address"
        );
    }

    #[test]
    fn string_validation_rejects_pattern_mismatch() {
        let schema = acp::StringPropertySchema::new().pattern("^prod-[0-9]+$");

        validate_string_value("Environment".into(), &schema, "prod-42")
            .expect("matching pattern should be accepted");
        assert_eq!(
            validate_string_value("Environment".into(), &schema, "dev-42")
                .expect_err("pattern mismatch should be rejected")
                .to_string(),
            "Environment does not match the requested pattern"
        );
    }

    #[test]
    fn string_validation_supports_bounded_advanced_patterns() {
        let schema = acp::StringPropertySchema::new().pattern(r"^prod-(?=[0-9]+$)([0-9])\1$");

        validate_string_value("Environment".into(), &schema, "prod-44")
            .expect("lookahead and backreference should be accepted");
        assert_eq!(
            validate_string_value("Environment".into(), &schema, "prod-45")
                .expect_err("backreference mismatch should be rejected")
                .to_string(),
            "Environment does not match the requested pattern"
        );
    }

    #[test]
    fn string_validation_rejects_patterns_outside_resource_limits() {
        let oversized_pattern =
            acp::StringPropertySchema::new().pattern("a".repeat(MAX_ELICITATION_PATTERN_BYTES + 1));
        assert_eq!(
            validate_string_value("Value".into(), &oversized_pattern, "a")
                .expect_err("oversized pattern should be rejected")
                .to_string(),
            "Value has an invalid validation pattern"
        );

        let schema = acp::StringPropertySchema::new().pattern(".*");
        let oversized_value = "a".repeat(MAX_ELICITATION_PATTERN_INPUT_BYTES + 1);
        assert_eq!(
            validate_string_value("Value".into(), &schema, &oversized_value)
                .expect_err("oversized pattern input should be rejected")
                .to_string(),
            "Value is too long to validate safely"
        );
    }

    #[test]
    fn number_validation_rejects_non_finite_values() {
        let schema = acp::NumberPropertySchema::new();

        assert_eq!(
            validate_number_value("Amount".into(), &schema, "42.5")
                .expect("finite number should be accepted"),
            42.5
        );

        for value in ["NaN", "inf", "-inf", "1e309"] {
            assert_eq!(
                validate_number_value("Amount".into(), &schema, value)
                    .expect_err("non-finite number should be rejected")
                    .to_string(),
                "Amount must be a finite number"
            );
        }
    }

    #[test]
    fn should_render_pending_and_accepted_url_elicitations() {
        let pending = Elicitation {
            id: ElicitationEntryId("pending".into()),
            request: acp::CreateElicitationRequest::new(
                acp::ElicitationFormMode::new(
                    preview_request_scope(0),
                    acp::ElicitationSchema::new(),
                ),
                "Review this request.",
            ),
            status: pending_status(),
        };
        assert!(should_render_elicitation(&pending));

        let accepted_url = Elicitation {
            id: ElicitationEntryId("accepted-url".into()),
            request: acp::CreateElicitationRequest::new(
                acp::ElicitationUrlMode::new(
                    preview_request_scope(1),
                    acp::ElicitationId::new("accepted-url"),
                    "https://auth.example.com/device",
                ),
                "Authorize Zed in your browser.",
            ),
            status: ElicitationStatus::Accepted,
        };
        assert!(should_render_elicitation(&accepted_url));

        let accepted_form = Elicitation {
            id: ElicitationEntryId("accepted-form".into()),
            request: acp::CreateElicitationRequest::new(
                acp::ElicitationFormMode::new(
                    preview_request_scope(2),
                    acp::ElicitationSchema::new(),
                ),
                "Review this request.",
            ),
            status: ElicitationStatus::Accepted,
        };
        assert!(!should_render_elicitation(&accepted_form));

        let pending_unknown = Elicitation {
            id: ElicitationEntryId("pending-unknown".into()),
            request: acp::CreateElicitationRequest::new(
                acp::OtherElicitationMode::new("future", preview_request_scope(3), BTreeMap::new()),
                "Use a future input mode.",
            ),
            status: pending_status(),
        };
        assert!(!should_render_elicitation(&pending_unknown));
    }

    #[test]
    fn url_host_presentation_highlights_destination_and_suspicious_idn() {
        assert_eq!(
            url_host_presentation("https://auth.example.com/device"),
            Some(UrlHostPresentation {
                host: "auth.example.com".to_string(),
                suspicious_decoded_host: None,
            })
        );
        assert_eq!(
            url_host_presentation("https://xn--pple-43d.com/device"),
            Some(UrlHostPresentation {
                host: "xn--pple-43d.com".to_string(),
                suspicious_decoded_host: Some("\u{0430}pple.com".to_string()),
            })
        );
    }

    #[gpui::test]
    fn url_request_text_is_selectable_without_elision(cx: &mut TestAppContext) {
        init_keyboard_test(cx);
        let url = format!(
            "https://xn--pple-43d.com/authorize?state={}&scope=repository",
            "a".repeat(128)
        );
        let (view, cx) = cx.add_window_view(|window, cx| {
            let mut view = TestElicitationView::new(acp::ElicitationSchema::new(), window, cx);
            view.elicitation.request = acp::CreateElicitationRequest::new(
                acp::ElicitationUrlMode::new(
                    preview_request_scope(1),
                    acp::ElicitationId::new("url-test"),
                    url.clone(),
                ),
                "Authorize access.",
            );
            view
        });
        cx.simulate_resize(size(px(320.), px(800.)));
        cx.run_until_parked();
        let url_bounds = cx.debug_bounds("elicitation-url").expect("URL");
        let question_bounds = cx.debug_bounds("elicitation-question").expect("question");
        assert!(url_bounds.size.height > question_bounds.size.height);
        assert!(url_bounds.right() <= px(320.));
        for (selector, expected) in [
            ("elicitation-question", "Authorize access."),
            ("elicitation-url-host", "xn--pple-43d.com"),
            (
                "elicitation-url-warning",
                "This internationalized address displays as аpple.com. Verify it carefully.",
            ),
            ("elicitation-url", url.as_str()),
        ] {
            select_text(cx, selector);
            copy_selection_from_menu(cx, selector, "KEY_BINDING-c", expected);
        }
        let position = url_bounds.origin + point(px(30.), px(8.));
        cx.simulate_click(position, Modifiers::default());
        let modifiers = Modifiers::secondary_key();
        cx.simulate_modifiers_change(modifiers);
        cx.simulate_mouse_move(position, None, modifiers);
        cx.run_until_parked();
        cx.simulate_click(position, modifiers);
        assert_eq!(cx.opened_url(), None);
        let events = view.read_with(cx, |view, _| view.events.clone());
        assert_eq!(*events.borrow(), Vec::<String>::new());
    }

    #[test]
    fn single_select_options_include_titled_descriptions() {
        let schema = acp::StringPropertySchema::new().one_of(vec![
            acp::EnumOption::new("production", "Production").description("Use live resources"),
        ]);

        let options = single_select_options(&schema);

        let [option] = options.as_slice() else {
            panic!("expected one option, got {}", options.len());
        };
        assert_eq!(option.value, "production");
        assert_eq!(option.label.to_string(), "Production");
        assert_eq!(
            option
                .description
                .as_ref()
                .map(|description| description.to_string()),
            Some("Use live resources".to_string())
        );
    }

    #[test]
    fn multi_select_options_include_titled_descriptions() {
        let schema = acp::MultiSelectPropertySchema::titled(vec![
            acp::EnumOption::new("repository", "Repository Access")
                .description("Read and update repositories"),
        ]);

        let options = multi_select_options(&schema);

        let [option] = options.as_slice() else {
            panic!("expected one option, got {}", options.len());
        };
        assert_eq!(option.value, "repository");
        assert_eq!(option.label.to_string(), "Repository Access");
        assert_eq!(
            option
                .description
                .as_ref()
                .map(|description| description.to_string()),
            Some("Read and update repositories".to_string())
        );
    }

    #[gpui::test]
    fn form_state_preserves_string_whitespace(cx: &mut TestAppContext) {
        crate::conversation_view::tests::init_test(cx);

        cx.add_window(|window, cx| {
            let schema = acp::ElicitationSchema::new().property(
                "token",
                acp::StringPropertySchema::new()
                    .title("Token")
                    .default_value("  secret  "),
                true,
            );
            let form_state = ElicitationFormState::new(&schema, window, cx);
            let content = form_state
                .collect(&schema, cx)
                .expect("string with whitespace should be submitted");

            assert_eq!(
                content.get("token"),
                Some(&acp::ElicitationContentValue::String(
                    "  secret  ".to_string()
                ))
            );

            Editor::single_line(window, cx)
        });
    }

    #[gpui::test]
    fn form_state_prevents_duplicate_submissions(cx: &mut TestAppContext) {
        crate::conversation_view::tests::init_test(cx);

        cx.add_window(|window, cx| {
            let schema = acp::ElicitationSchema::new().string("name", true);
            let mut form_state = ElicitationFormState::new(&schema, window, cx);

            assert!(form_state.begin_submission(cx).is_some());
            assert!(form_state.begin_submission(cx).is_none());

            form_state.set_errors(HashMap::default());
            assert!(form_state.begin_submission(cx).is_some());

            Editor::single_line(window, cx)
        });
    }

    #[gpui::test]
    fn form_state_discards_validation_for_stale_values(cx: &mut TestAppContext) {
        crate::conversation_view::tests::init_test(cx);

        cx.add_window(|window, cx| {
            let schema = acp::ElicitationSchema::new().property(
                "name",
                acp::StringPropertySchema::new().default_value("before"),
                true,
            );
            let mut form_state = ElicitationFormState::new(&schema, window, cx);
            let submission = form_state
                .begin_submission(cx)
                .expect("first submission should start");
            let editor = match form_state.fields.get("name") {
                Some(ElicitationFieldState::Text(editor)) => editor.clone(),
                _ => panic!("expected a text field"),
            };

            editor.update(cx, |editor, cx| editor.set_text("after", window, cx));

            assert!(!form_state.validation_matches_current_values(&submission, cx));
            assert!(form_state.begin_submission(cx).is_some());

            Editor::single_line(window, cx)
        });
    }

    #[gpui::test]
    fn form_state_discards_invalid_optional_single_select_default(cx: &mut TestAppContext) {
        crate::conversation_view::tests::init_test(cx);

        cx.add_window(|window, cx| {
            let schema = acp::ElicitationSchema::new().property(
                "environment",
                acp::StringPropertySchema::new()
                    .title("Environment")
                    .enum_values(vec!["production".to_string(), "staging".to_string()])
                    .default_value("development"),
                false,
            );
            let form_state = ElicitationFormState::new(&schema, window, cx);
            let content = form_state
                .collect(&schema, cx)
                .expect("invalid optional default should be ignored");

            assert_eq!(content.get("environment"), None);

            Editor::single_line(window, cx)
        });
    }

    #[gpui::test]
    fn form_state_replaces_invalid_required_single_select_default(cx: &mut TestAppContext) {
        crate::conversation_view::tests::init_test(cx);

        cx.add_window(|window, cx| {
            let schema = acp::ElicitationSchema::new().property(
                "environment",
                acp::StringPropertySchema::new()
                    .title("Environment")
                    .enum_values(vec!["production".to_string(), "staging".to_string()])
                    .default_value("development"),
                true,
            );
            let form_state = ElicitationFormState::new(&schema, window, cx);
            let content = form_state
                .collect(&schema, cx)
                .expect("required select should use the first valid choice");

            assert_eq!(
                content.get("environment"),
                Some(&acp::ElicitationContentValue::String(
                    "production".to_string()
                ))
            );

            Editor::single_line(window, cx)
        });
    }

    #[gpui::test]
    fn form_state_rejects_invalid_single_select_value(cx: &mut TestAppContext) {
        crate::conversation_view::tests::init_test(cx);

        cx.add_window(|window, cx| {
            let schema = acp::ElicitationSchema::new().property(
                "environment",
                acp::StringPropertySchema::new()
                    .title("Environment")
                    .enum_values(vec!["production".to_string(), "staging".to_string()]),
                false,
            );
            let mut form_state = ElicitationFormState::new(&schema, window, cx);
            form_state.set_single_select("environment", "development".to_string());

            let errors = form_state
                .collect(&schema, cx)
                .expect_err("invalid selected value should be rejected");
            assert_eq!(
                errors
                    .get("environment")
                    .expect("environment should have an error")
                    .to_string(),
                "Environment must be one of the provided options"
            );

            Editor::single_line(window, cx)
        });
    }

    #[gpui::test]
    fn form_state_reports_all_validation_errors(cx: &mut TestAppContext) {
        crate::conversation_view::tests::init_test(cx);

        cx.add_window(|window, cx| {
            let schema = acp::ElicitationSchema::new()
                .string("account", true)
                .property(
                    "age",
                    acp::IntegerPropertySchema::new().title("Age").minimum(18),
                    true,
                )
                .property(
                    "environment",
                    acp::StringPropertySchema::new()
                        .title("Environment")
                        .enum_values(vec!["production".to_string(), "staging".to_string()]),
                    false,
                );
            let mut form_state = ElicitationFormState::new(&schema, window, cx);
            if let Some(ElicitationFieldState::Text(editor)) = form_state.fields.get("age") {
                editor.update(cx, |editor, cx| editor.set_text("abc", window, cx));
            }
            form_state.set_single_select("environment", "development".to_string());

            let errors = form_state
                .collect(&schema, cx)
                .expect_err("all invalid fields should be reported");
            assert_eq!(
                errors
                    .get("account")
                    .expect("account should have an error")
                    .to_string(),
                "account is required"
            );
            assert_eq!(
                errors
                    .get("age")
                    .expect("age should have an error")
                    .to_string(),
                "Age must be an integer"
            );
            assert_eq!(
                errors
                    .get("environment")
                    .expect("environment should have an error")
                    .to_string(),
                "Environment must be one of the provided options"
            );

            Editor::single_line(window, cx)
        });
    }

    fn select_text(cx: &mut VisualTestContext, selector: &'static str) {
        let bounds = cx.debug_bounds(selector).expect("selectable text");
        let start = bounds.origin + point(px(1.), px(1.));
        let end = bounds.bottom_right() - point(px(1.), px(1.));
        cx.simulate_mouse_down(start, MouseButton::Left, Modifiers::default());
        cx.simulate_mouse_move(end, MouseButton::Left, Modifiers::default());
        cx.simulate_mouse_up(end, MouseButton::Left, Modifiers::default());
    }

    fn copy_selection_from_menu(
        cx: &mut VisualTestContext,
        selector: &'static str,
        binding_selector: &'static str,
        expected: &str,
    ) {
        cx.update(|_, cx| {
            cx.write_to_clipboard(ClipboardItem::new_string("unchanged".to_string()))
        });
        let bounds = cx.debug_bounds(selector).expect("selected text");
        cx.simulate_mouse_down(bounds.center(), MouseButton::Right, Modifiers::default());
        cx.simulate_mouse_up(bounds.center(), MouseButton::Right, Modifiers::default());
        let copy = cx.debug_bounds("MENU_ITEM-Copy").expect("Copy menu entry");
        assert!(
            cx.debug_bounds(binding_selector).is_some(),
            "Copy keybinding"
        );
        assert!(cx.debug_bounds("MENU_ITEM-Cut").is_none());
        assert!(cx.debug_bounds("MENU_ITEM-Paste").is_none());
        cx.simulate_click(copy.center(), Modifiers::default());
        assert_eq!(
            cx.read_from_clipboard()
                .and_then(|item| item.text())
                .as_deref(),
            Some(expected)
        );
    }
}

#[derive(RegisterComponent)]
pub struct ElicitationCardPreview;

impl Component for ElicitationCardPreview {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn description() -> &'static str {
        "ACP elicitation request cards as rendered in the agent panel."
    }

    fn preview(window: &mut Window, cx: &mut App) -> AnyElement {
        v_flex()
            .gap_6()
            .children([
                example_group_with_title(
                    "Form Requests",
                    vec![
                        single_example(
                            "Pending Form",
                            render_form_preview(0, pending_status(), &[], window, cx),
                        )
                        .width(px(640.)),
                        single_example(
                            "Validation Errors",
                            render_form_preview(
                                1,
                                pending_status(),
                                &[
                                    ("account_name", "Account name is required"),
                                    ("environment", "Choose an environment"),
                                    ("scopes", "Choose at least one access scope"),
                                ],
                                window,
                                cx,
                            ),
                        )
                        .width(px(640.)),
                    ],
                )
                .vertical()
                .into_any_element(),
                example_group_with_title(
                    "URL Requests",
                    vec![
                        single_example(
                            "URL Consent",
                            render_url_preview(3, pending_status(), window, cx),
                        )
                        .width(px(640.)),
                    ],
                )
                .vertical()
                .into_any_element(),
                example_group_with_title(
                    "Terminal States",
                    vec![
                        single_example(
                            "Declined",
                            render_form_preview(6, ElicitationStatus::Declined, &[], window, cx),
                        )
                        .width(px(640.)),
                        single_example(
                            "Canceled",
                            render_form_preview(7, ElicitationStatus::Canceled, &[], window, cx),
                        )
                        .width(px(640.)),
                    ],
                )
                .vertical()
                .into_any_element(),
            ])
            .into_any_element()
    }
}

fn render_form_preview(
    entry_ix: usize,
    status: ElicitationStatus,
    field_errors: &[(&'static str, &'static str)],
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let request = acp::CreateElicitationRequest::new(
        acp::ElicitationFormMode::new(preview_request_scope(entry_ix), preview_form_schema()),
        "Choose how Zed should connect to this account.",
    );
    let mut form_state = matches!(status, ElicitationStatus::Pending { .. }).then(|| {
        let acp::ElicitationMode::Form(mode) = &request.mode else {
            unreachable!();
        };
        ElicitationFormState::new(&mode.requested_schema, window, cx)
    });
    if let Some(form_state) = &mut form_state {
        for (field_name, error) in field_errors {
            form_state.set_field_error(*field_name, *error);
        }
    }

    render_preview_card(entry_ix, request, status, form_state.as_ref(), cx)
}

fn preview_url() -> &'static str {
    "https://auth.example.com/oauth/authorize?client_id=zed-desktop&redirect_uri=zed%3A%2F%2Fagent%2Facp%2Fcallback&scope=profile%20repository%20terminal&state=9b8b0a873a1e4b57b7f9f7b6d2d3d0f4"
}

fn render_url_preview(
    entry_ix: usize,
    status: ElicitationStatus,
    _window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let request = acp::CreateElicitationRequest::new(
        acp::ElicitationUrlMode::new(
            preview_request_scope(entry_ix),
            acp::ElicitationId::new(format!("preview-url-{entry_ix}")),
            preview_url(),
        ),
        "Authorize Zed in your browser to finish signing in.",
    );

    render_preview_card(entry_ix, request, status, None, cx)
}

fn render_preview_card(
    entry_ix: usize,
    request: acp::CreateElicitationRequest,
    status: ElicitationStatus,
    form_state: Option<&ElicitationFormState>,
    cx: &App,
) -> AnyElement {
    let elicitation = Elicitation {
        id: ElicitationEntryId(format!("preview-elicitation-{entry_ix}").into()),
        request,
        status,
    };

    div()
        .w_full()
        .max_w(px(640.))
        .child(
            ElicitationCard::new(
                entry_ix,
                &elicitation,
                "Example Agent".into(),
                form_state,
                ElicitationCardHandlers::noop(),
            )
            .render(cx),
        )
        .into_any_element()
}

fn pending_status() -> ElicitationStatus {
    let (respond_tx, _response_rx) = oneshot::channel();
    ElicitationStatus::Pending { respond_tx }
}

fn preview_request_scope(index: usize) -> acp::ElicitationRequestScope {
    acp::ElicitationRequestScope::new(acp::RequestId::Number(index as i64))
}

fn preview_form_schema() -> acp::ElicitationSchema {
    acp::ElicitationSchema::new()
        .property(
            "account_name",
            acp::StringPropertySchema::new()
                .title("Account Name")
                .description("Used to label this connection in the agent panel.")
                .default_value("Work"),
            true,
        )
        .property(
            "environment",
            acp::StringPropertySchema::new()
                .title("Environment")
                .description("Select the environment this credential should target.")
                .one_of(vec![
                    acp::EnumOption::new("production", "Production")
                        .description("Use the live account and production resources."),
                    acp::EnumOption::new("staging", "Staging")
                        .description("Validate changes against staging data first."),
                    acp::EnumOption::new("development", "Development"),
                ])
                .default_value("staging"),
            true,
        )
        .property(
            "scopes",
            acp::MultiSelectPropertySchema::titled(vec![
                acp::EnumOption::new("profile", "Profile")
                    .description("Read account identity and basic profile details."),
                acp::EnumOption::new("repository", "Repository Access")
                    .description("Read and update repositories connected to this account."),
                acp::EnumOption::new("terminal", "Terminal Commands"),
            ])
            .title("Access")
            .description("Choose what the agent can use for this authorization.")
            .min_items(1)
            .default_value(vec!["profile".to_string(), "repository".to_string()]),
            true,
        )
        .property(
            "remember",
            acp::BooleanPropertySchema::new()
                .title("Remember Authorization")
                .description("Store this authorization for future sessions.")
                .default_value(true),
            false,
        )
}

fn single_select_options(schema: &acp::StringPropertySchema) -> Vec<ElicitationOption> {
    if let Some(options) = &schema.one_of {
        return options
            .iter()
            .map(|option| ElicitationOption {
                value: option.value.clone(),
                label: SharedString::from(option.title.clone()),
                description: option.description.clone().map(SharedString::from),
            })
            .collect();
    }

    schema
        .enum_values
        .as_deref()
        .unwrap_or_default()
        .iter()
        .map(|value| ElicitationOption {
            value: value.clone(),
            label: SharedString::from(value.clone()),
            description: None,
        })
        .collect()
}

fn single_select_default_value(
    schema: &acp::StringPropertySchema,
    options: &[ElicitationOption],
) -> Option<String> {
    schema
        .default
        .as_ref()
        .filter(|default| {
            options
                .iter()
                .any(|option| option.value.as_str() == default.as_str())
        })
        .cloned()
}

fn multi_select_options(schema: &acp::MultiSelectPropertySchema) -> Vec<ElicitationOption> {
    match &schema.items {
        acp::MultiSelectItems::String(items) => items
            .values
            .iter()
            .map(|value| ElicitationOption {
                value: value.clone(),
                label: SharedString::from(value.clone()),
                description: None,
            })
            .collect(),
        acp::MultiSelectItems::Titled(items) => items
            .options
            .iter()
            .map(|option| ElicitationOption {
                value: option.value.clone(),
                label: SharedString::from(option.title.clone()),
                description: option.description.clone().map(SharedString::from),
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn validate_number_value(
    title: SharedString,
    schema: &acp::NumberPropertySchema,
    value: &str,
) -> Result<f64, SharedString> {
    let parsed = value
        .parse::<f64>()
        .map_err(|_| SharedString::from(format!("{title} must be a number")))?;
    if !parsed.is_finite() {
        return Err(format!("{title} must be a finite number").into());
    }
    if let Some(minimum) = schema.minimum
        && parsed < minimum
    {
        return Err(format!("{title} must be at least {minimum}").into());
    }
    if let Some(maximum) = schema.maximum
        && parsed > maximum
    {
        return Err(format!("{title} must be at most {maximum}").into());
    }

    Ok(parsed)
}

fn validate_integer_value(
    title: SharedString,
    schema: &acp::IntegerPropertySchema,
    value: &str,
) -> Result<i64, SharedString> {
    let parsed = value
        .parse::<i64>()
        .map_err(|_| SharedString::from(format!("{title} must be an integer")))?;
    if let Some(minimum) = schema.minimum
        && parsed < minimum
    {
        return Err(format!("{title} must be at least {minimum}").into());
    }
    if let Some(maximum) = schema.maximum
        && parsed > maximum
    {
        return Err(format!("{title} must be at most {maximum}").into());
    }

    Ok(parsed)
}

fn validate_string_value(
    title: SharedString,
    schema: &acp::StringPropertySchema,
    value: &str,
) -> Result<(), SharedString> {
    let length = value.chars().count();
    if schema
        .min_length
        .is_some_and(|min_length| length < min_length as usize)
    {
        return Err(format!("{title} is too short").into());
    }
    if schema
        .max_length
        .is_some_and(|max_length| length > max_length as usize)
    {
        return Err(format!("{title} is too long").into());
    }

    validate_string_pattern_and_format(title, schema, value)
}

fn validate_single_select_value(
    title: SharedString,
    schema: &acp::StringPropertySchema,
    value: &str,
) -> Result<(), SharedString> {
    let options = single_select_options(schema);
    if options.iter().any(|option| option.value.as_str() == value) {
        Ok(())
    } else {
        Err(format!("{title} must be one of the provided options").into())
    }
}

const MAX_ELICITATION_PATTERN_BYTES: usize = 16 * 1024;
const MAX_ELICITATION_PATTERN_INPUT_BYTES: usize = 1024 * 1024;
const ELICITATION_PATTERN_BACKTRACK_LIMIT: usize = 10_000;
const ELICITATION_PATTERN_COMPILED_SIZE_LIMIT: usize = 1024 * 1024;
const ELICITATION_PATTERN_DFA_SIZE_LIMIT: usize = 1024 * 1024;

fn validate_string_pattern_and_format(
    title: SharedString,
    schema: &acp::StringPropertySchema,
    value: &str,
) -> Result<(), SharedString> {
    if schema.pattern.is_none() && schema.format.and_then(string_format_json_name).is_none() {
        return Ok(());
    }
    if schema
        .pattern
        .as_ref()
        .is_some_and(|pattern| pattern.len() > MAX_ELICITATION_PATTERN_BYTES)
    {
        return Err(format!("{title} has an invalid validation pattern").into());
    }
    if schema.pattern.is_some() && value.len() > MAX_ELICITATION_PATTERN_INPUT_BYTES {
        return Err(format!("{title} is too long to validate safely").into());
    }

    let mut validation_schema = serde_json::Map::new();
    validation_schema.insert(
        "type".to_string(),
        serde_json::Value::String("string".into()),
    );
    if let Some(pattern) = &schema.pattern {
        validation_schema.insert(
            "pattern".to_string(),
            serde_json::Value::String(pattern.clone()),
        );
    }
    if let Some(format) = schema.format.and_then(string_format_json_name) {
        validation_schema.insert(
            "format".to_string(),
            serde_json::Value::String(format.into()),
        );
    }

    let validation_schema = serde_json::Value::Object(validation_schema);
    let validator = jsonschema::options()
        .should_validate_formats(true)
        .with_pattern_options(
            jsonschema::PatternOptions::fancy_regex()
                .backtrack_limit(ELICITATION_PATTERN_BACKTRACK_LIMIT)
                .size_limit(ELICITATION_PATTERN_COMPILED_SIZE_LIMIT)
                .dfa_size_limit(ELICITATION_PATTERN_DFA_SIZE_LIMIT),
        )
        .build(&validation_schema)
        .map_err(|_| {
            if schema.pattern.is_some() {
                format!("{title} has an invalid validation pattern")
            } else {
                format!("{title} has an invalid validation format")
            }
        })?;
    let value = serde_json::Value::String(value.to_string());
    if let Err(error) = validator.validate(&value) {
        if matches!(
            error.kind(),
            jsonschema::error::ValidationErrorKind::BacktrackLimitExceeded { .. }
        ) {
            return Err(format!("{title} has a validation pattern that is too complex").into());
        }
    } else {
        return Ok(());
    }

    match (
        schema.pattern.is_some(),
        schema.format.and_then(string_format_label),
    ) {
        (true, Some(_)) => Err(format!("{title} does not match the requested constraints").into()),
        (true, None) => Err(format!("{title} does not match the requested pattern").into()),
        (false, Some(format)) => Err(format!("{title} must be {format}").into()),
        (false, None) => Ok(()),
    }
}

fn string_format_json_name(format: acp::StringFormat) -> Option<&'static str> {
    match format {
        acp::StringFormat::Email => Some("email"),
        acp::StringFormat::Uri => Some("uri"),
        acp::StringFormat::Date => Some("date"),
        acp::StringFormat::DateTime => Some("date-time"),
        _ => None,
    }
}

fn string_format_label(format: acp::StringFormat) -> Option<&'static str> {
    match format {
        acp::StringFormat::Email => Some("an email address"),
        acp::StringFormat::Uri => Some("a URI"),
        acp::StringFormat::Date => Some("a date"),
        acp::StringFormat::DateTime => Some("a date and time"),
        _ => None,
    }
}

fn property_title(name: &str, property: &acp::ElicitationPropertySchema) -> SharedString {
    let title = match property {
        acp::ElicitationPropertySchema::String(schema) => schema.title.as_deref(),
        acp::ElicitationPropertySchema::Number(schema) => schema.title.as_deref(),
        acp::ElicitationPropertySchema::Integer(schema) => schema.title.as_deref(),
        acp::ElicitationPropertySchema::Boolean(schema) => schema.title.as_deref(),
        acp::ElicitationPropertySchema::Array(schema) => schema.title.as_deref(),
        _ => None,
    };
    SharedString::from(title.unwrap_or(name).to_string())
}

fn property_description(property: &acp::ElicitationPropertySchema) -> Option<SharedString> {
    match property {
        acp::ElicitationPropertySchema::String(schema) => schema.description.clone(),
        acp::ElicitationPropertySchema::Number(schema) => schema.description.clone(),
        acp::ElicitationPropertySchema::Integer(schema) => schema.description.clone(),
        acp::ElicitationPropertySchema::Boolean(schema) => schema.description.clone(),
        acp::ElicitationPropertySchema::Array(schema) => schema.description.clone(),
        _ => None,
    }
    .map(SharedString::from)
}

type TextClickHandler = Rc<dyn Fn(&mut Window, &mut App)>;

struct ElicitationOptionView {
    option: ElicitationOption,
    label: ElicitationText,
    description: Option<ElicitationText>,
    focus_handle: FocusHandle,
}

impl ElicitationOptionView {
    fn new(option: ElicitationOption, window: &mut Window, cx: &mut App) -> Self {
        Self {
            label: ElicitationText::new(option.label.clone(), Color::Default, window, cx),
            description: option.description.as_ref().map(|description| {
                ElicitationText::new(description.clone(), Color::Muted, window, cx)
            }),
            focus_handle: cx.focus_handle(),
            option,
        }
    }

    fn render(&self, on_click: TextClickHandler) -> Div {
        let focus_handle = self.focus_handle.clone();
        let on_click: TextClickHandler = Rc::new(move |window, cx| {
            window.focus(&focus_handle, cx);
            on_click(window, cx);
        });
        let label_value = self.option.value.clone();
        let description_value = self.option.value.clone();
        v_flex()
            .gap_0p5()
            .child(
                div()
                    .debug_selector(move || format!("elicitation-option-label-{label_value}"))
                    .child(self.label.clone().on_click(on_click.clone())),
            )
            .when_some(self.description.clone(), |content, description| {
                content.child(
                    div()
                        .debug_selector(move || {
                            format!("elicitation-option-description-{description_value}")
                        })
                        .child(description.on_click(on_click)),
                )
            })
    }
}

#[derive(Clone, IntoElement)]
struct ElicitationText {
    editor: Entity<Editor>,
    color: Color,
    pending_click: Rc<Cell<bool>>,
    on_click: Option<TextClickHandler>,
}

impl ElicitationText {
    fn new(text: SharedString, color: Color, window: &mut Window, cx: &mut App) -> Self {
        Self {
            editor: cx.new(|cx| {
                let buffer = cx.new(|cx| {
                    let mut buffer = Buffer::local(text, cx);
                    buffer.set_capability(Capability::ReadOnly, cx);
                    buffer
                });
                let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
                let mut editor = Editor::new(
                    EditorMode::AutoHeight {
                        min_lines: 1,
                        max_lines: None,
                    },
                    buffer,
                    None,
                    window,
                    cx,
                );
                editor.set_read_only(true);
                editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
                editor.set_offset_content(false, cx);
                editor.set_show_cursor_when_unfocused(false, cx);
                editor.disable_scrollbars_and_minimap(window, cx);
                editor.set_custom_context_menu(|editor, _, window, cx| {
                    let has_selection =
                        editor.has_non_empty_selection(&editor.display_snapshot(cx));
                    let focus_handle = editor.focus_handle(cx);
                    Some(ContextMenu::build(window, cx, |menu, _, _| {
                        menu.context(focus_handle).action_disabled_when(
                            !has_selection,
                            "Copy",
                            Box::new(editor::actions::Copy),
                        )
                    }))
                });
                editor
            }),
            color,
            pending_click: Rc::default(),
            on_click: None,
        }
    }

    fn on_click(mut self, on_click: TextClickHandler) -> Self {
        self.on_click = Some(on_click);
        self
    }
}

impl RenderOnce for ElicitationText {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.editor.update(cx, |editor, cx| {
            editor.set_text_style_refinement(TextStyleRefinement {
                font_size: Some(ui::TextSize::Small.rems(cx).into()),
                line_height: Some(window.text_style().line_height),
                color: Some(self.color.color(cx)),
                ..TextStyleRefinement::default()
            });
        });
        div()
            .id(("elicitation-text", self.editor.entity_id()))
            .track_focus(&self.editor.focus_handle(cx))
            .capture_any_mouse_down({
                let editor = self.editor.clone();
                let pending_click = self.pending_click.clone();
                move |event, window, cx| {
                    pending_click.set(
                        event.button == MouseButton::Left
                            && event.click_count == 1
                            && !event.modifiers.modified(),
                    );
                    if event.button == MouseButton::Left
                        && (event.modifiers.secondary() || event.modifiers.alt)
                    {
                        cx.stop_propagation();
                        return;
                    }
                    if event.button == MouseButton::Left && !event.modifiers.modified() {
                        editor.update(cx, |editor, cx| {
                            editor.clear_selection_drag_state();
                            editor.change_selections(
                                SelectionEffects::no_scroll(),
                                window,
                                cx,
                                |selections| {
                                    selections.select_ranges([
                                        MultiBufferOffset(0)..MultiBufferOffset(0)
                                    ]);
                                },
                            );
                        });
                    }
                }
            })
            .on_mouse_down_out({
                let pending_click = self.pending_click.clone();
                move |_, _, _| pending_click.set(false)
            })
            .on_hover({
                let pending_click = self.pending_click.clone();
                move |hovered, _, _| {
                    if !hovered {
                        pending_click.set(false);
                    }
                }
            })
            .on_mouse_move({
                let editor = self.editor.clone();
                let pending_click = self.pending_click.clone();
                move |_, _, cx| {
                    if editor.read(cx).has_pending_nonempty_selection() {
                        pending_click.set(false);
                    }
                }
            })
            .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                if self.pending_click.replace(false)
                    && let Some(on_click) = &self.on_click
                {
                    on_click(window, cx);
                }
            })
            .child(self.editor)
    }
}

#[derive(IntoElement)]
struct ElicitationLabel {
    id: ElementId,
    role: SharedString,
    text: SharedString,
    color: Color,
    on_click: Option<TextClickHandler>,
}

impl ElicitationLabel {
    fn on_click(mut self, on_click: TextClickHandler) -> Self {
        self.on_click = Some(on_click);
        self
    }
}

impl RenderOnce for ElicitationLabel {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state((self.id, self.text.clone()), cx, |window, cx| {
            ElicitationText::new(self.text, self.color, window, cx)
        });
        let mut text = state.read(cx).clone();
        text.color = self.color;
        text.on_click = self.on_click;
        div()
            .w_full()
            .min_w_0()
            .debug_selector(move || format!("elicitation-{}", self.role))
            .child(text)
    }
}

type RespondHandler = Rc<dyn Fn(ElicitationEntryId, &mut Window, &mut App)>;
type OpenUrlHandler = Rc<dyn Fn(ElicitationEntryId, String, &mut Window, &mut App)>;
type BooleanHandler = Rc<dyn Fn(ElicitationEntryId, String, bool, &mut App)>;
type SelectHandler = Rc<dyn Fn(ElicitationEntryId, String, String, &mut App)>;
type MultiSelectHandler = Rc<dyn Fn(ElicitationEntryId, String, String, bool, &mut App)>;

#[derive(Clone)]
pub(crate) struct ElicitationCardHandlers {
    on_submit: RespondHandler,
    on_decline: RespondHandler,
    on_cancel: RespondHandler,
    on_dismiss_url: RespondHandler,
    on_open_url: OpenUrlHandler,
    on_boolean_change: BooleanHandler,
    on_single_select_change: SelectHandler,
    on_multi_select_change: MultiSelectHandler,
}

impl ElicitationCardHandlers {
    pub(crate) fn new(
        on_submit: impl Fn(ElicitationEntryId, &mut Window, &mut App) + 'static,
        on_decline: impl Fn(ElicitationEntryId, &mut Window, &mut App) + 'static,
        on_cancel: impl Fn(ElicitationEntryId, &mut Window, &mut App) + 'static,
        on_dismiss_url: impl Fn(ElicitationEntryId, &mut Window, &mut App) + 'static,
        on_open_url: impl Fn(ElicitationEntryId, String, &mut Window, &mut App) + 'static,
        on_boolean_change: impl Fn(ElicitationEntryId, String, bool, &mut App) + 'static,
        on_single_select_change: impl Fn(ElicitationEntryId, String, String, &mut App) + 'static,
        on_multi_select_change: impl Fn(ElicitationEntryId, String, String, bool, &mut App) + 'static,
    ) -> Self {
        Self {
            on_submit: Rc::new(on_submit),
            on_decline: Rc::new(on_decline),
            on_cancel: Rc::new(on_cancel),
            on_dismiss_url: Rc::new(on_dismiss_url),
            on_open_url: Rc::new(on_open_url),
            on_boolean_change: Rc::new(on_boolean_change),
            on_single_select_change: Rc::new(on_single_select_change),
            on_multi_select_change: Rc::new(on_multi_select_change),
        }
    }

    pub(crate) fn noop() -> Self {
        Self::new(
            |_, _, _| {},
            |_, _, _| {},
            |_, _, _| {},
            |_, _, _| {},
            |_, _, _, _| {},
            |_, _, _, _| {},
            |_, _, _, _| {},
            |_, _, _, _, _| {},
        )
    }
}

pub(crate) fn should_render_elicitation(elicitation: &Elicitation) -> bool {
    matches!(
        (&elicitation.status, &elicitation.request.mode),
        (
            ElicitationStatus::Pending { .. },
            acp::ElicitationMode::Form(_) | acp::ElicitationMode::Url(_)
        ) | (ElicitationStatus::Accepted, acp::ElicitationMode::Url(_))
    )
}

#[derive(Debug, PartialEq, Eq)]
struct UrlHostPresentation {
    host: String,
    suspicious_decoded_host: Option<String>,
}

fn url_host_presentation(url: &str) -> Option<UrlHostPresentation> {
    let url = url::Url::parse(url).ok()?;
    let host = url.host_str()?.to_string();
    let (decoded_host, suspicious_characters) = crate::unicode_confusables::scan_host(&host);

    Some(UrlHostPresentation {
        host,
        suspicious_decoded_host: (!suspicious_characters.is_empty()).then_some(decoded_host),
    })
}

pub(crate) struct ElicitationCard<'a> {
    entry_ix: usize,
    elicitation: &'a Elicitation,
    requester_name: SharedString,
    form_state: Option<&'a ElicitationFormState>,
    handlers: ElicitationCardHandlers,
}

impl<'a> ElicitationCard<'a> {
    pub(crate) fn new(
        entry_ix: usize,
        elicitation: &'a Elicitation,
        requester_name: SharedString,
        form_state: Option<&'a ElicitationFormState>,
        handlers: ElicitationCardHandlers,
    ) -> Self {
        Self {
            entry_ix,
            elicitation,
            requester_name,
            form_state,
            handlers,
        }
    }

    pub(crate) fn render(self, cx: &App) -> Div {
        let border_color = cx.theme().colors().border.opacity(0.8);
        let header_background = cx
            .theme()
            .colors()
            .element_background
            .blend(cx.theme().colors().editor_foreground.opacity(0.025));
        let tool_name_font_size = rems_from_px(13_f32);
        let is_pending = matches!(&self.elicitation.status, ElicitationStatus::Pending { .. });
        let is_accepted_url = matches!(
            (&self.elicitation.status, &self.elicitation.request.mode),
            (ElicitationStatus::Accepted, acp::ElicitationMode::Url(_))
        );
        let (status_label, status_icon, status_color) = match &self.elicitation.status {
            ElicitationStatus::Pending { .. } => ("Waiting for input", IconName::Info, Color::Info),
            ElicitationStatus::Accepted if is_accepted_url => {
                ("Waiting for completion", IconName::Info, Color::Info)
            }
            ElicitationStatus::Accepted => ("Submitted", IconName::Check, Color::Success),
            ElicitationStatus::Declined => ("Declined", IconName::Close, Color::Muted),
            ElicitationStatus::Canceled => ("Canceled", IconName::Circle, Color::Muted),
            ElicitationStatus::Completed => ("Completed", IconName::Check, Color::Success),
        };

        let body = v_flex().gap_2().p_3().child(self.text(
            "question",
            self.elicitation.request.message.clone(),
            Color::Default,
        ));
        let body = match &self.elicitation.request.mode {
            acp::ElicitationMode::Form(mode) if is_pending => {
                body.child(self.render_form(mode, cx))
            }
            acp::ElicitationMode::Url(mode) if is_pending || is_accepted_url => {
                body.child(self.render_url_elicitation(mode))
            }
            _ => body,
        };

        v_flex()
            .key_context("Elicitation")
            .tab_group()
            .on_action(|_: &menu::SelectNext, window, cx| window.focus_next(cx))
            .on_action(|_: &menu::SelectPrevious, window, cx| window.focus_prev(cx))
            .mx_5()
            .my_1p5()
            .rounded_md()
            .border_1()
            .border_color(border_color)
            .overflow_hidden()
            .child(
                h_flex()
                    .h_8()
                    .p_1()
                    .w_full()
                    .justify_between()
                    .bg(header_background)
                    .child(
                        h_flex()
                            .min_w_0()
                            .gap_1p5()
                            .px_1()
                            .child(
                                Icon::new(status_icon)
                                    .size(IconSize::Small)
                                    .color(status_color),
                            )
                            .child(
                                Label::new(format!("Input Requested by {}", self.requester_name))
                                    .size(LabelSize::Custom(tool_name_font_size))
                                    .truncate(),
                            ),
                    )
                    .child(
                        Label::new(status_label)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(body)
            .when(is_pending || is_accepted_url, |this| {
                this.child(self.render_actions(cx))
            })
    }

    fn text(
        &self,
        role: impl Into<SharedString>,
        text: impl Into<SharedString>,
        color: Color,
    ) -> ElicitationLabel {
        let role = role.into();
        ElicitationLabel {
            id: ElementId::from((
                ElementId::from((
                    ElementId::from("elicitation"),
                    SharedString::from(self.elicitation.id.0.clone()),
                )),
                role.clone(),
            )),
            role,
            text: text.into(),
            color,
            on_click: None,
        }
    }

    fn render_form(&self, mode: &acp::ElicitationFormMode, cx: &App) -> AnyElement {
        let Some(state) = self.form_state else {
            return Empty.into_any_element();
        };

        v_flex()
            .gap_2()
            .when_some(mode.requested_schema.title.clone(), |this, title| {
                this.child(self.text("title", title, Color::Default))
            })
            .when_some(
                mode.requested_schema.description.clone(),
                |this, description| this.child(self.text("description", description, Color::Muted)),
            )
            .children(mode.requested_schema.properties.iter().filter_map(
                |(field_name, property)| {
                    let field = state.fields.get(field_name)?;
                    Some(self.render_field(
                        field_name,
                        property,
                        field,
                        state.field_errors.get(field_name),
                        cx,
                    ))
                },
            ))
            .into_any_element()
    }

    fn render_field(
        &self,
        field_name: &str,
        property: &acp::ElicitationPropertySchema,
        field: &ElicitationFieldState,
        error: Option<&SharedString>,
        cx: &App,
    ) -> AnyElement {
        let label = property_title(field_name, property);
        let description = property_description(property);
        let border_color = cx.theme().colors().border.opacity(0.8);
        let focused_border_color = cx.theme().colors().border_focused;
        let field_border_color = if error.is_some() {
            Color::Error.color(cx)
        } else {
            border_color
        };
        let editor_background = cx.theme().colors().editor_background;
        let label_color = if error.is_some() {
            Color::Error
        } else {
            Color::Default
        };

        if let ElicitationFieldState::Boolean {
            value,
            focus_handle,
        } = field
        {
            let checkbox_state = if *value {
                ToggleState::Selected
            } else {
                ToggleState::Unselected
            };
            let next_value = !*value;
            let on_boolean_change = self.handlers.on_boolean_change.clone();
            let elicitation_id = self.elicitation.id.clone();
            let field_name = field_name.to_string();
            let row_id = format!("elicitation-bool-row-{}-{field_name}", self.entry_ix);
            let checkbox_id = format!("elicitation-bool-{}-{field_name}", self.entry_ix);
            let on_click: TextClickHandler = Rc::new({
                let field_name = field_name.clone();
                let focus_handle = focus_handle.clone();
                move |window, cx| {
                    window.focus(&focus_handle, cx);
                    on_boolean_change(elicitation_id.clone(), field_name.clone(), next_value, cx);
                }
            });

            return v_flex()
                .gap_1()
                .child(
                    h_flex()
                        .id(row_id)
                        .track_focus(&focus_handle.clone().tab_stop(true).tab_index(0))
                        .w_full()
                        .items_start()
                        .gap_1()
                        .cursor_pointer()
                        .focus_visible(|this| this.bg(cx.theme().colors().element_hover))
                        .on_click({
                            let on_click = on_click.clone();
                            move |_, window, cx| on_click(window, cx)
                        })
                        .child(div().child(Checkbox::new(checkbox_id, checkbox_state)))
                        .child(
                            v_flex()
                                .min_w_0()
                                .flex_1()
                                .gap_0p5()
                                .child(
                                    self.text(
                                        format!("field-title-{field_name}"),
                                        label,
                                        label_color,
                                    )
                                    .on_click(on_click.clone()),
                                )
                                .when_some(description, |content, description| {
                                    content.child(
                                        self.text(
                                            format!("field-description-{field_name}"),
                                            description,
                                            Color::Muted,
                                        )
                                        .on_click(on_click),
                                    )
                                }),
                        ),
                )
                .when_some(error.cloned(), |content, error| {
                    content.child(self.text(
                        format!("field-error-{field_name}"),
                        error,
                        Color::Error,
                    ))
                })
                .into_any_element();
        }

        let group_label = label.clone();
        let label = self.text(format!("field-title-{field_name}"), label, label_color);

        v_flex()
            .gap_1()
            .child(label)
            .when_some(description, |this, description| {
                this.child(self.text(
                    format!("field-description-{field_name}"),
                    description,
                    Color::Muted,
                ))
            })
            .child(match field {
                ElicitationFieldState::Text(editor) => {
                    let on_submit = self.handlers.on_submit.clone();
                    let elicitation_id = self.elicitation.id.clone();
                    let is_submitting = self.form_state.is_some_and(|state| state.is_submitting);

                    div()
                        .track_focus(&editor.focus_handle(cx).tab_stop(true))
                        .on_action(move |_: &menu::Confirm, window, cx| {
                            if !is_submitting {
                                on_submit(elicitation_id.clone(), window, cx);
                            }
                        })
                        .rounded_sm()
                        .border_1()
                        .border_color(field_border_color)
                        .focus_visible(|this| this.border_color(focused_border_color))
                        .bg(editor_background)
                        .px_1()
                        .py_0p5()
                        .text_xs()
                        .child(editor.clone().into_any_element())
                        .into_any_element()
                }
                ElicitationFieldState::Boolean { .. } => Empty.into_any_element(),
                ElicitationFieldState::SingleSelect { value } => {
                    let options = self.option_views(field_name);
                    self.render_single_select(
                        field_name,
                        group_label,
                        value.as_ref(),
                        options,
                        error.is_some(),
                    )
                }
                ElicitationFieldState::MultiSelect(selected) => {
                    let options = self.option_views(field_name);
                    v_flex()
                        .id(format!("elicitation-multi-{}-{field_name}", self.entry_ix))
                        .role(Role::Group)
                        .aria_label(group_label)
                        .gap_1()
                        .children(options.iter().map(|option_view| {
                            let option = &option_view.option;
                            let is_selected = selected.contains(&option.value);
                            let on_multi_select_change =
                                self.handlers.on_multi_select_change.clone();
                            let elicitation_id = self.elicitation.id.clone();
                            let field_name = field_name.to_string();
                            let card_id = format!(
                                "elicitation-multi-option-{}-{field_name}-{}",
                                self.entry_ix, option.value
                            );
                            let value = option.value.clone();
                            let on_click: TextClickHandler = Rc::new(move |_, cx| {
                                on_multi_select_change(
                                    elicitation_id.clone(),
                                    field_name.clone(),
                                    value.clone(),
                                    !is_selected,
                                    cx,
                                );
                            });
                            ChoiceCard::checkbox(card_id, option.label.clone(), is_selected)
                                .content(option_view.render(on_click.clone()))
                                .focus_handle(option_view.focus_handle.clone())
                                .when_some(option.description.clone(), |card, description| {
                                    card.description(description)
                                })
                                .invalid(error.is_some())
                                .on_click(move |_, window, cx| on_click(window, cx))
                        }))
                        .into_any_element()
                }
            })
            .when_some(error.cloned(), |content, error| {
                content.child(self.text(format!("field-error-{field_name}"), error, Color::Error))
            })
            .into_any_element()
    }

    fn render_single_select(
        &self,
        field_name: &str,
        group_label: SharedString,
        selected_value: Option<&String>,
        options: &[ElicitationOptionView],
        has_error: bool,
    ) -> AnyElement {
        let entry_ix = self.entry_ix;
        let elicitation_id = self.elicitation.id.clone();
        let field_name = field_name.to_string();
        let on_single_select_change = self.handlers.on_single_select_change.clone();

        v_flex()
            .id(format!("elicitation-select-{entry_ix}-{field_name}"))
            .role(Role::RadioGroup)
            .aria_label(group_label)
            .gap_1()
            .children(options.iter().map(|option_view| {
                let option = &option_view.option;
                let card_id = format!(
                    "elicitation-select-option-{entry_ix}-{field_name}-{}",
                    option.value
                );
                let is_selected =
                    selected_value.is_some_and(|selected_value| selected_value == &option.value);
                let option_value = option.value.clone();
                let elicitation_id = elicitation_id.clone();
                let field_name = field_name.clone();
                let on_single_select_change = on_single_select_change.clone();

                let on_click: TextClickHandler = Rc::new(move |_, cx| {
                    on_single_select_change(
                        elicitation_id.clone(),
                        field_name.clone(),
                        option_value.clone(),
                        cx,
                    );
                });
                ChoiceCard::radio(card_id, option.label.clone(), is_selected)
                    .content(option_view.render(on_click.clone()))
                    .focus_handle(option_view.focus_handle.clone())
                    .when_some(option.description.clone(), |card, description| {
                        card.description(description)
                    })
                    .invalid(has_error)
                    .on_click(move |_, window, cx| on_click(window, cx))
            }))
            .into_any_element()
    }

    fn option_views(&self, field_name: &str) -> &[ElicitationOptionView] {
        self.form_state
            .and_then(|state| state.option_views.get(field_name))
            .map(Vec::as_slice)
            .unwrap_or_default()
    }

    fn render_url_elicitation(&self, mode: &acp::ElicitationUrlMode) -> AnyElement {
        v_flex()
            .gap_2()
            .when_some(url_host_presentation(&mode.url), |this, presentation| {
                this.child(
                    v_flex()
                        .gap_1()
                        .child(
                            h_flex()
                                .gap_1()
                                .child(
                                    Label::new("Destination")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(div().min_w_0().flex_1().child(self.text("url-host", presentation.host, Color::Default))),
                        )
                        .when_some(
                            presentation.suspicious_decoded_host,
                            |this, decoded_host| {
                                this.child(
                                    h_flex()
                                        .items_start()
                                        .gap_1()
                                        .child(
                                            Icon::new(IconName::Warning)
                                                .size(IconSize::XSmall)
                                                .color(Color::Warning),
                                        )
                                        .child(
                                            div().min_w_0().flex_1().child(self.text("url-warning", format!(
                                                "This internationalized address displays as {decoded_host}. Verify it carefully."
                                            ), Color::Warning)),
                                        ),
                                )
                            },
                        ),
                )
            })
            .child(self.render_url_summary(&mode.url))
            .into_any_element()
    }

    fn render_url_summary(&self, url: &str) -> AnyElement {
        h_flex()
            .gap_1()
            .w_full()
            .min_w_0()
            .items_start()
            .child(
                div().h(rems_from_px(16_f32)).flex().items_center().child(
                    Icon::new(IconName::Link)
                        .size(IconSize::XSmall)
                        .color(Color::Muted),
                ),
            )
            .child(
                div()
                    .min_w_0()
                    .flex_1()
                    .child(self.text("url", url.to_string(), Color::Muted)),
            )
            .into_any_element()
    }

    fn render_actions(&self, cx: &App) -> AnyElement {
        let open_url = match &self.elicitation.request.mode {
            acp::ElicitationMode::Url(mode) => Some(mode.url.clone()),
            _ => None,
        };
        let is_accepted_url =
            open_url.is_some() && matches!(self.elicitation.status, ElicitationStatus::Accepted);
        let is_submitting = self.form_state.is_some_and(|state| state.is_submitting);
        let (accept_label, accept_icon, accept_icon_color) = if is_accepted_url {
            ("Open Again", IconName::ArrowUpRight, Color::Muted)
        } else if open_url.is_some() {
            ("Open", IconName::ArrowUpRight, Color::Muted)
        } else {
            ("Submit", IconName::Check, Color::Success)
        };
        let border_color = cx.theme().colors().border.opacity(0.8);
        let on_submit = self.handlers.on_submit.clone();
        let on_open_url = self.handlers.on_open_url.clone();
        let on_decline = self.handlers.on_decline.clone();
        let on_cancel = self.handlers.on_cancel.clone();
        let on_dismiss_url = self.handlers.on_dismiss_url.clone();
        let submit_id = self.elicitation.id.clone();
        let decline_id = self.elicitation.id.clone();
        let cancel_id = self.elicitation.id.clone();
        let dismiss_id = self.elicitation.id.clone();

        h_flex()
            .w_full()
            .p_1()
            .gap_1()
            .justify_end()
            .border_t_1()
            .border_color(border_color)
            .child(
                Button::new(("elicitation-accept", self.entry_ix), accept_label)
                    .tab_index(0_isize)
                    .start_icon(
                        Icon::new(accept_icon)
                            .size(IconSize::XSmall)
                            .color(accept_icon_color),
                    )
                    .label_size(LabelSize::Small)
                    .disabled(is_submitting)
                    .on_click(move |_, window, cx| {
                        if let Some(url) = &open_url {
                            on_open_url(submit_id.clone(), url.clone(), window, cx);
                            if !is_accepted_url {
                                on_submit(submit_id.clone(), window, cx);
                            }
                        } else {
                            on_submit(submit_id.clone(), window, cx);
                        }
                    }),
            )
            .when(!is_accepted_url, |this| {
                this.child(
                    Button::new(("elicitation-decline", self.entry_ix), "Decline")
                        .tab_index(0_isize)
                        .start_icon(
                            Icon::new(IconName::Close)
                                .size(IconSize::XSmall)
                                .color(Color::Error),
                        )
                        .label_size(LabelSize::Small)
                        .on_click(move |_, window, cx| {
                            on_decline(decline_id.clone(), window, cx);
                        }),
                )
                .child(
                    Button::new(("elicitation-cancel", self.entry_ix), "Cancel")
                        .tab_index(0_isize)
                        .label_size(LabelSize::Small)
                        .on_click(move |_, window, cx| {
                            on_cancel(cancel_id.clone(), window, cx);
                        }),
                )
            })
            .when(is_accepted_url, |this| {
                this.child(
                    Button::new(("elicitation-dismiss-url", self.entry_ix), "Cancel")
                        .tab_index(0_isize)
                        .label_size(LabelSize::Small)
                        .on_click(move |_, window, cx| {
                            on_dismiss_url(dismiss_id.clone(), window, cx);
                        }),
                )
            })
            .into_any_element()
    }
}
