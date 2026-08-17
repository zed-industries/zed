pub type Result<T> = std::result::Result<T, crate::error::PortalError>;

pub mod access;
#[cfg(feature = "account")]
#[cfg_attr(docsrs, doc(cfg(feature = "account")))]
pub mod account;
pub mod app_chooser;
#[cfg(feature = "background")]
#[cfg_attr(docsrs, doc(cfg(feature = "background")))]
pub mod background;
mod builder;
pub use builder::Builder;
#[cfg(feature = "email")]
#[cfg_attr(docsrs, doc(cfg(feature = "email")))]
pub mod email;
#[cfg(feature = "file_chooser")]
#[cfg_attr(docsrs, doc(cfg(feature = "file_chooser")))]
pub mod file_chooser;
pub mod lockdown;
#[cfg(feature = "documents")]
#[cfg_attr(docsrs, doc(cfg(feature = "documents")))]
pub mod permission_store;
#[cfg(feature = "print")]
#[cfg_attr(docsrs, doc(cfg(feature = "print")))]
pub mod print;
pub mod request;
#[cfg(feature = "screencast")]
#[cfg_attr(docsrs, doc(cfg(feature = "screencast")))]
pub mod screencast;
#[cfg(feature = "screenshot")]
#[cfg_attr(docsrs, doc(cfg(feature = "screenshot")))]
pub mod screenshot;
#[cfg(feature = "secret")]
#[cfg_attr(docsrs, doc(cfg(feature = "secret")))]
pub mod secret;
pub mod session;
#[cfg(feature = "settings")]
#[cfg_attr(docsrs, doc(cfg(feature = "settings")))]
pub mod settings;
mod spawn;
#[cfg(feature = "usb")]
#[cfg_attr(docsrs, doc(cfg(feature = "usb")))]
pub mod usb;
#[cfg(feature = "wallpaper")]
#[cfg_attr(docsrs, doc(cfg(feature = "wallpaper")))]
pub mod wallpaper;
