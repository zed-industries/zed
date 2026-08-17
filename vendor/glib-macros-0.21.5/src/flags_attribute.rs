// Take a look at the license at the top of the repository in the LICENSE file.

use heck::{ToKebabCase, ToShoutySnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, Ident, Variant, Visibility,
};

use crate::utils::{
    crate_ident_new, parse_nested_meta_items, parse_optional_nested_meta_items, NestedMetaItem,
};

pub const WRONG_PLACE_MSG: &str = "#[glib::flags] only supports enums";

pub struct AttrInput {
    pub enum_name: syn::LitStr,
    pub allow_name_conflict: bool,
}
struct FlagsDesc {
    variant: Variant,
    name: Option<String>,
    nick: Option<String>,
    skip: bool,
}
impl FlagsDesc {
    fn from_attrs(variant: Variant, attrs: &[Attribute]) -> syn::Result<Self> {
        let mut name = NestedMetaItem::<syn::LitStr>::new("name").value_required();
        let mut nick = NestedMetaItem::<syn::LitStr>::new("nick").value_required();
        let mut skip = NestedMetaItem::<syn::LitBool>::new("skip").value_optional();

        parse_nested_meta_items(attrs, "flags_value", &mut [&mut name, &mut nick, &mut skip])?;

        Ok(Self {
            variant,
            name: name.value.map(|s| s.value()),
            nick: nick.value.map(|s| s.value()),
            skip: skip.found || skip.value.map(|b| b.value()).unwrap_or(false),
        })
    }
}

// Generate glib::gobject_ffi::GFlagsValue structs mapping the enum such as:
//     glib::gobject_ffi::GFlagsValue {
//         value: MyFlags::A.bits(),
//         value_name: "The Name\0" as *const _ as *const _,
//         value_nick: "nick\0" as *const _ as *const _,
//     },
fn gen_flags_values(
    enum_name: &Ident,
    enum_variants: &Punctuated<Variant, Comma>,
) -> (TokenStream, usize) {
    let crate_ident = crate_ident_new();

    // start at one as GFlagsValue array is null-terminated
    let mut n = 1;
    let recurse = enum_variants
        .iter()
        .map(|v| FlagsDesc::from_attrs(v.clone(), &v.attrs).unwrap())
        .filter(|desc| !desc.skip)
        .map(|desc| {
            let v = desc.variant;
            let name = &v.ident;
            let mut value_name = name.to_string().to_upper_camel_case();
            let mut value_nick = name.to_string().to_kebab_case();

            if let Some(n) = desc.name {
                value_name = n;
            }
            if let Some(n) = desc.nick {
                value_nick = n;
            }

            let value_name = format!("{value_name}\0");
            let value_nick = format!("{value_nick}\0");

            n += 1;
            quote_spanned! {v.span()=>
                #crate_ident::gobject_ffi::GFlagsValue {
                    value: #enum_name::#name.bits(),
                    value_name: #value_name as *const _ as *const _,
                    value_nick: #value_nick as *const _ as *const _,
                },
            }
        });
    (
        quote! {
            #(#recurse)*
        },
        n,
    )
}

fn gen_bitflags(
    enum_name: &Ident,
    visibility: &Visibility,
    enum_variants: &Punctuated<Variant, Comma>,
    crate_ident: &TokenStream,
) -> TokenStream {
    let recurse = enum_variants.iter().map(|v| {
        let name = &v.ident;
        let disc = v.discriminant.as_ref().expect("missing discriminant");
        let value = &disc.1;

        quote_spanned! {v.span()=>
            const #name = #value;
        }
    });

    quote! {
        #crate_ident::bitflags::bitflags! {
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
            #visibility struct #enum_name: u32 {
                #(#recurse)*
            }
        }
    }
}

fn gen_default(
    enum_name: &Ident,
    enum_variants: &Punctuated<Variant, Comma>,
) -> Option<TokenStream> {
    enum_variants
        .iter()
        .find(|v| v.attrs.iter().any(|attr| attr.path().is_ident("default")))
        .map(|v| {
            let default_value = &v.ident;

            quote! {
                impl Default for #enum_name {
                    fn default() -> Self {
                        Self::from_bits_retain(#enum_name::#default_value.bits())
                    }
                }
            }
        })
}

pub fn impl_flags(attr_meta: AttrInput, input: &mut syn::ItemEnum) -> TokenStream {
    let gtype_name = attr_meta.enum_name;

    let syn::ItemEnum {
        attrs,
        ident: name,
        vis: visibility,
        ..
    } = input;

    let enum_variants = &input.variants;
    let (g_flags_values, nb_flags_values) = gen_flags_values(name, enum_variants);

    let crate_ident = crate_ident_new();

    let mut plugin_type = NestedMetaItem::<syn::Path>::new("plugin_type").value_required();
    let mut lazy_registration =
        NestedMetaItem::<syn::LitBool>::new("lazy_registration").value_required();

    let found = parse_optional_nested_meta_items(
        &*attrs,
        "flags_dynamic",
        &mut [&mut plugin_type, &mut lazy_registration],
    );

    let register_flags = match found {
        Err(e) => return e.to_compile_error(),
        Ok(None) => register_flags_as_static(
            &crate_ident,
            name,
            gtype_name,
            attr_meta.allow_name_conflict,
            g_flags_values,
            nb_flags_values,
        ),
        Ok(Some(_)) => {
            if attr_meta.allow_name_conflict {
                return syn::Error::new_spanned(
                    input,
                    "#[flags_dynamic] and #[glib::flags(allow_name_conflict)] are not allowed together",
                ).to_compile_error();
            }

            // remove attribute 'flags_dynamic' from the attribute list because it is not a real proc_macro_attribute
            attrs.retain(|attr| !attr.path().is_ident("flags_dynamic"));
            let plugin_ty = plugin_type
                .value
                .map(|p| p.into_token_stream())
                .unwrap_or(quote!(#crate_ident::TypeModule));
            let lazy_registration = lazy_registration.value.map(|b| b.value).unwrap_or_default();
            register_flags_as_dynamic(
                &crate_ident,
                plugin_ty,
                lazy_registration,
                name,
                gtype_name,
                g_flags_values,
                nb_flags_values,
            )
        }
    };

    let bitflags = gen_bitflags(name, visibility, enum_variants, &crate_ident);
    let default_impl = gen_default(name, enum_variants);

    quote! {
        #bitflags

        #default_impl

        impl #crate_ident::translate::IntoGlib for #name {
            type GlibType = u32;

            #[inline]
            fn into_glib(self) -> u32 {
                self.bits()
            }
        }

        impl #crate_ident::translate::FromGlib<u32> for #name {
            #[inline]
            unsafe fn from_glib(value: u32) -> Self {
                Self::from_bits_truncate(value)
            }
        }

        impl #crate_ident::value::ValueType for #name {
            type Type = Self;
        }

        unsafe impl<'a> #crate_ident::value::FromValue<'a> for #name {
            type Checker = #crate_ident::value::GenericValueTypeChecker<Self>;

            #[inline]
            unsafe fn from_value(value: &'a #crate_ident::value::Value) -> Self {
                #crate_ident::translate::from_glib(#crate_ident::gobject_ffi::g_value_get_flags(
                    #crate_ident::translate::ToGlibPtr::to_glib_none(value).0
                ))
            }
        }

        impl #crate_ident::value::ToValue for #name {
            #[inline]
            fn to_value(&self) -> #crate_ident::value::Value {
                let mut value = #crate_ident::value::Value::for_value_type::<Self>();
                unsafe {
                    #crate_ident::gobject_ffi::g_value_set_flags(
                        #crate_ident::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        #crate_ident::translate::IntoGlib::into_glib(*self)
                    )
                }
                value
            }

            #[inline]
            fn value_type(&self) -> #crate_ident::Type {
                <Self as #crate_ident::prelude::StaticType>::static_type()
            }
        }

        impl #crate_ident::HasParamSpec for #name {
            type ParamSpec = #crate_ident::ParamSpecFlags;
            type SetValue = Self;
            type BuilderFn = fn(&::core::primitive::str) -> #crate_ident::ParamSpecFlagsBuilder<Self>;

            fn param_spec_builder() -> Self::BuilderFn {
                |name| Self::ParamSpec::builder(name)
            }
        }

        impl ::std::convert::From<#name> for #crate_ident::Value {
            #[inline]
            fn from(v: #name) -> Self {
                #crate_ident::value::ToValue::to_value(&v)
            }
        }

        impl #crate_ident::prelude::StaticType for #name {
            #[inline]
            fn static_type() -> #crate_ident::Type {
                Self::register_flags()
            }
        }

        #register_flags
    }
}

// Registers the flags as a static type.
fn register_flags_as_static(
    crate_ident: &TokenStream,
    name: &syn::Ident,
    gtype_name: syn::LitStr,
    allow_name_conflict: bool,
    g_flags_values: TokenStream,
    nb_flags_values: usize,
) -> TokenStream {
    let type_name_snippet = if allow_name_conflict {
        quote! {
            unsafe {
                let mut i = 0;
                loop {
                    let type_name = ::std::ffi::CString::new(if i == 0 {
                        #gtype_name
                    } else {
                        format!("{}-{}", #gtype_name, i)
                    })
                    .unwrap();
                    if #crate_ident::gobject_ffi::g_type_from_name(type_name.as_ptr()) == #crate_ident::gobject_ffi::G_TYPE_INVALID
                    {
                        break type_name;
                    }
                    i += 1;
                }
            }
        }
    } else {
        quote! {
            unsafe {
                let type_name = ::std::ffi::CString::new(#gtype_name).unwrap();
                assert_eq!(
                    #crate_ident::gobject_ffi::g_type_from_name(type_name.as_ptr()),
                    #crate_ident::gobject_ffi::G_TYPE_INVALID,
                    "Type {} has already been registered",
                    type_name.to_str().unwrap()
                );

                type_name
            }
        }
    };

    // registers the flags on first use (lazy registration).
    quote! {
        impl #name {
            /// Registers the flags only once.
            #[inline]
            fn register_flags() -> #crate_ident::Type {
                static TYPE: ::std::sync::OnceLock<#crate_ident::Type> = ::std::sync::OnceLock::new();
                *TYPE.get_or_init(|| {
                    static mut VALUES: [#crate_ident::gobject_ffi::GFlagsValue; #nb_flags_values] = [
                        #g_flags_values
                        #crate_ident::gobject_ffi::GFlagsValue {
                            value: 0,
                            value_name: ::std::ptr::null(),
                            value_nick: ::std::ptr::null(),
                        },
                    ];

                    let type_name = #type_name_snippet;
                    unsafe {
                        let type_ = #crate_ident::gobject_ffi::g_flags_register_static(type_name.as_ptr(), VALUES.as_ptr());
                        let type_: #crate_ident::Type = #crate_ident::translate::from_glib(type_);
                        assert!(type_.is_valid());
                        type_
                    }
                })
            }
        }
    }
}

// The following implementations follows the lifecycle of plugins and of dynamic types (see [`TypePluginExt`] and [`TypeModuleExt`]).
// Flags can be reregistered as a dynamic type.
fn register_flags_as_dynamic(
    crate_ident: &TokenStream,
    plugin_ty: TokenStream,
    lazy_registration: bool,
    name: &syn::Ident,
    gtype_name: syn::LitStr,
    g_flags_values: TokenStream,
    nb_flags_values: usize,
) -> TokenStream {
    // Wrap each GFlagsValue to FlagsValue
    let g_flags_values_expr: syn::ExprArray = syn::parse_quote! { [#g_flags_values] };
    let flags_values_iter = g_flags_values_expr.elems.iter().map(|v| {
        quote_spanned! {syn::spanned::Spanned::span(&v)=>
            #crate_ident::FlagsValue::unsafe_from(#v),
        }
    });

    let flags_values = quote! {
        #crate_ident::enums::FlagsValuesStorage<#nb_flags_values> = unsafe {
            #crate_ident::enums::FlagsValuesStorage::<#nb_flags_values>::new([
                #(#flags_values_iter)*
            ])
        }
    };

    // The following implementations follows the lifecycle of plugins and of dynamic types (see [`TypePluginExt`] and [`TypeModuleExt`]).
    // Flags can be reregistered as a dynamic type.
    if lazy_registration {
        // registers the flags as a dynamic type on the first use (lazy registration).
        // a weak reference on the plugin is stored and will be used later on the first use of the flags.
        // this implementation relies on a static storage of a weak reference on the plugin and of the GLib type to know if the flags have been registered.

        // the registration status type.
        let registration_status_type = format_ident!("{}RegistrationStatus", name);
        // name of the static variable to store the registration status.
        let registration_status = format_ident!(
            "{}",
            registration_status_type.to_string().to_shouty_snake_case()
        );
        // name of the static array to store the flags values.
        let flags_values_array =
            format_ident!("{}_VALUES", name.to_string().to_shouty_snake_case());

        quote! {
            /// The registration status type: a tuple of the weak reference on the plugin and of the GLib type.
            struct #registration_status_type(<#plugin_ty as #crate_ident::clone::Downgrade>::Weak, #crate_ident::Type);
            unsafe impl Send for #registration_status_type {}

            /// The registration status protected by a mutex guarantees so that no other threads are concurrently accessing the data.
            static #registration_status: ::std::sync::Mutex<Option<#registration_status_type>> = ::std::sync::Mutex::new(None);

            /// Array of `FlagsValue` for the possible flags values.
            static #flags_values_array: #flags_values;

            impl #name {
                /// Registers the flags as a dynamic type within the plugin only once.
                /// Plugin must have been used at least once.
                /// Do nothing if plugin has never been used or if the flags are already registered as a dynamic type.
                #[inline]
                fn register_flags() -> #crate_ident::Type {
                    let mut registration_status = #registration_status.lock().unwrap();
                    match ::std::ops::DerefMut::deref_mut(&mut registration_status) {
                        // plugin has never been used, so the flags cannot be registered as a dynamic type.
                        None => #crate_ident::Type::INVALID,
                        // plugin has been used and the flags have not been registered yet, so registers tem as a dynamic type.
                        Some(#registration_status_type(type_plugin, type_)) if !type_.is_valid() => {
                            *type_ = <#plugin_ty as glib::prelude::DynamicObjectRegisterExt>::register_dynamic_flags(type_plugin.upgrade().unwrap().as_ref(), #gtype_name, #flags_values_array.as_ref());
                            *type_
                        },
                        // plugin has been used and the flags have already been registered as a dynamic type.
                        Some(#registration_status_type(_, type_)) => *type_
                    }
                }

                /// Depending on the plugin lifecycle state and on the registration status of the flags:
                /// If plugin is used (and has loaded the implementation) for the first time, postpones the registration and stores a weak reference on the plugin.
                /// If plugin is reused (and has reloaded the implementation) and the flags have been already registered as a dynamic type, reregisters them.
                /// Flags can be reregistered several times as a dynamic type.
                /// If plugin is reused (and has reloaded the implementation) and the flags have not been registered yet as a dynamic type, do nothing.
                #[inline]
                pub fn on_implementation_load(type_plugin: &#plugin_ty) -> bool {
                    let mut registration_status = #registration_status.lock().unwrap();
                    match ::std::ops::DerefMut::deref_mut(&mut registration_status) {
                        // plugin has never been used (this is the first time), so postpones registration of the flags as a dynamic type on the first use.
                        None => {
                            *registration_status = Some(#registration_status_type(#crate_ident::clone::Downgrade::downgrade(type_plugin), #crate_ident::Type::INVALID));
                            true
                        },
                        // plugin has been used at least one time and the flags have been registered as a dynamic type at least one time, so re-registers them.
                        Some(#registration_status_type(_, type_)) if type_.is_valid() => {
                            *type_ = <#plugin_ty as glib::prelude::DynamicObjectRegisterExt>::register_dynamic_flags(type_plugin, #gtype_name, #flags_values_array.as_ref());
                            type_.is_valid()
                        },
                        // plugin has been used at least one time but the flags have not been registered yet as a dynamic type, so keeps postponed registration.
                        Some(_) => {
                            true
                        }
                    }
                }

                /// Depending on the plugin lifecycle state and on the registration status of the flags:
                /// If plugin has been used (or reused) but the flags have not been registered yet as a dynamic type, cancels the postponed registration by deleting the weak reference on the plugin.
                /// Else do nothing.
                #[inline]
                pub fn on_implementation_unload(type_plugin_: &#plugin_ty) -> bool {
                    let mut registration_status = #registration_status.lock().unwrap();
                    match ::std::ops::DerefMut::deref_mut(&mut registration_status) {
                        // plugin has never been used, so unload implementation is unexpected.
                        None => false,
                        // plugin has been used at least one time and the flags have been registered as a dynamic type at least one time.
                        Some(#registration_status_type(_, type_)) if type_.is_valid() => true,
                        // plugin has been used at least one time but the flags have not been registered yet as a dynamic type, so cancels the postponed registration.
                        Some(_) => {
                            *registration_status = None;
                            true
                        }
                    }
                }
            }
        }
    } else {
        // registers immediately the flags as a dynamic type.

        // name of the static variable to store the GLib type.
        let gtype_status = format_ident!("{}_G_TYPE", name.to_string().to_shouty_snake_case());

        quote! {
            /// The GLib type which can be safely shared between threads.
            static #gtype_status: ::std::sync::atomic::AtomicUsize = ::std::sync::atomic::AtomicUsize::new(#crate_ident::gobject_ffi::G_TYPE_INVALID);

            impl #name {
                /// Do nothing as the flags has been registered on implementation load.
                #[inline]
                fn register_flags() -> #crate_ident::Type {
                    let gtype = #gtype_status.load(::std::sync::atomic::Ordering::Acquire);
                    unsafe { <#crate_ident::Type as #crate_ident::translate::FromGlib<#crate_ident::ffi::GType>>::from_glib(gtype) }
                }

                /// Registers the flags as a dynamic type within the plugin.
                /// The flags can be registered several times as a dynamic type.
                #[inline]
                pub fn on_implementation_load(type_plugin: &#plugin_ty) -> bool {
                    static VALUES: #flags_values;
                    let gtype = #crate_ident::translate::IntoGlib::into_glib(<#plugin_ty as glib::prelude::DynamicObjectRegisterExt>::register_dynamic_flags(type_plugin, #gtype_name, VALUES.as_ref()));
                    #gtype_status.store(gtype, ::std::sync::atomic::Ordering::Release);
                    gtype != #crate_ident::gobject_ffi::G_TYPE_INVALID
                }

                /// Do nothing as flags registered as dynamic types are never unregistered.
                #[inline]
                pub fn on_implementation_unload(type_plugin_: &#plugin_ty) -> bool {
                    true
                }
            }
        }
    }
}
