use crate::field::*;
use crate::*;

use core::fmt;

/// A struct-like [`Valuable`] sub-type.
///
/// Implemented by [`Valuable`] types that have a struct-like shape. Fields may
/// be named or unnamed (tuple). Values that implement `Structable` must return
/// [`Value::Structable`] from their [`Valuable::as_value`] implementation.
///
/// # Inspecting
///
/// Inspecting fields contained by a `Structable` instance is done by visiting
/// the struct. When visiting a `Structable`, either the `visit_named_fields()`
/// or the `visit_unnamed_fields()` methods of `Visit` are called. Each method
/// may be called multiple times per `Structable`, but the two methods are never
/// mixed.
///
/// ```
/// use valuable::{NamedValues, Valuable, Value, Visit};
///
/// #[derive(Valuable)]
/// struct MyStruct {
///     foo: u32,
///     bar: u32,
/// }
///
/// struct PrintFields;
///
/// impl Visit for PrintFields {
///     fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
///         for (field, value) in named_values.iter() {
///             println!("{}: {:?}", field.name(), value);
///         }
///     }
///
///     fn visit_value(&mut self, value: Value<'_>) {
///         match value {
///             Value::Structable(v) => v.visit(self),
///             _ => {} // do nothing for other types
///         }
///     }
/// }
///
/// let my_struct = MyStruct {
///     foo: 123,
///     bar: 456,
/// };
///
/// valuable::visit(&my_struct, &mut PrintFields);
/// ```
///
/// If the struct is **statically** defined, then all fields are known ahead of
/// time and may be accessed via the [`StructDef`] instance returned by
/// [`definition()`]. [`NamedField`] instances returned by [`definition()`]
/// maybe used to efficiently extract specific field values.
///
/// # Implementing
///
/// Implementing `Structable` is usually done by adding `#[derive(Valuable)]` to
/// a Rust `struct` definition.
///
/// ```
/// use valuable::{Fields, Valuable, Structable, StructDef};
///
/// #[derive(Valuable)]
/// struct MyStruct {
///     foo: &'static str,
/// }
///
/// let my_struct = MyStruct { foo: "Hello" };
/// let fields = match my_struct.definition() {
///     StructDef::Static { name, fields, .. } => {
///         assert_eq!("MyStruct", name);
///         fields
///     }
///     _ => unreachable!(),
/// };
///
/// match fields {
///     Fields::Named(named_fields) => {
///         assert_eq!(1, named_fields.len());
///         assert_eq!("foo", named_fields[0].name());
///     }
///     _ => unreachable!(),
/// }
/// ```
///
/// [`definition()`]: Structable::definition()
pub trait Structable: Valuable {
    /// Returns the struct's definition.
    ///
    /// See [`StructDef`] documentation for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{Structable, Valuable};
    ///
    /// #[derive(Valuable)]
    /// struct MyStruct {
    ///     foo: u32,
    /// }
    ///
    /// let my_struct = MyStruct {
    ///     foo: 123,
    /// };
    ///
    /// assert_eq!("MyStruct", my_struct.definition().name());
    fn definition(&self) -> StructDef<'_>;
}

/// A struct's name, fields, and other struct-level information.
///
/// Returned by [`Structable::definition()`], `StructDef` provides the caller
/// with information about the struct's definition.
///
/// [`Structable::definition()`]: Structable::definition
#[derive(Debug)]
#[non_exhaustive]
pub enum StructDef<'a> {
    /// The struct is statically-defined, all fields are known ahead of time.
    ///
    /// Most `Structable` definitions for Rust struct types will be
    /// `StructDef::Static`.
    ///
    /// # Examples
    ///
    /// A statically defined struct
    ///
    /// ```
    /// use valuable::{Fields, Valuable, Structable, StructDef};
    ///
    /// #[derive(Valuable)]
    /// struct MyStruct {
    ///     foo: &'static str,
    /// }
    ///
    /// let my_struct = MyStruct { foo: "Hello" };
    /// let fields = match my_struct.definition() {
    ///     StructDef::Static { name, fields, ..} => {
    ///         assert_eq!("MyStruct", name);
    ///         fields
    ///     }
    ///     _ => unreachable!(),
    /// };
    ///
    /// match fields {
    ///     Fields::Named(named_fields) => {
    ///         assert_eq!(1, named_fields.len());
    ///         assert_eq!("foo", named_fields[0].name());
    ///     }
    ///     _ => unreachable!(),
    /// }
    /// ```
    #[non_exhaustive]
    Static {
        /// The struct's name.
        name: &'static str,

        /// The struct's fields.
        fields: Fields<'static>,
    },

    /// The struct is dynamically-defined, not all fields are known ahead of
    /// time.
    ///
    /// A dynamically-defined struct **could** be represented using
    /// [`Mappable`], though, using `Structable` offers benefits in a couple of
    /// cases. For example, when serializing a `Value`, some formats will
    /// serialize maps and structs differently. In this case, differentiating
    /// the two is required. There also are times when **some** struct fields
    /// are known statically, but not all of them (see second example).
    ///
    /// # Examples
    ///
    /// The struct stores field values in a `HashMap`.
    ///
    /// ```
    /// use valuable::{Fields, NamedField, NamedValues, Structable, StructDef, Value, Valuable, Visit};
    /// use std::collections::HashMap;
    ///
    /// /// A dynamic struct
    /// struct Dyn {
    ///     // The struct name
    ///     name: String,
    ///
    ///     // Named values.
    ///     values: HashMap<String, Box<dyn Valuable>>,
    /// }
    ///
    /// impl Valuable for Dyn {
    ///     fn as_value(&self) -> Value<'_> {
    ///         Value::Structable(self)
    ///     }
    ///
    ///     fn visit(&self, visit: &mut dyn Visit) {
    ///         // This could be optimized to batch some.
    ///         for (field, value) in self.values.iter() {
    ///             visit.visit_named_fields(&NamedValues::new(
    ///                 &[NamedField::new(field)],
    ///                 &[value.as_value()],
    ///             ));
    ///         }
    ///     }
    /// }
    ///
    /// impl Structable for Dyn {
    ///     fn definition(&self) -> StructDef<'_> {
    ///         StructDef::new_dynamic(&self.name, Fields::Named(&[]))
    ///     }
    /// }
    /// ```
    ///
    /// Some fields are known statically.
    ///
    /// ```
    /// use valuable::{Fields, NamedField, NamedValues, Structable, StructDef, Value, Valuable, Visit};
    /// use std::collections::HashMap;
    ///
    /// struct HalfStatic {
    ///     foo: u32,
    ///     bar: u32,
    ///     extra_values: HashMap<String, Box<dyn Valuable>>,
    /// }
    ///
    /// impl Valuable for HalfStatic {
    ///     fn as_value(&self) -> Value<'_> {
    ///         Value::Structable(self)
    ///     }
    ///
    ///     fn visit(&self, visit: &mut dyn Visit) {
    ///         // First, visit static fields
    ///         visit.visit_named_fields(&NamedValues::new(
    ///             FIELDS,
    ///             &[self.foo.as_value(), self.bar.as_value()],
    ///         ));
    ///
    ///         // This could be optimized to batch some.
    ///         for (field, value) in self.extra_values.iter() {
    ///             visit.visit_named_fields(&NamedValues::new(
    ///                 &[NamedField::new(field)],
    ///                 &[value.as_value()],
    ///             ));
    ///         }
    ///     }
    /// }
    ///
    /// static FIELDS: &[NamedField<'static>] = &[
    ///     NamedField::new("foo"),
    ///     NamedField::new("bar"),
    /// ];
    ///
    /// impl Structable for HalfStatic {
    ///     fn definition(&self) -> StructDef<'_> {
    ///         // Include known fields.
    ///         StructDef::new_dynamic(
    ///             "HalfStatic",
    ///             Fields::Named(FIELDS))
    ///     }
    /// }
    /// ```
    #[non_exhaustive]
    Dynamic {
        /// The struct's name
        name: &'a str,

        /// The struct's fields.
        fields: Fields<'a>,
    },
}

impl fmt::Debug for dyn Structable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let def = self.definition();

        if def.fields().is_named() {
            struct DebugStruct<'a, 'b> {
                fmt: fmt::DebugStruct<'a, 'b>,
            }

            let mut debug = DebugStruct {
                fmt: fmt.debug_struct(def.name()),
            };

            impl Visit for DebugStruct<'_, '_> {
                fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
                    for (field, value) in named_values {
                        self.fmt.field(field.name(), value);
                    }
                }

                fn visit_value(&mut self, _: Value<'_>) {
                    unreachable!()
                }
            }

            self.visit(&mut debug);

            debug.fmt.finish()
        } else {
            struct DebugStruct<'a, 'b> {
                fmt: fmt::DebugTuple<'a, 'b>,
            }

            let mut debug = DebugStruct {
                fmt: fmt.debug_tuple(def.name()),
            };

            impl Visit for DebugStruct<'_, '_> {
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

impl<'a> StructDef<'a> {
    /// Create a new [`StructDef::Static`] instance.
    ///
    /// This should be used when a struct's fields are fixed and known ahead of time.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_static("Foo", Fields::Unnamed(2));
    /// ```
    pub const fn new_static(name: &'static str, fields: Fields<'static>) -> StructDef<'a> {
        StructDef::Static { name, fields }
    }

    /// Create a new [`StructDef::Dynamic`] instance.
    ///
    /// This is used when the struct's fields may vary at runtime.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_dynamic("Foo", Fields::Unnamed(3));
    /// ```
    pub const fn new_dynamic(name: &'a str, fields: Fields<'a>) -> StructDef<'a> {
        StructDef::Dynamic { name, fields }
    }

    /// Returns the struct's name
    ///
    /// # Examples
    ///
    /// With a static struct
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_static("Foo", Fields::Unnamed(1));
    /// assert_eq!("Foo", def.name());
    /// ```
    ///
    /// With a dynamic struct
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_dynamic("Foo", Fields::Unnamed(2));
    /// assert_eq!("Foo", def.name());
    /// ```
    pub const fn name(&self) -> &'a str {
        match self {
            StructDef::Static { name, .. } => name,
            StructDef::Dynamic { name, .. } => name,
        }
    }

    /// Returns the struct's fields
    ///
    /// # Examples
    ///
    /// With a static struct
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_static("Foo", Fields::Unnamed(3));
    /// assert!(matches!(def.fields(), Fields::Unnamed(_)));
    /// ```
    ///
    /// With a dynamic struct
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_dynamic("Foo", Fields::Unnamed(1));
    /// assert!(matches!(def.fields(), Fields::Unnamed(_)));
    /// ```
    pub const fn fields(&self) -> &Fields<'a> {
        match self {
            StructDef::Static { fields, .. } => fields,
            StructDef::Dynamic { fields, .. } => fields,
        }
    }

    /// Returns `true` if the struct is [statically defined](StructDef::Static).
    ///
    /// # Examples
    ///
    /// With a static struct
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_static("Foo", Fields::Unnamed(2));
    /// assert!(def.is_static());
    /// ```
    ///
    /// With a dynamic struct
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_dynamic("Foo", Fields::Unnamed(4));
    /// assert!(!def.is_static());
    /// ```
    pub const fn is_static(&self) -> bool {
        matches!(self, StructDef::Static { .. })
    }

    /// Returns `true` if the struct is [dynamically defined](StructDef::Dynamic).
    ///
    /// # Examples
    ///
    /// With a static struct
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_static("Foo", Fields::Unnamed(1));
    /// assert!(!def.is_dynamic());
    /// ```
    ///
    /// With a dynamic struct
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_dynamic("Foo", Fields::Unnamed(1));
    /// assert!(def.is_dynamic());
    /// ```
    pub const fn is_dynamic(&self) -> bool {
        matches!(self, StructDef::Dynamic { .. })
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
            impl<T: ?Sized + Structable> Structable for $ty {
                fn definition(&self) -> StructDef<'_> {
                    T::definition(&**self)
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
