use std::sync::Arc;
use fs::Fs;
use language::LanguageRegistry;
use ui::prelude::*;

pub(crate) fn init(_language_registry: Arc<LanguageRegistry>, _fs: Arc<dyn Fs>, _cx: &mut App) {}
