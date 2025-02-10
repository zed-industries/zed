use gpui::View;
use picker::{Picker, PickerDelegate};

struct UnifiedPalette {
    picker: View<Picker<UnifiedPaletteDelegate>>,
}

// (Symbol in VSCode) (crate name in Zed)
enum PaletteMode {
    // default (file_finder crate)
    FileFinder,
    // > (command_palette)
    Commands,
    // @ (outline)
    // BufferSymbols,
    // # (project_symbols)
    // ProjectSymbols,
    // : (go_to_line)
    // GoToLine,
}

struct UnifiedPaletteDelegate {
    entries: Vec<()>,
}

impl UnifiedPaletteDelegate {
    fn switch_mode(&mut self) {
        if query.starts_with(">") {
            //
            let commands = Commands::new();
            //

            self.mode = PalletteMode(commands);
        }
    }
}
// Open unified palette
// Load file finder mode
// >When you type ">"
// Drop that mode, open command mode,

impl PickerDelegate for UnifiedPaletteDelegate {
    type ListItem = ();

    fn match_count(&self) -> usize {
        match self.mode {
            PaletteMode::Commands(commands) => commands.match_count(),
        }
    }

    fn selected_index(&self) -> usize {
        todo!()
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ui::ViewContext<Picker<Self>>) {
        todo!()
    }

    fn placeholder_text(&self, _cx: &mut ui::WindowContext) -> std::sync::Arc<str> {
        todo!()
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ui::ViewContext<Picker<Self>>,
    ) -> gpui::Task<()> {
        let query = self.mode.trim_starting_char(query);
        match self.mode {
            PaletteMode::Commands(commands) => commands.query(query),
        }
    }

    fn confirm(&mut self, secondary: bool, cx: &mut ui::ViewContext<Picker<Self>>) {
        todo!()
    }

    fn dismissed(&mut self, cx: &mut ui::ViewContext<Picker<Self>>) {
        todo!()
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ui::ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        todo!()
    }
}
