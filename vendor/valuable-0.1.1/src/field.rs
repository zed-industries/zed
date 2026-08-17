/// Data stored within a `Structable` or  an `Enumerable`.
#[derive(Debug)]
pub enum Fields<'a> {
    /// Named fields
    Named(&'a [NamedField<'a>]),

    /// Unnamed (positional) fields or unit
    ///
    /// The `usize` value represents the number of fields.
    Unnamed(usize),
}

/// A named field
#[derive(Debug, Clone, Copy)]
pub struct NamedField<'a>(&'a str);

impl Fields<'_> {
    /// Returns `true` if the fields are named.
    ///
    /// # Examples
    ///
    /// Named fields
    ///
    /// ```
    /// use valuable::Fields;
    ///
    /// let fields = Fields::Named(&[]);
    /// assert!(fields.is_named());
    /// ```
    ///
    /// Unnamed fields
    ///
    /// ```
    /// use valuable::Fields;
    ///
    /// let fields = Fields::Unnamed(2);
    /// assert!(!fields.is_named());
    /// ```
    pub const fn is_named(&self) -> bool {
        matches!(self, Fields::Named(..))
    }

    /// Returns `true` if the fields are unnamed.
    ///
    /// # Examples
    ///
    /// Named fields
    ///
    /// ```
    /// use valuable::Fields;
    ///
    /// let fields = Fields::Named(&[]);
    /// assert!(!fields.is_unnamed());
    /// ```
    ///
    /// Unnamed fields
    ///
    /// ```
    /// use valuable::Fields;
    ///
    /// let fields = Fields::Unnamed(3);
    /// assert!(fields.is_unnamed());
    /// ```
    pub const fn is_unnamed(&self) -> bool {
        matches!(self, Fields::Unnamed(_))
    }

    /// Returns the number of fields.
    ///
    /// # Examples
    ///
    /// Named fields
    ///
    /// ```
    /// use valuable::{Fields, NamedField};
    ///
    /// let fields = &[
    ///     NamedField::new("alice"),
    ///     NamedField::new("bob"),
    /// ];
    /// let fields = Fields::Named(fields);
    ///
    /// assert_eq!(fields.len(), 2);
    /// ```
    ///
    /// Unnamed fields
    ///
    /// ```
    /// use valuable::Fields;
    ///
    /// let fields = Fields::Unnamed(2);
    /// assert_eq!(fields.len(), 2);
    /// ```
    pub const fn len(&self) -> usize {
        match self {
            Self::Named(names) => names.len(),
            Self::Unnamed(len) => *len,
        }
    }

    /// Returns `true` if this set of fields defines no fields.
    ///
    /// # Examples
    ///
    /// Named fields
    ///
    /// ```
    /// use valuable::{Fields, NamedField};
    ///
    /// let fields = &[
    ///     NamedField::new("alice"),
    ///     NamedField::new("bob"),
    /// ];
    /// let non_empty = Fields::Named(fields);
    ///
    /// let empty = Fields::Named(&[]);
    ///
    /// assert!(!non_empty.is_empty());
    /// assert!(empty.is_empty());
    /// ```
    ///
    /// Unnamed fields
    ///
    /// ```
    /// use valuable::Fields;
    ///
    /// let non_empty = Fields::Unnamed(2);
    /// let empty = Fields::Unnamed(0);
    ///
    /// assert!(!non_empty.is_empty());
    /// assert!(empty.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a> NamedField<'a> {
    /// Create a new `NamedField` instance with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::NamedField;
    ///
    /// let field = NamedField::new("hello");
    /// assert_eq!("hello", field.name());
    /// ```
    pub const fn new(name: &'a str) -> NamedField<'a> {
        NamedField(name)
    }

    /// Returns the field name
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::NamedField;
    ///
    /// let field = NamedField::new("hello");
    /// assert_eq!("hello", field.name());
    /// ```
    pub const fn name(&self) -> &str {
        self.0
    }
}
