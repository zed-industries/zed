use ui::prelude::*;
use ui::{Keybinding, ModifierKeys, Palette, PaletteItem};

use crate::story::Story;

#[derive(Element, Default)]
pub struct PaletteStory {}

impl PaletteStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title_for::<_, Palette<V>>(cx))
            .child(Story::label(cx, "Default"))
            .child(Palette::new(ScrollState::default()))
            .child(Story::label(cx, "With Items"))
            .child(
                Palette::new(ScrollState::default())
                    .placeholder("Execute a command...")
                    .items(vec![
                        PaletteItem::new("theme selector: toggle").keybinding(
                            Keybinding::new_chord(
                                ("k".to_string(), ModifierKeys::new().command(true)),
                                ("t".to_string(), ModifierKeys::new().command(true)),
                            ),
                        ),
                        PaletteItem::new("assistant: inline assist").keybinding(Keybinding::new(
                            "enter".to_string(),
                            ModifierKeys::new().command(true),
                        )),
                        PaletteItem::new("assistant: quote selection").keybinding(Keybinding::new(
                            ">".to_string(),
                            ModifierKeys::new().command(true),
                        )),
                        PaletteItem::new("assistant: toggle focus").keybinding(Keybinding::new(
                            "?".to_string(),
                            ModifierKeys::new().command(true),
                        )),
                        PaletteItem::new("auto update: check"),
                        PaletteItem::new("auto update: view release notes"),
                        PaletteItem::new("branches: open recent").keybinding(Keybinding::new(
                            "b".to_string(),
                            ModifierKeys::new().command(true).alt(true),
                        )),
                        PaletteItem::new("chat panel: toggle focus"),
                        PaletteItem::new("cli: install"),
                        PaletteItem::new("client: sign in"),
                        PaletteItem::new("client: sign out"),
                        PaletteItem::new("editor: cancel")
                            .keybinding(Keybinding::new("escape".to_string(), ModifierKeys::new())),
                    ]),
            )
    }
}
