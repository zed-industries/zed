use super::*;
use feature_flags::FeatureFlagAppExt;
use fs::FakeFs;
use gpui::TestAppContext;
use project::DisableAiSettings;
use settings::SettingsStore;

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        DisableAiSettings::register(cx);
        cx.update_flags(false, vec!["agent-v2".into()]);
    });
}

#[gpui::test]
async fn test_sidebar_disabled_when_disable_ai_is_enabled(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, [], cx).await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));

    multi_workspace.read_with(cx, |mw, cx| {
        assert!(mw.multi_workspace_enabled(cx));
    });

    multi_workspace.update_in(cx, |mw, _window, cx| {
        mw.open_sidebar(cx);
        assert!(mw.sidebar_open());
    });

    cx.update(|_window, cx| {
        DisableAiSettings::override_global(DisableAiSettings { disable_ai: true }, cx);
    });
    cx.run_until_parked();

    multi_workspace.read_with(cx, |mw, cx| {
        assert!(
            !mw.sidebar_open(),
            "Sidebar should be closed when disable_ai is true"
        );
        assert!(
            !mw.multi_workspace_enabled(cx),
            "Multi-workspace should be disabled when disable_ai is true"
        );
    });

    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.toggle_sidebar(window, cx);
    });
    multi_workspace.read_with(cx, |mw, _cx| {
        assert!(
            !mw.sidebar_open(),
            "Sidebar should remain closed when toggled with disable_ai true"
        );
    });

    cx.update(|_window, cx| {
        DisableAiSettings::override_global(DisableAiSettings { disable_ai: false }, cx);
    });
    cx.run_until_parked();

    multi_workspace.read_with(cx, |mw, cx| {
        assert!(
            mw.multi_workspace_enabled(cx),
            "Multi-workspace should be enabled after re-enabling AI"
        );
        assert!(
            !mw.sidebar_open(),
            "Sidebar should still be closed after re-enabling AI (not auto-opened)"
        );
    });

    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.toggle_sidebar(window, cx);
    });
    multi_workspace.read_with(cx, |mw, _cx| {
        assert!(
            mw.sidebar_open(),
            "Sidebar should open when toggled after re-enabling AI"
        );
    });
}

#[gpui::test]
async fn test_replace(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    let project_a = Project::test(fs.clone(), [], cx).await;
    let project_b = Project::test(fs.clone(), [], cx).await;
    let project_c = Project::test(fs.clone(), [], cx).await;
    let project_d = Project::test(fs.clone(), [], cx).await;

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project_a.clone(), window, cx));

    let workspace_a_id = multi_workspace.read_with(cx, |mw, _cx| mw.workspaces()[0].entity_id());

    // Replace the only workspace (single-workspace case).
    let workspace_b = multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = cx.new(|cx| Workspace::test_new(project_b.clone(), window, cx));
        mw.replace(workspace.clone(), &*window, cx);
        workspace
    });

    multi_workspace.read_with(cx, |mw, _cx| {
        assert_eq!(mw.workspaces().len(), 1);
        assert_eq!(
            mw.workspaces()[0].entity_id(),
            workspace_b.entity_id(),
            "slot should now be project_b"
        );
        assert_ne!(
            mw.workspaces()[0].entity_id(),
            workspace_a_id,
            "project_a should be gone"
        );
    });

    // Add project_c as a second workspace, then replace it with project_d.
    let workspace_c = multi_workspace.update_in(cx, |mw, window, cx| {
        mw.test_add_workspace(project_c.clone(), window, cx)
    });

    multi_workspace.read_with(cx, |mw, _cx| {
        assert_eq!(mw.workspaces().len(), 2);
        assert_eq!(mw.active_workspace_index(), 1);
    });

    let workspace_d = multi_workspace.update_in(cx, |mw, window, cx| {
        let workspace = cx.new(|cx| Workspace::test_new(project_d.clone(), window, cx));
        mw.replace(workspace.clone(), &*window, cx);
        workspace
    });

    multi_workspace.read_with(cx, |mw, _cx| {
        assert_eq!(mw.workspaces().len(), 2, "should still have 2 workspaces");
        assert_eq!(mw.active_workspace_index(), 1);
        assert_eq!(
            mw.workspaces()[1].entity_id(),
            workspace_d.entity_id(),
            "active slot should now be project_d"
        );
        assert_ne!(
            mw.workspaces()[1].entity_id(),
            workspace_c.entity_id(),
            "project_c should be gone"
        );
    });

    // Replace with workspace_b which is already in the list — should just switch.
    multi_workspace.update_in(cx, |mw, window, cx| {
        mw.replace(workspace_b.clone(), &*window, cx);
    });

    multi_workspace.read_with(cx, |mw, _cx| {
        assert_eq!(
            mw.workspaces().len(),
            2,
            "no workspace should be added or removed"
        );
        assert_eq!(
            mw.active_workspace_index(),
            0,
            "should have switched to workspace_b"
        );
    });
}
