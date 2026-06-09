use acp_thread::{Elicitation, ElicitationEntryId, ElicitationStatus};
use agent_client_protocol::schema as acp;
use collections::{HashMap, HashSet};
use component::{Component, ComponentScope, example_group_with_title, single_example};
use editor::Editor;
use futures::channel::oneshot;
use gpui::{AnyElement, App, Div, Empty, Entity, Hsla, Rems, SharedString, Window, div};
use std::collections::BTreeMap;
use std::rc::Rc;
use ui::{
    Button, Checkbox, Color, ContextMenu, Icon, IconName, IconPosition, IconSize, Label, LabelSize,
    PopoverMenu, PopoverMenuHandle, ToggleState, prelude::*,
};

#[derive(Clone)]
struct ElicitationOption {
    value: String,
    label: SharedString,
}

enum ElicitationFieldState {
    Text(Entity<Editor>),
    Boolean(bool),
    SingleSelect { value: Option<String> },
    MultiSelect(HashSet<String>),
}

pub(crate) struct ElicitationFormState {
    fields: HashMap<String, ElicitationFieldState>,
    error: Option<SharedString>,
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
            error: None,
        }
    }

    pub(crate) fn collect(
        &self,
        schema: &acp::ElicitationSchema,
        cx: &App,
    ) -> Result<BTreeMap<String, acp::ElicitationContentValue>, SharedString> {
        let required = schema.required.as_deref().unwrap_or_default();
        let mut content = BTreeMap::new();

        for (name, property) in &schema.properties {
            let is_required = required.iter().any(|required| required == name);
            let Some(field) = self.fields.get(name) else {
                continue;
            };

            match (property, field) {
                (
                    acp::ElicitationPropertySchema::String(schema),
                    ElicitationFieldState::Text(editor),
                ) => {
                    let value = editor.read(cx).text(cx).to_string();
                    if value.is_empty() {
                        if is_required {
                            return Err(
                                format!("{} is required", property_title(name, property)).into()
                            );
                        }
                        continue;
                    }

                    validate_string_value(property_title(name, property), schema, &value)?;
                    content.insert(name.clone(), value.into());
                }
                (
                    acp::ElicitationPropertySchema::String(schema),
                    ElicitationFieldState::SingleSelect { value },
                ) => {
                    if let Some(value) = value {
                        validate_single_select_value(
                            property_title(name, property),
                            schema,
                            value,
                        )?;
                        validate_string_value(property_title(name, property), schema, value)?;
                        content.insert(name.clone(), value.clone().into());
                    } else if is_required {
                        return Err(
                            format!("{} is required", property_title(name, property)).into()
                        );
                    }
                }
                (
                    acp::ElicitationPropertySchema::Number(schema),
                    ElicitationFieldState::Text(editor),
                ) => {
                    let value = editor.read(cx).text(cx).trim().to_string();
                    if value.is_empty() {
                        if is_required {
                            return Err(
                                format!("{} is required", property_title(name, property)).into()
                            );
                        }
                        continue;
                    }
                    let parsed =
                        validate_number_value(property_title(name, property), schema, &value)?;
                    content.insert(name.clone(), parsed.into());
                }
                (
                    acp::ElicitationPropertySchema::Integer(schema),
                    ElicitationFieldState::Text(editor),
                ) => {
                    let value = editor.read(cx).text(cx).trim().to_string();
                    if value.is_empty() {
                        if is_required {
                            return Err(
                                format!("{} is required", property_title(name, property)).into()
                            );
                        }
                        continue;
                    }
                    let parsed = value.parse::<i64>().map_err(|_| {
                        SharedString::from(format!(
                            "{} must be an integer",
                            property_title(name, property)
                        ))
                    })?;
                    if schema.minimum.is_some_and(|minimum| parsed < minimum) {
                        return Err(format!(
                            "{} must be at least {}",
                            property_title(name, property),
                            schema.minimum.unwrap_or_default()
                        )
                        .into());
                    }
                    if schema.maximum.is_some_and(|maximum| parsed > maximum) {
                        return Err(format!(
                            "{} must be at most {}",
                            property_title(name, property),
                            schema.maximum.unwrap_or_default()
                        )
                        .into());
                    }
                    content.insert(name.clone(), parsed.into());
                }
                (
                    acp::ElicitationPropertySchema::Boolean(schema),
                    ElicitationFieldState::Boolean(value),
                ) => {
                    if is_required || *value || schema.default.is_some() {
                        content.insert(name.clone(), (*value).into());
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
                        continue;
                    }
                    if schema
                        .min_items
                        .is_some_and(|min_items| values.len() < min_items as usize)
                    {
                        return Err(format!(
                            "{} needs more selections",
                            property_title(name, property)
                        )
                        .into());
                    }
                    if schema
                        .max_items
                        .is_some_and(|max_items| values.len() > max_items as usize)
                    {
                        return Err(format!(
                            "{} has too many selections",
                            property_title(name, property)
                        )
                        .into());
                    }
                    content.insert(name.clone(), values.into());
                }
                _ => {}
            }
        }

        Ok(content)
    }

    pub(crate) fn set_error(&mut self, error: impl Into<SharedString>) {
        self.error = Some(error.into());
    }

    pub(crate) fn set_boolean(&mut self, field_name: &str, value: bool) {
        if let Some(ElicitationFieldState::Boolean(field)) = self.fields.get_mut(field_name) {
            *field = value;
            self.error.take();
        }
    }

    pub(crate) fn set_single_select(&mut self, field_name: &str, value: String) {
        if let Some(ElicitationFieldState::SingleSelect { value: selected }) =
            self.fields.get_mut(field_name)
        {
            *selected = Some(value);
            self.error.take();
        }
    }

    pub(crate) fn set_multi_select(&mut self, field_name: &str, value: String, selected: bool) {
        if let Some(ElicitationFieldState::MultiSelect(values)) = self.fields.get_mut(field_name) {
            if selected {
                values.insert(value);
            } else {
                values.remove(&value);
            }
            self.error.take();
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

            assert_eq!(
                form_state
                    .collect(&schema, cx)
                    .expect_err("invalid selected value should be rejected")
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
                            render_form_preview(0, pending_status(), None, window, cx),
                        )
                        .width(px(640.)),
                        single_example(
                            "Validation Error",
                            render_form_preview(
                                1,
                                pending_status(),
                                Some("Account name is required"),
                                window,
                                cx,
                            ),
                        )
                        .width(px(640.)),
                        single_example(
                            "Submitted Form",
                            render_form_preview(2, ElicitationStatus::Accepted, None, window, cx),
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
                        single_example(
                            "URL Accepted",
                            render_url_preview(4, ElicitationStatus::Accepted, window, cx),
                        )
                        .width(px(640.)),
                        single_example(
                            "URL Completed",
                            render_url_preview(5, ElicitationStatus::Completed, window, cx),
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
                            render_form_preview(6, ElicitationStatus::Declined, None, window, cx),
                        )
                        .width(px(640.)),
                        single_example(
                            "Canceled",
                            render_form_preview(7, ElicitationStatus::Canceled, None, window, cx),
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
    error: Option<&'static str>,
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
    if let Some(error) = error
        && let Some(form_state) = &mut form_state
    {
        form_state.set_error(error);
    }

    render_preview_card(entry_ix, request, status, form_state.as_ref(), cx)
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
            "https://auth.example.com/device",
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
                PopoverMenuHandle::<ContextMenu>::default(),
                preview_style(cx),
                ElicitationCardHandlers::noop(),
            )
            .render(),
        )
        .into_any_element()
}

fn preview_style(cx: &App) -> ElicitationCardStyle {
    ElicitationCardStyle::new(
        cx.theme().colors().border.opacity(0.8),
        cx.theme()
            .colors()
            .element_background
            .blend(cx.theme().colors().editor_foreground.opacity(0.025)),
        cx.theme().colors().editor_background,
        rems_from_px(13.),
    )
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
                    acp::EnumOption::new("production", "Production"),
                    acp::EnumOption::new("staging", "Staging"),
                    acp::EnumOption::new("development", "Development"),
                ])
                .default_value("staging"),
            true,
        )
        .property(
            "scopes",
            acp::MultiSelectPropertySchema::titled(vec![
                acp::EnumOption::new("profile", "Profile"),
                acp::EnumOption::new("repository", "Repository Access"),
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
        acp::MultiSelectItems::Untitled(items) => items
            .values
            .iter()
            .map(|value| ElicitationOption {
                value: value.clone(),
                label: SharedString::from(value.clone()),
            })
            .collect(),
        acp::MultiSelectItems::Titled(items) => items
            .options
            .iter()
            .map(|option| ElicitationOption {
                value: option.value.clone(),
                label: SharedString::from(option.title.clone()),
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

#[derive(Clone, Copy)]
pub(crate) struct ElicitationCardStyle {
    border_color: Hsla,
    header_background: Hsla,
    editor_background: Hsla,
    tool_name_font_size: Rems,
}

impl ElicitationCardStyle {
    pub(crate) fn new(
        border_color: Hsla,
        header_background: Hsla,
        editor_background: Hsla,
        tool_name_font_size: Rems,
    ) -> Self {
        Self {
            border_color,
            header_background,
            editor_background,
            tool_name_font_size,
        }
    }
}

pub(crate) struct ElicitationCard<'a> {
    entry_ix: usize,
    elicitation: &'a Elicitation,
    form_state: Option<&'a ElicitationFormState>,
    dropdown_handle: PopoverMenuHandle<ContextMenu>,
    style: ElicitationCardStyle,
    handlers: ElicitationCardHandlers,
}

impl<'a> ElicitationCard<'a> {
    pub(crate) fn new(
        entry_ix: usize,
        elicitation: &'a Elicitation,
        form_state: Option<&'a ElicitationFormState>,
        dropdown_handle: PopoverMenuHandle<ContextMenu>,
        style: ElicitationCardStyle,
        handlers: ElicitationCardHandlers,
    ) -> Self {
        Self {
            entry_ix,
            elicitation,
            form_state,
            dropdown_handle,
            style,
            handlers,
        }
    }

    pub(crate) fn render(self) -> Div {
        let is_pending = matches!(&self.elicitation.status, ElicitationStatus::Pending { .. });
        let (status_label, status_icon, status_color) = match &self.elicitation.status {
            ElicitationStatus::Pending { .. } => ("Waiting for input", IconName::Info, Color::Info),
            ElicitationStatus::Accepted
                if matches!(&self.elicitation.request.mode, acp::ElicitationMode::Url(_)) =>
            {
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
            acp::ElicitationMode::Form(mode) if is_pending => body.child(self.render_form(mode)),
            acp::ElicitationMode::Url(mode) if is_pending => {
                body.child(self.render_url_elicitation(mode))
            }
            acp::ElicitationMode::Url(mode) => body.child(Self::render_url_summary(&mode.url)),
            _ => body,
        };

        v_flex()
            .mx_5()
            .my_1p5()
            .rounded_md()
            .border_1()
            .border_color(self.style.border_color)
            .overflow_hidden()
            .child(
                h_flex()
                    .h_8()
                    .p_1()
                    .w_full()
                    .justify_between()
                    .bg(self.style.header_background)
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
                                    .size(LabelSize::Custom(self.style.tool_name_font_size))
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
            .when(is_pending, |this| this.child(self.render_actions()))
    }

    fn render_form(&self, mode: &acp::ElicitationFormMode) -> AnyElement {
        let Some(state) = self.form_state else {
            return Empty.into_any_element();
        };

        v_flex()
            .gap_2()
            .children(mode.requested_schema.properties.iter().filter_map(
                |(field_name, property)| {
                    let field = state.fields.get(field_name)?;
                    Some(self.render_field(field_name, property, field))
                },
            ))
            .when_some(state.error.clone(), |this, error| {
                this.child(Label::new(error).size(LabelSize::Small).color(Color::Error))
            })
            .into_any_element()
    }

    fn render_field(
        &self,
        field_name: &str,
        property: &acp::ElicitationPropertySchema,
        field: &ElicitationFieldState,
    ) -> AnyElement {
        let label = property_title(field_name, property);
        let description = property_description(property);

        v_flex()
            .gap_1()
            .child(Label::new(label).size(LabelSize::Small))
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
                    .border_color(self.style.border_color)
                    .bg(self.style.editor_background)
                    .px_1()
                    .py_0p5()
                    .text_xs()
                    .child(editor.clone().into_any_element())
                    .into_any_element(),
                ElicitationFieldState::Boolean(value) => {
                    let checkbox_state = if *value {
                        ToggleState::Selected
                    } else {
                        ToggleState::Unselected
                    };
                    let on_boolean_change = self.handlers.on_boolean_change.clone();
                    let elicitation_id = self.elicitation.id.clone();
                    let field_name = field_name.to_string();
                    Checkbox::new(
                        format!("elicitation-bool-{}-{field_name}", self.entry_ix),
                        checkbox_state,
                    )
                    .on_click(move |state, _window, cx| {
                        let value = matches!(state, ToggleState::Selected);
                        on_boolean_change(elicitation_id.clone(), field_name.clone(), value, cx);
                    })
                    .into_any_element()
                }
                ElicitationFieldState::SingleSelect { value } => {
                    let options = match property {
                        acp::ElicitationPropertySchema::String(schema) => {
                            single_select_options(schema)
                        }
                        _ => Vec::new(),
                    };
                    self.render_select(field_name, value.clone(), options)
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
                            let on_multi_select_change =
                                self.handlers.on_multi_select_change.clone();
                            let elicitation_id = self.elicitation.id.clone();
                            let field_name = field_name.to_string();
                            let value = option.value.clone();
                            h_flex()
                                .gap_1()
                                .child(
                                    Checkbox::new(
                                        format!(
                                            "elicitation-multi-{}-{field_name}-{}",
                                            self.entry_ix, option.value
                                        ),
                                        checkbox_state,
                                    )
                                    .on_click(
                                        move |state, _window, cx| {
                                            let is_selected =
                                                matches!(state, ToggleState::Selected);
                                            on_multi_select_change(
                                                elicitation_id.clone(),
                                                field_name.clone(),
                                                value.clone(),
                                                is_selected,
                                                cx,
                                            );
                                        },
                                    ),
                                )
                                .child(Label::new(option.label).size(LabelSize::Small))
                        }))
                        .into_any_element()
                }
            })
            .into_any_element()
    }

    fn render_select(
        &self,
        field_name: &str,
        selected_value: Option<String>,
        options: Vec<ElicitationOption>,
    ) -> AnyElement {
        let current_label = selected_value
            .as_ref()
            .and_then(|value| {
                options
                    .iter()
                    .find(|option| &option.value == value)
                    .map(|option| option.label.clone())
            })
            .unwrap_or_else(|| SharedString::from("Choose"));
        let dropdown_handle = self.dropdown_handle.clone();
        let elicitation_id = self.elicitation.id.clone();
        let field_name = field_name.to_string();
        let on_single_select_change = self.handlers.on_single_select_change.clone();

        PopoverMenu::new(format!("elicitation-select-{}-{field_name}", self.entry_ix))
            .with_handle(dropdown_handle)
            .trigger(
                Button::new(
                    format!("elicitation-select-trigger-{}-{field_name}", self.entry_ix),
                    current_label,
                )
                .end_icon(
                    Icon::new(IconName::ChevronDown)
                        .size(IconSize::XSmall)
                        .color(Color::Muted),
                )
                .label_size(LabelSize::Small),
            )
            .menu(move |window, cx| {
                let options = options.clone();
                let elicitation_id = elicitation_id.clone();
                let field_name = field_name.clone();
                let selected_value = selected_value.clone();
                let on_single_select_change = on_single_select_change.clone();

                Some(ContextMenu::build(window, cx, move |mut menu, _, _| {
                    for option in options.iter() {
                        let option_value = option.value.clone();
                        let option_label = option.label.clone();
                        let is_selected = selected_value
                            .as_ref()
                            .is_some_and(|selected| selected == &option_value);
                        let elicitation_id = elicitation_id.clone();
                        let field_name = field_name.clone();
                        let on_single_select_change = on_single_select_change.clone();
                        menu = menu.toggleable_entry(
                            option_label,
                            is_selected,
                            IconPosition::End,
                            None,
                            move |_window, cx| {
                                on_single_select_change(
                                    elicitation_id.clone(),
                                    field_name.clone(),
                                    option_value.clone(),
                                    cx,
                                );
                            },
                        );
                    }
                    menu
                }))
            })
            .into_any_element()
    }

    fn render_url_elicitation(&self, mode: &acp::ElicitationUrlMode) -> AnyElement {
        let url = mode.url.clone();
        let on_open_url = self.handlers.on_open_url.clone();
        let elicitation_id = self.elicitation.id.clone();

        v_flex()
            .gap_2()
            .child(Self::render_url_summary(&url))
            .child(
                Button::new(("open-elicit-url", self.entry_ix), "Open")
                    .start_icon(
                        Icon::new(IconName::ArrowUpRight)
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .label_size(LabelSize::Small)
                    .on_click(move |_, window, cx| {
                        on_open_url(elicitation_id.clone(), url.clone(), window, cx);
                    }),
            )
            .into_any_element()
    }

    fn render_url_summary(url: &str) -> AnyElement {
        h_flex()
            .gap_1()
            .min_w_0()
            .child(
                Icon::new(IconName::Link)
                    .size(IconSize::XSmall)
                    .color(Color::Muted),
            )
            .child(
                Label::new(url.to_string())
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .truncate(),
            )
            .into_any_element()
    }

    fn render_actions(&self) -> AnyElement {
        let accept_label = match &self.elicitation.request.mode {
            acp::ElicitationMode::Url(_) => "Done",
            _ => "Submit",
        };
        let on_submit = self.handlers.on_submit.clone();
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
            .border_color(self.style.border_color)
            .child(
                Button::new(("elicitation-accept", self.entry_ix), accept_label)
                    .start_icon(
                        Icon::new(IconName::Check)
                            .size(IconSize::XSmall)
                            .color(Color::Success),
                    )
                    .label_size(LabelSize::Small)
                    .on_click(move |_, window, cx| {
                        on_submit(submit_id.clone(), window, cx);
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
