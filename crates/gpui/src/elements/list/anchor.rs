use super::ListOffset;
use crate::Pixels;
use std::ops::Range;

/// Deferred scroll adjustment applied after the scroll-top item has been remeasured.
///
/// An absolute pending scroll preserves the same pixel offset into the item, which keeps
/// visible text stable while content is appended to or removed from that item. A
/// proportional pending scroll preserves the same fractional position within the item,
/// which is useful when the whole list is being resized and each item scales similarly.
#[derive(Clone)]
pub(super) enum PendingScroll {
    /// Preserve the same pixel offset into the item after it is remeasured.
    Absolute { item_ix: usize, offset: Pixels },
    /// Preserve the same fractional offset into the item after it is remeasured.
    Proportional(PendingScrollFraction),
}

/// Keeps track of a fractional scroll position within an item for restoration
/// after remeasurement.
#[derive(Clone)]
pub(super) struct PendingScrollFraction {
    /// The index of the item to scroll within.
    pub(super) item_ix: usize,
    /// Fractional offset (0.0 to 1.0) within the item's height.
    pub(super) fraction: f32,
}

impl PendingScroll {
    pub(super) fn item_ix(&self) -> usize {
        match self {
            PendingScroll::Absolute { item_ix, .. } => *item_ix,
            PendingScroll::Proportional(pending) => pending.item_ix,
        }
    }

    /// Shifts the pending item's index to match a splice before it. The
    /// caller drops a pending whose item was removed or replaced first; this
    /// only adjusts indices at or after the end of the spliced range.
    pub(super) fn reindex_for_splice(&mut self, old_range: &Range<usize>, spliced_count: usize) {
        let item_ix = match self {
            PendingScroll::Absolute { item_ix, .. } => item_ix,
            PendingScroll::Proportional(pending) => &mut pending.item_ix,
        };
        if old_range.end <= *item_ix {
            *item_ix = *item_ix - (old_range.end - old_range.start) + spliced_count;
        }
    }

    fn into_offset(self) -> PendingOffset {
        match self {
            PendingScroll::Absolute { offset, .. } => PendingOffset::Absolute { offset },
            PendingScroll::Proportional(pending) => PendingOffset::Proportional {
                fraction: pending.fraction,
            },
        }
    }
}

/// A deferred scroll adjustment scoped to a provisional anchor.
///
/// Unlike [`PendingScroll`], it carries no item index: it is always an offset
/// into the anchored item, so it cannot outlive or refer to a different item.
#[derive(Clone, Copy)]
pub(super) enum PendingOffset {
    Absolute { offset: Pixels },
    Proportional { fraction: f32 },
}

impl PendingOffset {
    pub(super) fn into_pending_scroll(self, item_ix: usize) -> PendingScroll {
        match self {
            PendingOffset::Absolute { offset } => PendingScroll::Absolute { item_ix, offset },
            PendingOffset::Proportional { fraction } => {
                PendingScroll::Proportional(PendingScrollFraction { item_ix, fraction })
            }
        }
    }
}

/// The lifecycle of the scroll anchor's provisional heights.
///
/// A pin and the deferred offset captured under it are one value: entering
/// provisional mode captures the anchor, and leaving it applies the pending
/// offset (if any) to that same anchor. There is no second index that can
/// disagree with the first, and moving the anchor drops both together.
#[derive(Default, Clone, Copy)]
pub(super) enum AnchorState {
    #[default]
    Free,
    Provisional {
        anchor: ListOffset,
        pending: Option<PendingOffset>,
    },
}

impl AnchorState {
    pub(super) fn is_provisional(&self, anchor: Option<ListOffset>) -> bool {
        matches!(*self, AnchorState::Provisional { anchor: pinned, .. } if anchor == Some(pinned))
    }

    /// Enters provisional mode for `anchor`. Returns a pending scroll that
    /// could not be absorbed because it belongs to a different item, so the
    /// caller can keep it in the non-provisional slot.
    ///
    /// Re-entering at the pinned anchor preserves its stored adjustment unless
    /// superseded by a new `pending` for the same item.
    pub(super) fn begin(
        &mut self,
        anchor: ListOffset,
        pending: Option<PendingScroll>,
    ) -> Option<PendingScroll> {
        let (absorbed, leftover) = match pending {
            Some(pending) if pending.item_ix() == anchor.item_ix => {
                (Some(pending.into_offset()), None)
            }
            pending => (None, pending),
        };

        let pending = match *self {
            AnchorState::Provisional {
                anchor: pinned,
                pending: existing,
            } if pinned == anchor => absorbed.or(existing),
            _ => absorbed,
        };

        *self = AnchorState::Provisional { anchor, pending };
        leftover
    }

    /// Leaves provisional mode, returning the anchor and its deferred offset
    /// for the caller to apply on the next layout.
    pub(super) fn end(&mut self) -> Option<(ListOffset, Option<PendingOffset>)> {
        match std::mem::replace(self, AnchorState::Free) {
            AnchorState::Free => None,
            AnchorState::Provisional { anchor, pending } => Some((anchor, pending)),
        }
    }

    /// Stores the adjustment under the pin. Returns `false` when free, so the
    /// caller can fall back to the non-provisional pending slot.
    pub(super) fn stash(&mut self, pending: PendingOffset) -> bool {
        if let AnchorState::Provisional { pending: slot, .. } = self {
            *slot = Some(pending);
            true
        } else {
            false
        }
    }

    pub(super) fn invalidate_for_splice(&mut self, old_range: &Range<usize>) {
        if let AnchorState::Provisional { anchor, .. } = self {
            if old_range.start <= anchor.item_ix {
                *self = AnchorState::Free;
            }
        }
    }

    /// Drops the pin and its pending offset together once the anchor moves.
    pub(super) fn move_anchor(&mut self, anchor: Option<ListOffset>) {
        if !self.is_provisional(anchor) {
            *self = AnchorState::Free;
        }
    }
}

/// Determines how remeasurement preserves the scroll position when the scroll-top item
/// changes height.
pub(super) enum ScrollAnchor {
    /// Preserve the same pixel offset into the scroll-top item.
    Absolute,
    /// Preserve the same fractional position within the scroll-top item.
    Proportional,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::px;

    fn offset(item_ix: usize, offset_in_item: f32) -> ListOffset {
        ListOffset {
            item_ix,
            offset_in_item: px(offset_in_item),
        }
    }

    #[test]
    fn begin_captures_the_anchor_and_absorbs_a_matching_pending() {
        let mut state = AnchorState::Free;
        let anchor = offset(5, 40.);

        let leftover = state.begin(
            anchor,
            Some(PendingScroll::Absolute {
                item_ix: 5,
                offset: px(40.),
            }),
        );

        assert!(leftover.is_none());
        assert!(state.is_provisional(Some(anchor)));
        match state {
            AnchorState::Provisional {
                pending: Some(PendingOffset::Absolute { offset: pending }),
                ..
            } => assert_eq!(pending, px(40.)),
            _ => panic!("expected an absorbed absolute pending"),
        }
    }

    #[test]
    fn begin_again_at_the_same_anchor_keeps_the_stored_pending() {
        let mut state = AnchorState::Free;
        let anchor = offset(5, 50.);
        state.begin(anchor, None);
        assert!(state.stash(PendingOffset::Proportional { fraction: 0.5 }));

        assert!(state.begin(anchor, None).is_none());
        assert!(matches!(
            state,
            AnchorState::Provisional {
                pending: Some(PendingOffset::Proportional { fraction }),
                ..
            } if fraction == 0.5
        ));

        state.begin(
            anchor,
            Some(PendingScroll::Absolute {
                item_ix: 5,
                offset: px(20.),
            }),
        );
        assert!(matches!(
            state,
            AnchorState::Provisional {
                pending: Some(PendingOffset::Absolute { offset }),
                ..
            } if offset == px(20.)
        ));
    }

    #[test]
    fn begin_at_a_new_anchor_drops_the_stored_pending() {
        let mut state = AnchorState::Free;
        state.begin(offset(5, 50.), None);
        state.stash(PendingOffset::Proportional { fraction: 0.5 });

        state.begin(offset(6, 0.), None);
        assert!(matches!(
            state,
            AnchorState::Provisional {
                anchor,
                pending: None,
            } if anchor == offset(6, 0.)
        ));
    }

    #[test]
    fn begin_returns_a_pending_for_another_item() {
        let mut state = AnchorState::Free;

        let leftover = state.begin(
            offset(5, 0.),
            Some(PendingScroll::Proportional(PendingScrollFraction {
                item_ix: 4,
                fraction: 0.5,
            })),
        );

        assert!(matches!(
            leftover,
            Some(PendingScroll::Proportional(PendingScrollFraction {
                item_ix: 4,
                ..
            }))
        ));
        assert!(matches!(
            state,
            AnchorState::Provisional { pending: None, .. }
        ));
    }

    #[test]
    fn end_hands_back_the_anchor_and_pending_once() {
        let mut state = AnchorState::Provisional {
            anchor: offset(5, 40.),
            pending: Some(PendingOffset::Absolute { offset: px(40.) }),
        };

        let (anchor, pending) = state.end().expect("provisional state should end");
        assert_eq!(anchor, offset(5, 40.));
        assert!(matches!(
            pending,
            Some(PendingOffset::Absolute { offset: pending }) if pending == px(40.)
        ));
        assert!(matches!(state, AnchorState::Free));
        assert!(state.end().is_none());
    }

    #[test]
    fn stash_only_stores_while_provisional() {
        let mut state = AnchorState::Free;
        assert!(!state.stash(PendingOffset::Absolute { offset: px(5.) }));

        state.begin(offset(5, 0.), None);
        assert!(state.stash(PendingOffset::Absolute { offset: px(5.) }));
        assert!(matches!(
            state,
            AnchorState::Provisional {
                pending: Some(PendingOffset::Absolute { offset }),
                ..
            } if offset == px(5.)
        ));
    }

    #[test]
    fn move_anchor_drops_the_pin_as_a_unit() {
        let mut state = AnchorState::Provisional {
            anchor: offset(5, 40.),
            pending: Some(PendingOffset::Absolute { offset: px(40.) }),
        };

        state.move_anchor(Some(offset(5, 40.)));
        assert!(state.is_provisional(Some(offset(5, 40.))));

        state.move_anchor(Some(offset(6, 0.)));
        assert!(matches!(state, AnchorState::Free));
    }

    #[test]
    fn splice_at_or_before_the_anchor_drops_the_pin() {
        let pinned = || AnchorState::Provisional {
            anchor: offset(5, 40.),
            pending: Some(PendingOffset::Absolute { offset: px(40.) }),
        };

        let mut state = pinned();
        state.invalidate_for_splice(&(6..7));
        assert!(state.is_provisional(Some(offset(5, 40.))));

        state.invalidate_for_splice(&(5..6));
        assert!(matches!(state, AnchorState::Free));
    }

    #[test]
    fn is_provisional_requires_the_anchor_to_match() {
        let mut state = AnchorState::Free;
        assert!(!state.is_provisional(Some(offset(5, 0.))));

        state.begin(offset(5, 0.), None);
        assert!(state.is_provisional(Some(offset(5, 0.))));
        assert!(!state.is_provisional(Some(offset(6, 0.))));
        assert!(!state.is_provisional(None));
    }
}
