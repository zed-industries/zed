#[cfg(feature = "Win32_System_Com")]
pub trait IMXAttributes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn addAttribute(&self, struri: &windows_core::BSTR, strlocalname: &windows_core::BSTR, strqname: &windows_core::BSTR, strtype: &windows_core::BSTR, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn addAttributeFromIndex(&self, varatts: &windows_core::VARIANT, nindex: i32) -> windows_core::Result<()>;
    fn clear(&self) -> windows_core::Result<()>;
    fn removeAttribute(&self, nindex: i32) -> windows_core::Result<()>;
    fn setAttribute(&self, nindex: i32, struri: &windows_core::BSTR, strlocalname: &windows_core::BSTR, strqname: &windows_core::BSTR, strtype: &windows_core::BSTR, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn setAttributes(&self, varatts: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn setLocalName(&self, nindex: i32, strlocalname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn setQName(&self, nindex: i32, strqname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn setType(&self, nindex: i32, strtype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn setURI(&self, nindex: i32, struri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn setValue(&self, nindex: i32, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMXAttributes {}
#[cfg(feature = "Win32_System_Com")]
impl IMXAttributes_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>() -> IMXAttributes_Vtbl {
        unsafe extern "system" fn addAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, struri: core::mem::MaybeUninit<windows_core::BSTR>, strlocalname: core::mem::MaybeUninit<windows_core::BSTR>, strqname: core::mem::MaybeUninit<windows_core::BSTR>, strtype: core::mem::MaybeUninit<windows_core::BSTR>, strvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXAttributes_Impl::addAttribute(this, core::mem::transmute(&struri), core::mem::transmute(&strlocalname), core::mem::transmute(&strqname), core::mem::transmute(&strtype), core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn addAttributeFromIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varatts: core::mem::MaybeUninit<windows_core::VARIANT>, nindex: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXAttributes_Impl::addAttributeFromIndex(this, core::mem::transmute(&varatts), core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn clear<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXAttributes_Impl::clear(this).into()
        }
        unsafe extern "system" fn removeAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXAttributes_Impl::removeAttribute(this, core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn setAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, struri: core::mem::MaybeUninit<windows_core::BSTR>, strlocalname: core::mem::MaybeUninit<windows_core::BSTR>, strqname: core::mem::MaybeUninit<windows_core::BSTR>, strtype: core::mem::MaybeUninit<windows_core::BSTR>, strvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXAttributes_Impl::setAttribute(this, core::mem::transmute_copy(&nindex), core::mem::transmute(&struri), core::mem::transmute(&strlocalname), core::mem::transmute(&strqname), core::mem::transmute(&strtype), core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn setAttributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varatts: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXAttributes_Impl::setAttributes(this, core::mem::transmute(&varatts)).into()
        }
        unsafe extern "system" fn setLocalName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, strlocalname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXAttributes_Impl::setLocalName(this, core::mem::transmute_copy(&nindex), core::mem::transmute(&strlocalname)).into()
        }
        unsafe extern "system" fn setQName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, strqname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXAttributes_Impl::setQName(this, core::mem::transmute_copy(&nindex), core::mem::transmute(&strqname)).into()
        }
        unsafe extern "system" fn setType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, strtype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXAttributes_Impl::setType(this, core::mem::transmute_copy(&nindex), core::mem::transmute(&strtype)).into()
        }
        unsafe extern "system" fn setURI<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, struri: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXAttributes_Impl::setURI(this, core::mem::transmute_copy(&nindex), core::mem::transmute(&struri)).into()
        }
        unsafe extern "system" fn setValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, strvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXAttributes_Impl::setValue(this, core::mem::transmute_copy(&nindex), core::mem::transmute(&strvalue)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            addAttribute: addAttribute::<Identity, Impl, OFFSET>,
            addAttributeFromIndex: addAttributeFromIndex::<Identity, Impl, OFFSET>,
            clear: clear::<Identity, Impl, OFFSET>,
            removeAttribute: removeAttribute::<Identity, Impl, OFFSET>,
            setAttribute: setAttribute::<Identity, Impl, OFFSET>,
            setAttributes: setAttributes::<Identity, Impl, OFFSET>,
            setLocalName: setLocalName::<Identity, Impl, OFFSET>,
            setQName: setQName::<Identity, Impl, OFFSET>,
            setType: setType::<Identity, Impl, OFFSET>,
            setURI: setURI::<Identity, Impl, OFFSET>,
            setValue: setValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMXAttributes as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXNamespaceManager_Impl: Sized {
    fn putAllowOverride(&self, foverride: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn getAllowOverride(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn reset(&self) -> windows_core::Result<()>;
    fn pushContext(&self) -> windows_core::Result<()>;
    fn pushNodeContext(&self, contextnode: Option<&IXMLDOMNode>, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn popContext(&self) -> windows_core::Result<()>;
    fn declarePrefix(&self, prefix: &windows_core::PCWSTR, namespaceuri: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn getDeclaredPrefix(&self, nindex: i32, pwchprefix: windows_core::PWSTR, pcchprefix: *mut i32) -> windows_core::Result<()>;
    fn getPrefix(&self, pwsznamespaceuri: &windows_core::PCWSTR, nindex: i32, pwchprefix: windows_core::PWSTR, pcchprefix: *mut i32) -> windows_core::Result<()>;
    fn getURI(&self, pwchprefix: &windows_core::PCWSTR, pcontextnode: Option<&IXMLDOMNode>, pwchuri: windows_core::PWSTR, pcchuri: *mut i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMXNamespaceManager {}
#[cfg(feature = "Win32_System_Com")]
impl IMXNamespaceManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>() -> IMXNamespaceManager_Vtbl {
        unsafe extern "system" fn putAllowOverride<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, foverride: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXNamespaceManager_Impl::putAllowOverride(this, core::mem::transmute_copy(&foverride)).into()
        }
        unsafe extern "system" fn getAllowOverride<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, foverride: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXNamespaceManager_Impl::getAllowOverride(this) {
                Ok(ok__) => {
                    core::ptr::write(foverride, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXNamespaceManager_Impl::reset(this).into()
        }
        unsafe extern "system" fn pushContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXNamespaceManager_Impl::pushContext(this).into()
        }
        unsafe extern "system" fn pushNodeContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextnode: *mut core::ffi::c_void, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXNamespaceManager_Impl::pushNodeContext(this, windows_core::from_raw_borrowed(&contextnode), core::mem::transmute_copy(&fdeep)).into()
        }
        unsafe extern "system" fn popContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXNamespaceManager_Impl::popContext(this).into()
        }
        unsafe extern "system" fn declarePrefix<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefix: windows_core::PCWSTR, namespaceuri: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXNamespaceManager_Impl::declarePrefix(this, core::mem::transmute(&prefix), core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn getDeclaredPrefix<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, pwchprefix: windows_core::PWSTR, pcchprefix: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXNamespaceManager_Impl::getDeclaredPrefix(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&pwchprefix), core::mem::transmute_copy(&pcchprefix)).into()
        }
        unsafe extern "system" fn getPrefix<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsznamespaceuri: windows_core::PCWSTR, nindex: i32, pwchprefix: windows_core::PWSTR, pcchprefix: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXNamespaceManager_Impl::getPrefix(this, core::mem::transmute(&pwsznamespaceuri), core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&pwchprefix), core::mem::transmute_copy(&pcchprefix)).into()
        }
        unsafe extern "system" fn getURI<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchprefix: windows_core::PCWSTR, pcontextnode: *mut core::ffi::c_void, pwchuri: windows_core::PWSTR, pcchuri: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXNamespaceManager_Impl::getURI(this, core::mem::transmute(&pwchprefix), windows_core::from_raw_borrowed(&pcontextnode), core::mem::transmute_copy(&pwchuri), core::mem::transmute_copy(&pcchuri)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            putAllowOverride: putAllowOverride::<Identity, Impl, OFFSET>,
            getAllowOverride: getAllowOverride::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            pushContext: pushContext::<Identity, Impl, OFFSET>,
            pushNodeContext: pushNodeContext::<Identity, Impl, OFFSET>,
            popContext: popContext::<Identity, Impl, OFFSET>,
            declarePrefix: declarePrefix::<Identity, Impl, OFFSET>,
            getDeclaredPrefix: getDeclaredPrefix::<Identity, Impl, OFFSET>,
            getPrefix: getPrefix::<Identity, Impl, OFFSET>,
            getURI: getURI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMXNamespaceManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXNamespacePrefixes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn length(&self) -> windows_core::Result<i32>;
    fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMXNamespacePrefixes {}
#[cfg(feature = "Win32_System_Com")]
impl IMXNamespacePrefixes_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>() -> IMXNamespacePrefixes_Vtbl {
        unsafe extern "system" fn get_item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, prefix: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXNamespacePrefixes_Impl::get_item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(prefix, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXNamespacePrefixes_Impl::length(this) {
                Ok(ok__) => {
                    core::ptr::write(length, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXNamespacePrefixes_Impl::_newEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_item: get_item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMXNamespacePrefixes as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXReaderControl_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn abort(&self) -> windows_core::Result<()>;
    fn resume(&self) -> windows_core::Result<()>;
    fn suspend(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMXReaderControl {}
#[cfg(feature = "Win32_System_Com")]
impl IMXReaderControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXReaderControl_Impl, const OFFSET: isize>() -> IMXReaderControl_Vtbl {
        unsafe extern "system" fn abort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXReaderControl_Impl::abort(this).into()
        }
        unsafe extern "system" fn resume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXReaderControl_Impl::resume(this).into()
        }
        unsafe extern "system" fn suspend<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXReaderControl_Impl::suspend(this).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            abort: abort::<Identity, Impl, OFFSET>,
            resume: resume::<Identity, Impl, OFFSET>,
            suspend: suspend::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMXReaderControl as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXSchemaDeclHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn schemaElementDecl(&self, oschemaelement: Option<&ISchemaElement>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMXSchemaDeclHandler {}
#[cfg(feature = "Win32_System_Com")]
impl IMXSchemaDeclHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXSchemaDeclHandler_Impl, const OFFSET: isize>() -> IMXSchemaDeclHandler_Vtbl {
        unsafe extern "system" fn schemaElementDecl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXSchemaDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oschemaelement: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXSchemaDeclHandler_Impl::schemaElementDecl(this, windows_core::from_raw_borrowed(&oschemaelement)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            schemaElementDecl: schemaElementDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMXSchemaDeclHandler as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXWriter_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Setoutput(&self, vardestination: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn output(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Setencoding(&self, strencoding: &windows_core::BSTR) -> windows_core::Result<()>;
    fn encoding(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetbyteOrderMark(&self, fwritebyteordermark: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn byteOrderMark(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setindent(&self, findentmode: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn indent(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setstandalone(&self, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn standalone(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetomitXMLDeclaration(&self, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn omitXMLDeclaration(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setversion(&self, strversion: &windows_core::BSTR) -> windows_core::Result<()>;
    fn version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetdisableOutputEscaping(&self, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn disableOutputEscaping(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn flush(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMXWriter {}
#[cfg(feature = "Win32_System_Com")]
impl IMXWriter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>() -> IMXWriter_Vtbl {
        unsafe extern "system" fn Setoutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardestination: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXWriter_Impl::Setoutput(this, core::mem::transmute(&vardestination)).into()
        }
        unsafe extern "system" fn output<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardestination: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXWriter_Impl::output(this) {
                Ok(ok__) => {
                    core::ptr::write(vardestination, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setencoding<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencoding: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXWriter_Impl::Setencoding(this, core::mem::transmute(&strencoding)).into()
        }
        unsafe extern "system" fn encoding<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencoding: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXWriter_Impl::encoding(this) {
                Ok(ok__) => {
                    core::ptr::write(strencoding, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetbyteOrderMark<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fwritebyteordermark: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXWriter_Impl::SetbyteOrderMark(this, core::mem::transmute_copy(&fwritebyteordermark)).into()
        }
        unsafe extern "system" fn byteOrderMark<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fwritebyteordermark: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXWriter_Impl::byteOrderMark(this) {
                Ok(ok__) => {
                    core::ptr::write(fwritebyteordermark, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setindent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, findentmode: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXWriter_Impl::Setindent(this, core::mem::transmute_copy(&findentmode)).into()
        }
        unsafe extern "system" fn indent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, findentmode: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXWriter_Impl::indent(this) {
                Ok(ok__) => {
                    core::ptr::write(findentmode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setstandalone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXWriter_Impl::Setstandalone(this, core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn standalone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXWriter_Impl::standalone(this) {
                Ok(ok__) => {
                    core::ptr::write(fvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetomitXMLDeclaration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXWriter_Impl::SetomitXMLDeclaration(this, core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn omitXMLDeclaration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXWriter_Impl::omitXMLDeclaration(this) {
                Ok(ok__) => {
                    core::ptr::write(fvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setversion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strversion: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXWriter_Impl::Setversion(this, core::mem::transmute(&strversion)).into()
        }
        unsafe extern "system" fn version<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strversion: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXWriter_Impl::version(this) {
                Ok(ok__) => {
                    core::ptr::write(strversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetdisableOutputEscaping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXWriter_Impl::SetdisableOutputEscaping(this, core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn disableOutputEscaping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXWriter_Impl::disableOutputEscaping(this) {
                Ok(ok__) => {
                    core::ptr::write(fvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn flush<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXWriter_Impl::flush(this).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Setoutput: Setoutput::<Identity, Impl, OFFSET>,
            output: output::<Identity, Impl, OFFSET>,
            Setencoding: Setencoding::<Identity, Impl, OFFSET>,
            encoding: encoding::<Identity, Impl, OFFSET>,
            SetbyteOrderMark: SetbyteOrderMark::<Identity, Impl, OFFSET>,
            byteOrderMark: byteOrderMark::<Identity, Impl, OFFSET>,
            Setindent: Setindent::<Identity, Impl, OFFSET>,
            indent: indent::<Identity, Impl, OFFSET>,
            Setstandalone: Setstandalone::<Identity, Impl, OFFSET>,
            standalone: standalone::<Identity, Impl, OFFSET>,
            SetomitXMLDeclaration: SetomitXMLDeclaration::<Identity, Impl, OFFSET>,
            omitXMLDeclaration: omitXMLDeclaration::<Identity, Impl, OFFSET>,
            Setversion: Setversion::<Identity, Impl, OFFSET>,
            version: version::<Identity, Impl, OFFSET>,
            SetdisableOutputEscaping: SetdisableOutputEscaping::<Identity, Impl, OFFSET>,
            disableOutputEscaping: disableOutputEscaping::<Identity, Impl, OFFSET>,
            flush: flush::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMXWriter as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXXMLFilter_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn getFeature(&self, strname: &windows_core::BSTR) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn putFeature(&self, strname: &windows_core::BSTR, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn getProperty(&self, strname: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn putProperty(&self, strname: &windows_core::BSTR, varvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn entityResolver(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn putref_entityResolver(&self, oresolver: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn contentHandler(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn putref_contentHandler(&self, ohandler: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn dtdHandler(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn putref_dtdHandler(&self, ohandler: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn errorHandler(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn putref_errorHandler(&self, ohandler: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMXXMLFilter {}
#[cfg(feature = "Win32_System_Com")]
impl IMXXMLFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>() -> IMXXMLFilter_Vtbl {
        unsafe extern "system" fn getFeature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXXMLFilter_Impl::getFeature(this, core::mem::transmute(&strname)) {
                Ok(ok__) => {
                    core::ptr::write(fvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXXMLFilter_Impl::putFeature(this, core::mem::transmute(&strname), core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn getProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, varvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXXMLFilter_Impl::getProperty(this, core::mem::transmute(&strname)) {
                Ok(ok__) => {
                    core::ptr::write(varvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, varvalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXXMLFilter_Impl::putProperty(this, core::mem::transmute(&strname), core::mem::transmute(&varvalue)).into()
        }
        unsafe extern "system" fn entityResolver<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oresolver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXXMLFilter_Impl::entityResolver(this) {
                Ok(ok__) => {
                    core::ptr::write(oresolver, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_entityResolver<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oresolver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXXMLFilter_Impl::putref_entityResolver(this, windows_core::from_raw_borrowed(&oresolver)).into()
        }
        unsafe extern "system" fn contentHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXXMLFilter_Impl::contentHandler(this) {
                Ok(ok__) => {
                    core::ptr::write(ohandler, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_contentHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXXMLFilter_Impl::putref_contentHandler(this, windows_core::from_raw_borrowed(&ohandler)).into()
        }
        unsafe extern "system" fn dtdHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXXMLFilter_Impl::dtdHandler(this) {
                Ok(ok__) => {
                    core::ptr::write(ohandler, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_dtdHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXXMLFilter_Impl::putref_dtdHandler(this, windows_core::from_raw_borrowed(&ohandler)).into()
        }
        unsafe extern "system" fn errorHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMXXMLFilter_Impl::errorHandler(this) {
                Ok(ok__) => {
                    core::ptr::write(ohandler, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_errorHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMXXMLFilter_Impl::putref_errorHandler(this, windows_core::from_raw_borrowed(&ohandler)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            getFeature: getFeature::<Identity, Impl, OFFSET>,
            putFeature: putFeature::<Identity, Impl, OFFSET>,
            getProperty: getProperty::<Identity, Impl, OFFSET>,
            putProperty: putProperty::<Identity, Impl, OFFSET>,
            entityResolver: entityResolver::<Identity, Impl, OFFSET>,
            putref_entityResolver: putref_entityResolver::<Identity, Impl, OFFSET>,
            contentHandler: contentHandler::<Identity, Impl, OFFSET>,
            putref_contentHandler: putref_contentHandler::<Identity, Impl, OFFSET>,
            dtdHandler: dtdHandler::<Identity, Impl, OFFSET>,
            putref_dtdHandler: putref_dtdHandler::<Identity, Impl, OFFSET>,
            errorHandler: errorHandler::<Identity, Impl, OFFSET>,
            putref_errorHandler: putref_errorHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMXXMLFilter as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait ISAXAttributes_Impl: Sized {
    fn getLength(&self) -> windows_core::Result<i32>;
    fn getURI(&self, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> windows_core::Result<()>;
    fn getLocalName(&self, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> windows_core::Result<()>;
    fn getQName(&self, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> windows_core::Result<()>;
    fn getName(&self, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> windows_core::Result<()>;
    fn getIndexFromName(&self, pwchuri: &windows_core::PCWSTR, cchuri: i32, pwchlocalname: &windows_core::PCWSTR, cchlocalname: i32) -> windows_core::Result<i32>;
    fn getIndexFromQName(&self, pwchqname: &windows_core::PCWSTR, cchqname: i32) -> windows_core::Result<i32>;
    fn getType(&self, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> windows_core::Result<()>;
    fn getTypeFromName(&self, pwchuri: &windows_core::PCWSTR, cchuri: i32, pwchlocalname: &windows_core::PCWSTR, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> windows_core::Result<()>;
    fn getTypeFromQName(&self, pwchqname: &windows_core::PCWSTR, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> windows_core::Result<()>;
    fn getValue(&self, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> windows_core::Result<()>;
    fn getValueFromName(&self, pwchuri: &windows_core::PCWSTR, cchuri: i32, pwchlocalname: &windows_core::PCWSTR, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> windows_core::Result<()>;
    fn getValueFromQName(&self, pwchqname: &windows_core::PCWSTR, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISAXAttributes {}
impl ISAXAttributes_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>() -> ISAXAttributes_Vtbl {
        unsafe extern "system" fn getLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnlength: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXAttributes_Impl::getLength(this) {
                Ok(ok__) => {
                    core::ptr::write(pnlength, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXAttributes_Impl::getURI(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&ppwchuri), core::mem::transmute_copy(&pcchuri)).into()
        }
        unsafe extern "system" fn getLocalName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXAttributes_Impl::getLocalName(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&ppwchlocalname), core::mem::transmute_copy(&pcchlocalname)).into()
        }
        unsafe extern "system" fn getQName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXAttributes_Impl::getQName(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&ppwchqname), core::mem::transmute_copy(&pcchqname)).into()
        }
        unsafe extern "system" fn getName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXAttributes_Impl::getName(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&ppwchuri), core::mem::transmute_copy(&pcchuri), core::mem::transmute_copy(&ppwchlocalname), core::mem::transmute_copy(&pcchlocalname), core::mem::transmute_copy(&ppwchqname), core::mem::transmute_copy(&pcchqname)).into()
        }
        unsafe extern "system" fn getIndexFromName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchuri: windows_core::PCWSTR, cchuri: i32, pwchlocalname: windows_core::PCWSTR, cchlocalname: i32, pnindex: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXAttributes_Impl::getIndexFromName(this, core::mem::transmute(&pwchuri), core::mem::transmute_copy(&cchuri), core::mem::transmute(&pwchlocalname), core::mem::transmute_copy(&cchlocalname)) {
                Ok(ok__) => {
                    core::ptr::write(pnindex, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromQName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchqname: windows_core::PCWSTR, cchqname: i32, pnindex: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXAttributes_Impl::getIndexFromQName(this, core::mem::transmute(&pwchqname), core::mem::transmute_copy(&cchqname)) {
                Ok(ok__) => {
                    core::ptr::write(pnindex, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXAttributes_Impl::getType(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&ppwchtype), core::mem::transmute_copy(&pcchtype)).into()
        }
        unsafe extern "system" fn getTypeFromName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchuri: windows_core::PCWSTR, cchuri: i32, pwchlocalname: windows_core::PCWSTR, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXAttributes_Impl::getTypeFromName(this, core::mem::transmute(&pwchuri), core::mem::transmute_copy(&cchuri), core::mem::transmute(&pwchlocalname), core::mem::transmute_copy(&cchlocalname), core::mem::transmute_copy(&ppwchtype), core::mem::transmute_copy(&pcchtype)).into()
        }
        unsafe extern "system" fn getTypeFromQName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchqname: windows_core::PCWSTR, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXAttributes_Impl::getTypeFromQName(this, core::mem::transmute(&pwchqname), core::mem::transmute_copy(&cchqname), core::mem::transmute_copy(&ppwchtype), core::mem::transmute_copy(&pcchtype)).into()
        }
        unsafe extern "system" fn getValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXAttributes_Impl::getValue(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&ppwchvalue), core::mem::transmute_copy(&pcchvalue)).into()
        }
        unsafe extern "system" fn getValueFromName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchuri: windows_core::PCWSTR, cchuri: i32, pwchlocalname: windows_core::PCWSTR, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXAttributes_Impl::getValueFromName(this, core::mem::transmute(&pwchuri), core::mem::transmute_copy(&cchuri), core::mem::transmute(&pwchlocalname), core::mem::transmute_copy(&cchlocalname), core::mem::transmute_copy(&ppwchvalue), core::mem::transmute_copy(&pcchvalue)).into()
        }
        unsafe extern "system" fn getValueFromQName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchqname: windows_core::PCWSTR, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXAttributes_Impl::getValueFromQName(this, core::mem::transmute(&pwchqname), core::mem::transmute_copy(&cchqname), core::mem::transmute_copy(&ppwchvalue), core::mem::transmute_copy(&pcchvalue)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            getLength: getLength::<Identity, Impl, OFFSET>,
            getURI: getURI::<Identity, Impl, OFFSET>,
            getLocalName: getLocalName::<Identity, Impl, OFFSET>,
            getQName: getQName::<Identity, Impl, OFFSET>,
            getName: getName::<Identity, Impl, OFFSET>,
            getIndexFromName: getIndexFromName::<Identity, Impl, OFFSET>,
            getIndexFromQName: getIndexFromQName::<Identity, Impl, OFFSET>,
            getType: getType::<Identity, Impl, OFFSET>,
            getTypeFromName: getTypeFromName::<Identity, Impl, OFFSET>,
            getTypeFromQName: getTypeFromQName::<Identity, Impl, OFFSET>,
            getValue: getValue::<Identity, Impl, OFFSET>,
            getValueFromName: getValueFromName::<Identity, Impl, OFFSET>,
            getValueFromQName: getValueFromQName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXAttributes as windows_core::Interface>::IID
    }
}
pub trait ISAXContentHandler_Impl: Sized {
    fn putDocumentLocator(&self, plocator: Option<&ISAXLocator>) -> windows_core::Result<()>;
    fn startDocument(&self) -> windows_core::Result<()>;
    fn endDocument(&self) -> windows_core::Result<()>;
    fn startPrefixMapping(&self, pwchprefix: &windows_core::PCWSTR, cchprefix: i32, pwchuri: &windows_core::PCWSTR, cchuri: i32) -> windows_core::Result<()>;
    fn endPrefixMapping(&self, pwchprefix: &windows_core::PCWSTR, cchprefix: i32) -> windows_core::Result<()>;
    fn startElement(&self, pwchnamespaceuri: &windows_core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: &windows_core::PCWSTR, cchlocalname: i32, pwchqname: &windows_core::PCWSTR, cchqname: i32, pattributes: Option<&ISAXAttributes>) -> windows_core::Result<()>;
    fn endElement(&self, pwchnamespaceuri: &windows_core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: &windows_core::PCWSTR, cchlocalname: i32, pwchqname: &windows_core::PCWSTR, cchqname: i32) -> windows_core::Result<()>;
    fn characters(&self, pwchchars: &windows_core::PCWSTR, cchchars: i32) -> windows_core::Result<()>;
    fn ignorableWhitespace(&self, pwchchars: &windows_core::PCWSTR, cchchars: i32) -> windows_core::Result<()>;
    fn processingInstruction(&self, pwchtarget: &windows_core::PCWSTR, cchtarget: i32, pwchdata: &windows_core::PCWSTR, cchdata: i32) -> windows_core::Result<()>;
    fn skippedEntity(&self, pwchname: &windows_core::PCWSTR, cchname: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISAXContentHandler {}
impl ISAXContentHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>() -> ISAXContentHandler_Vtbl {
        unsafe extern "system" fn putDocumentLocator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocator: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXContentHandler_Impl::putDocumentLocator(this, windows_core::from_raw_borrowed(&plocator)).into()
        }
        unsafe extern "system" fn startDocument<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXContentHandler_Impl::startDocument(this).into()
        }
        unsafe extern "system" fn endDocument<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXContentHandler_Impl::endDocument(this).into()
        }
        unsafe extern "system" fn startPrefixMapping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchprefix: windows_core::PCWSTR, cchprefix: i32, pwchuri: windows_core::PCWSTR, cchuri: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXContentHandler_Impl::startPrefixMapping(this, core::mem::transmute(&pwchprefix), core::mem::transmute_copy(&cchprefix), core::mem::transmute(&pwchuri), core::mem::transmute_copy(&cchuri)).into()
        }
        unsafe extern "system" fn endPrefixMapping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchprefix: windows_core::PCWSTR, cchprefix: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXContentHandler_Impl::endPrefixMapping(this, core::mem::transmute(&pwchprefix), core::mem::transmute_copy(&cchprefix)).into()
        }
        unsafe extern "system" fn startElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchnamespaceuri: windows_core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: windows_core::PCWSTR, cchlocalname: i32, pwchqname: windows_core::PCWSTR, cchqname: i32, pattributes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXContentHandler_Impl::startElement(this, core::mem::transmute(&pwchnamespaceuri), core::mem::transmute_copy(&cchnamespaceuri), core::mem::transmute(&pwchlocalname), core::mem::transmute_copy(&cchlocalname), core::mem::transmute(&pwchqname), core::mem::transmute_copy(&cchqname), windows_core::from_raw_borrowed(&pattributes)).into()
        }
        unsafe extern "system" fn endElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchnamespaceuri: windows_core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: windows_core::PCWSTR, cchlocalname: i32, pwchqname: windows_core::PCWSTR, cchqname: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXContentHandler_Impl::endElement(this, core::mem::transmute(&pwchnamespaceuri), core::mem::transmute_copy(&cchnamespaceuri), core::mem::transmute(&pwchlocalname), core::mem::transmute_copy(&cchlocalname), core::mem::transmute(&pwchqname), core::mem::transmute_copy(&cchqname)).into()
        }
        unsafe extern "system" fn characters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchchars: windows_core::PCWSTR, cchchars: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXContentHandler_Impl::characters(this, core::mem::transmute(&pwchchars), core::mem::transmute_copy(&cchchars)).into()
        }
        unsafe extern "system" fn ignorableWhitespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchchars: windows_core::PCWSTR, cchchars: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXContentHandler_Impl::ignorableWhitespace(this, core::mem::transmute(&pwchchars), core::mem::transmute_copy(&cchchars)).into()
        }
        unsafe extern "system" fn processingInstruction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchtarget: windows_core::PCWSTR, cchtarget: i32, pwchdata: windows_core::PCWSTR, cchdata: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXContentHandler_Impl::processingInstruction(this, core::mem::transmute(&pwchtarget), core::mem::transmute_copy(&cchtarget), core::mem::transmute(&pwchdata), core::mem::transmute_copy(&cchdata)).into()
        }
        unsafe extern "system" fn skippedEntity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: windows_core::PCWSTR, cchname: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXContentHandler_Impl::skippedEntity(this, core::mem::transmute(&pwchname), core::mem::transmute_copy(&cchname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            putDocumentLocator: putDocumentLocator::<Identity, Impl, OFFSET>,
            startDocument: startDocument::<Identity, Impl, OFFSET>,
            endDocument: endDocument::<Identity, Impl, OFFSET>,
            startPrefixMapping: startPrefixMapping::<Identity, Impl, OFFSET>,
            endPrefixMapping: endPrefixMapping::<Identity, Impl, OFFSET>,
            startElement: startElement::<Identity, Impl, OFFSET>,
            endElement: endElement::<Identity, Impl, OFFSET>,
            characters: characters::<Identity, Impl, OFFSET>,
            ignorableWhitespace: ignorableWhitespace::<Identity, Impl, OFFSET>,
            processingInstruction: processingInstruction::<Identity, Impl, OFFSET>,
            skippedEntity: skippedEntity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXContentHandler as windows_core::Interface>::IID
    }
}
pub trait ISAXDTDHandler_Impl: Sized {
    fn notationDecl(&self, pwchname: &windows_core::PCWSTR, cchname: i32, pwchpublicid: &windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: &windows_core::PCWSTR, cchsystemid: i32) -> windows_core::Result<()>;
    fn unparsedEntityDecl(&self, pwchname: &windows_core::PCWSTR, cchname: i32, pwchpublicid: &windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: &windows_core::PCWSTR, cchsystemid: i32, pwchnotationname: &windows_core::PCWSTR, cchnotationname: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISAXDTDHandler {}
impl ISAXDTDHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXDTDHandler_Impl, const OFFSET: isize>() -> ISAXDTDHandler_Vtbl {
        unsafe extern "system" fn notationDecl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXDTDHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: windows_core::PCWSTR, cchname: i32, pwchpublicid: windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: windows_core::PCWSTR, cchsystemid: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXDTDHandler_Impl::notationDecl(this, core::mem::transmute(&pwchname), core::mem::transmute_copy(&cchname), core::mem::transmute(&pwchpublicid), core::mem::transmute_copy(&cchpublicid), core::mem::transmute(&pwchsystemid), core::mem::transmute_copy(&cchsystemid)).into()
        }
        unsafe extern "system" fn unparsedEntityDecl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXDTDHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: windows_core::PCWSTR, cchname: i32, pwchpublicid: windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: windows_core::PCWSTR, cchsystemid: i32, pwchnotationname: windows_core::PCWSTR, cchnotationname: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXDTDHandler_Impl::unparsedEntityDecl(this, core::mem::transmute(&pwchname), core::mem::transmute_copy(&cchname), core::mem::transmute(&pwchpublicid), core::mem::transmute_copy(&cchpublicid), core::mem::transmute(&pwchsystemid), core::mem::transmute_copy(&cchsystemid), core::mem::transmute(&pwchnotationname), core::mem::transmute_copy(&cchnotationname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            notationDecl: notationDecl::<Identity, Impl, OFFSET>,
            unparsedEntityDecl: unparsedEntityDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXDTDHandler as windows_core::Interface>::IID
    }
}
pub trait ISAXDeclHandler_Impl: Sized {
    fn elementDecl(&self, pwchname: &windows_core::PCWSTR, cchname: i32, pwchmodel: &windows_core::PCWSTR, cchmodel: i32) -> windows_core::Result<()>;
    fn attributeDecl(&self, pwchelementname: &windows_core::PCWSTR, cchelementname: i32, pwchattributename: &windows_core::PCWSTR, cchattributename: i32, pwchtype: &windows_core::PCWSTR, cchtype: i32, pwchvaluedefault: &windows_core::PCWSTR, cchvaluedefault: i32, pwchvalue: &windows_core::PCWSTR, cchvalue: i32) -> windows_core::Result<()>;
    fn internalEntityDecl(&self, pwchname: &windows_core::PCWSTR, cchname: i32, pwchvalue: &windows_core::PCWSTR, cchvalue: i32) -> windows_core::Result<()>;
    fn externalEntityDecl(&self, pwchname: &windows_core::PCWSTR, cchname: i32, pwchpublicid: &windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: &windows_core::PCWSTR, cchsystemid: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISAXDeclHandler {}
impl ISAXDeclHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>() -> ISAXDeclHandler_Vtbl {
        unsafe extern "system" fn elementDecl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: windows_core::PCWSTR, cchname: i32, pwchmodel: windows_core::PCWSTR, cchmodel: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXDeclHandler_Impl::elementDecl(this, core::mem::transmute(&pwchname), core::mem::transmute_copy(&cchname), core::mem::transmute(&pwchmodel), core::mem::transmute_copy(&cchmodel)).into()
        }
        unsafe extern "system" fn attributeDecl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchelementname: windows_core::PCWSTR, cchelementname: i32, pwchattributename: windows_core::PCWSTR, cchattributename: i32, pwchtype: windows_core::PCWSTR, cchtype: i32, pwchvaluedefault: windows_core::PCWSTR, cchvaluedefault: i32, pwchvalue: windows_core::PCWSTR, cchvalue: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXDeclHandler_Impl::attributeDecl(this, core::mem::transmute(&pwchelementname), core::mem::transmute_copy(&cchelementname), core::mem::transmute(&pwchattributename), core::mem::transmute_copy(&cchattributename), core::mem::transmute(&pwchtype), core::mem::transmute_copy(&cchtype), core::mem::transmute(&pwchvaluedefault), core::mem::transmute_copy(&cchvaluedefault), core::mem::transmute(&pwchvalue), core::mem::transmute_copy(&cchvalue)).into()
        }
        unsafe extern "system" fn internalEntityDecl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: windows_core::PCWSTR, cchname: i32, pwchvalue: windows_core::PCWSTR, cchvalue: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXDeclHandler_Impl::internalEntityDecl(this, core::mem::transmute(&pwchname), core::mem::transmute_copy(&cchname), core::mem::transmute(&pwchvalue), core::mem::transmute_copy(&cchvalue)).into()
        }
        unsafe extern "system" fn externalEntityDecl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: windows_core::PCWSTR, cchname: i32, pwchpublicid: windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: windows_core::PCWSTR, cchsystemid: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXDeclHandler_Impl::externalEntityDecl(this, core::mem::transmute(&pwchname), core::mem::transmute_copy(&cchname), core::mem::transmute(&pwchpublicid), core::mem::transmute_copy(&cchpublicid), core::mem::transmute(&pwchsystemid), core::mem::transmute_copy(&cchsystemid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            elementDecl: elementDecl::<Identity, Impl, OFFSET>,
            attributeDecl: attributeDecl::<Identity, Impl, OFFSET>,
            internalEntityDecl: internalEntityDecl::<Identity, Impl, OFFSET>,
            externalEntityDecl: externalEntityDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXDeclHandler as windows_core::Interface>::IID
    }
}
pub trait ISAXEntityResolver_Impl: Sized {
    fn resolveEntity(&self, pwchpublicid: &windows_core::PCWSTR, pwchsystemid: &windows_core::PCWSTR) -> windows_core::Result<windows_core::VARIANT>;
}
impl windows_core::RuntimeName for ISAXEntityResolver {}
impl ISAXEntityResolver_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXEntityResolver_Impl, const OFFSET: isize>() -> ISAXEntityResolver_Vtbl {
        unsafe extern "system" fn resolveEntity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXEntityResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchpublicid: windows_core::PCWSTR, pwchsystemid: windows_core::PCWSTR, pvarinput: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXEntityResolver_Impl::resolveEntity(this, core::mem::transmute(&pwchpublicid), core::mem::transmute(&pwchsystemid)) {
                Ok(ok__) => {
                    core::ptr::write(pvarinput, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), resolveEntity: resolveEntity::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXEntityResolver as windows_core::Interface>::IID
    }
}
pub trait ISAXErrorHandler_Impl: Sized {
    fn error(&self, plocator: Option<&ISAXLocator>, pwcherrormessage: &windows_core::PCWSTR, hrerrorcode: windows_core::HRESULT) -> windows_core::Result<()>;
    fn fatalError(&self, plocator: Option<&ISAXLocator>, pwcherrormessage: &windows_core::PCWSTR, hrerrorcode: windows_core::HRESULT) -> windows_core::Result<()>;
    fn ignorableWarning(&self, plocator: Option<&ISAXLocator>, pwcherrormessage: &windows_core::PCWSTR, hrerrorcode: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISAXErrorHandler {}
impl ISAXErrorHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXErrorHandler_Impl, const OFFSET: isize>() -> ISAXErrorHandler_Vtbl {
        unsafe extern "system" fn error<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocator: *mut core::ffi::c_void, pwcherrormessage: windows_core::PCWSTR, hrerrorcode: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXErrorHandler_Impl::error(this, windows_core::from_raw_borrowed(&plocator), core::mem::transmute(&pwcherrormessage), core::mem::transmute_copy(&hrerrorcode)).into()
        }
        unsafe extern "system" fn fatalError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocator: *mut core::ffi::c_void, pwcherrormessage: windows_core::PCWSTR, hrerrorcode: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXErrorHandler_Impl::fatalError(this, windows_core::from_raw_borrowed(&plocator), core::mem::transmute(&pwcherrormessage), core::mem::transmute_copy(&hrerrorcode)).into()
        }
        unsafe extern "system" fn ignorableWarning<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocator: *mut core::ffi::c_void, pwcherrormessage: windows_core::PCWSTR, hrerrorcode: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXErrorHandler_Impl::ignorableWarning(this, windows_core::from_raw_borrowed(&plocator), core::mem::transmute(&pwcherrormessage), core::mem::transmute_copy(&hrerrorcode)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            error: error::<Identity, Impl, OFFSET>,
            fatalError: fatalError::<Identity, Impl, OFFSET>,
            ignorableWarning: ignorableWarning::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXErrorHandler as windows_core::Interface>::IID
    }
}
pub trait ISAXLexicalHandler_Impl: Sized {
    fn startDTD(&self, pwchname: &windows_core::PCWSTR, cchname: i32, pwchpublicid: &windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: &windows_core::PCWSTR, cchsystemid: i32) -> windows_core::Result<()>;
    fn endDTD(&self) -> windows_core::Result<()>;
    fn startEntity(&self, pwchname: &windows_core::PCWSTR, cchname: i32) -> windows_core::Result<()>;
    fn endEntity(&self, pwchname: &windows_core::PCWSTR, cchname: i32) -> windows_core::Result<()>;
    fn startCDATA(&self) -> windows_core::Result<()>;
    fn endCDATA(&self) -> windows_core::Result<()>;
    fn comment(&self, pwchchars: &windows_core::PCWSTR, cchchars: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISAXLexicalHandler {}
impl ISAXLexicalHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>() -> ISAXLexicalHandler_Vtbl {
        unsafe extern "system" fn startDTD<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: windows_core::PCWSTR, cchname: i32, pwchpublicid: windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: windows_core::PCWSTR, cchsystemid: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXLexicalHandler_Impl::startDTD(this, core::mem::transmute(&pwchname), core::mem::transmute_copy(&cchname), core::mem::transmute(&pwchpublicid), core::mem::transmute_copy(&cchpublicid), core::mem::transmute(&pwchsystemid), core::mem::transmute_copy(&cchsystemid)).into()
        }
        unsafe extern "system" fn endDTD<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXLexicalHandler_Impl::endDTD(this).into()
        }
        unsafe extern "system" fn startEntity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: windows_core::PCWSTR, cchname: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXLexicalHandler_Impl::startEntity(this, core::mem::transmute(&pwchname), core::mem::transmute_copy(&cchname)).into()
        }
        unsafe extern "system" fn endEntity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: windows_core::PCWSTR, cchname: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXLexicalHandler_Impl::endEntity(this, core::mem::transmute(&pwchname), core::mem::transmute_copy(&cchname)).into()
        }
        unsafe extern "system" fn startCDATA<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXLexicalHandler_Impl::startCDATA(this).into()
        }
        unsafe extern "system" fn endCDATA<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXLexicalHandler_Impl::endCDATA(this).into()
        }
        unsafe extern "system" fn comment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchchars: windows_core::PCWSTR, cchchars: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXLexicalHandler_Impl::comment(this, core::mem::transmute(&pwchchars), core::mem::transmute_copy(&cchchars)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            startDTD: startDTD::<Identity, Impl, OFFSET>,
            endDTD: endDTD::<Identity, Impl, OFFSET>,
            startEntity: startEntity::<Identity, Impl, OFFSET>,
            endEntity: endEntity::<Identity, Impl, OFFSET>,
            startCDATA: startCDATA::<Identity, Impl, OFFSET>,
            endCDATA: endCDATA::<Identity, Impl, OFFSET>,
            comment: comment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXLexicalHandler as windows_core::Interface>::IID
    }
}
pub trait ISAXLocator_Impl: Sized {
    fn getColumnNumber(&self) -> windows_core::Result<i32>;
    fn getLineNumber(&self) -> windows_core::Result<i32>;
    fn getPublicId(&self) -> windows_core::Result<*mut u16>;
    fn getSystemId(&self) -> windows_core::Result<*mut u16>;
}
impl windows_core::RuntimeName for ISAXLocator {}
impl ISAXLocator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: isize>() -> ISAXLocator_Vtbl {
        unsafe extern "system" fn getColumnNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pncolumn: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXLocator_Impl::getColumnNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(pncolumn, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getLineNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnline: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXLocator_Impl::getLineNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(pnline, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPublicId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwchpublicid: *mut *mut u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXLocator_Impl::getPublicId(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwchpublicid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSystemId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwchsystemid: *mut *mut u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXLocator_Impl::getSystemId(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwchsystemid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            getColumnNumber: getColumnNumber::<Identity, Impl, OFFSET>,
            getLineNumber: getLineNumber::<Identity, Impl, OFFSET>,
            getPublicId: getPublicId::<Identity, Impl, OFFSET>,
            getSystemId: getSystemId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXLocator as windows_core::Interface>::IID
    }
}
pub trait ISAXXMLFilter_Impl: Sized + ISAXXMLReader_Impl {
    fn getParent(&self) -> windows_core::Result<ISAXXMLReader>;
    fn putParent(&self, preader: Option<&ISAXXMLReader>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISAXXMLFilter {}
impl ISAXXMLFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLFilter_Impl, const OFFSET: isize>() -> ISAXXMLFilter_Vtbl {
        unsafe extern "system" fn getParent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXXMLFilter_Impl::getParent(this) {
                Ok(ok__) => {
                    core::ptr::write(ppreader, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putParent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXXMLFilter_Impl::putParent(this, windows_core::from_raw_borrowed(&preader)).into()
        }
        Self {
            base__: ISAXXMLReader_Vtbl::new::<Identity, Impl, OFFSET>(),
            getParent: getParent::<Identity, Impl, OFFSET>,
            putParent: putParent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXXMLFilter as windows_core::Interface>::IID || iid == &<ISAXXMLReader as windows_core::Interface>::IID
    }
}
pub trait ISAXXMLReader_Impl: Sized {
    fn getFeature(&self, pwchname: &windows_core::PCWSTR) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn putFeature(&self, pwchname: &windows_core::PCWSTR, vfvalue: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn getProperty(&self, pwchname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn putProperty(&self, pwchname: &windows_core::PCWSTR, varvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn getEntityResolver(&self) -> windows_core::Result<ISAXEntityResolver>;
    fn putEntityResolver(&self, presolver: Option<&ISAXEntityResolver>) -> windows_core::Result<()>;
    fn getContentHandler(&self) -> windows_core::Result<ISAXContentHandler>;
    fn putContentHandler(&self, phandler: Option<&ISAXContentHandler>) -> windows_core::Result<()>;
    fn getDTDHandler(&self) -> windows_core::Result<ISAXDTDHandler>;
    fn putDTDHandler(&self, phandler: Option<&ISAXDTDHandler>) -> windows_core::Result<()>;
    fn getErrorHandler(&self) -> windows_core::Result<ISAXErrorHandler>;
    fn putErrorHandler(&self, phandler: Option<&ISAXErrorHandler>) -> windows_core::Result<()>;
    fn getBaseURL(&self) -> windows_core::Result<*mut u16>;
    fn putBaseURL(&self, pwchbaseurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn getSecureBaseURL(&self) -> windows_core::Result<*mut u16>;
    fn putSecureBaseURL(&self, pwchsecurebaseurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn parse(&self, varinput: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn parseURL(&self, pwchurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISAXXMLReader {}
impl ISAXXMLReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>() -> ISAXXMLReader_Vtbl {
        unsafe extern "system" fn getFeature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: windows_core::PCWSTR, pvfvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXXMLReader_Impl::getFeature(this, core::mem::transmute(&pwchname)) {
                Ok(ok__) => {
                    core::ptr::write(pvfvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: windows_core::PCWSTR, vfvalue: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXXMLReader_Impl::putFeature(this, core::mem::transmute(&pwchname), core::mem::transmute_copy(&vfvalue)).into()
        }
        unsafe extern "system" fn getProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: windows_core::PCWSTR, pvarvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXXMLReader_Impl::getProperty(this, core::mem::transmute(&pwchname)) {
                Ok(ok__) => {
                    core::ptr::write(pvarvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: windows_core::PCWSTR, varvalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXXMLReader_Impl::putProperty(this, core::mem::transmute(&pwchname), core::mem::transmute(&varvalue)).into()
        }
        unsafe extern "system" fn getEntityResolver<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresolver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXXMLReader_Impl::getEntityResolver(this) {
                Ok(ok__) => {
                    core::ptr::write(ppresolver, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putEntityResolver<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presolver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXXMLReader_Impl::putEntityResolver(this, windows_core::from_raw_borrowed(&presolver)).into()
        }
        unsafe extern "system" fn getContentHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXXMLReader_Impl::getContentHandler(this) {
                Ok(ok__) => {
                    core::ptr::write(pphandler, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putContentHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXXMLReader_Impl::putContentHandler(this, windows_core::from_raw_borrowed(&phandler)).into()
        }
        unsafe extern "system" fn getDTDHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXXMLReader_Impl::getDTDHandler(this) {
                Ok(ok__) => {
                    core::ptr::write(pphandler, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putDTDHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXXMLReader_Impl::putDTDHandler(this, windows_core::from_raw_borrowed(&phandler)).into()
        }
        unsafe extern "system" fn getErrorHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXXMLReader_Impl::getErrorHandler(this) {
                Ok(ok__) => {
                    core::ptr::write(pphandler, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putErrorHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXXMLReader_Impl::putErrorHandler(this, windows_core::from_raw_borrowed(&phandler)).into()
        }
        unsafe extern "system" fn getBaseURL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwchbaseurl: *mut *mut u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXXMLReader_Impl::getBaseURL(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwchbaseurl, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putBaseURL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchbaseurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXXMLReader_Impl::putBaseURL(this, core::mem::transmute(&pwchbaseurl)).into()
        }
        unsafe extern "system" fn getSecureBaseURL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwchsecurebaseurl: *mut *mut u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISAXXMLReader_Impl::getSecureBaseURL(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwchsecurebaseurl, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putSecureBaseURL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchsecurebaseurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXXMLReader_Impl::putSecureBaseURL(this, core::mem::transmute(&pwchsecurebaseurl)).into()
        }
        unsafe extern "system" fn parse<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varinput: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXXMLReader_Impl::parse(this, core::mem::transmute(&varinput)).into()
        }
        unsafe extern "system" fn parseURL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISAXXMLReader_Impl::parseURL(this, core::mem::transmute(&pwchurl)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            getFeature: getFeature::<Identity, Impl, OFFSET>,
            putFeature: putFeature::<Identity, Impl, OFFSET>,
            getProperty: getProperty::<Identity, Impl, OFFSET>,
            putProperty: putProperty::<Identity, Impl, OFFSET>,
            getEntityResolver: getEntityResolver::<Identity, Impl, OFFSET>,
            putEntityResolver: putEntityResolver::<Identity, Impl, OFFSET>,
            getContentHandler: getContentHandler::<Identity, Impl, OFFSET>,
            putContentHandler: putContentHandler::<Identity, Impl, OFFSET>,
            getDTDHandler: getDTDHandler::<Identity, Impl, OFFSET>,
            putDTDHandler: putDTDHandler::<Identity, Impl, OFFSET>,
            getErrorHandler: getErrorHandler::<Identity, Impl, OFFSET>,
            putErrorHandler: putErrorHandler::<Identity, Impl, OFFSET>,
            getBaseURL: getBaseURL::<Identity, Impl, OFFSET>,
            putBaseURL: putBaseURL::<Identity, Impl, OFFSET>,
            getSecureBaseURL: getSecureBaseURL::<Identity, Impl, OFFSET>,
            putSecureBaseURL: putSecureBaseURL::<Identity, Impl, OFFSET>,
            parse: parse::<Identity, Impl, OFFSET>,
            parseURL: parseURL::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXXMLReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchema_Impl: Sized + ISchemaItem_Impl {
    fn targetNamespace(&self) -> windows_core::Result<windows_core::BSTR>;
    fn version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn types(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn elements(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn attributes(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn attributeGroups(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn modelGroups(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn notations(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn schemaLocations(&self) -> windows_core::Result<ISchemaStringCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISchema {}
#[cfg(feature = "Win32_System_Com")]
impl ISchema_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>() -> ISchema_Vtbl {
        unsafe extern "system" fn targetNamespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetnamespace: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchema_Impl::targetNamespace(this) {
                Ok(ok__) => {
                    core::ptr::write(targetnamespace, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn version<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchema_Impl::version(this) {
                Ok(ok__) => {
                    core::ptr::write(version, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn types<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, types: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchema_Impl::types(this) {
                Ok(ok__) => {
                    core::ptr::write(types, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn elements<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, elements: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchema_Impl::elements(this) {
                Ok(ok__) => {
                    core::ptr::write(elements, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchema_Impl::attributes(this) {
                Ok(ok__) => {
                    core::ptr::write(attributes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributeGroups<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributegroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchema_Impl::attributeGroups(this) {
                Ok(ok__) => {
                    core::ptr::write(attributegroups, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn modelGroups<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, modelgroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchema_Impl::modelGroups(this) {
                Ok(ok__) => {
                    core::ptr::write(modelgroups, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notations<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notations: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchema_Impl::notations(this) {
                Ok(ok__) => {
                    core::ptr::write(notations, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schemaLocations<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, schemalocations: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchema_Impl::schemaLocations(this) {
                Ok(ok__) => {
                    core::ptr::write(schemalocations, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            targetNamespace: targetNamespace::<Identity, Impl, OFFSET>,
            version: version::<Identity, Impl, OFFSET>,
            types: types::<Identity, Impl, OFFSET>,
            elements: elements::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
            attributeGroups: attributeGroups::<Identity, Impl, OFFSET>,
            modelGroups: modelGroups::<Identity, Impl, OFFSET>,
            notations: notations::<Identity, Impl, OFFSET>,
            schemaLocations: schemaLocations::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchema as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaAny_Impl: Sized + ISchemaParticle_Impl {
    fn namespaces(&self) -> windows_core::Result<ISchemaStringCollection>;
    fn processContents(&self) -> windows_core::Result<SCHEMAPROCESSCONTENTS>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISchemaAny {}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaAny_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAny_Impl, const OFFSET: isize>() -> ISchemaAny_Vtbl {
        unsafe extern "system" fn namespaces<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAny_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaces: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaAny_Impl::namespaces(this) {
                Ok(ok__) => {
                    core::ptr::write(namespaces, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn processContents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAny_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processcontents: *mut SCHEMAPROCESSCONTENTS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaAny_Impl::processContents(this) {
                Ok(ok__) => {
                    core::ptr::write(processcontents, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaParticle_Vtbl::new::<Identity, Impl, OFFSET>(),
            namespaces: namespaces::<Identity, Impl, OFFSET>,
            processContents: processContents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaAny as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID || iid == &<ISchemaParticle as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaAttribute_Impl: Sized + ISchemaItem_Impl {
    fn r#type(&self) -> windows_core::Result<ISchemaType>;
    fn scope(&self) -> windows_core::Result<ISchemaComplexType>;
    fn defaultValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn fixedValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn r#use(&self) -> windows_core::Result<SCHEMAUSE>;
    fn isReference(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISchemaAttribute {}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaAttribute_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>() -> ISchemaAttribute_Vtbl {
        unsafe extern "system" fn r#type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaAttribute_Impl::r#type(this) {
                Ok(ok__) => {
                    core::ptr::write(r#type, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn scope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaAttribute_Impl::scope(this) {
                Ok(ok__) => {
                    core::ptr::write(scope, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn defaultValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, defaultvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaAttribute_Impl::defaultValue(this) {
                Ok(ok__) => {
                    core::ptr::write(defaultvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fixedValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fixedvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaAttribute_Impl::fixedValue(this) {
                Ok(ok__) => {
                    core::ptr::write(fixedvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#use<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#use: *mut SCHEMAUSE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaAttribute_Impl::r#use(this) {
                Ok(ok__) => {
                    core::ptr::write(r#use, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isReference<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reference: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaAttribute_Impl::isReference(this) {
                Ok(ok__) => {
                    core::ptr::write(reference, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            r#type: r#type::<Identity, Impl, OFFSET>,
            scope: scope::<Identity, Impl, OFFSET>,
            defaultValue: defaultValue::<Identity, Impl, OFFSET>,
            fixedValue: fixedValue::<Identity, Impl, OFFSET>,
            r#use: r#use::<Identity, Impl, OFFSET>,
            isReference: isReference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaAttribute as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaAttributeGroup_Impl: Sized + ISchemaItem_Impl {
    fn anyAttribute(&self) -> windows_core::Result<ISchemaAny>;
    fn attributes(&self) -> windows_core::Result<ISchemaItemCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISchemaAttributeGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaAttributeGroup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttributeGroup_Impl, const OFFSET: isize>() -> ISchemaAttributeGroup_Vtbl {
        unsafe extern "system" fn anyAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttributeGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, anyattribute: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaAttributeGroup_Impl::anyAttribute(this) {
                Ok(ok__) => {
                    core::ptr::write(anyattribute, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttributeGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaAttributeGroup_Impl::attributes(this) {
                Ok(ok__) => {
                    core::ptr::write(attributes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            anyAttribute: anyAttribute::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaAttributeGroup as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaComplexType_Impl: Sized + ISchemaType_Impl {
    fn isAbstract(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn anyAttribute(&self) -> windows_core::Result<ISchemaAny>;
    fn attributes(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn contentType(&self) -> windows_core::Result<SCHEMACONTENTTYPE>;
    fn contentModel(&self) -> windows_core::Result<ISchemaModelGroup>;
    fn prohibitedSubstitutions(&self) -> windows_core::Result<SCHEMADERIVATIONMETHOD>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISchemaComplexType {}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaComplexType_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>() -> ISchemaComplexType_Vtbl {
        unsafe extern "system" fn isAbstract<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#abstract: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaComplexType_Impl::isAbstract(this) {
                Ok(ok__) => {
                    core::ptr::write(r#abstract, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn anyAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, anyattribute: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaComplexType_Impl::anyAttribute(this) {
                Ok(ok__) => {
                    core::ptr::write(anyattribute, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaComplexType_Impl::attributes(this) {
                Ok(ok__) => {
                    core::ptr::write(attributes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn contentType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: *mut SCHEMACONTENTTYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaComplexType_Impl::contentType(this) {
                Ok(ok__) => {
                    core::ptr::write(contenttype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn contentModel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentmodel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaComplexType_Impl::contentModel(this) {
                Ok(ok__) => {
                    core::ptr::write(contentmodel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn prohibitedSubstitutions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prohibited: *mut SCHEMADERIVATIONMETHOD) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaComplexType_Impl::prohibitedSubstitutions(this) {
                Ok(ok__) => {
                    core::ptr::write(prohibited, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaType_Vtbl::new::<Identity, Impl, OFFSET>(),
            isAbstract: isAbstract::<Identity, Impl, OFFSET>,
            anyAttribute: anyAttribute::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
            contentType: contentType::<Identity, Impl, OFFSET>,
            contentModel: contentModel::<Identity, Impl, OFFSET>,
            prohibitedSubstitutions: prohibitedSubstitutions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaComplexType as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID || iid == &<ISchemaType as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaElement_Impl: Sized + ISchemaParticle_Impl {
    fn r#type(&self) -> windows_core::Result<ISchemaType>;
    fn scope(&self) -> windows_core::Result<ISchemaComplexType>;
    fn defaultValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn fixedValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn isNillable(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn identityConstraints(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn substitutionGroup(&self) -> windows_core::Result<ISchemaElement>;
    fn substitutionGroupExclusions(&self) -> windows_core::Result<SCHEMADERIVATIONMETHOD>;
    fn disallowedSubstitutions(&self) -> windows_core::Result<SCHEMADERIVATIONMETHOD>;
    fn isAbstract(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn isReference(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISchemaElement {}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaElement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>() -> ISchemaElement_Vtbl {
        unsafe extern "system" fn r#type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaElement_Impl::r#type(this) {
                Ok(ok__) => {
                    core::ptr::write(r#type, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn scope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaElement_Impl::scope(this) {
                Ok(ok__) => {
                    core::ptr::write(scope, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn defaultValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, defaultvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaElement_Impl::defaultValue(this) {
                Ok(ok__) => {
                    core::ptr::write(defaultvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fixedValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fixedvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaElement_Impl::fixedValue(this) {
                Ok(ok__) => {
                    core::ptr::write(fixedvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isNillable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nillable: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaElement_Impl::isNillable(this) {
                Ok(ok__) => {
                    core::ptr::write(nillable, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn identityConstraints<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, constraints: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaElement_Impl::identityConstraints(this) {
                Ok(ok__) => {
                    core::ptr::write(constraints, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn substitutionGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaElement_Impl::substitutionGroup(this) {
                Ok(ok__) => {
                    core::ptr::write(element, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn substitutionGroupExclusions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, exclusions: *mut SCHEMADERIVATIONMETHOD) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaElement_Impl::substitutionGroupExclusions(this) {
                Ok(ok__) => {
                    core::ptr::write(exclusions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn disallowedSubstitutions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disallowed: *mut SCHEMADERIVATIONMETHOD) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaElement_Impl::disallowedSubstitutions(this) {
                Ok(ok__) => {
                    core::ptr::write(disallowed, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isAbstract<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#abstract: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaElement_Impl::isAbstract(this) {
                Ok(ok__) => {
                    core::ptr::write(r#abstract, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isReference<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reference: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaElement_Impl::isReference(this) {
                Ok(ok__) => {
                    core::ptr::write(reference, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaParticle_Vtbl::new::<Identity, Impl, OFFSET>(),
            r#type: r#type::<Identity, Impl, OFFSET>,
            scope: scope::<Identity, Impl, OFFSET>,
            defaultValue: defaultValue::<Identity, Impl, OFFSET>,
            fixedValue: fixedValue::<Identity, Impl, OFFSET>,
            isNillable: isNillable::<Identity, Impl, OFFSET>,
            identityConstraints: identityConstraints::<Identity, Impl, OFFSET>,
            substitutionGroup: substitutionGroup::<Identity, Impl, OFFSET>,
            substitutionGroupExclusions: substitutionGroupExclusions::<Identity, Impl, OFFSET>,
            disallowedSubstitutions: disallowedSubstitutions::<Identity, Impl, OFFSET>,
            isAbstract: isAbstract::<Identity, Impl, OFFSET>,
            isReference: isReference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaElement as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID || iid == &<ISchemaParticle as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaIdentityConstraint_Impl: Sized + ISchemaItem_Impl {
    fn selector(&self) -> windows_core::Result<windows_core::BSTR>;
    fn fields(&self) -> windows_core::Result<ISchemaStringCollection>;
    fn referencedKey(&self) -> windows_core::Result<ISchemaIdentityConstraint>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISchemaIdentityConstraint {}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaIdentityConstraint_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>() -> ISchemaIdentityConstraint_Vtbl {
        unsafe extern "system" fn selector<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selector: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaIdentityConstraint_Impl::selector(this) {
                Ok(ok__) => {
                    core::ptr::write(selector, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fields<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fields: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaIdentityConstraint_Impl::fields(this) {
                Ok(ok__) => {
                    core::ptr::write(fields, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn referencedKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaIdentityConstraint_Impl::referencedKey(this) {
                Ok(ok__) => {
                    core::ptr::write(key, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            selector: selector::<Identity, Impl, OFFSET>,
            fields: fields::<Identity, Impl, OFFSET>,
            referencedKey: referencedKey::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaIdentityConstraint as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaItem_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn namespaceURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn schema(&self) -> windows_core::Result<ISchema>;
    fn id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn itemType(&self) -> windows_core::Result<SOMITEMTYPE>;
    fn unhandledAttributes(&self) -> windows_core::Result<IVBSAXAttributes>;
    fn writeAnnotation(&self, annotationsink: Option<&windows_core::IUnknown>) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISchemaItem {}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>() -> ISchemaItem_Vtbl {
        unsafe extern "system" fn name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaItem_Impl::name(this) {
                Ok(ok__) => {
                    core::ptr::write(name, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn namespaceURI<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceuri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaItem_Impl::namespaceURI(this) {
                Ok(ok__) => {
                    core::ptr::write(namespaceuri, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schema<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, schema: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaItem_Impl::schema(this) {
                Ok(ok__) => {
                    core::ptr::write(schema, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn id<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaItem_Impl::id(this) {
                Ok(ok__) => {
                    core::ptr::write(id, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemtype: *mut SOMITEMTYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaItem_Impl::itemType(this) {
                Ok(ok__) => {
                    core::ptr::write(itemtype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn unhandledAttributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaItem_Impl::unhandledAttributes(this) {
                Ok(ok__) => {
                    core::ptr::write(attributes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn writeAnnotation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, annotationsink: *mut core::ffi::c_void, iswritten: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaItem_Impl::writeAnnotation(this, windows_core::from_raw_borrowed(&annotationsink)) {
                Ok(ok__) => {
                    core::ptr::write(iswritten, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            name: name::<Identity, Impl, OFFSET>,
            namespaceURI: namespaceURI::<Identity, Impl, OFFSET>,
            schema: schema::<Identity, Impl, OFFSET>,
            id: id::<Identity, Impl, OFFSET>,
            itemType: itemType::<Identity, Impl, OFFSET>,
            unhandledAttributes: unhandledAttributes::<Identity, Impl, OFFSET>,
            writeAnnotation: writeAnnotation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaItem as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaItemCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(&self, index: i32) -> windows_core::Result<ISchemaItem>;
    fn itemByName(&self, name: &windows_core::BSTR) -> windows_core::Result<ISchemaItem>;
    fn itemByQName(&self, name: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::Result<ISchemaItem>;
    fn length(&self) -> windows_core::Result<i32>;
    fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISchemaItemCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaItemCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>() -> ISchemaItemCollection_Vtbl {
        unsafe extern "system" fn get_item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaItemCollection_Impl::get_item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(item, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaItemCollection_Impl::itemByName(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(item, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemByQName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, namespaceuri: core::mem::MaybeUninit<windows_core::BSTR>, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaItemCollection_Impl::itemByQName(this, core::mem::transmute(&name), core::mem::transmute(&namespaceuri)) {
                Ok(ok__) => {
                    core::ptr::write(item, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaItemCollection_Impl::length(this) {
                Ok(ok__) => {
                    core::ptr::write(length, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaItemCollection_Impl::_newEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_item: get_item::<Identity, Impl, OFFSET>,
            itemByName: itemByName::<Identity, Impl, OFFSET>,
            itemByQName: itemByQName::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaItemCollection as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaModelGroup_Impl: Sized + ISchemaParticle_Impl {
    fn particles(&self) -> windows_core::Result<ISchemaItemCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISchemaModelGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaModelGroup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaModelGroup_Impl, const OFFSET: isize>() -> ISchemaModelGroup_Vtbl {
        unsafe extern "system" fn particles<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaModelGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, particles: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaModelGroup_Impl::particles(this) {
                Ok(ok__) => {
                    core::ptr::write(particles, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ISchemaParticle_Vtbl::new::<Identity, Impl, OFFSET>(), particles: particles::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaModelGroup as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID || iid == &<ISchemaParticle as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaNotation_Impl: Sized + ISchemaItem_Impl {
    fn systemIdentifier(&self) -> windows_core::Result<windows_core::BSTR>;
    fn publicIdentifier(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISchemaNotation {}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaNotation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaNotation_Impl, const OFFSET: isize>() -> ISchemaNotation_Vtbl {
        unsafe extern "system" fn systemIdentifier<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaNotation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaNotation_Impl::systemIdentifier(this) {
                Ok(ok__) => {
                    core::ptr::write(uri, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn publicIdentifier<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaNotation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaNotation_Impl::publicIdentifier(this) {
                Ok(ok__) => {
                    core::ptr::write(uri, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            systemIdentifier: systemIdentifier::<Identity, Impl, OFFSET>,
            publicIdentifier: publicIdentifier::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaNotation as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaParticle_Impl: Sized + ISchemaItem_Impl {
    fn minOccurs(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn maxOccurs(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISchemaParticle {}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaParticle_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaParticle_Impl, const OFFSET: isize>() -> ISchemaParticle_Vtbl {
        unsafe extern "system" fn minOccurs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaParticle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minoccurs: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaParticle_Impl::minOccurs(this) {
                Ok(ok__) => {
                    core::ptr::write(minoccurs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxOccurs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaParticle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxoccurs: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaParticle_Impl::maxOccurs(this) {
                Ok(ok__) => {
                    core::ptr::write(maxoccurs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            minOccurs: minOccurs::<Identity, Impl, OFFSET>,
            maxOccurs: maxOccurs::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaParticle as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaStringCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn length(&self) -> windows_core::Result<i32>;
    fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISchemaStringCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaStringCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaStringCollection_Impl, const OFFSET: isize>() -> ISchemaStringCollection_Vtbl {
        unsafe extern "system" fn get_item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, bstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaStringCollection_Impl::get_item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(bstr, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaStringCollection_Impl::length(this) {
                Ok(ok__) => {
                    core::ptr::write(length, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaStringCollection_Impl::_newEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_item: get_item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaStringCollection as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaType_Impl: Sized + ISchemaItem_Impl {
    fn baseTypes(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn r#final(&self) -> windows_core::Result<SCHEMADERIVATIONMETHOD>;
    fn variety(&self) -> windows_core::Result<SCHEMATYPEVARIETY>;
    fn derivedBy(&self) -> windows_core::Result<SCHEMADERIVATIONMETHOD>;
    fn isValid(&self, data: &windows_core::BSTR) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn minExclusive(&self) -> windows_core::Result<windows_core::BSTR>;
    fn minInclusive(&self) -> windows_core::Result<windows_core::BSTR>;
    fn maxExclusive(&self) -> windows_core::Result<windows_core::BSTR>;
    fn maxInclusive(&self) -> windows_core::Result<windows_core::BSTR>;
    fn totalDigits(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn fractionDigits(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn length(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn minLength(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn maxLength(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn enumeration(&self) -> windows_core::Result<ISchemaStringCollection>;
    fn whitespace(&self) -> windows_core::Result<SCHEMAWHITESPACE>;
    fn patterns(&self) -> windows_core::Result<ISchemaStringCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISchemaType {}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaType_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>() -> ISchemaType_Vtbl {
        unsafe extern "system" fn baseTypes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, basetypes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::baseTypes(this) {
                Ok(ok__) => {
                    core::ptr::write(basetypes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#final<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#final: *mut SCHEMADERIVATIONMETHOD) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::r#final(this) {
                Ok(ok__) => {
                    core::ptr::write(r#final, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn variety<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variety: *mut SCHEMATYPEVARIETY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::variety(this) {
                Ok(ok__) => {
                    core::ptr::write(variety, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn derivedBy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, derivedby: *mut SCHEMADERIVATIONMETHOD) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::derivedBy(this) {
                Ok(ok__) => {
                    core::ptr::write(derivedby, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isValid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::BSTR>, valid: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::isValid(this, core::mem::transmute(&data)) {
                Ok(ok__) => {
                    core::ptr::write(valid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minExclusive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minexclusive: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::minExclusive(this) {
                Ok(ok__) => {
                    core::ptr::write(minexclusive, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minInclusive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mininclusive: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::minInclusive(this) {
                Ok(ok__) => {
                    core::ptr::write(mininclusive, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxExclusive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxexclusive: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::maxExclusive(this) {
                Ok(ok__) => {
                    core::ptr::write(maxexclusive, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxInclusive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxinclusive: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::maxInclusive(this) {
                Ok(ok__) => {
                    core::ptr::write(maxinclusive, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn totalDigits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, totaldigits: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::totalDigits(this) {
                Ok(ok__) => {
                    core::ptr::write(totaldigits, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fractionDigits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fractiondigits: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::fractionDigits(this) {
                Ok(ok__) => {
                    core::ptr::write(fractiondigits, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::length(this) {
                Ok(ok__) => {
                    core::ptr::write(length, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minlength: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::minLength(this) {
                Ok(ok__) => {
                    core::ptr::write(minlength, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlength: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::maxLength(this) {
                Ok(ok__) => {
                    core::ptr::write(maxlength, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn enumeration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumeration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::enumeration(this) {
                Ok(ok__) => {
                    core::ptr::write(enumeration, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn whitespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, whitespace: *mut SCHEMAWHITESPACE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::whitespace(this) {
                Ok(ok__) => {
                    core::ptr::write(whitespace, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn patterns<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, patterns: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISchemaType_Impl::patterns(this) {
                Ok(ok__) => {
                    core::ptr::write(patterns, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            baseTypes: baseTypes::<Identity, Impl, OFFSET>,
            r#final: r#final::<Identity, Impl, OFFSET>,
            variety: variety::<Identity, Impl, OFFSET>,
            derivedBy: derivedBy::<Identity, Impl, OFFSET>,
            isValid: isValid::<Identity, Impl, OFFSET>,
            minExclusive: minExclusive::<Identity, Impl, OFFSET>,
            minInclusive: minInclusive::<Identity, Impl, OFFSET>,
            maxExclusive: maxExclusive::<Identity, Impl, OFFSET>,
            maxInclusive: maxInclusive::<Identity, Impl, OFFSET>,
            totalDigits: totalDigits::<Identity, Impl, OFFSET>,
            fractionDigits: fractionDigits::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            minLength: minLength::<Identity, Impl, OFFSET>,
            maxLength: maxLength::<Identity, Impl, OFFSET>,
            enumeration: enumeration::<Identity, Impl, OFFSET>,
            whitespace: whitespace::<Identity, Impl, OFFSET>,
            patterns: patterns::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaType as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IServerXMLHTTPRequest_Impl: Sized + IXMLHTTPRequest_Impl {
    fn setTimeouts(&self, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> windows_core::Result<()>;
    fn waitForResponse(&self, timeoutinseconds: &windows_core::VARIANT) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn getOption(&self, option: SERVERXMLHTTP_OPTION) -> windows_core::Result<windows_core::VARIANT>;
    fn setOption(&self, option: SERVERXMLHTTP_OPTION, value: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IServerXMLHTTPRequest {}
#[cfg(feature = "Win32_System_Com")]
impl IServerXMLHTTPRequest_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>() -> IServerXMLHTTPRequest_Vtbl {
        unsafe extern "system" fn setTimeouts<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServerXMLHTTPRequest_Impl::setTimeouts(this, core::mem::transmute_copy(&resolvetimeout), core::mem::transmute_copy(&connecttimeout), core::mem::transmute_copy(&sendtimeout), core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn waitForResponse<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeoutinseconds: core::mem::MaybeUninit<windows_core::VARIANT>, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IServerXMLHTTPRequest_Impl::waitForResponse(this, core::mem::transmute(&timeoutinseconds)) {
                Ok(ok__) => {
                    core::ptr::write(issuccessful, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getOption<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IServerXMLHTTPRequest_Impl::getOption(this, core::mem::transmute_copy(&option)) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setOption<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServerXMLHTTPRequest_Impl::setOption(this, core::mem::transmute_copy(&option), core::mem::transmute(&value)).into()
        }
        Self {
            base__: IXMLHTTPRequest_Vtbl::new::<Identity, Impl, OFFSET>(),
            setTimeouts: setTimeouts::<Identity, Impl, OFFSET>,
            waitForResponse: waitForResponse::<Identity, Impl, OFFSET>,
            getOption: getOption::<Identity, Impl, OFFSET>,
            setOption: setOption::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServerXMLHTTPRequest as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLHTTPRequest as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IServerXMLHTTPRequest2_Impl: Sized + IServerXMLHTTPRequest_Impl {
    fn setProxy(&self, proxysetting: SXH_PROXY_SETTING, varproxyserver: &windows_core::VARIANT, varbypasslist: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn setProxyCredentials(&self, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IServerXMLHTTPRequest2 {}
#[cfg(feature = "Win32_System_Com")]
impl IServerXMLHTTPRequest2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>() -> IServerXMLHTTPRequest2_Vtbl {
        unsafe extern "system" fn setProxy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, proxysetting: SXH_PROXY_SETTING, varproxyserver: core::mem::MaybeUninit<windows_core::VARIANT>, varbypasslist: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServerXMLHTTPRequest2_Impl::setProxy(this, core::mem::transmute_copy(&proxysetting), core::mem::transmute(&varproxyserver), core::mem::transmute(&varbypasslist)).into()
        }
        unsafe extern "system" fn setProxyCredentials<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServerXMLHTTPRequest2_Impl::setProxyCredentials(this, core::mem::transmute(&bstrusername), core::mem::transmute(&bstrpassword)).into()
        }
        Self {
            base__: IServerXMLHTTPRequest_Vtbl::new::<Identity, Impl, OFFSET>(),
            setProxy: setProxy::<Identity, Impl, OFFSET>,
            setProxyCredentials: setProxyCredentials::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServerXMLHTTPRequest2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLHTTPRequest as windows_core::Interface>::IID || iid == &<IServerXMLHTTPRequest as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBMXNamespaceManager_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn SetallowOverride(&self, foverride: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn allowOverride(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn reset(&self) -> windows_core::Result<()>;
    fn pushContext(&self) -> windows_core::Result<()>;
    fn pushNodeContext(&self, contextnode: Option<&IXMLDOMNode>, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn popContext(&self) -> windows_core::Result<()>;
    fn declarePrefix(&self, prefix: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn getDeclaredPrefixes(&self) -> windows_core::Result<IMXNamespacePrefixes>;
    fn getPrefixes(&self, namespaceuri: &windows_core::BSTR) -> windows_core::Result<IMXNamespacePrefixes>;
    fn getURI(&self, prefix: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn getURIFromNode(&self, strprefix: &windows_core::BSTR, contextnode: Option<&IXMLDOMNode>) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IVBMXNamespaceManager {}
#[cfg(feature = "Win32_System_Com")]
impl IVBMXNamespaceManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>() -> IVBMXNamespaceManager_Vtbl {
        unsafe extern "system" fn SetallowOverride<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, foverride: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBMXNamespaceManager_Impl::SetallowOverride(this, core::mem::transmute_copy(&foverride)).into()
        }
        unsafe extern "system" fn allowOverride<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, foverride: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBMXNamespaceManager_Impl::allowOverride(this) {
                Ok(ok__) => {
                    core::ptr::write(foverride, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBMXNamespaceManager_Impl::reset(this).into()
        }
        unsafe extern "system" fn pushContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBMXNamespaceManager_Impl::pushContext(this).into()
        }
        unsafe extern "system" fn pushNodeContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextnode: *mut core::ffi::c_void, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBMXNamespaceManager_Impl::pushNodeContext(this, windows_core::from_raw_borrowed(&contextnode), core::mem::transmute_copy(&fdeep)).into()
        }
        unsafe extern "system" fn popContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBMXNamespaceManager_Impl::popContext(this).into()
        }
        unsafe extern "system" fn declarePrefix<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefix: core::mem::MaybeUninit<windows_core::BSTR>, namespaceuri: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBMXNamespaceManager_Impl::declarePrefix(this, core::mem::transmute(&prefix), core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn getDeclaredPrefixes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefixes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBMXNamespaceManager_Impl::getDeclaredPrefixes(this) {
                Ok(ok__) => {
                    core::ptr::write(prefixes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPrefixes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceuri: core::mem::MaybeUninit<windows_core::BSTR>, prefixes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBMXNamespaceManager_Impl::getPrefixes(this, core::mem::transmute(&namespaceuri)) {
                Ok(ok__) => {
                    core::ptr::write(prefixes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefix: core::mem::MaybeUninit<windows_core::BSTR>, uri: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBMXNamespaceManager_Impl::getURI(this, core::mem::transmute(&prefix)) {
                Ok(ok__) => {
                    core::ptr::write(uri, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURIFromNode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strprefix: core::mem::MaybeUninit<windows_core::BSTR>, contextnode: *mut core::ffi::c_void, uri: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBMXNamespaceManager_Impl::getURIFromNode(this, core::mem::transmute(&strprefix), windows_core::from_raw_borrowed(&contextnode)) {
                Ok(ok__) => {
                    core::ptr::write(uri, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetallowOverride: SetallowOverride::<Identity, Impl, OFFSET>,
            allowOverride: allowOverride::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            pushContext: pushContext::<Identity, Impl, OFFSET>,
            pushNodeContext: pushNodeContext::<Identity, Impl, OFFSET>,
            popContext: popContext::<Identity, Impl, OFFSET>,
            declarePrefix: declarePrefix::<Identity, Impl, OFFSET>,
            getDeclaredPrefixes: getDeclaredPrefixes::<Identity, Impl, OFFSET>,
            getPrefixes: getPrefixes::<Identity, Impl, OFFSET>,
            getURI: getURI::<Identity, Impl, OFFSET>,
            getURIFromNode: getURIFromNode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBMXNamespaceManager as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXAttributes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn length(&self) -> windows_core::Result<i32>;
    fn getURI(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn getLocalName(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn getQName(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn getIndexFromName(&self, struri: &windows_core::BSTR, strlocalname: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn getIndexFromQName(&self, strqname: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn getType(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn getTypeFromName(&self, struri: &windows_core::BSTR, strlocalname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn getTypeFromQName(&self, strqname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn getValue(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn getValueFromName(&self, struri: &windows_core::BSTR, strlocalname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn getValueFromQName(&self, strqname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IVBSAXAttributes {}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXAttributes_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>() -> IVBSAXAttributes_Vtbl {
        unsafe extern "system" fn length<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nlength: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXAttributes_Impl::length(this) {
                Ok(ok__) => {
                    core::ptr::write(nlength, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, struri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXAttributes_Impl::getURI(this, core::mem::transmute_copy(&nindex)) {
                Ok(ok__) => {
                    core::ptr::write(struri, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getLocalName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, strlocalname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXAttributes_Impl::getLocalName(this, core::mem::transmute_copy(&nindex)) {
                Ok(ok__) => {
                    core::ptr::write(strlocalname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getQName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, strqname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXAttributes_Impl::getQName(this, core::mem::transmute_copy(&nindex)) {
                Ok(ok__) => {
                    core::ptr::write(strqname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, struri: core::mem::MaybeUninit<windows_core::BSTR>, strlocalname: core::mem::MaybeUninit<windows_core::BSTR>, nindex: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXAttributes_Impl::getIndexFromName(this, core::mem::transmute(&struri), core::mem::transmute(&strlocalname)) {
                Ok(ok__) => {
                    core::ptr::write(nindex, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromQName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strqname: core::mem::MaybeUninit<windows_core::BSTR>, nindex: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXAttributes_Impl::getIndexFromQName(this, core::mem::transmute(&strqname)) {
                Ok(ok__) => {
                    core::ptr::write(nindex, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, strtype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXAttributes_Impl::getType(this, core::mem::transmute_copy(&nindex)) {
                Ok(ok__) => {
                    core::ptr::write(strtype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getTypeFromName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, struri: core::mem::MaybeUninit<windows_core::BSTR>, strlocalname: core::mem::MaybeUninit<windows_core::BSTR>, strtype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXAttributes_Impl::getTypeFromName(this, core::mem::transmute(&struri), core::mem::transmute(&strlocalname)) {
                Ok(ok__) => {
                    core::ptr::write(strtype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getTypeFromQName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strqname: core::mem::MaybeUninit<windows_core::BSTR>, strtype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXAttributes_Impl::getTypeFromQName(this, core::mem::transmute(&strqname)) {
                Ok(ok__) => {
                    core::ptr::write(strtype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, strvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXAttributes_Impl::getValue(this, core::mem::transmute_copy(&nindex)) {
                Ok(ok__) => {
                    core::ptr::write(strvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValueFromName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, struri: core::mem::MaybeUninit<windows_core::BSTR>, strlocalname: core::mem::MaybeUninit<windows_core::BSTR>, strvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXAttributes_Impl::getValueFromName(this, core::mem::transmute(&struri), core::mem::transmute(&strlocalname)) {
                Ok(ok__) => {
                    core::ptr::write(strvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValueFromQName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strqname: core::mem::MaybeUninit<windows_core::BSTR>, strvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXAttributes_Impl::getValueFromQName(this, core::mem::transmute(&strqname)) {
                Ok(ok__) => {
                    core::ptr::write(strvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            length: length::<Identity, Impl, OFFSET>,
            getURI: getURI::<Identity, Impl, OFFSET>,
            getLocalName: getLocalName::<Identity, Impl, OFFSET>,
            getQName: getQName::<Identity, Impl, OFFSET>,
            getIndexFromName: getIndexFromName::<Identity, Impl, OFFSET>,
            getIndexFromQName: getIndexFromQName::<Identity, Impl, OFFSET>,
            getType: getType::<Identity, Impl, OFFSET>,
            getTypeFromName: getTypeFromName::<Identity, Impl, OFFSET>,
            getTypeFromQName: getTypeFromQName::<Identity, Impl, OFFSET>,
            getValue: getValue::<Identity, Impl, OFFSET>,
            getValueFromName: getValueFromName::<Identity, Impl, OFFSET>,
            getValueFromQName: getValueFromQName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXAttributes as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXContentHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn putref_documentLocator(&self, olocator: Option<&IVBSAXLocator>) -> windows_core::Result<()>;
    fn startDocument(&self) -> windows_core::Result<()>;
    fn endDocument(&self) -> windows_core::Result<()>;
    fn startPrefixMapping(&self, strprefix: *mut windows_core::BSTR, struri: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn endPrefixMapping(&self, strprefix: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn startElement(&self, strnamespaceuri: *mut windows_core::BSTR, strlocalname: *mut windows_core::BSTR, strqname: *mut windows_core::BSTR, oattributes: Option<&IVBSAXAttributes>) -> windows_core::Result<()>;
    fn endElement(&self, strnamespaceuri: *mut windows_core::BSTR, strlocalname: *mut windows_core::BSTR, strqname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn characters(&self, strchars: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn ignorableWhitespace(&self, strchars: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn processingInstruction(&self, strtarget: *mut windows_core::BSTR, strdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn skippedEntity(&self, strname: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IVBSAXContentHandler {}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXContentHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>() -> IVBSAXContentHandler_Vtbl {
        unsafe extern "system" fn putref_documentLocator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, olocator: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXContentHandler_Impl::putref_documentLocator(this, windows_core::from_raw_borrowed(&olocator)).into()
        }
        unsafe extern "system" fn startDocument<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXContentHandler_Impl::startDocument(this).into()
        }
        unsafe extern "system" fn endDocument<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXContentHandler_Impl::endDocument(this).into()
        }
        unsafe extern "system" fn startPrefixMapping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strprefix: *mut core::mem::MaybeUninit<windows_core::BSTR>, struri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXContentHandler_Impl::startPrefixMapping(this, core::mem::transmute_copy(&strprefix), core::mem::transmute_copy(&struri)).into()
        }
        unsafe extern "system" fn endPrefixMapping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strprefix: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXContentHandler_Impl::endPrefixMapping(this, core::mem::transmute_copy(&strprefix)).into()
        }
        unsafe extern "system" fn startElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnamespaceuri: *mut core::mem::MaybeUninit<windows_core::BSTR>, strlocalname: *mut core::mem::MaybeUninit<windows_core::BSTR>, strqname: *mut core::mem::MaybeUninit<windows_core::BSTR>, oattributes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXContentHandler_Impl::startElement(this, core::mem::transmute_copy(&strnamespaceuri), core::mem::transmute_copy(&strlocalname), core::mem::transmute_copy(&strqname), windows_core::from_raw_borrowed(&oattributes)).into()
        }
        unsafe extern "system" fn endElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnamespaceuri: *mut core::mem::MaybeUninit<windows_core::BSTR>, strlocalname: *mut core::mem::MaybeUninit<windows_core::BSTR>, strqname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXContentHandler_Impl::endElement(this, core::mem::transmute_copy(&strnamespaceuri), core::mem::transmute_copy(&strlocalname), core::mem::transmute_copy(&strqname)).into()
        }
        unsafe extern "system" fn characters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strchars: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXContentHandler_Impl::characters(this, core::mem::transmute_copy(&strchars)).into()
        }
        unsafe extern "system" fn ignorableWhitespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strchars: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXContentHandler_Impl::ignorableWhitespace(this, core::mem::transmute_copy(&strchars)).into()
        }
        unsafe extern "system" fn processingInstruction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strtarget: *mut core::mem::MaybeUninit<windows_core::BSTR>, strdata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXContentHandler_Impl::processingInstruction(this, core::mem::transmute_copy(&strtarget), core::mem::transmute_copy(&strdata)).into()
        }
        unsafe extern "system" fn skippedEntity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXContentHandler_Impl::skippedEntity(this, core::mem::transmute_copy(&strname)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            putref_documentLocator: putref_documentLocator::<Identity, Impl, OFFSET>,
            startDocument: startDocument::<Identity, Impl, OFFSET>,
            endDocument: endDocument::<Identity, Impl, OFFSET>,
            startPrefixMapping: startPrefixMapping::<Identity, Impl, OFFSET>,
            endPrefixMapping: endPrefixMapping::<Identity, Impl, OFFSET>,
            startElement: startElement::<Identity, Impl, OFFSET>,
            endElement: endElement::<Identity, Impl, OFFSET>,
            characters: characters::<Identity, Impl, OFFSET>,
            ignorableWhitespace: ignorableWhitespace::<Identity, Impl, OFFSET>,
            processingInstruction: processingInstruction::<Identity, Impl, OFFSET>,
            skippedEntity: skippedEntity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXContentHandler as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXDTDHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn notationDecl(&self, strname: *mut windows_core::BSTR, strpublicid: *mut windows_core::BSTR, strsystemid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn unparsedEntityDecl(&self, strname: *mut windows_core::BSTR, strpublicid: *mut windows_core::BSTR, strsystemid: *mut windows_core::BSTR, strnotationname: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IVBSAXDTDHandler {}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXDTDHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDTDHandler_Impl, const OFFSET: isize>() -> IVBSAXDTDHandler_Vtbl {
        unsafe extern "system" fn notationDecl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDTDHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>, strpublicid: *mut core::mem::MaybeUninit<windows_core::BSTR>, strsystemid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXDTDHandler_Impl::notationDecl(this, core::mem::transmute_copy(&strname), core::mem::transmute_copy(&strpublicid), core::mem::transmute_copy(&strsystemid)).into()
        }
        unsafe extern "system" fn unparsedEntityDecl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDTDHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>, strpublicid: *mut core::mem::MaybeUninit<windows_core::BSTR>, strsystemid: *mut core::mem::MaybeUninit<windows_core::BSTR>, strnotationname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXDTDHandler_Impl::unparsedEntityDecl(this, core::mem::transmute_copy(&strname), core::mem::transmute_copy(&strpublicid), core::mem::transmute_copy(&strsystemid), core::mem::transmute_copy(&strnotationname)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            notationDecl: notationDecl::<Identity, Impl, OFFSET>,
            unparsedEntityDecl: unparsedEntityDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXDTDHandler as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXDeclHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn elementDecl(&self, strname: *mut windows_core::BSTR, strmodel: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn attributeDecl(&self, strelementname: *mut windows_core::BSTR, strattributename: *mut windows_core::BSTR, strtype: *mut windows_core::BSTR, strvaluedefault: *mut windows_core::BSTR, strvalue: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn internalEntityDecl(&self, strname: *mut windows_core::BSTR, strvalue: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn externalEntityDecl(&self, strname: *mut windows_core::BSTR, strpublicid: *mut windows_core::BSTR, strsystemid: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IVBSAXDeclHandler {}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXDeclHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>() -> IVBSAXDeclHandler_Vtbl {
        unsafe extern "system" fn elementDecl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>, strmodel: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXDeclHandler_Impl::elementDecl(this, core::mem::transmute_copy(&strname), core::mem::transmute_copy(&strmodel)).into()
        }
        unsafe extern "system" fn attributeDecl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strelementname: *mut core::mem::MaybeUninit<windows_core::BSTR>, strattributename: *mut core::mem::MaybeUninit<windows_core::BSTR>, strtype: *mut core::mem::MaybeUninit<windows_core::BSTR>, strvaluedefault: *mut core::mem::MaybeUninit<windows_core::BSTR>, strvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXDeclHandler_Impl::attributeDecl(this, core::mem::transmute_copy(&strelementname), core::mem::transmute_copy(&strattributename), core::mem::transmute_copy(&strtype), core::mem::transmute_copy(&strvaluedefault), core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn internalEntityDecl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>, strvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXDeclHandler_Impl::internalEntityDecl(this, core::mem::transmute_copy(&strname), core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn externalEntityDecl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>, strpublicid: *mut core::mem::MaybeUninit<windows_core::BSTR>, strsystemid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXDeclHandler_Impl::externalEntityDecl(this, core::mem::transmute_copy(&strname), core::mem::transmute_copy(&strpublicid), core::mem::transmute_copy(&strsystemid)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            elementDecl: elementDecl::<Identity, Impl, OFFSET>,
            attributeDecl: attributeDecl::<Identity, Impl, OFFSET>,
            internalEntityDecl: internalEntityDecl::<Identity, Impl, OFFSET>,
            externalEntityDecl: externalEntityDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXDeclHandler as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXEntityResolver_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn resolveEntity(&self, strpublicid: *mut windows_core::BSTR, strsystemid: *mut windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IVBSAXEntityResolver {}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXEntityResolver_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXEntityResolver_Impl, const OFFSET: isize>() -> IVBSAXEntityResolver_Vtbl {
        unsafe extern "system" fn resolveEntity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXEntityResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpublicid: *mut core::mem::MaybeUninit<windows_core::BSTR>, strsystemid: *mut core::mem::MaybeUninit<windows_core::BSTR>, varinput: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXEntityResolver_Impl::resolveEntity(this, core::mem::transmute_copy(&strpublicid), core::mem::transmute_copy(&strsystemid)) {
                Ok(ok__) => {
                    core::ptr::write(varinput, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            resolveEntity: resolveEntity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXEntityResolver as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXErrorHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn error(&self, olocator: Option<&IVBSAXLocator>, strerrormessage: *mut windows_core::BSTR, nerrorcode: i32) -> windows_core::Result<()>;
    fn fatalError(&self, olocator: Option<&IVBSAXLocator>, strerrormessage: *mut windows_core::BSTR, nerrorcode: i32) -> windows_core::Result<()>;
    fn ignorableWarning(&self, olocator: Option<&IVBSAXLocator>, strerrormessage: *mut windows_core::BSTR, nerrorcode: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IVBSAXErrorHandler {}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXErrorHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>() -> IVBSAXErrorHandler_Vtbl {
        unsafe extern "system" fn error<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, olocator: *mut core::ffi::c_void, strerrormessage: *mut core::mem::MaybeUninit<windows_core::BSTR>, nerrorcode: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXErrorHandler_Impl::error(this, windows_core::from_raw_borrowed(&olocator), core::mem::transmute_copy(&strerrormessage), core::mem::transmute_copy(&nerrorcode)).into()
        }
        unsafe extern "system" fn fatalError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, olocator: *mut core::ffi::c_void, strerrormessage: *mut core::mem::MaybeUninit<windows_core::BSTR>, nerrorcode: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXErrorHandler_Impl::fatalError(this, windows_core::from_raw_borrowed(&olocator), core::mem::transmute_copy(&strerrormessage), core::mem::transmute_copy(&nerrorcode)).into()
        }
        unsafe extern "system" fn ignorableWarning<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, olocator: *mut core::ffi::c_void, strerrormessage: *mut core::mem::MaybeUninit<windows_core::BSTR>, nerrorcode: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXErrorHandler_Impl::ignorableWarning(this, windows_core::from_raw_borrowed(&olocator), core::mem::transmute_copy(&strerrormessage), core::mem::transmute_copy(&nerrorcode)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            error: error::<Identity, Impl, OFFSET>,
            fatalError: fatalError::<Identity, Impl, OFFSET>,
            ignorableWarning: ignorableWarning::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXErrorHandler as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXLexicalHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn startDTD(&self, strname: *mut windows_core::BSTR, strpublicid: *mut windows_core::BSTR, strsystemid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn endDTD(&self) -> windows_core::Result<()>;
    fn startEntity(&self, strname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn endEntity(&self, strname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn startCDATA(&self) -> windows_core::Result<()>;
    fn endCDATA(&self) -> windows_core::Result<()>;
    fn comment(&self, strchars: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IVBSAXLexicalHandler {}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXLexicalHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>() -> IVBSAXLexicalHandler_Vtbl {
        unsafe extern "system" fn startDTD<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>, strpublicid: *mut core::mem::MaybeUninit<windows_core::BSTR>, strsystemid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXLexicalHandler_Impl::startDTD(this, core::mem::transmute_copy(&strname), core::mem::transmute_copy(&strpublicid), core::mem::transmute_copy(&strsystemid)).into()
        }
        unsafe extern "system" fn endDTD<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXLexicalHandler_Impl::endDTD(this).into()
        }
        unsafe extern "system" fn startEntity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXLexicalHandler_Impl::startEntity(this, core::mem::transmute_copy(&strname)).into()
        }
        unsafe extern "system" fn endEntity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXLexicalHandler_Impl::endEntity(this, core::mem::transmute_copy(&strname)).into()
        }
        unsafe extern "system" fn startCDATA<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXLexicalHandler_Impl::startCDATA(this).into()
        }
        unsafe extern "system" fn endCDATA<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXLexicalHandler_Impl::endCDATA(this).into()
        }
        unsafe extern "system" fn comment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strchars: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXLexicalHandler_Impl::comment(this, core::mem::transmute_copy(&strchars)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            startDTD: startDTD::<Identity, Impl, OFFSET>,
            endDTD: endDTD::<Identity, Impl, OFFSET>,
            startEntity: startEntity::<Identity, Impl, OFFSET>,
            endEntity: endEntity::<Identity, Impl, OFFSET>,
            startCDATA: startCDATA::<Identity, Impl, OFFSET>,
            endCDATA: endCDATA::<Identity, Impl, OFFSET>,
            comment: comment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXLexicalHandler as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXLocator_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn columnNumber(&self) -> windows_core::Result<i32>;
    fn lineNumber(&self) -> windows_core::Result<i32>;
    fn publicId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn systemId(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IVBSAXLocator {}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXLocator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: isize>() -> IVBSAXLocator_Vtbl {
        unsafe extern "system" fn columnNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncolumn: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXLocator_Impl::columnNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(ncolumn, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lineNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nline: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXLocator_Impl::lineNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(nline, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn publicId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpublicid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXLocator_Impl::publicId(this) {
                Ok(ok__) => {
                    core::ptr::write(strpublicid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn systemId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsystemid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXLocator_Impl::systemId(this) {
                Ok(ok__) => {
                    core::ptr::write(strsystemid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            columnNumber: columnNumber::<Identity, Impl, OFFSET>,
            lineNumber: lineNumber::<Identity, Impl, OFFSET>,
            publicId: publicId::<Identity, Impl, OFFSET>,
            systemId: systemId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXLocator as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXXMLFilter_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn parent(&self) -> windows_core::Result<IVBSAXXMLReader>;
    fn putref_parent(&self, oreader: Option<&IVBSAXXMLReader>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IVBSAXXMLFilter {}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXXMLFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLFilter_Impl, const OFFSET: isize>() -> IVBSAXXMLFilter_Vtbl {
        unsafe extern "system" fn parent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXXMLFilter_Impl::parent(this) {
                Ok(ok__) => {
                    core::ptr::write(oreader, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_parent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oreader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXXMLFilter_Impl::putref_parent(this, windows_core::from_raw_borrowed(&oreader)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            parent: parent::<Identity, Impl, OFFSET>,
            putref_parent: putref_parent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXXMLFilter as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXXMLReader_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn getFeature(&self, strname: &windows_core::BSTR) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn putFeature(&self, strname: &windows_core::BSTR, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn getProperty(&self, strname: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn putProperty(&self, strname: &windows_core::BSTR, varvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn entityResolver(&self) -> windows_core::Result<IVBSAXEntityResolver>;
    fn putref_entityResolver(&self, oresolver: Option<&IVBSAXEntityResolver>) -> windows_core::Result<()>;
    fn contentHandler(&self) -> windows_core::Result<IVBSAXContentHandler>;
    fn putref_contentHandler(&self, ohandler: Option<&IVBSAXContentHandler>) -> windows_core::Result<()>;
    fn dtdHandler(&self) -> windows_core::Result<IVBSAXDTDHandler>;
    fn putref_dtdHandler(&self, ohandler: Option<&IVBSAXDTDHandler>) -> windows_core::Result<()>;
    fn errorHandler(&self) -> windows_core::Result<IVBSAXErrorHandler>;
    fn putref_errorHandler(&self, ohandler: Option<&IVBSAXErrorHandler>) -> windows_core::Result<()>;
    fn baseURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetbaseURL(&self, strbaseurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn secureBaseURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetsecureBaseURL(&self, strsecurebaseurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn parse(&self, varinput: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn parseURL(&self, strurl: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IVBSAXXMLReader {}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXXMLReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>() -> IVBSAXXMLReader_Vtbl {
        unsafe extern "system" fn getFeature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXXMLReader_Impl::getFeature(this, core::mem::transmute(&strname)) {
                Ok(ok__) => {
                    core::ptr::write(fvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXXMLReader_Impl::putFeature(this, core::mem::transmute(&strname), core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn getProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, varvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXXMLReader_Impl::getProperty(this, core::mem::transmute(&strname)) {
                Ok(ok__) => {
                    core::ptr::write(varvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, varvalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXXMLReader_Impl::putProperty(this, core::mem::transmute(&strname), core::mem::transmute(&varvalue)).into()
        }
        unsafe extern "system" fn entityResolver<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oresolver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXXMLReader_Impl::entityResolver(this) {
                Ok(ok__) => {
                    core::ptr::write(oresolver, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_entityResolver<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oresolver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXXMLReader_Impl::putref_entityResolver(this, windows_core::from_raw_borrowed(&oresolver)).into()
        }
        unsafe extern "system" fn contentHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXXMLReader_Impl::contentHandler(this) {
                Ok(ok__) => {
                    core::ptr::write(ohandler, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_contentHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXXMLReader_Impl::putref_contentHandler(this, windows_core::from_raw_borrowed(&ohandler)).into()
        }
        unsafe extern "system" fn dtdHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXXMLReader_Impl::dtdHandler(this) {
                Ok(ok__) => {
                    core::ptr::write(ohandler, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_dtdHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXXMLReader_Impl::putref_dtdHandler(this, windows_core::from_raw_borrowed(&ohandler)).into()
        }
        unsafe extern "system" fn errorHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXXMLReader_Impl::errorHandler(this) {
                Ok(ok__) => {
                    core::ptr::write(ohandler, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_errorHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXXMLReader_Impl::putref_errorHandler(this, windows_core::from_raw_borrowed(&ohandler)).into()
        }
        unsafe extern "system" fn baseURL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbaseurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXXMLReader_Impl::baseURL(this) {
                Ok(ok__) => {
                    core::ptr::write(strbaseurl, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetbaseURL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbaseurl: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXXMLReader_Impl::SetbaseURL(this, core::mem::transmute(&strbaseurl)).into()
        }
        unsafe extern "system" fn secureBaseURL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsecurebaseurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IVBSAXXMLReader_Impl::secureBaseURL(this) {
                Ok(ok__) => {
                    core::ptr::write(strsecurebaseurl, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetsecureBaseURL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsecurebaseurl: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXXMLReader_Impl::SetsecureBaseURL(this, core::mem::transmute(&strsecurebaseurl)).into()
        }
        unsafe extern "system" fn parse<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varinput: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXXMLReader_Impl::parse(this, core::mem::transmute(&varinput)).into()
        }
        unsafe extern "system" fn parseURL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strurl: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IVBSAXXMLReader_Impl::parseURL(this, core::mem::transmute(&strurl)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            getFeature: getFeature::<Identity, Impl, OFFSET>,
            putFeature: putFeature::<Identity, Impl, OFFSET>,
            getProperty: getProperty::<Identity, Impl, OFFSET>,
            putProperty: putProperty::<Identity, Impl, OFFSET>,
            entityResolver: entityResolver::<Identity, Impl, OFFSET>,
            putref_entityResolver: putref_entityResolver::<Identity, Impl, OFFSET>,
            contentHandler: contentHandler::<Identity, Impl, OFFSET>,
            putref_contentHandler: putref_contentHandler::<Identity, Impl, OFFSET>,
            dtdHandler: dtdHandler::<Identity, Impl, OFFSET>,
            putref_dtdHandler: putref_dtdHandler::<Identity, Impl, OFFSET>,
            errorHandler: errorHandler::<Identity, Impl, OFFSET>,
            putref_errorHandler: putref_errorHandler::<Identity, Impl, OFFSET>,
            baseURL: baseURL::<Identity, Impl, OFFSET>,
            SetbaseURL: SetbaseURL::<Identity, Impl, OFFSET>,
            secureBaseURL: secureBaseURL::<Identity, Impl, OFFSET>,
            SetsecureBaseURL: SetsecureBaseURL::<Identity, Impl, OFFSET>,
            parse: parse::<Identity, Impl, OFFSET>,
            parseURL: parseURL::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXXMLReader as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLAttribute_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn value(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLAttribute {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLAttribute_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLAttribute_Impl, const OFFSET: isize>() -> IXMLAttribute_Vtbl {
        unsafe extern "system" fn name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, n: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLAttribute_Impl::name(this) {
                Ok(ok__) => {
                    core::ptr::write(n, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn value<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLAttribute_Impl::value(this) {
                Ok(ok__) => {
                    core::ptr::write(v, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            name: name::<Identity, Impl, OFFSET>,
            value: value::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLAttribute as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMAttribute_Impl: Sized + IXMLDOMNode_Impl {
    fn name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn value(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Setvalue(&self, attributevalue: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMAttribute {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMAttribute_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>() -> IXMLDOMAttribute_Vtbl {
        unsafe extern "system" fn name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMAttribute_Impl::name(this) {
                Ok(ok__) => {
                    core::ptr::write(attributename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn value<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributevalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMAttribute_Impl::value(this) {
                Ok(ok__) => {
                    core::ptr::write(attributevalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setvalue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributevalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMAttribute_Impl::Setvalue(this, core::mem::transmute(&attributevalue)).into()
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            name: name::<Identity, Impl, OFFSET>,
            value: value::<Identity, Impl, OFFSET>,
            Setvalue: Setvalue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMAttribute as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMCDATASection_Impl: Sized + IXMLDOMText_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMCDATASection {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMCDATASection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCDATASection_Impl, const OFFSET: isize>() -> IXMLDOMCDATASection_Vtbl {
        Self { base__: IXMLDOMText_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMCDATASection as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID || iid == &<IXMLDOMCharacterData as windows_core::Interface>::IID || iid == &<IXMLDOMText as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMCharacterData_Impl: Sized + IXMLDOMNode_Impl {
    fn data(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Setdata(&self, data: &windows_core::BSTR) -> windows_core::Result<()>;
    fn length(&self) -> windows_core::Result<i32>;
    fn substringData(&self, offset: i32, count: i32) -> windows_core::Result<windows_core::BSTR>;
    fn appendData(&self, data: &windows_core::BSTR) -> windows_core::Result<()>;
    fn insertData(&self, offset: i32, data: &windows_core::BSTR) -> windows_core::Result<()>;
    fn deleteData(&self, offset: i32, count: i32) -> windows_core::Result<()>;
    fn replaceData(&self, offset: i32, count: i32, data: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMCharacterData {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMCharacterData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>() -> IXMLDOMCharacterData_Vtbl {
        unsafe extern "system" fn data<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMCharacterData_Impl::data(this) {
                Ok(ok__) => {
                    core::ptr::write(data, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setdata<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMCharacterData_Impl::Setdata(this, core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn length<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datalength: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMCharacterData_Impl::length(this) {
                Ok(ok__) => {
                    core::ptr::write(datalength, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn substringData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: i32, count: i32, data: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMCharacterData_Impl::substringData(this, core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    core::ptr::write(data, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn appendData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMCharacterData_Impl::appendData(this, core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn insertData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: i32, data: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMCharacterData_Impl::insertData(this, core::mem::transmute_copy(&offset), core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn deleteData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: i32, count: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMCharacterData_Impl::deleteData(this, core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn replaceData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: i32, count: i32, data: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMCharacterData_Impl::replaceData(this, core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count), core::mem::transmute(&data)).into()
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            data: data::<Identity, Impl, OFFSET>,
            Setdata: Setdata::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            substringData: substringData::<Identity, Impl, OFFSET>,
            appendData: appendData::<Identity, Impl, OFFSET>,
            insertData: insertData::<Identity, Impl, OFFSET>,
            deleteData: deleteData::<Identity, Impl, OFFSET>,
            replaceData: replaceData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMCharacterData as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMComment_Impl: Sized + IXMLDOMCharacterData_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMComment {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMComment_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMComment_Impl, const OFFSET: isize>() -> IXMLDOMComment_Vtbl {
        Self { base__: IXMLDOMCharacterData_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMComment as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID || iid == &<IXMLDOMCharacterData as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMDocument_Impl: Sized + IXMLDOMNode_Impl {
    fn doctype(&self) -> windows_core::Result<IXMLDOMDocumentType>;
    fn implementation(&self) -> windows_core::Result<IXMLDOMImplementation>;
    fn documentElement(&self) -> windows_core::Result<IXMLDOMElement>;
    fn putref_documentElement(&self, domelement: Option<&IXMLDOMElement>) -> windows_core::Result<()>;
    fn createElement(&self, tagname: &windows_core::BSTR) -> windows_core::Result<IXMLDOMElement>;
    fn createDocumentFragment(&self) -> windows_core::Result<IXMLDOMDocumentFragment>;
    fn createTextNode(&self, data: &windows_core::BSTR) -> windows_core::Result<IXMLDOMText>;
    fn createComment(&self, data: &windows_core::BSTR) -> windows_core::Result<IXMLDOMComment>;
    fn createCDATASection(&self, data: &windows_core::BSTR) -> windows_core::Result<IXMLDOMCDATASection>;
    fn createProcessingInstruction(&self, target: &windows_core::BSTR, data: &windows_core::BSTR) -> windows_core::Result<IXMLDOMProcessingInstruction>;
    fn createAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<IXMLDOMAttribute>;
    fn createEntityReference(&self, name: &windows_core::BSTR) -> windows_core::Result<IXMLDOMEntityReference>;
    fn getElementsByTagName(&self, tagname: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNodeList>;
    fn createNode(&self, r#type: &windows_core::VARIANT, name: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode>;
    fn nodeFromID(&self, idstring: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode>;
    fn load(&self, xmlsource: &windows_core::VARIANT) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn readyState(&self) -> windows_core::Result<i32>;
    fn parseError(&self) -> windows_core::Result<IXMLDOMParseError>;
    fn url(&self) -> windows_core::Result<windows_core::BSTR>;
    fn r#async(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setasync(&self, isasync: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn abort(&self) -> windows_core::Result<()>;
    fn loadXML(&self, bstrxml: &windows_core::BSTR) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn save(&self, destination: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn validateOnParse(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetvalidateOnParse(&self, isvalidating: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn resolveExternals(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetresolveExternals(&self, isresolving: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn preserveWhiteSpace(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetpreserveWhiteSpace(&self, ispreserving: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Setonreadystatechange(&self, readystatechangesink: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Setondataavailable(&self, ondataavailablesink: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Setontransformnode(&self, ontransformnodesink: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMDocument {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMDocument_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>() -> IXMLDOMDocument_Vtbl {
        unsafe extern "system" fn doctype<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documenttype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::doctype(this) {
                Ok(ok__) => {
                    core::ptr::write(documenttype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn implementation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#impl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::implementation(this) {
                Ok(ok__) => {
                    core::ptr::write(r#impl, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn documentElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domelement: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::documentElement(this) {
                Ok(ok__) => {
                    core::ptr::write(domelement, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_documentElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domelement: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMDocument_Impl::putref_documentElement(this, windows_core::from_raw_borrowed(&domelement)).into()
        }
        unsafe extern "system" fn createElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tagname: core::mem::MaybeUninit<windows_core::BSTR>, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::createElement(this, core::mem::transmute(&tagname)) {
                Ok(ok__) => {
                    core::ptr::write(element, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createDocumentFragment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, docfrag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::createDocumentFragment(this) {
                Ok(ok__) => {
                    core::ptr::write(docfrag, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createTextNode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::BSTR>, text: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::createTextNode(this, core::mem::transmute(&data)) {
                Ok(ok__) => {
                    core::ptr::write(text, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createComment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::BSTR>, comment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::createComment(this, core::mem::transmute(&data)) {
                Ok(ok__) => {
                    core::ptr::write(comment, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createCDATASection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::BSTR>, cdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::createCDATASection(this, core::mem::transmute(&data)) {
                Ok(ok__) => {
                    core::ptr::write(cdata, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createProcessingInstruction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: core::mem::MaybeUninit<windows_core::BSTR>, data: core::mem::MaybeUninit<windows_core::BSTR>, pi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::createProcessingInstruction(this, core::mem::transmute(&target), core::mem::transmute(&data)) {
                Ok(ok__) => {
                    core::ptr::write(pi, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, attribute: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::createAttribute(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(attribute, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createEntityReference<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, entityref: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::createEntityReference(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(entityref, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getElementsByTagName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tagname: core::mem::MaybeUninit<windows_core::BSTR>, resultlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::getElementsByTagName(this, core::mem::transmute(&tagname)) {
                Ok(ok__) => {
                    core::ptr::write(resultlist, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createNode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: core::mem::MaybeUninit<windows_core::VARIANT>, name: core::mem::MaybeUninit<windows_core::BSTR>, namespaceuri: core::mem::MaybeUninit<windows_core::BSTR>, node: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::createNode(this, core::mem::transmute(&r#type), core::mem::transmute(&name), core::mem::transmute(&namespaceuri)) {
                Ok(ok__) => {
                    core::ptr::write(node, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeFromID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idstring: core::mem::MaybeUninit<windows_core::BSTR>, node: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::nodeFromID(this, core::mem::transmute(&idstring)) {
                Ok(ok__) => {
                    core::ptr::write(node, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn load<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xmlsource: core::mem::MaybeUninit<windows_core::VARIANT>, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::load(this, core::mem::transmute(&xmlsource)) {
                Ok(ok__) => {
                    core::ptr::write(issuccessful, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::readyState(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parseError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::parseError(this) {
                Ok(ok__) => {
                    core::ptr::write(errorobj, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn url<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, urlstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::url(this) {
                Ok(ok__) => {
                    core::ptr::write(urlstring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#async<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isasync: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::r#async(this) {
                Ok(ok__) => {
                    core::ptr::write(isasync, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setasync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isasync: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMDocument_Impl::Setasync(this, core::mem::transmute_copy(&isasync)).into()
        }
        unsafe extern "system" fn abort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMDocument_Impl::abort(this).into()
        }
        unsafe extern "system" fn loadXML<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: core::mem::MaybeUninit<windows_core::BSTR>, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::loadXML(this, core::mem::transmute(&bstrxml)) {
                Ok(ok__) => {
                    core::ptr::write(issuccessful, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destination: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMDocument_Impl::save(this, core::mem::transmute(&destination)).into()
        }
        unsafe extern "system" fn validateOnParse<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isvalidating: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::validateOnParse(this) {
                Ok(ok__) => {
                    core::ptr::write(isvalidating, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetvalidateOnParse<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isvalidating: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMDocument_Impl::SetvalidateOnParse(this, core::mem::transmute_copy(&isvalidating)).into()
        }
        unsafe extern "system" fn resolveExternals<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isresolving: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::resolveExternals(this) {
                Ok(ok__) => {
                    core::ptr::write(isresolving, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetresolveExternals<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isresolving: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMDocument_Impl::SetresolveExternals(this, core::mem::transmute_copy(&isresolving)).into()
        }
        unsafe extern "system" fn preserveWhiteSpace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ispreserving: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument_Impl::preserveWhiteSpace(this) {
                Ok(ok__) => {
                    core::ptr::write(ispreserving, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetpreserveWhiteSpace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ispreserving: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMDocument_Impl::SetpreserveWhiteSpace(this, core::mem::transmute_copy(&ispreserving)).into()
        }
        unsafe extern "system" fn Setonreadystatechange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, readystatechangesink: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMDocument_Impl::Setonreadystatechange(this, core::mem::transmute(&readystatechangesink)).into()
        }
        unsafe extern "system" fn Setondataavailable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ondataavailablesink: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMDocument_Impl::Setondataavailable(this, core::mem::transmute(&ondataavailablesink)).into()
        }
        unsafe extern "system" fn Setontransformnode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ontransformnodesink: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMDocument_Impl::Setontransformnode(this, core::mem::transmute(&ontransformnodesink)).into()
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            doctype: doctype::<Identity, Impl, OFFSET>,
            implementation: implementation::<Identity, Impl, OFFSET>,
            documentElement: documentElement::<Identity, Impl, OFFSET>,
            putref_documentElement: putref_documentElement::<Identity, Impl, OFFSET>,
            createElement: createElement::<Identity, Impl, OFFSET>,
            createDocumentFragment: createDocumentFragment::<Identity, Impl, OFFSET>,
            createTextNode: createTextNode::<Identity, Impl, OFFSET>,
            createComment: createComment::<Identity, Impl, OFFSET>,
            createCDATASection: createCDATASection::<Identity, Impl, OFFSET>,
            createProcessingInstruction: createProcessingInstruction::<Identity, Impl, OFFSET>,
            createAttribute: createAttribute::<Identity, Impl, OFFSET>,
            createEntityReference: createEntityReference::<Identity, Impl, OFFSET>,
            getElementsByTagName: getElementsByTagName::<Identity, Impl, OFFSET>,
            createNode: createNode::<Identity, Impl, OFFSET>,
            nodeFromID: nodeFromID::<Identity, Impl, OFFSET>,
            load: load::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
            parseError: parseError::<Identity, Impl, OFFSET>,
            url: url::<Identity, Impl, OFFSET>,
            r#async: r#async::<Identity, Impl, OFFSET>,
            Setasync: Setasync::<Identity, Impl, OFFSET>,
            abort: abort::<Identity, Impl, OFFSET>,
            loadXML: loadXML::<Identity, Impl, OFFSET>,
            save: save::<Identity, Impl, OFFSET>,
            validateOnParse: validateOnParse::<Identity, Impl, OFFSET>,
            SetvalidateOnParse: SetvalidateOnParse::<Identity, Impl, OFFSET>,
            resolveExternals: resolveExternals::<Identity, Impl, OFFSET>,
            SetresolveExternals: SetresolveExternals::<Identity, Impl, OFFSET>,
            preserveWhiteSpace: preserveWhiteSpace::<Identity, Impl, OFFSET>,
            SetpreserveWhiteSpace: SetpreserveWhiteSpace::<Identity, Impl, OFFSET>,
            Setonreadystatechange: Setonreadystatechange::<Identity, Impl, OFFSET>,
            Setondataavailable: Setondataavailable::<Identity, Impl, OFFSET>,
            Setontransformnode: Setontransformnode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMDocument as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMDocument2_Impl: Sized + IXMLDOMDocument_Impl {
    fn namespaces(&self) -> windows_core::Result<IXMLDOMSchemaCollection>;
    fn schemas(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn putref_schemas(&self, othercollection: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn validate(&self) -> windows_core::Result<IXMLDOMParseError>;
    fn setProperty(&self, name: &windows_core::BSTR, value: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn getProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMDocument2 {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMDocument2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>() -> IXMLDOMDocument2_Vtbl {
        unsafe extern "system" fn namespaces<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespacecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument2_Impl::namespaces(this) {
                Ok(ok__) => {
                    core::ptr::write(namespacecollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schemas<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, othercollection: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument2_Impl::schemas(this) {
                Ok(ok__) => {
                    core::ptr::write(othercollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_schemas<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, othercollection: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMDocument2_Impl::putref_schemas(this, core::mem::transmute(&othercollection)).into()
        }
        unsafe extern "system" fn validate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument2_Impl::validate(this) {
                Ok(ok__) => {
                    core::ptr::write(errorobj, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, value: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMDocument2_Impl::setProperty(this, core::mem::transmute(&name), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn getProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument2_Impl::getProperty(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXMLDOMDocument_Vtbl::new::<Identity, Impl, OFFSET>(),
            namespaces: namespaces::<Identity, Impl, OFFSET>,
            schemas: schemas::<Identity, Impl, OFFSET>,
            putref_schemas: putref_schemas::<Identity, Impl, OFFSET>,
            validate: validate::<Identity, Impl, OFFSET>,
            setProperty: setProperty::<Identity, Impl, OFFSET>,
            getProperty: getProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMDocument2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID || iid == &<IXMLDOMDocument as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMDocument3_Impl: Sized + IXMLDOMDocument2_Impl {
    fn validateNode(&self, node: Option<&IXMLDOMNode>) -> windows_core::Result<IXMLDOMParseError>;
    fn importNode(&self, node: Option<&IXMLDOMNode>, deep: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IXMLDOMNode>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMDocument3 {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMDocument3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument3_Impl, const OFFSET: isize>() -> IXMLDOMDocument3_Vtbl {
        unsafe extern "system" fn validateNode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void, errorobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument3_Impl::validateNode(this, windows_core::from_raw_borrowed(&node)) {
                Ok(ok__) => {
                    core::ptr::write(errorobj, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn importNode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void, deep: super::super::super::Foundation::VARIANT_BOOL, clone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocument3_Impl::importNode(this, windows_core::from_raw_borrowed(&node), core::mem::transmute_copy(&deep)) {
                Ok(ok__) => {
                    core::ptr::write(clone, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXMLDOMDocument2_Vtbl::new::<Identity, Impl, OFFSET>(),
            validateNode: validateNode::<Identity, Impl, OFFSET>,
            importNode: importNode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMDocument3 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID || iid == &<IXMLDOMDocument as windows_core::Interface>::IID || iid == &<IXMLDOMDocument2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMDocumentFragment_Impl: Sized + IXMLDOMNode_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMDocumentFragment {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMDocumentFragment_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocumentFragment_Impl, const OFFSET: isize>() -> IXMLDOMDocumentFragment_Vtbl {
        Self { base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMDocumentFragment as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMDocumentType_Impl: Sized + IXMLDOMNode_Impl {
    fn name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn entities(&self) -> windows_core::Result<IXMLDOMNamedNodeMap>;
    fn notations(&self) -> windows_core::Result<IXMLDOMNamedNodeMap>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMDocumentType {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMDocumentType_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>() -> IXMLDOMDocumentType_Vtbl {
        unsafe extern "system" fn name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rootname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocumentType_Impl::name(this) {
                Ok(ok__) => {
                    core::ptr::write(rootname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn entities<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, entitymap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocumentType_Impl::entities(this) {
                Ok(ok__) => {
                    core::ptr::write(entitymap, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notations<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notationmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMDocumentType_Impl::notations(this) {
                Ok(ok__) => {
                    core::ptr::write(notationmap, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            name: name::<Identity, Impl, OFFSET>,
            entities: entities::<Identity, Impl, OFFSET>,
            notations: notations::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMDocumentType as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMElement_Impl: Sized + IXMLDOMNode_Impl {
    fn tagName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn getAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn setAttribute(&self, name: &windows_core::BSTR, value: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn removeAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn getAttributeNode(&self, name: &windows_core::BSTR) -> windows_core::Result<IXMLDOMAttribute>;
    fn setAttributeNode(&self, domattribute: Option<&IXMLDOMAttribute>) -> windows_core::Result<IXMLDOMAttribute>;
    fn removeAttributeNode(&self, domattribute: Option<&IXMLDOMAttribute>) -> windows_core::Result<IXMLDOMAttribute>;
    fn getElementsByTagName(&self, tagname: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNodeList>;
    fn normalize(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMElement {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMElement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>() -> IXMLDOMElement_Vtbl {
        unsafe extern "system" fn tagName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tagname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMElement_Impl::tagName(this) {
                Ok(ok__) => {
                    core::ptr::write(tagname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMElement_Impl::getAttribute(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, value: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMElement_Impl::setAttribute(this, core::mem::transmute(&name), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn removeAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMElement_Impl::removeAttribute(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn getAttributeNode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, attributenode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMElement_Impl::getAttributeNode(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(attributenode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttributeNode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domattribute: *mut core::ffi::c_void, attributenode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMElement_Impl::setAttributeNode(this, windows_core::from_raw_borrowed(&domattribute)) {
                Ok(ok__) => {
                    core::ptr::write(attributenode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttributeNode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domattribute: *mut core::ffi::c_void, attributenode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMElement_Impl::removeAttributeNode(this, windows_core::from_raw_borrowed(&domattribute)) {
                Ok(ok__) => {
                    core::ptr::write(attributenode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getElementsByTagName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tagname: core::mem::MaybeUninit<windows_core::BSTR>, resultlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMElement_Impl::getElementsByTagName(this, core::mem::transmute(&tagname)) {
                Ok(ok__) => {
                    core::ptr::write(resultlist, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn normalize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMElement_Impl::normalize(this).into()
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            tagName: tagName::<Identity, Impl, OFFSET>,
            getAttribute: getAttribute::<Identity, Impl, OFFSET>,
            setAttribute: setAttribute::<Identity, Impl, OFFSET>,
            removeAttribute: removeAttribute::<Identity, Impl, OFFSET>,
            getAttributeNode: getAttributeNode::<Identity, Impl, OFFSET>,
            setAttributeNode: setAttributeNode::<Identity, Impl, OFFSET>,
            removeAttributeNode: removeAttributeNode::<Identity, Impl, OFFSET>,
            getElementsByTagName: getElementsByTagName::<Identity, Impl, OFFSET>,
            normalize: normalize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMElement as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMEntity_Impl: Sized + IXMLDOMNode_Impl {
    fn publicId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn systemId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn notationName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMEntity {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMEntity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMEntity_Impl, const OFFSET: isize>() -> IXMLDOMEntity_Vtbl {
        unsafe extern "system" fn publicId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, publicid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMEntity_Impl::publicId(this) {
                Ok(ok__) => {
                    core::ptr::write(publicid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn systemId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, systemid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMEntity_Impl::systemId(this) {
                Ok(ok__) => {
                    core::ptr::write(systemid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notationName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMEntity_Impl::notationName(this) {
                Ok(ok__) => {
                    core::ptr::write(name, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            publicId: publicId::<Identity, Impl, OFFSET>,
            systemId: systemId::<Identity, Impl, OFFSET>,
            notationName: notationName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMEntity as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMEntityReference_Impl: Sized + IXMLDOMNode_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMEntityReference {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMEntityReference_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMEntityReference_Impl, const OFFSET: isize>() -> IXMLDOMEntityReference_Vtbl {
        Self { base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMEntityReference as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMImplementation_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn hasFeature(&self, feature: &windows_core::BSTR, version: &windows_core::BSTR) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMImplementation {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMImplementation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMImplementation_Impl, const OFFSET: isize>() -> IXMLDOMImplementation_Vtbl {
        unsafe extern "system" fn hasFeature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: core::mem::MaybeUninit<windows_core::BSTR>, version: core::mem::MaybeUninit<windows_core::BSTR>, hasfeature: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMImplementation_Impl::hasFeature(this, core::mem::transmute(&feature), core::mem::transmute(&version)) {
                Ok(ok__) => {
                    core::ptr::write(hasfeature, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), hasFeature: hasFeature::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMImplementation as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMNamedNodeMap_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn getNamedItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode>;
    fn setNamedItem(&self, newitem: Option<&IXMLDOMNode>) -> windows_core::Result<IXMLDOMNode>;
    fn removeNamedItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode>;
    fn get_item(&self, index: i32) -> windows_core::Result<IXMLDOMNode>;
    fn length(&self) -> windows_core::Result<i32>;
    fn getQualifiedItem(&self, basename: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode>;
    fn removeQualifiedItem(&self, basename: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode>;
    fn nextNode(&self) -> windows_core::Result<IXMLDOMNode>;
    fn reset(&self) -> windows_core::Result<()>;
    fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMNamedNodeMap {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMNamedNodeMap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>() -> IXMLDOMNamedNodeMap_Vtbl {
        unsafe extern "system" fn getNamedItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, nameditem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNamedNodeMap_Impl::getNamedItem(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(nameditem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setNamedItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newitem: *mut core::ffi::c_void, nameitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNamedNodeMap_Impl::setNamedItem(this, windows_core::from_raw_borrowed(&newitem)) {
                Ok(ok__) => {
                    core::ptr::write(nameitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeNamedItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, nameditem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNamedNodeMap_Impl::removeNamedItem(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(nameditem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, listitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNamedNodeMap_Impl::get_item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(listitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listlength: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNamedNodeMap_Impl::length(this) {
                Ok(ok__) => {
                    core::ptr::write(listlength, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getQualifiedItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, basename: core::mem::MaybeUninit<windows_core::BSTR>, namespaceuri: core::mem::MaybeUninit<windows_core::BSTR>, qualifieditem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNamedNodeMap_Impl::getQualifiedItem(this, core::mem::transmute(&basename), core::mem::transmute(&namespaceuri)) {
                Ok(ok__) => {
                    core::ptr::write(qualifieditem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeQualifiedItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, basename: core::mem::MaybeUninit<windows_core::BSTR>, namespaceuri: core::mem::MaybeUninit<windows_core::BSTR>, qualifieditem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNamedNodeMap_Impl::removeQualifiedItem(this, core::mem::transmute(&basename), core::mem::transmute(&namespaceuri)) {
                Ok(ok__) => {
                    core::ptr::write(qualifieditem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nextNode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nextitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNamedNodeMap_Impl::nextNode(this) {
                Ok(ok__) => {
                    core::ptr::write(nextitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMNamedNodeMap_Impl::reset(this).into()
        }
        unsafe extern "system" fn _newEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNamedNodeMap_Impl::_newEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            getNamedItem: getNamedItem::<Identity, Impl, OFFSET>,
            setNamedItem: setNamedItem::<Identity, Impl, OFFSET>,
            removeNamedItem: removeNamedItem::<Identity, Impl, OFFSET>,
            get_item: get_item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            getQualifiedItem: getQualifiedItem::<Identity, Impl, OFFSET>,
            removeQualifiedItem: removeQualifiedItem::<Identity, Impl, OFFSET>,
            nextNode: nextNode::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMNamedNodeMap as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMNode_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn nodeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn nodeValue(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetnodeValue(&self, value: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn nodeType(&self) -> windows_core::Result<DOMNodeType>;
    fn parentNode(&self) -> windows_core::Result<IXMLDOMNode>;
    fn childNodes(&self) -> windows_core::Result<IXMLDOMNodeList>;
    fn firstChild(&self) -> windows_core::Result<IXMLDOMNode>;
    fn lastChild(&self) -> windows_core::Result<IXMLDOMNode>;
    fn previousSibling(&self) -> windows_core::Result<IXMLDOMNode>;
    fn nextSibling(&self) -> windows_core::Result<IXMLDOMNode>;
    fn attributes(&self) -> windows_core::Result<IXMLDOMNamedNodeMap>;
    fn insertBefore(&self, newchild: Option<&IXMLDOMNode>, refchild: &windows_core::VARIANT) -> windows_core::Result<IXMLDOMNode>;
    fn replaceChild(&self, newchild: Option<&IXMLDOMNode>, oldchild: Option<&IXMLDOMNode>) -> windows_core::Result<IXMLDOMNode>;
    fn removeChild(&self, childnode: Option<&IXMLDOMNode>) -> windows_core::Result<IXMLDOMNode>;
    fn appendChild(&self, newchild: Option<&IXMLDOMNode>) -> windows_core::Result<IXMLDOMNode>;
    fn hasChildNodes(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn ownerDocument(&self) -> windows_core::Result<IXMLDOMDocument>;
    fn cloneNode(&self, deep: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IXMLDOMNode>;
    fn nodeTypeString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn text(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Settext(&self, text: &windows_core::BSTR) -> windows_core::Result<()>;
    fn specified(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn definition(&self) -> windows_core::Result<IXMLDOMNode>;
    fn nodeTypedValue(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetnodeTypedValue(&self, typedvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn dataType(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetdataType(&self, datatypename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn xml(&self) -> windows_core::Result<windows_core::BSTR>;
    fn transformNode(&self, stylesheet: Option<&IXMLDOMNode>) -> windows_core::Result<windows_core::BSTR>;
    fn selectNodes(&self, querystring: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNodeList>;
    fn selectSingleNode(&self, querystring: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode>;
    fn parsed(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn namespaceURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn prefix(&self) -> windows_core::Result<windows_core::BSTR>;
    fn baseName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn transformNodeToObject(&self, stylesheet: Option<&IXMLDOMNode>, outputobject: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMNode {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMNode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>() -> IXMLDOMNode_Vtbl {
        unsafe extern "system" fn nodeName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::nodeName(this) {
                Ok(ok__) => {
                    core::ptr::write(name, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::nodeValue(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetnodeValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMNode_Impl::SetnodeValue(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn nodeType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut DOMNodeType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::nodeType(this) {
                Ok(ok__) => {
                    core::ptr::write(r#type, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parentNode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::parentNode(this) {
                Ok(ok__) => {
                    core::ptr::write(parent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn childNodes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, childlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::childNodes(this) {
                Ok(ok__) => {
                    core::ptr::write(childlist, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn firstChild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, firstchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::firstChild(this) {
                Ok(ok__) => {
                    core::ptr::write(firstchild, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lastChild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::lastChild(this) {
                Ok(ok__) => {
                    core::ptr::write(lastchild, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn previousSibling<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previoussibling: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::previousSibling(this) {
                Ok(ok__) => {
                    core::ptr::write(previoussibling, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nextSibling<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nextsibling: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::nextSibling(this) {
                Ok(ok__) => {
                    core::ptr::write(nextsibling, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributemap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::attributes(this) {
                Ok(ok__) => {
                    core::ptr::write(attributemap, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn insertBefore<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newchild: *mut core::ffi::c_void, refchild: core::mem::MaybeUninit<windows_core::VARIANT>, outnewchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::insertBefore(this, windows_core::from_raw_borrowed(&newchild), core::mem::transmute(&refchild)) {
                Ok(ok__) => {
                    core::ptr::write(outnewchild, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn replaceChild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newchild: *mut core::ffi::c_void, oldchild: *mut core::ffi::c_void, outoldchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::replaceChild(this, windows_core::from_raw_borrowed(&newchild), windows_core::from_raw_borrowed(&oldchild)) {
                Ok(ok__) => {
                    core::ptr::write(outoldchild, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeChild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, childnode: *mut core::ffi::c_void, oldchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::removeChild(this, windows_core::from_raw_borrowed(&childnode)) {
                Ok(ok__) => {
                    core::ptr::write(oldchild, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn appendChild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newchild: *mut core::ffi::c_void, outnewchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::appendChild(this, windows_core::from_raw_borrowed(&newchild)) {
                Ok(ok__) => {
                    core::ptr::write(outnewchild, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasChildNodes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::hasChildNodes(this) {
                Ok(ok__) => {
                    core::ptr::write(haschild, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ownerDocument<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xmldomdocument: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::ownerDocument(this) {
                Ok(ok__) => {
                    core::ptr::write(xmldomdocument, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn cloneNode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deep: super::super::super::Foundation::VARIANT_BOOL, cloneroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::cloneNode(this, core::mem::transmute_copy(&deep)) {
                Ok(ok__) => {
                    core::ptr::write(cloneroot, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeTypeString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodetype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::nodeTypeString(this) {
                Ok(ok__) => {
                    core::ptr::write(nodetype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn text<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::text(this) {
                Ok(ok__) => {
                    core::ptr::write(text, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMNode_Impl::Settext(this, core::mem::transmute(&text)).into()
        }
        unsafe extern "system" fn specified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::specified(this) {
                Ok(ok__) => {
                    core::ptr::write(isspecified, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn definition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, definitionnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::definition(this) {
                Ok(ok__) => {
                    core::ptr::write(definitionnode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeTypedValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, typedvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::nodeTypedValue(this) {
                Ok(ok__) => {
                    core::ptr::write(typedvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetnodeTypedValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, typedvalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMNode_Impl::SetnodeTypedValue(this, core::mem::transmute(&typedvalue)).into()
        }
        unsafe extern "system" fn dataType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datatypename: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::dataType(this) {
                Ok(ok__) => {
                    core::ptr::write(datatypename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetdataType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datatypename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMNode_Impl::SetdataType(this, core::mem::transmute(&datatypename)).into()
        }
        unsafe extern "system" fn xml<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xmlstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::xml(this) {
                Ok(ok__) => {
                    core::ptr::write(xmlstring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn transformNode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stylesheet: *mut core::ffi::c_void, xmlstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::transformNode(this, windows_core::from_raw_borrowed(&stylesheet)) {
                Ok(ok__) => {
                    core::ptr::write(xmlstring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn selectNodes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, querystring: core::mem::MaybeUninit<windows_core::BSTR>, resultlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::selectNodes(this, core::mem::transmute(&querystring)) {
                Ok(ok__) => {
                    core::ptr::write(resultlist, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn selectSingleNode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, querystring: core::mem::MaybeUninit<windows_core::BSTR>, resultnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::selectSingleNode(this, core::mem::transmute(&querystring)) {
                Ok(ok__) => {
                    core::ptr::write(resultnode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parsed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::parsed(this) {
                Ok(ok__) => {
                    core::ptr::write(isparsed, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn namespaceURI<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceuri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::namespaceURI(this) {
                Ok(ok__) => {
                    core::ptr::write(namespaceuri, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn prefix<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefixstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::prefix(this) {
                Ok(ok__) => {
                    core::ptr::write(prefixstring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn baseName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namestring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNode_Impl::baseName(this) {
                Ok(ok__) => {
                    core::ptr::write(namestring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn transformNodeToObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stylesheet: *mut core::ffi::c_void, outputobject: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMNode_Impl::transformNodeToObject(this, windows_core::from_raw_borrowed(&stylesheet), core::mem::transmute(&outputobject)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            nodeName: nodeName::<Identity, Impl, OFFSET>,
            nodeValue: nodeValue::<Identity, Impl, OFFSET>,
            SetnodeValue: SetnodeValue::<Identity, Impl, OFFSET>,
            nodeType: nodeType::<Identity, Impl, OFFSET>,
            parentNode: parentNode::<Identity, Impl, OFFSET>,
            childNodes: childNodes::<Identity, Impl, OFFSET>,
            firstChild: firstChild::<Identity, Impl, OFFSET>,
            lastChild: lastChild::<Identity, Impl, OFFSET>,
            previousSibling: previousSibling::<Identity, Impl, OFFSET>,
            nextSibling: nextSibling::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
            insertBefore: insertBefore::<Identity, Impl, OFFSET>,
            replaceChild: replaceChild::<Identity, Impl, OFFSET>,
            removeChild: removeChild::<Identity, Impl, OFFSET>,
            appendChild: appendChild::<Identity, Impl, OFFSET>,
            hasChildNodes: hasChildNodes::<Identity, Impl, OFFSET>,
            ownerDocument: ownerDocument::<Identity, Impl, OFFSET>,
            cloneNode: cloneNode::<Identity, Impl, OFFSET>,
            nodeTypeString: nodeTypeString::<Identity, Impl, OFFSET>,
            text: text::<Identity, Impl, OFFSET>,
            Settext: Settext::<Identity, Impl, OFFSET>,
            specified: specified::<Identity, Impl, OFFSET>,
            definition: definition::<Identity, Impl, OFFSET>,
            nodeTypedValue: nodeTypedValue::<Identity, Impl, OFFSET>,
            SetnodeTypedValue: SetnodeTypedValue::<Identity, Impl, OFFSET>,
            dataType: dataType::<Identity, Impl, OFFSET>,
            SetdataType: SetdataType::<Identity, Impl, OFFSET>,
            xml: xml::<Identity, Impl, OFFSET>,
            transformNode: transformNode::<Identity, Impl, OFFSET>,
            selectNodes: selectNodes::<Identity, Impl, OFFSET>,
            selectSingleNode: selectSingleNode::<Identity, Impl, OFFSET>,
            parsed: parsed::<Identity, Impl, OFFSET>,
            namespaceURI: namespaceURI::<Identity, Impl, OFFSET>,
            prefix: prefix::<Identity, Impl, OFFSET>,
            baseName: baseName::<Identity, Impl, OFFSET>,
            transformNodeToObject: transformNodeToObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMNode as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMNodeList_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(&self, index: i32) -> windows_core::Result<IXMLDOMNode>;
    fn length(&self) -> windows_core::Result<i32>;
    fn nextNode(&self) -> windows_core::Result<IXMLDOMNode>;
    fn reset(&self) -> windows_core::Result<()>;
    fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMNodeList {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMNodeList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>() -> IXMLDOMNodeList_Vtbl {
        unsafe extern "system" fn get_item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, listitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNodeList_Impl::get_item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(listitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listlength: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNodeList_Impl::length(this) {
                Ok(ok__) => {
                    core::ptr::write(listlength, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nextNode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nextitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNodeList_Impl::nextNode(this) {
                Ok(ok__) => {
                    core::ptr::write(nextitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMNodeList_Impl::reset(this).into()
        }
        unsafe extern "system" fn _newEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNodeList_Impl::_newEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_item: get_item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            nextNode: nextNode::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMNodeList as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMNotation_Impl: Sized + IXMLDOMNode_Impl {
    fn publicId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn systemId(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMNotation {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMNotation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNotation_Impl, const OFFSET: isize>() -> IXMLDOMNotation_Vtbl {
        unsafe extern "system" fn publicId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNotation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, publicid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNotation_Impl::publicId(this) {
                Ok(ok__) => {
                    core::ptr::write(publicid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn systemId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNotation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, systemid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMNotation_Impl::systemId(this) {
                Ok(ok__) => {
                    core::ptr::write(systemid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            publicId: publicId::<Identity, Impl, OFFSET>,
            systemId: systemId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMNotation as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMParseError_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn errorCode(&self) -> windows_core::Result<i32>;
    fn url(&self) -> windows_core::Result<windows_core::BSTR>;
    fn reason(&self) -> windows_core::Result<windows_core::BSTR>;
    fn srcText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn line(&self) -> windows_core::Result<i32>;
    fn linepos(&self) -> windows_core::Result<i32>;
    fn filepos(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMParseError {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMParseError_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>() -> IXMLDOMParseError_Vtbl {
        unsafe extern "system" fn errorCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorcode: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMParseError_Impl::errorCode(this) {
                Ok(ok__) => {
                    core::ptr::write(errorcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn url<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, urlstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMParseError_Impl::url(this) {
                Ok(ok__) => {
                    core::ptr::write(urlstring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reason<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reasonstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMParseError_Impl::reason(this) {
                Ok(ok__) => {
                    core::ptr::write(reasonstring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn srcText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMParseError_Impl::srcText(this) {
                Ok(ok__) => {
                    core::ptr::write(sourcestring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn line<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linenumber: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMParseError_Impl::line(this) {
                Ok(ok__) => {
                    core::ptr::write(linenumber, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn linepos<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lineposition: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMParseError_Impl::linepos(this) {
                Ok(ok__) => {
                    core::ptr::write(lineposition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn filepos<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileposition: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMParseError_Impl::filepos(this) {
                Ok(ok__) => {
                    core::ptr::write(fileposition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            errorCode: errorCode::<Identity, Impl, OFFSET>,
            url: url::<Identity, Impl, OFFSET>,
            reason: reason::<Identity, Impl, OFFSET>,
            srcText: srcText::<Identity, Impl, OFFSET>,
            line: line::<Identity, Impl, OFFSET>,
            linepos: linepos::<Identity, Impl, OFFSET>,
            filepos: filepos::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMParseError as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMParseError2_Impl: Sized + IXMLDOMParseError_Impl {
    fn errorXPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn allErrors(&self) -> windows_core::Result<IXMLDOMParseErrorCollection>;
    fn errorParameters(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn errorParametersCount(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMParseError2 {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMParseError2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>() -> IXMLDOMParseError2_Vtbl {
        unsafe extern "system" fn errorXPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpathexpr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMParseError2_Impl::errorXPath(this) {
                Ok(ok__) => {
                    core::ptr::write(xpathexpr, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn allErrors<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allerrors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMParseError2_Impl::allErrors(this) {
                Ok(ok__) => {
                    core::ptr::write(allerrors, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, param1: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMParseError2_Impl::errorParameters(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(param1, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorParametersCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMParseError2_Impl::errorParametersCount(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXMLDOMParseError_Vtbl::new::<Identity, Impl, OFFSET>(),
            errorXPath: errorXPath::<Identity, Impl, OFFSET>,
            allErrors: allErrors::<Identity, Impl, OFFSET>,
            errorParameters: errorParameters::<Identity, Impl, OFFSET>,
            errorParametersCount: errorParametersCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMParseError2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMParseError as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMParseErrorCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(&self, index: i32) -> windows_core::Result<IXMLDOMParseError2>;
    fn length(&self) -> windows_core::Result<i32>;
    fn next(&self) -> windows_core::Result<IXMLDOMParseError2>;
    fn reset(&self) -> windows_core::Result<()>;
    fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMParseErrorCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMParseErrorCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>() -> IXMLDOMParseErrorCollection_Vtbl {
        unsafe extern "system" fn get_item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, error: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMParseErrorCollection_Impl::get_item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(error, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMParseErrorCollection_Impl::length(this) {
                Ok(ok__) => {
                    core::ptr::write(length, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMParseErrorCollection_Impl::next(this) {
                Ok(ok__) => {
                    core::ptr::write(error, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMParseErrorCollection_Impl::reset(this).into()
        }
        unsafe extern "system" fn _newEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMParseErrorCollection_Impl::_newEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_item: get_item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            next: next::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMParseErrorCollection as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMProcessingInstruction_Impl: Sized + IXMLDOMNode_Impl {
    fn target(&self) -> windows_core::Result<windows_core::BSTR>;
    fn data(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Setdata(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMProcessingInstruction {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMProcessingInstruction_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>() -> IXMLDOMProcessingInstruction_Vtbl {
        unsafe extern "system" fn target<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMProcessingInstruction_Impl::target(this) {
                Ok(ok__) => {
                    core::ptr::write(name, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn data<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMProcessingInstruction_Impl::data(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setdata<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMProcessingInstruction_Impl::Setdata(this, core::mem::transmute(&value)).into()
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            target: target::<Identity, Impl, OFFSET>,
            data: data::<Identity, Impl, OFFSET>,
            Setdata: Setdata::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMProcessingInstruction as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMSchemaCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn add(&self, namespaceuri: &windows_core::BSTR, var: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn get(&self, namespaceuri: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode>;
    fn remove(&self, namespaceuri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn length(&self) -> windows_core::Result<i32>;
    fn get_namespaceURI(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn addCollection(&self, othercollection: Option<&IXMLDOMSchemaCollection>) -> windows_core::Result<()>;
    fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMSchemaCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMSchemaCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>() -> IXMLDOMSchemaCollection_Vtbl {
        unsafe extern "system" fn add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceuri: core::mem::MaybeUninit<windows_core::BSTR>, var: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMSchemaCollection_Impl::add(this, core::mem::transmute(&namespaceuri), core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn get<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceuri: core::mem::MaybeUninit<windows_core::BSTR>, schemanode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMSchemaCollection_Impl::get(this, core::mem::transmute(&namespaceuri)) {
                Ok(ok__) => {
                    core::ptr::write(schemanode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceuri: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMSchemaCollection_Impl::remove(this, core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn length<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMSchemaCollection_Impl::length(this) {
                Ok(ok__) => {
                    core::ptr::write(length, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_namespaceURI<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, length: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMSchemaCollection_Impl::get_namespaceURI(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(length, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addCollection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, othercollection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMSchemaCollection_Impl::addCollection(this, windows_core::from_raw_borrowed(&othercollection)).into()
        }
        unsafe extern "system" fn _newEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMSchemaCollection_Impl::_newEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            add: add::<Identity, Impl, OFFSET>,
            get: get::<Identity, Impl, OFFSET>,
            remove: remove::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            get_namespaceURI: get_namespaceURI::<Identity, Impl, OFFSET>,
            addCollection: addCollection::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMSchemaCollection as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMSchemaCollection2_Impl: Sized + IXMLDOMSchemaCollection_Impl {
    fn validate(&self) -> windows_core::Result<()>;
    fn SetvalidateOnLoad(&self, validateonload: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn validateOnLoad(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn getSchema(&self, namespaceuri: &windows_core::BSTR) -> windows_core::Result<ISchema>;
    fn getDeclaration(&self, node: Option<&IXMLDOMNode>) -> windows_core::Result<ISchemaItem>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMSchemaCollection2 {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMSchemaCollection2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>() -> IXMLDOMSchemaCollection2_Vtbl {
        unsafe extern "system" fn validate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMSchemaCollection2_Impl::validate(this).into()
        }
        unsafe extern "system" fn SetvalidateOnLoad<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, validateonload: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMSchemaCollection2_Impl::SetvalidateOnLoad(this, core::mem::transmute_copy(&validateonload)).into()
        }
        unsafe extern "system" fn validateOnLoad<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, validateonload: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMSchemaCollection2_Impl::validateOnLoad(this) {
                Ok(ok__) => {
                    core::ptr::write(validateonload, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSchema<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceuri: core::mem::MaybeUninit<windows_core::BSTR>, schema: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMSchemaCollection2_Impl::getSchema(this, core::mem::transmute(&namespaceuri)) {
                Ok(ok__) => {
                    core::ptr::write(schema, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDeclaration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMSchemaCollection2_Impl::getDeclaration(this, windows_core::from_raw_borrowed(&node)) {
                Ok(ok__) => {
                    core::ptr::write(item, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXMLDOMSchemaCollection_Vtbl::new::<Identity, Impl, OFFSET>(),
            validate: validate::<Identity, Impl, OFFSET>,
            SetvalidateOnLoad: SetvalidateOnLoad::<Identity, Impl, OFFSET>,
            validateOnLoad: validateOnLoad::<Identity, Impl, OFFSET>,
            getSchema: getSchema::<Identity, Impl, OFFSET>,
            getDeclaration: getDeclaration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMSchemaCollection2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMSchemaCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMSelection_Impl: Sized + IXMLDOMNodeList_Impl {
    fn expr(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Setexpr(&self, expression: &windows_core::BSTR) -> windows_core::Result<()>;
    fn context(&self) -> windows_core::Result<IXMLDOMNode>;
    fn putref_context(&self, pnode: Option<&IXMLDOMNode>) -> windows_core::Result<()>;
    fn peekNode(&self) -> windows_core::Result<IXMLDOMNode>;
    fn matches(&self, pnode: Option<&IXMLDOMNode>) -> windows_core::Result<IXMLDOMNode>;
    fn removeNext(&self) -> windows_core::Result<IXMLDOMNode>;
    fn removeAll(&self) -> windows_core::Result<()>;
    fn clone(&self) -> windows_core::Result<IXMLDOMSelection>;
    fn getProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn setProperty(&self, name: &windows_core::BSTR, value: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMSelection {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMSelection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>() -> IXMLDOMSelection_Vtbl {
        unsafe extern "system" fn expr<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expression: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMSelection_Impl::expr(this) {
                Ok(ok__) => {
                    core::ptr::write(expression, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setexpr<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expression: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMSelection_Impl::Setexpr(this, core::mem::transmute(&expression)).into()
        }
        unsafe extern "system" fn context<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMSelection_Impl::context(this) {
                Ok(ok__) => {
                    core::ptr::write(ppnode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_context<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMSelection_Impl::putref_context(this, windows_core::from_raw_borrowed(&pnode)).into()
        }
        unsafe extern "system" fn peekNode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMSelection_Impl::peekNode(this) {
                Ok(ok__) => {
                    core::ptr::write(ppnode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn matches<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMSelection_Impl::matches(this, windows_core::from_raw_borrowed(&pnode)) {
                Ok(ok__) => {
                    core::ptr::write(ppnode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeNext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMSelection_Impl::removeNext(this) {
                Ok(ok__) => {
                    core::ptr::write(ppnode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAll<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMSelection_Impl::removeAll(this).into()
        }
        unsafe extern "system" fn clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMSelection_Impl::clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppnode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, value: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMSelection_Impl::getProperty(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, value: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDOMSelection_Impl::setProperty(this, core::mem::transmute(&name), core::mem::transmute(&value)).into()
        }
        Self {
            base__: IXMLDOMNodeList_Vtbl::new::<Identity, Impl, OFFSET>(),
            expr: expr::<Identity, Impl, OFFSET>,
            Setexpr: Setexpr::<Identity, Impl, OFFSET>,
            context: context::<Identity, Impl, OFFSET>,
            putref_context: putref_context::<Identity, Impl, OFFSET>,
            peekNode: peekNode::<Identity, Impl, OFFSET>,
            matches: matches::<Identity, Impl, OFFSET>,
            removeNext: removeNext::<Identity, Impl, OFFSET>,
            removeAll: removeAll::<Identity, Impl, OFFSET>,
            clone: clone::<Identity, Impl, OFFSET>,
            getProperty: getProperty::<Identity, Impl, OFFSET>,
            setProperty: setProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMSelection as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNodeList as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMText_Impl: Sized + IXMLDOMCharacterData_Impl {
    fn splitText(&self, offset: i32) -> windows_core::Result<IXMLDOMText>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDOMText {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMText_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMText_Impl, const OFFSET: isize>() -> IXMLDOMText_Vtbl {
        unsafe extern "system" fn splitText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: i32, righthandtextnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDOMText_Impl::splitText(this, core::mem::transmute_copy(&offset)) {
                Ok(ok__) => {
                    core::ptr::write(righthandtextnode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IXMLDOMCharacterData_Vtbl::new::<Identity, Impl, OFFSET>(), splitText: splitText::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMText as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID || iid == &<IXMLDOMCharacterData as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDSOControl_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn XMLDocument(&self) -> windows_core::Result<IXMLDOMDocument>;
    fn SetXMLDocument(&self, ppdoc: Option<&IXMLDOMDocument>) -> windows_core::Result<()>;
    fn JavaDSOCompatible(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetJavaDSOCompatible(&self, fjavadsocompatible: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn readyState(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDSOControl {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDSOControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>() -> IXMLDSOControl_Vtbl {
        unsafe extern "system" fn XMLDocument<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdoc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDSOControl_Impl::XMLDocument(this) {
                Ok(ok__) => {
                    core::ptr::write(ppdoc, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXMLDocument<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdoc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDSOControl_Impl::SetXMLDocument(this, windows_core::from_raw_borrowed(&ppdoc)).into()
        }
        unsafe extern "system" fn JavaDSOCompatible<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fjavadsocompatible: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDSOControl_Impl::JavaDSOCompatible(this) {
                Ok(ok__) => {
                    core::ptr::write(fjavadsocompatible, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJavaDSOCompatible<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fjavadsocompatible: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDSOControl_Impl::SetJavaDSOCompatible(this, core::mem::transmute_copy(&fjavadsocompatible)).into()
        }
        unsafe extern "system" fn readyState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDSOControl_Impl::readyState(this) {
                Ok(ok__) => {
                    core::ptr::write(state, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            XMLDocument: XMLDocument::<Identity, Impl, OFFSET>,
            SetXMLDocument: SetXMLDocument::<Identity, Impl, OFFSET>,
            JavaDSOCompatible: JavaDSOCompatible::<Identity, Impl, OFFSET>,
            SetJavaDSOCompatible: SetJavaDSOCompatible::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDSOControl as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDocument_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn root(&self) -> windows_core::Result<IXMLElement>;
    fn fileSize(&self) -> windows_core::Result<windows_core::BSTR>;
    fn fileModifiedDate(&self) -> windows_core::Result<windows_core::BSTR>;
    fn fileUpdatedDate(&self) -> windows_core::Result<windows_core::BSTR>;
    fn URL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetURL(&self, p: &windows_core::BSTR) -> windows_core::Result<()>;
    fn mimeType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn readyState(&self) -> windows_core::Result<i32>;
    fn charset(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Setcharset(&self, p: &windows_core::BSTR) -> windows_core::Result<()>;
    fn version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn doctype(&self) -> windows_core::Result<windows_core::BSTR>;
    fn dtdURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn createElement(&self, vtype: &windows_core::VARIANT, var1: &windows_core::VARIANT) -> windows_core::Result<IXMLElement>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDocument {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDocument_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>() -> IXMLDocument_Vtbl {
        unsafe extern "system" fn root<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument_Impl::root(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument_Impl::fileSize(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileModifiedDate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument_Impl::fileModifiedDate(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileUpdatedDate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument_Impl::fileUpdatedDate(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn URL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument_Impl::URL(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDocument_Impl::SetURL(this, core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn mimeType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument_Impl::mimeType(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pl: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument_Impl::readyState(this) {
                Ok(ok__) => {
                    core::ptr::write(pl, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn charset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument_Impl::charset(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setcharset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDocument_Impl::Setcharset(this, core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn version<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument_Impl::version(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn doctype<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument_Impl::doctype(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dtdURL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument_Impl::dtdURL(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtype: core::mem::MaybeUninit<windows_core::VARIANT>, var1: core::mem::MaybeUninit<windows_core::VARIANT>, ppelem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument_Impl::createElement(this, core::mem::transmute(&vtype), core::mem::transmute(&var1)) {
                Ok(ok__) => {
                    core::ptr::write(ppelem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            root: root::<Identity, Impl, OFFSET>,
            fileSize: fileSize::<Identity, Impl, OFFSET>,
            fileModifiedDate: fileModifiedDate::<Identity, Impl, OFFSET>,
            fileUpdatedDate: fileUpdatedDate::<Identity, Impl, OFFSET>,
            URL: URL::<Identity, Impl, OFFSET>,
            SetURL: SetURL::<Identity, Impl, OFFSET>,
            mimeType: mimeType::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
            charset: charset::<Identity, Impl, OFFSET>,
            Setcharset: Setcharset::<Identity, Impl, OFFSET>,
            version: version::<Identity, Impl, OFFSET>,
            doctype: doctype::<Identity, Impl, OFFSET>,
            dtdURL: dtdURL::<Identity, Impl, OFFSET>,
            createElement: createElement::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDocument as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDocument2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn root(&self) -> windows_core::Result<IXMLElement2>;
    fn fileSize(&self) -> windows_core::Result<windows_core::BSTR>;
    fn fileModifiedDate(&self) -> windows_core::Result<windows_core::BSTR>;
    fn fileUpdatedDate(&self) -> windows_core::Result<windows_core::BSTR>;
    fn URL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetURL(&self, p: &windows_core::BSTR) -> windows_core::Result<()>;
    fn mimeType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn readyState(&self) -> windows_core::Result<i32>;
    fn charset(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Setcharset(&self, p: &windows_core::BSTR) -> windows_core::Result<()>;
    fn version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn doctype(&self) -> windows_core::Result<windows_core::BSTR>;
    fn dtdURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn createElement(&self, vtype: &windows_core::VARIANT, var1: &windows_core::VARIANT) -> windows_core::Result<IXMLElement2>;
    fn r#async(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setasync(&self, f: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLDocument2 {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDocument2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>() -> IXMLDocument2_Vtbl {
        unsafe extern "system" fn root<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument2_Impl::root(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument2_Impl::fileSize(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileModifiedDate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument2_Impl::fileModifiedDate(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileUpdatedDate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument2_Impl::fileUpdatedDate(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn URL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument2_Impl::URL(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDocument2_Impl::SetURL(this, core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn mimeType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument2_Impl::mimeType(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pl: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument2_Impl::readyState(this) {
                Ok(ok__) => {
                    core::ptr::write(pl, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn charset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument2_Impl::charset(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setcharset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDocument2_Impl::Setcharset(this, core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn version<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument2_Impl::version(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn doctype<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument2_Impl::doctype(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dtdURL<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument2_Impl::dtdURL(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createElement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtype: core::mem::MaybeUninit<windows_core::VARIANT>, var1: core::mem::MaybeUninit<windows_core::VARIANT>, ppelem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument2_Impl::createElement(this, core::mem::transmute(&vtype), core::mem::transmute(&var1)) {
                Ok(ok__) => {
                    core::ptr::write(ppelem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#async<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pf: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLDocument2_Impl::r#async(this) {
                Ok(ok__) => {
                    core::ptr::write(pf, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setasync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, f: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLDocument2_Impl::Setasync(this, core::mem::transmute_copy(&f)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            root: root::<Identity, Impl, OFFSET>,
            fileSize: fileSize::<Identity, Impl, OFFSET>,
            fileModifiedDate: fileModifiedDate::<Identity, Impl, OFFSET>,
            fileUpdatedDate: fileUpdatedDate::<Identity, Impl, OFFSET>,
            URL: URL::<Identity, Impl, OFFSET>,
            SetURL: SetURL::<Identity, Impl, OFFSET>,
            mimeType: mimeType::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
            charset: charset::<Identity, Impl, OFFSET>,
            Setcharset: Setcharset::<Identity, Impl, OFFSET>,
            version: version::<Identity, Impl, OFFSET>,
            doctype: doctype::<Identity, Impl, OFFSET>,
            dtdURL: dtdURL::<Identity, Impl, OFFSET>,
            createElement: createElement::<Identity, Impl, OFFSET>,
            r#async: r#async::<Identity, Impl, OFFSET>,
            Setasync: Setasync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDocument2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLElement_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn tagName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SettagName(&self, p: &windows_core::BSTR) -> windows_core::Result<()>;
    fn parent(&self) -> windows_core::Result<IXMLElement>;
    fn setAttribute(&self, strpropertyname: &windows_core::BSTR, propertyvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn getAttribute(&self, strpropertyname: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn removeAttribute(&self, strpropertyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn children(&self) -> windows_core::Result<IXMLElementCollection>;
    fn r#type(&self) -> windows_core::Result<i32>;
    fn text(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Settext(&self, p: &windows_core::BSTR) -> windows_core::Result<()>;
    fn addChild(&self, pchildelem: Option<&IXMLElement>, lindex: i32, lreserved: i32) -> windows_core::Result<()>;
    fn removeChild(&self, pchildelem: Option<&IXMLElement>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLElement {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLElement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>() -> IXMLElement_Vtbl {
        unsafe extern "system" fn tagName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLElement_Impl::tagName(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettagName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLElement_Impl::SettagName(this, core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn parent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLElement_Impl::parent(this) {
                Ok(ok__) => {
                    core::ptr::write(ppparent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLElement_Impl::setAttribute(this, core::mem::transmute(&strpropertyname), core::mem::transmute(&propertyvalue)).into()
        }
        unsafe extern "system" fn getAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLElement_Impl::getAttribute(this, core::mem::transmute(&strpropertyname)) {
                Ok(ok__) => {
                    core::ptr::write(propertyvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLElement_Impl::removeAttribute(this, core::mem::transmute(&strpropertyname)).into()
        }
        unsafe extern "system" fn children<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLElement_Impl::children(this) {
                Ok(ok__) => {
                    core::ptr::write(pp, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltype: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLElement_Impl::r#type(this) {
                Ok(ok__) => {
                    core::ptr::write(pltype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn text<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLElement_Impl::text(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLElement_Impl::Settext(this, core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn addChild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchildelem: *mut core::ffi::c_void, lindex: i32, lreserved: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLElement_Impl::addChild(this, windows_core::from_raw_borrowed(&pchildelem), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&lreserved)).into()
        }
        unsafe extern "system" fn removeChild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchildelem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLElement_Impl::removeChild(this, windows_core::from_raw_borrowed(&pchildelem)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            tagName: tagName::<Identity, Impl, OFFSET>,
            SettagName: SettagName::<Identity, Impl, OFFSET>,
            parent: parent::<Identity, Impl, OFFSET>,
            setAttribute: setAttribute::<Identity, Impl, OFFSET>,
            getAttribute: getAttribute::<Identity, Impl, OFFSET>,
            removeAttribute: removeAttribute::<Identity, Impl, OFFSET>,
            children: children::<Identity, Impl, OFFSET>,
            r#type: r#type::<Identity, Impl, OFFSET>,
            text: text::<Identity, Impl, OFFSET>,
            Settext: Settext::<Identity, Impl, OFFSET>,
            addChild: addChild::<Identity, Impl, OFFSET>,
            removeChild: removeChild::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLElement as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLElement2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn tagName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SettagName(&self, p: &windows_core::BSTR) -> windows_core::Result<()>;
    fn parent(&self) -> windows_core::Result<IXMLElement2>;
    fn setAttribute(&self, strpropertyname: &windows_core::BSTR, propertyvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn getAttribute(&self, strpropertyname: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn removeAttribute(&self, strpropertyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn children(&self) -> windows_core::Result<IXMLElementCollection>;
    fn r#type(&self) -> windows_core::Result<i32>;
    fn text(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Settext(&self, p: &windows_core::BSTR) -> windows_core::Result<()>;
    fn addChild(&self, pchildelem: Option<&IXMLElement2>, lindex: i32, lreserved: i32) -> windows_core::Result<()>;
    fn removeChild(&self, pchildelem: Option<&IXMLElement2>) -> windows_core::Result<()>;
    fn attributes(&self) -> windows_core::Result<IXMLElementCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLElement2 {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLElement2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>() -> IXMLElement2_Vtbl {
        unsafe extern "system" fn tagName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLElement2_Impl::tagName(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettagName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLElement2_Impl::SettagName(this, core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn parent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLElement2_Impl::parent(this) {
                Ok(ok__) => {
                    core::ptr::write(ppparent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLElement2_Impl::setAttribute(this, core::mem::transmute(&strpropertyname), core::mem::transmute(&propertyvalue)).into()
        }
        unsafe extern "system" fn getAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLElement2_Impl::getAttribute(this, core::mem::transmute(&strpropertyname)) {
                Ok(ok__) => {
                    core::ptr::write(propertyvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttribute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLElement2_Impl::removeAttribute(this, core::mem::transmute(&strpropertyname)).into()
        }
        unsafe extern "system" fn children<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLElement2_Impl::children(this) {
                Ok(ok__) => {
                    core::ptr::write(pp, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltype: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLElement2_Impl::r#type(this) {
                Ok(ok__) => {
                    core::ptr::write(pltype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn text<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLElement2_Impl::text(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLElement2_Impl::Settext(this, core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn addChild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchildelem: *mut core::ffi::c_void, lindex: i32, lreserved: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLElement2_Impl::addChild(this, windows_core::from_raw_borrowed(&pchildelem), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&lreserved)).into()
        }
        unsafe extern "system" fn removeChild<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchildelem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLElement2_Impl::removeChild(this, windows_core::from_raw_borrowed(&pchildelem)).into()
        }
        unsafe extern "system" fn attributes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLElement2_Impl::attributes(this) {
                Ok(ok__) => {
                    core::ptr::write(pp, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            tagName: tagName::<Identity, Impl, OFFSET>,
            SettagName: SettagName::<Identity, Impl, OFFSET>,
            parent: parent::<Identity, Impl, OFFSET>,
            setAttribute: setAttribute::<Identity, Impl, OFFSET>,
            getAttribute: getAttribute::<Identity, Impl, OFFSET>,
            removeAttribute: removeAttribute::<Identity, Impl, OFFSET>,
            children: children::<Identity, Impl, OFFSET>,
            r#type: r#type::<Identity, Impl, OFFSET>,
            text: text::<Identity, Impl, OFFSET>,
            Settext: Settext::<Identity, Impl, OFFSET>,
            addChild: addChild::<Identity, Impl, OFFSET>,
            removeChild: removeChild::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLElement2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLElementCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Setlength(&self, v: i32) -> windows_core::Result<()>;
    fn length(&self) -> windows_core::Result<i32>;
    fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn item(&self, var1: &windows_core::VARIANT, var2: &windows_core::VARIANT) -> windows_core::Result<super::super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLElementCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLElementCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: isize>() -> IXMLElementCollection_Vtbl {
        unsafe extern "system" fn Setlength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLElementCollection_Impl::Setlength(this, core::mem::transmute_copy(&v)).into()
        }
        unsafe extern "system" fn length<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLElementCollection_Impl::length(this) {
                Ok(ok__) => {
                    core::ptr::write(p, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLElementCollection_Impl::_newEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, var1: core::mem::MaybeUninit<windows_core::VARIANT>, var2: core::mem::MaybeUninit<windows_core::VARIANT>, ppdisp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLElementCollection_Impl::item(this, core::mem::transmute(&var1), core::mem::transmute(&var2)) {
                Ok(ok__) => {
                    core::ptr::write(ppdisp, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Setlength: Setlength::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLElementCollection as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IXMLError_Impl: Sized {
    fn GetErrorInfo(&self, perrorreturn: *mut XML_ERROR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXMLError {}
impl IXMLError_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLError_Impl, const OFFSET: isize>() -> IXMLError_Vtbl {
        unsafe extern "system" fn GetErrorInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrorreturn: *mut XML_ERROR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLError_Impl::GetErrorInfo(this, core::mem::transmute_copy(&perrorreturn)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetErrorInfo: GetErrorInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLError as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHTTPRequest_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn open(&self, bstrmethod: &windows_core::BSTR, bstrurl: &windows_core::BSTR, varasync: &windows_core::VARIANT, bstruser: &windows_core::VARIANT, bstrpassword: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn setRequestHeader(&self, bstrheader: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn getResponseHeader(&self, bstrheader: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn getAllResponseHeaders(&self) -> windows_core::Result<windows_core::BSTR>;
    fn send(&self, varbody: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn abort(&self) -> windows_core::Result<()>;
    fn status(&self) -> windows_core::Result<i32>;
    fn statusText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn responseXML(&self) -> windows_core::Result<super::super::super::System::Com::IDispatch>;
    fn responseText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn responseBody(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn responseStream(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn readyState(&self) -> windows_core::Result<i32>;
    fn Setonreadystatechange(&self, preadystatesink: Option<&super::super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLHTTPRequest {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLHTTPRequest_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>() -> IXMLHTTPRequest_Vtbl {
        unsafe extern "system" fn open<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmethod: core::mem::MaybeUninit<windows_core::BSTR>, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, varasync: core::mem::MaybeUninit<windows_core::VARIANT>, bstruser: core::mem::MaybeUninit<windows_core::VARIANT>, bstrpassword: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest_Impl::open(this, core::mem::transmute(&bstrmethod), core::mem::transmute(&bstrurl), core::mem::transmute(&varasync), core::mem::transmute(&bstruser), core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn setRequestHeader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrheader: core::mem::MaybeUninit<windows_core::BSTR>, bstrvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest_Impl::setRequestHeader(this, core::mem::transmute(&bstrheader), core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn getResponseHeader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrheader: core::mem::MaybeUninit<windows_core::BSTR>, pbstrvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLHTTPRequest_Impl::getResponseHeader(this, core::mem::transmute(&bstrheader)) {
                Ok(ok__) => {
                    core::ptr::write(pbstrvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAllResponseHeaders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrheaders: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLHTTPRequest_Impl::getAllResponseHeaders(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrheaders, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn send<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varbody: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest_Impl::send(this, core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn abort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest_Impl::abort(this).into()
        }
        unsafe extern "system" fn status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatus: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLHTTPRequest_Impl::status(this) {
                Ok(ok__) => {
                    core::ptr::write(plstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn statusText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatus: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLHTTPRequest_Impl::statusText(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseXML<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbody: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLHTTPRequest_Impl::responseXML(this) {
                Ok(ok__) => {
                    core::ptr::write(ppbody, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbody: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLHTTPRequest_Impl::responseText(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrbody, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseBody<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbody: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLHTTPRequest_Impl::responseBody(this) {
                Ok(ok__) => {
                    core::ptr::write(pvarbody, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbody: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLHTTPRequest_Impl::responseStream(this) {
                Ok(ok__) => {
                    core::ptr::write(pvarbody, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstate: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLHTTPRequest_Impl::readyState(this) {
                Ok(ok__) => {
                    core::ptr::write(plstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setonreadystatechange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preadystatesink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest_Impl::Setonreadystatechange(this, windows_core::from_raw_borrowed(&preadystatesink)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            open: open::<Identity, Impl, OFFSET>,
            setRequestHeader: setRequestHeader::<Identity, Impl, OFFSET>,
            getResponseHeader: getResponseHeader::<Identity, Impl, OFFSET>,
            getAllResponseHeaders: getAllResponseHeaders::<Identity, Impl, OFFSET>,
            send: send::<Identity, Impl, OFFSET>,
            abort: abort::<Identity, Impl, OFFSET>,
            status: status::<Identity, Impl, OFFSET>,
            statusText: statusText::<Identity, Impl, OFFSET>,
            responseXML: responseXML::<Identity, Impl, OFFSET>,
            responseText: responseText::<Identity, Impl, OFFSET>,
            responseBody: responseBody::<Identity, Impl, OFFSET>,
            responseStream: responseStream::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
            Setonreadystatechange: Setonreadystatechange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLHTTPRequest as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHTTPRequest2_Impl: Sized {
    fn Open(&self, pwszmethod: &windows_core::PCWSTR, pwszurl: &windows_core::PCWSTR, pstatuscallback: Option<&IXMLHTTPRequest2Callback>, pwszusername: &windows_core::PCWSTR, pwszpassword: &windows_core::PCWSTR, pwszproxyusername: &windows_core::PCWSTR, pwszproxypassword: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Send(&self, pbody: Option<&super::super::super::System::Com::ISequentialStream>, cbbody: u64) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn SetCookie(&self, pcookie: *const XHR_COOKIE) -> windows_core::Result<u32>;
    fn SetCustomResponseStream(&self, psequentialstream: Option<&super::super::super::System::Com::ISequentialStream>) -> windows_core::Result<()>;
    fn SetProperty(&self, eproperty: XHR_PROPERTY, ullvalue: u64) -> windows_core::Result<()>;
    fn SetRequestHeader(&self, pwszheader: &windows_core::PCWSTR, pwszvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetAllResponseHeaders(&self) -> windows_core::Result<*mut u16>;
    fn GetCookie(&self, pwszurl: &windows_core::PCWSTR, pwszname: &windows_core::PCWSTR, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> windows_core::Result<()>;
    fn GetResponseHeader(&self, pwszheader: &windows_core::PCWSTR) -> windows_core::Result<*mut u16>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLHTTPRequest2 {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLHTTPRequest2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>() -> IXMLHTTPRequest2_Vtbl {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszmethod: windows_core::PCWSTR, pwszurl: windows_core::PCWSTR, pstatuscallback: *mut core::ffi::c_void, pwszusername: windows_core::PCWSTR, pwszpassword: windows_core::PCWSTR, pwszproxyusername: windows_core::PCWSTR, pwszproxypassword: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest2_Impl::Open(this, core::mem::transmute(&pwszmethod), core::mem::transmute(&pwszurl), windows_core::from_raw_borrowed(&pstatuscallback), core::mem::transmute(&pwszusername), core::mem::transmute(&pwszpassword), core::mem::transmute(&pwszproxyusername), core::mem::transmute(&pwszproxypassword)).into()
        }
        unsafe extern "system" fn Send<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *mut core::ffi::c_void, cbbody: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest2_Impl::Send(this, windows_core::from_raw_borrowed(&pbody), core::mem::transmute_copy(&cbbody)).into()
        }
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest2_Impl::Abort(this).into()
        }
        unsafe extern "system" fn SetCookie<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcookie: *const XHR_COOKIE, pdwcookiestate: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLHTTPRequest2_Impl::SetCookie(this, core::mem::transmute_copy(&pcookie)) {
                Ok(ok__) => {
                    core::ptr::write(pdwcookiestate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomResponseStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psequentialstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest2_Impl::SetCustomResponseStream(this, windows_core::from_raw_borrowed(&psequentialstream)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eproperty: XHR_PROPERTY, ullvalue: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest2_Impl::SetProperty(this, core::mem::transmute_copy(&eproperty), core::mem::transmute_copy(&ullvalue)).into()
        }
        unsafe extern "system" fn SetRequestHeader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszheader: windows_core::PCWSTR, pwszvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest2_Impl::SetRequestHeader(this, core::mem::transmute(&pwszheader), core::mem::transmute(&pwszvalue)).into()
        }
        unsafe extern "system" fn GetAllResponseHeaders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszheaders: *mut *mut u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLHTTPRequest2_Impl::GetAllResponseHeaders(this) {
                Ok(ok__) => {
                    core::ptr::write(ppwszheaders, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCookie<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, pwszname: windows_core::PCWSTR, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest2_Impl::GetCookie(this, core::mem::transmute(&pwszurl), core::mem::transmute(&pwszname), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pccookies), core::mem::transmute_copy(&ppcookies)).into()
        }
        unsafe extern "system" fn GetResponseHeader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszheader: windows_core::PCWSTR, ppwszvalue: *mut *mut u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXMLHTTPRequest2_Impl::GetResponseHeader(this, core::mem::transmute(&pwszheader)) {
                Ok(ok__) => {
                    core::ptr::write(ppwszvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Send: Send::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            SetCookie: SetCookie::<Identity, Impl, OFFSET>,
            SetCustomResponseStream: SetCustomResponseStream::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            SetRequestHeader: SetRequestHeader::<Identity, Impl, OFFSET>,
            GetAllResponseHeaders: GetAllResponseHeaders::<Identity, Impl, OFFSET>,
            GetCookie: GetCookie::<Identity, Impl, OFFSET>,
            GetResponseHeader: GetResponseHeader::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLHTTPRequest2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHTTPRequest2Callback_Impl: Sized {
    fn OnRedirect(&self, pxhr: Option<&IXMLHTTPRequest2>, pwszredirecturl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnHeadersAvailable(&self, pxhr: Option<&IXMLHTTPRequest2>, dwstatus: u32, pwszstatus: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnDataAvailable(&self, pxhr: Option<&IXMLHTTPRequest2>, presponsestream: Option<&super::super::super::System::Com::ISequentialStream>) -> windows_core::Result<()>;
    fn OnResponseReceived(&self, pxhr: Option<&IXMLHTTPRequest2>, presponsestream: Option<&super::super::super::System::Com::ISequentialStream>) -> windows_core::Result<()>;
    fn OnError(&self, pxhr: Option<&IXMLHTTPRequest2>, hrerror: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLHTTPRequest2Callback {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLHTTPRequest2Callback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>() -> IXMLHTTPRequest2Callback_Vtbl {
        unsafe extern "system" fn OnRedirect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxhr: *mut core::ffi::c_void, pwszredirecturl: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest2Callback_Impl::OnRedirect(this, windows_core::from_raw_borrowed(&pxhr), core::mem::transmute(&pwszredirecturl)).into()
        }
        unsafe extern "system" fn OnHeadersAvailable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxhr: *mut core::ffi::c_void, dwstatus: u32, pwszstatus: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest2Callback_Impl::OnHeadersAvailable(this, windows_core::from_raw_borrowed(&pxhr), core::mem::transmute_copy(&dwstatus), core::mem::transmute(&pwszstatus)).into()
        }
        unsafe extern "system" fn OnDataAvailable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxhr: *mut core::ffi::c_void, presponsestream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest2Callback_Impl::OnDataAvailable(this, windows_core::from_raw_borrowed(&pxhr), windows_core::from_raw_borrowed(&presponsestream)).into()
        }
        unsafe extern "system" fn OnResponseReceived<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxhr: *mut core::ffi::c_void, presponsestream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest2Callback_Impl::OnResponseReceived(this, windows_core::from_raw_borrowed(&pxhr), windows_core::from_raw_borrowed(&presponsestream)).into()
        }
        unsafe extern "system" fn OnError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxhr: *mut core::ffi::c_void, hrerror: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest2Callback_Impl::OnError(this, windows_core::from_raw_borrowed(&pxhr), core::mem::transmute_copy(&hrerror)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnRedirect: OnRedirect::<Identity, Impl, OFFSET>,
            OnHeadersAvailable: OnHeadersAvailable::<Identity, Impl, OFFSET>,
            OnDataAvailable: OnDataAvailable::<Identity, Impl, OFFSET>,
            OnResponseReceived: OnResponseReceived::<Identity, Impl, OFFSET>,
            OnError: OnError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLHTTPRequest2Callback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHTTPRequest3_Impl: Sized + IXMLHTTPRequest2_Impl {
    fn SetClientCertificate(&self, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLHTTPRequest3 {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLHTTPRequest3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest3_Impl, const OFFSET: isize>() -> IXMLHTTPRequest3_Vtbl {
        unsafe extern "system" fn SetClientCertificate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest3_Impl::SetClientCertificate(this, core::mem::transmute_copy(&cbclientcertificatehash), core::mem::transmute_copy(&pbclientcertificatehash), core::mem::transmute(&pwszpin)).into()
        }
        Self { base__: IXMLHTTPRequest2_Vtbl::new::<Identity, Impl, OFFSET>(), SetClientCertificate: SetClientCertificate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLHTTPRequest3 as windows_core::Interface>::IID || iid == &<IXMLHTTPRequest2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHTTPRequest3Callback_Impl: Sized + IXMLHTTPRequest2Callback_Impl {
    fn OnServerCertificateReceived(&self, pxhr: Option<&IXMLHTTPRequest3>, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> windows_core::Result<()>;
    fn OnClientCertificateRequested(&self, pxhr: Option<&IXMLHTTPRequest3>, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXMLHTTPRequest3Callback {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLHTTPRequest3Callback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>() -> IXMLHTTPRequest3Callback_Vtbl {
        unsafe extern "system" fn OnServerCertificateReceived<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxhr: *mut core::ffi::c_void, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest3Callback_Impl::OnServerCertificateReceived(this, windows_core::from_raw_borrowed(&pxhr), core::mem::transmute_copy(&dwcertificateerrors), core::mem::transmute_copy(&cservercertificatechain), core::mem::transmute_copy(&rgservercertificatechain)).into()
        }
        unsafe extern "system" fn OnClientCertificateRequested<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxhr: *mut core::ffi::c_void, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXMLHTTPRequest3Callback_Impl::OnClientCertificateRequested(this, windows_core::from_raw_borrowed(&pxhr), core::mem::transmute_copy(&cissuerlist), core::mem::transmute_copy(&rgpwszissuerlist)).into()
        }
        Self {
            base__: IXMLHTTPRequest2Callback_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnServerCertificateReceived: OnServerCertificateReceived::<Identity, Impl, OFFSET>,
            OnClientCertificateRequested: OnClientCertificateRequested::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLHTTPRequest3Callback as windows_core::Interface>::IID || iid == &<IXMLHTTPRequest2Callback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXSLProcessor_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Setinput(&self, var: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn input(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn ownerTemplate(&self) -> windows_core::Result<IXSLTemplate>;
    fn setStartMode(&self, mode: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn startMode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn startModeURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Setoutput(&self, output: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn output(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn transform(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn reset(&self) -> windows_core::Result<()>;
    fn readyState(&self) -> windows_core::Result<i32>;
    fn addParameter(&self, basename: &windows_core::BSTR, parameter: &windows_core::VARIANT, namespaceuri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn addObject(&self, obj: Option<&super::super::super::System::Com::IDispatch>, namespaceuri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn stylesheet(&self) -> windows_core::Result<IXMLDOMNode>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXSLProcessor {}
#[cfg(feature = "Win32_System_Com")]
impl IXSLProcessor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>() -> IXSLProcessor_Vtbl {
        unsafe extern "system" fn Setinput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, var: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXSLProcessor_Impl::Setinput(this, core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn input<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXSLProcessor_Impl::input(this) {
                Ok(ok__) => {
                    core::ptr::write(pvar, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ownerTemplate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptemplate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXSLProcessor_Impl::ownerTemplate(this) {
                Ok(ok__) => {
                    core::ptr::write(pptemplate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setStartMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: core::mem::MaybeUninit<windows_core::BSTR>, namespaceuri: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXSLProcessor_Impl::setStartMode(this, core::mem::transmute(&mode), core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn startMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXSLProcessor_Impl::startMode(this) {
                Ok(ok__) => {
                    core::ptr::write(mode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startModeURI<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceuri: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXSLProcessor_Impl::startModeURI(this) {
                Ok(ok__) => {
                    core::ptr::write(namespaceuri, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setoutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, output: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXSLProcessor_Impl::Setoutput(this, core::mem::transmute(&output)).into()
        }
        unsafe extern "system" fn output<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutput: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXSLProcessor_Impl::output(this) {
                Ok(ok__) => {
                    core::ptr::write(poutput, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn transform<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdone: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXSLProcessor_Impl::transform(this) {
                Ok(ok__) => {
                    core::ptr::write(pdone, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXSLProcessor_Impl::reset(this).into()
        }
        unsafe extern "system" fn readyState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preadystate: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXSLProcessor_Impl::readyState(this) {
                Ok(ok__) => {
                    core::ptr::write(preadystate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addParameter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, basename: core::mem::MaybeUninit<windows_core::BSTR>, parameter: core::mem::MaybeUninit<windows_core::VARIANT>, namespaceuri: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXSLProcessor_Impl::addParameter(this, core::mem::transmute(&basename), core::mem::transmute(&parameter), core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn addObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, obj: *mut core::ffi::c_void, namespaceuri: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXSLProcessor_Impl::addObject(this, windows_core::from_raw_borrowed(&obj), core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn stylesheet<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stylesheet: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXSLProcessor_Impl::stylesheet(this) {
                Ok(ok__) => {
                    core::ptr::write(stylesheet, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Setinput: Setinput::<Identity, Impl, OFFSET>,
            input: input::<Identity, Impl, OFFSET>,
            ownerTemplate: ownerTemplate::<Identity, Impl, OFFSET>,
            setStartMode: setStartMode::<Identity, Impl, OFFSET>,
            startMode: startMode::<Identity, Impl, OFFSET>,
            startModeURI: startModeURI::<Identity, Impl, OFFSET>,
            Setoutput: Setoutput::<Identity, Impl, OFFSET>,
            output: output::<Identity, Impl, OFFSET>,
            transform: transform::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
            addParameter: addParameter::<Identity, Impl, OFFSET>,
            addObject: addObject::<Identity, Impl, OFFSET>,
            stylesheet: stylesheet::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXSLProcessor as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXSLTemplate_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn putref_stylesheet(&self, stylesheet: Option<&IXMLDOMNode>) -> windows_core::Result<()>;
    fn stylesheet(&self) -> windows_core::Result<IXMLDOMNode>;
    fn createProcessor(&self) -> windows_core::Result<IXSLProcessor>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXSLTemplate {}
#[cfg(feature = "Win32_System_Com")]
impl IXSLTemplate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLTemplate_Impl, const OFFSET: isize>() -> IXSLTemplate_Vtbl {
        unsafe extern "system" fn putref_stylesheet<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stylesheet: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXSLTemplate_Impl::putref_stylesheet(this, windows_core::from_raw_borrowed(&stylesheet)).into()
        }
        unsafe extern "system" fn stylesheet<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stylesheet: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXSLTemplate_Impl::stylesheet(this) {
                Ok(ok__) => {
                    core::ptr::write(stylesheet, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createProcessor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprocessor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXSLTemplate_Impl::createProcessor(this) {
                Ok(ok__) => {
                    core::ptr::write(ppprocessor, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            putref_stylesheet: putref_stylesheet::<Identity, Impl, OFFSET>,
            stylesheet: stylesheet::<Identity, Impl, OFFSET>,
            createProcessor: createProcessor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXSLTemplate as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXTLRuntime_Impl: Sized + IXMLDOMNode_Impl {
    fn uniqueID(&self, pnode: Option<&IXMLDOMNode>) -> windows_core::Result<i32>;
    fn depth(&self, pnode: Option<&IXMLDOMNode>) -> windows_core::Result<i32>;
    fn childNumber(&self, pnode: Option<&IXMLDOMNode>) -> windows_core::Result<i32>;
    fn ancestorChildNumber(&self, bstrnodename: &windows_core::BSTR, pnode: Option<&IXMLDOMNode>) -> windows_core::Result<i32>;
    fn absoluteChildNumber(&self, pnode: Option<&IXMLDOMNode>) -> windows_core::Result<i32>;
    fn formatIndex(&self, lindex: i32, bstrformat: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn formatNumber(&self, dblnumber: f64, bstrformat: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn formatDate(&self, vardate: &windows_core::VARIANT, bstrformat: &windows_core::BSTR, vardestlocale: &windows_core::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn formatTime(&self, vartime: &windows_core::VARIANT, bstrformat: &windows_core::BSTR, vardestlocale: &windows_core::VARIANT) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXTLRuntime {}
#[cfg(feature = "Win32_System_Com")]
impl IXTLRuntime_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>() -> IXTLRuntime_Vtbl {
        unsafe extern "system" fn uniqueID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void, pid: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXTLRuntime_Impl::uniqueID(this, windows_core::from_raw_borrowed(&pnode)) {
                Ok(ok__) => {
                    core::ptr::write(pid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn depth<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void, pdepth: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXTLRuntime_Impl::depth(this, windows_core::from_raw_borrowed(&pnode)) {
                Ok(ok__) => {
                    core::ptr::write(pdepth, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn childNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void, pnumber: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXTLRuntime_Impl::childNumber(this, windows_core::from_raw_borrowed(&pnode)) {
                Ok(ok__) => {
                    core::ptr::write(pnumber, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ancestorChildNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnodename: core::mem::MaybeUninit<windows_core::BSTR>, pnode: *mut core::ffi::c_void, pnumber: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXTLRuntime_Impl::ancestorChildNumber(this, core::mem::transmute(&bstrnodename), windows_core::from_raw_borrowed(&pnode)) {
                Ok(ok__) => {
                    core::ptr::write(pnumber, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn absoluteChildNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void, pnumber: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXTLRuntime_Impl::absoluteChildNumber(this, windows_core::from_raw_borrowed(&pnode)) {
                Ok(ok__) => {
                    core::ptr::write(pnumber, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn formatIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, bstrformat: core::mem::MaybeUninit<windows_core::BSTR>, pbstrformattedstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXTLRuntime_Impl::formatIndex(this, core::mem::transmute_copy(&lindex), core::mem::transmute(&bstrformat)) {
                Ok(ok__) => {
                    core::ptr::write(pbstrformattedstring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn formatNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dblnumber: f64, bstrformat: core::mem::MaybeUninit<windows_core::BSTR>, pbstrformattedstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXTLRuntime_Impl::formatNumber(this, core::mem::transmute_copy(&dblnumber), core::mem::transmute(&bstrformat)) {
                Ok(ok__) => {
                    core::ptr::write(pbstrformattedstring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn formatDate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardate: core::mem::MaybeUninit<windows_core::VARIANT>, bstrformat: core::mem::MaybeUninit<windows_core::BSTR>, vardestlocale: core::mem::MaybeUninit<windows_core::VARIANT>, pbstrformattedstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXTLRuntime_Impl::formatDate(this, core::mem::transmute(&vardate), core::mem::transmute(&bstrformat), core::mem::transmute(&vardestlocale)) {
                Ok(ok__) => {
                    core::ptr::write(pbstrformattedstring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn formatTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vartime: core::mem::MaybeUninit<windows_core::VARIANT>, bstrformat: core::mem::MaybeUninit<windows_core::BSTR>, vardestlocale: core::mem::MaybeUninit<windows_core::VARIANT>, pbstrformattedstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXTLRuntime_Impl::formatTime(this, core::mem::transmute(&vartime), core::mem::transmute(&bstrformat), core::mem::transmute(&vardestlocale)) {
                Ok(ok__) => {
                    core::ptr::write(pbstrformattedstring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            uniqueID: uniqueID::<Identity, Impl, OFFSET>,
            depth: depth::<Identity, Impl, OFFSET>,
            childNumber: childNumber::<Identity, Impl, OFFSET>,
            ancestorChildNumber: ancestorChildNumber::<Identity, Impl, OFFSET>,
            absoluteChildNumber: absoluteChildNumber::<Identity, Impl, OFFSET>,
            formatIndex: formatIndex::<Identity, Impl, OFFSET>,
            formatNumber: formatNumber::<Identity, Impl, OFFSET>,
            formatDate: formatDate::<Identity, Impl, OFFSET>,
            formatTime: formatTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXTLRuntime as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait XMLDOMDocumentEvents_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for XMLDOMDocumentEvents {}
#[cfg(feature = "Win32_System_Com")]
impl XMLDOMDocumentEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: XMLDOMDocumentEvents_Impl, const OFFSET: isize>() -> XMLDOMDocumentEvents_Vtbl {
        Self { base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<XMLDOMDocumentEvents as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
