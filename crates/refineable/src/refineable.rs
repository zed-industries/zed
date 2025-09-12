pub use derive_refineable::Refineable;

/// A trait for types that can be refined with partial updates.
///
/// The `Refineable` trait enables hierarchical configuration patterns where a base configuration
/// can be selectively overridden by refinements. This is particularly useful for styling and
/// settings, and theme hierarchies.
///
/// # Derive Macro
///
/// The `#[derive(Refineable)]` macro automatically generates a companion refinement type and
/// implements this trait. For a struct `Style`, it creates `StyleRefinement` where each field is
/// wrapped appropriately:
///
/// - **Refineable fields** (marked with `#[refineable]`): Become the corresponding refinement type
///   (e.g., `Bar` becomes `BarRefinement`)
/// - **Optional fields** (`Option<T>`): Remain as `Option<T>`
/// - **Regular fields**: Become `Option<T>`
///
/// ## Example
///
/// ```
/// use derive_refineable::Refineable as _;
/// use refineable::Refineable;
///
/// #[derive(Refineable, Clone, Default)]
/// struct Example {
///     color: String,
///     font_size: Option<u32>,
///     #[refineable]
///     margin: Margin,
/// }
///
/// #[derive(Refineable, Clone, Default)]
/// struct Margin {
///     top: u32,
///     left: u32,
/// }
///
///
/// fn example() {
///     let mut base_style = Example::default();
///     let refinement = ExampleRefinement {
///         color: Some("red".to_string()),
///         font_size: None,
///         margin: MarginRefinement {
///             top: Some(10),
///             left: None,
///         },
///     };
///
///     base_style.refine(&refinement);
/// }
/// ```
///
/// This generates `ExampleRefinement` with:
/// - `color: Option<String>`
/// - `font_size: Option<u32>` (unchanged)
/// - `margin: MarginRefinement`
///
/// ## Attributes
///
/// The derive macro supports these attributes on the struct:
/// - `#[refineable(Debug)]`: Implements `Debug` for the refinement type
/// - `#[refineable(Serialize)]`: Derives `Serialize` which skips serializing `None`
/// - `#[refineable(OtherTrait)]`: Derives additional traits on the refinement type
///
/// Fields can be marked with:
/// - `#[refineable]`: Field is itself refineable (uses nested refinement type)
pub trait Refineable: Clone {
    type Refinement: Refineable<Refinement = Self::Refinement> + IsEmpty + Default;

    /// Applies the given refinement to this instance, modifying it in place.
    ///
    /// Only non-empty values in the refinement are applied.
    ///
    /// * For refineable fields, this recursively calls `refine`.
    /// * For other fields, the value is replaced if present in the refinement.
    fn refine(&mut self, refinement: &Self::Refinement);

    /// Returns a new instance with the refinement applied, equivalent to cloning `self` and calling
    /// `refine` on it.
    fn refined(self, refinement: Self::Refinement) -> Self;

    /// Creates an instance from a cascade by merging all refinements atop the default value.
    fn from_cascade(cascade: &Cascade<Self>) -> Self
    where
        Self: Default + Sized,
    {
        Self::default().refined(cascade.merged())
    }

    /// Returns `true` if this instance would contain all values from the refinement.
    ///
    /// For refineable fields, this recursively checks `is_superset_of`. For other fields, this
    /// checks if the refinement's `Some` values match this instance's values.
    fn is_superset_of(&self, refinement: &Self::Refinement) -> bool;

    /// Returns a refinement that represents the difference between this instance and the given
    /// refinement.
    ///
    /// For refineable fields, this recursively calls `subtract`. For other fields, the field is
    /// `None` if the field's value is equal to the refinement.
    fn subtract(&self, refinement: &Self::Refinement) -> Self::Refinement;
}

pub trait IsEmpty {
    /// Returns `true` if applying this refinement would have no effect.
    fn is_empty(&self) -> bool;
}

/// A cascade of refinements that can be merged in priority order.
///
/// A cascade maintains a sequence of optional refinements where later entries
/// take precedence over earlier ones. The first slot (index 0) is always the
/// base refinement and is guaranteed to be present.
///
/// This is useful for implementing configuration hierarchies like CSS cascading,
/// where styles from different sources (user agent, user, author) are combined
/// with specific precedence rules.
pub struct Cascade<S: Refineable>(Vec<Option<S::Refinement>>);

impl<S: Refineable + Default> Default for Cascade<S> {
    fn default() -> Self {
        Self(vec![Some(Default::default())])
    }
}

/// A handle to a specific slot in a cascade.
///
/// Slots are used to identify specific positions in the cascade where
/// refinements can be set or updated.
#[derive(Copy, Clone)]
pub struct CascadeSlot(usize);

impl<S: Refineable + Default> Cascade<S> {
    /// Reserves a new slot in the cascade and returns a handle to it.
    ///
    /// The new slot is initially empty (`None`) and can be populated later
    /// using `set()`.
    pub fn reserve(&mut self) -> CascadeSlot {
        self.0.push(None);
        CascadeSlot(self.0.len() - 1)
    }

    /// Returns a mutable reference to the base refinement (slot 0).
    ///
    /// The base refinement is always present and serves as the foundation
    /// for the cascade.
    pub fn base(&mut self) -> &mut S::Refinement {
        self.0[0].as_mut().unwrap()
    }

    /// Sets the refinement for a specific slot in the cascade.
    ///
    /// Setting a slot to `None` effectively removes it from consideration
    /// during merging.
    pub fn set(&mut self, slot: CascadeSlot, refinement: Option<S::Refinement>) {
        self.0[slot.0] = refinement
    }

    /// Merges all refinements in the cascade into a single refinement.
    ///
    /// Refinements are applied in order, with later slots taking precedence.
    /// Empty slots (`None`) are skipped during merging.
    pub fn merged(&self) -> S::Refinement {
        let mut merged = self.0[0].clone().unwrap();
        for refinement in self.0.iter().skip(1).flatten() {
            merged.refine(refinement);
        }
        merged
    }
}
