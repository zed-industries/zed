use crate::{FormatOptions, Indent};

pub(crate) struct Indentation {
    options: FormatOptions,
    indent_types: Vec<IndentType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IndentType {
    TopLevel,
    BlockLevel,
}

impl Indentation {
    pub fn new(options: FormatOptions) -> Self {
        Indentation {
            options,
            indent_types: Vec::new(),
        }
    }

    pub fn get_indent(&self) -> String {
        match self.options.indent {
            Indent::Spaces(num_spaces) => " "
                .repeat(num_spaces as usize)
                .repeat(self.indent_types.len()),
            Indent::Tabs => "\t".repeat(self.indent_types.len()),
        }
    }

    pub fn increase_top_level(&mut self) {
        self.indent_types.push(IndentType::TopLevel);
    }

    pub fn increase_block_level(&mut self) {
        self.indent_types.push(IndentType::BlockLevel);
    }

    pub fn decrease_top_level(&mut self) {
        if self.indent_types.last() == Some(&IndentType::TopLevel) {
            self.indent_types.pop();
        }
    }

    pub fn decrease_block_level(&mut self) {
        while !self.indent_types.is_empty() {
            let kind = self.indent_types.pop();
            if kind != Some(IndentType::TopLevel) {
                break;
            }
        }
    }

    pub fn reset_indentation(&mut self) {
        self.indent_types.clear();
    }
}
