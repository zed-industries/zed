use std::rc::Rc;

use anyhow::Result;
use gpui::{App, Context, Entity, Subscription, Task, Window};
use language_model::ApiKeyState;
use ui::{Tooltip, prelude::*};
use ui_input::InputField;

/// The current credential state of a single-API-key provider, as reported by the
/// provider when constructing an [`ApiKeyEditor`].
pub enum ApiKeyStatus {
    /// No key is configured; show the input field.
    Unset,
    /// A key is configured via the UI; show a "configured" row with a reset.
    Configured,
    /// The key comes from an environment variable and can't be edited here.
    FromEnvVar(SharedString),
}

/// Maps a provider's [`ApiKeyState`] to the [`ApiKeyStatus`] the editor renders.
/// Shared so the API-key providers don't each duplicate this mapping.
pub fn api_key_status(state: &ApiKeyState) -> ApiKeyStatus {
    if state.is_from_env_var() {
        ApiKeyStatus::FromEnvVar(state.env_var_name().clone())
    } else if state.has_key() {
        ApiKeyStatus::Configured
    } else {
        ApiKeyStatus::Unset
    }
}

/// A compact, reusable control for editing a provider's single API key, intended
/// to be returned from `LanguageModelProvider::configuration_view_v2` as an
/// inline control.
///
/// It is deliberately provider-agnostic: the provider supplies closures that
/// read the current [`ApiKeyStatus`] and store/clear the key against its own
/// state, so all credential knowledge stays in the provider.
pub struct ApiKeyEditor {
    input: Entity<InputField>,
    api_key_url: SharedString,
    status: Rc<dyn Fn(&App) -> ApiKeyStatus>,
    set_key: Rc<dyn Fn(String, &mut App) -> Task<Result<()>>>,
    reset_key: Rc<dyn Fn(&mut App) -> Task<Result<()>>>,
    _subscription: Subscription,
}

impl ApiKeyEditor {
    pub fn new<S: 'static>(
        state: Entity<S>,
        api_key_url: impl Into<SharedString>,
        placeholder: &str,
        status: impl Fn(&S, &App) -> ApiKeyStatus + 'static,
        set_key: impl Fn(&Entity<S>, String, &mut App) -> Task<Result<()>> + 'static,
        reset_key: impl Fn(&Entity<S>, &mut App) -> Task<Result<()>> + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let input = cx.new(|cx| {
            InputField::new(window, cx, placeholder)
                .masked(true)
                .tab_index(0)
        });
        let subscription = cx.observe(&state, |_, _, cx| cx.notify());

        let status_state = state.clone();
        let set_state = state.clone();
        Self {
            input,
            api_key_url: api_key_url.into(),
            status: Rc::new(move |cx| status(status_state.read(cx), cx)),
            set_key: Rc::new(move |key, cx| set_key(&set_state, key, cx)),
            reset_key: Rc::new(move |cx| reset_key(&state, cx)),
            _subscription: subscription,
        }
    }

    fn save(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let key = self.input.read(cx).text(cx).trim().to_string();
        if key.is_empty() {
            return;
        }
        self.input
            .update(cx, |input, cx| input.set_text("", window, cx));
        (self.set_key.clone())(key, cx).detach_and_log_err(cx);
    }

    fn reset(&mut self, cx: &mut Context<Self>) {
        (self.reset_key.clone())(cx).detach_and_log_err(cx);
    }

    fn render_where_to_find_key(&self) -> impl IntoElement {
        let url = self.api_key_url.clone();
        let click_url = url.to_string();
        h_flex()
            .id("where-to-find-key")
            .gap_0p5()
            .cursor_pointer()
            .child(
                Icon::new(IconName::Info)
                    .size(IconSize::XSmall)
                    .color(Color::Muted),
            )
            .child(
                Label::new("Where to find key")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .tooltip(Tooltip::text(format!("Create an API key at {url}")))
            .on_click(move |_, _window, cx| cx.open_url(&click_url))
    }
}

impl Render for ApiKeyEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match (self.status)(cx) {
            ApiKeyStatus::FromEnvVar(env_var_name) => Label::new(format!("Set via {env_var_name}"))
                .size(LabelSize::Small)
                .color(Color::Muted)
                .into_any_element(),
            ApiKeyStatus::Configured => h_flex()
                .gap_2()
                .items_center()
                .child(
                    Icon::new(IconName::Check)
                        .size(IconSize::Small)
                        .color(Color::Success),
                )
                .child(
                    Label::new("Configured")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .child(
                    Button::new("reset-api-key", "Reset")
                        .style(ButtonStyle::Outlined)
                        .label_size(LabelSize::Small)
                        .tab_index(0isize)
                        .on_click(cx.listener(|this, _, _window, cx| this.reset(cx))),
                )
                .into_any_element(),
            ApiKeyStatus::Unset => v_flex()
                .w_full()
                .gap_1()
                .child(self.render_where_to_find_key())
                .child(
                    div()
                        .w_full()
                        .on_action(cx.listener(Self::save))
                        .child(self.input.clone()),
                )
                .into_any_element(),
        }
    }
}
