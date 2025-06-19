use crate::{FocusHandle, FocusId};

/// Represents a collection of tab handles.
///
/// Used to manage the `Tab` event to switch between focus handles.
#[derive(Default)]
pub(crate) struct TabHandles {
    handles: Vec<FocusHandle>,
}

impl TabHandles {
    pub(crate) fn insert(&mut self, focus_handle: &FocusHandle) {
        if !focus_handle.tab_stop {
            return;
        }

        let focus_handle = focus_handle.clone();

        // Insert handle with same tab_index last
        if let Some(ix) = self
            .handles
            .iter()
            .position(|tab| tab.tab_index > focus_handle.tab_index)
        {
            self.handles.insert(ix, focus_handle);
        } else {
            self.handles.push(focus_handle);
        }
    }

    pub(crate) fn clear(&mut self) {
        self.handles.clear();
    }

    fn current_index(&self, focused_id: Option<&FocusId>) -> usize {
        self.handles
            .iter()
            .position(|h| Some(&h.id) == focused_id)
            .unwrap_or_default()
    }

    pub(crate) fn next(&mut self, focused_id: Option<&FocusId>) -> Option<FocusHandle> {
        let ix = self.current_index(focused_id);

        let mut next_ix = ix + 1;
        if next_ix + 1 > self.handles.len() {
            next_ix = 0;
        }

        if let Some(next_handle) = self.handles.get(next_ix) {
            Some(next_handle.clone())
        } else {
            None
        }
    }

    pub(crate) fn previous(&mut self, focused_id: Option<&FocusId>) -> Option<FocusHandle> {
        let ix = self.current_index(focused_id);
        let prev_ix;
        if ix == 0 {
            prev_ix = self.handles.len().saturating_sub(1);
        } else {
            prev_ix = ix.saturating_sub(1);
        }

        if let Some(prev_handle) = self.handles.get(prev_ix) {
            Some(prev_handle.clone())
        } else {
            None
        }
    }
}
