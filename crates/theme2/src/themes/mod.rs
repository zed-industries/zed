mod andromeda;
mod ayu;
mod dracula;
mod gruvbox;
mod night_owl;
mod nord;
mod notctis;
mod palenight;
mod rose_pine;
mod solarized;
mod synthwave_84;

pub use andromeda::*;
pub use ayu::*;
pub use dracula::*;
pub use gruvbox::*;
pub use night_owl::*;
pub use nord::*;
pub use notctis::*;
pub use palenight::*;
pub use rose_pine::*;
pub use solarized::*;
pub use synthwave_84::*;

use crate::ThemeFamily;

pub(crate) fn all_imported_themes() -> Vec<ThemeFamily> {
    vec![
        rose_pine(),
        night_owl(),
        andromeda(),
        synthwave_84(),
        palenight(),
        dracula(),
        solarized(),
        nord(),
        notctis(),
        ayu(),
        gruvbox(),
    ]
}
