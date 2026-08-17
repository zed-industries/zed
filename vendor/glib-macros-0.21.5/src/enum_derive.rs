// Take a look at the license at the top of the repository in the LICENSE file.

use heck::{ToKebabCase, ToShoutySnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};

use crate::utils::{
    crate_ident_new, gen_enum_from_glib, parse_nested_meta_items, parse_optional_nested_meta_items,
    NestedMetaItem,
};

// generates glib::gobject_ffi::GEnumValue structs mapping the enum such as:
//     glib::gobject_ffi::GEnumValue {
//         value: Animal::Goat as i32,
//         value_name: "Goat\0" as *const _ as *const _,
//         value_nick: "goat\0" as *const _ as *const _,
//     },
fn gen_enum_values(
    enum_name: &syn::Ident,
    enum_variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> (TokenStream, usize) {
    let crate_ident = crate_ident_new();

    // starts at one as GEnumValue array is null-terminated.
    let mut n = 1;
    let recurse = enum_variants.iter().map(|v| {
        let name = &v.ident;
        let mut value_name = name.to_string().to_upper_camel_case();
        let mut value_nick = name.to_string().to_kebab_case();

        let mut name_attr = NestedMetaItem::<syn::LitStr>::new("name").value_required();
        let mut nick = NestedMetaItem::<syn::LitStr>::new("nick").value_required();

        let found =
            parse_nested_meta_items(&v.attrs, "enum_value", &mut [&mut name_attr, &mut nick]);
        if let Err(e) = found {
            return e.to_compile_error();
        }

        value_name = name_attr.value.map(|s| s.value()).unwrap_or(value_name);
        value_nick = nick.value.map(|s| s.value()).unwrap_or(value_nick);

        let value_name = format!("{value_name}\0");
        let value_nick = format!("{value_nick}\0");

        n += 1;
        // generates a glib::gobject_ffi::GEnumValue.
        quote_spanned! {syn::spanned::Spanned::span(&v)=>
            #crate_ident::gobject_ffi::GEnumValue {
                value: #enum_name::#name as i32,
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

pub fn impl_enum(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;

    let enum_variants = match input.data {
        syn::Data::Enum(ref e) => &e.variants,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "#[derive(glib::Enum)] only supports enums",
            ))
        }
    };
    let (g_enum_values, nb_enum_values) = gen_enum_values(name, enum_variants);

    let mut gtype_name = NestedMetaItem::<syn::LitStr>::new("name")
        .required()
        .value_required();
    let mut allow_name_conflict =
        NestedMetaItem::<syn::LitBool>::new("allow_name_conflict").value_optional();
    let found = parse_nested_meta_items(
        &input.attrs,
        "enum_type",
        &mut [&mut gtype_name, &mut allow_name_conflict],
    )?;

    if found.is_none() {
        return Err(syn::Error::new_spanned(
            input,
            "#[derive(glib::Enum)] requires #[enum_type(name = \"EnumTypeName\")]",
        ));
    }
    let gtype_name = gtype_name.value.unwrap();
    let allow_name_conflict = allow_name_conflict.found
        || allow_name_conflict
            .value
            .map(|b| b.value())
            .unwrap_or(false);

    let mut plugin_type = NestedMetaItem::<syn::Path>::new("plugin_type").value_required();
    let mut lazy_registration =
        NestedMetaItem::<syn::LitBool>::new("lazy_registration").value_required();

    let found = parse_optional_nested_meta_items(
        &input.attrs,
        "enum_dynamic",
        &mut [&mut plugin_type, &mut lazy_registration],
    )?;

    let crate_ident = crate_ident_new();

    let register_enum = match found {
        None => register_enum_as_static(
            &crate_ident,
            name,
            gtype_name,
            allow_name_conflict,
            g_enum_values,
            nb_enum_values,
        ),
        Some(_) => {
            if allow_name_conflict {
                return Err(syn::Error::new_spanned(
                    input,
                    "#[enum_dynamic] and #[enum_type(allow_name_conflict)] are not allowed together",
                ));
            }

            let plugin_ty = plugin_type
                .value
                .map(|p| p.into_token_stream())
                .unwrap_or(quote!(#crate_ident::TypeModule));
            let lazy_registration = lazy_registration.value.map(|b| b.value).unwrap_or_default();
            register_enum_as_dynamic(
                &crate_ident,
                plugin_ty,
                lazy_registration,
                name,
                gtype_name,
                g_enum_values,
                nb_enum_values,
            )
        }
    };

    let from_glib = gen_enum_from_glib(name, enum_variants);

    Ok(quote! {
        impl #crate_ident::translate::IntoGlib for #name {
            type GlibType = i32;

            #[inline]
            fn into_glib(self) -> i32 {
                self as i32
            }
        }

        impl #crate_ident::translate::TryFromGlib<i32> for #name {
            type Error = i32;

            #[inline]
            unsafe fn try_from_glib(value: i32) -> ::core::result::Result<Self, i32> {
                let from_glib = || {
                    #from_glib
                };

                from_glib().ok_or(value)
            }
        }

        impl #crate_ident::translate::FromGlib<i32> for #name {
            #[inline]
            unsafe fn from_glib(value: i32) -> Self {
                use #crate_ident::translate::TryFromGlib;

                Self::try_from_glib(value).unwrap()
            }
        }

        impl #crate_ident::value::ValueType for #name {
            type Type = Self;
        }

        unsafe impl<'a> #crate_ident::value::FromValue<'a> for #name {
            type Checker = #crate_ident::value::GenericValueTypeChecker<Self>;

            #[inline]
            unsafe fn from_value(value: &'a #crate_ident::value::Value) -> Self {
                #crate_ident::translate::from_glib(#crate_ident::gobject_ffi::g_value_get_enum(
                    #crate_ident::translate::ToGlibPtr::to_glib_none(value).0
                ))
            }
        }

        impl #crate_ident::prelude::ToValue for #name {
            #[inline]
            fn to_value(&self) -> #crate_ident::value::Value {
                let mut value = #crate_ident::value::Value::for_value_type::<Self>();
                unsafe {
                    #crate_ident::gobject_ffi::g_value_set_enum(
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

        impl ::std::convert::From<#name> for #crate_ident::Value {
            #[inline]
            fn from(v: #name) -> Self {
                #crate_ident::value::ToValue::to_value(&v)
            }
        }

        impl #crate_ident::prelude::StaticType for #name {
            #[inline]
            fn static_type() -> #crate_ident::Type {
                Self::register_enum()
            }
        }

        #register_enum

        impl #crate_ident::HasParamSpec for #name {
            type ParamSpec = #crate_ident::ParamSpecEnum;
            type SetValue = Self;
            type BuilderFn = fn(&::core::primitive::str, Self) -> #crate_ident::ParamSpecEnumBuilder<Self>;

            fn param_spec_builder() -> Self::BuilderFn {
                |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
            }
        }
    })
}

// Registers the enum as a static type.
fn register_enum_as_static(
    crate_ident: &TokenStream,
    name: &syn::Ident,
    gtype_name: syn::LitStr,
    allow_name_conflict: bool,
    g_enum_values: TokenStream,
    nb_enum_values: usize,
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

    // registers the enum on first use (lazy registration).
    quote! {
        impl #name {
            /// Registers the enum only once.
            #[inline]
            fn register_enum() -> #crate_ident::Type {
                static TYPE: ::std::sync::OnceLock<#crate_ident::Type> = ::std::sync::OnceLock::new();
                *TYPE.get_or_init(|| {
                    static mut VALUES: [#crate_ident::gobject_ffi::GEnumValue; #nb_enum_values] = [
                        #g_enum_values
                        #crate_ident::gobject_ffi::GEnumValue {
                            value: 0,
                            value_name: ::std::ptr::null(),
                            value_nick: ::std::ptr::null(),
                        },
                    ];
                    let type_name = #type_name_snippet;
                    unsafe {
                        let type_ = #crate_ident::gobject_ffi::g_enum_register_static(type_name.as_ptr(), VALUES.as_ptr());
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
// An enum can be reregistered as a dynamic type.
fn register_enum_as_dynamic(
    crate_ident: &TokenStream,
    plugin_ty: TokenStream,
    lazy_registration: bool,
    name: &syn::Ident,
    gtype_name: syn::LitStr,
    g_enum_values: TokenStream,
    nb_enum_values: usize,
) -> TokenStream {
    // Wrap each GEnumValue to EnumValue
    let g_enum_values_expr: syn::ExprArray = syn::parse_quote! { [#g_enum_values] };
    let enum_values_iter = g_enum_values_expr.elems.iter().map(|v| {
        quote_spanned! {syn::spanned::Spanned::span(&v)=>
            #crate_ident::EnumValue::unsafe_from(#v),
        }
    });

    let enum_values = quote! {
        #crate_ident::enums::EnumValuesStorage<#nb_enum_values> = unsafe {
            #crate_ident::enums::EnumValuesStorage::<#nb_enum_values>::new([
                #(#enum_values_iter)*
            ])
        }
    };

    // The following implementations follows the lifecycle of plugins and of dynamic types (see [`TypePluginExt`] and [`TypeModuleExt`]).
    // An enum can be reregistered as a dynamic type.
    if lazy_registration {
        // registers the enum as a dynamic type on the first use (lazy registration).
        // a weak reference on the plugin is stored and will be used later on the first use of the enum.
        // this implementation relies on a static storage of a weak reference on the plugin and of the GLib type to know if the enum has been registered.

        // the registration status type.
        let registration_status_type = format_ident!("{}RegistrationStatus", name);
        // name of the static variable to store the registration status.
        let registration_status = format_ident!(
            "{}",
            registration_status_type.to_string().to_shouty_snake_case()
        );
        // name of the static array to store the enumeration values.
        let enum_values_array = format_ident!("{}_VALUES", name.to_string().to_shouty_snake_case());

        quote! {
            /// The registration status type: a tuple of the weak reference on the plugin and of the GLib type.
            struct #registration_status_type(<#plugin_ty as #crate_ident::clone::Downgrade>::Weak, #crate_ident::Type);
            unsafe impl Send for #registration_status_type {}

            /// The registration status protected by a mutex guarantees so that no other threads are concurrently accessing the data.
            static #registration_status: ::std::sync::Mutex<Option<#registration_status_type>> = ::std::sync::Mutex::new(None);

            /// Array of `EnumValue` for the possible enumeration values.
            static #enum_values_array: #enum_values;

            impl #name {
                /// Registers the enum as a dynamic type within the plugin only once.
                /// Plugin must have been used at least once.
                /// Do nothing if plugin has never been used or if the enum is already registered as a dynamic type.
                #[inline]
                fn register_enum() -> #crate_ident::Type {
                    let mut registration_status = #registration_status.lock().unwrap();
                    match ::std::ops::DerefMut::deref_mut(&mut registration_status) {
                        // plugin has never been used, so the enum cannot be registered as a dynamic type.
                        None => #crate_ident::Type::INVALID,
                        // plugin has been used and the enum has not been registered yet, so registers it as a dynamic type.
                        Some(#registration_status_type(type_plugin, type_)) if !type_.is_valid() => {
                            *type_ = <#plugin_ty as glib::prelude::DynamicObjectRegisterExt>::register_dynamic_enum(type_plugin.upgrade().unwrap().as_ref(), #gtype_name, #enum_values_array.as_ref());
                            *type_
                        },
                        // plugin has been used and the enum has already been registered as a dynamic type.
                        Some(#registration_status_type(_, type_)) => *type_
                    }
                }

                /// Depending on the plugin lifecycle state and on the registration status of the enum:
                /// If plugin is used (and has loaded the implementation) for the first time, postpones the registration and stores a weak reference on the plugin.
                /// If plugin is reused (and has reloaded the implementation) and the enum has been already registered as a dynamic type, reregisters it.
                /// An enum can be reregistered several times as a dynamic type.
                /// If plugin is reused (and has reloaded the implementation) and the enum has not been registered yet as a dynamic type, do nothing.
                #[inline]
                pub fn on_implementation_load(type_plugin: &#plugin_ty) -> bool {
                    let mut registration_status = #registration_status.lock().unwrap();
                    match ::std::ops::DerefMut::deref_mut(&mut registration_status) {
                        // plugin has never been used (this is the first time), so postpones registration of the enum as a dynamic type on the first use.
                        None => {
                            *registration_status = Some(#registration_status_type(#crate_ident::clone::Downgrade::downgrade(type_plugin), #crate_ident::Type::INVALID));
                            true
                        },
                        // plugin has been used at least one time and the enum has been registered as a dynamic type at least one time, so re-registers it.
                        Some(#registration_status_type(_, type_)) if type_.is_valid() => {
                            *type_ = <#plugin_ty as glib::prelude::DynamicObjectRegisterExt>::register_dynamic_enum(type_plugin, #gtype_name, #enum_values_array.as_ref());
                            type_.is_valid()
                        },
                        // plugin has been used at least one time but the enum has not been registered yet as a dynamic type, so keeps postponed registration.
                        Some(_) => {
                            true
                        }
                    }
                }

                /// Depending on the plugin lifecycle state and on the registration status of the enum:
                /// If plugin has been used (or reused) but the enum has not been registered yet as a dynamic type, cancels the postponed registration by deleting the weak reference on the plugin.
                /// Else do nothing.
                #[inline]
                pub fn on_implementation_unload(type_plugin_: &#plugin_ty) -> bool {
                    let mut registration_status = #registration_status.lock().unwrap();
                    match ::std::ops::DerefMut::deref_mut(&mut registration_status) {
                        // plugin has never been used, so unload implementation is unexpected.
                        None => false,
                        // plugin has been used at least one time and the enum has been registered as a dynamic type at least one time.
                        Some(#registration_status_type(_, type_)) if type_.is_valid() => true,
                        // plugin has been used at least one time but the enum has not been registered yet as a dynamic type, so cancels the postponed registration.
                        Some(_) => {
                            *registration_status = None;
                            true
                        }
                    }
                }
            }
        }
    } else {
        // registers immediately the enum as a dynamic type.

        // name of the static variable to store the GLib type.
        let gtype_status = format_ident!("{}_G_TYPE", name.to_string().to_shouty_snake_case());

        quote! {
            /// The GLib type which can be safely shared between threads.
            static #gtype_status: ::std::sync::atomic::AtomicUsize = ::std::sync::atomic::AtomicUsize::new(#crate_ident::gobject_ffi::G_TYPE_INVALID);

            impl #name {
                /// Do nothing as the enum has been registered on implementation load.
                #[inline]
                fn register_enum() -> #crate_ident::Type {
                    let gtype = #gtype_status.load(::std::sync::atomic::Ordering::Acquire);
                    unsafe { <#crate_ident::Type as #crate_ident::translate::FromGlib<#crate_ident::ffi::GType>>::from_glib(gtype) }
                }

                /// Registers the enum as a dynamic type within the plugin.
                /// The enum can be registered several times as a dynamic type.
                #[inline]
                pub fn on_implementation_load(type_plugin: &#plugin_ty) -> bool {
                    static VALUES: #enum_values;
                    let gtype = #crate_ident::translate::IntoGlib::into_glib(<#plugin_ty as glib::prelude::DynamicObjectRegisterExt>::register_dynamic_enum(type_plugin, #gtype_name, VALUES.as_ref()));
                    #gtype_status.store(gtype, ::std::sync::atomic::Ordering::Release);
                    gtype != #crate_ident::gobject_ffi::G_TYPE_INVALID
                }

                /// Do nothing as enums registered as dynamic types are never unregistered.
                #[inline]
                pub fn on_implementation_unload(type_plugin_: &#plugin_ty) -> bool {
                    true
                }
            }
        }
    }
}
