//! This crate provides UI components that can be used for form-like scenarios, such as a input and number field.
//!
//! It can't be located in the `ui` crate because it depends on `editor`.
//!
mod input_field;

use std::{
    any::Any,
    sync::{Arc, OnceLock},
};

use gpui::{FocusHandle, Subscription};
pub use input_field::*;
use ui::{AnyElement, App, Window};

pub trait ErasedEditor: 'static {
    fn text(&self, cx: &App) -> String;
    fn set_text(&self, text: &str, window: &mut Window, cx: &mut App);
    fn clear(&self, window: &mut Window, cx: &mut App);
    fn set_placeholder_text(&self, text: &str, window: &mut Window, _: &mut App);
    fn move_selection_to_end(&self, window: &mut Window, _: &mut App);

    fn focus_handle(&self, cx: &App) -> FocusHandle;

    fn subscribe(
        &self,
        callback: Box<dyn FnMut(ErasedEditorEvent, &mut Window, &mut App) + 'static>,
        window: &mut Window,
        cx: &mut App,
    ) -> Subscription;
    fn render(&self, window: &mut Window, cx: &App) -> AnyElement;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErasedEditorEvent {
    BufferEdited,
    Blurred,
}
pub static ERASED_EDITOR_FACTORY: OnceLock<fn(&mut Window, &mut App) -> Arc<dyn ErasedEditor>> =
    OnceLock::new();
