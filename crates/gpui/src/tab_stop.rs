use crate::{FocusHandle, FocusId};

/// Represents a collection of tab handles.
///
/// Used to manage the `Tab` event to switch between focus handles.
#[derive(Default)]
pub(crate) struct TabHandles {
    pub(crate) handles: Vec<FocusHandle>,
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

    fn current_index(&self, focused_id: Option<&FocusId>) -> Option<usize> {
        self.handles.iter().position(|h| Some(&h.id) == focused_id)
    }

    pub(crate) fn next(&self, focused_id: Option<&FocusId>) -> Option<FocusHandle> {
        let next_ix = self
            .current_index(focused_id)
            .and_then(|ix| {
                let next_ix = ix + 1;
                (next_ix < self.handles.len()).then_some(next_ix)
            })
            .unwrap_or_default();

        self.handles.get(next_ix).cloned()
    }

    pub(crate) fn prev(&self, focused_id: Option<&FocusId>) -> Option<FocusHandle> {
        let ix = self.current_index(focused_id).unwrap_or_default();
        let prev_ix = if ix == 0 {
            self.handles.len().saturating_sub(1)
        } else {
            ix.saturating_sub(1)
        };

        self.handles.get(prev_ix).cloned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{FocusHandle, FocusMap, TabHandles};
    use std::sync::Arc;

    #[test]
    fn test_tab_handles() {
        let focus_map = Arc::new(FocusMap::default());
        let mut tab = TabHandles::default();

        let focus_handles = vec![
            FocusHandle::new(&focus_map).tab_stop(true).tab_index(0),
            FocusHandle::new(&focus_map).tab_stop(true).tab_index(1),
            FocusHandle::new(&focus_map).tab_stop(true).tab_index(1),
            FocusHandle::new(&focus_map),
            FocusHandle::new(&focus_map).tab_index(2),
            FocusHandle::new(&focus_map).tab_stop(true).tab_index(0),
            FocusHandle::new(&focus_map).tab_stop(true).tab_index(2),
        ];

        for handle in focus_handles.iter() {
            tab.insert(handle);
        }
        assert_eq!(
            tab.handles
                .iter()
                .map(|handle| handle.id)
                .collect::<Vec<_>>(),
            vec![
                focus_handles[0].id,
                focus_handles[5].id,
                focus_handles[1].id,
                focus_handles[2].id,
                focus_handles[6].id,
            ]
        );

        // Select first tab index if no handle is currently focused.
        assert_eq!(tab.next(None), Some(tab.handles[0].clone()));
        // Select last tab index if no handle is currently focused.
        assert_eq!(
            tab.prev(None),
            Some(tab.handles[tab.handles.len() - 1].clone())
        );

        assert_eq!(
            tab.next(Some(&tab.handles[0].id)),
            Some(tab.handles[1].clone())
        );
        assert_eq!(
            tab.next(Some(&tab.handles[1].id)),
            Some(tab.handles[2].clone())
        );
        assert_eq!(
            tab.next(Some(&tab.handles[2].id)),
            Some(tab.handles[3].clone())
        );
        assert_eq!(
            tab.next(Some(&tab.handles[3].id)),
            Some(tab.handles[4].clone())
        );
        assert_eq!(
            tab.next(Some(&tab.handles[4].id)),
            Some(tab.handles[0].clone())
        );

        // prev
        assert_eq!(tab.prev(None), Some(tab.handles[4].clone()));
        assert_eq!(
            tab.prev(Some(&tab.handles[0].id)),
            Some(tab.handles[4].clone())
        );
        assert_eq!(
            tab.prev(Some(&tab.handles[1].id)),
            Some(tab.handles[0].clone())
        );
        assert_eq!(
            tab.prev(Some(&tab.handles[2].id)),
            Some(tab.handles[1].clone())
        );
        assert_eq!(
            tab.prev(Some(&tab.handles[3].id)),
            Some(tab.handles[2].clone())
        );
        assert_eq!(
            tab.prev(Some(&tab.handles[4].id)),
            Some(tab.handles[3].clone())
        );
    }
}
