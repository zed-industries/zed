use super::*;

#[derive(Clone, Debug)]
pub struct CppMethod {
    pub def: MethodDef,
    pub signature: Signature,
    pub dependencies: TypeMap,
    pub return_hint: ReturnHint,
    pub param_hints: Vec<ParamHint>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReturnHint {
    None,
    Query(usize, usize),
    QueryOptional(usize, usize),
    ResultValue,
    ResultVoid,
    ReturnStruct,
    ReturnValue,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ParamHint {
    None,
    ArrayFixed(usize),
    ArrayRelativeLen(usize),
    ArrayRelativeByteLen(usize),
    ArrayRelativePtr(usize),
    IntoParam,
    Optional,
    ValueType,
    Blittable,
    Bool,
}

impl From<&Param> for ParamHint {
    fn from(param: &Param) -> Self {
        for attribute in param.def.attributes() {
            match attribute.name() {
                "NativeArrayInfoAttribute" => {
                    for (_, value) in attribute.args() {
                        match value {
                            Value::I16(value) => {
                                return ParamHint::ArrayRelativeLen(value as usize)
                            }
                            Value::I32(value) => return ParamHint::ArrayFixed(value as usize),
                            _ => {}
                        }
                    }
                }
                "MemorySizeAttribute" => {
                    for (_, value) in attribute.args() {
                        if let Value::I16(value) = value {
                            return ParamHint::ArrayRelativeByteLen(value as usize);
                        }
                    }
                }
                _ => {}
            }
        }
        ParamHint::None
    }
}

impl ParamHint {
    fn is_array(&self) -> bool {
        matches!(
            self,
            Self::ArrayFixed(_)
                | Self::ArrayRelativeLen(_)
                | Self::ArrayRelativeByteLen(_)
                | Self::ArrayRelativePtr(_)
        )
    }
}

impl CppMethod {
    pub fn new(def: MethodDef, namespace: &'static str) -> Self {
        let signature = def.signature(namespace, &[]);
        let dependencies = signature.dependencies();
        let mut param_hints = vec![ParamHint::None; signature.params.len()];

        for (position, param) in signature.params.iter().enumerate() {
            param_hints[position] = param.into();
        }

        for position in 0..signature.params.len() {
            // Point len params back to the corresponding ptr params.
            match param_hints[position] {
                ParamHint::ArrayRelativeLen(relative)
                | ParamHint::ArrayRelativeByteLen(relative) => {
                    // The len params must be input only.
                    if signature.params[relative].is_input()
                        && position != relative
                        && !signature.params[relative].is_pointer()
                    {
                        param_hints[relative] = ParamHint::ArrayRelativePtr(position);
                    } else {
                        param_hints[position] = ParamHint::None;
                    }
                }
                _ => {}
            }
        }

        let mut sets = BTreeMap::<usize, Vec<usize>>::new();

        // Finds sets of ptr params pointing at the same len param.
        for (position, hint) in param_hints.iter().enumerate() {
            match hint {
                ParamHint::ArrayRelativeLen(relative)
                | ParamHint::ArrayRelativeByteLen(relative) => {
                    sets.entry(*relative).or_default().push(position);
                }
                _ => {}
            }
        }

        // Remove all sets.
        for (len, ptrs) in sets {
            if ptrs.len() > 1 {
                param_hints[len] = ParamHint::None;
                for ptr in ptrs {
                    param_hints[ptr] = ParamHint::None;
                }
            }
        }

        // Remove any byte arrays that aren't byte-sized types.
        for position in 0..param_hints.len() {
            if let ParamHint::ArrayRelativeByteLen(relative) = param_hints[position] {
                if !signature.params[position].is_byte_size() {
                    param_hints[position] = ParamHint::None;
                    param_hints[relative] = ParamHint::None;
                }
            }
        }

        for (position, hint) in param_hints.iter_mut().enumerate() {
            if *hint == ParamHint::None {
                let param = &signature.params[position];

                if param.is_convertible() && !hint.is_array() {
                    *hint = ParamHint::IntoParam;
                } else if param.is_copyable() && param.is_optional() {
                    *hint = ParamHint::Optional;
                } else if param.is_input() && param.ty == Type::BOOL {
                    *hint = ParamHint::Bool;
                } else if param.is_primitive()
                    && (!param.is_pointer() || param.deref().is_copyable())
                {
                    *hint = ParamHint::ValueType;
                } else if param.is_copyable() {
                    *hint = ParamHint::Blittable;
                }
            }
        }

        let is_retval = signature.is_retval();
        let mut return_hint = ReturnHint::None;

        let last_error = if let Some(map) = def.impl_map() {
            map.flags().contains(PInvokeAttributes::SupportsLastError)
        } else {
            false
        };

        if !def.has_attribute("CanReturnMultipleSuccessValuesAttribute") {
            match &signature.return_type {
                Type::Void if is_retval => return_hint = ReturnHint::ReturnValue,
                Type::HRESULT => {
                    if is_retval {
                        return_hint = ReturnHint::ResultValue
                    } else {
                        return_hint = ReturnHint::ResultVoid
                    };

                    if signature.params.len() >= 2 {
                        if let Some((guid, object)) = signature_param_is_query(&signature.params) {
                            if signature.params[object].is_optional() {
                                return_hint = ReturnHint::QueryOptional(object, guid);
                            } else {
                                return_hint = ReturnHint::Query(object, guid);
                            }
                        }
                    }
                }
                Type::BOOL => {
                    if last_error {
                        return_hint = ReturnHint::ResultVoid
                    }
                }
                Type::GUID => return_hint = ReturnHint::ReturnStruct,
                Type::CppStruct(ty) if !ty.is_handle() => return_hint = ReturnHint::ReturnStruct,
                _ => {}
            };
        }

        Self {
            def,
            signature,
            dependencies,
            param_hints,
            return_hint,
        }
    }

    pub fn write_cfg(&self, config: &Config<'_>, parent: &Cfg, not: bool) -> TokenStream {
        if !config.package {
            return quote! {};
        }

        parent
            .difference(self.def, &self.dependencies, config)
            .write(config, not)
    }

    pub fn write_generics(&self) -> TokenStream {
        let mut tokens = quote! {};

        for (position, hint) in self.param_hints.iter().enumerate() {
            if *hint == ParamHint::IntoParam {
                let name: TokenStream = format!("P{position},").into();
                tokens.combine(name);
            }
        }

        tokens
    }

    pub fn write_where(&self, config: &Config<'_>, query: bool) -> TokenStream {
        let mut tokens = quote! {};

        for (position, param) in self.signature.params.iter().enumerate() {
            if self.param_hints[position] == ParamHint::IntoParam {
                let name: TokenStream = format!("P{position}").into();
                let into = param.write_name(config);
                tokens.combine(quote! { #name: windows_core::Param<#into>, })
            }
        }

        if query {
            tokens.combine(quote! { T: windows_core::Interface })
        }

        if tokens.is_empty() {
            tokens
        } else {
            quote! { where #tokens }
        }
    }

    pub fn write(
        &self,
        config: &Config<'_>,
        method_names: &mut MethodNames,
        virtual_names: &mut MethodNames,
    ) -> TokenStream {
        let name = method_names.add(self.def);
        let vname = virtual_names.add(self.def);

        let args = self.write_args();
        let params = self.write_params(config);
        let generics = self.write_generics();
        let abi_return_type = self.write_return(config);

        match self.return_hint {
            ReturnHint::Query(..) => {
                let where_clause = self.write_where(config, true);

                quote! {
                    pub unsafe fn #name<#generics T>(&self, #params) -> windows_core::Result<T> #where_clause {
                        let mut result__ = core::ptr::null_mut();
                        unsafe { (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self),#args).and_then(||windows_core::Type::from_abi(result__)) }
                    }
                }
            }
            ReturnHint::QueryOptional(..) => {
                let where_clause = self.write_where(config, true);

                quote! {
                    pub unsafe fn #name<#generics T>(&self, #params result__: *mut Option<T>) -> windows_core::Result<()> #where_clause {
                        unsafe { (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self),#args).ok() }
                    }
                }
            }
            ReturnHint::ResultValue => {
                let where_clause = self.write_where(config, false);

                let return_type = self.signature.params[self.signature.params.len() - 1].deref();

                let map = return_type.write_result_map();
                let return_type = return_type.write_name(config);

                quote! {
                    pub unsafe fn #name<#generics>(&self, #params) -> windows_core::Result<#return_type> #where_clause {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self),#args).#map
                        }
                    }
                }
            }
            ReturnHint::ResultVoid => {
                let where_clause = self.write_where(config, false);

                quote! {
                    pub unsafe fn #name<#generics>(&self, #params) -> windows_core::Result<()> #where_clause {
                        unsafe { (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self),#args).ok() }
                    }
                }
            }
            ReturnHint::ReturnValue => {
                let where_clause = self.write_where(config, false);

                let return_type = self.signature.params[self.signature.params.len() - 1].deref();

                if return_type.is_interface() {
                    let return_type = return_type.write_name(config);

                    quote! {
                        pub unsafe fn #name<#generics>(&self, #params) -> windows_core::Result<#return_type> #where_clause {
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args);
                                windows_core::Type::from_abi(result__)
                            }
                        }
                    }
                } else {
                    let map = if return_type.is_copyable() {
                        quote! { result__ }
                    } else {
                        quote! { core::mem::transmute(result__) }
                    };

                    let return_type = return_type.write_name(config);
                    let where_clause = self.write_where(config, false);

                    quote! {
                        pub unsafe fn #name<#generics>(&self, #params) -> #return_type #where_clause {
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args);
                                #map
                            }
                        }
                    }
                }
            }
            ReturnHint::ReturnStruct => {
                let return_type = self.signature.return_type.write_name(config);
                let where_clause = self.write_where(config, false);

                quote! {
                    pub unsafe fn #name<#generics>(&self, #params) -> #return_type #where_clause {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), &mut result__, #args);
                            result__
                        }
                    }
                }
            }
            ReturnHint::None => {
                let where_clause = self.write_where(config, false);

                quote! {
                    pub unsafe fn #name<#generics>(&self, #params) #abi_return_type #where_clause {
                        unsafe { (windows_core::Interface::vtable(self).#vname)(windows_core::Interface::as_raw(self), #args) }
                    }
                }
            }
        }
    }

    pub fn write_upcall(&self, parent_impl: &TokenStream, name: &TokenStream) -> TokenStream {
        match self.return_hint {
            ReturnHint::ResultValue => {
                let invoke_args = self.signature.params[..self.signature.params.len() - 1]
                    .iter()
                    .map(write_invoke_arg);

                let result = self.signature.params[self.signature.params.len() - 1].write_ident();

                quote! {
                    match #parent_impl::#name(this, #(#invoke_args,)*) {
                        Ok(ok__) => {
                            // use `ptr::write` since the result could be uninitialized
                            #result.write(core::mem::transmute(ok__));
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into()
                    }
                }
            }
            ReturnHint::Query(..) | ReturnHint::QueryOptional(..) | ReturnHint::ResultVoid => {
                let invoke_args = self.signature.params.iter().map(write_invoke_arg);

                quote! {
                    #parent_impl::#name(this, #(#invoke_args,)*).into()
                }
            }
            ReturnHint::ReturnStruct => {
                let invoke_args = self.signature.params.iter().map(write_invoke_arg);

                quote! {
                    *result__ = #parent_impl::#name(this, #(#invoke_args,)*)
                }
            }
            _ => {
                let invoke_args = self.signature.params.iter().map(write_invoke_arg);

                quote! {
                    #parent_impl::#name(this, #(#invoke_args,)*)
                }
            }
        }
    }

    pub fn write_impl_signature(&self, config: &Config<'_>, _named_params: bool) -> TokenStream {
        let mut params = quote! {};

        if self.return_hint == ReturnHint::ResultValue {
            for (position, param) in self.signature.params[..self.signature.params.len() - 1]
                .iter()
                .enumerate()
            {
                params.combine(write_produce_type(
                    config,
                    param,
                    self.param_hints[position],
                ));
            }
        } else {
            for (position, param) in self.signature.params.iter().enumerate() {
                params.combine(write_produce_type(
                    config,
                    param,
                    self.param_hints[position],
                ));
            }
        }

        let return_type = match self.return_hint {
            ReturnHint::Query(..) | ReturnHint::QueryOptional(..) | ReturnHint::ResultVoid => {
                quote! { -> windows_core::Result<()> }
            }
            ReturnHint::ResultValue => {
                let return_type = self.signature.params[self.signature.params.len() - 1].deref();
                let return_type = return_type.write_name(config);

                quote! { -> windows_core::Result<#return_type> }
            }
            _ => self.write_return(config),
        };

        quote! { (&self, #params) #return_type }
    }

    pub fn write_abi(&self, config: &Config<'_>, named_params: bool) -> TokenStream {
        let mut params: Vec<_> = self
            .signature
            .params
            .iter()
            .map(|param| {
                let ty = param.write_abi(config);

                if named_params {
                    let name = param.write_ident();
                    quote! { #name: #ty }
                } else {
                    ty
                }
            })
            .collect();

        let mut return_sig = quote! {};

        match self.return_hint {
            ReturnHint::ReturnStruct => {
                let return_type = self.signature.return_type.write_abi(config);

                if named_params {
                    params.insert(0, quote! { result__: *mut #return_type });
                } else {
                    params.insert(0, quote! { *mut #return_type });
                }
            }
            _ => {
                return_sig = config.write_return_sig(self.def, &self.signature, false);
            }
        }

        let this = if named_params {
            quote! { this: *mut core::ffi::c_void }
        } else {
            quote! { *mut core::ffi::c_void }
        };

        quote! { (#this, #(#params),*) #return_sig }
    }

    pub fn write_params(&self, config: &Config<'_>) -> TokenStream {
        let mut tokens = quote! {};

        for (position, param) in self.signature.params.iter().enumerate() {
            match self.return_hint {
                ReturnHint::Query(object, guid) | ReturnHint::QueryOptional(object, guid) => {
                    if object == position || guid == position {
                        continue;
                    }
                }
                ReturnHint::ReturnValue | ReturnHint::ResultValue
                    if self.signature.params.len() - 1 == position =>
                {
                    continue;
                }
                _ => {}
            }

            let name = param.write_ident();

            match self.param_hints[position] {
                ParamHint::ArrayFixed(fixed) => {
                    let ty = param.deref();
                    let ty = ty.write_default(config);
                    let len = Literal::u32_unsuffixed(fixed as u32);
                    let ty = if param.is_input() {
                        quote! { &[#ty; #len] }
                    } else {
                        quote! { &mut [#ty; #len] }
                    };
                    if param.is_optional() {
                        tokens.combine(&quote! { #name: Option<#ty>, });
                    } else {
                        tokens.combine(&quote! { #name: #ty, });
                    }
                }
                ParamHint::ArrayRelativeLen(_) => {
                    let ty = param.deref();
                    let ty = ty.write_default(config);
                    let ty = if param.is_input() {
                        quote! { &[#ty] }
                    } else {
                        quote! { &mut [#ty] }
                    };
                    if param.is_optional() {
                        tokens.combine(&quote! { #name: Option<#ty>, });
                    } else {
                        tokens.combine(&quote! { #name: #ty, });
                    }
                }
                ParamHint::ArrayRelativeByteLen(_) => {
                    let ty = if param.is_input() {
                        quote! { &[u8] }
                    } else {
                        quote! { &mut [u8] }
                    };
                    if param.is_optional() {
                        tokens.combine(&quote! { #name: Option<#ty>, });
                    } else {
                        tokens.combine(&quote! { #name: #ty, });
                    }
                }
                ParamHint::ArrayRelativePtr(_) => {}
                ParamHint::IntoParam => {
                    let kind: TokenStream = format!("P{position}").into();
                    tokens.combine(&quote! { #name: #kind, });
                }
                ParamHint::Optional => {
                    if matches!(param.ty, Type::CppDelegate(..)) {
                        let kind = param.write_name(config);
                        tokens.combine(&quote! { #name: #kind, });
                    } else {
                        let kind = param.write_name(config);
                        tokens.combine(&quote! { #name: Option<#kind>, });
                    }
                }
                ParamHint::Bool => {
                    tokens.combine(&quote! { #name: bool, });
                }
                ParamHint::ValueType | ParamHint::Blittable => {
                    let kind = param.write_default(config);
                    tokens.combine(&quote! { #name: #kind, });
                }
                ParamHint::None => {
                    let kind = param.write_default(config);
                    tokens.combine(&quote! { #name: &#kind, });
                }
            }
        }

        tokens
    }

    pub fn write_args(&self) -> TokenStream {
        let mut tokens = quote! {};

        for (position, param) in self.signature.params.iter().enumerate() {
            let new = match self.return_hint {
                ReturnHint::Query(object, _) if object == position => {
                    quote! { &mut result__, }
                }
                ReturnHint::ReturnValue | ReturnHint::ResultValue
                    if self.signature.params.len() - 1 == position =>
                {
                    quote! { &mut result__, }
                }
                ReturnHint::QueryOptional(object, _) if object == position => {
                    quote! { result__ as *mut _ as *mut _, }
                }
                ReturnHint::Query(_, guid) | ReturnHint::QueryOptional(_, guid)
                    if guid == position =>
                {
                    quote! { &T::IID, }
                }
                _ => {
                    let name = param.write_ident();
                    match self.param_hints[position] {
                        ParamHint::ArrayFixed(_)
                        | ParamHint::ArrayRelativeLen(_)
                        | ParamHint::ArrayRelativeByteLen(_) => {
                            let map = if param.is_optional() {
                                quote! { #name.as_deref().map_or(core::ptr::null(), |slice|slice.as_ptr()) }
                            } else {
                                quote! { #name.as_ptr() }
                            };
                            quote! { core::mem::transmute(#map), }
                        }
                        ParamHint::ArrayRelativePtr(relative) => {
                            let relative_param = &self.signature.params[relative];
                            let name = relative_param.write_ident();
                            if relative_param.is_optional() {
                                quote! { #name.as_deref().map_or(0, |slice|slice.len().try_into().unwrap()), }
                            } else {
                                quote! { #name.len().try_into().unwrap(), }
                            }
                        }
                        ParamHint::IntoParam => {
                            quote! { #name.param().abi(), }
                        }
                        ParamHint::Optional => {
                            if matches!(param.ty, Type::CppDelegate(..)) {
                                quote! { #name, }
                            } else {
                                quote! { #name.unwrap_or(core::mem::zeroed()) as _, }
                            }
                        }
                        ParamHint::Bool => {
                            quote! { #name.into(), }
                        }
                        ParamHint::ValueType => {
                            if param.is_input() {
                                quote! { #name, }
                            } else {
                                quote! { #name as _, }
                            }
                        }
                        ParamHint::Blittable => {
                            if matches!(param.ty, Type::PrimitiveOrEnum(_, _)) {
                                quote! { #name.0 as _, }
                            } else {
                                quote! { core::mem::transmute(#name), }
                            }
                        }
                        ParamHint::None => {
                            quote! { core::mem::transmute_copy(#name), }
                        }
                    }
                }
            };
            tokens.combine(&new)
        }

        tokens
    }

    pub fn write_return(&self, config: &Config<'_>) -> TokenStream {
        match &self.signature.return_type {
            Type::Void if self.def.has_attribute("DoesNotReturnAttribute") => quote! {  -> ! },
            Type::Void => quote! {},
            ty => {
                let ty = ty.write_default(config);
                quote! { -> #ty }
            }
        }
    }

    pub fn handle_last_error(&self) -> bool {
        if let Some(map) = self.def.impl_map() {
            if map.flags().contains(PInvokeAttributes::SupportsLastError) {
                if let Type::CppStruct(ty) = &self.signature.return_type {
                    if ty.is_handle() {
                        // https://github.com/microsoft/windows-rs/issues/2392#issuecomment-1477765781
                        if self.def.name() == "LocalFree" {
                            return false;
                        }
                        if ty.def.underlying_type().is_pointer() {
                            return true;
                        }
                        if !ty.def.invalid_values().is_empty() {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

fn write_produce_type(config: &Config<'_>, param: &Param, hint: ParamHint) -> TokenStream {
    let name = param.write_ident();
    let kind = param.write_default(config);

    if param.is_input() && param.is_interface() {
        let type_name = param.write_name(config);
        quote! { #name: windows_core::Ref<'_, #type_name>, }
    } else if !param.is_input() && param.deref().is_interface() && !hint.is_array() {
        let type_name = param.deref().write_name(config);
        quote! { #name: windows_core::OutRef<'_, #type_name>, }
    } else if param.is_input() {
        if param.is_primitive() {
            quote! { #name: #kind, }
        } else {
            quote! { #name: &#kind, }
        }
    } else {
        quote! { #name: #kind, }
    }
}

fn write_invoke_arg(param: &Param) -> TokenStream {
    let name = param.write_ident();

    if param.is_input() && param.is_interface() {
        quote! { core::mem::transmute_copy(&#name) }
    } else if (!param.is_pointer() && param.is_interface())
        || (param.is_input() && !param.is_primitive())
    {
        quote! { core::mem::transmute(&#name) }
    } else {
        quote! { core::mem::transmute_copy(&#name) }
    }
}

fn signature_param_is_query(params: &[Param]) -> Option<(usize, usize)> {
    if let Some(guid) = params
        .iter()
        .rposition(|param| param.ty == Type::PtrConst(Box::new(Type::GUID), 1) && param.is_input())
    {
        if let Some(object) = params.iter().rposition(|param| {
            param.ty == Type::PtrMut(Box::new(Type::Void), 2)
                && param.def.has_attribute("ComOutPtrAttribute")
        }) {
            return Some((guid, object));
        }
    }

    None
}
