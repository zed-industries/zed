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
        ("tooltip.leave_call", "Leave Call"),
        ("tooltip.open_shared_screen", "Open shared screen"),
        ("tooltip.open_channel_notes", "Open Channel Notes"),
        ("tooltip.clear_filter", "Clear Filter"),
        ("tooltip.search_contact", "Search for new contact"),
        ("tooltip.create_channel", "Create a channel"),
        ("tooltip.open_channel_notes", "Open Channel Notes"),
        ("message.decline_invite", "Decline invite"),
        ("message.accept_invite", "Accept invite"),
        ("message.cancel_invite", "Cancel invite"),
        ("label.calling", "Calling"),
        ("label.guest", "Guest"),
        ("label.mic_only", "Mic only"),
        ("label.screen", "Screen"),
        ("label.notes", "notes"),
        ("label.contacts", "Contacts"),
        ("label.invite_new_contacts", "Invite new contacts"),
        ("label.notifications", "Notifications"),
        ("label.connect_to_view_notifications", "Connect to view notifications."),
        ("label.you_have_no_notifications", "You have no notifications."),
        ("label.add_a_contact", "Add a Contact"),
        ("label.call_diagnostics", "Call Diagnostics"),
        ("label.not_in_a_call", "Not in a call"),
        ("label.network", "Network"),
        ("label.manage_members", "Manage Members"),
        ("label.invite_members", "Invite Members"),
        ("label.invited", "Invited"),
        ("label.admin", "Admin"),
        ("label.guest", "Guest"),
        ("label.member", "Member"),
        ("label.you", "You"),
        ("label.public", "Public"),
        ("label.join_channel", "Join channel"),
        ("label.sign_in_to_enable_collaboration", "Sign in to enable collaboration."),
        ("label.show_all_channels", "Show All Channels"),
        ("label.show_active_channels", "Show Active Channels"),
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