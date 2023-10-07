use super::{
    AbsoluteLength, Bounds, DefiniteLength, Edges, Length, Pixels, Point, Result, Size, Style,
};
use collections::HashMap;
use std::fmt::Debug;
use taffy::{
    geometry::Size as TaffySize,
    style::AvailableSpace as TaffyAvailableSpace,
    tree::{Measurable, MeasureFunc, NodeId},
    Taffy,
};

pub struct TaffyLayoutEngine {
    taffy: Taffy,
    children_to_parents: HashMap<LayoutId, LayoutId>,
    absolute_layout_bounds: HashMap<LayoutId, Bounds<Pixels>>,
}

impl TaffyLayoutEngine {
    pub fn new() -> Self {
        TaffyLayoutEngine {
            taffy: Taffy::new(),
            children_to_parents: HashMap::default(),
            absolute_layout_bounds: HashMap::default(),
        }
    }

    pub fn request_layout(
        &mut self,
        style: Style,
        rem_size: Pixels,
        children: &[LayoutId],
    ) -> Result<LayoutId> {
        let style = style.to_taffy(rem_size);
        if children.is_empty() {
            Ok(self.taffy.new_leaf(style)?.into())
        } else {
            let parent_id = self
                .taffy
                // This is safe because LayoutId is repr(transparent) to taffy::tree::NodeId.
                .new_with_children(style, unsafe { std::mem::transmute(children) })?
                .into();
            for child_id in children {
                self.children_to_parents.insert(*child_id, parent_id);
            }
            Ok(parent_id)
        }
    }

    pub fn request_measured_layout(
        &mut self,
        style: Style,
        rem_size: Pixels,
        measure: impl Fn(Size<Option<Pixels>>, Size<AvailableSpace>) -> Size<Pixels>
            + Send
            + Sync
            + 'static,
    ) -> Result<LayoutId> {
        let style = style.to_taffy(rem_size);

        let measurable = Box::new(Measureable(measure)) as Box<dyn Measurable>;
        Ok(self
            .taffy
            .new_leaf_with_measure(style, MeasureFunc::Boxed(measurable))?
            .into())
    }

    fn count_all_children(&self, parent: LayoutId) -> Result<u32> {
        let mut count = 0;

        for child in self.taffy.children(parent.0)? {
            // Count this child.
            count += 1;

            // Count all of this child's children.
            count += self.count_all_children(LayoutId(child))?
        }

        Ok(count)
    }

    fn max_depth(&self, depth: u32, parent: LayoutId) -> Result<u32> {
        println!(
            "{parent:?} at depth {depth} has {} children",
            self.taffy.child_count(parent.0)?
        );

        let mut max_child_depth = 0;

        for child in self.taffy.children(parent.0)? {
            max_child_depth = std::cmp::max(max_child_depth, self.max_depth(0, LayoutId(child))?);
        }

        Ok(depth + 1 + max_child_depth)
    }

    fn get_edges(&self, parent: LayoutId) -> Result<Vec<(LayoutId, LayoutId)>> {
        let mut edges = Vec::new();

        for child in self.taffy.children(parent.0)? {
            edges.push((parent, LayoutId(child)));

            edges.extend(self.get_edges(LayoutId(child))?);
        }

        Ok(edges)
    }

    pub fn compute_layout(
        &mut self,
        id: LayoutId,
        available_space: Size<AvailableSpace>,
    ) -> Result<()> {
        // println!("Laying out {} children", self.count_all_children(id)?);
        // println!("Max layout depth: {}", self.max_depth(0, id)?);

        // Output the edges (branches) of the tree in Mermaid format for visualization.
        // println!("Edges:");
        // for (a, b) in self.get_edges(id)? {
        //     println!("N{} --> N{}", u64::from(a), u64::from(b));
        // }
        // println!("");

        let started_at = std::time::Instant::now();
        self.taffy
            .compute_layout(id.into(), available_space.into())?;
        println!("compute_layout took {:?}", started_at.elapsed());
        Ok(())
    }

    pub fn layout_bounds(&mut self, id: LayoutId) -> Result<Bounds<Pixels>> {
        if let Some(layout) = self.absolute_layout_bounds.get(&id).cloned() {
            return Ok(layout);
        }

        let layout = self.taffy.layout(id.into())?;
        let mut bounds = Bounds {
            origin: layout.location.into(),
            size: layout.size.into(),
        };

        if let Some(parent_id) = self.children_to_parents.get(&id).copied() {
            let parent_bounds = self.layout_bounds(parent_id)?;
            bounds.origin += parent_bounds.origin;
        }
        self.absolute_layout_bounds.insert(id, bounds);

        Ok(bounds)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct LayoutId(NodeId);

impl std::hash::Hash for LayoutId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        u64::from(self.0).hash(state);
    }
}

impl From<NodeId> for LayoutId {
    fn from(node_id: NodeId) -> Self {
        Self(node_id)
    }
}

impl From<LayoutId> for NodeId {
    fn from(layout_id: LayoutId) -> NodeId {
        layout_id.0
    }
}

struct Measureable<F>(F);

impl<F> taffy::tree::Measurable for Measureable<F>
where
    F: Send + Sync + Fn(Size<Option<Pixels>>, Size<AvailableSpace>) -> Size<Pixels>,
{
    fn measure(
        &self,
        known_dimensions: TaffySize<Option<f32>>,
        available_space: TaffySize<TaffyAvailableSpace>,
    ) -> TaffySize<f32> {
        let known_dimensions: Size<Option<f32>> = known_dimensions.into();
        let known_dimensions: Size<Option<Pixels>> = known_dimensions.map(|d| d.map(Into::into));
        let available_space = available_space.into();
        let size = (self.0)(known_dimensions, available_space);
        size.into()
    }
}

trait ToTaffy<Output> {
    fn to_taffy(&self, rem_size: Pixels) -> Output;
}

impl ToTaffy<taffy::style::Style> for Style {
    fn to_taffy(&self, rem_size: Pixels) -> taffy::style::Style {
        taffy::style::Style {
            display: self.display,
            overflow: self.overflow.clone().into(),
            scrollbar_width: self.scrollbar_width,
            position: self.position,
            inset: self.inset.to_taffy(rem_size),
            size: self.size.to_taffy(rem_size),
            min_size: self.min_size.to_taffy(rem_size),
            max_size: self.max_size.to_taffy(rem_size),
            aspect_ratio: self.aspect_ratio,
            margin: self.margin.to_taffy(rem_size),
            padding: self.padding.to_taffy(rem_size),
            border: self.border_widths.to_taffy(rem_size),
            align_items: self.align_items,
            align_self: self.align_self,
            align_content: self.align_content,
            justify_content: self.justify_content,
            gap: self.gap.to_taffy(rem_size),
            flex_direction: self.flex_direction,
            flex_wrap: self.flex_wrap,
            flex_basis: self.flex_basis.to_taffy(rem_size),
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            ..Default::default() // Ignore grid properties for now
        }
    }
}

// impl ToTaffy for Bounds<Length> {
//     type Output = taffy::prelude::Bounds<taffy::prelude::LengthPercentageAuto>;

//     fn to_taffy(
//         &self,
//         rem_size: Pixels,
//     ) -> taffy::prelude::Bounds<taffy::prelude::LengthPercentageAuto> {
//         taffy::prelude::Bounds {
//             origin: self.origin.to_taffy(rem_size),
//             size: self.size.to_taffy(rem_size),
//         }
//     }
// }

impl ToTaffy<taffy::style::LengthPercentageAuto> for Length {
    fn to_taffy(&self, rem_size: Pixels) -> taffy::prelude::LengthPercentageAuto {
        match self {
            Length::Definite(length) => length.to_taffy(rem_size),
            Length::Auto => taffy::prelude::LengthPercentageAuto::Auto,
        }
    }
}

impl ToTaffy<taffy::style::Dimension> for Length {
    fn to_taffy(&self, rem_size: Pixels) -> taffy::prelude::Dimension {
        match self {
            Length::Definite(length) => length.to_taffy(rem_size),
            Length::Auto => taffy::prelude::Dimension::Auto,
        }
    }
}

impl ToTaffy<taffy::style::LengthPercentage> for DefiniteLength {
    fn to_taffy(&self, rem_size: Pixels) -> taffy::style::LengthPercentage {
        match self {
            DefiniteLength::Absolute(length) => match length {
                AbsoluteLength::Pixels(pixels) => {
                    taffy::style::LengthPercentage::Length(pixels.into())
                }
                AbsoluteLength::Rems(rems) => {
                    taffy::style::LengthPercentage::Length((*rems * rem_size).into())
                }
            },
            DefiniteLength::Fraction(fraction) => {
                taffy::style::LengthPercentage::Percent(*fraction)
            }
        }
    }
}

impl ToTaffy<taffy::style::LengthPercentageAuto> for DefiniteLength {
    fn to_taffy(&self, rem_size: Pixels) -> taffy::style::LengthPercentageAuto {
        match self {
            DefiniteLength::Absolute(length) => match length {
                AbsoluteLength::Pixels(pixels) => {
                    taffy::style::LengthPercentageAuto::Length(pixels.into())
                }
                AbsoluteLength::Rems(rems) => {
                    taffy::style::LengthPercentageAuto::Length((*rems * rem_size).into())
                }
            },
            DefiniteLength::Fraction(fraction) => {
                taffy::style::LengthPercentageAuto::Percent(*fraction)
            }
        }
    }
}

impl ToTaffy<taffy::style::Dimension> for DefiniteLength {
    fn to_taffy(&self, rem_size: Pixels) -> taffy::style::Dimension {
        match self {
            DefiniteLength::Absolute(length) => match length {
                AbsoluteLength::Pixels(pixels) => taffy::style::Dimension::Length(pixels.into()),
                AbsoluteLength::Rems(rems) => {
                    taffy::style::Dimension::Length((*rems * rem_size).into())
                }
            },
            DefiniteLength::Fraction(fraction) => taffy::style::Dimension::Percent(*fraction),
        }
    }
}

impl ToTaffy<taffy::style::LengthPercentage> for AbsoluteLength {
    fn to_taffy(&self, rem_size: Pixels) -> taffy::style::LengthPercentage {
        match self {
            AbsoluteLength::Pixels(pixels) => taffy::style::LengthPercentage::Length(pixels.into()),
            AbsoluteLength::Rems(rems) => {
                taffy::style::LengthPercentage::Length((*rems * rem_size).into())
            }
        }
    }
}

impl<T, T2: Clone + Debug> From<taffy::geometry::Point<T>> for Point<T2>
where
    T: Into<T2>,
{
    fn from(point: taffy::geometry::Point<T>) -> Point<T2> {
        Point {
            x: point.x.into(),
            y: point.y.into(),
        }
    }
}

impl<T: Clone + Debug, T2> Into<taffy::geometry::Point<T2>> for Point<T>
where
    T: Into<T2>,
{
    fn into(self) -> taffy::geometry::Point<T2> {
        taffy::geometry::Point {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}

impl<T: ToTaffy<U> + Clone + Debug, U> ToTaffy<taffy::geometry::Size<U>> for Size<T> {
    fn to_taffy(&self, rem_size: Pixels) -> taffy::geometry::Size<U> {
        taffy::geometry::Size {
            width: self.width.to_taffy(rem_size).into(),
            height: self.height.to_taffy(rem_size).into(),
        }
    }
}

impl<T, U> ToTaffy<taffy::geometry::Rect<U>> for Edges<T>
where
    T: ToTaffy<U> + Clone + Debug,
{
    fn to_taffy(&self, rem_size: Pixels) -> taffy::geometry::Rect<U> {
        taffy::geometry::Rect {
            top: self.top.to_taffy(rem_size).into(),
            right: self.right.to_taffy(rem_size).into(),
            bottom: self.bottom.to_taffy(rem_size).into(),
            left: self.left.to_taffy(rem_size).into(),
        }
    }
}

impl<T: Into<U>, U: Clone + Debug> From<TaffySize<T>> for Size<U> {
    fn from(taffy_size: taffy::geometry::Size<T>) -> Self {
        Size {
            width: taffy_size.width.into(),
            height: taffy_size.height.into(),
        }
    }
}

impl<T: Into<U> + Clone + Debug, U> From<Size<T>> for taffy::geometry::Size<U> {
    fn from(size: Size<T>) -> Self {
        taffy::geometry::Size {
            width: size.width.into(),
            height: size.height.into(),
        }
    }
}

// impl From<TaffySize<Option<f32>>> for Size<Option<Pixels>> {
//     fn from(value: TaffySize<Option<f32>>) -> Self {
//         Self {
//             width: value.width.map(Into::into),
//             height: value.height.map(Into::into),
//         }
//     }
// }

#[derive(Copy, Clone, Debug)]
pub enum AvailableSpace {
    /// The amount of space available is the specified number of pixels
    Definite(Pixels),
    /// The amount of space available is indefinite and the node should be laid out under a min-content constraint
    MinContent,
    /// The amount of space available is indefinite and the node should be laid out under a max-content constraint
    MaxContent,
}

impl From<AvailableSpace> for TaffyAvailableSpace {
    fn from(space: AvailableSpace) -> TaffyAvailableSpace {
        match space {
            AvailableSpace::Definite(Pixels(value)) => TaffyAvailableSpace::Definite(value),
            AvailableSpace::MinContent => TaffyAvailableSpace::MinContent,
            AvailableSpace::MaxContent => TaffyAvailableSpace::MaxContent,
        }
    }
}

impl From<TaffyAvailableSpace> for AvailableSpace {
    fn from(space: TaffyAvailableSpace) -> AvailableSpace {
        match space {
            TaffyAvailableSpace::Definite(value) => AvailableSpace::Definite(Pixels(value)),
            TaffyAvailableSpace::MinContent => AvailableSpace::MinContent,
            TaffyAvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }
}

impl From<Pixels> for AvailableSpace {
    fn from(pixels: Pixels) -> Self {
        AvailableSpace::Definite(pixels)
    }
}
