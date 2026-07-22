use crate::multibuffer_hint::MultibufferHint;
use agent_ui::AgentPanel;
use client::{Client, UserStore, zed_urls};
use cloud_api_types::Plan;
use db::kvp::{Dismissable, KeyValueStore};
use fs::Fs;
use gpui::{
    Action, AnyElement, App, AppContext, AsyncWindowContext, Context, Entity, EventEmitter,
    FocusHandle, Focusable, Global, IntoElement, KeyContext, Render, ScrollHandle, SharedString,
    Subscription, Task, WeakEntity, Window, actions,
};
use notifications::status_toast::StatusToast;
use project::agent_server_store::AllAgentServersSettings;
use schemars::JsonSchema;
use serde::Deserialize;
use settings::{SettingsStore, VsCodeSettingsSource};
use std::sync::Arc;
use ui::{
    Divider, KeyBinding, ParentElement as _, StatefulInteractiveElement, Vector, VectorName,
    WithScrollbar as _, prelude::*, rems_from_px,
};

pub use workspace::welcome::ShowWelcome;
use workspace::welcome::WelcomePage;
use workspace::{
    AppState, Workspace, WorkspaceId,
    dock::{DockPosition, Panel},
    item::{Item, ItemEvent},
    notifications::NotifyResultExt as _,
    open_new, register_serializable_item, with_active_or_new_workspace,
};
use zed_actions::OpenOnboarding;

mod base_keymap_picker;
mod basics_page;
pub mod multibuffer_hint;
mod theme_preview;

/// Imports settings from Visual Studio Code.
#[derive(Copy, Clone, Debug, Default, PartialEq, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct ImportVsCodeSettings {
    #[serde(default)]
    pub skip_prompt: bool,
}

/// Imports settings from Cursor editor.
#[derive(Copy, Clone, Debug, Default, PartialEq, Deserialize, JsonSchema, Action)]
#[action(namespace = zed)]
#[serde(deny_unknown_fields)]
pub struct ImportCursorSettings {
    #[serde(default)]
    pub skip_prompt: bool,
}

pub const FIRST_OPEN: &str = "first_open";

actions!(
    onboarding,
    [
        /// Finish the onboarding process.
        Finish,
        /// Sign in while in the onboarding flow.
        SignIn,
        /// Open the user account in zed.dev while in the onboarding flow.
        OpenAccount,
        /// Resets the welcome screen hints to their initial state.
        ResetHints
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _cx| {
        workspace
            .register_action(|_workspace, _: &ResetHints, _, cx| MultibufferHint::set_count(0, cx));
    })
    .detach();

    cx.on_action(|_: &OpenOnboarding, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            workspace
                .with_local_workspace(window, cx, |workspace, window, cx| {
                    let existing = workspace
                        .active_pane()
                        .read(cx)
                        .items()
                        .find_map(|item| item.downcast::<Onboarding>());

                    if let Some(existing) = existing {
                        workspace.activate_item(&existing, true, true, window, cx);
                    } else {
                        let settings_page = Onboarding::new(workspace, cx);
                        workspace.add_item_to_active_pane(
                            Box::new(settings_page),
                            None,
                            true,
                            window,
                            cx,
                        )
                    }
                })
                .detach();
        });
    });

    cx.on_action(|_: &ShowWelcome, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            workspace
                .with_local_workspace(window, cx, |workspace, window, cx| {
                    let existing = workspace
                        .active_pane()
                        .read(cx)
                        .items()
                        .find_map(|item| item.downcast::<WelcomePage>());

                    if let Some(existing) = existing {
                        workspace.activate_item(&existing, true, true, window, cx);
                    } else {
                        let settings_page = cx
                            .new(|cx| WelcomePage::new(workspace.weak_handle(), false, window, cx));
                        workspace.add_item_to_active_pane(
                            Box::new(settings_page),
                            None,
                            true,
                            window,
                            cx,
                        )
                    }
                })
                .detach();
        });
    });

    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|_workspace, action: &ImportVsCodeSettings, window, cx| {
            let fs = <dyn Fs>::global(cx);
            let action = *action;

            let workspace = cx.weak_entity();

            window
                .spawn(cx, async move |cx: &mut AsyncWindowContext| {
                    handle_import_vscode_settings(
                        workspace,
                        VsCodeSettingsSource::VsCode,
                        action.skip_prompt,
                        fs,
                        cx,
                    )
                    .await
                })
                .detach();
        });

        workspace.register_action(|_workspace, action: &ImportCursorSettings, window, cx| {
            let fs = <dyn Fs>::global(cx);
            let action = *action;

            let workspace = cx.weak_entity();

            window
                .spawn(cx, async move |cx: &mut AsyncWindowContext| {
                    handle_import_vscode_settings(
                        workspace,
                        VsCodeSettingsSource::Cursor,
                        action.skip_prompt,
                        fs,
                        cx,
                    )
                    .await
                })
                .detach();
        });
    })
    .detach();

    base_keymap_picker::init(cx);

    register_serializable_item::<Onboarding>(cx);
    register_serializable_item::<WelcomePage>(cx);
}

pub fn show_onboarding_view(app_state: Arc<AppState>, cx: &mut App) -> Task<anyhow::Result<()>> {
    telemetry::event!("Onboarding Page Opened");
    open_new(
        Default::default(),
        app_state,
        cx,
        |workspace, window, cx| {
            {
                workspace.toggle_dock(DockPosition::Left, window, cx);
                let onboarding_page = Onboarding::new(workspace, cx);
                workspace.add_item_to_center(Box::new(onboarding_page.clone()), window, cx);

                window.focus(&onboarding_page.focus_handle(cx), cx);

                cx.notify();
            };
            let kvp = KeyValueStore::global(cx);
            db::write_and_log(cx, move || async move {
                kvp.write_kvp(FIRST_OPEN.to_string(), "false".to_string())
                    .await
            });
        },
    )
}

struct Onboarding {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    user_store: Entity<UserStore>,
    scroll_handle: ScrollHandle,
    _settings_subscription: Subscription,
    // Registered lazily on first render, since `new` has no `Window`. Fires when
    // focus leaves onboarding for another surface (a dock panel, another tab,
    // etc.), which we treat as leaving onboarding.
    _focus_out_subscription: Option<Subscription>,
}

impl Onboarding {
    fn new(workspace: &Workspace, cx: &mut App) -> Entity<Self> {
        let font_family_cache = theme::FontFamilyCache::global(cx);

        let installed_agents = cx
            .global::<SettingsStore>()
            .get::<AllAgentServersSettings>(None)
            .clone();
        let client = Client::global(cx);
        let status = *client.status().borrow();
        let plan = workspace.user_store().read(cx).plan();
        let zed_agent_state = if status.is_signed_out()
            || matches!(
                status,
                client::Status::AuthenticationError | client::Status::ConnectionError
            ) {
            "signed_out"
        } else if status.is_signing_in() {
            "signing_in"
        } else {
            match plan {
                Some(Plan::ZedPro) => "pro",
                Some(Plan::ZedProTrial) => "trial",
                Some(Plan::ZedBusiness) => "business",
                Some(Plan::ZedVip) => "vip",
                Some(Plan::ZedStudent) => "student",
                Some(Plan::ZedFree) | None => "free",
            }
        };
        let agents_installed = basics_page::FEATURED_AGENT_IDS
            .iter()
            .filter(|id| installed_agents.contains_key(**id))
            .copied()
            .collect::<Vec<_>>();
        telemetry::event!(
            "Welcome Agent Setup Viewed",
            zed_agent = zed_agent_state,
            agents_installed = agents_installed,
        );

        cx.new(|cx| {
            cx.spawn(async move |this, cx| {
                font_family_cache.prefetch(cx).await;
                this.update(cx, |_, cx| {
                    cx.notify();
                })
            })
            .detach();

            Self {
                workspace: workspace.weak_handle(),
                focus_handle: cx.focus_handle(),
                scroll_handle: ScrollHandle::new(),
                user_store: workspace.user_store().clone(),
                _settings_subscription: cx
                    .observe_global::<SettingsStore>(move |_, cx| cx.notify()),
                _focus_out_subscription: None,
            }
        })
    }

    fn on_finish(_: &Finish, _: &mut Window, cx: &mut App) {
        telemetry::event!("Finish Setup");
        go_to_welcome_page(cx);
        on_leave_onboarding(LeaveTrigger::Finish, cx);
    }

    fn handle_sign_in(&mut self, _: &SignIn, window: &mut Window, cx: &mut Context<Self>) {
        let client = Client::global(cx);
        let workspace = self.workspace.clone();

        window
            .spawn(cx, async move |mut cx| {
                client
                    .sign_in_with_optional_connect(true, &cx)
                    .await
                    .notify_workspace_async_err(workspace, &mut cx);
            })
            .detach();
    }

    fn handle_open_account(_: &OpenAccount, _: &mut Window, cx: &mut App) {
        cx.open_url(&zed_urls::account_url(cx))
    }

    fn render_page(&mut self, cx: &mut Context<Self>) -> AnyElement {
        crate::basics_page::render_basics_page(&self.user_store, cx).into_any_element()
    }

    fn register_focus_out_subscription(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self._focus_out_subscription.is_some() {
            return;
        }
        self._focus_out_subscription =
            Some(cx.on_focus_out(&self.focus_handle, window, |_, _, _, cx| {
                // Focus moved away from onboarding to another surface (a dock
                // panel, another tab, etc.). Treat that as leaving onboarding.
                // Opening a modal from within onboarding (e.g. the base keymap
                // picker) also fires this, but that case is filtered out via
                // `has_active_modal` in `on_leave_onboarding`.
                on_leave_onboarding(LeaveTrigger::FocusOut, cx);
            }));
    }
}

impl Render for Onboarding {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.register_focus_out_subscription(window, cx);
        div()
            .image_cache(gpui::retain_all("onboarding-page"))
            .key_context({
                let mut ctx = KeyContext::new_with_defaults();
                ctx.add("Onboarding");
                ctx.add("menu");
                ctx
            })
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .on_action(Self::on_finish)
            .on_action(cx.listener(Self::handle_sign_in))
            .on_action(Self::handle_open_account)
            .on_action(cx.listener(|_, _: &menu::SelectNext, window, cx| {
                window.focus_next(cx);
                cx.notify();
            }))
            .on_action(cx.listener(|_, _: &menu::SelectPrevious, window, cx| {
                window.focus_prev(cx);
                cx.notify();
            }))
            .vertical_scrollbar_for(&self.scroll_handle, window, cx)
            .child(
                div()
                    .id("page-content")
                    .size_full()
                    .overflow_y_scroll()
                    .child(
                        v_flex()
                            .min_w_0()
                            .max_w(rems_from_px(780.))
                            .w_full()
                            .mx_auto()
                            .p_12()
                            .gap_6()
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_4()
                                    .justify_between()
                                    .child(
                                        h_flex()
                                            .gap_4()
                                            .child(Vector::square(VectorName::ZedLogo, rems(2.5)))
                                            .child(
                                                v_flex()
                                                    .child(
                                                        Headline::new("Welcome to Zed")
                                                            .size(HeadlineSize::Small),
                                                    )
                                                    .child(
                                                        Label::new("The editor for what's next")
                                                            .color(Color::Muted)
                                                            .size(LabelSize::Small)
                                                            .italic(),
                                                    ),
                                            ),
                                    )
                                    .child({
                                        Button::new("finish_setup", "Finish Setup")
                                            .style(ButtonStyle::Filled)
                                            .size(ButtonSize::Medium)
                                            .width(rems_from_px(200.))
                                            .key_binding(KeyBinding::for_action_in(
                                                &Finish,
                                                &self.focus_handle,
                                                cx,
                                            ))
                                            .on_click(|_, window, cx| {
                                                window.dispatch_action(Finish.boxed_clone(), cx);
                                            })
                                    }),
                            )
                            .child(Divider::horizontal().color(ui::DividerColor::BorderVariant))
                            .child(self.render_page(cx)),
                    )
                    .track_scroll(&self.scroll_handle),
            )
    }
}

impl EventEmitter<ItemEvent> for Onboarding {}

impl Focusable for Onboarding {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for Onboarding {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Onboarding".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Onboarding Page Opened")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>> {
        Task::ready(Some(cx.new(|cx| Onboarding {
            workspace: self.workspace.clone(),
            user_store: self.user_store.clone(),
            scroll_handle: ScrollHandle::new(),
            focus_handle: cx.focus_handle(),
            _settings_subscription: cx.observe_global::<SettingsStore>(move |_, cx| cx.notify()),
            _focus_out_subscription: None,
        })))
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }

    fn deactivated(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        // The user has navigated away from onboarding (e.g. opened a file or
        // switched tabs) without explicitly finishing setup. Treat this as an
        // exit from onboarding and reveal the agent panel if they configured an
        // agent, so it's easy to find.
        on_leave_onboarding(LeaveTrigger::Deactivated, cx);
    }
}

fn go_to_welcome_page(cx: &mut App) {
    with_active_or_new_workspace(cx, |workspace, window, cx| {
        let Some((onboarding_id, onboarding_idx)) = workspace
            .active_pane()
            .read(cx)
            .items()
            .enumerate()
            .find_map(|(idx, item)| {
                let _ = item.downcast::<Onboarding>()?;
                Some((item.item_id(), idx))
            })
        else {
            return;
        };

        workspace.active_pane().update(cx, |pane, cx| {
            // Get the index here to get around the borrow checker
            let idx = pane.items().enumerate().find_map(|(idx, item)| {
                let _ = item.downcast::<WelcomePage>()?;
                Some(idx)
            });

            if let Some(idx) = idx {
                pane.activate_item(idx, true, true, window, cx);
            } else {
                let item = Box::new(
                    cx.new(|cx| WelcomePage::new(workspace.weak_handle(), false, window, cx)),
                );
                pane.add_item(item, true, true, Some(onboarding_idx), window, cx);
            }

            pane.remove_item(onboarding_id, false, false, window, cx);
        });
    });
}

/// Persists whether the user has already been exposed to the post-onboarding
/// agent-panel experiment, so each user is enrolled exactly once (across
/// sessions) regardless of how many times they leave onboarding.
struct OnboardingAgentExperimentExposed;

impl Dismissable for OnboardingAgentExperimentExposed {
    const KEY: &'static str = "onboarding_agent_experiment_exposed";
}

/// In-memory companion to the persisted exposure flag. The persisted flag is
/// written asynchronously, so several leave-signals firing in the same tick
/// (e.g. on "Finish": deactivation + focus-out + the explicit call) would all
/// read it as not-yet-set and each enroll/reveal. This flag is set synchronously
/// the moment we enroll, so later signals in the same tick see it and skip.
#[derive(Default)]
struct OnboardingAgentExperimentExposedThisSession(bool);

impl Global for OnboardingAgentExperimentExposedThisSession {}

impl OnboardingAgentExperimentExposedThisSession {
    fn is_set(cx: &App) -> bool {
        cx.try_global::<Self>().is_some_and(|this| this.0)
    }

    fn set(cx: &mut App) {
        cx.update_default_global::<Self, _>(|this, _| this.0 = true);
    }
}

/// Whether this user has already been enrolled in the experiment, this session
/// or any previous one.
fn already_exposed_to_experiment(cx: &App) -> bool {
    OnboardingAgentExperimentExposedThisSession::is_set(cx)
        || OnboardingAgentExperimentExposed::dismissed(cx)
}

fn mark_exposed_to_experiment(cx: &mut App) {
    // Set the synchronous flag first so sibling exits queued in this same tick
    // observe it before the async persisted write lands.
    OnboardingAgentExperimentExposedThisSession::set(cx);
    OnboardingAgentExperimentExposed::set_dismissed(true, cx);
}

/// Salt + ramp for the post-onboarding agent-panel experiment. Bump the salt to
/// start a fresh experiment (re-buckets everyone); change the percentage to ramp
/// the treatment arm (0 disables it, 100 gives everyone the treatment).
const EXPERIMENT_SALT: &str = "agent-panel-autoopen-v1";
const TREATMENT_PERCENTAGE: u64 = 50;

/// Which arm of the experiment a user is in. Assignment is computed locally from
/// a stable installation id so it works for logged-out users too (many configure
/// external CLI agents without signing in), and is emitted on the exposure event
/// so downstream analytics can segment by it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ExperimentArm {
    Control,
    Treatment,
}

impl ExperimentArm {
    fn as_str(self) -> &'static str {
        match self {
            ExperimentArm::Control => "control",
            ExperimentArm::Treatment => "treatment",
        }
    }
}

/// The way the user left onboarding when the experiment fired. Emitted on the
/// exposure event so we can later learn which trigger drives engagement.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum LeaveTrigger {
    Finish,
    Deactivated,
    FocusOut,
}

impl LeaveTrigger {
    fn as_str(self) -> &'static str {
        match self {
            LeaveTrigger::Finish => "finish",
            LeaveTrigger::Deactivated => "deactivated",
            LeaveTrigger::FocusOut => "focus_out",
        }
    }
}

/// Deterministic 64-bit FNV-1a hash. Used for experiment bucketing: stable
/// across builds and platforms (unlike `DefaultHasher`), so a user keeps the
/// same arm across sessions and releases.
fn stable_hash(salt: &str, id: &str) -> u64 {
    const OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;
    let mut hash = OFFSET_BASIS;
    for byte in salt.bytes().chain(std::iter::once(b':')).chain(id.bytes()) {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

fn experiment_arm(installation_id: &str) -> ExperimentArm {
    if stable_hash(EXPERIMENT_SALT, installation_id) % 100 < TREATMENT_PERCENTAGE {
        ExperimentArm::Treatment
    } else {
        ExperimentArm::Control
    }
}

/// The featured external agents the user has installed, for the exposure event.
fn configured_featured_agents(cx: &App) -> Vec<&'static str> {
    let installed_agents = cx
        .global::<SettingsStore>()
        .get::<AllAgentServersSettings>(None);
    basics_page::FEATURED_AGENT_IDS
        .iter()
        .filter(|id| installed_agents.contains_key(**id))
        .copied()
        .collect()
}

fn zed_agent_signed_in(cx: &App) -> bool {
    let status = *Client::global(cx).status().borrow();
    !(status.is_signed_out()
        || matches!(
            status,
            client::Status::AuthenticationError | client::Status::ConnectionError
        ))
}

/// Returns whether the user has at least one agent configured: either signed in
/// to the Zed Agent, or one of the featured external agents installed.
fn any_agent_configured(cx: &App) -> bool {
    !configured_featured_agents(cx).is_empty() || zed_agent_signed_in(cx)
}

/// How the agent panel is revealed after onboarding. Injectable as a global so
/// tests can observe reveals: the production implementation opens the concrete
/// `AgentPanel` (whose constructor is private to `agent_ui`), while tests can
/// substitute a recorder. Returns whether the panel was actually revealed, so
/// the one-shot is only "spent" on a real reveal (e.g. not when the panel is
/// disabled).
#[derive(Clone)]
struct AgentPanelRevealer(
    Arc<dyn Fn(&mut Workspace, &mut Window, &mut Context<Workspace>) -> bool + Send + Sync>,
);

impl Global for AgentPanelRevealer {}

impl AgentPanelRevealer {
    fn get(cx: &App) -> Self {
        cx.try_global::<Self>().cloned().unwrap_or_else(|| {
            AgentPanelRevealer(Arc::new(|workspace, window, cx| {
                let panel_enabled = workspace
                    .panel::<AgentPanel>(cx)
                    .is_some_and(|panel| panel.read(cx).enabled(cx));
                if panel_enabled {
                    workspace.open_panel::<AgentPanel>(window, cx);
                }
                panel_enabled
            }))
        })
    }
}

/// Enrolls the user in the post-onboarding agent-panel experiment at most once
/// ever, when they *leave* onboarding (by finishing setup or navigating
/// elsewhere) having configured an agent. For the treatment arm this reveals the
/// agent panel (without taking keyboard focus); both arms emit an exposure event
/// so downstream analytics can measure engagement lift by arm.
///
/// We intentionally fire on leaving rather than the moment an agent is
/// configured, so the panel doesn't pop open mid-setup. The work is deferred
/// (via `with_active_or_new_workspace`) so it runs after the current
/// pane/item/focus update has settled rather than re-entering the workspace.
fn on_leave_onboarding(trigger: LeaveTrigger, cx: &mut App) {
    if already_exposed_to_experiment(cx) {
        return;
    }
    if !any_agent_configured(cx) {
        return;
    }

    // Assignment is computed from a stable installation id so logged-out users
    // are bucketed too. Without one (e.g. telemetry disabled) we can't enroll or
    // measure, so we leave the user out of the experiment entirely.
    let Some(installation_id) = Client::global(cx).telemetry().installation_id() else {
        return;
    };
    let arm = experiment_arm(&installation_id);
    let agents = configured_featured_agents(cx);
    let zed_agent = zed_agent_signed_in(cx);
    let revealer = AgentPanelRevealer::get(cx);

    with_active_or_new_workspace(cx, move |workspace, window, cx| {
        // Re-check inside the deferred closure: another exit (possibly earlier in
        // this same tick) may have already enrolled the user.
        if already_exposed_to_experiment(cx) {
            return;
        }
        // A modal opened from onboarding (e.g. the base keymap picker) counts as
        // still being in setup, so don't enroll/reveal underneath it. A later
        // exit can still enroll.
        if workspace.has_active_modal(window, cx) {
            return;
        }

        let revealed = match arm {
            ExperimentArm::Treatment => (revealer.0)(workspace, window, cx),
            ExperimentArm::Control => false,
        };

        mark_exposed_to_experiment(cx);
        telemetry::event!(
            "Onboarding Agent Experiment Exposed",
            arm = arm.as_str(),
            trigger = trigger.as_str(),
            revealed = revealed,
            configured_agents = agents,
            zed_agent_signed_in = zed_agent,
        );
    });
}

pub async fn handle_import_vscode_settings(
    workspace: WeakEntity<Workspace>,
    source: VsCodeSettingsSource,
    skip_prompt: bool,
    fs: Arc<dyn Fs>,
    cx: &mut AsyncWindowContext,
) {
    use util::truncate_and_remove_front;

    let vscode_settings =
        match settings::VsCodeSettings::load_user_settings(source, fs.clone()).await {
            Ok(vscode_settings) => vscode_settings,
            Err(err) => {
                zlog::error!("{err:?}");
                let _ = cx.prompt(
                    gpui::PromptLevel::Info,
                    &format!("Could not find or load a {source} settings file"),
                    None,
                    &["OK"],
                );
                return;
            }
        };

    if !skip_prompt {
        let prompt = cx.prompt(
            gpui::PromptLevel::Warning,
            &format!(
                "Importing {} settings may overwrite your existing settings. \
                Will import settings from {}",
                vscode_settings.source,
                truncate_and_remove_front(&vscode_settings.path.to_string_lossy(), 128),
            ),
            None,
            &["Import", "Cancel"],
        );
        let result = cx.spawn(async move |_| prompt.await.ok()).await;
        if result != Some(0) {
            return;
        }
    };

    let Ok(result_channel) = cx.update(|_, cx| {
        let source = vscode_settings.source;
        let path = vscode_settings.path.clone();
        let result_channel = cx
            .global::<SettingsStore>()
            .import_vscode_settings(fs, vscode_settings);
        zlog::info!("Imported {source} settings from {}", path.display());
        result_channel
    }) else {
        return;
    };

    let result = result_channel.await;
    workspace
        .update_in(cx, |workspace, _, cx| match result {
            Ok(_) => {
                let confirmation_toast = StatusToast::new(
                    format!("Your {} settings were successfully imported.", source),
                    cx,
                    |this, _| {
                        this.icon(
                            Icon::new(IconName::Check)
                                .size(IconSize::Small)
                                .color(Color::Success),
                        )
                        .dismiss_button(true)
                    },
                );
                SettingsImportState::update(cx, |state, _| match source {
                    VsCodeSettingsSource::VsCode => {
                        state.vscode = true;
                    }
                    VsCodeSettingsSource::Cursor => {
                        state.cursor = true;
                    }
                });
                workspace.toggle_status_toast(confirmation_toast, cx);
            }
            Err(_) => {
                let error_toast = StatusToast::new(
                    "Failed to import settings. See log for details",
                    cx,
                    |this, _| {
                        this.icon(
                            Icon::new(IconName::Close)
                                .size(IconSize::Small)
                                .color(Color::Error),
                        )
                        .action("Open Log", |window, cx| {
                            window.dispatch_action(workspace::OpenLog.boxed_clone(), cx)
                        })
                        .dismiss_button(true)
                    },
                );
                workspace.toggle_status_toast(error_toast, cx);
            }
        })
        .ok();
}

#[derive(Default, Copy, Clone)]
pub struct SettingsImportState {
    pub cursor: bool,
    pub vscode: bool,
}

impl Global for SettingsImportState {}

impl SettingsImportState {
    pub fn global(cx: &App) -> Self {
        cx.try_global().cloned().unwrap_or_default()
    }
    pub fn update<R>(cx: &mut App, f: impl FnOnce(&mut Self, &mut App) -> R) -> R {
        cx.update_default_global(f)
    }
}

impl workspace::SerializableItem for Onboarding {
    fn serialized_item_kind() -> &'static str {
        "OnboardingPage"
    }

    fn cleanup(
        workspace_id: workspace::WorkspaceId,
        alive_items: Vec<workspace::ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> gpui::Task<gpui::Result<()>> {
        workspace::delete_unloaded_items(
            alive_items,
            workspace_id,
            "onboarding_pages",
            &persistence::OnboardingPagesDb::global(cx),
            cx,
        )
    }

    fn deserialize(
        _project: Entity<project::Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: workspace::WorkspaceId,
        item_id: workspace::ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::Task<gpui::Result<Entity<Self>>> {
        let db = persistence::OnboardingPagesDb::global(cx);
        window.spawn(cx, async move |cx| {
            if let Some(_) = db.get_onboarding_page(item_id, workspace_id)? {
                workspace.update(cx, |workspace, cx| Onboarding::new(workspace, cx))
            } else {
                Err(anyhow::anyhow!("No onboarding page to deserialize"))
            }
        })
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: workspace::ItemId,
        _closing: bool,
        _window: &mut Window,
        cx: &mut ui::Context<Self>,
    ) -> Option<gpui::Task<gpui::Result<()>>> {
        let workspace_id = workspace.database_id()?;

        let db = persistence::OnboardingPagesDb::global(cx);
        Some(
            cx.background_spawn(
                async move { db.save_onboarding_page(item_id, workspace_id).await },
            ),
        )
    }

    fn should_serialize(&self, event: &Self::Event) -> bool {
        event == &ItemEvent::UpdateTab
    }
}

mod persistence {
    use db::{
        query,
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };
    use workspace::WorkspaceDb;

    pub struct OnboardingPagesDb(ThreadSafeConnection);

    impl Domain for OnboardingPagesDb {
        const NAME: &str = stringify!(OnboardingPagesDb);

        const MIGRATIONS: &[&str] = &[
            sql!(
                        CREATE TABLE onboarding_pages (
                            workspace_id INTEGER,
                            item_id INTEGER UNIQUE,
                            page_number INTEGER,

                            PRIMARY KEY(workspace_id, item_id),
                            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                            ON DELETE CASCADE
                        ) STRICT;
            ),
            sql!(
                        CREATE TABLE onboarding_pages_2 (
                            workspace_id INTEGER,
                            item_id INTEGER UNIQUE,

                            PRIMARY KEY(workspace_id, item_id),
                            FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                            ON DELETE CASCADE
                        ) STRICT;
                        INSERT INTO onboarding_pages_2 SELECT workspace_id, item_id FROM onboarding_pages;
                        DROP TABLE onboarding_pages;
                        ALTER TABLE onboarding_pages_2 RENAME TO onboarding_pages;
            ),
        ];
    }

    db::static_connection!(OnboardingPagesDb, [WorkspaceDb]);

    impl OnboardingPagesDb {
        query! {
            pub async fn save_onboarding_page(
                item_id: workspace::ItemId,
                workspace_id: workspace::WorkspaceId
            ) -> Result<()> {
                INSERT OR REPLACE INTO onboarding_pages(item_id, workspace_id)
                VALUES (?, ?)
            }
        }

        query! {
            pub fn get_onboarding_page(
                item_id: workspace::ItemId,
                workspace_id: workspace::WorkspaceId
            ) -> Result<Option<workspace::ItemId>> {
                SELECT item_id
                FROM onboarding_pages
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{DismissEvent, TestAppContext, UpdateGlobal, VisualTestContext};
    use project::Project;
    use settings::SettingsStore;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use workspace::{ModalView, MultiWorkspace, dock::test::TestPanel};

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            // Isolate the KVP DB so the once-ever flag is hermetic per test and
            // doesn't read/write the developer's real database.
            cx.set_global(db::AppDatabase::test_new());
            let app_state = workspace::AppState::test(cx);
            // `AppState::test` builds a client but doesn't install it as the
            // global; `Onboarding::new`/`any_agent_configured` read it via
            // `Client::global`.
            Client::set_global(app_state.client.clone(), cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            // The basics page renders theme previews for specific named themes
            // that aren't part of the base set; register stand-ins (cloned from
            // the base theme) so rendering doesn't panic in tests.
            let registry = theme::ThemeRegistry::global(cx);
            if let Ok(base) = registry.get("One Dark") {
                for name in [
                    "One Light",
                    "Ayu Light",
                    "Gruvbox Light",
                    "Ayu Dark",
                    "Gruvbox Dark",
                ] {
                    let mut theme = (*base).clone();
                    theme.name = name.into();
                    registry.insert_themes([theme]);
                }
            }
        });
    }

    /// Replaces the real agent-panel opener with a recorder, returning a counter
    /// of how many times a reveal actually happened.
    fn record_reveals(cx: &mut App) -> Arc<AtomicUsize> {
        let count = Arc::new(AtomicUsize::new(0));
        let count_for_revealer = count.clone();
        cx.set_global(AgentPanelRevealer(Arc::new(
            move |_workspace, _window, _cx| {
                count_for_revealer.fetch_add(1, Ordering::SeqCst);
                true
            },
        )));
        count
    }

    /// Marks an agent as configured so `any_agent_configured` returns true.
    fn set_agent_configured(cx: &mut App) {
        SettingsStore::update_global(cx, |store, cx| {
            store
                .set_user_settings(r#"{"agent_servers":{"cursor":{"type":"registry"}}}"#, cx)
                .unwrap();
        });
    }

    /// Returns an installation id that the experiment buckets into `arm`.
    fn installation_id_for(arm: ExperimentArm) -> String {
        (0..1_000_000u32)
            .map(|i| format!("install-{i}"))
            .find(|id| experiment_arm(id) == arm)
            .expect("an installation id should map to each arm")
    }

    fn already_exposed(cx: &mut VisualTestContext) -> bool {
        cx.update(|_, cx| already_exposed_to_experiment(cx))
    }

    /// Builds a workspace with an active, focused onboarding item and returns the
    /// reveal counter. `installation_id` controls experiment assignment; pass
    /// `None` to simulate a user with no stable id (e.g. telemetry disabled).
    /// Leaves the agent unconfigured by default.
    async fn setup_onboarding(
        cx: &mut TestAppContext,
        installation_id: Option<String>,
    ) -> (
        Entity<Workspace>,
        Entity<Onboarding>,
        Arc<AtomicUsize>,
        gpui::AnyWindowHandle,
    ) {
        let fs = fs::FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let window = cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = window
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let any_window: gpui::AnyWindowHandle = window.into();
        let cx = &mut VisualTestContext::from_window(any_window, cx);

        let count = cx.update(|_, cx| {
            if let Some(installation_id) = installation_id {
                Client::global(cx).telemetry().start(
                    Some("test-system".to_string()),
                    Some(installation_id),
                    "test-session".to_string(),
                    cx,
                );
            }
            record_reveals(cx)
        });

        let onboarding = workspace.update_in(cx, |workspace, window, cx| {
            let onboarding = Onboarding::new(workspace, cx);
            workspace.add_item_to_active_pane(Box::new(onboarding.clone()), None, true, window, cx);
            onboarding
        });
        // Render once so the focus-out subscription registers, then focus it.
        cx.run_until_parked();
        onboarding.update_in(cx, |onboarding, window, cx| {
            let handle = onboarding.focus_handle(cx);
            window.focus(&handle, cx);
        });
        cx.run_until_parked();

        (workspace, onboarding, count, any_window)
    }

    #[test]
    fn test_experiment_arm_is_deterministic_and_split() {
        // Stable for a given id.
        assert_eq!(experiment_arm("abc-123"), experiment_arm("abc-123"));

        // Roughly balanced around the 50% treatment ramp.
        let mut treatment = 0;
        let mut control = 0;
        for i in 0..2000 {
            match experiment_arm(&format!("installation-{i}")) {
                ExperimentArm::Treatment => treatment += 1,
                ExperimentArm::Control => control += 1,
            }
        }
        assert!(treatment > 800, "treatment was {treatment}");
        assert!(control > 800, "control was {control}");
    }

    #[gpui::test]
    async fn test_treatment_arm_reveals_on_finish(cx: &mut TestAppContext) {
        init_test(cx);
        let id = installation_id_for(ExperimentArm::Treatment);
        let (workspace, _onboarding, count, window) = setup_onboarding(cx, Some(id)).await;
        let cx = &mut VisualTestContext::from_window(window, cx);
        cx.update(|_, cx| set_agent_configured(cx));

        workspace.update_in(cx, |_, window, cx| {
            Onboarding::on_finish(&Finish, window, cx)
        });
        cx.run_until_parked();

        assert_eq!(count.load(Ordering::SeqCst), 1);
        assert!(already_exposed(cx));
    }

    #[gpui::test]
    async fn test_control_arm_enrolls_but_does_not_reveal(cx: &mut TestAppContext) {
        init_test(cx);
        let id = installation_id_for(ExperimentArm::Control);
        let (workspace, _onboarding, count, window) = setup_onboarding(cx, Some(id)).await;
        let cx = &mut VisualTestContext::from_window(window, cx);
        cx.update(|_, cx| set_agent_configured(cx));

        workspace.update_in(cx, |_, window, cx| {
            Onboarding::on_finish(&Finish, window, cx)
        });
        cx.run_until_parked();

        // Control users don't get the panel, but they are still enrolled (and
        // logged) exactly once, so they form the comparison cohort.
        assert_eq!(count.load(Ordering::SeqCst), 0);
        assert!(already_exposed(cx));
    }

    #[gpui::test]
    async fn test_not_configured_does_not_enroll(cx: &mut TestAppContext) {
        init_test(cx);
        let id = installation_id_for(ExperimentArm::Treatment);
        let (workspace, _onboarding, count, window) = setup_onboarding(cx, Some(id)).await;
        let cx = &mut VisualTestContext::from_window(window, cx);

        workspace.update_in(cx, |_, window, cx| {
            Onboarding::on_finish(&Finish, window, cx)
        });
        cx.run_until_parked();

        assert_eq!(count.load(Ordering::SeqCst), 0);
        assert!(!already_exposed(cx));
    }

    #[gpui::test]
    async fn test_no_installation_id_is_not_enrolled(cx: &mut TestAppContext) {
        init_test(cx);
        let (workspace, _onboarding, count, window) = setup_onboarding(cx, None).await;
        let cx = &mut VisualTestContext::from_window(window, cx);
        cx.update(|_, cx| set_agent_configured(cx));

        workspace.update_in(cx, |_, window, cx| {
            Onboarding::on_finish(&Finish, window, cx)
        });
        cx.run_until_parked();

        // Without a stable id we can't bucket or measure, so the user is left out.
        assert_eq!(count.load(Ordering::SeqCst), 0);
        assert!(!already_exposed(cx));
    }

    #[gpui::test]
    async fn test_deactivated_reveals_for_treatment(cx: &mut TestAppContext) {
        init_test(cx);
        let id = installation_id_for(ExperimentArm::Treatment);
        let (_workspace, onboarding, count, window) = setup_onboarding(cx, Some(id)).await;
        let cx = &mut VisualTestContext::from_window(window, cx);
        cx.update(|_, cx| set_agent_configured(cx));

        onboarding.update_in(cx, |onboarding, window, cx| {
            onboarding.deactivated(window, cx)
        });
        cx.run_until_parked();

        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[gpui::test]
    async fn test_focus_out_to_dock_panel_reveals_for_treatment(cx: &mut TestAppContext) {
        init_test(cx);
        let id = installation_id_for(ExperimentArm::Treatment);
        let (workspace, _onboarding, count, window) = setup_onboarding(cx, Some(id)).await;
        let cx = &mut VisualTestContext::from_window(window, cx);
        cx.update(|_, cx| set_agent_configured(cx));

        // Add a dock panel and move focus to it. Onboarding stays the active
        // center item, so this exercises the focus-out path (not deactivated).
        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = cx.new(|cx| TestPanel::new(workspace::dock::DockPosition::Left, 0, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });
        cx.run_until_parked();

        panel.update_in(cx, |panel, window, cx| {
            let handle = panel.focus_handle(cx);
            window.focus(&handle, cx);
        });
        cx.run_until_parked();

        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[gpui::test]
    async fn test_enrolls_only_once(cx: &mut TestAppContext) {
        init_test(cx);
        let id = installation_id_for(ExperimentArm::Treatment);
        let (workspace, onboarding, count, window) = setup_onboarding(cx, Some(id)).await;
        let cx = &mut VisualTestContext::from_window(window, cx);
        cx.update(|_, cx| set_agent_configured(cx));

        workspace.update_in(cx, |_, window, cx| {
            Onboarding::on_finish(&Finish, window, cx)
        });
        cx.run_until_parked();
        assert_eq!(count.load(Ordering::SeqCst), 1);

        // A subsequent leave must not reveal again.
        onboarding.update_in(cx, |onboarding, window, cx| {
            onboarding.deactivated(window, cx)
        });
        cx.run_until_parked();
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[gpui::test]
    async fn test_leaving_unconfigured_then_configured_enrolls_later(cx: &mut TestAppContext) {
        init_test(cx);
        let id = installation_id_for(ExperimentArm::Treatment);
        let (_workspace, onboarding, count, window) = setup_onboarding(cx, Some(id)).await;
        let cx = &mut VisualTestContext::from_window(window, cx);

        // Leave once with nothing configured: no enrollment, latch not spent.
        onboarding.update_in(cx, |onboarding, window, cx| {
            onboarding.deactivated(window, cx)
        });
        cx.run_until_parked();
        assert_eq!(count.load(Ordering::SeqCst), 0);
        assert!(!already_exposed(cx));

        // Configure an agent, then leave again: now it enrolls and reveals.
        cx.update(|_, cx| set_agent_configured(cx));
        onboarding.update_in(cx, |onboarding, window, cx| {
            onboarding.deactivated(window, cx)
        });
        cx.run_until_parked();
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[gpui::test]
    async fn test_modal_suppresses_enrollment(cx: &mut TestAppContext) {
        init_test(cx);
        let id = installation_id_for(ExperimentArm::Treatment);
        let (workspace, _onboarding, count, window) = setup_onboarding(cx, Some(id)).await;
        let cx = &mut VisualTestContext::from_window(window, cx);
        cx.update(|_, cx| set_agent_configured(cx));

        // Opening a modal moves focus out of onboarding, but a modal means we're
        // still in setup, so enrollment/reveal must be suppressed.
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_modal(window, cx, |_, cx| TestModal::new(cx));
        });
        cx.run_until_parked();
        assert_eq!(count.load(Ordering::SeqCst), 0);
        assert!(!already_exposed(cx));

        // Dismiss the modal and genuinely leave: now it enrolls and reveals.
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_modal(window, cx, |_, cx| TestModal::new(cx));
        });
        cx.run_until_parked();
        workspace.update_in(cx, |_, window, cx| {
            Onboarding::on_finish(&Finish, window, cx)
        });
        cx.run_until_parked();
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    struct TestModal {
        focus_handle: FocusHandle,
    }

    impl TestModal {
        fn new(cx: &mut Context<Self>) -> Self {
            Self {
                focus_handle: cx.focus_handle(),
            }
        }
    }

    impl Render for TestModal {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            gpui::div().track_focus(&self.focus_handle)
        }
    }

    impl Focusable for TestModal {
        fn focus_handle(&self, _: &App) -> FocusHandle {
            self.focus_handle.clone()
        }
    }

    impl EventEmitter<DismissEvent> for TestModal {}

    impl ModalView for TestModal {}
}
