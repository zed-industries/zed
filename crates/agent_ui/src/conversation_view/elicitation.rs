use acp_thread::{Elicitation, ElicitationEntryId, ElicitationStatus};
use agent_client_protocol::schema::v1 as acp;
use collections::{HashMap, HashSet};
use component::{Component, ComponentScope, example_group_with_title, single_example};
use editor::Editor;
use futures::channel::oneshot;
use gpui::{AnyElement, App, Div, Empty, Entity, Hsla, SharedString, Window, div};
use std::collections::BTreeMap;
use std::rc::Rc;
use ui::{
    Button, Checkbox, Color, Icon, IconName, IconSize, Indicator, Label, LabelSize, ToggleState,
    prelude::*,
};

#[derive(Clone)]
struct ElicitationOption {
    value: String,
    label: SharedString,
    description: Option<SharedString>,
}

enum ElicitationFieldState {
    Text(Entity<Editor>),
    Boolean(bool),
    SingleSelect { value: Option<String> },
    MultiSelect(HashSet<String>),
}

pub(crate) struct ElicitationFormState {
    fields: HashMap<String, ElicitationFieldState>,
    field_errors: HashMap<String, SharedString>,
}

impl ElicitationFormState {
    pub(crate) fn new(schema: &acp::ElicitationSchema, window: &mut Window, cx: &mut App) -> Self {
        let required = schema.required.as_deref().unwrap_or_default();
        let mut fields = HashMap::default();

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
                acp::ElicitationPropertySchema::Boolean(schema) => {
                    ElicitationFieldState::Boolean(schema.default.unwrap_or(false))
                }
                acp::ElicitationPropertySchema::Array(schema) => {
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
            field_errors: HashMap::default(),
        }
    }

    pub(crate) fn collect(
        &self,
        schema: &acp::ElicitationSchema,
        cx: &App,
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
                    ElicitationFieldState::Text(editor),
                ) => {
                    let value = editor.read(cx).text(cx).to_string();
                    if value.is_empty() {
                        if is_required {
                            Err(format!("{} is required", property_title(name, property)).into())
                        } else {
                            Ok(None)
                        }
                    } else {
                        validate_string_value(property_title(name, property), schema, &value)
                            .map(|()| Some(value.into()))
                    }
                }
                (
                    acp::ElicitationPropertySchema::String(schema),
                    ElicitationFieldState::SingleSelect { value },
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
                    ElicitationFieldState::Text(editor),
                ) => {
                    let value = editor.read(cx).text(cx).trim().to_string();
                    if value.is_empty() {
                        if is_required {
                            Err(format!("{} is required", property_title(name, property)).into())
                        } else {
                            Ok(None)
                        }
                    } else {
                        validate_number_value(property_title(name, property), schema, &value)
                            .map(|parsed| Some(parsed.into()))
                    }
                }
                (
                    acp::ElicitationPropertySchema::Integer(schema),
                    ElicitationFieldState::Text(editor),
                ) => {
                    let value = editor.read(cx).text(cx).trim().to_string();
                    if value.is_empty() {
                        if is_required {
                            Err(format!("{} is required", property_title(name, property)).into())
                        } else {
                            Ok(None)
                        }
                    } else {
                        validate_integer_value(property_title(name, property), schema, &value)
                            .map(|parsed| Some(parsed.into()))
                    }
                }
                (
                    acp::ElicitationPropertySchema::Boolean(schema),
                    ElicitationFieldState::Boolean(value),
                ) => {
                    if is_required || *value || schema.default.is_some() {
                        Ok(Some((*value).into()))
                    } else {
                        Ok(None)
                    }
                }
                (
                    acp::ElicitationPropertySchema::Array(schema),
                    ElicitationFieldState::MultiSelect(selected),
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

    pub(crate) fn set_errors(&mut self, errors: HashMap<String, SharedString>) {
        self.field_errors = errors;
    }

    pub(crate) fn set_field_error(
        &mut self,
        field_name: impl Into<String>,
        error: impl Into<SharedString>,
    ) {
        self.field_errors.insert(field_name.into(), error.into());
    }

    pub(crate) fn set_boolean(&mut self, field_name: &str, value: bool) {
        if let Some(ElicitationFieldState::Boolean(field)) = self.fields.get_mut(field_name) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

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
    }

    #[test]
    fn display_url_segments_never_elide_url_characters() {
        let url = format!(
            "https://auth.example.com/oauth/authorize?state={}",
            "a".repeat(MAX_URL_DISPLAY_SEGMENT_CHARS * 2)
        );

        let display_url_segments = display_url_segments(&url)
            .into_iter()
            .map(|segment| segment.to_string())
            .collect::<Vec<_>>();
        assert!(display_url_segments.len() > 1);
        assert!(
            !display_url_segments
                .iter()
                .any(|segment| segment.contains('…'))
        );
        assert!(
            display_url_segments
                .iter()
                .all(|segment| segment.chars().count() <= MAX_URL_DISPLAY_SEGMENT_CHARS)
        );
        assert_eq!(display_url_segments.concat(), url);
    }

    #[test]
    fn display_url_segments_use_larger_url_boundaries() {
        let url = "https://auth.example.com/oauth/authorize?client_id=zed-desktop&scope=repository";

        let display_url_segments = display_url_segments(url)
            .into_iter()
            .map(|segment| segment.to_string())
            .collect::<Vec<_>>();
        assert_eq!(display_url_segments.concat(), url);
        assert_eq!(display_url_segments[0], "https://auth.example.com/");
        assert!(
            display_url_segments
                .iter()
                .any(|segment| segment.ends_with('?'))
        );
        assert!(
            display_url_segments
                .iter()
                .any(|segment| segment.ends_with('&'))
        );
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

fn validate_string_pattern_and_format(
    title: SharedString,
    schema: &acp::StringPropertySchema,
    value: &str,
) -> Result<(), SharedString> {
    if schema.pattern.is_none() && schema.format.and_then(string_format_json_name).is_none() {
        return Ok(());
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
        .build(&validation_schema)
        .map_err(|_| {
            if schema.pattern.is_some() {
                format!("{title} has an invalid validation pattern")
            } else {
                format!("{title} has an invalid validation format")
            }
        })?;
    if validator.is_valid(&serde_json::Value::String(value.to_string())) {
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
        on_open_url: impl Fn(ElicitationEntryId, String, &mut Window, &mut App) + 'static,
        on_boolean_change: impl Fn(ElicitationEntryId, String, bool, &mut App) + 'static,
        on_single_select_change: impl Fn(ElicitationEntryId, String, String, &mut App) + 'static,
        on_multi_select_change: impl Fn(ElicitationEntryId, String, String, bool, &mut App) + 'static,
    ) -> Self {
        Self {
            on_submit: Rc::new(on_submit),
            on_decline: Rc::new(on_decline),
            on_cancel: Rc::new(on_cancel),
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
        (ElicitationStatus::Pending { .. }, _)
            | (ElicitationStatus::Accepted, acp::ElicitationMode::Url(_))
    )
}

const MIN_URL_DISPLAY_SEGMENT_CHARS: usize = 16;
const MAX_URL_DISPLAY_SEGMENT_CHARS: usize = 64;

fn display_url_segments(url: &str) -> Vec<SharedString> {
    let mut segments = Vec::new();
    let mut segment = String::new();
    let mut segment_chars = 0;
    let mut characters = url.chars().peekable();

    while let Some(character) = characters.next() {
        segment.push(character);
        segment_chars += 1;

        let should_split_at_boundary = segment_chars >= MIN_URL_DISPLAY_SEGMENT_CHARS
            && is_url_display_segment_boundary(character);
        let should_split_at_length = segment_chars >= MAX_URL_DISPLAY_SEGMENT_CHARS;

        if characters.peek().is_some() && (should_split_at_boundary || should_split_at_length) {
            segments.push(std::mem::take(&mut segment).into());
            segment_chars = 0;
        }
    }

    if !segment.is_empty() {
        segments.push(segment.into());
    }

    segments
}

fn is_url_display_segment_boundary(character: char) -> bool {
    matches!(character, '/' | '?' | '&' | '#')
}

pub(crate) struct ElicitationCard<'a> {
    entry_ix: usize,
    elicitation: &'a Elicitation,
    form_state: Option<&'a ElicitationFormState>,
    handlers: ElicitationCardHandlers,
}

impl<'a> ElicitationCard<'a> {
    pub(crate) fn new(
        entry_ix: usize,
        elicitation: &'a Elicitation,
        form_state: Option<&'a ElicitationFormState>,
        handlers: ElicitationCardHandlers,
    ) -> Self {
        Self {
            entry_ix,
            elicitation,
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
        let tool_name_font_size = rems_from_px(13.);
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

        let body = v_flex()
            .gap_2()
            .p_3()
            .child(Label::new(self.elicitation.request.message.clone()).size(LabelSize::Small));
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
                                Label::new("Input Requested")
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
            .when(is_pending, |this| this.child(self.render_actions(cx)))
    }

    fn render_form(&self, mode: &acp::ElicitationFormMode, cx: &App) -> AnyElement {
        let Some(state) = self.form_state else {
            return Empty.into_any_element();
        };

        v_flex()
            .gap_2()
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

        if let ElicitationFieldState::Boolean(value) = field {
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

            return v_flex()
                .gap_1()
                .child(
                    h_flex()
                        .id(row_id)
                        .w_full()
                        .items_start()
                        .gap_1()
                        .cursor_pointer()
                        .on_click(move |_, _window, cx| {
                            on_boolean_change(
                                elicitation_id.clone(),
                                field_name.clone(),
                                next_value,
                                cx,
                            );
                        })
                        .child(div().child(Checkbox::new(checkbox_id, checkbox_state)))
                        .child(
                            v_flex()
                                .gap_0p5()
                                .child(Label::new(label).size(LabelSize::Small).color(label_color))
                                .when_some(description, |this, description| {
                                    this.child(
                                        Label::new(description)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                }),
                        ),
                )
                .when_some(error.cloned(), |this, error| {
                    this.child(Label::new(error).size(LabelSize::Small).color(Color::Error))
                })
                .into_any_element();
        }

        let label = if error.is_some() {
            Label::new(label).size(LabelSize::Small).color(Color::Error)
        } else {
            Label::new(label).size(LabelSize::Small)
        };

        v_flex()
            .gap_1()
            .child(label)
            .when_some(description, |this, description| {
                this.child(
                    Label::new(description)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
            })
            .child(match field {
                ElicitationFieldState::Text(editor) => div()
                    .rounded_sm()
                    .border_1()
                    .border_color(field_border_color)
                    .bg(editor_background)
                    .px_1()
                    .py_0p5()
                    .text_xs()
                    .child(editor.clone().into_any_element())
                    .into_any_element(),
                ElicitationFieldState::Boolean(_) => Empty.into_any_element(),
                ElicitationFieldState::SingleSelect { value } => {
                    let options = match property {
                        acp::ElicitationPropertySchema::String(schema) => {
                            single_select_options(schema)
                        }
                        _ => Vec::new(),
                    };
                    self.render_single_select(
                        field_name,
                        value.as_ref(),
                        options,
                        error.is_some(),
                        cx,
                    )
                }
                ElicitationFieldState::MultiSelect(selected) => {
                    let options = match property {
                        acp::ElicitationPropertySchema::Array(schema) => {
                            multi_select_options(schema)
                        }
                        _ => Vec::new(),
                    };
                    v_flex()
                        .gap_1()
                        .children(options.into_iter().map(|option| {
                            let is_selected = selected.contains(&option.value);
                            let checkbox_state = if is_selected {
                                ToggleState::Selected
                            } else {
                                ToggleState::Unselected
                            };
                            let row_background = Self::option_row_background(is_selected, cx);
                            let hover_background =
                                Self::option_row_hover_background(is_selected, cx);
                            let on_multi_select_change =
                                self.handlers.on_multi_select_change.clone();
                            let elicitation_id = self.elicitation.id.clone();
                            let field_name = field_name.to_string();
                            let value = option.value.clone();
                            let checkbox_id = format!(
                                "elicitation-multi-{}-{field_name}-{}",
                                self.entry_ix, option.value
                            );
                            h_flex()
                                .id(SharedString::from(format!(
                                    "elicitation-multi-option-{}-{field_name}-{}",
                                    self.entry_ix, option.value
                                )))
                                .w_full()
                                .min_h(rems_from_px(28.))
                                .items_start()
                                .gap_1p5()
                                .rounded_sm()
                                .border_1()
                                .border_color(field_border_color.opacity(0.5))
                                .bg(row_background)
                                .px_2()
                                .py_1()
                                .hover(move |this| this.bg(hover_background).cursor_pointer())
                                .on_click(move |_, _window, cx| {
                                    on_multi_select_change(
                                        elicitation_id.clone(),
                                        field_name.clone(),
                                        value.clone(),
                                        !is_selected,
                                        cx,
                                    );
                                })
                                .child(div().child(Checkbox::new(checkbox_id, checkbox_state)))
                                .child(Self::render_option_content(option))
                        }))
                        .into_any_element()
                }
            })
            .when_some(error.cloned(), |this, error| {
                this.child(Label::new(error).size(LabelSize::Small).color(Color::Error))
            })
            .into_any_element()
    }

    fn render_single_select(
        &self,
        field_name: &str,
        selected_value: Option<&String>,
        options: Vec<ElicitationOption>,
        has_error: bool,
        cx: &App,
    ) -> AnyElement {
        let entry_ix = self.entry_ix;
        let border_color = if has_error {
            Color::Error.color(cx)
        } else {
            cx.theme().colors().border.opacity(0.8)
        };
        let elicitation_id = self.elicitation.id.clone();
        let field_name = field_name.to_string();
        let on_single_select_change = self.handlers.on_single_select_change.clone();

        v_flex()
            .gap_1()
            .children(options.into_iter().map(move |option| {
                let option_value = option.value.clone();
                let option_id =
                    format!("elicitation-select-option-{entry_ix}-{field_name}-{option_value}");
                let is_selected =
                    selected_value.is_some_and(|selected_value| selected_value == &option.value);
                let row_background = Self::option_row_background(is_selected, cx);
                let hover_background = Self::option_row_hover_background(is_selected, cx);
                let control_background = Self::option_control_background(cx);
                let elicitation_id = elicitation_id.clone();
                let field_name = field_name.clone();
                let on_single_select_change = on_single_select_change.clone();

                h_flex()
                    .id(option_id)
                    .w_full()
                    .min_h(rems_from_px(28.))
                    .items_start()
                    .gap_1p5()
                    .rounded_sm()
                    .border_1()
                    .border_color(border_color.opacity(0.5))
                    .bg(row_background)
                    .px_2()
                    .py_1()
                    .hover(move |this| this.bg(hover_background).cursor_pointer())
                    .on_click(move |_, _window, cx| {
                        on_single_select_change(
                            elicitation_id.clone(),
                            field_name.clone(),
                            option_value.clone(),
                            cx,
                        );
                    })
                    .child(
                        div()
                            .size(Checkbox::container_size())
                            .flex_none()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(Self::render_radio_indicator(
                                is_selected,
                                border_color,
                                control_background,
                            )),
                    )
                    .child(Self::render_option_content(option))
            }))
            .into_any_element()
    }

    fn render_option_content(option: ElicitationOption) -> Div {
        v_flex()
            .min_w_0()
            .flex_1()
            .gap_0p5()
            .child(Label::new(option.label).size(LabelSize::Small).truncate())
            .when_some(option.description, |this, description| {
                this.child(
                    Label::new(description)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
            })
    }

    fn option_row_background(is_selected: bool, cx: &App) -> Hsla {
        let editor_background = cx.theme().colors().editor_background;
        if is_selected {
            editor_background.blend(Color::Accent.color(cx).opacity(0.08))
        } else {
            editor_background
        }
    }

    fn option_row_hover_background(is_selected: bool, cx: &App) -> Hsla {
        let editor_background = cx.theme().colors().editor_background;
        if is_selected {
            editor_background.blend(Color::Accent.color(cx).opacity(0.1))
        } else {
            cx.theme()
                .colors()
                .element_background
                .blend(cx.theme().colors().editor_foreground.opacity(0.025))
        }
    }

    fn option_control_background(cx: &App) -> Hsla {
        cx.theme().colors().editor_background
    }

    fn render_radio_indicator(is_selected: bool, border_color: Hsla, background: Hsla) -> Div {
        div()
            .size_3()
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .border_1()
            .border_color(border_color)
            .bg(background)
            .when(is_selected, |this| {
                this.child(Indicator::dot().color(Color::Accent))
            })
    }

    fn render_url_elicitation(&self, mode: &acp::ElicitationUrlMode) -> AnyElement {
        v_flex()
            .gap_2()
            .child(Self::render_url_summary(&mode.url))
            .into_any_element()
    }

    fn render_url_summary(url: &str) -> AnyElement {
        h_flex()
            .gap_1()
            .w_full()
            .min_w_0()
            .items_start()
            .child(
                div().h(rems_from_px(16.)).flex().items_center().child(
                    Icon::new(IconName::Link)
                        .size(IconSize::XSmall)
                        .color(Color::Muted),
                ),
            )
            .child(h_flex().min_w_0().flex_1().flex_wrap().children(
                display_url_segments(url).into_iter().map(|segment| {
                    Label::new(segment)
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                }),
            ))
            .into_any_element()
    }

    fn render_actions(&self, cx: &App) -> AnyElement {
        let open_url = match &self.elicitation.request.mode {
            acp::ElicitationMode::Url(mode) => Some(mode.url.clone()),
            _ => None,
        };
        let (accept_label, accept_icon, accept_icon_color) = if open_url.is_some() {
            ("Open", IconName::ArrowUpRight, Color::Muted)
        } else {
            ("Submit", IconName::Check, Color::Success)
        };
        let border_color = cx.theme().colors().border.opacity(0.8);
        let on_submit = self.handlers.on_submit.clone();
        let on_open_url = self.handlers.on_open_url.clone();
        let on_decline = self.handlers.on_decline.clone();
        let on_cancel = self.handlers.on_cancel.clone();
        let submit_id = self.elicitation.id.clone();
        let decline_id = self.elicitation.id.clone();
        let cancel_id = self.elicitation.id.clone();

        h_flex()
            .w_full()
            .p_1()
            .gap_1()
            .justify_end()
            .border_t_1()
            .border_color(border_color)
            .child(
                Button::new(("elicitation-accept", self.entry_ix), accept_label)
                    .start_icon(
                        Icon::new(accept_icon)
                            .size(IconSize::XSmall)
                            .color(accept_icon_color),
                    )
                    .label_size(LabelSize::Small)
                    .on_click(move |_, window, cx| {
                        if let Some(url) = &open_url {
                            on_open_url(submit_id.clone(), url.clone(), window, cx);
                        } else {
                            on_submit(submit_id.clone(), window, cx);
                        }
                    }),
            )
            .child(
                Button::new(("elicitation-decline", self.entry_ix), "Decline")
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
                    .label_size(LabelSize::Small)
                    .on_click(move |_, window, cx| {
                        on_cancel(cancel_id.clone(), window, cx);
                    }),
            )
            .into_any_element()
    }
}
