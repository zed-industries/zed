use crate::{
    AnyView, Bounds, ContentMask, CursorStyleRequest, EntityId, GlobalElementId, Hitbox,
    PaintIndex, Pixels, PrepaintStateIndex, Scene, TextStyle, TooltipRequest,
};
use collections::FxHashSet;
use std::{ops::Range, rc::Rc};

#[derive(Clone, PartialEq)]
pub(crate) struct ViewNodeCacheKey {
    pub(crate) bounds: Bounds<Pixels>,
    pub(crate) content_mask: ContentMask<Pixels>,
    pub(crate) text_style: TextStyle,
}

pub(crate) struct ViewNodeRecording {
    pub(crate) scene: Rc<Scene>,
    pub(crate) hitboxes: Rc<[Hitbox]>,
    pub(crate) tooltip_requests: Rc<[Option<TooltipRequest>]>,
    pub(crate) cursor_styles: Rc<[CursorStyleRequest]>,
    pub(crate) prepaint_range: Range<PrepaintStateIndex>,
    pub(crate) paint_range: Range<PaintIndex>,
}

pub(crate) struct ViewNode {
    pub(crate) occurrence: GlobalElementId,
    pub(crate) parent: Option<super::node_engine::ViewNodeId>,
    pub(crate) children: Vec<super::node_engine::ViewNodeId>,
    pub(crate) next_children: Vec<super::node_engine::ViewNodeId>,
    pub(crate) view: AnyView,
    pub(crate) cache_key: ViewNodeCacheKey,
    pub(crate) previous_bounds: Bounds<Pixels>,
    pub(crate) accessed_entities: FxHashSet<EntityId>,
    pub(crate) recording: Option<Rc<ViewNodeRecording>>,
}
