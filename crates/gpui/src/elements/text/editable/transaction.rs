use std::{fmt::Debug, ops::Range};

use crate::SharedString;

use super::selection::Selection;

pub trait Transaction: Debug {
    fn apply(&mut self, value: &mut String, selection: &mut Selection);
    fn revert(&mut self, value: &mut String, selection: &mut Selection);
}

#[derive(Debug)]
pub struct ReplaceTextInRange {
    text: SharedString,
    range: Option<Range<usize>>,
    undo: Option<Box<dyn Transaction>>,
}

impl ReplaceTextInRange {
    pub fn new(text: SharedString, range: Option<Range<usize>>) -> Self {
        Self {
            text,
            range,
            undo: None,
        }
    }
}

impl Transaction for ReplaceTextInRange {
    fn apply(&mut self, value: &mut String, selection: &mut Selection) {
        let old_selection = selection.clone();

        let old_selected_text = value[old_selection.span.clone()].to_owned();

        value.replace_range(old_selection.span.clone(), &self.text);

        let next_index = old_selection.span.start + self.text.len();

        selection.span = next_index..next_index;

        self.undo = Some(Box::new(ReplaceTextInRangeAndSelect {
            range: Some(old_selection.span.start..old_selection.span.start + self.text.len()),
            text: old_selected_text.into(),
            new_selection: old_selection,
            undo: None,
        }));
    }

    fn revert(&mut self, value: &mut String, selection: &mut Selection) {
        self.undo
            .as_mut()
            .expect("has not been applied")
            .apply(value, selection);
    }
}

#[derive(Debug)]
pub struct ReplaceTextInRangeAndSelect {
    text: SharedString,
    range: Option<Range<usize>>,
    new_selection: Selection,
    undo: Option<Box<dyn Transaction>>,
}

impl ReplaceTextInRangeAndSelect {
    pub fn new(text: SharedString, range: Option<Range<usize>>, new_selection: Selection) -> Self {
        Self {
            text,
            range,
            new_selection,
            undo: None,
        }
    }
}

impl Transaction for ReplaceTextInRangeAndSelect {
    fn apply(&mut self, value: &mut String, selection: &mut Selection) {
        let (range, select_in_undo) = if let Some(range) = &self.range {
            (range, false)
        } else {
            (&selection.span, true)
        };

        let old_text_in_range = value[range.clone()].to_owned();

        value.replace_range(range.clone(), &self.text);

        let old_selection = selection.clone();

        *selection = self.new_selection.clone();

        self.undo = Some(if select_in_undo {
            Box::new(ReplaceTextInRangeAndSelect {
                range: None,
                text: old_text_in_range.into(),
                new_selection: old_selection,
                undo: None,
            })
        } else {
            Box::new(ReplaceTextInRange {
                range: None,
                text: old_text_in_range.into(),
                undo: None,
            })
        });
    }

    fn revert(&mut self, value: &mut String, selection: &mut Selection) {
        self.undo
            .as_mut()
            .expect("has not been applied")
            .apply(value, selection);
    }
}

#[derive(Default)]
pub struct History {
    undo_stack: Vec<Box<dyn Transaction>>,
    redo_stack: Vec<Box<dyn Transaction>>,
}

impl History {
    pub fn apply(
        &mut self,
        mut tx: impl Transaction + 'static,
        value: &mut String,
        selection: &mut Selection,
    ) {
        tx.apply(value, selection);

        self.undo_stack.push(Box::new(tx));

        if !self.redo_stack.is_empty() {
            self.redo_stack.clear();
        }
    }

    pub fn undo(&mut self, value: &mut String, selection: &mut Selection) {
        if let Some(mut tx) = self.undo_stack.pop() {
            tx.revert(value, selection);
            self.redo_stack.push(tx);
        }
    }

    pub fn redo(&mut self, value: &mut String, selection: &mut Selection) {
        if let Some(mut tx) = self.redo_stack.pop() {
            tx.apply(value, selection);
            self.undo_stack.push(tx);
        }
    }
}
