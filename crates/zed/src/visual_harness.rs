//! Reusable harness for visual tests that drive real Zed UI programmatically.
//!
//! `setup_zed_for_visual_test` initialises all Zed subsystems inside a
//! `VisualTestAppContext` and opens an off-screen workspace window.  Callers get
//! back a `WindowHandle<Workspace>` they can drive with `dispatch_action`,
//! `simulate_keystrokes`, `record_frame`, etc.

use anyhow::{Context as _, Result};
use assets::Assets;
use gpui::{App, AppContext as _, Bounds, VisualTestAppContext, WindowBounds, WindowHandle, WindowOptions, point, px, size};
use settings::{NotifyWhenAgentWaiting, PlaySoundWhenAgentDone, Settings as _};
use std::sync::Arc;
use workspace::{AppState, Workspace};

/// Initialises all Zed subsystems and opens an off-screen workspace window.
///
/// The returned `WindowHandle<Workspace>` is positioned off-screen and invisible
/// to the user but fully rendered by the Metal compositor.  Use it with
/// `cx.dispatch_action`, `cx.simulate_keystrokes`, and `cx.record_frame`.
pub fn setup_zed_for_visual_test(
    cx: &mut VisualTestAppContext,
) -> Result<(WindowHandle<Workspace>, Arc<AppState>)> {
    // Load embedded fonts so UI renders with correct typefaces
    cx.update(|cx| {
        Assets.load_fonts(cx).unwrap();
    });

    // Real default settings (not the test settings that use Courier font)
    cx.update(|cx| {
        settings::init(cx);
    });

    let app_state = cx.update(|cx| init_app_state(cx));

    cx.update(|cx| {
        AppState::set_global(app_state.clone(), cx);
    });

    cx.update(|cx| {
        gpui_tokio::init(cx);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        client::init(&app_state.client, cx);
        audio::init(cx);
        workspace::init(app_state.clone(), cx);
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        command_palette::init(cx);
        editor::init(cx);
        call::init(app_state.client.clone(), app_state.user_store.clone(), cx);
        title_bar::init(cx);
        project_panel::init(cx);
        outline_panel::init(cx);
        terminal_view::init(cx);
        image_viewer::init(cx);
        search::init(cx);
        cx.set_global(workspace::PaneSearchBarCallbacks {
            setup_search_bar: |languages, toolbar, window, cx| {
                let search_bar = cx.new(|cx| search::BufferSearchBar::new(languages, window, cx));
                toolbar.update(cx, |toolbar, cx| {
                    toolbar.add_item(search_bar, window, cx);
                });
            },
            wrap_div_with_search_actions: search::buffer_search::register_pane_search_actions,
        });
        prompt_store::init(cx);
        let prompt_builder = prompt_store::PromptBuilder::load(app_state.fs.clone(), false, cx);
        language_model::init(cx);
        client::RefreshLlmTokenListener::register(
            app_state.client.clone(),
            app_state.user_store.clone(),
            cx,
        );
        language_models::init(app_state.user_store.clone(), app_state.client.clone(), cx);
        git_ui::init(cx);
        project::AgentRegistryStore::init_global(
            cx,
            app_state.fs.clone(),
            app_state.client.http_client(),
        );
        agent_ui::init(
            app_state.fs.clone(),
            prompt_builder,
            app_state.languages.clone(),
            true,
            false,
            cx,
        );
        settings_ui::init(cx);

        agent_settings::AgentSettings::override_global(
            agent_settings::AgentSettings {
                notify_when_agent_waiting: NotifyWhenAgentWaiting::Never,
                play_sound_when_agent_done: PlaySoundWhenAgentDone::Never,
                ..agent_settings::AgentSettings::get_global(cx).clone()
            },
            cx,
        );
    });

    cx.run_until_parked();

    let bounds = Bounds {
        origin: point(px(0.0), px(0.0)),
        size: size(px(1280.0), px(800.0)),
    };

    let project = cx.update(|cx| {
        project::Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            project::LocalProjectFlags {
                init_worktree_trust: false,
                ..Default::default()
            },
            cx,
        )
    });

    let workspace_window: WindowHandle<Workspace> = cx
        .update(|cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    focus: false,
                    show: false,
                    ..Default::default()
                },
                |window, cx| {
                    cx.new(|cx| {
                        Workspace::new(None, project.clone(), app_state.clone(), window, cx)
                    })
                },
            )
        })
        .context("Failed to open workspace window")?;

    cx.run_until_parked();

    Ok((workspace_window, app_state))
}

fn init_app_state(cx: &mut App) -> Arc<AppState> {
    use fs::Fs;
    use node_runtime::NodeRuntime;
    use session::Session;
    use settings::SettingsStore;

    if !cx.has_global::<SettingsStore>() {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
    }

    let fs: Arc<dyn Fs> = Arc::new(fs::RealFs::new(None, cx.background_executor().clone()));
    <dyn Fs>::set_global(fs.clone(), cx);

    let languages = Arc::new(language::LanguageRegistry::test(
        cx.background_executor().clone(),
    ));
    let clock = Arc::new(clock::FakeSystemClock::new());
    let http_client = http_client::FakeHttpClient::with_404_response();
    let client = client::Client::new(clock, http_client, cx);
    let session = cx.new(|cx| session::AppSession::new(Session::test(), cx));
    let user_store = cx.new(|cx| client::UserStore::new(client.clone(), cx));
    let workspace_store = cx.new(|cx| workspace::WorkspaceStore::new(client.clone(), cx));

    Arc::new(AppState {
        client,
        fs,
        languages,
        user_store,
        workspace_store,
        node_runtime: NodeRuntime::unavailable(),
        build_window_options: |_, _| Default::default(),
        session,
    })
}
