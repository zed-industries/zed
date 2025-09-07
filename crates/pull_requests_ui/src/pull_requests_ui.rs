// pub mod create_pr_modal; // Temporarily disabled - needs API updates
// pub mod pr_detail_panel;
pub mod pr_detail_view;
pub mod pr_list_panel;

use gpui::App;
use settings::Settings;

// pub use create_pr_modal::CreatePrModal;
// pub use pr_detail_panel::PullRequestDetailPanel;
pub use pr_detail_view::PullRequestDetailView;
pub use pr_list_panel::{PullRequestListPanel, PullRequestListPanelSettings};

pub fn init(cx: &mut App) {
    eprintln!("PULL_REQUESTS_UI_INIT: Initializing pull_requests_ui module");
    PullRequestListPanelSettings::register(cx);
    pr_list_panel::init(cx);
    pr_detail_view::init(cx);
    eprintln!("PULL_REQUESTS_UI_INIT: Done initializing pull_requests_ui module");
}
