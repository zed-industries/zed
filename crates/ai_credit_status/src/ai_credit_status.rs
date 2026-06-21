mod fetch;
mod ai_credit_settings;
mod status_item;

pub use ai_credit_settings::AiCreditStatusSettings;
pub use status_item::AiCreditStatusItem;

use gpui::App;
use settings::Settings;

pub fn init(cx: &mut App) {
    AiCreditStatusSettings::register(cx);
}
