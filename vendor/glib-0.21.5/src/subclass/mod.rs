// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::needless_doctest_main)]
// rustdoc-stripper-ignore-next
//! Module containing infrastructure for subclassing `GObject`s and registering boxed types.
//!
//! # Example for registering a `glib::Object` subclass
//!
//! The following code implements a subclass of `glib::Object` with a
//! string-typed "name" property.
//!
//! ```rust
//! use glib::prelude::*;
//! use glib::subclass;
//! use glib::subclass::prelude::*;
//! use glib::{Variant, VariantType};
//!
//! use std::cell::{Cell, RefCell};
//!
//! #[derive(Debug, Eq, PartialEq, Clone, Copy, glib::Enum)]
//! #[repr(u32)]
//! // type_name: GType name of the enum (mandatory)
//! #[enum_type(name = "SimpleObjectAnimal")]
//! enum Animal {
//!     Goat = 0,
//!     #[enum_value(name = "The Dog")]
//!     Dog = 1,
//!     // name: the name of the GEnumValue (optional), default to the enum name in CamelCase
//!     // nick: the nick of the GEnumValue (optional), default to the enum name in kebab-case
//!     #[enum_value(name = "The Cat", nick = "chat")]
//!     Cat = 2,
//! }
//!
//! impl Default for Animal {
//!     fn default() -> Self {
//!         Animal::Goat
//!     }
//! }
//!
//! #[glib::flags(name = "MyFlags")]
//! enum MyFlags {
//!     #[flags_value(name = "Flag A", nick = "nick-a")]
//!     A = 0b00000001,
//!     #[flags_value(name = "Flag B")]
//!     B = 0b00000010,
//!     #[flags_value(skip)]
//!     AB = Self::A.bits() | Self::B.bits(),
//!     C = 0b00000100,
//! }
//!
//! impl Default for MyFlags {
//!     fn default() -> Self {
//!         MyFlags::A
//!     }
//! }
//!
//! mod imp {
//!     use super::*;
//!
//!     // This is the struct containing all state carried with
//!     // the new type. Generally this has to make use of
//!     // interior mutability.
//!     // If it implements the `Default` trait, then `Self::default()`
//!     // will be called every time a new instance is created.
//!     #[derive(Default)]
//!     pub struct SimpleObject {
//!         name: RefCell<Option<String>>,
//!         animal: Cell<Animal>,
//!         flags: Cell<MyFlags>,
//!         variant: RefCell<Option<Variant>>,
//!     }
//!
//!     // ObjectSubclass is the trait that defines the new type and
//!     // contains all information needed by the GObject type system,
//!     // including the new type's name, parent type, etc.
//!     // If you do not want to implement `Default`, you can provide
//!     // a `new()` method.
//!     #[glib::object_subclass]
//!     impl ObjectSubclass for SimpleObject {
//!         // This type name must be unique per process.
//!         const NAME: &'static str = "SimpleObject";
//!
//!         type Type = super::SimpleObject;
//!
//!         // The parent type this one is inheriting from.
//!         // Optional, if not specified it defaults to `glib::Object`
//!         type ParentType = glib::Object;
//!
//!         // Interfaces this type implements.
//!         // Optional, if not specified it defaults to `()`
//!         type Interfaces = ();
//!     }
//!
//!     // Trait that is used to override virtual methods of glib::Object.
//!     impl ObjectImpl for SimpleObject {
//!         // Called once in the very beginning to list all properties of this class.
//!         fn properties() -> &'static [glib::ParamSpec] {
//!             use std::sync::OnceLock;
//!             static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();
//!             PROPERTIES.get_or_init(|| {
//!                 vec![
//!                     glib::ParamSpecString::builder("name")
//!                         .build(),
//!                     glib::ParamSpecEnum::builder::<Animal>("animal")
//!                         .build(),
//!                     glib::ParamSpecFlags::builder::<MyFlags>("flags")
//!                         .build(),
//!                     glib::ParamSpecVariant::builder("variant", glib::VariantTy::ANY)
//!                         .build(),
//!                 ]
//!             })
//!         }
//!
//!         // Called whenever a property is set on this instance. The id
//!         // is the same as the index of the property in the PROPERTIES array.
//!         fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
//!             match pspec.name() {
//!                 "name" => {
//!                     let name = value
//!                         .get()
//!                         .expect("type conformity checked by `Object::set_property`");
//!                     self.name.replace(name);
//!                 },
//!                 "animal" => {
//!                     let animal = value
//!                         .get()
//!                         .expect("type conformity checked by `Object::set_property`");
//!                     self.animal.replace(animal);
//!                 },
//!                 "flags" => {
//!                     let flags = value
//!                         .get()
//!                         .expect("type conformity checked by `Object::set_property`");
//!                     self.flags.replace(flags);
//!                 },
//!                 "variant" => {
//!                     let variant = value
//!                         .get()
//!                         .expect("type conformity checked by `Object::set_property`");
//!                     self.variant.replace(variant);
//!                 },
//!                 _ => unimplemented!(),
//!             }
//!         }
//!
//!         // Called whenever a property is retrieved from this instance. The id
//!         // is the same as the index of the property in the PROPERTIES array.
//!         fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
//!             match pspec.name() {
//!                 "name" => self.name.borrow().to_value(),
//!                 "animal" => self.animal.get().to_value(),
//!                 "flags" => self.flags.get().to_value(),
//!                 "variant" => self.variant.borrow().to_value(),
//!                 _ => unimplemented!(),
//!             }
//!         }
//!
//!         // Called right after construction of the instance.
//!         fn constructed(&self) {
//!             // Chain up to the parent type's implementation of this virtual
//!             // method.
//!             self.parent_constructed();
//!
//!             // And here we could do our own initialization.
//!         }
//!     }
//! }
//!
//! // Optionally, define a wrapper type to make it more ergonomic to use from Rust
//! glib::wrapper! {
//!     pub struct SimpleObject(ObjectSubclass<imp::SimpleObject>);
//! }
//!
//! impl SimpleObject {
//!     // Create an object instance of the new type.
//!     pub fn new() -> Self {
//!         glib::Object::new()
//!     }
//! }
//!
//! pub fn main() {
//!     let obj = SimpleObject::new();
//!
//!     // Get the name property and change its value.
//!     assert_eq!(obj.property::<Option<String>>("name"), None);
//!     obj.set_property("name", "test");
//!     assert_eq!(&obj.property::<String>("name"), "test");
//!
//!     assert_eq!(obj.property::<Animal>("animal"), Animal::Goat);
//!     obj.set_property("animal", Animal::Cat);
//!     assert_eq!(obj.property::<Animal>("animal"), Animal::Cat);
//!
//!     assert_eq!(obj.property::<MyFlags>("flags"), MyFlags::A);
//!     obj.set_property("flags", MyFlags::B);
//!     assert_eq!(obj.property::<MyFlags>("flags"), MyFlags::B);
//! }
//! ```
//!
//! # Example for registering a `glib::Object` subclass within a module
//!
//! The following code implements a subclass of `glib::Object` and registers it as
//! a dynamic type.
//!
//! ```rust
//! use glib::prelude::*;
//! use glib::subclass::prelude::*;
//!
//! pub mod imp {
//!     use super::*;
//!
//!     // SimpleModuleObject is a dynamic type.
//!     #[derive(Default)]
//!     pub struct SimpleModuleObject;
//!
//!     #[glib::object_subclass]
//!     #[object_subclass_dynamic]
//!     impl ObjectSubclass for SimpleModuleObject {
//!         const NAME: &'static str = "SimpleModuleObject";
//!         type Type = super::SimpleModuleObject;
//!     }
//!
//!     impl ObjectImpl for SimpleModuleObject {}
//!
//!     // SimpleTypeModule is the type module within the object subclass is registered as a dynamic type.
//!     #[derive(Default)]
//!     pub struct SimpleTypeModule;
//!
//!     #[glib::object_subclass]
//!     impl ObjectSubclass for SimpleTypeModule {
//!         const NAME: &'static str = "SimpleTypeModule";
//!         type Type = super::SimpleTypeModule;
//!         type ParentType = glib::TypeModule;
//!         type Interfaces = (glib::TypePlugin,);
//!     }
//!
//!     impl ObjectImpl for SimpleTypeModule {}
//!
//!     impl TypeModuleImpl for SimpleTypeModule {
//!         /// Loads the module and registers the object subclass as a dynamic type.
//!         fn load(&self) -> bool {
//!             SimpleModuleObject::on_implementation_load(self.obj().upcast_ref::<glib::TypeModule>())
//!         }
//!
//!         /// Unloads the module.
//!         fn unload(&self) {
//!             SimpleModuleObject::on_implementation_unload(self.obj().upcast_ref::<glib::TypeModule>());
//!         }
//!     }
//!
//!     impl TypePluginImpl for SimpleTypeModule {}
//! }
//!
//! // Optionally, defines a wrapper type to make SimpleModuleObject more ergonomic to use from Rust.
//! glib::wrapper! {
//!     pub struct SimpleModuleObject(ObjectSubclass<imp::SimpleModuleObject>);
//! }
//!
//! // Optionally, defines a wrapper type to make SimpleTypeModule more ergonomic to use from Rust.
//! glib::wrapper! {
//!     pub struct SimpleTypeModule(ObjectSubclass<imp::SimpleTypeModule>)
//!     @extends glib::TypeModule, @implements glib::TypePlugin;
//! }
//!
//! impl SimpleTypeModule {
//!     // Creates an object instance of the new type.
//!     pub fn new() -> Self {
//!         glib::Object::new()
//!     }
//! }
//!
//! pub fn main() {
//!     let simple_type_module = SimpleTypeModule::new();
//!     // at this step, SimpleTypeModule has not been loaded therefore
//!     // SimpleModuleObject must not be registered yet.
//!     let simple_module_object_type = imp::SimpleModuleObject::type_();
//!     assert!(!simple_module_object_type.is_valid());
//!
//!     // simulates the GLib type system to load the module.
//!     TypeModuleExt::use_(&simple_type_module);
//!
//!     // at this step, SimpleModuleObject must have been registered.
//!     let simple_module_object_type = imp::SimpleModuleObject::type_();
//!     assert!(simple_module_object_type.is_valid());
//! }
//! ```
//!
//! # Example for registering a `glib::Object` subclass within a plugin
//!
//! The following code implements a subclass of `glib::Object` and registers it as
//! a dynamic type.
//!
//! ```rust
//! use glib::prelude::*;
//! use glib::subclass::prelude::*;
//!
//! pub mod imp {
//!     use super::*;
//!
//!     // SimplePluginObject is a dynamic type.
//!     #[derive(Default)]
//!     pub struct SimplePluginObject;
//!
//!     #[glib::object_subclass]
//!     #[object_subclass_dynamic(plugin_type = super::SimpleTypePlugin)]
//!     impl ObjectSubclass for SimplePluginObject {
//!         const NAME: &'static str = "SimplePluginObject";
//!         type Type = super::SimplePluginObject;
//!     }
//!
//!     impl ObjectImpl for SimplePluginObject {}
//!
//!     // SimpleTypePlugin is the type plugin within the object subclass is registered as a dynamic type.
//!     #[derive(Default)]
//!     pub struct SimpleTypePlugin {
//!         type_info: std::cell::Cell<Option<glib::TypeInfo>>
//!     }
//!
//!     #[glib::object_subclass]
//!     impl ObjectSubclass for SimpleTypePlugin {
//!         const NAME: &'static str = "SimpleTypePlugin";
//!         type Type = super::SimpleTypePlugin;
//!         type Interfaces = (glib::TypePlugin,);
//!     }
//!
//!     impl ObjectImpl for SimpleTypePlugin {}
//!
//!     impl TypePluginImpl for SimpleTypePlugin {
//!         /// Uses the plugin and registers the object subclass as a dynamic type.
//!         fn use_plugin(&self) {
//!             SimplePluginObject::on_implementation_load(self.obj().as_ref());
//!         }
//!
//!         /// Unuses the plugin.
//!         fn unuse_plugin(&self) {
//!             SimplePluginObject::on_implementation_unload(self.obj().as_ref());
//!         }
//!
//!         /// Returns type information about the object subclass registered as a dynamic type.
//!         fn complete_type_info(&self, _type_: glib::Type) -> (glib::TypeInfo, glib::TypeValueTable) {
//!             assert!(self.type_info.get().is_some());
//!             // returns type info.
//!             (self.type_info.get().unwrap(), glib::TypeValueTable::default())
//!         }
//!     }
//!
//!     impl TypePluginRegisterImpl for SimpleTypePlugin {
//!         fn register_dynamic_type(&self, parent_type: glib::Type, type_name: &str, type_info: &glib::TypeInfo, flags: glib::TypeFlags) -> glib::Type {
//!             let type_ = glib::Type::from_name(type_name).unwrap_or_else(|| {
//!                 glib::Type::register_dynamic(parent_type, type_name, self.obj().upcast_ref::<glib::TypePlugin>(), flags)
//!             });
//!             if type_.is_valid() {
//!                 // saves type info.
//!                 self.type_info.set(Some(*type_info));
//!             }
//!             type_
//!         }
//!     }
//! }
//!
//! // Optionally, defines a wrapper type to make SimplePluginObject more ergonomic to use from Rust.
//! glib::wrapper! {
//!     pub struct SimplePluginObject(ObjectSubclass<imp::SimplePluginObject>);
//! }
//!
//! // Optionally, defines a wrapper type to make SimpleTypePlugin more ergonomic to use from Rust.
//! glib::wrapper! {
//!     pub struct SimpleTypePlugin(ObjectSubclass<imp::SimpleTypePlugin>)
//!     @implements glib::TypePlugin;
//! }
//!
//! impl SimpleTypePlugin {
//!     // Creates an object instance of the new type.
//!     pub fn new() -> Self {
//!         glib::Object::new()
//!     }
//! }
//!
//! pub fn main() {
//!     let simple_type_plugin = SimpleTypePlugin::new();
//!     // at this step, SimpleTypePlugin has not been used therefore
//!     // SimplePluginObject must not be registered yet.
//!     let simple_plugin_object_type = imp::SimplePluginObject::type_();
//!     assert!(!simple_plugin_object_type.is_valid());
//!
//!     // simulates the GLib type system to use the plugin.
//!     TypePluginExt::use_(&simple_type_plugin);
//!
//!     // at this step, SimplePluginObject must have been registered.
//!     let simple_plugin_object_type = imp::SimplePluginObject::type_();
//!     assert!(simple_plugin_object_type.is_valid());
//! }
//! ```
//!
//!//! # Example for registering a boxed type for a Rust struct
//!
//! The following code boxed type for a tuple struct around `String` and uses it in combination
//! with `glib::Value`.
//!
//! ```rust
//! use glib::prelude::*;
//! use glib::subclass;
//! use glib::subclass::prelude::*;
//!
//! #[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
//! #[boxed_type(name = "MyBoxed")]
//! struct MyBoxed(String);
//!
//! pub fn main() {
//!     assert!(MyBoxed::static_type().is_valid());
//!
//!     let b = MyBoxed(String::from("abc"));
//!     let v = b.to_value();
//!     let b2 = v.get::<&MyBoxed>().unwrap();
//!     assert_eq!(&b, b2);
//! }
//! ```

pub mod basic;
#[macro_use]
pub mod types;

#[macro_use]
pub mod interface;

#[macro_use]
pub mod object;

#[macro_use]
pub mod boxed;

pub mod shared;

pub mod signal;

mod object_impl_ref;
pub use object_impl_ref::{ObjectImplRef, ObjectImplWeakRef};

pub mod type_module;

pub mod type_plugin;

pub mod prelude {
    // rustdoc-stripper-ignore-next
    //! Prelude that re-exports all important traits from this crate.
    pub use super::{
        boxed::BoxedType,
        interface::{ObjectInterface, ObjectInterfaceExt, ObjectInterfaceType},
        object::{DerivedObjectProperties, ObjectClassSubclassExt, ObjectImpl, ObjectImplExt},
        shared::{RefCounted, SharedType},
        type_module::{TypeModuleImpl, TypeModuleImplExt},
        type_plugin::{TypePluginImpl, TypePluginImplExt, TypePluginRegisterImpl},
        types::{
            ClassStruct, InstanceStruct, InstanceStructExt, InterfaceStruct, IsImplementable,
            IsSubclassable, IsSubclassableExt, ObjectSubclass, ObjectSubclassExt,
            ObjectSubclassIsExt, ObjectSubclassType,
        },
    };
}

pub use self::{
    boxed::register_boxed_type,
    interface::{register_dynamic_interface, register_interface},
    signal::{
        Signal, SignalClassHandlerToken, SignalId, SignalInvocationHint, SignalQuery, SignalType,
    },
    types::{register_dynamic_type, register_type, InitializingObject, InitializingType, TypeData},
};
