use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeMap;
use syn::{
    AngleBracketedGenericArguments, Attribute, Error, Expr, ExprLit, FnArg, GenericArgument, Ident,
    ImplItem, ImplItemFn, ItemImpl,
    Lit::Str,
    Meta, MetaNameValue, PatType, PathArguments, ReturnType, Signature, Token, Type, TypePath,
    Visibility,
    parse::{Parse, ParseStream},
    parse_quote, parse_str,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Async, Comma},
};
use zvariant_utils::{case, def_attrs};

use crate::utils::*;

def_attrs! {
    crate zbus;

    pub ImplAttributes("impl block") {
        interface str,
        name str,
        spawn bool,
        introspection_docs bool,
        crate_path str,
        proxy {
            // Keep this in sync with proxy's method attributes.
            // TODO: Find a way to share code with proxy module.
            pub ProxyAttributes("proxy") {
                assume_defaults bool,
                default_path str,
                default_service str,
                async_name str,
                blocking_name str,
                gen_async bool,
                gen_blocking bool,
                visibility str
            }
        }
    };

    pub MethodAttributes("method") {
        name str,
        signal none,
        property {
            pub PropertyAttributes("property") {
                emits_changed_signal str
            }
        },
        out_args [str],
        proxy {
            // Keep this in sync with proxy's method attributes.
            // TODO: Find a way to share code with proxy module.
            pub ProxyMethodAttributes("proxy") {
                object str,
                async_object str,
                blocking_object str,
                object_vec none,
                no_reply none,
                no_autostart none,
                allow_interactive_auth none
            }
        }
    };

    pub ArgAttributes("argument") {
        object_server none,
        connection none,
        header none,
        signal_context none,
        signal_emitter none
    };
}

#[derive(Debug, Default)]
struct Property {
    read: bool,
    write: bool,
    emits_changed_signal: PropertyEmitsChangedSignal,
    ty: Option<Type>,
    doc_comments: TokenStream,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum MethodType {
    Signal,
    Property(PropertyType),
    Other,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum PropertyType {
    Setter,
    Getter,
}

#[derive(Debug, Clone)]
struct MethodInfo {
    /// The method identifier
    ident: Ident,
    /// The type of method being parsed
    method_type: MethodType,
    /// Whether the method has inputs
    has_inputs: bool,
    /// Whether the method is async
    is_async: bool,
    /// Doc comments on the methods
    doc_comments: TokenStream,
    /// Whether self is passed as mutable to the method
    is_mut: bool,
    /// The await to append to method calls
    method_await: TokenStream,
    /// The typed inputs passed to the method
    typed_inputs: Vec<PatType>,
    /// The method arguments' introspection
    intro_args: TokenStream,
    /// Whether the output type is a Result
    is_result_output: bool,
    /// Code block to deserialize arguments from zbus message
    args_from_msg: TokenStream,
    /// Names of all arguments to the method
    args_names: TokenStream,
    /// Code stream to match on the reply of the method call
    reply: TokenStream,
    /// The signal context object argument
    signal_emitter_arg: Option<PatType>,
    /// The name of the method (setters are stripped of set_ prefix)
    member_name: String,
    /// The proxy method attributes, if any.
    proxy_attrs: Option<ProxyMethodAttributes>,
    /// The method output type.
    output: ReturnType,
    /// The cfg attributes of the method.
    cfg_attrs: Vec<Attribute>,
    /// The doc attributes of the method.
    doc_attrs: Vec<Attribute>,
}

impl MethodInfo {
    fn new(
        zbus: &TokenStream,
        method: &ImplItemFn,
        attrs: &MethodAttributes,
        cfg_attrs: &[&Attribute],
        doc_attrs: &[&Attribute],
        introspect_docs: bool,
    ) -> syn::Result<MethodInfo> {
        let is_async = method.sig.asyncness.is_some();
        let Signature {
            ident,
            inputs,
            output,
            ..
        } = &method.sig;
        let doc_comments = if introspect_docs {
            let docs = get_doc_attrs(&method.attrs)
                .iter()
                .filter_map(|attr| {
                    if let Ok(MetaNameValue {
                        value: Expr::Lit(ExprLit { lit: Str(s), .. }),
                        ..
                    }) = &attr.meta.require_name_value()
                    {
                        Some(s.value())
                    } else {
                        // non #[doc = "..."] attributes are not our concern
                        // we leave them for rustc to handle
                        None
                    }
                })
                .collect();
            to_xml_docs(docs)
        } else {
            quote!()
        };
        let is_property = attrs.property.is_some();
        let is_signal = attrs.signal;
        assert!(!is_property || !is_signal);

        let mut typed_inputs = inputs
            .iter()
            .filter_map(typed_arg)
            .cloned()
            .collect::<Vec<_>>();

        let has_inputs = count_regular_args(&typed_inputs) > 0;

        let method_type = if is_signal {
            MethodType::Signal
        } else if is_property {
            if has_inputs {
                MethodType::Property(PropertyType::Setter)
            } else {
                MethodType::Property(PropertyType::Getter)
            }
        } else {
            MethodType::Other
        };

        let is_mut = if let FnArg::Receiver(r) = inputs
            .first()
            .ok_or_else(|| Error::new_spanned(ident, "not &self method"))?
        {
            r.mutability.is_some()
        } else if is_signal {
            false
        } else {
            return Err(Error::new_spanned(method, "missing receiver"));
        };
        if is_signal && !is_async {
            return Err(Error::new_spanned(method, "signals must be async"));
        }
        let method_await = if is_async {
            quote! { .await }
        } else {
            quote! {}
        };

        let signal_emitter_arg: Option<PatType> = if is_signal {
            if typed_inputs.is_empty() {
                return Err(Error::new_spanned(
                    inputs,
                    "Expected a `&zbus::object_server::SignalEmitter<'_> argument",
                ));
            }
            Some(typed_inputs.remove(0))
        } else {
            None
        };

        let mut intro_args = quote!();
        intro_args.extend(introspect_input_args(&typed_inputs, is_signal, cfg_attrs));
        let is_result_output = introspect_add_output_args(
            &mut intro_args,
            output,
            attrs.out_args.as_deref(),
            cfg_attrs,
        )?;

        let (args_from_msg, args_names) = get_args_from_inputs(&typed_inputs, method_type, zbus)?;

        let reply = if is_result_output {
            let ret = quote!(r);

            quote!(match reply {
                ::std::result::Result::Ok(r) => __zbus__connection.reply(&hdr, &#ret).await,
                ::std::result::Result::Err(e) => __zbus__connection.reply_dbus_error(&hdr, e).await,
            })
        } else {
            quote!(__zbus__connection.reply(&hdr, &reply).await)
        };

        let member_name = attrs.name.clone().unwrap_or_else(|| {
            let mut name = ident.to_string();
            if is_property && has_inputs {
                assert!(name.starts_with("set_"));
                name = name[4..].to_string();
            } else if name.starts_with("r#") {
                name = name[2..].to_string();
            }
            pascal_case(&name)
        });

        Ok(MethodInfo {
            ident: ident.clone(),
            method_type,
            has_inputs,
            is_async,
            doc_comments,
            is_mut,
            method_await,
            typed_inputs,
            signal_emitter_arg,
            intro_args,
            is_result_output,
            args_from_msg,
            args_names,
            reply,
            member_name,
            proxy_attrs: attrs.proxy.clone(),
            output: output.clone(),
            cfg_attrs: cfg_attrs.iter().cloned().cloned().collect(),
            doc_attrs: doc_attrs.iter().cloned().cloned().collect(),
        })
    }
}

pub fn expand(args: Punctuated<Meta, Token![,]>, mut input: ItemImpl) -> syn::Result<TokenStream> {
    let impl_attrs = ImplAttributes::parse_nested_metas(args)?;
    let crate_path = parse_crate_path(impl_attrs.crate_path.as_deref())?;
    let zbus = zbus_path(crate_path.as_ref());

    let self_ty = &input.self_ty;
    let mut properties = BTreeMap::new();
    let mut set_dispatch = quote!();
    let mut set_mut_dispatch = quote!();
    let mut get_dispatch = quote!();
    let mut get_all = quote!();
    let mut call_dispatch = quote!();
    let mut call_mut_dispatch = quote!();
    let mut introspect = quote!();
    let mut generated_signals = quote!();
    let mut signals_trait_methods = quote!();
    let mut signals_emitter_impl_methods = quote!();
    let mut signals_interface_ref_impl_methods = quote!();

    // the impl Type
    let ty = match input.self_ty.as_ref() {
        Type::Path(p) => {
            &p.path
                .segments
                .last()
                .ok_or_else(|| Error::new_spanned(p, "Unsupported 'impl' type"))?
                .ident
        }
        _ => return Err(Error::new_spanned(&input.self_ty, "Invalid type")),
    };
    let iface_name = {
        match (impl_attrs.name, impl_attrs.interface) {
            // Ensure the interface name is valid.
            (Some(name), None) | (None, Some(name)) => zbus_names::InterfaceName::try_from(name)
                .map_err(|e| Error::new(input.span(), format!("{e}")))
                .map(|i| i.to_string())?,
            (None, None) => format!("org.freedesktop.{ty}"),
            (Some(_), Some(_)) => {
                return Err(syn::Error::new(
                    input.span(),
                    "`name` and `interface` attributes should not be specified at the same time",
                ));
            }
        }
    };
    let with_spawn = impl_attrs.spawn.unwrap_or(true);
    let mut proxy = impl_attrs
        .proxy
        .map(|p| Proxy::new(ty, &iface_name, p, &zbus));
    let introspect_docs = impl_attrs.introspection_docs.unwrap_or(true);

    // Store parsed information about each method
    let mut methods = vec![];
    for item in &mut input.items {
        let (method, is_signal) = match item {
            ImplItem::Fn(m) => (m, false),
            // Since signals do not have a function body, they don't parse as ImplItemFn…
            ImplItem::Verbatim(tokens) => {
                // … thus parse them ourselves and construct an ImplItemFn from that
                let decl = syn::parse2::<ImplItemSignal>(tokens.clone())?;
                let ImplItemSignal { attrs, vis, sig } = decl;
                *item = ImplItem::Fn(ImplItemFn {
                    attrs,
                    vis,
                    defaultness: None,
                    sig,
                    // This empty block will be replaced below.
                    block: parse_quote!({}),
                });
                match item {
                    ImplItem::Fn(m) => (m, true),
                    _ => unreachable!(),
                }
            }
            _ => continue,
        };

        let method_attrs = MethodAttributes::parse(&method.attrs)?;

        method.attrs.retain(|attr| !attr.path().is_ident("zbus"));

        if is_signal && !method_attrs.signal {
            return Err(syn::Error::new_spanned(
                item,
                "methods that are not signals must have a body",
            ));
        }

        let cfg_attrs: Vec<_> = method
            .attrs
            .iter()
            .filter(|a| a.path().is_ident("cfg"))
            .collect();
        let doc_attrs: Vec<_> = method
            .attrs
            .iter()
            .filter(|a| a.path().is_ident("doc"))
            .collect();

        let method_info = MethodInfo::new(
            &zbus,
            method,
            &method_attrs,
            &cfg_attrs,
            &doc_attrs,
            introspect_docs,
        )?;
        let attr_property = method_attrs.property;
        if let Some(prop_attrs) = &attr_property {
            let property: &mut Property = properties
                .entry(method_info.member_name.to_string())
                .or_default();
            if method_info.method_type == MethodType::Property(PropertyType::Getter) {
                let emits_changed_signal = if let Some(s) = &prop_attrs.emits_changed_signal {
                    PropertyEmitsChangedSignal::parse(s, method.span())?
                } else {
                    PropertyEmitsChangedSignal::True
                };
                property.read = true;
                property.emits_changed_signal = emits_changed_signal;
            } else {
                property.write = true;
                if prop_attrs.emits_changed_signal.is_some() {
                    return Err(Error::new_spanned(
                        method,
                        "`emits_changed_signal` cannot be specified on setters",
                    ));
                }
            }
        }
        methods.push((method, method_info));
    }

    for (method, method_info) in methods {
        let info = method_info.clone();
        let MethodInfo {
            method_type,
            has_inputs,
            is_async,
            doc_comments,
            is_mut,
            method_await,
            typed_inputs,
            signal_emitter_arg,
            intro_args,
            is_result_output,
            args_from_msg,
            args_names,
            reply,
            member_name,
            cfg_attrs,
            ..
        } = method_info;

        let mut method_clone = method.clone();
        let Signature {
            ident,
            inputs,
            output,
            ..
        } = &mut method.sig;

        clear_input_arg_attrs(inputs);

        match method_type {
            MethodType::Signal => {
                introspect.extend(doc_comments);
                introspect.extend(introspect_signal(&member_name, &intro_args));
                let signal_emitter = signal_emitter_arg.unwrap().pat;

                method.block = parse_quote!({
                    #signal_emitter.emit(
                        <#self_ty as #zbus::object_server::Interface>::name(),
                        #member_name,
                        &(#args_names),
                    )
                    .await
                });

                method_clone.sig.asyncness = Some(Async(method_clone.span()));
                *method_clone.sig.inputs.first_mut().unwrap() = parse_quote!(&self);
                method_clone.vis = Visibility::Inherited;
                let sig = &method_clone.sig;
                signals_trait_methods.extend(quote! {
                    #sig;
                });
                method_clone.block = parse_quote!({
                    self.emit(
                        #iface_name,
                        #member_name,
                        &(#args_names),
                    )
                    .await
                });
                signals_emitter_impl_methods.extend(quote! {
                    #method_clone
                });
                method_clone.block = parse_quote!({
                    <#zbus::object_server::InterfaceRef<#self_ty>>::signal_emitter(self)
                        .emit(
                            #iface_name,
                            #member_name,
                            &(#args_names),
                        )
                        .await
                });
                signals_interface_ref_impl_methods.extend(quote! {
                    #method_clone
                });
            }
            MethodType::Property(_) => {
                let p = properties.get_mut(&member_name).unwrap();

                let sk_member_name = case::snake_or_kebab_case(&member_name, true);
                let prop_changed_method_name = format_ident!("{sk_member_name}_changed");
                let prop_invalidate_method_name = format_ident!("{sk_member_name}_invalidate");

                p.doc_comments.extend(doc_comments);
                if has_inputs {
                    let set_call = if is_result_output {
                        quote!(self.#ident(#args_names)#method_await)
                    } else if is_async {
                        quote!(
                            ::std::result::Result::Ok(self.#ident(#args_names).await)
                        )
                    } else {
                        quote!(
                            ::std::result::Result::Ok(self.#ident(#args_names))
                        )
                    };

                    // * For reference arg, we convert from `&Value` (so `TryFrom<&Value<'_>>` is
                    //   required).
                    //
                    // * For argument type with lifetimes, we convert from `Value` (so
                    //   `TryFrom<Value<'_>>` is required).
                    //
                    // * For all other arg types, we convert the passed value to `OwnedValue` first
                    //   and then pass it as `Value` (so `TryFrom<OwnedValue>` is required).
                    let value_to_owned = quote! {
                        match ::zbus::zvariant::Value::try_to_owned(value) {
                            ::std::result::Result::Ok(val) => ::zbus::zvariant::Value::from(val),
                            ::std::result::Result::Err(e) => {
                                return ::std::result::Result::Err(
                                    ::std::convert::Into::into(#zbus::Error::Variant(::std::convert::Into::into(e)))
                                );
                            }
                        }
                    };

                    let value_param = typed_inputs
                        .iter()
                        .find(|input| {
                            let a = ArgAttributes::parse(&input.attrs).unwrap();
                            !a.object_server
                                && !a.connection
                                && !a.header
                                && !a.signal_context
                                && !a.signal_emitter
                        })
                        .ok_or_else(|| Error::new_spanned(inputs, "Expected a value argument"))?;

                    // Use setter argument type as the property type if the getter is not
                    // explicitly defined.
                    if !p.read {
                        p.ty = Some((*value_param.ty).clone());
                        p.emits_changed_signal = PropertyEmitsChangedSignal::False;
                    }

                    let value_arg = match &*value_param.ty {
                        Type::Reference(_) => quote!(value),
                        Type::Path(path) => path
                            .path
                            .segments
                            .first()
                            .map(|segment| match &segment.arguments {
                                PathArguments::AngleBracketed(angled) => angled
                                    .args
                                    .first()
                                    .filter(|arg| matches!(arg, GenericArgument::Lifetime(_)))
                                    .map(|_| quote!(match ::zbus::zvariant::Value::try_clone(value) {
                                        ::std::result::Result::Ok(val) => val,
                                        ::std::result::Result::Err(e) => {
                                            return ::std::result::Result::Err(
                                                ::std::convert::Into::into(#zbus::Error::Variant(::std::convert::Into::into(e)))
                                            );
                                        }
                                    }))
                                    .unwrap_or_else(|| value_to_owned.clone()),
                                _ => value_to_owned.clone(),
                            })
                            .unwrap_or_else(|| value_to_owned.clone()),
                        _ => value_to_owned,
                    };

                    let value_param_name = &value_param.pat;
                    let prop_changed_method = match p.emits_changed_signal {
                        PropertyEmitsChangedSignal::True => {
                            quote!({
                                self
                                    .#prop_changed_method_name(&__zbus__signal_emitter)
                                    .await
                                    .map(|_| set_result)
                                    .map_err(Into::into)
                            })
                        }
                        PropertyEmitsChangedSignal::Invalidates => {
                            quote!({
                                self
                                    .#prop_invalidate_method_name(&__zbus__signal_emitter)
                                    .await
                                    .map(|_| set_result)
                                    .map_err(Into::into)
                            })
                        }
                        PropertyEmitsChangedSignal::False | PropertyEmitsChangedSignal::Const => {
                            quote!({ Ok(()) })
                        }
                    };
                    let do_set = quote!({
                        #args_from_msg
                        let value = #value_arg;
                        match ::std::convert::TryInto::try_into(value) {
                            ::std::result::Result::Ok(val) => {
                                let #value_param_name = val;
                                match #set_call {
                                    ::std::result::Result::Ok(set_result) => #prop_changed_method
                                    e => e,
                                }
                            }
                            ::std::result::Result::Err(e) => {
                                ::std::result::Result::Err(
                                    ::std::convert::Into::into(#zbus::Error::Variant(::std::convert::Into::into(e))),
                                )
                            }
                        }
                    });

                    if is_mut {
                        let q = quote!(
                            #(#cfg_attrs)*
                            #member_name => {
                                ::std::option::Option::Some((move || async move { #do_set }) ().await)
                            }
                        );
                        set_mut_dispatch.extend(q);

                        let q = quote!(
                            #(#cfg_attrs)*
                            #member_name => #zbus::object_server::DispatchResult::RequiresMut,
                        );
                        set_dispatch.extend(q);
                    } else {
                        let q = quote!(
                            #(#cfg_attrs)*
                            #member_name => {
                                #zbus::object_server::DispatchResult::Async(::std::boxed::Box::pin(async move {
                                    #do_set
                                }))
                            }
                        );
                        set_dispatch.extend(q);
                    }
                } else {
                    let is_fallible_property = is_result_output;

                    p.ty = Some(get_return_type(output)?.clone());

                    let value_convert = quote!(
                        <#zbus::zvariant::OwnedValue as ::std::convert::TryFrom<_>>::try_from(
                            <#zbus::zvariant::Value as ::std::convert::From<_>>::from(
                                value,
                            ),
                        )
                        .map_err(|e| #zbus::fdo::Error::Failed(e.to_string()))
                    );
                    let inner = if is_fallible_property {
                        quote!(self.#ident(#args_names) #method_await .and_then(|value| #value_convert))
                    } else {
                        quote!({
                            let value = self.#ident(#args_names)#method_await;
                            #value_convert
                        })
                    };

                    let q = quote!(
                        #(#cfg_attrs)*
                        #member_name => {
                            #args_from_msg
                            ::std::option::Option::Some(#inner)
                        },
                    );
                    get_dispatch.extend(q);

                    let q = if is_fallible_property {
                        quote!({
                            #args_from_msg
                            if let Ok(prop) = self.#ident(#args_names)#method_await {
                            props.insert(
                                ::std::string::ToString::to_string(#member_name),
                                <#zbus::zvariant::OwnedValue as ::std::convert::TryFrom<_>>::try_from(
                                    <#zbus::zvariant::Value as ::std::convert::From<_>>::from(
                                        prop,
                                    ),
                                )
                                .map_err(|e| #zbus::fdo::Error::Failed(e.to_string()))?,
                            );
                        }})
                    } else {
                        quote!({
                            #args_from_msg
                            props.insert(
                                ::std::string::ToString::to_string(#member_name),
                                <#zbus::zvariant::OwnedValue as ::std::convert::TryFrom<_>>::try_from(
                                    <#zbus::zvariant::Value as ::std::convert::From<_>>::from(
                                        self.#ident(#args_names)#method_await,
                                    ),
                                )
                                .map_err(|e| #zbus::fdo::Error::Failed(e.to_string()))?,
                            );
                        })
                    };

                    get_all.extend(q);

                    let prop_value_handled = if is_fallible_property {
                        quote!(self.#ident(#args_names)#method_await?)
                    } else {
                        quote!(self.#ident(#args_names)#method_await)
                    };

                    if p.emits_changed_signal == PropertyEmitsChangedSignal::True {
                        let changed_doc = format!(
                            "Emit the “PropertiesChanged” signal with the new value for the\n\
                             `{member_name}` property.\n\n\
                             This method should be called if a property value changes outside\n\
                             its setter method."
                        );
                        let prop_changed_method = quote!(
                            #[doc = #changed_doc]
                            pub async fn #prop_changed_method_name(
                                &self,
                                __zbus__signal_emitter: &#zbus::object_server::SignalEmitter<'_>,
                            ) -> #zbus::Result<()> {
                                let __zbus__header = ::std::option::Option::None::<&#zbus::message::Header<'_>>;
                                let __zbus__connection = __zbus__signal_emitter.connection();
                                let __zbus__object_server = __zbus__connection.object_server();
                                #args_from_msg
                                let mut changed = ::std::collections::HashMap::new();
                                let value = <#zbus::zvariant::Value as ::std::convert::From<_>>::from(#prop_value_handled);
                                changed.insert(#member_name, value);
                                #zbus::fdo::Properties::properties_changed(
                                    __zbus__signal_emitter,
                                    #zbus::names::InterfaceName::from_static_str_unchecked(#iface_name),
                                    changed,
                                    ::std::borrow::Cow::Borrowed(&[]),
                                ).await
                            }
                        );

                        generated_signals.extend(prop_changed_method);
                    }

                    if p.emits_changed_signal == PropertyEmitsChangedSignal::Invalidates {
                        let invalidate_doc = format!(
                            "Emit the “PropertiesChanged” signal for the `{member_name}` property\n\
                             without including the new value.\n\n\
                             It is usually better to call `{prop_changed_method_name}` instead so\n\
                             that interested peers do not need to fetch the new value separately\n\
                             (causing excess traffic on the bus)."
                        );
                        let prop_invalidate_method = quote!(
                            #[doc = #invalidate_doc]
                            pub async fn #prop_invalidate_method_name(
                                &self,
                                __zbus__signal_emitter: &#zbus::object_server::SignalEmitter<'_>,
                            ) -> #zbus::Result<()> {
                                #zbus::fdo::Properties::properties_changed(
                                    __zbus__signal_emitter,
                                    #zbus::names::InterfaceName::from_static_str_unchecked(#iface_name),
                                    ::std::collections::HashMap::new(),
                                    ::std::borrow::Cow::Borrowed(&[#member_name]),
                                ).await
                            }
                        );

                        generated_signals.extend(prop_invalidate_method);
                    }
                }
            }
            MethodType::Other => {
                introspect.extend(doc_comments);
                introspect.extend(introspect_method(&member_name, &intro_args));

                let m = quote! {
                    #(#cfg_attrs)*
                    #member_name => {
                        let future = async move {
                            #args_from_msg
                            let reply = self.#ident(#args_names)#method_await;
                            let hdr = __zbus__message.header();
                            if hdr.primary().flags().contains(zbus::message::Flags::NoReplyExpected) {
                                Ok(())
                            } else {
                                #reply
                            }
                        };
                        #zbus::object_server::DispatchResult::Async(::std::boxed::Box::pin(async move {
                            future.await
                        }))
                    },
                };

                if is_mut {
                    call_dispatch.extend(quote! {
                        #(#cfg_attrs)*
                        #member_name => #zbus::object_server::DispatchResult::RequiresMut,
                    });
                    call_mut_dispatch.extend(m);
                } else {
                    call_dispatch.extend(m);
                }
            }
        }

        if let Some(proxy) = &mut proxy {
            proxy.add_method(info, &properties)?;
        }
    }

    introspect_properties(&mut introspect, properties)?;

    let generics = &input.generics;
    let where_clause = &generics.where_clause;

    let generated_signals_impl = if generated_signals.is_empty() {
        quote!()
    } else {
        quote! {
            impl #generics #self_ty
            #where_clause
            {
                #generated_signals
            }
        }
    };
    let signals_trait_and_impl = if signals_trait_methods.is_empty() {
        quote!()
    } else {
        let signals_trait_name = format_ident!("{}Signals", ty);
        let signals_trait_doc = format!("Trait providing all signal emission methods for `{ty}`.");

        quote! {
            #[doc = #signals_trait_doc]
            #[#zbus::export::async_trait::async_trait]
            pub trait #signals_trait_name {
                #signals_trait_methods
            }

            #[#zbus::export::async_trait::async_trait]
            impl #signals_trait_name for #zbus::object_server::SignalEmitter<'_>
            {
                #signals_emitter_impl_methods
            }

            #[#zbus::export::async_trait::async_trait]
            impl #generics #signals_trait_name for #zbus::object_server::InterfaceRef<#self_ty>
            #where_clause
            {
                #signals_interface_ref_impl_methods
            }
        }
    };

    let proxy = proxy.map(|proxy| proxy.r#gen()).transpose()?;
    let introspect_format_str = format!("{}<interface name=\"{iface_name}\">", "{:indent$}");

    Ok(quote! {
        #input

        #generated_signals_impl

        #signals_trait_and_impl

        #[#zbus::export::async_trait::async_trait]
        impl #generics #zbus::object_server::Interface for #self_ty
        #where_clause
        {
            fn name() -> #zbus::names::InterfaceName<'static> {
                #zbus::names::InterfaceName::from_static_str_unchecked(#iface_name)
            }

            fn spawn_tasks_for_methods(&self) -> bool {
                #with_spawn
            }

            async fn get(
                &self,
                __zbus__property_name: &str,
                __zbus__object_server: &#zbus::ObjectServer,
                __zbus__connection: &#zbus::Connection,
                __zbus__header: Option<&#zbus::message::Header<'_>>,
                __zbus__signal_emitter: &#zbus::object_server::SignalEmitter<'_>,
            ) -> ::std::option::Option<#zbus::fdo::Result<#zbus::zvariant::OwnedValue>> {
                match __zbus__property_name {
                    #get_dispatch
                    _ => ::std::option::Option::None,
                }
            }

            async fn get_all(
                &self,
                __zbus__object_server: &#zbus::ObjectServer,
                __zbus__connection: &#zbus::Connection,
                __zbus__header: Option<&#zbus::message::Header<'_>>,
                __zbus__signal_emitter: &#zbus::object_server::SignalEmitter<'_>,
            ) -> #zbus::fdo::Result<::std::collections::HashMap<
                ::std::string::String,
                #zbus::zvariant::OwnedValue,
            >> {
                let mut props: ::std::collections::HashMap<
                    ::std::string::String,
                    #zbus::zvariant::OwnedValue,
                > = ::std::collections::HashMap::new();
                #get_all
                Ok(props)
            }

            fn set<'call>(
                &'call self,
                __zbus__property_name: &'call str,
                value: &'call #zbus::zvariant::Value<'_>,
                __zbus__object_server: &'call #zbus::ObjectServer,
                __zbus__connection: &'call #zbus::Connection,
                __zbus__header: Option<&'call #zbus::message::Header<'_>>,
                __zbus__signal_emitter: &'call #zbus::object_server::SignalEmitter<'_>,
            ) -> #zbus::object_server::DispatchResult<'call> {
                match __zbus__property_name {
                    #set_dispatch
                    _ => #zbus::object_server::DispatchResult::NotFound,
                }
            }

            async fn set_mut(
                &mut self,
                __zbus__property_name: &str,
                value: &#zbus::zvariant::Value<'_>,
                __zbus__object_server: &#zbus::ObjectServer,
                __zbus__connection: &#zbus::Connection,
                __zbus__header: Option<&#zbus::message::Header<'_>>,
                __zbus__signal_emitter: &#zbus::object_server::SignalEmitter<'_>,
            ) -> ::std::option::Option<#zbus::fdo::Result<()>> {
                match __zbus__property_name {
                    #set_mut_dispatch
                    _ => ::std::option::Option::None,
                }
            }

            fn call<'call>(
                &'call self,
                __zbus__object_server: &'call #zbus::ObjectServer,
                __zbus__connection: &'call #zbus::Connection,
                __zbus__message: &'call #zbus::message::Message,
                name: #zbus::names::MemberName<'call>,
            ) -> #zbus::object_server::DispatchResult<'call> {
                match name.as_str() {
                    #call_dispatch
                    _ => #zbus::object_server::DispatchResult::NotFound,
                }
            }

            fn call_mut<'call>(
                &'call mut self,
                __zbus__object_server: &'call #zbus::ObjectServer,
                __zbus__connection: &'call #zbus::Connection,
                __zbus__message: &'call #zbus::message::Message,
                name: #zbus::names::MemberName<'call>,
            ) -> #zbus::object_server::DispatchResult<'call> {
                match name.as_str() {
                    #call_mut_dispatch
                    _ => #zbus::object_server::DispatchResult::NotFound,
                }
            }

            fn introspect_to_writer(&self, writer: &mut dyn ::std::fmt::Write, level: usize) {
                ::std::writeln!(
                    writer,
                    #introspect_format_str,
                    "",
                    indent = level
                ).unwrap();
                {
                    use #zbus::zvariant::Type;

                    let level = level + 2;
                    #introspect
                }
                ::std::writeln!(writer, r#"{:indent$}</interface>"#, "", indent = level).unwrap();
            }
        }

        #proxy
    })
}

fn get_args_from_inputs(
    inputs: &[PatType],
    method_type: MethodType,
    zbus: &TokenStream,
) -> syn::Result<(TokenStream, TokenStream)> {
    if inputs.is_empty() {
        Ok((quote!(), quote!()))
    } else {
        let mut server_arg_decl = None;
        let mut conn_arg_decl = None;
        let mut header_arg_decl = None;
        let mut signal_emitter_arg_decl = None;
        let mut args_names = Vec::new();
        let mut tys = Vec::new();

        for input in inputs {
            let ArgAttributes {
                object_server,
                connection,
                header,
                signal_emitter,
                signal_context,
            } = ArgAttributes::parse(&input.attrs)?;

            if object_server {
                if server_arg_decl.is_some() {
                    return Err(Error::new_spanned(
                        input,
                        "There can only be one object_server argument",
                    ));
                }

                let server_arg = &input.pat;
                server_arg_decl = Some(quote! { let #server_arg = &__zbus__object_server; });
            } else if connection {
                if conn_arg_decl.is_some() {
                    return Err(Error::new_spanned(
                        input,
                        "There can only be one connection argument",
                    ));
                }

                let conn_arg = &input.pat;
                conn_arg_decl = Some(quote! { let #conn_arg = &__zbus__connection; });
            } else if header {
                if header_arg_decl.is_some() {
                    return Err(Error::new_spanned(
                        input,
                        "There can only be one header argument",
                    ));
                }

                let header_arg = &input.pat;

                header_arg_decl = match method_type {
                    MethodType::Property(_) => Some(quote! {
                        let #header_arg =
                            ::std::option::Option::<&#zbus::message::Header<'_>>::cloned(
                                __zbus__header,
                            );
                    }),
                    _ => Some(quote! { let #header_arg = __zbus__message.header(); }),
                };
            } else if signal_context || signal_emitter {
                if signal_emitter_arg_decl.is_some() {
                    return Err(Error::new_spanned(
                        input,
                        "There can only be one `signal_emitter` or `signal_context` argument",
                    ));
                }

                let signal_context_arg = &input.pat;

                signal_emitter_arg_decl = match method_type {
                    MethodType::Property(_) => Some(
                        quote! { let #signal_context_arg = ::std::clone::Clone::clone(__zbus__signal_emitter); },
                    ),
                    _ => Some(quote! {
                        let #signal_context_arg = match hdr.path() {
                            ::std::option::Option::Some(p) => {
                                #zbus::object_server::SignalEmitter::new(__zbus__connection, p).expect("Infallible conversion failed")
                            }
                            ::std::option::Option::None => {
                                let err = #zbus::fdo::Error::UnknownObject("Path Required".into());
                                return __zbus__connection.reply_dbus_error(&hdr, err).await;
                            }
                        };
                    }),
                };
            } else {
                args_names.push(pat_ident(input).unwrap());
                tys.push(&input.ty);
            }
        }

        let (hdr_init, msg_init, args_decl) = match method_type {
            MethodType::Property(PropertyType::Getter) => (quote! {}, quote! {}, quote! {}),
            MethodType::Property(PropertyType::Setter) => (
                quote! { let hdr = __zbus__header.as_ref().unwrap(); },
                quote! {},
                quote! {},
            ),
            _ => (
                quote! { let hdr = __zbus__message.header(); },
                quote! { let msg_body = __zbus__message.body(); },
                quote! {
                    let (#(#args_names),*): (#(#tys),*) =
                        match msg_body.deserialize() {
                            ::std::result::Result::Ok(r) => r,
                            ::std::result::Result::Err(e) => {
                                let err = <#zbus::fdo::Error as ::std::convert::From<_>>::from(e);
                                return __zbus__connection.reply_dbus_error(&hdr, err).await;
                            }
                        };
                },
            ),
        };

        let args_from_msg = quote! {
            #hdr_init

            #msg_init

            #server_arg_decl

            #conn_arg_decl

            #header_arg_decl

            #signal_emitter_arg_decl

            #args_decl
        };

        let all_args_names = inputs.iter().filter_map(pat_ident);
        let all_args_names = quote! { #(#all_args_names,)* };

        Ok((args_from_msg, all_args_names))
    }
}

// Removes all `zbus` attributes from the given inputs.
fn clear_input_arg_attrs(inputs: &mut Punctuated<FnArg, Token![,]>) {
    for input in inputs {
        if let FnArg::Typed(t) = input {
            t.attrs.retain(|attr| !attr.path().is_ident("zbus"));
        }
    }
}

fn introspect_signal(name: &str, args: &TokenStream) -> TokenStream {
    let format_str = format!("{}<signal name=\"{name}\">", "{:indent$}");
    quote!(
        ::std::writeln!(writer, #format_str, "", indent = level).unwrap();
        {
            let level = level + 2;
            #args
        }
        ::std::writeln!(writer, "{:indent$}</signal>", "", indent = level).unwrap();
    )
}

fn introspect_method(name: &str, args: &TokenStream) -> TokenStream {
    let format_str = format!("{}<method name=\"{name}\">", "{:indent$}");
    quote!(
        ::std::writeln!(writer, #format_str, "", indent = level).unwrap();
        {
            let level = level + 2;
            #args
        }
        ::std::writeln!(writer, "{:indent$}</method>", "", indent = level).unwrap();
    )
}

fn introspect_input_args<'i>(
    inputs: &'i [PatType],
    is_signal: bool,
    cfg_attrs: &'i [&'i syn::Attribute],
) -> impl Iterator<Item = TokenStream> + 'i {
    inputs
        .iter()
        .filter_map(move |pat_type @ PatType { ty, attrs, .. }| {
            if is_special_arg(attrs) {
                return None;
            }

            let ident = pat_ident(pat_type).unwrap();
            let arg_name = quote!(#ident).to_string();
            let arg_name = arg_name.strip_prefix("r#").unwrap_or(arg_name.as_str());
            let dir = if is_signal { "" } else { " direction=\"in\"" };
            let format_str = format!(
                "{}<arg name=\"{arg_name}\" type=\"{}\"{dir}/>",
                "{:indent$}", "{}",
            );
            Some(quote!(
                #(#cfg_attrs)*
                ::std::writeln!(writer, #format_str, "", <#ty>::SIGNATURE, indent = level).unwrap();
            ))
        })
}

fn count_regular_args(inputs: &[PatType]) -> usize {
    inputs
        .iter()
        .filter(|PatType { attrs, .. }| !is_special_arg(attrs))
        .count()
}

fn is_special_arg(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("zbus") {
            return false;
        }

        let Ok(list) = &attr.meta.require_list() else {
            return false;
        };
        let Ok(nested) = list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
        else {
            return false;
        };

        nested.iter().any(|nested_meta| {
            matches!(
                nested_meta,
                Meta::Path(path)
                if path.is_ident("object_server") ||
                    path.is_ident("connection") ||
                    path.is_ident("header") ||
                    path.is_ident("signal_context") ||
                    path.is_ident("signal_emitter")
            )
        })
    })
}

fn introspect_output_arg(
    ty: &Type,
    arg_name: Option<&String>,
    cfg_attrs: &[&syn::Attribute],
) -> TokenStream {
    let arg_name_attr = match arg_name {
        Some(name) => format!("name=\"{name}\" "),
        None => String::from(""),
    };

    let format_str = format!(
        "{}<arg {arg_name_attr}type=\"{}\" direction=\"out\"/>",
        "{:indent$}", "{}",
    );
    quote!(
        #(#cfg_attrs)*
        ::std::writeln!(writer, #format_str, "", <#ty>::SIGNATURE, indent = level).unwrap();
    )
}

fn get_result_inner_type(p: &TypePath) -> syn::Result<&Type> {
    if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = &p
        .path
        .segments
        .last()
        .ok_or_else(|| Error::new_spanned(p, "unsupported result type"))?
        .arguments
    {
        if let Some(syn::GenericArgument::Type(ty)) = args.first() {
            return Ok(ty);
        }
    }

    Err(Error::new_spanned(p, "unhandled Result return"))
}

fn introspect_add_output_args(
    args: &mut TokenStream,
    output: &ReturnType,
    arg_names: Option<&[String]>,
    cfg_attrs: &[&syn::Attribute],
) -> syn::Result<bool> {
    let mut is_result_output = false;

    if let ReturnType::Type(_, ty) = output {
        let mut ty = ty.as_ref();

        if let Type::Path(p) = ty {
            is_result_output = p
                .path
                .segments
                .last()
                .ok_or_else(|| Error::new_spanned(ty, "unsupported output type"))?
                .ident
                == "Result";
            if is_result_output {
                ty = get_result_inner_type(p)?;
            }
        }

        if let Type::Tuple(t) = ty {
            if let Some(arg_names) = arg_names {
                if t.elems.len() != arg_names.len() {
                    return Err(Error::new_spanned(
                        ty,
                        format!(
                            "out_args specifies {} names but method returns {} values",
                            arg_names.len(),
                            t.elems.len()
                        ),
                    ));
                }
            }
            for i in 0..t.elems.len() {
                let name = arg_names.map(|names| &names[i]);
                args.extend(introspect_output_arg(&t.elems[i], name, cfg_attrs));
            }
        } else {
            // Note: The type might still serialize as a DBus struct/tuple if it has a custom
            // signature (e.g., `#[zvariant(signature = "(...)")]`), but we can't detect this
            // from the Rust type structure alone.
            //
            // If out_args has exactly one name, apply it to this output.
            // If out_args has multiple names, the user likely has a type with a tuple signature,
            // but we can't generate separate <arg> elements for each field since we don't have
            // access to the individual field types. In this case, we generate a single <arg>
            // with the type's full signature and no name. The signature will be correct at runtime.
            let name = arg_names.and_then(|names| {
                if names.len() == 1 {
                    Some(&names[0])
                } else {
                    None
                }
            });
            args.extend(introspect_output_arg(ty, name, cfg_attrs));
        }
    }

    Ok(is_result_output)
}

fn get_return_type(output: &ReturnType) -> syn::Result<&Type> {
    if let ReturnType::Type(_, ty) = output {
        let ty = ty.as_ref();

        if let Type::Path(p) = ty {
            let is_result_output = p
                .path
                .segments
                .last()
                .ok_or_else(|| Error::new_spanned(ty, "unsupported property type"))?
                .ident
                == "Result";
            if is_result_output {
                return get_result_inner_type(p);
            }
        }

        Ok(ty)
    } else {
        Err(Error::new_spanned(output, "Invalid return type"))
    }
}

fn introspect_properties(
    introspection: &mut TokenStream,
    properties: BTreeMap<String, Property>,
) -> syn::Result<()> {
    for (name, prop) in properties {
        let access = if prop.read && prop.write {
            "readwrite"
        } else if prop.read {
            "read"
        } else if prop.write {
            "write"
        } else {
            unreachable!("Properties should have at least one access type");
        };
        let ty = prop.ty.unwrap();

        let doc_comments = prop.doc_comments;
        if prop.emits_changed_signal == PropertyEmitsChangedSignal::True {
            let format_str = format!(
                "{}<property name=\"{name}\" type=\"{}\" access=\"{access}\"/>",
                "{:indent$}", "{}",
            );
            introspection.extend(quote!(
                #doc_comments
                ::std::writeln!(writer, #format_str, "", <#ty>::SIGNATURE, indent = level).unwrap();
            ));
        } else {
            let emits_changed_signal = prop.emits_changed_signal.to_string();
            let annot_name = "org.freedesktop.DBus.Property.EmitsChangedSignal";
            let format_str = format!(
                "{}<property name=\"{name}\" type=\"{}\" access=\"{access}\">\n\
                    {}<annotation name=\"{annot_name}\" value=\"{emits_changed_signal}\"/>\n\
                {}</property>",
                "{:indent$}", "{}", "{:annot_indent$}", "{:indent$}",
            );
            introspection.extend(quote!(
                #doc_comments
                ::std::writeln!(
                    writer,
                    #format_str,
                    "", <#ty>::SIGNATURE, "", "", indent = level, annot_indent = level + 2,
                ).unwrap();
            ));
        }
    }

    Ok(())
}

pub fn to_xml_docs(lines: Vec<String>) -> TokenStream {
    let mut docs = quote!();

    let mut lines: Vec<&str> = lines
        .iter()
        .skip_while(|s| is_blank(s))
        .flat_map(|s| s.split('\n'))
        .collect();

    while let Some(true) = lines.last().map(|s| is_blank(s)) {
        lines.pop();
    }

    if lines.is_empty() {
        return docs;
    }

    docs.extend(quote!(::std::writeln!(writer, "{:indent$}<!--", "", indent = level).unwrap();));
    for line in lines {
        if !line.is_empty() {
            docs.extend(
                quote!(::std::writeln!(writer, "{:indent$}{}", "", #line, indent = level).unwrap();),
            );
        } else {
            docs.extend(quote!(::std::writeln!(writer, "").unwrap();));
        }
    }
    docs.extend(quote!(::std::writeln!(writer, "{:indent$} -->", "", indent = level).unwrap();));

    docs
}

// Like ImplItemFn, but with a semicolon at the end instead of a body block
struct ImplItemSignal {
    attrs: Vec<Attribute>,
    vis: Visibility,
    sig: Signature,
}

impl Parse for ImplItemSignal {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        let sig = input.parse()?;
        let _: Token![;] = input.parse()?;

        Ok(ImplItemSignal { attrs, vis, sig })
    }
}

#[derive(Debug)]
struct Proxy {
    // The type name
    ty: Ident,
    // The interface name
    iface_name: String,
    // The zbus crate
    zbus: TokenStream,

    // Input
    attrs: ProxyAttributes,

    // Output
    methods: TokenStream,
}

impl Proxy {
    fn new(ty: &Ident, iface_name: &str, attrs: ProxyAttributes, zbus: &TokenStream) -> Self {
        Self {
            iface_name: iface_name.to_string(),
            ty: ty.clone(),
            zbus: zbus.clone(),
            attrs,
            methods: quote!(),
        }
    }

    fn add_method(
        &mut self,
        method_info: MethodInfo,
        properties: &BTreeMap<String, Property>,
    ) -> syn::Result<()> {
        let inputs: Punctuated<PatType, Comma> = method_info
            .typed_inputs
            .iter()
            .filter(|input| {
                let a = ArgAttributes::parse(&input.attrs).unwrap();
                !a.object_server
                    && !a.connection
                    && !a.header
                    && !a.signal_context
                    && !a.signal_emitter
            })
            .cloned()
            .collect();
        let zbus = &self.zbus;
        let ret = match &method_info.output {
            ReturnType::Type(_, ty) => {
                let ty = ty.as_ref();

                if let Type::Path(p) = ty {
                    let is_result_output = p
                        .path
                        .segments
                        .last()
                        .ok_or_else(|| Error::new_spanned(ty, "unsupported return type"))?
                        .ident
                        == "Result";
                    if is_result_output {
                        let is_prop = matches!(method_info.method_type, MethodType::Property(_));

                        if is_prop {
                            // Proxy methods always return `zbus::Result<T>`
                            let inner_ty = get_result_inner_type(p)?;
                            quote! { #zbus::Result<#inner_ty> }
                        } else {
                            quote! { #ty }
                        }
                    } else {
                        quote! { #zbus::Result<#ty> }
                    }
                } else {
                    quote! { #zbus::Result<#ty> }
                }
            }
            ReturnType::Default => quote! { #zbus::Result<()> },
        };
        let ident = &method_info.ident;
        let member_name = method_info.member_name;
        let mut proxy_method_attrs = quote! { name = #member_name, };
        proxy_method_attrs.extend(match method_info.method_type {
            MethodType::Signal => quote!(signal,),
            MethodType::Property(_) => {
                let emits_changed_signal = properties
                    .get(&member_name)
                    .unwrap()
                    .emits_changed_signal
                    .to_string();
                let emits_changed_signal = quote! { emits_changed_signal = #emits_changed_signal };

                quote! { property(#emits_changed_signal), }
            }
            MethodType::Other => quote!(),
        });
        if let Some(attrs) = method_info.proxy_attrs {
            if let Some(object) = attrs.object {
                proxy_method_attrs.extend(quote! { object = #object, });
            }
            if let Some(async_object) = attrs.async_object {
                proxy_method_attrs.extend(quote! { async_object = #async_object, });
            }
            if let Some(blocking_object) = attrs.blocking_object {
                proxy_method_attrs.extend(quote! { blocking_object = #blocking_object, });
            }
            if attrs.object_vec {
                proxy_method_attrs.extend(quote! { object_vec, });
            }
            if attrs.no_reply {
                proxy_method_attrs.extend(quote! { no_reply, });
            }
            if attrs.no_autostart {
                proxy_method_attrs.extend(quote! { no_autostart, });
            }
            if attrs.allow_interactive_auth {
                proxy_method_attrs.extend(quote! { allow_interactive_auth, });
            }
        }
        let cfg_attrs = method_info.cfg_attrs;
        let doc_attrs = method_info.doc_attrs;
        self.methods.extend(quote! {
            #(#cfg_attrs)*
            #(#doc_attrs)*
            #[zbus(#proxy_method_attrs)]
            fn #ident(&self, #inputs) -> #ret;
        });

        Ok(())
    }

    fn r#gen(&self) -> syn::Result<TokenStream> {
        let attrs = &self.attrs;
        let (
            assume_defaults,
            default_path,
            default_service,
            async_name,
            blocking_name,
            gen_async,
            gen_blocking,
            ty,
            methods,
        ) = (
            attrs
                .assume_defaults
                .map(|value| quote! { assume_defaults = #value, }),
            attrs
                .default_path
                .as_ref()
                .map(|value| quote! { default_path = #value, }),
            attrs
                .default_service
                .as_ref()
                .map(|value| quote! { default_service = #value, }),
            attrs
                .async_name
                .as_ref()
                .map(|value| quote! { async_name = #value, }),
            attrs
                .blocking_name
                .as_ref()
                .map(|value| quote! { blocking_name = #value, }),
            attrs.gen_async.map(|value| quote! { gen_async = #value, }),
            attrs
                .gen_blocking
                .map(|value| quote! { gen_blocking = #value, }),
            &self.ty,
            &self.methods,
        );
        let iface_name = &self.iface_name;
        let vis = match &self.attrs.visibility {
            Some(s) => parse_str::<Visibility>(s)?,
            None => Visibility::Public(Token![pub](ty.span())),
        };
        let zbus = &self.zbus;
        let proxy_doc = format!("Proxy for the `{iface_name}` interface.");
        Ok(quote! {
            #[doc = #proxy_doc]
            #[#zbus::proxy(
                name = #iface_name,
                #assume_defaults
                #default_path
                #default_service
                #async_name
                #blocking_name
                #gen_async
                #gen_blocking
            )]
            #vis trait #ty {
                #methods
            }
        })
    }
}
