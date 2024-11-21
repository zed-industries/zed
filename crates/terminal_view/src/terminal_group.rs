/// A group of terminals that can be arranged horizontally or vertically.
/// A pane may have multiple terminal groups, each created on `workspace::NewTerminal`.
/// TODO(dennis): I am adopting VSCode's TerminalGroup idea.
pub struct TerminalGroup {
    terminals: Vec<View<TerminalView>>,
    split_direction: SplitDirection,
    active_terminal_index: usize,
}

/// TODO(dennis): Can we replace this with an existing enum from the pane model?
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

impl TerminalGroup {
    pub fn new() -> Self {
        Self {
            terminals: Vec::new(),
            split_direction: SplitDirection::Horizontal,
            active_terminal_index: 0,
        }
    }

    pub fn split_terminal(&mut self, cx: &mut ViewContext<Self>) {
        let terminal = self.terminals[self.active_terminal_index].clone();
        let terminal_view = terminal.read(cx);
        let new_terminal = terminal_view.split_terminal(cx);
        self.terminals.push(new_terminal);
    }

    pub fn set_split_direction(&mut self, split_direction: SplitDirection) {
        self.split_direction = split_direction;
    }

    pub fn set_active_terminal(&mut self, index: usize) {
        self.active_terminal_index = index;
    }

    pub fn close_terminal(&mut self, cx: &mut ViewContext<Self>, index: usize) {
        self.terminals.remove(index);
    }
}
