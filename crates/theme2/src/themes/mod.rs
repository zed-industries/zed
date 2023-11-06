mod andromeda;
mod ayu;
mod dracula;
mod gruvbox;
mod nord;
mod notctis;

pub use andromeda::*;
pub use ayu::*;
pub use dracula::*;
pub use gruvbox::*;
pub use nord::*;
pub use notctis::*;

use crate::ThemeFamily;

pub(crate) fn all_imported_themes() -> Vec<ThemeFamily> {
    vec![andromeda(), dracula(), nord(), notctis(), ayu(), gruvbox()]
}
