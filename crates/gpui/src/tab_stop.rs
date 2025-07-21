use std::sync::Arc;

use crate::{FocusHandle, FocusId, GlobalElementId};

/// Represents a collection of tab handles.
///
/// Used to manage the `Tab` event to switch between focus handles.
#[derive(Default)]
pub(crate) struct TabHandles {
    handles: Vec<FocusHandle>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct TabGroupId(pub(crate) Arc<GlobalElementId>);

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

    pub(crate) fn with_group<'a>(
        &'a self,
        focused_id: Option<&FocusId>,
    ) -> Box<dyn Iterator<Item = &'a FocusHandle> + 'a> {
        if let Some(focused_id) = focused_id {
            if let Some(handle) = self.handles.iter().find(|h| &h.id == focused_id) {
                return Box::new(
                    self.handles
                        .iter()
                        .filter(|h| h.tab_group == handle.tab_group),
                );
            }
        }

        Box::new(self.handles.iter().filter(|h| h.tab_group.is_none()))
    }

    pub(crate) fn next(&self, focused_id: Option<&FocusId>) -> Option<FocusHandle> {
        let group_handles: Vec<&FocusHandle> = self.with_group(focused_id).collect();
        let ix = group_handles
            .iter()
            .position(|h| Some(&h.id) == focused_id)
            .unwrap_or_default();

        let mut next_ix = ix + 1;
        if next_ix + 1 > group_handles.len() {
            next_ix = 0;
        }

        if let Some(next_handle) = group_handles.get(next_ix).cloned() {
            Some(next_handle.clone())
        } else {
            None
        }
    }

    pub(crate) fn prev(&self, focused_id: Option<&FocusId>) -> Option<FocusHandle> {
        let group_handles: Vec<&FocusHandle> = self.with_group(focused_id).collect();
        let ix = group_handles
            .iter()
            .position(|h| Some(&h.id) == focused_id)
            .unwrap_or_default();

        let prev_ix;
        if ix == 0 {
            prev_ix = group_handles.len().saturating_sub(1);
        } else {
            prev_ix = ix.saturating_sub(1);
        }

        if let Some(prev_handle) = group_handles.get(prev_ix).cloned() {
            Some(prev_handle.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FocusHandle, FocusMap, GlobalElementId, TabGroupId, TabHandles};
    use std::sync::Arc;

    #[test]
    fn test_tab_handles() {
        let focus_map = Arc::new(FocusMap::default());
        let mut tab = TabHandles::default();

        let group_id = TabGroupId(Arc::new(GlobalElementId(smallvec::smallvec![
            "group1".into()
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
                .tab_group(&group_id),
            FocusHandle::new(&focus_map)
                .tab_stop(true)
                .tab_index(0)
                .tab_group(&group_id),
            FocusHandle::new(&focus_map)
                .tab_stop(true)
                .tab_index(0)
                .tab_group(&group_id),
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

        // next
        assert_eq!(tab.next(None), Some(tab.handles[4].clone()));
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
