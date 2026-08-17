use crate::utils::{PropertyEmitsChangedSignal, parse_crate_path, pat_ident, typed_arg, zbus_path};
use proc_macro2::{Literal, Span, TokenStream};
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::{
    Error, FnArg, Ident, ItemTrait, Meta, Path, ReturnType, Token, TraitItemFn, Visibility,
    fold::Fold, parse_quote, parse_str, punctuated::Punctuated, spanned::Spanned,
};
use zvariant_utils::{case, def_attrs};

def_attrs! {
    crate zbus;

    // Keep this in sync with interface's proxy method attributes.
    pub TraitAttributes("trait") {
        interface str,
        name str,
        assume_defaults bool,
        default_path str,
        default_service str,
        async_name str,
        blocking_name str,
        gen_async bool,
        gen_blocking bool,
        crate_path str
    };

    // Keep this in sync with interface's proxy method attributes.
    pub MethodAttributes("method") {
        name str,
        property {
            pub PropertyAttributes("property") {
                emits_changed_signal str
            }
        },
        signal none,
        object str,
        async_object str,
        blocking_object str,
        object_vec none,
        no_reply none,
        no_autostart none,
        allow_interactive_auth none
    };
}

struct AsyncOpts {
    blocking: bool,
    usage: TokenStream,
    wait: TokenStream,
}

impl AsyncOpts {
    fn new(blocking: bool) -> Self {
        let (usage, wait) = if blocking {
            (quote! {}, quote! {})
        } else {
            (quote! { async }, quote! { .await })
        };
        Self {
            blocking,
            usage,
            wait,
        }
    }
}

pub fn expand(args: Punctuated<Meta, Token![,]>, input: ItemTrait) -> Result<TokenStream, Error> {
    let attrs = TraitAttributes::parse_nested_metas(args)?;
    let crate_path = parse_crate_path(attrs.crate_path.as_deref())?;

    let iface_name = match (attrs.interface, attrs.name) {
        (Some(name), None) | (None, Some(name)) => Ok(Some(name)),
        (None, None) => Ok(None),
        (Some(_), Some(_)) => Err(syn::Error::new(
            input.span(),
            "both `interface` and `name` attributes shouldn't be specified at the same time",
        )),
    }?;
    let gen_async = attrs.gen_async.unwrap_or(true);
    #[cfg(feature = "blocking-api")]
    let gen_blocking = attrs.gen_blocking.unwrap_or(true);
    #[cfg(not(feature = "blocking-api"))]
    let gen_blocking = false;

    // Some sanity checks
    assert!(
        gen_blocking || gen_async,
        "Can't disable both asynchronous and blocking proxy. 😸",
    );
    assert!(
        gen_blocking || attrs.blocking_name.is_none(),
        "Can't set blocking proxy's name if you disabled it. 😸",
    );
    assert!(
        gen_async || attrs.async_name.is_none(),
        "Can't set asynchronous proxy's name if you disabled it. 😸",
    );

    let blocking_proxy = if gen_blocking {
        let proxy_name = attrs.blocking_name.unwrap_or_else(|| {
            if gen_async {
                format!("{}ProxyBlocking", input.ident)
            } else {
                // When only generating blocking proxy, there is no need for a suffix.
                format!("{}Proxy", input.ident)
            }
        });
        create_proxy(
            &input,
            iface_name.as_deref(),
            attrs.assume_defaults,
            attrs.default_path.as_deref(),
            attrs.default_service.as_deref(),
            &proxy_name,
            true,
            // Signal args structs are shared between the two proxies so always generate it for
            // async proxy only unless async proxy generation is disabled.
            !gen_async,
            crate_path.as_ref(),
        )?
    } else {
        quote! {}
    };
    let async_proxy = if gen_async {
        let proxy_name = attrs
            .async_name
            .unwrap_or_else(|| format!("{}Proxy", input.ident));
        create_proxy(
            &input,
            iface_name.as_deref(),
            attrs.assume_defaults,
            attrs.default_path.as_deref(),
            attrs.default_service.as_deref(),
            &proxy_name,
            false,
            true,
            crate_path.as_ref(),
        )?
    } else {
        quote! {}
    };

    Ok(quote! {
        #blocking_proxy

        #async_proxy
    })
}

#[allow(clippy::too_many_arguments)]
pub fn create_proxy(
    input: &ItemTrait,
    iface_name: Option<&str>,
    assume_defaults: Option<bool>,
    default_path: Option<&str>,
    default_service: Option<&str>,
    proxy_name: &str,
    blocking: bool,
    gen_sig_args: bool,
    crate_path: Option<&syn::Path>,
) -> Result<TokenStream, Error> {
    let zbus = zbus_path(crate_path);

    let other_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| !a.path().is_ident("zbus"))
        .collect();
    let proxy_name = Ident::new(proxy_name, Span::call_site());
    let ident = input.ident.to_string();
    let iface_name = iface_name
        .map(|iface| {
            // Ensure the interface name is valid.
            zbus_names::InterfaceName::try_from(iface)
                .map_err(|e| Error::new(input.span(), format!("{e}")))
                .map(|i| i.to_string())
        })
        .transpose()?
        .unwrap_or(format!("org.freedesktop.{ident}"));
    let assume_defaults = assume_defaults.unwrap_or(false);
    let default_path = default_path
        .map(|path| {
            // Ensure the path is valid.
            zvariant::ObjectPath::try_from(path)
                .map_err(|e| Error::new(input.span(), format!("{e}")))
                .map(|p| p.to_string())
        })
        .transpose()?;
    let default_service = default_service
        .map(|srv| {
            // Ensure the service is valid.
            zbus_names::BusName::try_from(srv)
                .map_err(|e| Error::new(input.span(), format!("{e}")))
                .map(|n| n.to_string())
        })
        .transpose()?;
    let (default_path, default_service) = if assume_defaults {
        let path = default_path.or_else(|| Some(format!("/org/freedesktop/{ident}")));
        let svc = default_service.or_else(|| Some(iface_name.clone()));
        (path, svc)
    } else {
        (default_path, default_service)
    };
    let mut methods = TokenStream::new();
    let mut stream_types = TokenStream::new();
    let mut has_properties = false;
    let mut uncached_properties: Vec<String> = vec![];

    let async_opts = AsyncOpts::new(blocking);
    let visibility = &input.vis;

    for i in input.items.iter() {
        if let syn::TraitItem::Fn(m) = i {
            let method_attrs = MethodAttributes::parse(&m.attrs)?;
            let property = method_attrs.property.as_ref();

            let rust_method_name = m.sig.ident.to_string();
            let r#_stripped_rust_method_name = rust_method_name
                .strip_prefix("r#")
                .unwrap_or(rust_method_name.as_str());

            let is_signal = method_attrs.signal;
            let is_property = property.is_some();
            let has_inputs = m.sig.inputs.len() > 1;

            let dbus_member_name = method_attrs.name.clone().unwrap_or_else(|| {
                case::pascal_or_camel_case(
                    if is_property && has_inputs {
                        assert!(r#_stripped_rust_method_name.starts_with("set_"));
                        &r#_stripped_rust_method_name[4..]
                    } else {
                        r#_stripped_rust_method_name
                    },
                    true,
                )
            });

            let m = if let Some(prop_attrs) = property {
                has_properties = true;

                let emits_changed_signal = if let Some(s) = &prop_attrs.emits_changed_signal {
                    PropertyEmitsChangedSignal::parse(s, m.span())?
                } else {
                    PropertyEmitsChangedSignal::True
                };

                if let PropertyEmitsChangedSignal::False = emits_changed_signal {
                    uncached_properties.push(dbus_member_name.clone());
                }

                gen_proxy_property(
                    &dbus_member_name,
                    r#_stripped_rust_method_name,
                    m,
                    &method_attrs,
                    &async_opts,
                    emits_changed_signal,
                    &zbus,
                )
            } else if is_signal {
                let (method, types) = gen_proxy_signal(
                    &proxy_name,
                    &iface_name,
                    &dbus_member_name,
                    r#_stripped_rust_method_name,
                    m,
                    &async_opts,
                    visibility,
                    gen_sig_args,
                    &zbus,
                );
                stream_types.extend(types);

                method
            } else {
                gen_proxy_method_call(
                    &dbus_member_name,
                    &rust_method_name,
                    m,
                    method_attrs,
                    &async_opts,
                    &zbus,
                )?
            };
            methods.extend(m);
        }
    }

    let AsyncOpts { usage, wait, .. } = async_opts;
    let (proxy_struct, connection, builder, proxy_trait) = if blocking {
        let connection = quote! { #zbus::blocking::Connection };
        let proxy = quote! { #zbus::blocking::Proxy };
        let builder = quote! { #zbus::blocking::proxy::Builder };
        let proxy_trait = quote! { #zbus::blocking::proxy::ProxyImpl };

        (proxy, connection, builder, proxy_trait)
    } else {
        let connection = quote! { #zbus::Connection };
        let proxy = quote! { #zbus::Proxy };
        let builder = quote! { #zbus::proxy::Builder };
        let proxy_trait = quote! { #zbus::proxy::ProxyImpl };

        (proxy, connection, builder, proxy_trait)
    };

    let proxy_method_new = match (&default_path, &default_service) {
        (None, None) => {
            quote! {
                /// Creates a new proxy with the given service destination and path.
                pub #usage fn new<D, P>(conn: &#connection, destination: D, path: P) -> #zbus::Result<#proxy_name<'p>>
                where
                    D: ::std::convert::TryInto<#zbus::names::BusName<'p>>,
                    D::Error: ::std::convert::Into<#zbus::Error>,
                    P: ::std::convert::TryInto<#zbus::zvariant::ObjectPath<'p>>,
                    P::Error: ::std::convert::Into<#zbus::Error>,
                {
                    let obj_path = path.try_into().map_err(::std::convert::Into::into)?;
                    let obj_destination = destination.try_into().map_err(::std::convert::Into::into)?;
                    Self::builder(conn)
                        .path(obj_path)?
                        .destination(obj_destination)?
                        .build()#wait
                }
            }
        }
        (Some(_), None) => {
            quote! {
                /// Creates a new proxy with the given destination, and the default path.
                pub #usage fn new<D>(conn: &#connection, destination: D) -> #zbus::Result<#proxy_name<'p>>
                where
                    D: ::std::convert::TryInto<#zbus::names::BusName<'p>>,
                    D::Error: ::std::convert::Into<#zbus::Error>,
                {
                    let obj_dest = destination.try_into().map_err(::std::convert::Into::into)?;
                    Self::builder(conn)
                        .destination(obj_dest)?
                        .build()#wait
                }
            }
        }
        (None, Some(_)) => {
            quote! {
                /// Creates a new proxy with the given path, and the default destination.
                pub #usage fn new<P>(conn: &#connection, path: P) -> #zbus::Result<#proxy_name<'p>>
                where
                    P: ::std::convert::TryInto<#zbus::zvariant::ObjectPath<'p>>,
                    P::Error: ::std::convert::Into<#zbus::Error>,
                {
                    let obj_path = path.try_into().map_err(::std::convert::Into::into)?;
                    Self::builder(conn)
                        .path(obj_path)?
                        .build()#wait
                }
            }
        }
        (Some(_), Some(_)) => {
            quote! {
                /// Creates a new proxy with the default service and path.
                pub #usage fn new(conn: &#connection) -> #zbus::Result<#proxy_name<'p>> {
                    Self::builder(conn).build()#wait
                }
            }
        }
    };
    let default_path = match default_path {
        Some(p) => quote! { &Some(#zbus::zvariant::ObjectPath::from_static_str_unchecked(#p)) },
        None => quote! { &None },
    };
    let default_service = match default_service {
        Some(d) => {
            if d.starts_with(':') || d == "org.freedesktop.DBus" {
                quote! {
                    &Some(#zbus::names::BusName::Unique(
                        #zbus::names::UniqueName::from_static_str_unchecked(#d),
                    ))
                }
            } else {
                quote! {
                    &Some(#zbus::names::BusName::WellKnown(
                        #zbus::names::WellKnownName::from_static_str_unchecked(#d),
                    ))
                }
            }
        }
        None => quote! { &None },
    };

    Ok(quote! {
        impl<'a> #zbus::proxy::Defaults for #proxy_name<'a> {
            const INTERFACE: &'static Option<#zbus::names::InterfaceName<'static>> =
                &Some(#zbus::names::InterfaceName::from_static_str_unchecked(#iface_name));
            const DESTINATION: &'static Option<#zbus::names::BusName<'static>> = #default_service;
            const PATH: &'static Option<#zbus::zvariant::ObjectPath<'static>> = #default_path;
        }

        #(#other_attrs)*
        #[derive(Clone, Debug)]
        #visibility struct #proxy_name<'p>(#proxy_struct<'p>);

        impl<'p> #proxy_name<'p> {
            #proxy_method_new

            /// Returns a customizable builder for this proxy.
            pub fn builder(conn: &#connection) -> #builder<'p, Self> {
                let mut builder = #builder::new(conn) ;
                if #has_properties {
                    let uncached = vec![#(#uncached_properties),*];
                    builder.cache_properties(#zbus::proxy::CacheProperties::default())
                           .uncached_properties(&uncached)
                } else {
                    builder.cache_properties(#zbus::proxy::CacheProperties::No)
                }
            }

            /// Consumes `self`, returning the underlying `zbus::Proxy`.
            pub fn into_inner(self) -> #proxy_struct<'p> {
                self.0
            }

            /// The reference to the underlying `zbus::Proxy`.
            pub fn inner(&self) -> &#proxy_struct<'p> {
                &self.0
            }

            /// The mutable reference to the underlying `zbus::Proxy`.
            pub fn inner_mut(&mut self) -> &mut #proxy_struct<'p> {
                &mut self.0
            }

            #methods
        }

        impl<'p> #proxy_trait<'p> for #proxy_name<'p> {
            fn builder(conn: &#connection) -> #builder<'p, Self> {
                Self::builder(conn)
            }

            fn into_inner(self) -> #proxy_struct<'p> {
                self.into_inner()
            }

            fn inner(&self) -> &#proxy_struct<'p> {
                self.inner()
            }
        }

        impl<'p> ::std::convert::From<#zbus::Proxy<'p>> for #proxy_name<'p> {
            fn from(proxy: #zbus::Proxy<'p>) -> Self {
                #proxy_name(::std::convert::Into::into(proxy))
            }
        }

        impl<'p> ::std::convert::AsRef<#proxy_struct<'p>> for #proxy_name<'p> {
            fn as_ref(&self) -> &#proxy_struct<'p> {
                self.inner()
            }
        }

        impl<'p> ::std::convert::AsMut<#proxy_struct<'p>> for #proxy_name<'p> {
            fn as_mut(&mut self) -> &mut #proxy_struct<'p> {
                self.inner_mut()
            }
        }

        impl<'p> #zbus::zvariant::Type for #proxy_name<'p> {
            const SIGNATURE: &'static #zbus::zvariant::Signature =
                &#zbus::zvariant::Signature::ObjectPath;
        }

        impl<'p> #zbus::export::serde::ser::Serialize for #proxy_name<'p> {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: #zbus::export::serde::ser::Serializer,
            {
                ::std::string::String::serialize(
                    &::std::string::ToString::to_string(self.inner().path()),
                    serializer,
                )
            }
        }

        #stream_types
    })
}

fn gen_proxy_method_call(
    dbus_member_name: &str,
    rust_method_name: &str,
    m: &TraitItemFn,
    method_attrs: MethodAttributes,
    async_opts: &AsyncOpts,
    zbus: &TokenStream,
) -> Result<TokenStream, Error> {
    let AsyncOpts {
        usage,
        wait,
        blocking,
    } = async_opts;
    let other_attrs: Vec<_> = m
        .attrs
        .iter()
        .filter(|a| !a.path().is_ident("zbus"))
        .collect();
    let args: Vec<_> = m
        .sig
        .inputs
        .iter()
        .filter_map(typed_arg)
        .filter_map(pat_ident)
        .collect();

    let proxy_object = method_attrs.object.as_ref().map(|o| {
        if *blocking {
            // FIXME: for some reason Rust doesn't let us move `blocking_proxy_object` so we've to
            // clone.
            method_attrs
                .blocking_object
                .as_ref()
                .cloned()
                .unwrap_or_else(|| format!("{o}ProxyBlocking"))
        } else {
            method_attrs
                .async_object
                .as_ref()
                .cloned()
                .unwrap_or_else(|| format!("{o}Proxy"))
        }
    });
    let proxy_vec = method_attrs.object_vec;

    let method_flags = match (
        method_attrs.no_reply,
        method_attrs.no_autostart,
        method_attrs.allow_interactive_auth,
    ) {
        (true, false, false) => Some(quote!(::std::convert::Into::into(
            zbus::proxy::MethodFlags::NoReplyExpected
        ))),
        (false, true, false) => Some(quote!(::std::convert::Into::into(
            zbus::proxy::MethodFlags::NoAutoStart
        ))),
        (false, false, true) => Some(quote!(::std::convert::Into::into(
            zbus::proxy::MethodFlags::AllowInteractiveAuth
        ))),

        (true, true, false) => Some(quote!(
            zbus::proxy::MethodFlags::NoReplyExpected | zbus::proxy::MethodFlags::NoAutoStart
        )),
        (true, false, true) => Some(quote!(
            zbus::proxy::MethodFlags::NoReplyExpected
                | zbus::proxy::MethodFlags::AllowInteractiveAuth
        )),
        (false, true, true) => Some(quote!(
            zbus::proxy::MethodFlags::NoAutoStart | zbus::proxy::MethodFlags::AllowInteractiveAuth
        )),

        (true, true, true) => Some(quote!(
            zbus::proxy::MethodFlags::NoReplyExpected
                | zbus::proxy::MethodFlags::NoAutoStart
                | zbus::proxy::MethodFlags::AllowInteractiveAuth
        )),
        _ => None,
    };

    let mut method = parse_str::<Ident>(rust_method_name)?;
    method.set_span(Span::call_site());
    let inputs = &m.sig.inputs;
    let mut generics = m.sig.generics.clone();
    let where_clause = generics.where_clause.get_or_insert(parse_quote!(where));
    for param in generics
        .params
        .iter()
        .filter(|a| matches!(a, syn::GenericParam::Type(_)))
    {
        let is_input_type = inputs.iter().any(|arg| {
            // FIXME: We want to only require `Serialize` from input types and `DeserializeOwned`
            // from output types but since we don't have type introspection, we employ this
            // workaround of matching on the string representation of the the types to figure out
            // which generic types are input types.
            if let FnArg::Typed(pat) = arg {
                let pat = pat.ty.to_token_stream().to_string();

                if let Some(ty_name) = pat.strip_prefix('&') {
                    let ty_name = ty_name.trim_start();

                    ty_name == param.to_token_stream().to_string()
                } else {
                    false
                }
            } else {
                false
            }
        });
        let serde_bound: TokenStream = if is_input_type {
            parse_quote!(#zbus::export::serde::ser::Serialize)
        } else {
            parse_quote!(#zbus::export::serde::de::DeserializeOwned)
        };
        where_clause.predicates.push(parse_quote!(
            #param: #serde_bound + #zbus::zvariant::Type
        ));
    }
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    if let Some(proxy_path) = proxy_object {
        let proxy_path = parse_str::<Path>(&proxy_path)?;
        let ok_type = if proxy_vec {
            quote!(Vec<#proxy_path<'p>>)
        } else {
            quote!(#proxy_path<'p>)
        };
        let signature = quote! {
            fn #method #ty_generics(#inputs) -> #zbus::Result<#ok_type>
            #where_clause
        };

        let proxy_build = quote! {
            #proxy_path::builder(&self.0.connection())
                .path(object_path)?
                .build()
                #wait
        };
        let method_call = quote! {
            self.0.call(
                #dbus_member_name,
                &#zbus::zvariant::DynamicTuple((#(#args,)*)),
            )
            #wait?
        };
        let body = if proxy_vec {
            quote! {
                let object_paths: Vec<#zbus::zvariant::OwnedObjectPath> = #method_call;

                let mut proxies = Vec::with_capacity(object_paths.len());
                for object_path in object_paths {
                    let proxy = #proxy_build?;
                    proxies.push(proxy);
                }

                Ok(proxies)
            }
        } else {
            quote! {
                let object_path: #zbus::zvariant::OwnedObjectPath = #method_call;
                #proxy_build
            }
        };
        Ok(quote! {
            #(#other_attrs)*
            pub #usage #signature {
                #body
            }
        })
    } else {
        let body = if args.len() == 1 {
            // Wrap single arg in a tuple so if it's a struct/tuple itself, zbus will only remove
            // the '()' from the signature that we add and not the actual intended ones.
            let arg = &args[0];
            quote! {
                &#zbus::zvariant::DynamicTuple((#arg,))
            }
        } else {
            quote! {
                &#zbus::zvariant::DynamicTuple((#(#args),*))
            }
        };

        let output = &m.sig.output;
        let signature = quote! {
            fn #method #ty_generics(#inputs) #output
            #where_clause
        };

        if let Some(method_flags) = method_flags {
            if method_attrs.no_reply {
                Ok(quote! {
                    #(#other_attrs)*
                    pub #usage #signature {
                        self.0.call_with_flags::<_, _, ()>(#dbus_member_name, #method_flags, #body)#wait?;
                        ::std::result::Result::Ok(())
                    }
                })
            } else {
                Ok(quote! {
                    #(#other_attrs)*
                    pub #usage #signature {
                        let reply = self.0.call_with_flags(#dbus_member_name, #method_flags, #body)#wait?;

                        // SAFETY: This unwrap() cannot fail due to the guarantees in
                        // call_with_flags, which can only return Ok(None) if the
                        // NoReplyExpected is set. By not passing NoReplyExpected,
                        // we are guaranteed to get either an Err variant (handled
                        // in the previous statement) or Ok(Some(T)) which is safe to
                        // unwrap
                        ::std::result::Result::Ok(reply.unwrap())
                    }
                })
            }
        } else {
            Ok(quote! {
                #(#other_attrs)*
                pub #usage #signature {
                    let reply = self.0.call(#dbus_member_name, #body)#wait?;
                    ::std::result::Result::Ok(reply)
                }
            })
        }
    }
}

fn gen_proxy_property(
    property_name: &str,
    rust_method_name: &str,
    m: &TraitItemFn,
    method_attrs: &MethodAttributes,
    async_opts: &AsyncOpts,
    emits_changed_signal: PropertyEmitsChangedSignal,
    zbus: &TokenStream,
) -> TokenStream {
    let AsyncOpts {
        usage,
        wait,
        blocking,
    } = async_opts;
    let other_attrs: Vec<_> = m
        .attrs
        .iter()
        .filter(|a| !a.path().is_ident("zbus"))
        .collect();
    let signature = &m.sig;
    if signature.inputs.len() > 1 {
        let value = pat_ident(typed_arg(signature.inputs.last().unwrap()).unwrap()).unwrap();
        quote! {
            #(#other_attrs)*
            #[allow(clippy::needless_question_mark)]
            pub #usage #signature {
                ::std::result::Result::Ok(self.0.set_property(#property_name, #value)#wait?)
            }
        }
    } else {
        // Check for object attribute to return a proxy instead of OwnedObjectPath
        let proxy_object = method_attrs.object.as_ref().map(|o| {
            if *blocking {
                method_attrs
                    .blocking_object
                    .as_ref()
                    .cloned()
                    .unwrap_or_else(|| format!("{o}ProxyBlocking"))
            } else {
                method_attrs
                    .async_object
                    .as_ref()
                    .cloned()
                    .unwrap_or_else(|| format!("{o}Proxy"))
            }
        });

        let proxy_vec = method_attrs.object_vec;

        // This should fail to compile only if the return type is wrong,
        // so use that as the span.
        let body_span = if let ReturnType::Type(_, ty) = &signature.output {
            ty.span()
        } else {
            signature.span()
        };

        let ret_type = if let ReturnType::Type(_, ty) = &signature.output {
            Some(ty)
        } else {
            None
        };

        // Generate the getter body - either returning a proxy object or the raw property value
        let (body, custom_signature) = if let Some(proxy_path_str) = proxy_object {
            let proxy_path: Path = parse_str(&proxy_path_str).unwrap();
            let method_name = Ident::new(rust_method_name, m.sig.ident.span());
            let inputs = &signature.inputs;
            let custom_sig = if proxy_vec {
                quote! {
                    fn #method_name(#inputs) -> #zbus::Result<Vec<#proxy_path<'p>>>
                }
            } else {
                quote! {
                    fn #method_name(#inputs) -> #zbus::Result<#proxy_path<'p>>
                }
            };
            let property_get = quote! {
                self.0.get_property(#property_name)#wait?
            };
            let proxy_build = quote! {
                #proxy_path::builder(&self.0.connection())
                    .destination(self.0.destination().to_owned())?
                    .path(object_path)?
                    .build()
                    #wait
            };
            let body = if proxy_vec {
                quote_spanned! {body_span =>
                    let object_paths: Vec<#zbus::zvariant::OwnedObjectPath> =
                        #property_get;

                    let mut proxies = Vec::with_capacity(object_paths.len());
                    for object_path in object_paths {
                        let proxy = #proxy_build?;
                        proxies.push(proxy);
                    }

                    Ok(proxies)
                }
            } else {
                quote_spanned! {body_span =>
                    let object_path: #zbus::zvariant::OwnedObjectPath =
                        #property_get;
                    #proxy_build
                }
            };
            (body, Some(custom_sig))
        } else {
            let body = quote_spanned! {body_span =>
                ::std::result::Result::Ok(self.0.get_property(#property_name)#wait?)
            };
            (body, None)
        };

        let (proxy_name, prop_stream) = if *blocking {
            (
                "zbus::blocking::Proxy",
                quote! { #zbus::blocking::proxy::PropertyIterator },
            )
        } else {
            ("zbus::Proxy", quote! { #zbus::proxy::PropertyStream })
        };

        let receive_method = match emits_changed_signal {
            PropertyEmitsChangedSignal::True | PropertyEmitsChangedSignal::Invalidates => {
                let (_, ty_generics, where_clause) = m.sig.generics.split_for_impl();
                let receive = format_ident!("receive_{}_changed", rust_method_name);
                let gen_doc = format!(
                    "Create a stream for the `{property_name}` property changes. \
                This is a convenient wrapper around [`{proxy_name}::receive_property_changed`]."
                );
                quote! {
                    #(#other_attrs)*
                    #[doc = #gen_doc]
                    pub #usage fn #receive #ty_generics(
                        &self
                    ) -> #prop_stream<'p, <#ret_type as #zbus::ResultAdapter>::Ok>
                    #where_clause
                    {
                        self.0.receive_property_changed(#property_name)#wait
                    }
                }
            }
            PropertyEmitsChangedSignal::False | PropertyEmitsChangedSignal::Const => {
                quote! {}
            }
        };

        let cached_getter_method = match emits_changed_signal {
            PropertyEmitsChangedSignal::True
            | PropertyEmitsChangedSignal::Invalidates
            | PropertyEmitsChangedSignal::Const => {
                let cached_getter = format_ident!("cached_{}", rust_method_name);
                let cached_doc = format!(
                    " Get the cached value of the `{property_name}` property, or `None` if the property is not cached.",
                );
                quote! {
                    #(#other_attrs)*
                    #[doc = #cached_doc]
                    pub fn #cached_getter(&self) -> ::std::result::Result<
                        ::std::option::Option<<#ret_type as #zbus::ResultAdapter>::Ok>,
                        <#ret_type as #zbus::ResultAdapter>::Err>
                    {
                        self.0.cached_property(#property_name).map_err(::std::convert::Into::into)
                    }
                }
            }
            PropertyEmitsChangedSignal::False => quote! {},
        };

        // When object attribute is used, we use a custom signature and skip cached/receive methods
        // since those don't make sense for proxy objects
        let (sig, extra_methods) = if let Some(sig) = custom_signature {
            (sig, quote! {})
        } else {
            (
                quote! { #signature },
                quote! {
                    #cached_getter_method
                    #receive_method
                },
            )
        };

        quote! {
            #(#other_attrs)*
            #[allow(clippy::needless_question_mark)]
            pub #usage #sig {
                #body
            }

            #extra_methods
        }
    }
}

struct SetLifetimeS;

impl Fold for SetLifetimeS {
    fn fold_type_reference(&mut self, node: syn::TypeReference) -> syn::TypeReference {
        let mut t = syn::fold::fold_type_reference(self, node);
        t.lifetime = Some(syn::Lifetime::new("'s", Span::call_site()));
        t
    }

    fn fold_lifetime(&mut self, _node: syn::Lifetime) -> syn::Lifetime {
        syn::Lifetime::new("'s", Span::call_site())
    }
}

#[allow(clippy::too_many_arguments)]
fn gen_proxy_signal(
    proxy_name: &Ident,
    iface_name: &str,
    signal_name: &str,
    rust_method_name: &str,
    method: &TraitItemFn,
    async_opts: &AsyncOpts,
    visibility: &Visibility,
    gen_sig_args: bool,
    zbus: &TokenStream,
) -> (TokenStream, TokenStream) {
    let AsyncOpts {
        usage,
        wait,
        blocking,
    } = async_opts;
    let other_attrs: Vec<_> = method
        .attrs
        .iter()
        .filter(|a| !a.path().is_ident("zbus"))
        .collect();
    let input_types: Vec<_> = method
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(p) => Some(&*p.ty),
            _ => None,
        })
        .collect();
    let input_types_s: Vec<_> = SetLifetimeS
        .fold_signature(method.sig.clone())
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(p) => Some(p.ty.clone()),
            _ => None,
        })
        .collect();
    let args: Vec<Ident> = method
        .sig
        .inputs
        .iter()
        .filter_map(typed_arg)
        .filter_map(|arg| pat_ident(arg).cloned())
        .collect();
    let args_nth: Vec<Literal> = args
        .iter()
        .enumerate()
        .map(|(i, _)| Literal::usize_unsuffixed(i))
        .collect();

    let mut generics = method.sig.generics.clone();
    let where_clause = generics.where_clause.get_or_insert(parse_quote!(where));
    for param in generics
        .params
        .iter()
        .filter(|a| matches!(a, syn::GenericParam::Type(_)))
    {
        where_clause
                .predicates
                .push(parse_quote!(#param: #zbus::export::serde::de::Deserialize<'s> + #zbus::zvariant::Type + ::std::fmt::Debug));
    }
    generics.params.push(parse_quote!('s));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let (
        proxy_path,
        receive_signal_link,
        receive_signal_with_args_link,
        trait_name,
        trait_link,
        signal_type,
    ) = if *blocking {
        (
            "zbus::blocking::Proxy",
            "https://docs.rs/zbus/latest/zbus/blocking/proxy/struct.Proxy.html#method.receive_signal",
            "https://docs.rs/zbus/latest/zbus/blocking/proxy/struct.Proxy.html#method.receive_signal_with_args",
            "Iterator",
            "https://doc.rust-lang.org/std/iter/trait.Iterator.html",
            quote! { blocking::proxy::SignalIterator },
        )
    } else {
        (
            "zbus::Proxy",
            "https://docs.rs/zbus/latest/zbus/proxy/struct.Proxy.html#method.receive_signal",
            "https://docs.rs/zbus/latest/zbus/proxy/struct.Proxy.html#method.receive_signal_with_args",
            "Stream",
            "https://docs.rs/futures/0.3.15/futures/stream/trait.Stream.html",
            quote! { proxy::SignalStream },
        )
    };
    let receiver_name = format_ident!("receive_{rust_method_name}");
    let receiver_with_args_name = format_ident!("receive_{rust_method_name}_with_args");
    let stream_name = format_ident!("{signal_name}{trait_name}");
    let signal_args = format_ident!("{signal_name}Args");
    let signal_name_ident = format_ident!("{signal_name}");

    let receive_gen_doc = format!(
        "Create a stream that receives `{signal_name}` signals.\n\
            \n\
            This a convenient wrapper around [`{proxy_path}::receive_signal`]({receive_signal_link}).",
    );
    let receive_with_args_gen_doc = format!(
        "Create a stream that receives `{signal_name}` signals.\n\
            \n\
            This a convenient wrapper around [`{proxy_path}::receive_signal_with_args`]({receive_signal_with_args_link}).",
    );
    let receive_signal_with_args = if args.is_empty() {
        quote!()
    } else {
        quote! {
            #[doc = #receive_with_args_gen_doc]
            #(#other_attrs)*
            pub #usage fn #receiver_with_args_name(&self, args: &[(u8, &str)]) -> #zbus::Result<#stream_name>
            {
                self.0.receive_signal_with_args(#signal_name, args)#wait.map(#stream_name)
            }
        }
    };
    let receive_signal = quote! {
        #[doc = #receive_gen_doc]
        #(#other_attrs)*
        pub #usage fn #receiver_name(&self) -> #zbus::Result<#stream_name>
        {
            self.0.receive_signal(#signal_name)#wait.map(#stream_name)
        }

        #receive_signal_with_args
    };

    let stream_gen_doc = format!(
        "A [`{trait_name}`] implementation that yields [`{signal_name}`] signals.\n\
            \n\
            Use [`{proxy_name}::{receiver_name}`] to create an instance of this type.\n\
            \n\
            [`{trait_name}`]: {trait_link}",
    );
    let signal_args_gen_doc = format!("`{signal_name}` signal arguments.");
    let args_struct_gen_doc = format!("A `{signal_name}` signal.");
    let args_struct_decl = if gen_sig_args {
        quote! {
            #[doc = #args_struct_gen_doc]
            #[derive(Debug, Clone)]
            #visibility struct #signal_name_ident(#zbus::message::Body);

            impl #signal_name_ident {
                #[doc = "Try to construct a "]
                #[doc = #signal_name]
                #[doc = " from a [`zbus::Message`]."]
                pub fn from_message<M>(msg: M) -> ::std::option::Option<Self>
                where
                    M: ::std::convert::Into<#zbus::message::Message>,
                {
                    let msg = msg.into();
                    let hdr = msg.header();
                    let message_type = msg.message_type();
                    let interface = hdr.interface();
                    let member = hdr.member();
                    let interface = interface.as_ref().map(|i| i.as_str());
                    let member = member.as_ref().map(|m| m.as_str());

                    match (message_type, interface, member) {
                        (#zbus::message::Type::Signal, Some(#iface_name), Some(#signal_name)) => {
                            Some(Self(msg.body()))
                        }
                        _ => None,
                    }
                }

                #[doc = "The reference to the underlying [`zbus::Message`]."]
                pub fn message(&self) -> &#zbus::message::Message {
                    self.0.message()
                }
            }

            impl ::std::convert::From<#signal_name_ident> for #zbus::message::Message {
                fn from(signal: #signal_name_ident) -> Self {
                    signal.0.message().clone()
                }
            }
        }
    } else {
        quote!()
    };
    let args_impl = if args.is_empty() || !gen_sig_args {
        quote!()
    } else {
        let arg_fields_init = if args.len() == 1 {
            quote! { #(#args)*: args }
        } else {
            quote! { #(#args: args.#args_nth),* }
        };

        quote! {
            impl #signal_name_ident {
                /// Retrieve the signal arguments.
                pub fn args #ty_generics(&'s self) -> #zbus::Result<#signal_args #ty_generics>
                #where_clause
                {
                    ::std::convert::TryFrom::try_from(&self.0)
                }
            }

            #[doc = #signal_args_gen_doc]
            #visibility struct #signal_args #ty_generics {
                phantom: std::marker::PhantomData<&'s ()>,
                #(
                    pub #args: #input_types_s
                 ),*
            }

            impl #impl_generics #signal_args #ty_generics
                #where_clause
            {
                #(
                    pub fn #args(&self) -> &#input_types_s {
                        &self.#args
                    }
                 )*
            }

            impl #impl_generics std::fmt::Debug for #signal_args #ty_generics
                #where_clause
            {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(#signal_name)
                    #(
                     .field(stringify!(#args), &self.#args)
                    )*
                     .finish()
                }
            }

            impl #impl_generics ::std::convert::TryFrom<&'s #zbus::message::Body> for #signal_args #ty_generics
                #where_clause
            {
                type Error = #zbus::Error;

                fn try_from(msg_body: &'s #zbus::message::Body) -> #zbus::Result<Self> {
                    msg_body.deserialize::<(#(#input_types),*)>()
                        .map_err(::std::convert::Into::into)
                        .map(|args| {
                            #signal_args {
                                phantom: ::std::marker::PhantomData,
                                #arg_fields_init
                            }
                        })
                }
            }
        }
    };
    let stream_impl = if *blocking {
        quote! {
            impl ::std::iter::Iterator for #stream_name {
                type Item = #signal_name_ident;

                fn next(&mut self) -> ::std::option::Option<Self::Item> {
                    ::std::iter::Iterator::next(&mut self.0)
                        .map(|msg| #signal_name_ident(msg.body()))
                }
            }
        }
    } else {
        quote! {
            impl #zbus::export::futures_core::stream::Stream for #stream_name {
                type Item = #signal_name_ident;

                fn poll_next(
                    self: ::std::pin::Pin<&mut Self>,
                    cx: &mut ::std::task::Context<'_>,
                    ) -> ::std::task::Poll<::std::option::Option<Self::Item>> {
                    #zbus::export::futures_core::stream::Stream::poll_next(
                        ::std::pin::Pin::new(&mut self.get_mut().0),
                        cx,
                    )
                    .map(|msg| msg.map(|msg| #signal_name_ident(msg.body())))
                }
            }

            impl #zbus::export::ordered_stream::OrderedStream for #stream_name {
                type Data = #signal_name_ident;
                type Ordering = #zbus::message::Sequence;

                fn poll_next_before(
                    self: ::std::pin::Pin<&mut Self>,
                    cx: &mut ::std::task::Context<'_>,
                    before: ::std::option::Option<&Self::Ordering>
                    ) -> ::std::task::Poll<#zbus::export::ordered_stream::PollResult<Self::Ordering, Self::Data>> {
                    #zbus::export::ordered_stream::OrderedStream::poll_next_before(
                        ::std::pin::Pin::new(&mut self.get_mut().0),
                        cx,
                        before,
                    )
                    .map(|msg| msg.map_data(|msg| #signal_name_ident(msg.body())))
                }
            }

            impl #zbus::export::futures_core::stream::FusedStream for #stream_name {
                fn is_terminated(&self) -> bool {
                    self.0.is_terminated()
                }
            }

            #[#zbus::export::async_trait::async_trait]
            impl #zbus::AsyncDrop for #stream_name {
                async fn async_drop(self) {
                    self.0.async_drop().await
                }
            }
        }
    };
    let stream_types = quote! {
        #[doc = #stream_gen_doc]
        #[derive(Debug)]
        #visibility struct #stream_name(#zbus::#signal_type<'static>);

        impl #stream_name {
            /// Consumes `self`, returning the underlying `zbus::#signal_type`.
            pub fn into_inner(self) -> #zbus::#signal_type<'static> {
                self.0
            }

            /// The reference to the underlying `zbus::#signal_type`.
            pub fn inner(&self) -> & #zbus::#signal_type<'static> {
                &self.0
            }
        }

        #stream_impl

        #args_struct_decl

        #args_impl
    };

    (receive_signal, stream_types)
}
