/// Run the component preview application.
///
/// This initializes the application with minimal required infrastructure
/// and opens a workspace with the ComponentPreview item.
#[cfg(feature = "preview")]
pub fn run_component_preview() {
    use fs::RealFs;
    use gpui::{
        AppContext as _, Application, Bounds, KeyBinding, WindowBounds, WindowOptions, actions,
        size,
    };

    use client::{Client, UserStore};
    use language::LanguageRegistry;
    use node_runtime::NodeRuntime;
    use project::Project;
    use reqwest_client::ReqwestClient;
    use session::{AppSession, Session};
    use std::sync::Arc;
    use ui::{App, px};
    use workspace::{AppState, Workspace, WorkspaceStore};

    use crate::{ComponentPreview, init};

    actions!(zed, [Quit]);

    fn quit(_: &Quit, cx: &mut App) {
        cx.quit();
    }

    Application::new().run(|cx| {
        component::init();

        cx.on_action(quit);
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        let version = release_channel::AppVersion::load(env!("CARGO_PKG_VERSION"), None, None);
        release_channel::init(version, cx);

        let http_client =
            ReqwestClient::user_agent("component_preview").expect("Failed to create HTTP client");
        cx.set_http_client(Arc::new(http_client));

        let fs = Arc::new(RealFs::new(None, cx.background_executor().clone()));
        <dyn fs::Fs>::set_global(fs.clone(), cx);

        settings::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);

        let languages = Arc::new(LanguageRegistry::new(cx.background_executor().clone()));
        let client = Client::production(cx);
        client::init(&client, cx);

        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        let workspace_store = cx.new(|cx| WorkspaceStore::new(client.clone(), cx));
        let session_id = uuid::Uuid::new_v4().to_string();
        let session = cx.foreground_executor().block_on(Session::new(session_id));
        let session = cx.new(|cx| AppSession::new(session, cx));
        let node_runtime = NodeRuntime::unavailable();

        let app_state = Arc::new(AppState {
            languages,
            client,
            user_store,
            workspace_store,
            fs,
            build_window_options: |_, _| Default::default(),
            node_runtime,
            session,
        });
        AppState::set_global(Arc::downgrade(&app_state), cx);

        workspace::init(app_state.clone(), cx);
        init(app_state.clone(), cx);

        let size = size(px(1200.), px(800.));
        let bounds = Bounds::centered(None, size, cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            {
                move |window, cx| {
                    let app_state = app_state;
                    theme::setup_ui_font(window, cx);

                    let project = Project::local(
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
                    );

                    let workspace = cx.new(|cx| {
                        Workspace::new(
                            Default::default(),
                            project.clone(),
                            app_state.clone(),
                            window,
                            cx,
                        )
                    });

                    workspace.update(cx, |workspace, cx| {
                        let weak_workspace = cx.entity().downgrade();
                        let language_registry = app_state.languages.clone();
                        let user_store = app_state.user_store.clone();

                        let component_preview = cx.new(|cx| {
                            ComponentPreview::new(
                                weak_workspace,
                                project,
                                language_registry,
                                user_store,
                                None,
                                None,
                                window,
                                cx,
                            )
                            .expect("Failed to create component preview")
                        });

                        workspace.add_item_to_active_pane(
                            Box::new(component_preview),
                            None,
                            true,
                            window,
                            cx,
                        );
                    });

                    workspace
                }
            },
        )
        .expect("Failed to open component preview window");

        cx.activate(true);
    });
}
