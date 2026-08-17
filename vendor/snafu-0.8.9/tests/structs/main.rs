// Using #[path] to work around a https://github.com/rust-lang/rustfmt/issues/4404
// Once fixed and released, switch to a `mod structs { ... }`

mod backtrace;
mod backtrace_attributes;
mod context_selector_name;
mod display;
mod from_option;
mod generics;
mod module;
mod no_context;
mod single_use_lifetimes;
mod source_attributes;
mod visibility;
mod with_source;
mod without_source;
