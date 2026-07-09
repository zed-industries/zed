use gpui_shared_string::SharedString;

pub use fluent_bundle::FluentValue;

pub fn t(key: &'static str) -> SharedString {
    SharedString::from(key)
}

pub fn t_args<'a>(
    key: &'static str,
    _args: impl IntoIterator<Item = (&'a str, FluentValue<'a>)>,
) -> SharedString {
    SharedString::from(key)
}
