/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Layers and layered bags of configuration data.
//!
//! The [`ConfigBag`](crate::config_bag::ConfigBag) structure is used to store and pass around configuration for client operations.
//! Interacting with it may be required in order to write an `Interceptor` or `RuntimePlugin` to
//! customize a client.
//!
//! A `ConfigBag` is essentially a stack of several immutable and sharable layers, with a single _mutable_ layer at
//! the top of the stack that is called "interceptor state". The intent of this last mutable layer is to allow for
//! more performant mutation of config within the execution of an operation.
//!
//! There are three separate layer types to be aware of when using a `ConfigBag`:
//! 1. [`Layer`](crate::config_bag::Layer) - A mutable layer. This is usually only used for adding config
//!    to the `ConfigBag`, but is also used for the interceptor state.
//! 2. [`CloneableLayer`](crate::config_bag::CloneableLayer) - Identical to `Layer`, except that it requires
//!    `Clone` bounds on the items added to it so that it can be deep cloned. Can be converted to a `Layer`
//!    while retaining the cloneability of its items such that the resulting layer could be cloned as long as
//!    nothing else is added to it later. A `Layer` cannot be converted back into a `CloneableLayer`.
//! 3. [`FrozenLayer`](crate::config_bag::FrozenLayer) - Basically an [`Arc`](std::sync::Arc) wrapper around
//!    a `Layer`. This wrapper is used to make the layer immutable, and to make it shareable between multiple
//!    `ConfigBag` instances. The frozen layer can be converted back to a `Layer` if there is only a single reference to it.
//!
//! All of `Layer`, `CloneableLayer`, `FrozenLayer`, and `ConfigBag` are considered to be "bag" types.
//! That is, they store arbitrary types so long as they implement the [`Storable`](crate::config_bag::Storable) trait.
//!
//! A `Storable` requires a `Storer` to be configured, and the storer allows for storables to be stored
//! in two different modes:
//! 1. [`StoreReplace`](crate::config_bag::StoreReplace) - Only one value of this type is allowed in a bag, and
//!    calling [`store_put()`](crate::config_bag::Layer::store_put) multiple times will replace the existing value
//!    in the bag. Calling [`load::<T>()`](crate::config_bag::Layer::load) returns exactly one value, if present.
//! 2. [`StoreAppend`](crate::config_bag::StoreAppend) - Multiple values of this type are allowed in a bag, and
//!    calling [`store_append()`](crate::config_bag::Layer::store_append) will add an additional value of this type
//!    to the bag. Calling [`load::<T>()`](crate::config_bag::Layer::load) returns an iterator over multiple values.
//!
//! # Examples
//!
//! Creating a storable data type with `StoreReplace`:
//!
//! ```no_run
//! use aws_smithy_types::config_bag::{Storable, StoreReplace};
//!
//! #[derive(Debug)]
//! struct SomeDataType {
//!     some_data: String,
//! }
//! impl Storable for SomeDataType {
//!     type Storer = StoreReplace<Self>;
//! }
//! ```
//!
//! Creating a storable data type with `StoreAppend`:
//!
//! ```no_run
//! use aws_smithy_types::config_bag::{Storable, StoreAppend};
//!
//! #[derive(Debug)]
//! struct SomeDataType {
//!     some_data: String,
//! }
//! impl Storable for SomeDataType {
//!     type Storer = StoreAppend<Self>;
//! }
//! ```
//!
//! Storing a storable in a bag when it is configured for `StoreReplace`:
//!
//! ```no_run
//! # use aws_smithy_types::config_bag::{Storable, StoreReplace};
//! # #[derive(Clone, Debug)]
//! # struct SomeDataType { some_data: String }
//! # impl Storable for SomeDataType { type Storer = StoreReplace<Self>; }
//! use aws_smithy_types::config_bag::{CloneableLayer, Layer};
//!
//! let mut layer = Layer::new("example");
//! layer.store_put(SomeDataType { some_data: "some data".to_string() });
//!
//! // `store_put` can be called again to replace the original value:
//! layer.store_put(SomeDataType { some_data: "replacement".to_string() });
//!
//! // Note: `SomeDataType` below must implement `Clone` to work with `CloneableLayer`
//! let mut cloneable = CloneableLayer::new("example");
//! cloneable.store_put(SomeDataType { some_data: "some data".to_string() });
//! ```
//!
//! Storing a storable in a bag when it is configured for `StoreAppend`:
//!
//! ```no_run
//! # use aws_smithy_types::config_bag::{Storable, StoreAppend};
//! # #[derive(Clone, Debug)]
//! # struct SomeDataType { some_data: String }
//! # impl Storable for SomeDataType { type Storer = StoreAppend<Self>; }
//! use aws_smithy_types::config_bag::{CloneableLayer, Layer};
//!
//! let mut layer = Layer::new("example");
//! layer.store_append(SomeDataType { some_data: "1".to_string() });
//! layer.store_append(SomeDataType { some_data: "2".to_string() });
//! ```
//!
//! Loading a `StoreReplace` value from a bag:
//!
//! ```no_run
//! # use aws_smithy_types::config_bag::{Storable, StoreReplace};
//! # #[derive(Clone, Debug)]
//! # struct SomeDataType { some_data: String }
//! # impl Storable for SomeDataType { type Storer = StoreReplace<Self>; }
//! # use aws_smithy_types::config_bag::Layer;
//! # let layer = Layer::new("example");
//! let maybe_value: Option<&SomeDataType> = layer.load::<SomeDataType>();
//! ```
//!
//! Loading a `StoreAppend` value from a bag:
//!
//! ```no_run
//! # use aws_smithy_types::config_bag::{Storable, StoreAppend};
//! # #[derive(Clone, Debug)]
//! # struct SomeDataType { some_data: String }
//! # impl Storable for SomeDataType { type Storer = StoreAppend<Self>; }
//! # use aws_smithy_types::config_bag::Layer;
//! # let layer = Layer::new("example");
//! let values: Vec<SomeDataType> = layer.load::<SomeDataType>().cloned().collect();
//!
//! // or iterate over them directly:
//! for value in layer.load::<SomeDataType>() {
//!     # let _ = value;
//!     // ...
//! }
//! ```
//!
mod storable;
mod typeid_map;

use crate::config_bag::typeid_map::TypeIdMap;
use crate::type_erasure::TypeErasedBox;
use std::any::{type_name, TypeId};
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::iter::Rev;
use std::marker::PhantomData;
use std::ops::Deref;
use std::slice::Iter;
use std::sync::Arc;

pub use storable::{AppendItemIter, Storable, Store, StoreAppend, StoreReplace};

/// [`FrozenLayer`] is the immutable and shareable form of [`Layer`].
///
/// See the [module docs](crate::config_bag) for more documentation.
#[derive(Clone, Debug)]
#[must_use]
pub struct FrozenLayer(Arc<Layer>);

impl FrozenLayer {
    /// Attempts to convert this bag directly into a [`Layer`] if no other references exist.
    pub fn try_modify(self) -> Option<Layer> {
        Arc::try_unwrap(self.0).ok()
    }
}

impl Deref for FrozenLayer {
    type Target = Layer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Layer> for FrozenLayer {
    fn from(layer: Layer) -> Self {
        FrozenLayer(Arc::new(layer))
    }
}

/// Private module to keep Value type while avoiding "private type in public latest"
pub(crate) mod value {
    #[derive(Clone, Debug)]
    pub enum Value<T> {
        Set(T),
        ExplicitlyUnset(&'static str),
    }

    impl<T: Default> Default for Value<T> {
        fn default() -> Self {
            Self::Set(Default::default())
        }
    }
}
use value::Value;

/// [`CloneableLayer`] allows itself to be cloned. This is useful when a type that implements
/// `Clone` wishes to store a config layer.
///
/// It ensures that all the items in `CloneableLayer` are `Clone` upon entry, e.g. when they are
/// first stored, the mutable methods require that they have a `Clone` bound on them.
///
/// While [`FrozenLayer`] is also cloneable, which is a shallow clone via `Arc`, `CloneableLayer`
/// performs a deep clone that newly allocates all the items stored in it.
///
/// Cloneable enforces that non clone items cannot be added
/// ```rust,compile_fail
/// use aws_smithy_types::config_bag::Storable;
/// use aws_smithy_types::config_bag::StoreReplace;
/// use aws_smithy_types::config_bag::CloneableLayer;
/// #[derive(Debug)]
/// struct MyNotCloneStruct;
///
/// impl Storable for MyNotCloneStruct {
///     type Storer = StoreReplace<MyNotCloneStruct>;
/// }
/// let mut layer = CloneableLayer::new("layer");
/// layer.store_put(MyNotCloneStruct);
/// ```
///
/// See the [module docs](crate::config_bag) for more documentation.
#[derive(Debug, Default)]
pub struct CloneableLayer(Layer);

impl Deref for CloneableLayer {
    type Target = Layer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for CloneableLayer {
    fn clone(&self) -> Self {
        Self(
            self.try_clone()
                .expect("only cloneable types can be inserted"),
        )
    }
}

impl From<CloneableLayer> for Layer {
    fn from(cloneable_layer: CloneableLayer) -> Layer {
        cloneable_layer.0
    }
}

// We need to "override" the mutable methods to encode the information that an item being stored
// implements `Clone`. For the immutable methods, they can just be delegated via the `Deref` trait.
impl CloneableLayer {
    /// Creates a new `CloneableLayer` with a given name
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self(Layer::new(name))
    }

    /// Converts this layer into a frozen layer that can no longer be mutated.
    pub fn freeze(self) -> FrozenLayer {
        self.0.into()
    }

    /// Removes `T` from this bag
    pub fn unset<T: Send + Sync + Clone + Debug + 'static>(&mut self) -> &mut Self {
        self.put_directly_cloneable::<StoreReplace<T>>(Value::ExplicitlyUnset(type_name::<T>()));
        self
    }

    fn put_directly_cloneable<T: Store>(&mut self, value: T::StoredType) -> &mut Self
    where
        T::StoredType: Clone,
    {
        self.0.props.insert(
            TypeId::of::<T::StoredType>(),
            TypeErasedBox::new_with_clone(value),
        );
        self
    }

    /// Stores `item` of type `T` into the config bag, overriding a previous value of the same type
    pub fn store_put<T>(&mut self, item: T) -> &mut Self
    where
        T: Storable<Storer = StoreReplace<T>> + Clone,
    {
        self.put_directly_cloneable::<StoreReplace<T>>(Value::Set(item));
        self
    }

    /// Stores `item` of type `T` into the config bag, overriding a previous value of the same type,
    /// or unsets it by passing a `None`
    pub fn store_or_unset<T>(&mut self, item: Option<T>) -> &mut Self
    where
        T: Storable<Storer = StoreReplace<T>> + Clone,
    {
        let item = match item {
            Some(item) => Value::Set(item),
            None => Value::ExplicitlyUnset(type_name::<T>()),
        };
        self.put_directly_cloneable::<StoreReplace<T>>(item);
        self
    }

    /// Stores `item` of type `T` into the config bag, appending it to the existing list of the same
    /// type
    pub fn store_append<T>(&mut self, item: T) -> &mut Self
    where
        T: Storable<Storer = StoreAppend<T>> + Clone,
    {
        match self.get_mut_or_default::<StoreAppend<T>>() {
            Value::Set(list) => list.push(item),
            v @ Value::ExplicitlyUnset(_) => *v = Value::Set(vec![item]),
        }
        self
    }

    /// Clears the value of type `T` from the config bag
    pub fn clear<T>(&mut self)
    where
        T: Storable<Storer = StoreAppend<T>> + Clone,
    {
        self.put_directly_cloneable::<StoreAppend<T>>(Value::ExplicitlyUnset(type_name::<T>()));
    }

    fn get_mut_or_default<T: Send + Sync + Store + 'static>(&mut self) -> &mut T::StoredType
    where
        T::StoredType: Default + Clone,
    {
        self.0
            .props
            .entry(TypeId::of::<T::StoredType>())
            .or_insert_with(|| TypeErasedBox::new_with_clone(T::StoredType::default()))
            .downcast_mut()
            .expect("typechecked")
    }
}

/// A named layer comprising a config bag
///
/// See the [module docs](crate::config_bag) for more documentation.
#[derive(Default)]
pub struct Layer {
    name: Cow<'static, str>,
    props: TypeIdMap<TypeErasedBox>,
}

impl Debug for Layer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        struct Items<'a>(&'a Layer);
        impl Debug for Items<'_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_list().entries(self.0.props.values()).finish()
            }
        }
        f.debug_struct("Layer")
            .field("name", &self.name)
            .field("items", &Items(self))
            .finish()
    }
}

impl Layer {
    fn try_clone(&self) -> Option<Self> {
        let new_props = self
            .props
            .iter()
            .flat_map(|(tyid, erased)| erased.try_clone().map(|e| (*tyid, e)))
            .collect::<TypeIdMap<_>>();
        if new_props.len() == self.props.len() {
            Some(Layer {
                name: self.name.clone(),
                props: new_props,
            })
        } else {
            None
        }
    }

    /// Inserts `value` into the layer directly
    fn put_directly<T: Store>(&mut self, value: T::StoredType) -> &mut Self {
        self.props
            .insert(TypeId::of::<T::StoredType>(), TypeErasedBox::new(value));
        self
    }

    /// Returns true if this layer is empty.
    pub fn is_empty(&self) -> bool {
        self.props.is_empty()
    }

    /// Converts this layer into a frozen layer that can no longer be mutated.
    pub fn freeze(self) -> FrozenLayer {
        self.into()
    }

    /// Create a new Layer with a given name
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        let name = name.into();
        Self {
            name,
            props: Default::default(),
        }
    }

    /// Changes the name of this layer.
    pub fn with_name(self, name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            props: self.props,
        }
    }

    /// Load a storable item from the bag
    pub fn load<T: Storable>(&self) -> <T::Storer as Store>::ReturnedType<'_> {
        T::Storer::merge_iter(ItemIter {
            inner: BagIter {
                head: Some(self),
                tail: [].iter().rev(),
            },
            t: Default::default(),
        })
    }

    /// Remove `T` from this bag
    pub fn unset<T: Send + Sync + Debug + 'static>(&mut self) -> &mut Self {
        self.put_directly::<StoreReplace<T>>(Value::ExplicitlyUnset(type_name::<T>()));
        self
    }

    /// Stores `item` of type `T` into the config bag, overriding a previous value of the same type
    pub fn store_put<T>(&mut self, item: T) -> &mut Self
    where
        T: Storable<Storer = StoreReplace<T>>,
    {
        self.put_directly::<StoreReplace<T>>(Value::Set(item));
        self
    }

    /// Stores `item` of type `T` into the config bag, overriding a previous value of the same type,
    /// or unsets it by passing a `None`
    pub fn store_or_unset<T>(&mut self, item: Option<T>) -> &mut Self
    where
        T: Storable<Storer = StoreReplace<T>>,
    {
        let item = match item {
            Some(item) => Value::Set(item),
            None => Value::ExplicitlyUnset(type_name::<T>()),
        };
        self.put_directly::<StoreReplace<T>>(item);
        self
    }

    /// This can only be used for types that use [`StoreAppend`]
    /// ```
    /// use aws_smithy_types::config_bag::{ConfigBag, Layer, Storable, StoreAppend, StoreReplace};
    /// let mut layer_1 = Layer::new("example");
    /// #[derive(Debug, PartialEq, Eq)]
    /// struct Interceptor(&'static str);
    /// impl Storable for Interceptor {
    ///     type Storer = StoreAppend<Interceptor>;
    /// }
    ///
    /// layer_1.store_append(Interceptor("321"));
    /// layer_1.store_append(Interceptor("654"));
    ///
    /// let mut layer_2 = Layer::new("second layer");
    /// layer_2.store_append(Interceptor("987"));
    ///
    /// let bag = ConfigBag::of_layers(vec![layer_1, layer_2]);
    ///
    /// assert_eq!(
    ///     bag.load::<Interceptor>().collect::<Vec<_>>(),
    ///     vec![&Interceptor("987"), &Interceptor("654"), &Interceptor("321")]
    /// );
    /// ```
    pub fn store_append<T>(&mut self, item: T) -> &mut Self
    where
        T: Storable<Storer = StoreAppend<T>>,
    {
        match self.get_mut_or_default::<StoreAppend<T>>() {
            Value::Set(list) => list.push(item),
            v @ Value::ExplicitlyUnset(_) => *v = Value::Set(vec![item]),
        }
        self
    }

    /// Clears the value of type `T` from the config bag
    ///
    /// This internally marks the item of type `T` as cleared as opposed to wiping it out from the
    /// config bag.
    pub fn clear<T>(&mut self)
    where
        T: Storable<Storer = StoreAppend<T>>,
    {
        self.put_directly::<StoreAppend<T>>(Value::ExplicitlyUnset(type_name::<T>()));
    }

    /// Retrieves the value of type `T` from this layer if exists
    fn get<T: Send + Sync + Store + 'static>(&self) -> Option<&T::StoredType> {
        self.props
            .get(&TypeId::of::<T::StoredType>())
            .map(|t| t.downcast_ref().expect("typechecked"))
    }

    /// Returns a mutable reference to `T` if it is stored in this layer
    fn get_mut<T: Send + Sync + Store + 'static>(&mut self) -> Option<&mut T::StoredType> {
        self.props
            .get_mut(&TypeId::of::<T::StoredType>())
            .map(|t| t.downcast_mut().expect("typechecked"))
    }

    /// Returns a mutable reference to `T` if it is stored in this layer, otherwise returns the
    /// [`Default`] implementation of `T`
    fn get_mut_or_default<T: Send + Sync + Store + 'static>(&mut self) -> &mut T::StoredType
    where
        T::StoredType: Default,
    {
        self.props
            .entry(TypeId::of::<T::StoredType>())
            .or_insert_with(|| TypeErasedBox::new(T::StoredType::default()))
            .downcast_mut()
            .expect("typechecked")
    }
}

/// Layered configuration structure
///
/// See the [module docs](crate::config_bag) for more documentation.
#[must_use]
pub struct ConfigBag {
    interceptor_state: Layer,
    tail: Vec<FrozenLayer>,
}

impl Debug for ConfigBag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        struct Layers<'a>(&'a ConfigBag);
        impl Debug for Layers<'_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_list().entries(self.0.layers()).finish()
            }
        }
        f.debug_struct("ConfigBag")
            .field("layers", &Layers(self))
            .finish()
    }
}

impl ConfigBag {
    /// Create a new config bag "base".
    pub fn base() -> Self {
        ConfigBag {
            interceptor_state: Layer {
                name: Cow::Borrowed("interceptor_state"),
                props: Default::default(),
            },
            tail: vec![],
        }
    }

    /// Create a [`ConfigBag`] consisting of the given layers.
    pub fn of_layers(layers: impl IntoIterator<Item = Layer>) -> Self {
        let mut bag = ConfigBag::base();
        for layer in layers {
            bag.push_layer(layer);
        }
        bag
    }

    /// Add the given layer to the config bag.
    pub fn push_layer(&mut self, layer: Layer) -> &mut Self {
        self.tail.push(layer.freeze());
        self
    }

    /// Add a frozen/shared layer to the config bag.
    pub fn push_shared_layer(&mut self, layer: FrozenLayer) -> &mut Self {
        self.tail.push(layer);
        self
    }

    /// Return a mutable reference to the interceptor state.
    pub fn interceptor_state(&mut self) -> &mut Layer {
        &mut self.interceptor_state
    }

    /// Load a value (or values) of type `T` depending on how `T` implements [`Storable`]
    pub fn load<T: Storable>(&self) -> <T::Storer as Store>::ReturnedType<'_> {
        self.sourced_get::<T::Storer>()
    }

    /// Return a mutable reference to `T` if found within the interceptor state.
    ///
    /// This method does not search the tail of the config bag (i.e., the collection of frozen layers).
    ///
    /// If the caller is unsure whether `T` exists in the interceptor state,
    /// consider using [`Self::get_mut`] instead, which requires `T` to implement `Clone`, however.
    pub fn get_mut_from_interceptor_state<T>(&mut self) -> Option<&mut T>
    where
        T: Storable<Storer = StoreReplace<T>> + Send + Sync + Debug + 'static,
    {
        match self.interceptor_state.get_mut::<StoreReplace<T>>() {
            Some(Value::ExplicitlyUnset(_)) => None,
            Some(Value::Set(t)) => Some(t),
            None => None,
        }
    }

    /// Return a mutable reference to `T` if found within the config bag
    ///
    /// This method requires `T` to implement `Clone` because, if the requested item is found in the tail of the config bag,
    /// it will be cloned and moved to the head (i.e., the interceptor state).
    ///
    /// If the caller is certain that `T` already exists in the interceptor state,
    /// consider using [`Self::get_mut_from_interceptor_state`] instead, which does not require `T` to be `Clone`.
    pub fn get_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Storable<Storer = StoreReplace<T>> + Send + Sync + Debug + Clone + 'static,
    {
        if self
            .interceptor_state
            .get_mut::<StoreReplace<T>>()
            .is_none()
        {
            let new_item = match self.tail.iter().find_map(|b| b.load::<T>()) {
                Some(item) => item.clone(),
                None => return None,
            };
            self.interceptor_state.store_put(new_item);
        }
        self.get_mut_from_interceptor_state()
    }

    /// Returns a mutable reference to `T` if it is stored in the top layer of the bag
    ///
    /// - If `T` is in a deeper layer of the bag, that value will be cloned and inserted into the top layer
    /// - If `T` is not present in the bag, the [`Default`] implementation will be used.
    pub fn get_mut_or_default<T>(&mut self) -> &mut T
    where
        T: Storable<Storer = StoreReplace<T>> + Send + Sync + Debug + Clone + Default + 'static,
    {
        self.get_mut_or_else(|| T::default())
    }

    /// Returns a mutable reference to `T` if it is stored in the top layer of the bag
    ///
    /// - If `T` is in a deeper layer of the bag, that value will be cloned and inserted into the top layer
    /// - If `T` is not present in the bag, `default` will be used to construct a new value
    pub fn get_mut_or_else<T>(&mut self, default: impl Fn() -> T) -> &mut T
    where
        T: Storable<Storer = StoreReplace<T>> + Send + Sync + Debug + Clone + 'static,
    {
        // this code looks weird to satisfy the borrow checker—we can't keep the result of `get_mut`
        // alive (even in a returned branch) and then call `store_put`. So: drop the borrow immediately
        // store, the value, then pull it right back
        if self.get_mut::<T>().is_none() {
            self.interceptor_state.store_put((default)());
            return self
                .get_mut()
                .expect("item was just stored in the top layer");
        }
        // above it was None
        self.get_mut().unwrap()
    }

    /// Add another layer to this configuration bag
    ///
    /// Hint: If you want to re-use this layer, call `freeze` first.
    ///
    /// # Examples
    /// ```
    /// use aws_smithy_types::config_bag::{ConfigBag, Layer, Storable, StoreReplace};
    ///
    /// #[derive(Debug, Eq, PartialEq)]
    /// struct ExampleStr(&'static str);
    /// impl Storable for ExampleStr {
    ///     type Storer = StoreReplace<Self>;
    /// }
    ///
    /// #[derive(Debug, Eq, PartialEq)]
    /// struct ExampleInt(i32);
    /// impl Storable for ExampleInt {
    ///     type Storer = StoreReplace<Self>;
    /// }
    ///
    /// let mut bag = ConfigBag::base();
    /// bag = bag.with_fn("first", |layer: &mut Layer| { layer.store_put(ExampleStr("a")); });
    ///
    /// // We can now load the example string out
    /// assert_eq!(bag.load::<ExampleStr>(), Some(&ExampleStr("a")));
    ///
    /// // But there isn't a number stored in the bag yet
    /// assert_eq!(bag.load::<ExampleInt>(), None);
    ///
    /// // Add a layer with an example int
    /// bag = bag.with_fn("second", |layer: &mut Layer| { layer.store_put(ExampleInt(1)); });
    ///
    /// // Now the example int can be retrieved
    /// assert_eq!(bag.load::<ExampleInt>(), Some(&ExampleInt(1)));
    /// ```
    pub fn with_fn(
        self,
        name: impl Into<Cow<'static, str>>,
        next: impl Fn(&mut Layer),
    ) -> ConfigBag {
        let mut new_layer = Layer::new(name);
        next(&mut new_layer);
        let ConfigBag {
            interceptor_state: head,
            mut tail,
        } = self;
        tail.push(head.freeze());
        ConfigBag {
            interceptor_state: new_layer,
            tail,
        }
    }

    /// Add a new layer with `name` after freezing the top layer so far
    pub fn add_layer(self, name: impl Into<Cow<'static, str>>) -> ConfigBag {
        self.with_fn(name, |_| {})
    }

    /// Return a value (or values) of type `T` depending on how it has been stored in a `ConfigBag`
    ///
    /// It flexibly chooses to return a single value vs. an iterator of values depending on how
    /// `T` implements a [`Store`] trait.
    pub fn sourced_get<T: Store>(&self) -> T::ReturnedType<'_> {
        let stored_type_iter = ItemIter {
            inner: self.layers(),
            t: PhantomData,
        };
        T::merge_iter(stored_type_iter)
    }

    fn layers(&self) -> BagIter<'_> {
        BagIter {
            head: Some(&self.interceptor_state),
            tail: self.tail.iter().rev(),
        }
    }
}

/// Iterator of items returned from [`ConfigBag`].
pub struct ItemIter<'a, T> {
    inner: BagIter<'a>,
    t: PhantomData<T>,
}

impl<T> Debug for ItemIter<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ItemIter")
    }
}

impl<'a, T: 'a> Iterator for ItemIter<'a, T>
where
    T: Store,
{
    type Item = &'a T::StoredType;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(layer) => layer.get::<T>().or_else(|| self.next()),
            None => None,
        }
    }
}

/// Iterator over the layers of a config bag
struct BagIter<'a> {
    head: Option<&'a Layer>,
    tail: Rev<Iter<'a, FrozenLayer>>,
}

impl<'a> Iterator for BagIter<'a> {
    type Item = &'a Layer;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(head) = self.head.take() {
            Some(head)
        } else {
            self.tail.next().map(|t| t.deref())
        }
    }
}

#[cfg(test)]
mod test {
    use super::ConfigBag;
    use crate::config_bag::{CloneableLayer, Layer, Storable, StoreAppend, StoreReplace};

    #[test]
    fn layered_property_bag() {
        #[derive(Debug)]
        struct Prop1;
        impl Storable for Prop1 {
            type Storer = StoreReplace<Self>;
        }
        #[derive(Debug)]
        struct Prop2;
        impl Storable for Prop2 {
            type Storer = StoreReplace<Self>;
        }
        let layer_a = |bag: &mut Layer| {
            bag.store_put(Prop1);
        };

        let layer_b = |bag: &mut Layer| {
            bag.store_put(Prop2);
        };

        #[derive(Debug)]
        struct Prop3;
        impl Storable for Prop3 {
            type Storer = StoreReplace<Self>;
        }

        let mut base_bag = ConfigBag::base()
            .with_fn("a", layer_a)
            .with_fn("b", layer_b);
        base_bag.interceptor_state().store_put(Prop3);
        assert!(base_bag.load::<Prop1>().is_some());

        #[derive(Debug)]
        struct Prop4;
        impl Storable for Prop4 {
            type Storer = StoreReplace<Self>;
        }

        let layer_c = |bag: &mut Layer| {
            bag.store_put(Prop4);
            bag.unset::<Prop3>();
        };

        let final_bag = base_bag.with_fn("c", layer_c);

        assert!(final_bag.load::<Prop4>().is_some());
        assert!(final_bag.load::<Prop1>().is_some());
        assert!(final_bag.load::<Prop2>().is_some());
        // we unset prop3
        assert!(final_bag.load::<Prop3>().is_none());
        println!("{final_bag:#?}");
    }

    #[test]
    fn config_bag() {
        let bag = ConfigBag::base();
        #[derive(Debug)]
        struct Region(&'static str);
        impl Storable for Region {
            type Storer = StoreReplace<Self>;
        }
        let bag = bag.with_fn("service config", |layer: &mut Layer| {
            layer.store_put(Region("asdf"));
        });

        assert_eq!(bag.load::<Region>().unwrap().0, "asdf");

        #[derive(Debug)]
        struct SigningName(&'static str);
        impl Storable for SigningName {
            type Storer = StoreReplace<Self>;
        }
        let operation_config = bag.with_fn("operation", |layer: &mut Layer| {
            layer.store_put(SigningName("s3"));
        });

        assert_eq!(operation_config.load::<SigningName>().unwrap().0, "s3");

        #[derive(Debug)]
        struct Prop;
        impl Storable for Prop {
            type Storer = StoreReplace<Self>;
        }
        let mut open_bag = operation_config.with_fn("my_custom_info", |_bag: &mut Layer| {});
        open_bag.interceptor_state().store_put(Prop);

        assert_eq!(open_bag.layers().count(), 4);
    }

    #[test]
    fn store_append() {
        let mut layer = Layer::new("test");
        #[derive(Debug, PartialEq, Eq)]
        struct Interceptor(&'static str);
        impl Storable for Interceptor {
            type Storer = StoreAppend<Interceptor>;
        }

        layer.clear::<Interceptor>();
        // you can only call store_append because interceptor is marked with a vec
        layer.store_append(Interceptor("123"));
        layer.store_append(Interceptor("456"));

        let mut second_layer = Layer::new("next");
        second_layer.store_append(Interceptor("789"));

        let mut bag = ConfigBag::of_layers(vec![layer, second_layer]);

        assert_eq!(
            bag.load::<Interceptor>().collect::<Vec<_>>(),
            vec![
                &Interceptor("789"),
                &Interceptor("456"),
                &Interceptor("123")
            ]
        );

        let mut final_layer = Layer::new("final");
        final_layer.clear::<Interceptor>();
        bag.push_layer(final_layer);
        assert_eq!(bag.load::<Interceptor>().count(), 0);
    }

    #[test]
    fn store_append_many_layers() {
        #[derive(Debug, PartialEq, Eq, Clone)]
        struct TestItem(i32, i32);
        impl Storable for TestItem {
            type Storer = StoreAppend<TestItem>;
        }
        let mut expected = vec![];
        let mut bag = ConfigBag::base();
        for layer_idx in 0..100 {
            let mut layer = Layer::new(format!("{layer_idx}"));
            for item in 0..100 {
                expected.push(TestItem(layer_idx, item));
                layer.store_append(TestItem(layer_idx, item));
            }
            bag.push_layer(layer);
        }
        expected.reverse();
        assert_eq!(
            bag.load::<TestItem>().cloned().collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn adding_layers() {
        let mut layer_1 = Layer::new("layer1");

        let mut layer_2 = Layer::new("layer2");

        #[derive(Clone, Debug, PartialEq, Eq, Default)]
        struct Foo(usize);
        impl Storable for Foo {
            type Storer = StoreReplace<Foo>;
        }

        layer_1.store_put(Foo(0));
        layer_2.store_put(Foo(1));

        let layer_1 = layer_1.freeze();
        let layer_2 = layer_2.freeze();

        let mut bag_1 = ConfigBag::base();
        let mut bag_2 = ConfigBag::base();
        bag_1
            .push_shared_layer(layer_1.clone())
            .push_shared_layer(layer_2.clone());
        bag_2.push_shared_layer(layer_2).push_shared_layer(layer_1);

        // bags have same layers but in different orders
        assert_eq!(bag_1.load::<Foo>(), Some(&Foo(1)));
        assert_eq!(bag_2.load::<Foo>(), Some(&Foo(0)));

        bag_1.interceptor_state().store_put(Foo(3));
        assert_eq!(bag_1.load::<Foo>(), Some(&Foo(3)));
    }

    #[test]
    fn get_mut_or_else() {
        #[derive(Clone, Debug, PartialEq, Eq, Default)]
        struct Foo(usize);
        impl Storable for Foo {
            type Storer = StoreReplace<Foo>;
        }

        let mut bag = ConfigBag::base();
        assert_eq!(bag.get_mut::<Foo>(), None);
        assert_eq!(bag.get_mut_or_default::<Foo>(), &Foo(0));
        bag.get_mut_or_default::<Foo>().0 += 1;
        assert_eq!(bag.load::<Foo>(), Some(&Foo(1)));

        let old_ref = bag.load::<Foo>().unwrap();
        assert_eq!(old_ref, &Foo(1));

        // there is one in the bag, so it can be returned
        //let mut next = bag.add_layer("next");
        bag.get_mut::<Foo>().unwrap().0 += 1;
        let new_ref = bag.load::<Foo>().unwrap();
        assert_eq!(new_ref, &Foo(2));

        bag.interceptor_state().unset::<Foo>();
        // if it was unset, we can't clone the current one, that would be wrong
        assert_eq!(bag.get_mut::<Foo>(), None);
        assert_eq!(bag.get_mut_or_default::<Foo>(), &Foo(0));
    }

    #[test]
    fn get_mut_from_interceptor_state() {
        // excludes `Clone` to verify `get_mut_from_interceptor_state` works with types without the `Clone` trait
        #[derive(Debug, PartialEq, Eq, Default)]
        struct Foo(usize);
        impl Storable for Foo {
            type Storer = StoreReplace<Self>;
        }

        #[derive(Clone, Debug, PartialEq, Eq, Default)]
        struct Bar(usize);
        impl Storable for Bar {
            type Storer = StoreReplace<Self>;
        }

        let mut bag = ConfigBag::base();
        assert!(bag.get_mut_from_interceptor_state::<Foo>().is_none());

        bag.interceptor_state().store_put(Foo(1));
        assert_eq!(
            bag.get_mut_from_interceptor_state::<Foo>(),
            Some(&mut Foo(1))
        );

        // `Bar` is stored in the tail of the config bag.
        let mut layer = Layer::new("test");
        layer.store_put(Bar(1));
        bag.push_layer(layer);

        // the method under test should not find `Bar`, as it does not reside in the interceptor state.
        assert!(bag.get_mut_from_interceptor_state::<Bar>().is_none());
    }

    #[test]
    fn cloning_layers() {
        #[derive(Clone, Debug)]
        struct TestStr(String);
        impl Storable for TestStr {
            type Storer = StoreReplace<TestStr>;
        }
        let mut layer_1 = CloneableLayer::new("layer_1");
        let expected_str = "I can be cloned";
        layer_1.store_put(TestStr(expected_str.to_owned()));
        let layer_1_cloned = layer_1.clone();
        assert_eq!(expected_str, &layer_1_cloned.load::<TestStr>().unwrap().0);

        // Should still be cloneable after unsetting a field
        layer_1.unset::<TestStr>();
        assert!(layer_1.try_clone().unwrap().load::<TestStr>().is_none());

        // It is cloneable multiple times in succession
        let _ = layer_1
            .try_clone()
            .expect("clone 1")
            .try_clone()
            .expect("clone 2");

        #[derive(Clone, Debug)]
        struct Rope(String);
        impl Storable for Rope {
            type Storer = StoreAppend<Rope>;
        }
        let mut layer_2 = CloneableLayer::new("layer_2");
        layer_2.store_append(Rope("A".to_owned()));
        layer_2.store_append(Rope("big".to_owned()));
        layer_2.store_append(Rope("rope".to_owned()));
        let layer_2_cloned = layer_2.clone();
        let rope = layer_2_cloned.load::<Rope>().cloned().collect::<Vec<_>>();
        assert_eq!(
            "A big rope",
            rope.iter()
                .rev()
                .map(|r| r.0.clone())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
