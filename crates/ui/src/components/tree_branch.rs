use gpui::{canvas, fill, point, Bounds, Hsla};
use strum::EnumIter;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter)]
/// Represents different segments of a tree branch in a tree-like structure.
pub enum TreeBranchSegment {
    /// The topmost segment of a branch.
    ///
    /// Used for the first segment of any branch, typically connecting to a parent node.
    ///
    /// Example:
    /// ```
    /// Root
    /// │
    /// └─ Child
    /// ```
    Root,

    /// A middle segment that continues both upward and downward.
    ///
    /// Used for segments that connect upper and lower parts of a branch.
    ///
    /// Example:
    /// ```
    /// Parent
    /// │
    /// Continuous
    /// │
    /// └─ Child
    /// ```
    Continuous,

    /// The start of a leaf (includes a horizontal line).
    ///
    /// Used for segments that begin a leaf node, typically with children.
    ///
    /// Example:
    /// ```
    /// Parent
    /// ├─ LeafStart
    /// │  └─ Child
    /// └─ Sibling
    /// ```
    LeafStart,

    /// The end of a leaf (no continuation downward).
    ///
    /// Used for the last segment of a branch, typically for childless nodes.
    ///
    /// Example:
    /// ```
    /// Parent
    /// ├─ Sibling
    /// └─ LeafEnd
    /// ```
    LeafEnd,

    /// The start of a segment that skips a level (no horizontal line).
    ///
    /// Used when a branch needs to skip a level without creating a leaf.
    ///
    /// Example:
    /// ```
    /// Parent
    /// │
    /// SkipStart
    /// │
    /// └─ Grandchild
    /// ```
    SkipStart,

    /// The end of a segment that skips a level (continues downward).
    ///
    /// Used to end a skip and continue the branch downward.
    ///
    /// Example:
    /// ```
    /// Parent
    /// │
    /// SkipStart
    /// │
    /// SkipEnd
    /// └─ Grandchild
    /// ```
    SkipEnd,
}

#[derive(IntoElement)]
pub struct TreeBranch {
    /// The width of the branch.
    width: Pixels,
    /// The height of the branch.
    height: Pixels,
    /// The color of the branch.
    color: Hsla,
    /// The segment of the branch.
    segment: TreeBranchSegment,
    /// Whether the branch should draw beyond it's
    /// set height. This is useful for connecting
    /// branches that have a slight gap between them
    /// due to a margin or padding.
    overdraw: bool,
    /// The length of the overdraw.
    overdraw_length: Pixels,
}

impl TreeBranch {
    pub fn new(segment: TreeBranchSegment, cx: &WindowContext) -> Self {
        let rem_size = cx.rem_size();
        let line_height = cx.text_style().line_height_in_pixels(rem_size);

        let width = line_height * 1.5;
        let height = line_height;
        let color = cx.theme().colors().icon_placeholder;
        let overdraw_length = px(1.);

        let overdraw = matches!(segment, TreeBranchSegment::Continuous | TreeBranchSegment::SkipStart | TreeBranchSegment::SkipEnd);

        Self {
            width,
            height,
            color,
            segment,
            overdraw,
            overdraw_length,
        }
    }

    pub fn width(&mut self, width: Pixels) -> &mut Self {
        self.width = width;
        self
    }

    pub fn height(&mut self, height: Pixels) -> &mut Self {
        self.height = height;
        self
    }

    pub fn overdraw(&mut self, overdraw: bool) -> &mut Self {
        self.overdraw = overdraw;
        self
    }

    pub fn overdraw_length(&mut self, overdraw_length: Pixels) -> &mut Self {
        self.overdraw_length = overdraw_length;
        self
    }
}

impl RenderOnce for TreeBranch {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let thickness = px(1.);

        canvas(
            |_, _| {},
            move |bounds, _, cx| {
                let start_x = (bounds.left() + bounds.right() - thickness) / 2.;
                let start_y = (bounds.top() + bounds.bottom() - thickness) / 2.;
                let right = bounds.right();
                let top = bounds.top();

                // Vertical line
                if !matches!(self.segment, TreeBranchSegment::LeafEnd) {
                    let bottom = if self.overdraw {
                        bounds.bottom() + self.overdraw_length
                    } else {
                        bounds.bottom()
                    };
                    cx.paint_quad(fill(
                        Bounds::from_corners(
                            point(start_x, if matches!(self.segment, TreeBranchSegment::Root) { top } else { bounds.top() }),
                            point(start_x + thickness, bottom),
                        ),
                        self.color,
                    ));
                }

                // Horizontal line
                if matches!(self.segment, TreeBranchSegment::LeafStart | TreeBranchSegment::LeafEnd) {
                    cx.paint_quad(fill(
                        Bounds::from_corners(point(start_x, start_y), point(right, start_y + thickness)),
                        self.color,
                    ));
                }
            },
        )
        .w(self.width)
        .h(self.height)
    }
}

#[cfg(feature = "stories")]
mod stories {
    use gpui::Render;
    use story::{StoryContainer, StoryItem, StorySection};
    use strum::IntoEnumIterator;

    use crate::{prelude::*};

    use super::{TreeBranch, TreeBranchSegment};

    pub struct TreeBranchStory;

    impl Render for TreeBranchStory {
        fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
            StoryContainer::new("TreeBranch", "crates/ui/src/components/tree_branch.rs")
                .child(
                    StorySection::new()
                        .children(
                            // iter over all the segments
                            TreeBranchSegment::iter().map(|segment| {
                                StoryItem::new(format!("{:?}", segment), TreeBranch::new(segment, cx))
                            }),
                        )
                )
        }
    }
}

pub use stories::TreeBranchStory;
