use gpui::{App, Window, prelude::*};

use language_model::LanguageModelRegistry;
use ui::prelude::*;

pub(crate) fn render_ai_setup_page(_: &mut Window, cx: &mut App) -> impl IntoElement {
    let registry = LanguageModelRegistry::read_global(cx);

    div().children(
        registry
            .available_models(cx)
            .into_iter()
            .map(|model| model.id().0),
    )
}
