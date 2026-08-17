use std::num::NonZeroUsize;

use crate::query::DepthVariance;

/// Configuration for a minimum depth of matched files in a walk.
///
/// Unlike a maximum depth, a minimum depth cannot be zero, because such a minimum has no effect.
/// To configure a minimum depth or else an unbounded depth, use
/// [`DepthMin::from_min_or_unbounded`].
///
/// [`DepthMin::from_min_or_unbounded`]: crate::walk::DepthMin::from_min_or_unbounded
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DepthMin(pub NonZeroUsize);

impl DepthMin {
    /// Constructs a [`DepthBehavior`] with a minimum depth or, if zero, unbounded.
    ///
    /// # Examples
    ///
    /// The following example places a minimum bound on the depth of a walk.
    ///
    /// ```rust,no_run
    /// use wax::Glob;
    /// use wax::walk::DepthMin;
    ///
    /// for entry in Glob::new("**")
    ///     .unwrap()
    ///     .walk_with_behavior(".", DepthMin::from_min_or_unbounded(1))
    /// {
    ///     let entry = entry.unwrap();
    ///     // ...
    /// }
    /// ```
    ///
    /// [`DepthBehavior`]: crate::walk::DepthBehavior
    pub fn from_min_or_unbounded(min: usize) -> DepthBehavior {
        use DepthBehavior::{Min, Unbounded};

        DepthMin::try_from(min).map(Min).unwrap_or(Unbounded)
    }

    pub(crate) fn min_at_pivot(self, pivot: usize) -> usize {
        self.0.get().saturating_sub(pivot)
    }
}

impl From<NonZeroUsize> for DepthMin {
    fn from(min: NonZeroUsize) -> Self {
        DepthMin(min)
    }
}

impl From<DepthMin> for NonZeroUsize {
    fn from(min: DepthMin) -> Self {
        min.0
    }
}

impl TryFrom<usize> for DepthMin {
    type Error = ();

    fn try_from(min: usize) -> Result<Self, Self::Error> {
        NonZeroUsize::new(min).map(DepthMin).ok_or(())
    }
}

/// Configuration for a maximum depth of a walk.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DepthMax(pub usize);

impl DepthMax {
    pub(crate) fn max_at_pivot(self, pivot: usize) -> usize {
        self.0.saturating_sub(pivot)
    }
}

impl From<usize> for DepthMax {
    fn from(max: usize) -> Self {
        DepthMax(max)
    }
}

impl From<DepthMax> for usize {
    fn from(max: DepthMax) -> Self {
        max.0
    }
}

/// Configuration for minimum and maximum depths of a walk and matched files.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DepthMinMax {
    pub min: NonZeroUsize,
    pub extent: usize,
}

impl DepthMinMax {
    /// Constructs a [`DepthBehavior`] with a maximum depth and, if nonzero, a minimum depth.
    ///
    /// The depths need not be ordered.
    ///
    /// # Examples
    ///
    /// The following example places both a minimum and maximum bound on the depth of a walk.
    ///
    /// ```rust,no_run
    /// use wax::Glob;
    /// use wax::walk::DepthMinMax;
    ///
    /// for entry in Glob::new("**")
    ///     .unwrap()
    ///     .walk_with_behavior(".", DepthMinMax::from_depths_or_max(1, 2))
    /// {
    ///     let entry = entry.unwrap();
    ///     // ...
    /// }
    /// ```
    ///
    /// [`DepthBehavior`]: crate::walk::DepthBehavior
    pub fn from_depths_or_max(p: usize, q: usize) -> DepthBehavior {
        use DepthBehavior::{Max, MinMax};

        let [min, max] = crate::minmax(p, q);
        let extent = max - min;
        NonZeroUsize::new(min)
            .map(|min| DepthMinMax { min, extent })
            .map_or_else(|| Max(DepthMax(max)), MinMax)
    }

    pub(crate) fn min_max_at_pivot(self, pivot: usize) -> (usize, usize) {
        (
            self.min.get().saturating_sub(pivot),
            self.max().get().saturating_sub(pivot),
        )
    }

    pub fn max(&self) -> NonZeroUsize {
        self.min.saturating_add(self.extent)
    }
}

/// Configuration for filtering walks and files by depth.
///
/// Determines the minimum and maximum depths of a walk and files yielded by that walk relative to
/// the [root path segment][`Entry::root_relative_paths`]. A minimum depth only filters files, but
/// a maximum depth also limits the depth of the walk (directories beneath the maximum are not read
/// from the file system).
///
/// See [`WalkBehavior`].
///
/// # Defaults
///
/// The default depth behavior is [`Unbounded`].
///
/// [`Entry::root_relative_paths`]: crate::walk::Entry::root_relative_paths
/// [`Unbounded`]: crate::walk::DepthBehavior::Unbounded
/// [`WalkBehavior`]: crate::walk::WalkBehavior
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DepthBehavior {
    #[default]
    Unbounded,
    Min(DepthMin),
    Max(DepthMax),
    MinMax(DepthMinMax),
}

impl DepthBehavior {
    /// Constructs a bounded `DepthBehavior` from a minimum and/or maximum depth.
    ///
    /// This function provides an ergonomic way to place bounds on the depth of a walk. At least
    /// one closed depth is required. A given depth is closed if `Some` and is open if `None`. Note
    /// that a closed depth need not be explicitly wrapped in `Some`, because the depth parameters
    /// are `impl Into<Option<usize>>`.
    ///
    /// Returns `None` if both the minimum and maximum depths are both open (unbounded) or if both
    /// depths are closed but are misordered (the minimum is greater than the maximum). Never
    /// returns [`Unbounded`].
    ///
    /// # Examples
    ///
    /// The following example places a maximum bound on the depth of a walk by using an open
    /// minimum depth (`None`).
    ///
    /// ```rust,no_run
    /// use wax::Glob;
    /// use wax::walk::DepthBehavior;
    ///
    /// for entry in Glob::new("**")
    ///     .unwrap()
    ///     .walk_with_behavior(".", DepthBehavior::bounded(None, 2).unwrap())
    /// {
    ///     let entry = entry.unwrap();
    ///     // ...
    /// }
    /// ```
    ///
    /// [`Unbounded`]: crate::walk::DepthBehavior::Unbounded
    pub fn bounded(min: impl Into<Option<usize>>, max: impl Into<Option<usize>>) -> Option<Self> {
        use DepthBehavior::{Max, Min, MinMax};

        match (min.into(), max.into()) {
            (Some(min), None) => NonZeroUsize::new(min).map(DepthMin).map(Min),
            (None, Some(max)) => Some(Max(DepthMax(max))),
            (Some(min), Some(max)) if min <= max => NonZeroUsize::new(min)
                .map(|min| DepthMinMax {
                    min,
                    extent: max - min.get(),
                })
                .map(MinMax),
            _ => None,
        }
    }

    pub fn bounded_at_depth_variance(
        min: impl Into<Option<usize>>,
        max: impl Into<Option<usize>>,
        depth: DepthVariance,
    ) -> Option<Self> {
        let lower = match depth {
            DepthVariance::Invariant(depth) => depth,
            DepthVariance::Variant(bounds) => bounds.lower().bounded().map_or(0, usize::from),
        };
        let translation = move |depth: Option<usize>| -> Result<Option<usize>, ()> {
            depth
                .map(|depth| depth.checked_add(lower).ok_or(()))
                .transpose()
        };
        DepthBehavior::bounded(translation(min.into()).ok()?, translation(max.into()).ok()?)
    }
}

impl From<DepthMax> for DepthBehavior {
    fn from(max: DepthMax) -> Self {
        DepthBehavior::Max(max)
    }
}

impl From<DepthMin> for DepthBehavior {
    fn from(min: DepthMin) -> Self {
        DepthBehavior::Min(min)
    }
}

impl From<DepthMinMax> for DepthBehavior {
    fn from(minmax: DepthMinMax) -> Self {
        DepthBehavior::MinMax(minmax)
    }
}

/// Configuration for interpreting symbolic links.
///
/// Determines how symbolic links are interpreted when walking directory trees using functions like
/// [`Glob::walk_with_behavior`].
///
/// # Defaults
///
/// The default link behavior is [`ReadFile`] (links are read as regular files and their targets
/// are ignored).
///
/// [`Glob::walk_with_behavior`]: crate::Glob::walk_with_behavior
/// [`ReadFile`]: crate::walk::LinkBehavior::ReadFile
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LinkBehavior {
    /// Read the symbolic link file itself.
    ///
    /// This behavior reads the symbolic link as a regular file. The corresponding entry uses the
    /// path of the link file and its metadata describes the link file itself. The target is
    /// effectively ignored and traversal does **not** follow the link.
    #[default]
    ReadFile,
    /// Read the target of the symbolic link.
    ///
    /// This behavior reads the target of the symbolic link. The corresponding entry uses the path
    /// of the link file and its metadata describes the target. If the target is a directory, then
    /// traversal follows the link and descend into the target.
    ///
    /// If a link is re-entrant and forms a cycle, then an error will be emitted instead of an
    /// entry and traversal does not follow the link.
    ReadTarget,
}

/// Configuration for walking directory trees.
///
/// Determines the behavior of the traversal within a directory tree when using functions like
/// [`Glob::walk_with_behavior`]. `WalkBehavior` can be constructed via conversions from types
/// representing its fields and sub-fields. APIs generally accept `impl Into<WalkBehavior>`, so
/// these conversion can be used implicitly. When constructed using such a conversion,
/// `WalkBehavior` will use defaults for any remaining fields.
///
/// # Defaults
///
/// By default, walk behavior has [unbounded depth][`DepthBehavior::Unbounded`] and reads links as
/// [regular files][`LinkBehavior::ReadFile`] (ignoring their targets). Fields have the following
/// values:
///
/// | Field     | Description                       | Value                        |
/// |-----------|-----------------------------------|------------------------------|
/// | [`depth`] | Bounds on depth.                  | [`DepthBehavior::Unbounded`] |
/// | [`link`]  | Interpretation of symbolic links. | [`LinkBehavior::ReadFile`]   |
///
/// # Examples
///
/// By default, symbolic links are interpreted as regular files and targets are ignored. To read
/// linked targets, use [`LinkBehavior::ReadTarget`].
///
/// ```rust,no_run
/// use wax::Glob;
/// use wax::walk::LinkBehavior;
///
/// for entry in Glob::new("**")
///     .unwrap()
///     .walk_with_behavior(".", LinkBehavior::ReadTarget)
/// {
///     let entry = entry.unwrap();
///     // ...
/// }
/// ```
///
/// [`depth`]: crate::walk::WalkBehavior::depth
/// [`Glob::walk_with_behavior`]: crate::Glob::walk_with_behavior
/// [`link`]: crate::walk::WalkBehavior::link
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WalkBehavior {
    /// Bounds on the depth of the walk and matched files.
    ///
    /// Determines the minimum and maximum depths of a walk and matched files relative to the [root
    /// path segment][`Entry::root_relative_paths`]. The default value is
    /// [`DepthBehavior::Unbounded`].
    ///
    /// [`DepthBehavior::Unbounded`]: crate::walk::DepthBehavior::Unbounded
    /// [`Entry::root_relative_paths`]: crate::walk::Entry::root_relative_paths
    pub depth: DepthBehavior,
    /// Interpretation of symbolic links.
    ///
    /// Determines how symbolic links are interpreted when walking a directory tree. The default
    /// value is [`LinkBehavior::ReadFile`].
    ///
    /// [`LinkBehavior::ReadFile`]: crate::walk::LinkBehavior::ReadFile
    pub link: LinkBehavior,
}

impl From<()> for WalkBehavior {
    fn from(_: ()) -> Self {
        Default::default()
    }
}

impl From<DepthBehavior> for WalkBehavior {
    fn from(depth: DepthBehavior) -> Self {
        WalkBehavior {
            depth,
            ..Default::default()
        }
    }
}

impl From<DepthMax> for WalkBehavior {
    fn from(max: DepthMax) -> Self {
        DepthBehavior::from(max).into()
    }
}

impl From<DepthMin> for WalkBehavior {
    fn from(min: DepthMin) -> Self {
        DepthBehavior::from(min).into()
    }
}

impl From<DepthMinMax> for WalkBehavior {
    fn from(minmax: DepthMinMax) -> Self {
        DepthBehavior::from(minmax).into()
    }
}

impl From<LinkBehavior> for WalkBehavior {
    fn from(link: LinkBehavior) -> Self {
        WalkBehavior {
            link,
            ..Default::default()
        }
    }
}
