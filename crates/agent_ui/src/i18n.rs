use std::collections::HashMap;
use std::sync::RwLock;

static CURRENT_LOCALE: RwLock<&'static str> = RwLock::new("en");

fn get_locale() -> String {
    CURRENT_LOCALE.read().unwrap().to_string()
}

fn get_translations_map() -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    HashMap::from([
        ("en", translations_en()),
    ])
}

fn translations_en() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("tooltip.dismiss", "Dismiss"),
        ("tooltip.clear_search", "Clear Search"),
        ("tooltip.view_details", "View Details"),
        ("tooltip.token_limit_reached", "Token limit reached"),
        ("tooltip.token_limit_close", "Token limit is close to exhaustion"),
        ("tooltip.model_no_tools", "This model does not support tools."),
        ("tooltip.stop_subagent", "Stop Subagent"),
        ("tooltip.minimize_subagent", "Minimize Subagent"),
        ("tooltip.loading_context", "Loading Added Context…"),
        ("tooltip.restores_files", "Restores all files..."),
        ("tooltip.line_below", "Everything below this line..."),
        ("tooltip.open_thread_markdown", "Open Thread as Markdown"),
        ("tooltip.scroll_recent", "Scroll To Most Recent User Prompt"),
        ("tooltip.scroll_top", "Scroll To Top"),
        ("tooltip.interrupted_edit", "Interrupted Edit"),
        ("tooltip.go_to_file", "Go to File"),
        ("tooltip.subagent_cancelled", "Subagent Cancelled"),
        ("tooltip.subagent_failed", "Subagent Failed"),
        ("tooltip.make_full_screen", "Make Subagent Full Screen"),
        ("tooltip.view_user_rules", "View User Rules"),
        ("tooltip.view_project_rules", "View Project Rules"),
        ("tooltip.retry_generation", "Retry Generation"),
        ("tooltip.dismiss_warning", "Dismiss Warning"),
        ("tooltip.new_version", "New version available"),
        ("tooltip.sync_thread", "Sync with source thread"),
        ("tooltip.share_thread", "Share Thread"),
        ("tooltip.thanks_feedback", "Thanks for your feedback!"),
        ("tooltip.configure_mcp", "Configure MCP Server"),
        ("tooltip.uninstall_extension", "Uninstall Agent Extension"),
        ("tooltip.remove_registry", "Remove Registry Agent"),
        ("tooltip.remove_custom", "Remove Custom Agent"),
        ("tooltip.generating_changes", "Generating Changes…"),
        ("message.trial_expired", "Your Zed Pro Trial has expired"),
        ("message.auto_reset_free", "You've been automatically reset to the Free plan."),
        ("message.pro", "Pro"),
        ("message.free", "Free"),
        ("message.current_plan", "(Current Plan)"),
        ("message.claude_agent", "Claude Agent: Natively in Zed"),
        ("message.beta_release", "Beta Release"),
        ("message.bring_agent", "Bring Your Own Agent to Zed"),
        ("message.now_available", "Now Available"),
        ("message.agent_here", "Your Agent Here"),
        ("message.gemini_cli", "New Gemini CLI Thread"),
        ("message.you", "You"),
        ("message.agent", "Agent"),
        ("message.system", "System"),
        ("message.canceled", "Canceled"),
        ("message.free_exceeded", "Free Usage Exceeded"),
        ("message.error_model", "Error interacting with language model"),
        ("message.thinking", "Thinking…"),
        ("message.thought_process", "Thought Process"),
        ("message.awaiting_confirmation", "Awaiting Confirmation"),
        ("message.run_command", "Run Command"),
        ("message.truncated", "Truncated"),
        ("message.subagent_output", "Subagent Output"),
        ("message.unavailable_editing", "Unavailable Editing"),
        ("message.queue_send", "Queue and Send"),
        ("message.send_immediately", "Send Immediately"),
        ("message.change_effort", "Change Thinking Effort"),
        ("message.cycle_effort", "Cycle Thinking Effort"),
        ("message.context", "Context"),
        ("message.rules", "Rules"),
        ("message.edits", "Edits"),
        ("message.plan", "Plan"),
        ("message.current", "Current:"),
        ("message.no_threads", "You don't have any past threads yet."),
        ("message.no_match", "No threads match your search."),
        ("message.delete_all", "Delete all threads?"),
        ("message.no_recover", "You won't be able to recover them later."),
        ("message.new_thread", "New Thread…"),
        ("message.settings", "Settings"),
        ("message.agent_panel", "Agent"),
        ("message.creating_worktree", "Creating worktree…"),
        ("message.no_mcp_servers", "No MCP servers added yet."),
        ("message.waiting_server", "Waiting for Context Server"),
        ("message.models", "Models"),
        ("message.customize", "Customize"),
        ("message.custom_profiles", "Custom Profiles"),
        ("message.add_profile", "Add New Profile"),
        ("message.fork_profile", "Fork Profile"),
        ("message.configure_model", "Configure Default Model"),
        ("message.configure_tools", "Configure Built-in Tools"),
        ("message.configure_mcp_tools", "Configure MCP Tools"),
        ("message.delete_profile", "Delete Profile"),
        ("message.go_back", "Go Back"),
        ("message.change_model", "Change Model"),
        ("message.cycle_favorites", "Cycle Favorited Models"),
        ("message.change_profile", "Change Profile"),
        ("message.cycle_profiles", "Cycle Through Profiles"),
        ("message.change_mode", "Change Mode"),
        ("message.cycle_modes", "Cycle Through Modes"),
        ("message.request_refused", "Request Refused"),
        ("message.auth_required", "Authentication Required"),
        ("message.free_usage", "Free Usage Exceeded"),
        ("message.error_happened", "An Error Happened"),
        ("message.resumed_session", "Resumed Session"),
        ("message.codex_windows", "Codex on Windows"),
        ("message.review_sending", "Review before sending"),
        ("message.sign_in", "Sign in to continue using Zed as your LLM provider."),
        ("message.restricted_mode", "You're in Restricted Mode"),
        ("message.loading", "Loading…"),
        ("message.missing_entry", "Missing registry entry."),
        ("message.not_supported", "Not supported on this platform"),
        ("message.choose_auth", "Choose one of the following authentication options:"),
        ("message.create_command", "create-your-command"),
        ("message.create_custom", "Create your custom command"),
    ])
}

pub fn t(key: &str) -> String {
    let locale = get_locale();
    let map = get_translations_map();
    map.get(locale.as_str())
        .and_then(|m| m.get(key))
        .copied()
        .unwrap_or_else(|| {
            map.get("en")
                .and_then(|m| m.get(key))
                .copied()
                .unwrap_or(key)
        })
        .to_string()
}