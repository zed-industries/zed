use crate::*;
use assistant_tool::Tool;
use client::{Client, UserStore};
use gpui::{AppContext, TestAppContext};
use project::Project;
use serde_json::json;
use std::sync::Arc;

#[gpui::test]
async fn test_create_channel_tool(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings_store = settings::SettingsStore::test(cx);
        cx.set_global(settings_store);
        release_channel::init(gpui::SemanticVersion::default(), cx);
        client::init_settings(cx);
        Project::init_settings(cx);
    });

    // Create a minimal test setup without server
    let http_client = http_client::FakeHttpClient::create(|_| async { unreachable!() });
    let client = cx.update(|cx| {
        Client::new(
            Arc::new(clock::FakeSystemClock::new()),
            http_client.clone(),
            cx,
        )
    });

    let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
    cx.update(|cx| {
        channel::init(&client, user_store.clone(), cx);
    });
    let channel_store = cx.update(|cx| channel::ChannelStore::global(cx));

    let _project = cx.update(|cx| {
        Project::local(
            client.clone(),
            node_runtime::NodeRuntime::unavailable(),
            user_store.clone(),
            Arc::new(language::LanguageRegistry::new(
                cx.background_executor().clone(),
            )),
            fs::FakeFs::new(cx.background_executor().clone()),
            None,
            cx,
        )
    });

    let tool = Arc::new(CreateChannelTool::new(channel_store.clone()));

    // Test tool schema
    let schema = tool
        .input_schema(language_model::LanguageModelToolSchemaFormat::JsonSchema)
        .unwrap();
    assert!(schema.is_object());

    // Test UI text generation
    let input = json!({
        "name": "test-channel",
        "visibility": "members"
    });

    let ui_text = tool.ui_text(&input);
    assert_eq!(
        ui_text,
        "Create channel 'test-channel' (visibility: members)"
    );

    // Test needs_confirmation
    cx.update(|cx| {
        assert!(!tool.needs_confirmation(&input, cx));
    });

    // Test with parent channel
    let input_with_parent = json!({
        "name": "sub-channel",
        "parent": "parent-channel",
        "visibility": "public"
    });

    let ui_text = tool.ui_text(&input_with_parent);
    assert_eq!(
        ui_text,
        "Create channel 'sub-channel' under 'parent-channel' (visibility: public)"
    );
}

#[gpui::test]
async fn test_move_channel_tool(cx: &mut TestAppContext) {
    // Create minimal setup
    cx.update(|cx| {
        let settings_store = settings::SettingsStore::test(cx);
        cx.set_global(settings_store);
        release_channel::init(gpui::SemanticVersion::default(), cx);
        client::init_settings(cx);
        Project::init_settings(cx);
    });

    let http_client = http_client::FakeHttpClient::create(|_| async { unreachable!() });
    let client = cx.update(|cx| {
        Client::new(
            Arc::new(clock::FakeSystemClock::new()),
            http_client.clone(),
            cx,
        )
    });
    let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
    cx.update(|cx| {
        channel::init(&client, user_store.clone(), cx);
    });
    let channel_store = cx.update(|cx| channel::ChannelStore::global(cx));

    let tool = Arc::new(MoveChannelTool::new(channel_store));

    // Test schema
    let schema = tool
        .input_schema(language_model::LanguageModelToolSchemaFormat::JsonSchema)
        .unwrap();
    assert!(schema.is_object());

    // Test UI text generation
    let input = json!({
        "channel": "child",
        "to": "new-parent"
    });

    let ui_text = tool.ui_text(&input);
    assert_eq!(ui_text, "Move channel 'child' to 'new-parent'");

    // Test moving to root
    let input_to_root = json!({
        "channel": "child",
        "to": null
    });

    let ui_text = tool.ui_text(&input_to_root);
    assert_eq!(ui_text, "Move channel 'child' to root");
}

#[gpui::test]
async fn test_reorder_channel_tool(cx: &mut TestAppContext) {
    // Create minimal setup
    cx.update(|cx| {
        let settings_store = settings::SettingsStore::test(cx);
        cx.set_global(settings_store);
        release_channel::init(gpui::SemanticVersion::default(), cx);
        client::init_settings(cx);
        Project::init_settings(cx);
    });

    let http_client = http_client::FakeHttpClient::create(|_| async { unreachable!() });
    let client = cx.update(|cx| {
        Client::new(
            Arc::new(clock::FakeSystemClock::new()),
            http_client.clone(),
            cx,
        )
    });
    let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
    cx.update(|cx| {
        channel::init(&client, user_store.clone(), cx);
    });
    let channel_store = cx.update(|cx| channel::ChannelStore::global(cx));

    let tool = Arc::new(ReorderChannelTool::new(channel_store));

    // Test schema
    let schema = tool
        .input_schema(language_model::LanguageModelToolSchemaFormat::JsonSchema)
        .unwrap();
    assert!(schema.is_object());

    // Test UI text generation
    let input = json!({
        "channel": "test-channel",
        "direction": "up"
    });

    let ui_text = tool.ui_text(&input);
    assert_eq!(ui_text, "Move channel 'test-channel' up");

    // Test with down direction
    let input_down = json!({
        "channel": "test-channel",
        "direction": "down"
    });

    let ui_text = tool.ui_text(&input_down);
    assert_eq!(ui_text, "Move channel 'test-channel' down");
}

#[gpui::test]
async fn test_edit_channel_notes_tool(cx: &mut TestAppContext) {
    // Create minimal setup
    cx.update(|cx| {
        let settings_store = settings::SettingsStore::test(cx);
        cx.set_global(settings_store);
        release_channel::init(gpui::SemanticVersion::default(), cx);
        client::init_settings(cx);
        Project::init_settings(cx);
    });

    let http_client = http_client::FakeHttpClient::create(|_| async { unreachable!() });
    let client = cx.update(|cx| {
        Client::new(
            Arc::new(clock::FakeSystemClock::new()),
            http_client.clone(),
            cx,
        )
    });
    let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
    cx.update(|cx| {
        channel::init(&client, user_store.clone(), cx);
    });
    let channel_store = cx.update(|cx| channel::ChannelStore::global(cx));

    let tool = Arc::new(EditChannelNotesTool::new(channel_store));

    // Test schema
    let schema = tool
        .input_schema(language_model::LanguageModelToolSchemaFormat::JsonSchema)
        .unwrap();
    assert!(schema.is_object());

    // Test UI text generation - create
    let input_create = json!({
        "channel": "test-channel",
        "edits": [
            {
                "kind": "create",
                "content": "# Welcome\n\nChannel notes here."
            }
        ]
    });

    let ui_text = tool.ui_text(&input_create);
    assert_eq!(ui_text, "Create notes for channel 'test-channel'");

    // Test UI text generation - edit
    let input_edit = json!({
        "channel": "test-channel",
        "edits": [
            {
                "kind": "edit",
                "content": "Updated content"
            }
        ]
    });

    let ui_text = tool.ui_text(&input_edit);
    assert_eq!(ui_text, "Edit notes for channel 'test-channel'");

    // Test UI text generation - append
    let input_append = json!({
        "channel": "test-channel",
        "edits": [
            {
                "kind": "append",
                "content": "\n\n## New Section"
            }
        ]
    });

    let ui_text = tool.ui_text(&input_append);
    assert_eq!(ui_text, "Append to notes for channel 'test-channel'");

    // Test multiple edits
    let input_multiple = json!({
        "channel": "test-channel",
        "edits": [
            {
                "kind": "edit",
                "content": "Edit 1"
            },
            {
                "kind": "append",
                "content": "Edit 2"
            }
        ]
    });

    let ui_text = tool.ui_text(&input_multiple);
    assert_eq!(
        ui_text,
        "Apply multiple edits to notes for channel 'test-channel'"
    );
}

#[gpui::test]
async fn test_channel_tools_confirmation(cx: &mut TestAppContext) {
    // Create minimal setup
    cx.update(|cx| {
        let settings_store = settings::SettingsStore::test(cx);
        cx.set_global(settings_store);
        release_channel::init(gpui::SemanticVersion::default(), cx);
        client::init_settings(cx);
        Project::init_settings(cx);
    });

    let http_client = http_client::FakeHttpClient::create(|_| async { unreachable!() });
    let client = cx.update(|cx| {
        Client::new(
            Arc::new(clock::FakeSystemClock::new()),
            http_client.clone(),
            cx,
        )
    });
    let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
    cx.update(|cx| {
        channel::init(&client, user_store.clone(), cx);
    });
    let channel_store = cx.update(|cx| channel::ChannelStore::global(cx));

    // Test that move tool requires confirmation
    let move_tool = Arc::new(MoveChannelTool::new(channel_store.clone()));
    let input = json!({
        "channel": "important-channel",
        "to": "new-location"
    });

    cx.update(|cx| {
        assert!(move_tool.needs_confirmation(&input, cx));
    });

    // Test that create tool doesn't require confirmation
    let create_tool = Arc::new(CreateChannelTool::new(channel_store));
    let input = json!({
        "name": "new-channel",
        "visibility": "members"
    });

    cx.update(|cx| {
        assert!(!create_tool.needs_confirmation(&input, cx));
    });
}
