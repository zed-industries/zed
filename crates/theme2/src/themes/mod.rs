
        mod andromeda;
mod dracula;
mod nord;
mod notctis;
mod ayu;
mod gruvbox;

        pub use andromeda::*;
pub use dracula::*;
pub use nord::*;
pub use notctis::*;
pub use ayu::*;
pub use gruvbox::*;

        
        use crate::ThemeFamily;

        pub(crate) fn all_imported_themes() -> Vec<ThemeFamily> {
            vec![andromeda(), dracula(), nord(), notctis(), ayu(), gruvbox()]
        }
        
        