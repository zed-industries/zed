use crate::field::*;
use crate::*;

#[cfg(feature = "alloc")]
use alloc::format;
use core::fmt;

/// An enum-like [`Valuable`] sub-type.
///
/// Implemented by [`Valuable`] types that have an enum-like shape. Fields may
/// be named or unnamed (tuple). Values that implement `Enumerable` must return
/// [`Value::Enumerable`] from their [`Valuable::as_value`] implementation.
///
/// # Inspecting
///
/// The [`variant()`] method returns the `Enumerable` instance's variant. The
/// `Enumerable` may also have unnamed fields (tuple) or named fields.
/// Inspecting the field values is done by visiting the enum. When visiting an
/// `Enumerable`, either the [`visit_named_fields()`] or the
/// [`visit_unnamed_fields()`] methods of [`Visit`] are called. Each method may
/// be called multiple times per `Enumerable`, but the two methods are never
/// mixed.
///
/// [`variant()`]: Enumerable::variant
/// [`visit_named_fields()`]: Visit::visit_named_fields
/// [`visit_unnamed_fields()`]: Visit::visit_unnamed_fields
///
/// ```
/// use valuable::{Valuable, Value, Visit};
///
/// #[derive(Valuable)]
/// enum MyEnum {
///     Foo,
///     Bar(u32),
/// }
///
/// struct PrintVariant;
///
/// impl Visit for PrintVariant {
///     fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
///         for value in values {
///             println!(" - {:?}", value);
///         }
///     }
///
///     fn visit_value(&mut self, value: Value<'_>) {
///         match value {
///             Value::Enumerable(v) => {
///                 println!("{}", v.variant().name());
///                 v.visit(self)
///             }
///             _ => {}
///         }
///     }
/// }
///
/// let my_enum = MyEnum::Bar(123);
///
/// valuable::visit(&my_enum, &mut PrintVariant);
/// ```
///
/// If the enum is **statically** defined, then all variants, and variant fields
/// are known ahead of time and may be accessed via the [`EnumDef`] instance
/// returned by [`definition()`].
///
/// [`definition()`]: Enumerable::definition
///
/// # Implementing
///
/// Implementing `Enumerable` is usually done by adding `#[derive(Valuable)]` to
/// a Rust `enum` definition.
///
/// ```
/// use valuable::{Valuable, Enumerable, EnumDef};
///
/// #[derive(Valuable)]
/// enum MyEnum {
///     Foo,
///     Bar(u32),
/// }
///
/// let my_enum = MyEnum::Bar(123);
///
/// let variants = match my_enum.definition() {
///     EnumDef::Static { name, variants, .. } => {
///         assert_eq!("MyEnum", name);
///         variants
///     }
///     _ => unreachable!(),
/// };
///
/// assert_eq!(2, variants.len());
/// assert_eq!("Foo", variants[0].name());
/// assert!(variants[0].fields().is_unnamed());
/// ```
pub trait Enumerable: Valuable {
    /// Returns the enum's definition.
    ///
    /// See [`EnumDef`] documentation for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{Enumerable, Valuable};
    ///
    /// #[derive(Valuable)]
    /// enum MyEnum {
    ///     Foo,
    ///     Bar(u32),
    /// }
    ///
    /// let my_enum = MyEnum::Bar(123);
    ///
    /// assert_eq!("MyEnum", my_enum.definition().name());
    /// ```
    fn definition(&self) -> EnumDef<'_>;

    /// Returns the `enum`'s current variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{Enumerable, Valuable};
    ///
    /// #[derive(Valuable)]
    /// enum MyEnum {
    ///     Foo,
    ///     Bar(u32),
    /// }
    ///
    /// let my_enum = MyEnum::Foo;
    /// assert_eq!("Foo", my_enum.variant().name());
    /// ```
    fn variant(&self) -> Variant<'_>;
}

/// An enum's variants, variant fields, and other enum-level information.
///
/// Returned by [`Enumerable::definition()`], `EnumDef` provides the caller with
/// information about the enum's definition.
#[non_exhaustive]
#[derive(Debug)]
pub enum EnumDef<'a> {
    /// The enum is statically-defined, all variants and variant-level fields
    /// are known ahead of time.
    ///
    /// Most `Enumerable` definitions for Rust enum types will be
    /// `EnumDef::Static`.
    ///
    /// # Examples
    ///
    /// A statically defined enum
    ///
    /// ```
    /// use valuable::{Valuable, Enumerable, EnumDef};
    ///
    /// #[derive(Valuable)]
    /// enum MyEnum {
    ///     Foo,
    ///     Bar(u32),
    /// }
    ///
    /// let my_enum = MyEnum::Bar(123);
    ///
    /// let variants = match my_enum.definition() {
    ///     EnumDef::Static { name, variants, .. } => {
    ///         assert_eq!("MyEnum", name);
    ///         variants
    ///     }
    ///     _ => unreachable!(),
    /// };
    ///
    /// assert_eq!(2, variants.len());
    /// assert_eq!("Foo", variants[0].name());
    /// assert_eq!("Bar", variants[1].name());
    /// ```
    #[non_exhaustive]
    Static {
        /// The enum's name
        name: &'static str,

        /// The enum's variants
        variants: &'static [VariantDef<'static>],
    },

    /// The enum is dynamically-defined, not all variants and fields are known
    /// ahead of time.
    ///
    /// # Examples
    ///
    /// The enum variant is tracked as a string
    ///
    /// ```
    /// use valuable::{Enumerable, EnumDef, Fields, VariantDef, Valuable, Value, Variant, Visit};
    ///
    /// /// A dynamic enum
    /// struct DynEnum {
    ///     // The enum name
    ///     name: String,
    ///
    ///     // The current variant
    ///     variant: String,
    /// }
    ///
    /// impl Valuable for DynEnum {
    ///     fn as_value(&self) -> Value<'_> {
    ///         Value::Enumerable(self)
    ///     }
    ///
    ///     fn visit(&self, _visit: &mut dyn Visit) {
    ///         // No variant fields, so there is nothing to call here.
    ///     }
    /// }
    ///
    /// impl Enumerable for DynEnum {
    ///     fn definition(&self) -> EnumDef<'_> {
    ///         EnumDef::new_dynamic(&self.name, &[])
    ///     }
    ///
    ///     fn variant(&self) -> Variant<'_> {
    ///         Variant::Dynamic(VariantDef::new(&self.variant, Fields::Unnamed(0)))
    ///     }
    /// }
    /// ```
    #[non_exhaustive]
    Dynamic {
        /// The enum's name
        name: &'a str,

        /// The enum's variants
        variants: &'a [VariantDef<'a>],
    },
}

/// An enum variant definition.
///
/// Included with [`EnumDef`] returned by [`Enumerable::definition()`],
/// `VariantDef` provides the caller with information about a specific variant.
#[derive(Debug)]
pub struct VariantDef<'a> {
    /// Variant name
    name: &'a str,

    /// Variant fields
    fields: Fields<'a>,
}

/// An enum variant
///
/// Returned by [`Enumerable::variant()`], `Variant` represents a single enum
/// variant.
#[derive(Debug)]
pub enum Variant<'a> {
    /// The variant is statically defined by the associated enum.
    Static(&'static VariantDef<'static>),

    /// The variant is dynamically defined and not included as part of
    /// [`Enumerable::definition()`].
    Dynamic(VariantDef<'a>),
}

impl<'a> EnumDef<'a> {
    /// Create a new [`EnumDef::Static`] instance.
    ///
    /// This should be used when an enum's variants are fixed and known ahead of
    /// time.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{EnumDef, Fields, VariantDef};
    ///
    /// static VARIANTS: &[VariantDef<'static>] = &[
    ///     VariantDef::new("Bar", Fields::Unnamed(1)),
    /// ];
    ///
    /// let def = EnumDef::new_static( "Foo", VARIANTS);
    /// ```
    pub const fn new_static(
        name: &'static str,
        variants: &'static [VariantDef<'static>],
    ) -> EnumDef<'a> {
        EnumDef::Static { name, variants }
    }

    /// Create a new [`EnumDef::Dynamic`] instance.
    ///
    /// This is used when the enum's variants may vary at runtime.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{EnumDef, Fields, VariantDef};
    ///
    /// let def = EnumDef::new_dynamic(
    ///     "Foo",
    ///     &[VariantDef::new("Bar", Fields::Unnamed(1))]
    /// );
    /// ```
    pub const fn new_dynamic(name: &'a str, variants: &'a [VariantDef<'a>]) -> EnumDef<'a> {
        EnumDef::Dynamic { name, variants }
    }

    /// Returns the enum's name
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{Enumerable, Valuable};
    ///
    /// #[derive(Valuable)]
    /// enum Foo {
    ///     Bar,
    ///     Baz,
    /// }
    ///
    /// let def = Foo::Bar.definition();
    /// assert_eq!("Foo", def.name());
    /// ```
    pub fn name(&self) -> &str {
        match self {
            EnumDef::Static { name, .. } => name,
            EnumDef::Dynamic { name, .. } => name,
        }
    }

    /// Returns the enum's variants
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{Enumerable, Valuable};
    ///
    /// #[derive(Valuable)]
    /// enum Foo {
    ///     Bar,
    ///     Baz,
    /// }
    ///
    /// let def = Foo::Bar.definition();
    /// let variants = def.variants();
    ///
    /// assert_eq!(2, variants.len());
    /// assert_eq!("Bar", variants[0].name());
    /// ```
    pub fn variants(&self) -> &[VariantDef<'_>] {
        match self {
            EnumDef::Static { variants, .. } => variants,
            EnumDef::Dynamic { variants, .. } => variants,
        }
    }

    /// Returns `true` if the enum is [statically defined](EnumDef::Static).
    ///
    /// # Examples
    ///
    /// With a static enum
    ///
    /// ```
    /// use valuable::{Enumerable, Valuable};
    ///
    /// #[derive(Valuable)]
    /// enum Foo {
    ///     Bar,
    ///     Baz,
    /// }
    ///
    /// let def = Foo::Bar.definition();
    /// assert!(def.is_static());
    /// ```
    ///
    /// With a dynamic enum
    ///
    /// ```
    /// use valuable::{EnumDef, Fields, VariantDef};
    ///
    /// let def = EnumDef::new_dynamic("Foo", &[]);
    /// assert!(!def.is_static());
    /// ```
    pub fn is_static(&self) -> bool {
        matches!(self, EnumDef::Static { .. })
    }

    /// Returns `true` if the enum is [dynamically defined](EnumDef::Dynamic).
    ///
    /// # Examples
    ///
    /// With a static enum
    ///
    /// ```
    /// use valuable::{Enumerable, Valuable};
    ///
    /// #[derive(Valuable)]
    /// enum Foo {
    ///     Bar,
    ///     Baz,
    /// }
    ///
    /// let def = Foo::Bar.definition();
    /// assert!(!def.is_dynamic());
    /// ```
    ///
    /// With a dynamic enum
    ///
    /// ```
    /// use valuable::{EnumDef, Fields, VariantDef};
    ///
    /// let def = EnumDef::new_dynamic("Foo", &[]);
    /// assert!(def.is_dynamic());
    /// ```
    pub fn is_dynamic(&self) -> bool {
        matches!(self, EnumDef::Dynamic { .. })
    }
}

impl<'a> VariantDef<'a> {
    /// Creates a new `VariantDef` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{Fields, VariantDef};
    ///
    /// let def = VariantDef::new("Foo", Fields::Unnamed(2));
    /// ```
    pub const fn new(name: &'a str, fields: Fields<'a>) -> VariantDef<'a> {
        VariantDef { name, fields }
    }

    /// Returns the variant's name
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{Fields, VariantDef};
    ///
    /// let def = VariantDef::new("Foo", Fields::Unnamed(2));
    /// assert_eq!("Foo", def.name());
    /// ```
    pub fn name(&self) -> &str {
        self.name
    }

    /// Returns the variant's fields
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{Fields, VariantDef};
    ///
    /// let def = VariantDef::new("Foo", Fields::Unnamed(3));
    /// assert!(matches!(def.fields(), Fields::Unnamed(_)));
    /// ```
    pub fn fields(&self) -> &Fields<'_> {
        &self.fields
    }
}

impl Variant<'_> {
    /// Returns the variant's name
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{Fields, Variant, VariantDef};
    ///
    /// static VARIANT: &VariantDef<'static> = &VariantDef::new(
    ///     "Foo", Fields::Unnamed(2));
    ///
    /// let variant = Variant::Static(VARIANT);
    /// assert_eq!("Foo", variant.name());
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Variant::Static(v) => v.name(),
            Variant::Dynamic(v) => v.name(),
        }
    }

    /// Returns the variant's fields
    pub fn fields(&self) -> &Fields<'_> {
        match self {
            Variant::Static(v) => v.fields(),
            Variant::Dynamic(v) => v.fields(),
        }
    }

    /// Returns `true` if the variant has associated named fields.
    ///
    /// # Examples
    ///
    /// With named fields
    ///
    /// ```
    /// use valuable::{Fields, NamedField, Variant, VariantDef};
    ///
    /// static VARIANT: &VariantDef<'static> = &VariantDef::new(
    ///     "Foo", Fields::Named(&[NamedField::new("hello")]));
    ///
    /// let variant = Variant::Static(VARIANT);
    /// assert!(variant.is_named_fields());
    /// ```
    ///
    /// With unnamed fields
    ///
    /// ```
    /// use valuable::{Fields, Variant, VariantDef};
    ///
    /// static VARIANT: &VariantDef<'static> = &VariantDef::new(
    ///     "Foo", Fields::Unnamed(1));
    ///
    /// let variant = Variant::Static(VARIANT);
    /// assert!(!variant.is_named_fields());
    /// ```
    pub fn is_named_fields(&self) -> bool {
        self.fields().is_named()
    }

    /// Returns `true` if the variant has associated unnamed fields.
    ///
    /// # Examples
    ///
    /// With named fields
    ///
    /// ```
    /// use valuable::{Fields, NamedField, Variant, VariantDef};
    ///
    /// static VARIANT: &VariantDef<'static> = &VariantDef::new(
    ///     "Foo", Fields::Named(&[NamedField::new("hello")]));
    ///
    /// let variant = Variant::Static(VARIANT);
    /// assert!(!variant.is_unnamed_fields());
    /// ```
    ///
    /// With unnamed fields
    ///
    /// ```
    /// use valuable::{Fields, Variant, VariantDef};
    ///
    /// static VARIANT: &VariantDef<'static> = &VariantDef::new(
    ///     "Foo", Fields::Unnamed(1));
    ///
    /// let variant = Variant::Static(VARIANT);
    /// assert!(variant.is_unnamed_fields());
    /// ```
    pub fn is_unnamed_fields(&self) -> bool {
        !self.is_named_fields()
    }
}

impl fmt::Debug for dyn Enumerable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant = self.variant();
        #[cfg(feature = "alloc")]
        let name = format!("{}::{}", self.definition().name(), variant.name());
        #[cfg(not(feature = "alloc"))]
        let name = variant.name();

        if variant.is_named_fields() {
            struct DebugEnum<'a, 'b> {
                fmt: fmt::DebugStruct<'a, 'b>,
            }

            let mut debug = DebugEnum {
                fmt: fmt.debug_struct(&name),
            };

            impl Visit for DebugEnum<'_, '_> {
                fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
                    for (field, value) in named_values {
                        self.fmt.field(field.name(), value);
                    }
                }

                fn visit_value(&mut self, _: Value<'_>) {
                    unreachable!();
                }
            }

            self.visit(&mut debug);

            debug.fmt.finish()
        } else {
            struct DebugEnum<'a, 'b> {
                fmt: fmt::DebugTuple<'a, 'b>,
            }

            let mut debug = DebugEnum {
                fmt: fmt.debug_tuple(&name),
            };

            impl Visit for DebugEnum<'_, '_> {
                fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
                    for value in values {
                        self.fmt.field(value);
                    }
                }

                fn visit_value(&mut self, _: Value<'_>) {
                    unreachable!();
                }
            }

            self.visit(&mut debug);

            debug.fmt.finish()
        }
    }
}

macro_rules! deref {
    (
        $(
            $(#[$attrs:meta])*
            $ty:ty,
        )*
    ) => {
        $(
            $(#[$attrs])*
            impl<T: ?Sized + Enumerable> Enumerable for $ty {
                fn definition(&self) -> EnumDef<'_> {
                    T::definition(&**self)
                }

                fn variant(&self) -> Variant<'_> {
                    T::variant(&**self)
                }
            }
        )*
    };
}

deref! {
    &T,
    &mut T,
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<T>,
    #[cfg(feature = "alloc")]
    alloc::rc::Rc<T>,
    #[cfg(not(valuable_no_atomic_cas))]
    #[cfg(feature = "alloc")]
    alloc::sync::Arc<T>,
}

static RESULT_VARIANTS: &[VariantDef<'static>] = &[
    VariantDef::new("Ok", Fields::Unnamed(1)),
    VariantDef::new("Err", Fields::Unnamed(1)),
];

impl<T, E> Enumerable for Result<T, E>
where
    T: Valuable,
    E: Valuable,
{
    fn definition(&self) -> EnumDef<'_> {
        EnumDef::new_static("Result", RESULT_VARIANTS)
    }

    fn variant(&self) -> Variant<'_> {
        match self {
            Ok(_) => Variant::Static(&RESULT_VARIANTS[0]),
            Err(_) => Variant::Static(&RESULT_VARIANTS[1]),
        }
    }
}

impl<T, E> Valuable for Result<T, E>
where
    T: Valuable,
    E: Valuable,
{
    fn as_value(&self) -> Value<'_> {
        Value::Enumerable(self)
    }

    fn visit(&self, visitor: &mut dyn Visit) {
        match self {
            Ok(val) => visitor.visit_unnamed_fields(&[val.as_value()]),
            Err(val) => visitor.visit_unnamed_fields(&[val.as_value()]),
        }
    }
}
