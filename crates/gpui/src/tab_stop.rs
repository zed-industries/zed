use std::sync::Arc;

use crate::{FocusHandle, FocusId, GlobalElementId};

/// Represents a collection of tab handles.
///
/// Used to manage the `Tab` event to switch between focus handles.
#[derive(Default)]
pub(crate) struct TabHandles {
    pub(crate) handles: Vec<FocusHandle>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct FocusTrapId(pub(crate) Arc<GlobalElementId>);

impl TabHandles {
    pub(crate) fn insert(&mut self, focus_handle: FocusHandle) {
        if !focus_handle.tab_stop {
            return;
        }

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

    pub(crate) fn with_focus_trap(&self, focused_id: Option<&FocusId>) -> Vec<FocusHandle> {
        if let Some(focused_id) = focused_id {
            if let Some(handle) = self.handles.iter().find(|h| &h.id == focused_id) {
                return self
                    .handles
                    .iter()
                    .filter(|h| h.focus_trap == handle.focus_trap)
                    .cloned()
                    .collect();
            }
        }

        self.handles
            .iter()
            .filter(|h| h.focus_trap.is_none())
            .cloned()
            .collect()
    }

    pub(crate) fn next(&self, focused_id: Option<&FocusId>) -> Option<FocusHandle> {
        let group_handles = self.with_focus_trap(focused_id);
        let next_ix = group_handles
            .iter()
            .position(|h| Some(&h.id) == focused_id)
            .and_then(|ix| {
                let next_ix = ix + 1;
                (next_ix < group_handles.len()).then_some(next_ix)
            })
            .unwrap_or_default();

        group_handles.get(next_ix).cloned()
    }

    pub(crate) fn prev(&self, focused_id: Option<&FocusId>) -> Option<FocusHandle> {
        let group_handles = self.with_focus_trap(focused_id);
        let ix = group_handles
            .iter()
            .position(|h| Some(&h.id) == focused_id)
            .unwrap_or_default();

        let prev_ix = if ix == 0 {
            group_handles.len().saturating_sub(1)
        } else {
            ix.saturating_sub(1)
        };

        group_handles.get(prev_ix).cloned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{FocusHandle, FocusMap, FocusTrapId, GlobalElementId, TabHandles};
    use std::sync::Arc;

    #[test]
    fn test_tab_handles() {
        let focus_map = Arc::new(FocusMap::default());
        let mut tab = TabHandles::default();

        let trap_id = FocusTrapId(Arc::new(GlobalElementId(smallvec::smallvec![
            "trap1".into()
        ])));

        let focus_handles = vec![
            FocusHandle::new(&focus_map).tab_stop(true).tab_index(0),
            FocusHandle::new(&focus_map).tab_stop(true).tab_index(1),
            FocusHandle::new(&focus_map).tab_stop(true).tab_index(1),
            FocusHandle::new(&focus_map),
            FocusHandle::new(&focus_map).tab_index(2),
            FocusHandle::new(&focus_map)
                .tab_stop(true)
                .tab_index(0)
                .focus_trap(&trap_id),
            FocusHandle::new(&focus_map)
                .tab_stop(true)
                .tab_index(0)
                .focus_trap(&trap_id),
            FocusHandle::new(&focus_map)
                .tab_stop(true)
                .tab_index(0)
                .focus_trap(&trap_id),
            FocusHandle::new(&focus_map).tab_stop(true).tab_index(0),
            FocusHandle::new(&focus_map).tab_stop(true).tab_index(2),
        ];

        for handle in focus_handles.iter() {
            tab.insert(handle.clone());
        }
        assert_eq!(
            tab.handles
                .iter()
                .map(|handle| handle.id)
                .collect::<Vec<_>>(),
            vec![
                // ix 0
                focus_handles[0].id,
                // ix 1, group1
                focus_handles[5].id,
                // ix 2, group1
                focus_handles[6].id,
                // ix 3, group1
                focus_handles[7].id,
                // ix 4
                focus_handles[8].id,
                // ix 5
                focus_handles[1].id,
                // ix 6
                focus_handles[2].id,
                // ix 7
                focus_handles[9].id,
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
            Some(tab.handles[4].clone())
        );
        assert_eq!(
            tab.next(Some(&tab.handles[4].id)),
            Some(tab.handles[5].clone())
        );
        assert_eq!(
            tab.next(Some(&tab.handles[5].id)),
            Some(tab.handles[6].clone())
        );
        assert_eq!(
            tab.next(Some(&tab.handles[6].id)),
            Some(tab.handles[7].clone())
        );
        assert_eq!(
            tab.next(Some(&tab.handles[7].id)),
            Some(tab.handles[0].clone())
        );

        // prev
        assert_eq!(tab.prev(None), Some(tab.handles[7].clone()));
        assert_eq!(
            tab.prev(Some(&tab.handles[0].id)),
            Some(tab.handles[7].clone())
        );
        assert_eq!(
            tab.prev(Some(&tab.handles[4].id)),
            Some(tab.handles[0].clone())
        );
        assert_eq!(
            tab.prev(Some(&tab.handles[5].id)),
            Some(tab.handles[4].clone())
        );
        assert_eq!(
            tab.prev(Some(&tab.handles[6].id)),
            Some(tab.handles[5].clone())
        );
        assert_eq!(
            tab.prev(Some(&tab.handles[7].id)),
            Some(tab.handles[6].clone())
        );

        // next in group1
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
            Some(tab.handles[1].clone())
        );

        // prev in group1
        assert_eq!(
            tab.prev(Some(&tab.handles[1].id)),
            Some(tab.handles[3].clone())
        );
        assert_eq!(
            tab.prev(Some(&tab.handles[2].id)),
            Some(tab.handles[1].clone())
        );
        assert_eq!(
            tab.prev(Some(&tab.handles[3].id)),
            Some(tab.handles[2].clone())
        );
    }
}
