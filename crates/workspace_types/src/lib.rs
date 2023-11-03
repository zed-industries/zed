use std::any::Any;

use gpui::{
    actions,
    geometry::{rect::RectF, vector::Vector2F},
    Axis, WindowContext,
};

use serde::Deserialize;

actions!(
    workspace,
    [
        Open,
        NewFile,
        NewWindow,
        CloseWindow,
        CloseInactiveTabsAndPanes,
        AddFolderToProject,
        Unfollow,
        SaveAs,
        ReloadActiveItem,
        ActivatePreviousPane,
        ActivateNextPane,
        FollowNextCollaborator,
        NewTerminal,
        NewCenterTerminal,
        ToggleTerminalFocus,
        NewSearch,
        Feedback,
        Restart,
        Welcome,
        ToggleZoom,
        ToggleLeftDock,
        ToggleRightDock,
        ToggleBottomDock,
        CloseAllDocks,
    ]
);
pub type WorkspaceId = i64;

pub type ItemId = usize;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum SplitDirection {
    Up,
    Down,
    Left,
    Right,
}

impl SplitDirection {
    pub fn all() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }

    pub fn edge(&self, rect: RectF) -> f32 {
        match self {
            Self::Up => rect.min_y(),
            Self::Down => rect.max_y(),
            Self::Left => rect.min_x(),
            Self::Right => rect.max_x(),
        }
    }

    // Returns a new rectangle which shares an edge in SplitDirection and has `size` along SplitDirection
    pub fn along_edge(&self, rect: RectF, size: f32) -> RectF {
        match self {
            Self::Up => RectF::new(rect.origin(), Vector2F::new(rect.width(), size)),
            Self::Down => RectF::new(
                rect.lower_left() - Vector2F::new(0., size),
                Vector2F::new(rect.width(), size),
            ),
            Self::Left => RectF::new(rect.origin(), Vector2F::new(size, rect.height())),
            Self::Right => RectF::new(
                rect.upper_right() - Vector2F::new(size, 0.),
                Vector2F::new(size, rect.height()),
            ),
        }
    }

    pub fn axis(&self) -> Axis {
        match self {
            Self::Up | Self::Down => Axis::Vertical,
            Self::Left | Self::Right => Axis::Horizontal,
        }
    }

    pub fn increasing(&self) -> bool {
        match self {
            Self::Left | Self::Up => false,
            Self::Down | Self::Right => true,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ViewId {
    pub creator: rpc::proto::PeerId,
    pub id: u64,
}
pub trait NavigationHistory {
    fn push_any(&mut self, data: Option<Box<dyn Any>>, cx: &mut WindowContext);
    fn pop_forward(&mut self, cx: &mut WindowContext) -> Option<NavigationEntry>;
    fn pop_backward(&mut self, cx: &mut WindowContext) -> Option<NavigationEntry>;
}
impl dyn NavigationHistory {
    fn push<D: 'static + Any>(&mut self, data: Option<D>, cx: &mut WindowContext) {
        self.push_any(data.map(|data| Box::new(data) as _), cx)
    }
}
