#[inline]
pub unsafe fn DxcCreateInstance<T>(rclsid: *const windows_core::GUID) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dxcompiler.dll" "system" fn DxcCreateInstance(rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { DxcCreateInstance(rclsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DxcCreateInstance2<P0, T>(pmalloc: P0, rclsid: *const windows_core::GUID) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::super::super::System::Com::IMalloc>,
    T: windows_core::Interface,
{
    windows_core::link!("dxcompiler.dll" "system" fn DxcCreateInstance2(pmalloc : * mut core::ffi::c_void, rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { DxcCreateInstance2(pmalloc.param().abi(), rclsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
pub const CLSID_DxcAssembler: windows_core::GUID = windows_core::GUID::from_u128(0xd728db68_f903_4f80_94cd_dccf76ec7151);
pub const CLSID_DxcCompiler: windows_core::GUID = windows_core::GUID::from_u128(0x73e22d93_e6ce_47f3_b5bf_f0664f39c1b0);
pub const CLSID_DxcCompilerArgs: windows_core::GUID = windows_core::GUID::from_u128(0x3e56ae82_224d_470f_a1a1_fe3016ee9f9d);
pub const CLSID_DxcContainerBuilder: windows_core::GUID = windows_core::GUID::from_u128(0x94134294_411f_4574_b4d0_8741e25240d2);
pub const CLSID_DxcContainerReflection: windows_core::GUID = windows_core::GUID::from_u128(0xb9f54489_55b8_400c_ba3a_1675e4728b91);
pub const CLSID_DxcDiaDataSource: windows_core::GUID = windows_core::GUID::from_u128(0xcd1f6b73_2ab0_484d_8edc_ebe7a43ca09f);
pub const CLSID_DxcLibrary: windows_core::GUID = windows_core::GUID::from_u128(0x6245d6af_66e0_48fd_80b4_4d271796748c);
pub const CLSID_DxcLinker: windows_core::GUID = windows_core::GUID::from_u128(0xef6a8087_b0ea_4d56_9e45_d07e1a8b7806);
pub const CLSID_DxcOptimizer: windows_core::GUID = windows_core::GUID::from_u128(0xae2cd79f_cc22_453f_9b6b_b124e7a5204c);
pub const CLSID_DxcPdbUtils: windows_core::GUID = windows_core::GUID::from_u128(0x54621dfb_f2ce_457e_ae8c_ec355faeec7c);
pub const CLSID_DxcUtils: windows_core::GUID = windows_core::GUID::from_u128(0x6245d6af_66e0_48fd_80b4_4d271796748c);
pub const CLSID_DxcValidator: windows_core::GUID = windows_core::GUID::from_u128(0x8ca3e215_f728_4cf3_8cdd_88af917587a1);
pub const DXC_ARG_ALL_RESOURCES_BOUND: windows_core::PCWSTR = windows_core::w!("-all_resources_bound");
pub const DXC_ARG_AVOID_FLOW_CONTROL: windows_core::PCWSTR = windows_core::w!("-Gfa");
pub const DXC_ARG_DEBUG: windows_core::PCWSTR = windows_core::w!("-Zi");
pub const DXC_ARG_DEBUG_NAME_FOR_BINARY: windows_core::PCWSTR = windows_core::w!("-Zsb");
pub const DXC_ARG_DEBUG_NAME_FOR_SOURCE: windows_core::PCWSTR = windows_core::w!("-Zss");
pub const DXC_ARG_ENABLE_BACKWARDS_COMPATIBILITY: windows_core::PCWSTR = windows_core::w!("-Gec");
pub const DXC_ARG_ENABLE_STRICTNESS: windows_core::PCWSTR = windows_core::w!("-Ges");
pub const DXC_ARG_IEEE_STRICTNESS: windows_core::PCWSTR = windows_core::w!("-Gis");
pub const DXC_ARG_OPTIMIZATION_LEVEL0: windows_core::PCWSTR = windows_core::w!("-O0");
pub const DXC_ARG_OPTIMIZATION_LEVEL1: windows_core::PCWSTR = windows_core::w!("-O1");
pub const DXC_ARG_OPTIMIZATION_LEVEL2: windows_core::PCWSTR = windows_core::w!("-O2");
pub const DXC_ARG_OPTIMIZATION_LEVEL3: windows_core::PCWSTR = windows_core::w!("-O3");
pub const DXC_ARG_PACK_MATRIX_COLUMN_MAJOR: windows_core::PCWSTR = windows_core::w!("-Zpc");
pub const DXC_ARG_PACK_MATRIX_ROW_MAJOR: windows_core::PCWSTR = windows_core::w!("-Zpr");
pub const DXC_ARG_PREFER_FLOW_CONTROL: windows_core::PCWSTR = windows_core::w!("-Gfp");
pub const DXC_ARG_RESOURCES_MAY_ALIAS: windows_core::PCWSTR = windows_core::w!("-res_may_alias");
pub const DXC_ARG_SKIP_OPTIMIZATIONS: windows_core::PCWSTR = windows_core::w!("-Od");
pub const DXC_ARG_SKIP_VALIDATION: windows_core::PCWSTR = windows_core::w!("-Vd");
pub const DXC_ARG_WARNINGS_ARE_ERRORS: windows_core::PCWSTR = windows_core::w!("-WX");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXC_CP(pub u32);
pub const DXC_CP_ACP: DXC_CP = DXC_CP(0u32);
pub const DXC_CP_UTF16: DXC_CP = DXC_CP(1200u32);
pub const DXC_CP_UTF8: DXC_CP = DXC_CP(65001u32);
pub const DXC_EXTRA_OUTPUT_NAME_STDERR: windows_core::PCWSTR = windows_core::w!("*stderr*");
pub const DXC_EXTRA_OUTPUT_NAME_STDOUT: windows_core::PCWSTR = windows_core::w!("*stdout*");
pub const DXC_HASHFLAG_INCLUDES_SOURCE: u32 = 1u32;
pub const DXC_OUT_DISASSEMBLY: DXC_OUT_KIND = DXC_OUT_KIND(5i32);
pub const DXC_OUT_ERRORS: DXC_OUT_KIND = DXC_OUT_KIND(2i32);
pub const DXC_OUT_EXTRA_OUTPUTS: DXC_OUT_KIND = DXC_OUT_KIND(10i32);
pub const DXC_OUT_HLSL: DXC_OUT_KIND = DXC_OUT_KIND(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXC_OUT_KIND(pub i32);
pub const DXC_OUT_NONE: DXC_OUT_KIND = DXC_OUT_KIND(0i32);
pub const DXC_OUT_OBJECT: DXC_OUT_KIND = DXC_OUT_KIND(1i32);
pub const DXC_OUT_PDB: DXC_OUT_KIND = DXC_OUT_KIND(3i32);
pub const DXC_OUT_REFLECTION: DXC_OUT_KIND = DXC_OUT_KIND(8i32);
pub const DXC_OUT_ROOT_SIGNATURE: DXC_OUT_KIND = DXC_OUT_KIND(9i32);
pub const DXC_OUT_SHADER_HASH: DXC_OUT_KIND = DXC_OUT_KIND(4i32);
pub const DXC_OUT_TEXT: DXC_OUT_KIND = DXC_OUT_KIND(7i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DxcArgPair {
    pub pName: windows_core::PCWSTR,
    pub pValue: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DxcBuffer {
    pub Ptr: *const core::ffi::c_void,
    pub Size: usize,
    pub Encoding: u32,
}
impl Default for DxcBuffer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
pub type DxcCreateInstance2Proc = Option<unsafe extern "system" fn(pmalloc: windows_core::Ref<super::super::super::System::Com::IMalloc>, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type DxcCreateInstanceProc = Option<unsafe extern "system" fn(rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DxcDefine {
    pub Name: windows_core::PCWSTR,
    pub Value: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DxcShaderHash {
    pub Flags: u32,
    pub HashDigest: [u8; 16],
}
impl Default for DxcShaderHash {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DxcValidatorFlags_Default: u32 = 0u32;
pub const DxcValidatorFlags_InPlaceEdit: u32 = 1u32;
pub const DxcValidatorFlags_ModuleOnly: u32 = 4u32;
pub const DxcValidatorFlags_RootSignatureOnly: u32 = 2u32;
pub const DxcValidatorFlags_ValidMask: u32 = 7u32;
pub const DxcVersionInfoFlags_Debug: u32 = 1u32;
pub const DxcVersionInfoFlags_Internal: u32 = 2u32;
pub const DxcVersionInfoFlags_None: u32 = 0u32;
windows_core::imp::define_interface!(IDxcAssembler, IDxcAssembler_Vtbl, 0x091f7a26_1c1f_4948_904b_e6e3a8a771d5);
windows_core::imp::interface_hierarchy!(IDxcAssembler, windows_core::IUnknown);
impl IDxcAssembler {
    pub unsafe fn AssembleToContainer<P0>(&self, pshader: P0) -> windows_core::Result<IDxcOperationResult>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AssembleToContainer)(windows_core::Interface::as_raw(self), pshader.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcAssembler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AssembleToContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxcAssembler_Impl: windows_core::IUnknownImpl {
    fn AssembleToContainer(&self, pshader: windows_core::Ref<IDxcBlob>) -> windows_core::Result<IDxcOperationResult>;
}
impl IDxcAssembler_Vtbl {
    pub const fn new<Identity: IDxcAssembler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AssembleToContainer<Identity: IDxcAssembler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcAssembler_Impl::AssembleToContainer(this, core::mem::transmute_copy(&pshader)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AssembleToContainer: AssembleToContainer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcAssembler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcAssembler {}
windows_core::imp::define_interface!(IDxcBlob, IDxcBlob_Vtbl, 0x8ba5fb08_5195_40e2_ac58_0d989c3a0102);
windows_core::imp::interface_hierarchy!(IDxcBlob, windows_core::IUnknown);
impl IDxcBlob {
    pub unsafe fn GetBufferPointer(&self) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).GetBufferPointer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        unsafe { (windows_core::Interface::vtable(self).GetBufferSize)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlob_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBufferPointer: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
    pub GetBufferSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> usize,
}
pub trait IDxcBlob_Impl: windows_core::IUnknownImpl {
    fn GetBufferPointer(&self) -> *mut core::ffi::c_void;
    fn GetBufferSize(&self) -> usize;
}
impl IDxcBlob_Vtbl {
    pub const fn new<Identity: IDxcBlob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBufferPointer<Identity: IDxcBlob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcBlob_Impl::GetBufferPointer(this)
            }
        }
        unsafe extern "system" fn GetBufferSize<Identity: IDxcBlob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> usize {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcBlob_Impl::GetBufferSize(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBufferPointer: GetBufferPointer::<Identity, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcBlob as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcBlob {}
windows_core::imp::define_interface!(IDxcBlobEncoding, IDxcBlobEncoding_Vtbl, 0x7241d424_2646_4191_97c0_98e96e42fc68);
impl core::ops::Deref for IDxcBlobEncoding {
    type Target = IDxcBlob;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDxcBlobEncoding, windows_core::IUnknown, IDxcBlob);
impl IDxcBlobEncoding {
    pub unsafe fn GetEncoding(&self, pknown: *mut windows_core::BOOL, pcodepage: *mut DXC_CP) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEncoding)(windows_core::Interface::as_raw(self), pknown as _, pcodepage as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobEncoding_Vtbl {
    pub base__: IDxcBlob_Vtbl,
    pub GetEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut DXC_CP) -> windows_core::HRESULT,
}
pub trait IDxcBlobEncoding_Impl: IDxcBlob_Impl {
    fn GetEncoding(&self, pknown: *mut windows_core::BOOL, pcodepage: *mut DXC_CP) -> windows_core::Result<()>;
}
impl IDxcBlobEncoding_Vtbl {
    pub const fn new<Identity: IDxcBlobEncoding_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEncoding<Identity: IDxcBlobEncoding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknown: *mut windows_core::BOOL, pcodepage: *mut DXC_CP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcBlobEncoding_Impl::GetEncoding(this, core::mem::transmute_copy(&pknown), core::mem::transmute_copy(&pcodepage)).into()
            }
        }
        Self { base__: IDxcBlob_Vtbl::new::<Identity, OFFSET>(), GetEncoding: GetEncoding::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcBlobEncoding as windows_core::Interface>::IID || iid == &<IDxcBlob as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcBlobEncoding {}
windows_core::imp::define_interface!(IDxcBlobUtf16, IDxcBlobUtf16_Vtbl, 0xa3f84eab_0faa_497e_a39c_ee6ed60b2d84);
impl core::ops::Deref for IDxcBlobUtf16 {
    type Target = IDxcBlobEncoding;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDxcBlobUtf16, windows_core::IUnknown, IDxcBlob, IDxcBlobEncoding);
impl IDxcBlobUtf16 {
    pub unsafe fn GetStringPointer(&self) -> windows_core::PCWSTR {
        unsafe { (windows_core::Interface::vtable(self).GetStringPointer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetStringLength(&self) -> usize {
        unsafe { (windows_core::Interface::vtable(self).GetStringLength)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf16_Vtbl {
    pub base__: IDxcBlobEncoding_Vtbl,
    pub GetStringPointer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::PCWSTR,
    pub GetStringLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> usize,
}
pub trait IDxcBlobUtf16_Impl: IDxcBlobEncoding_Impl {
    fn GetStringPointer(&self) -> windows_core::PCWSTR;
    fn GetStringLength(&self) -> usize;
}
impl IDxcBlobUtf16_Vtbl {
    pub const fn new<Identity: IDxcBlobUtf16_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStringPointer<Identity: IDxcBlobUtf16_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::PCWSTR {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcBlobUtf16_Impl::GetStringPointer(this)
            }
        }
        unsafe extern "system" fn GetStringLength<Identity: IDxcBlobUtf16_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> usize {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcBlobUtf16_Impl::GetStringLength(this)
            }
        }
        Self {
            base__: IDxcBlobEncoding_Vtbl::new::<Identity, OFFSET>(),
            GetStringPointer: GetStringPointer::<Identity, OFFSET>,
            GetStringLength: GetStringLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcBlobUtf16 as windows_core::Interface>::IID || iid == &<IDxcBlob as windows_core::Interface>::IID || iid == &<IDxcBlobEncoding as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcBlobUtf16 {}
windows_core::imp::define_interface!(IDxcBlobUtf8, IDxcBlobUtf8_Vtbl, 0x3da636c9_ba71_4024_a301_30cbf125305b);
impl core::ops::Deref for IDxcBlobUtf8 {
    type Target = IDxcBlobEncoding;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDxcBlobUtf8, windows_core::IUnknown, IDxcBlob, IDxcBlobEncoding);
impl IDxcBlobUtf8 {
    pub unsafe fn GetStringPointer(&self) -> windows_core::PCSTR {
        unsafe { (windows_core::Interface::vtable(self).GetStringPointer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetStringLength(&self) -> usize {
        unsafe { (windows_core::Interface::vtable(self).GetStringLength)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf8_Vtbl {
    pub base__: IDxcBlobEncoding_Vtbl,
    pub GetStringPointer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::PCSTR,
    pub GetStringLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> usize,
}
pub trait IDxcBlobUtf8_Impl: IDxcBlobEncoding_Impl {
    fn GetStringPointer(&self) -> windows_core::PCSTR;
    fn GetStringLength(&self) -> usize;
}
impl IDxcBlobUtf8_Vtbl {
    pub const fn new<Identity: IDxcBlobUtf8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStringPointer<Identity: IDxcBlobUtf8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::PCSTR {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcBlobUtf8_Impl::GetStringPointer(this)
            }
        }
        unsafe extern "system" fn GetStringLength<Identity: IDxcBlobUtf8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> usize {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcBlobUtf8_Impl::GetStringLength(this)
            }
        }
        Self {
            base__: IDxcBlobEncoding_Vtbl::new::<Identity, OFFSET>(),
            GetStringPointer: GetStringPointer::<Identity, OFFSET>,
            GetStringLength: GetStringLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcBlobUtf8 as windows_core::Interface>::IID || iid == &<IDxcBlob as windows_core::Interface>::IID || iid == &<IDxcBlobEncoding as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcBlobUtf8 {}
windows_core::imp::define_interface!(IDxcCompiler, IDxcCompiler_Vtbl, 0x8c210bf3_011f_4422_8d70_6f9acb8db617);
windows_core::imp::interface_hierarchy!(IDxcCompiler, windows_core::IUnknown);
impl IDxcCompiler {
    pub unsafe fn Compile<P0, P1, P2, P3, P8>(&self, psource: P0, psourcename: P1, pentrypoint: P2, ptargetprofile: P3, parguments: Option<&[windows_core::PCWSTR]>, pdefines: &[DxcDefine], pincludehandler: P8) -> windows_core::Result<IDxcOperationResult>
    where
        P0: windows_core::Param<IDxcBlob>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P8: windows_core::Param<IDxcIncludeHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Compile)(windows_core::Interface::as_raw(self), psource.param().abi(), psourcename.param().abi(), pentrypoint.param().abi(), ptargetprofile.param().abi(), core::mem::transmute(parguments.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pdefines.as_ptr()), pdefines.len().try_into().unwrap(), pincludehandler.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Preprocess<P0, P1, P6>(&self, psource: P0, psourcename: P1, parguments: Option<&[windows_core::PCWSTR]>, pdefines: &[DxcDefine], pincludehandler: P6) -> windows_core::Result<IDxcOperationResult>
    where
        P0: windows_core::Param<IDxcBlob>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<IDxcIncludeHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Preprocess)(windows_core::Interface::as_raw(self), psource.param().abi(), psourcename.param().abi(), core::mem::transmute(parguments.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pdefines.as_ptr()), pdefines.len().try_into().unwrap(), pincludehandler.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Disassemble<P0>(&self, psource: P0) -> windows_core::Result<IDxcBlobEncoding>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Disassemble)(windows_core::Interface::as_raw(self), psource.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Compile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::PCWSTR, u32, *const DxcDefine, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Preprocess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::PCWSTR, u32, *const DxcDefine, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disassemble: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxcCompiler_Impl: windows_core::IUnknownImpl {
    fn Compile(&self, psource: windows_core::Ref<IDxcBlob>, psourcename: &windows_core::PCWSTR, pentrypoint: &windows_core::PCWSTR, ptargetprofile: &windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: windows_core::Ref<IDxcIncludeHandler>) -> windows_core::Result<IDxcOperationResult>;
    fn Preprocess(&self, psource: windows_core::Ref<IDxcBlob>, psourcename: &windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: windows_core::Ref<IDxcIncludeHandler>) -> windows_core::Result<IDxcOperationResult>;
    fn Disassemble(&self, psource: windows_core::Ref<IDxcBlob>) -> windows_core::Result<IDxcBlobEncoding>;
}
impl IDxcCompiler_Vtbl {
    pub const fn new<Identity: IDxcCompiler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Compile<Identity: IDxcCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, psourcename: windows_core::PCWSTR, pentrypoint: windows_core::PCWSTR, ptargetprofile: windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcCompiler_Impl::Compile(this, core::mem::transmute_copy(&psource), core::mem::transmute(&psourcename), core::mem::transmute(&pentrypoint), core::mem::transmute(&ptargetprofile), core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount), core::mem::transmute_copy(&pdefines), core::mem::transmute_copy(&definecount), core::mem::transmute_copy(&pincludehandler)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Preprocess<Identity: IDxcCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, psourcename: windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcCompiler_Impl::Preprocess(this, core::mem::transmute_copy(&psource), core::mem::transmute(&psourcename), core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount), core::mem::transmute_copy(&pdefines), core::mem::transmute_copy(&definecount), core::mem::transmute_copy(&pincludehandler)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Disassemble<Identity: IDxcCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, ppdisassembly: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcCompiler_Impl::Disassemble(this, core::mem::transmute_copy(&psource)) {
                    Ok(ok__) => {
                        ppdisassembly.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compile: Compile::<Identity, OFFSET>,
            Preprocess: Preprocess::<Identity, OFFSET>,
            Disassemble: Disassemble::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcCompiler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcCompiler {}
windows_core::imp::define_interface!(IDxcCompiler2, IDxcCompiler2_Vtbl, 0xa005a9d9_b8bb_4594_b5c9_0e633bec4d37);
impl core::ops::Deref for IDxcCompiler2 {
    type Target = IDxcCompiler;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDxcCompiler2, windows_core::IUnknown, IDxcCompiler);
impl IDxcCompiler2 {
    pub unsafe fn CompileWithDebug<P0, P1, P2, P3, P8>(&self, psource: P0, psourcename: P1, pentrypoint: P2, ptargetprofile: P3, parguments: Option<&[windows_core::PCWSTR]>, pdefines: &[DxcDefine], pincludehandler: P8, ppresult: *mut Option<IDxcOperationResult>, ppdebugblobname: Option<*mut windows_core::PWSTR>, ppdebugblob: Option<*mut Option<IDxcBlob>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDxcBlob>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P8: windows_core::Param<IDxcIncludeHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CompileWithDebug)(
                windows_core::Interface::as_raw(self),
                psource.param().abi(),
                psourcename.param().abi(),
                pentrypoint.param().abi(),
                ptargetprofile.param().abi(),
                core::mem::transmute(parguments.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
                parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(pdefines.as_ptr()),
                pdefines.len().try_into().unwrap(),
                pincludehandler.param().abi(),
                core::mem::transmute(ppresult),
                ppdebugblobname.unwrap_or(core::mem::zeroed()) as _,
                ppdebugblob.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler2_Vtbl {
    pub base__: IDxcCompiler_Vtbl,
    pub CompileWithDebug: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::PCWSTR, u32, *const DxcDefine, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut windows_core::PWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxcCompiler2_Impl: IDxcCompiler_Impl {
    fn CompileWithDebug(&self, psource: windows_core::Ref<IDxcBlob>, psourcename: &windows_core::PCWSTR, pentrypoint: &windows_core::PCWSTR, ptargetprofile: &windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: windows_core::Ref<IDxcIncludeHandler>, ppresult: windows_core::OutRef<IDxcOperationResult>, ppdebugblobname: *mut windows_core::PWSTR, ppdebugblob: windows_core::OutRef<IDxcBlob>) -> windows_core::Result<()>;
}
impl IDxcCompiler2_Vtbl {
    pub const fn new<Identity: IDxcCompiler2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CompileWithDebug<Identity: IDxcCompiler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, psourcename: windows_core::PCWSTR, pentrypoint: windows_core::PCWSTR, ptargetprofile: windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void, ppdebugblobname: *mut windows_core::PWSTR, ppdebugblob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcCompiler2_Impl::CompileWithDebug(this, core::mem::transmute_copy(&psource), core::mem::transmute(&psourcename), core::mem::transmute(&pentrypoint), core::mem::transmute(&ptargetprofile), core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount), core::mem::transmute_copy(&pdefines), core::mem::transmute_copy(&definecount), core::mem::transmute_copy(&pincludehandler), core::mem::transmute_copy(&ppresult), core::mem::transmute_copy(&ppdebugblobname), core::mem::transmute_copy(&ppdebugblob)).into()
            }
        }
        Self { base__: IDxcCompiler_Vtbl::new::<Identity, OFFSET>(), CompileWithDebug: CompileWithDebug::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcCompiler2 as windows_core::Interface>::IID || iid == &<IDxcCompiler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcCompiler2 {}
windows_core::imp::define_interface!(IDxcCompiler3, IDxcCompiler3_Vtbl, 0x228b4687_5a6a_4730_900c_9702b2203f54);
windows_core::imp::interface_hierarchy!(IDxcCompiler3, windows_core::IUnknown);
impl IDxcCompiler3 {
    pub unsafe fn Compile<P3, T>(&self, psource: *const DxcBuffer, parguments: Option<&[windows_core::PCWSTR]>, pincludehandler: P3) -> windows_core::Result<T>
    where
        P3: windows_core::Param<IDxcIncludeHandler>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Compile)(windows_core::Interface::as_raw(self), psource, core::mem::transmute(parguments.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pincludehandler.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn Disassemble<T>(&self, pobject: *const DxcBuffer) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Disassemble)(windows_core::Interface::as_raw(self), pobject, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Compile: unsafe extern "system" fn(*mut core::ffi::c_void, *const DxcBuffer, *const windows_core::PCWSTR, u32, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disassemble: unsafe extern "system" fn(*mut core::ffi::c_void, *const DxcBuffer, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxcCompiler3_Impl: windows_core::IUnknownImpl {
    fn Compile(&self, psource: *const DxcBuffer, parguments: *const windows_core::PCWSTR, argcount: u32, pincludehandler: windows_core::Ref<IDxcIncludeHandler>, riid: *const windows_core::GUID, ppresult: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Disassemble(&self, pobject: *const DxcBuffer, riid: *const windows_core::GUID, ppresult: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDxcCompiler3_Vtbl {
    pub const fn new<Identity: IDxcCompiler3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Compile<Identity: IDxcCompiler3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *const DxcBuffer, parguments: *const windows_core::PCWSTR, argcount: u32, pincludehandler: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcCompiler3_Impl::Compile(this, core::mem::transmute_copy(&psource), core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount), core::mem::transmute_copy(&pincludehandler), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppresult)).into()
            }
        }
        unsafe extern "system" fn Disassemble<Identity: IDxcCompiler3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *const DxcBuffer, riid: *const windows_core::GUID, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcCompiler3_Impl::Disassemble(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppresult)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compile: Compile::<Identity, OFFSET>,
            Disassemble: Disassemble::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcCompiler3 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcCompiler3 {}
windows_core::imp::define_interface!(IDxcCompilerArgs, IDxcCompilerArgs_Vtbl, 0x73effe2a_70dc_45f8_9690_eff64c02429d);
windows_core::imp::interface_hierarchy!(IDxcCompilerArgs, windows_core::IUnknown);
impl IDxcCompilerArgs {
    pub unsafe fn GetArguments(&self) -> *mut windows_core::PCWSTR {
        unsafe { (windows_core::Interface::vtable(self).GetArguments)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AddArguments(&self, parguments: Option<&[windows_core::PCWSTR]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddArguments)(windows_core::Interface::as_raw(self), core::mem::transmute(parguments.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
    }
    pub unsafe fn AddArgumentsUTF8(&self, parguments: Option<&[windows_core::PCSTR]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddArgumentsUTF8)(windows_core::Interface::as_raw(self), core::mem::transmute(parguments.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
    }
    pub unsafe fn AddDefines(&self, pdefines: &[DxcDefine]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddDefines)(windows_core::Interface::as_raw(self), core::mem::transmute(pdefines.as_ptr()), pdefines.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompilerArgs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetArguments: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut windows_core::PCWSTR,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub AddArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub AddArgumentsUTF8: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCSTR, u32) -> windows_core::HRESULT,
    pub AddDefines: unsafe extern "system" fn(*mut core::ffi::c_void, *const DxcDefine, u32) -> windows_core::HRESULT,
}
pub trait IDxcCompilerArgs_Impl: windows_core::IUnknownImpl {
    fn GetArguments(&self) -> *mut windows_core::PCWSTR;
    fn GetCount(&self) -> u32;
    fn AddArguments(&self, parguments: *const windows_core::PCWSTR, argcount: u32) -> windows_core::Result<()>;
    fn AddArgumentsUTF8(&self, parguments: *const windows_core::PCSTR, argcount: u32) -> windows_core::Result<()>;
    fn AddDefines(&self, pdefines: *const DxcDefine, definecount: u32) -> windows_core::Result<()>;
}
impl IDxcCompilerArgs_Vtbl {
    pub const fn new<Identity: IDxcCompilerArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetArguments<Identity: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut windows_core::PCWSTR {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcCompilerArgs_Impl::GetArguments(this)
            }
        }
        unsafe extern "system" fn GetCount<Identity: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcCompilerArgs_Impl::GetCount(this)
            }
        }
        unsafe extern "system" fn AddArguments<Identity: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parguments: *const windows_core::PCWSTR, argcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcCompilerArgs_Impl::AddArguments(this, core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount)).into()
            }
        }
        unsafe extern "system" fn AddArgumentsUTF8<Identity: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parguments: *const windows_core::PCSTR, argcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcCompilerArgs_Impl::AddArgumentsUTF8(this, core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount)).into()
            }
        }
        unsafe extern "system" fn AddDefines<Identity: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdefines: *const DxcDefine, definecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcCompilerArgs_Impl::AddDefines(this, core::mem::transmute_copy(&pdefines), core::mem::transmute_copy(&definecount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetArguments: GetArguments::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            AddArguments: AddArguments::<Identity, OFFSET>,
            AddArgumentsUTF8: AddArgumentsUTF8::<Identity, OFFSET>,
            AddDefines: AddDefines::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcCompilerArgs as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcCompilerArgs {}
windows_core::imp::define_interface!(IDxcContainerBuilder, IDxcContainerBuilder_Vtbl, 0x334b1f50_2292_4b35_99a1_25588d8c17fe);
windows_core::imp::interface_hierarchy!(IDxcContainerBuilder, windows_core::IUnknown);
impl IDxcContainerBuilder {
    pub unsafe fn Load<P0>(&self, pdxilcontainerheader: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pdxilcontainerheader.param().abi()).ok() }
    }
    pub unsafe fn AddPart<P1>(&self, fourcc: u32, psource: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IDxcBlob>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPart)(windows_core::Interface::as_raw(self), fourcc, psource.param().abi()).ok() }
    }
    pub unsafe fn RemovePart(&self, fourcc: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemovePart)(windows_core::Interface::as_raw(self), fourcc).ok() }
    }
    pub unsafe fn SerializeContainer(&self) -> windows_core::Result<IDxcOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SerializeContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerBuilder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddPart: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemovePart: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SerializeContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxcContainerBuilder_Impl: windows_core::IUnknownImpl {
    fn Load(&self, pdxilcontainerheader: windows_core::Ref<IDxcBlob>) -> windows_core::Result<()>;
    fn AddPart(&self, fourcc: u32, psource: windows_core::Ref<IDxcBlob>) -> windows_core::Result<()>;
    fn RemovePart(&self, fourcc: u32) -> windows_core::Result<()>;
    fn SerializeContainer(&self) -> windows_core::Result<IDxcOperationResult>;
}
impl IDxcContainerBuilder_Vtbl {
    pub const fn new<Identity: IDxcContainerBuilder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Load<Identity: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdxilcontainerheader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcContainerBuilder_Impl::Load(this, core::mem::transmute_copy(&pdxilcontainerheader)).into()
            }
        }
        unsafe extern "system" fn AddPart<Identity: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fourcc: u32, psource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcContainerBuilder_Impl::AddPart(this, core::mem::transmute_copy(&fourcc), core::mem::transmute_copy(&psource)).into()
            }
        }
        unsafe extern "system" fn RemovePart<Identity: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fourcc: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcContainerBuilder_Impl::RemovePart(this, core::mem::transmute_copy(&fourcc)).into()
            }
        }
        unsafe extern "system" fn SerializeContainer<Identity: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcContainerBuilder_Impl::SerializeContainer(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Load: Load::<Identity, OFFSET>,
            AddPart: AddPart::<Identity, OFFSET>,
            RemovePart: RemovePart::<Identity, OFFSET>,
            SerializeContainer: SerializeContainer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcContainerBuilder as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcContainerBuilder {}
windows_core::imp::define_interface!(IDxcContainerReflection, IDxcContainerReflection_Vtbl, 0xd2c21b26_8350_4bdc_976a_331ce6f4c54c);
windows_core::imp::interface_hierarchy!(IDxcContainerReflection, windows_core::IUnknown);
impl IDxcContainerReflection {
    pub unsafe fn Load<P0>(&self, pcontainer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pcontainer.param().abi()).ok() }
    }
    pub unsafe fn GetPartCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPartKind(&self, idx: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartKind)(windows_core::Interface::as_raw(self), idx, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPartContent(&self, idx: u32) -> windows_core::Result<IDxcBlob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartContent)(windows_core::Interface::as_raw(self), idx, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindFirstPartKind(&self, kind: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirstPartKind)(windows_core::Interface::as_raw(self), kind, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPartReflection(&self, idx: u32, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPartReflection)(windows_core::Interface::as_raw(self), idx, iid, ppvobject as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerReflection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPartCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPartKind: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetPartContent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFirstPartKind: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetPartReflection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxcContainerReflection_Impl: windows_core::IUnknownImpl {
    fn Load(&self, pcontainer: windows_core::Ref<IDxcBlob>) -> windows_core::Result<()>;
    fn GetPartCount(&self) -> windows_core::Result<u32>;
    fn GetPartKind(&self, idx: u32) -> windows_core::Result<u32>;
    fn GetPartContent(&self, idx: u32) -> windows_core::Result<IDxcBlob>;
    fn FindFirstPartKind(&self, kind: u32) -> windows_core::Result<u32>;
    fn GetPartReflection(&self, idx: u32, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDxcContainerReflection_Vtbl {
    pub const fn new<Identity: IDxcContainerReflection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Load<Identity: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontainer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcContainerReflection_Impl::Load(this, core::mem::transmute_copy(&pcontainer)).into()
            }
        }
        unsafe extern "system" fn GetPartCount<Identity: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcContainerReflection_Impl::GetPartCount(this) {
                    Ok(ok__) => {
                        presult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPartKind<Identity: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idx: u32, presult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcContainerReflection_Impl::GetPartKind(this, core::mem::transmute_copy(&idx)) {
                    Ok(ok__) => {
                        presult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPartContent<Identity: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idx: u32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcContainerReflection_Impl::GetPartContent(this, core::mem::transmute_copy(&idx)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFirstPartKind<Identity: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kind: u32, presult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcContainerReflection_Impl::FindFirstPartKind(this, core::mem::transmute_copy(&kind)) {
                    Ok(ok__) => {
                        presult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPartReflection<Identity: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idx: u32, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcContainerReflection_Impl::GetPartReflection(this, core::mem::transmute_copy(&idx), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Load: Load::<Identity, OFFSET>,
            GetPartCount: GetPartCount::<Identity, OFFSET>,
            GetPartKind: GetPartKind::<Identity, OFFSET>,
            GetPartContent: GetPartContent::<Identity, OFFSET>,
            FindFirstPartKind: FindFirstPartKind::<Identity, OFFSET>,
            GetPartReflection: GetPartReflection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcContainerReflection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcContainerReflection {}
windows_core::imp::define_interface!(IDxcExtraOutputs, IDxcExtraOutputs_Vtbl, 0x319b37a2_a5c2_494a_a5de_4801b2faf989);
windows_core::imp::interface_hierarchy!(IDxcExtraOutputs, windows_core::IUnknown);
impl IDxcExtraOutputs {
    pub unsafe fn GetOutputCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetOutputCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetOutput<T>(&self, uindex: u32, ppoutputtype: Option<*mut Option<IDxcBlobUtf16>>, ppoutputname: Option<*mut Option<IDxcBlobUtf16>>, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).GetOutput)(windows_core::Interface::as_raw(self), uindex, &T::IID, result__ as *mut _ as *mut _, ppoutputtype.unwrap_or(core::mem::zeroed()) as _, ppoutputname.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcExtraOutputs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOutputCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetOutput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxcExtraOutputs_Impl: windows_core::IUnknownImpl {
    fn GetOutputCount(&self) -> u32;
    fn GetOutput(&self, uindex: u32, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void, ppoutputtype: windows_core::OutRef<IDxcBlobUtf16>, ppoutputname: windows_core::OutRef<IDxcBlobUtf16>) -> windows_core::Result<()>;
}
impl IDxcExtraOutputs_Vtbl {
    pub const fn new<Identity: IDxcExtraOutputs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOutputCount<Identity: IDxcExtraOutputs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcExtraOutputs_Impl::GetOutputCount(this)
            }
        }
        unsafe extern "system" fn GetOutput<Identity: IDxcExtraOutputs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void, ppoutputtype: *mut *mut core::ffi::c_void, ppoutputname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcExtraOutputs_Impl::GetOutput(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvobject), core::mem::transmute_copy(&ppoutputtype), core::mem::transmute_copy(&ppoutputname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOutputCount: GetOutputCount::<Identity, OFFSET>,
            GetOutput: GetOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcExtraOutputs as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcExtraOutputs {}
windows_core::imp::define_interface!(IDxcIncludeHandler, IDxcIncludeHandler_Vtbl, 0x7f61fc7d_950d_467f_b3e3_3c02fb49187c);
windows_core::imp::interface_hierarchy!(IDxcIncludeHandler, windows_core::IUnknown);
impl IDxcIncludeHandler {
    pub unsafe fn LoadSource<P0>(&self, pfilename: P0) -> windows_core::Result<IDxcBlob>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadSource)(windows_core::Interface::as_raw(self), pfilename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcIncludeHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LoadSource: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxcIncludeHandler_Impl: windows_core::IUnknownImpl {
    fn LoadSource(&self, pfilename: &windows_core::PCWSTR) -> windows_core::Result<IDxcBlob>;
}
impl IDxcIncludeHandler_Vtbl {
    pub const fn new<Identity: IDxcIncludeHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadSource<Identity: IDxcIncludeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilename: windows_core::PCWSTR, ppincludesource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcIncludeHandler_Impl::LoadSource(this, core::mem::transmute(&pfilename)) {
                    Ok(ok__) => {
                        ppincludesource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LoadSource: LoadSource::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcIncludeHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcIncludeHandler {}
windows_core::imp::define_interface!(IDxcLibrary, IDxcLibrary_Vtbl, 0xe5204dc7_d18c_4c3c_bdfb_851673980fe7);
windows_core::imp::interface_hierarchy!(IDxcLibrary, windows_core::IUnknown);
impl IDxcLibrary {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMalloc<P0>(&self, pmalloc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IMalloc>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMalloc)(windows_core::Interface::as_raw(self), pmalloc.param().abi()).ok() }
    }
    pub unsafe fn CreateBlobFromBlob<P0>(&self, pblob: P0, offset: u32, length: u32) -> windows_core::Result<IDxcBlob>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlobFromBlob)(windows_core::Interface::as_raw(self), pblob.param().abi(), offset, length, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBlobFromFile<P0>(&self, pfilename: P0, codepage: Option<*const DXC_CP>) -> windows_core::Result<IDxcBlobEncoding>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlobFromFile)(windows_core::Interface::as_raw(self), pfilename.param().abi(), codepage.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBlobWithEncodingFromPinned(&self, ptext: *const core::ffi::c_void, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlobWithEncodingFromPinned)(windows_core::Interface::as_raw(self), ptext, size, codepage, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBlobWithEncodingOnHeapCopy(&self, ptext: *const core::ffi::c_void, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlobWithEncodingOnHeapCopy)(windows_core::Interface::as_raw(self), ptext, size, codepage, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBlobWithEncodingOnMalloc<P1>(&self, ptext: *const core::ffi::c_void, pimalloc: P1, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding>
    where
        P1: windows_core::Param<super::super::super::System::Com::IMalloc>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlobWithEncodingOnMalloc)(windows_core::Interface::as_raw(self), ptext, pimalloc.param().abi(), size, codepage, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateIncludeHandler(&self) -> windows_core::Result<IDxcIncludeHandler> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateIncludeHandler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStreamFromBlobReadOnly<P0>(&self, pblob: P0) -> windows_core::Result<super::super::super::System::Com::IStream>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStreamFromBlobReadOnly)(windows_core::Interface::as_raw(self), pblob.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBlobAsUtf8<P0>(&self, pblob: P0) -> windows_core::Result<IDxcBlobEncoding>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBlobAsUtf8)(windows_core::Interface::as_raw(self), pblob.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBlobAsUtf16<P0>(&self, pblob: P0) -> windows_core::Result<IDxcBlobEncoding>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBlobAsUtf16)(windows_core::Interface::as_raw(self), pblob.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLibrary_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMalloc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMalloc: usize,
    pub CreateBlobFromBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBlobFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const DXC_CP, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBlobWithEncodingFromPinned: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, DXC_CP, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBlobWithEncodingOnHeapCopy: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, DXC_CP, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBlobWithEncodingOnMalloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut core::ffi::c_void, u32, DXC_CP, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBlobWithEncodingOnMalloc: usize,
    pub CreateIncludeHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStreamFromBlobReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStreamFromBlobReadOnly: usize,
    pub GetBlobAsUtf8: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBlobAsUtf16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDxcLibrary_Impl: windows_core::IUnknownImpl {
    fn SetMalloc(&self, pmalloc: windows_core::Ref<super::super::super::System::Com::IMalloc>) -> windows_core::Result<()>;
    fn CreateBlobFromBlob(&self, pblob: windows_core::Ref<IDxcBlob>, offset: u32, length: u32) -> windows_core::Result<IDxcBlob>;
    fn CreateBlobFromFile(&self, pfilename: &windows_core::PCWSTR, codepage: *const DXC_CP) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingFromPinned(&self, ptext: *const core::ffi::c_void, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingOnHeapCopy(&self, ptext: *const core::ffi::c_void, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingOnMalloc(&self, ptext: *const core::ffi::c_void, pimalloc: windows_core::Ref<super::super::super::System::Com::IMalloc>, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateIncludeHandler(&self) -> windows_core::Result<IDxcIncludeHandler>;
    fn CreateStreamFromBlobReadOnly(&self, pblob: windows_core::Ref<IDxcBlob>) -> windows_core::Result<super::super::super::System::Com::IStream>;
    fn GetBlobAsUtf8(&self, pblob: windows_core::Ref<IDxcBlob>) -> windows_core::Result<IDxcBlobEncoding>;
    fn GetBlobAsUtf16(&self, pblob: windows_core::Ref<IDxcBlob>) -> windows_core::Result<IDxcBlobEncoding>;
}
#[cfg(feature = "Win32_System_Com")]
impl IDxcLibrary_Vtbl {
    pub const fn new<Identity: IDxcLibrary_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMalloc<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmalloc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcLibrary_Impl::SetMalloc(this, core::mem::transmute_copy(&pmalloc)).into()
            }
        }
        unsafe extern "system" fn CreateBlobFromBlob<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, offset: u32, length: u32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::CreateBlobFromBlob(this, core::mem::transmute_copy(&pblob), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&length)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBlobFromFile<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilename: windows_core::PCWSTR, codepage: *const DXC_CP, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::CreateBlobFromFile(this, core::mem::transmute(&pfilename), core::mem::transmute_copy(&codepage)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingFromPinned<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: *const core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::CreateBlobWithEncodingFromPinned(this, core::mem::transmute_copy(&ptext), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingOnHeapCopy<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: *const core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::CreateBlobWithEncodingOnHeapCopy(this, core::mem::transmute_copy(&ptext), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingOnMalloc<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: *const core::ffi::c_void, pimalloc: *mut core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::CreateBlobWithEncodingOnMalloc(this, core::mem::transmute_copy(&ptext), core::mem::transmute_copy(&pimalloc), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateIncludeHandler<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::CreateIncludeHandler(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStreamFromBlobReadOnly<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::CreateStreamFromBlobReadOnly(this, core::mem::transmute_copy(&pblob)) {
                    Ok(ok__) => {
                        ppstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBlobAsUtf8<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::GetBlobAsUtf8(this, core::mem::transmute_copy(&pblob)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBlobAsUtf16<Identity: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLibrary_Impl::GetBlobAsUtf16(this, core::mem::transmute_copy(&pblob)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMalloc: SetMalloc::<Identity, OFFSET>,
            CreateBlobFromBlob: CreateBlobFromBlob::<Identity, OFFSET>,
            CreateBlobFromFile: CreateBlobFromFile::<Identity, OFFSET>,
            CreateBlobWithEncodingFromPinned: CreateBlobWithEncodingFromPinned::<Identity, OFFSET>,
            CreateBlobWithEncodingOnHeapCopy: CreateBlobWithEncodingOnHeapCopy::<Identity, OFFSET>,
            CreateBlobWithEncodingOnMalloc: CreateBlobWithEncodingOnMalloc::<Identity, OFFSET>,
            CreateIncludeHandler: CreateIncludeHandler::<Identity, OFFSET>,
            CreateStreamFromBlobReadOnly: CreateStreamFromBlobReadOnly::<Identity, OFFSET>,
            GetBlobAsUtf8: GetBlobAsUtf8::<Identity, OFFSET>,
            GetBlobAsUtf16: GetBlobAsUtf16::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcLibrary as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDxcLibrary {}
windows_core::imp::define_interface!(IDxcLinker, IDxcLinker_Vtbl, 0xf1b5be2a_62dd_4327_a1c2_42ac1e1e78e6);
windows_core::imp::interface_hierarchy!(IDxcLinker, windows_core::IUnknown);
impl IDxcLinker {
    pub unsafe fn RegisterLibrary<P0, P1>(&self, plibname: P0, plib: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IDxcBlob>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterLibrary)(windows_core::Interface::as_raw(self), plibname.param().abi(), plib.param().abi()).ok() }
    }
    pub unsafe fn Link<P0, P1>(&self, pentryname: P0, ptargetprofile: P1, plibnames: &[windows_core::PCWSTR], parguments: Option<&[windows_core::PCWSTR]>) -> windows_core::Result<IDxcOperationResult>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Link)(windows_core::Interface::as_raw(self), pentryname.param().abi(), ptargetprofile.param().abi(), core::mem::transmute(plibnames.as_ptr()), plibnames.len().try_into().unwrap(), core::mem::transmute(parguments.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLinker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterLibrary: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Link: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::PCWSTR, u32, *const windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxcLinker_Impl: windows_core::IUnknownImpl {
    fn RegisterLibrary(&self, plibname: &windows_core::PCWSTR, plib: windows_core::Ref<IDxcBlob>) -> windows_core::Result<()>;
    fn Link(&self, pentryname: &windows_core::PCWSTR, ptargetprofile: &windows_core::PCWSTR, plibnames: *const windows_core::PCWSTR, libcount: u32, parguments: *const windows_core::PCWSTR, argcount: u32) -> windows_core::Result<IDxcOperationResult>;
}
impl IDxcLinker_Vtbl {
    pub const fn new<Identity: IDxcLinker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterLibrary<Identity: IDxcLinker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plibname: windows_core::PCWSTR, plib: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcLinker_Impl::RegisterLibrary(this, core::mem::transmute(&plibname), core::mem::transmute_copy(&plib)).into()
            }
        }
        unsafe extern "system" fn Link<Identity: IDxcLinker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pentryname: windows_core::PCWSTR, ptargetprofile: windows_core::PCWSTR, plibnames: *const windows_core::PCWSTR, libcount: u32, parguments: *const windows_core::PCWSTR, argcount: u32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcLinker_Impl::Link(this, core::mem::transmute(&pentryname), core::mem::transmute(&ptargetprofile), core::mem::transmute_copy(&plibnames), core::mem::transmute_copy(&libcount), core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterLibrary: RegisterLibrary::<Identity, OFFSET>,
            Link: Link::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcLinker as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcLinker {}
windows_core::imp::define_interface!(IDxcOperationResult, IDxcOperationResult_Vtbl, 0xcedb484a_d4e9_445a_b991_ca21ca157dc2);
windows_core::imp::interface_hierarchy!(IDxcOperationResult, windows_core::IUnknown);
impl IDxcOperationResult {
    pub unsafe fn GetStatus(&self) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetResult(&self) -> windows_core::Result<IDxcBlob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResult)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetErrorBuffer(&self) -> windows_core::Result<IDxcBlobEncoding> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorBuffer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOperationResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetErrorBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxcOperationResult_Impl: windows_core::IUnknownImpl {
    fn GetStatus(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn GetResult(&self) -> windows_core::Result<IDxcBlob>;
    fn GetErrorBuffer(&self) -> windows_core::Result<IDxcBlobEncoding>;
}
impl IDxcOperationResult_Vtbl {
    pub const fn new<Identity: IDxcOperationResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStatus<Identity: IDxcOperationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcOperationResult_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        pstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResult<Identity: IDxcOperationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcOperationResult_Impl::GetResult(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetErrorBuffer<Identity: IDxcOperationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperrors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcOperationResult_Impl::GetErrorBuffer(this) {
                    Ok(ok__) => {
                        pperrors.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetResult: GetResult::<Identity, OFFSET>,
            GetErrorBuffer: GetErrorBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcOperationResult as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcOperationResult {}
windows_core::imp::define_interface!(IDxcOptimizer, IDxcOptimizer_Vtbl, 0x25740e2e_9cba_401b_9119_4fb42f39f270);
windows_core::imp::interface_hierarchy!(IDxcOptimizer, windows_core::IUnknown);
impl IDxcOptimizer {
    pub unsafe fn GetAvailablePassCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAvailablePassCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAvailablePass(&self, index: u32) -> windows_core::Result<IDxcOptimizerPass> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAvailablePass)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RunOptimizer<P0>(&self, pblob: P0, ppoptions: &[windows_core::PCWSTR], poutputmodule: *mut Option<IDxcBlob>, ppoutputtext: Option<*mut Option<IDxcBlobEncoding>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe { (windows_core::Interface::vtable(self).RunOptimizer)(windows_core::Interface::as_raw(self), pblob.param().abi(), core::mem::transmute(ppoptions.as_ptr()), ppoptions.len().try_into().unwrap(), core::mem::transmute(poutputmodule), ppoutputtext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAvailablePassCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAvailablePass: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RunOptimizer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxcOptimizer_Impl: windows_core::IUnknownImpl {
    fn GetAvailablePassCount(&self) -> windows_core::Result<u32>;
    fn GetAvailablePass(&self, index: u32) -> windows_core::Result<IDxcOptimizerPass>;
    fn RunOptimizer(&self, pblob: windows_core::Ref<IDxcBlob>, ppoptions: *const windows_core::PCWSTR, optioncount: u32, poutputmodule: windows_core::OutRef<IDxcBlob>, ppoutputtext: windows_core::OutRef<IDxcBlobEncoding>) -> windows_core::Result<()>;
}
impl IDxcOptimizer_Vtbl {
    pub const fn new<Identity: IDxcOptimizer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAvailablePassCount<Identity: IDxcOptimizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcOptimizer_Impl::GetAvailablePassCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAvailablePass<Identity: IDxcOptimizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcOptimizer_Impl::GetAvailablePass(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RunOptimizer<Identity: IDxcOptimizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, ppoptions: *const windows_core::PCWSTR, optioncount: u32, poutputmodule: *mut *mut core::ffi::c_void, ppoutputtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcOptimizer_Impl::RunOptimizer(this, core::mem::transmute_copy(&pblob), core::mem::transmute_copy(&ppoptions), core::mem::transmute_copy(&optioncount), core::mem::transmute_copy(&poutputmodule), core::mem::transmute_copy(&ppoutputtext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAvailablePassCount: GetAvailablePassCount::<Identity, OFFSET>,
            GetAvailablePass: GetAvailablePass::<Identity, OFFSET>,
            RunOptimizer: RunOptimizer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcOptimizer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcOptimizer {}
windows_core::imp::define_interface!(IDxcOptimizerPass, IDxcOptimizerPass_Vtbl, 0xae2cd79f_cc22_453f_9b6b_b124e7a5204c);
windows_core::imp::interface_hierarchy!(IDxcOptimizerPass, windows_core::IUnknown);
impl IDxcOptimizerPass {
    pub unsafe fn GetOptionName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOptionArgCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionArgCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOptionArgName(&self, argindex: u32) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionArgName)(windows_core::Interface::as_raw(self), argindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOptionArgDescription(&self, argindex: u32) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionArgDescription)(windows_core::Interface::as_raw(self), argindex, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizerPass_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOptionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetOptionArgCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOptionArgName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetOptionArgDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IDxcOptimizerPass_Impl: windows_core::IUnknownImpl {
    fn GetOptionName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetOptionArgCount(&self) -> windows_core::Result<u32>;
    fn GetOptionArgName(&self, argindex: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn GetOptionArgDescription(&self, argindex: u32) -> windows_core::Result<windows_core::PWSTR>;
}
impl IDxcOptimizerPass_Vtbl {
    pub const fn new<Identity: IDxcOptimizerPass_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOptionName<Identity: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcOptimizerPass_Impl::GetOptionName(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDescription<Identity: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcOptimizerPass_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOptionArgCount<Identity: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcOptimizerPass_Impl::GetOptionArgCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOptionArgName<Identity: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, argindex: u32, ppresult: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcOptimizerPass_Impl::GetOptionArgName(this, core::mem::transmute_copy(&argindex)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOptionArgDescription<Identity: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, argindex: u32, ppresult: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcOptimizerPass_Impl::GetOptionArgDescription(this, core::mem::transmute_copy(&argindex)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOptionName: GetOptionName::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetOptionArgCount: GetOptionArgCount::<Identity, OFFSET>,
            GetOptionArgName: GetOptionArgName::<Identity, OFFSET>,
            GetOptionArgDescription: GetOptionArgDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcOptimizerPass as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcOptimizerPass {}
windows_core::imp::define_interface!(IDxcPdbUtils, IDxcPdbUtils_Vtbl, 0xe6c9647e_9d6a_4c3b_b94c_524b5a6c343d);
windows_core::imp::interface_hierarchy!(IDxcPdbUtils, windows_core::IUnknown);
impl IDxcPdbUtils {
    pub unsafe fn Load<P0>(&self, ppdbordxil: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), ppdbordxil.param().abi()).ok() }
    }
    pub unsafe fn GetSourceCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSource(&self, uindex: u32) -> windows_core::Result<IDxcBlobEncoding> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), uindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSourceName(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceName)(windows_core::Interface::as_raw(self), uindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetFlagCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlagCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFlag(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlag)(windows_core::Interface::as_raw(self), uindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetArgCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetArgCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetArg(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetArg)(windows_core::Interface::as_raw(self), uindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetArgPairCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetArgPairCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetArgPair(&self, uindex: u32, pname: *mut windows_core::BSTR, pvalue: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetArgPair)(windows_core::Interface::as_raw(self), uindex, core::mem::transmute(pname), core::mem::transmute(pvalue)).ok() }
    }
    pub unsafe fn GetDefineCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefineCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDefine(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefine)(windows_core::Interface::as_raw(self), uindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetTargetProfile(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTargetProfile)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetEntryPoint(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEntryPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetMainFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMainFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetHash(&self) -> windows_core::Result<IDxcBlob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHash)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsFullPDB(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsFullPDB)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFullPDB(&self) -> windows_core::Result<IDxcBlob> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFullPDB)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetVersionInfo(&self) -> windows_core::Result<IDxcVersionInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVersionInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetCompiler<P0>(&self, pcompiler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDxcCompiler3>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCompiler)(windows_core::Interface::as_raw(self), pcompiler.param().abi()).ok() }
    }
    pub unsafe fn CompileForFullPDB(&self) -> windows_core::Result<IDxcResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompileForFullPDB)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OverrideArgs(&self, pargpairs: *const DxcArgPair, unumargpairs: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OverrideArgs)(windows_core::Interface::as_raw(self), pargpairs, unumargpairs).ok() }
    }
    pub unsafe fn OverrideRootSignature<P0>(&self, prootsignature: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OverrideRootSignature)(windows_core::Interface::as_raw(self), prootsignature.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcPdbUtils_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSourceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSourceName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFlagCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFlag: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetArgCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetArg: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetArgPairCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetArgPair: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefineCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDefine: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTargetProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEntryPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMainFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsFullPDB: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetFullPDB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVersionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCompiler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CompileForFullPDB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OverrideArgs: unsafe extern "system" fn(*mut core::ffi::c_void, *const DxcArgPair, u32) -> windows_core::HRESULT,
    pub OverrideRootSignature: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IDxcPdbUtils_Impl: windows_core::IUnknownImpl {
    fn Load(&self, ppdbordxil: windows_core::Ref<IDxcBlob>) -> windows_core::Result<()>;
    fn GetSourceCount(&self) -> windows_core::Result<u32>;
    fn GetSource(&self, uindex: u32) -> windows_core::Result<IDxcBlobEncoding>;
    fn GetSourceName(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetFlagCount(&self) -> windows_core::Result<u32>;
    fn GetFlag(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetArgCount(&self) -> windows_core::Result<u32>;
    fn GetArg(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetArgPairCount(&self) -> windows_core::Result<u32>;
    fn GetArgPair(&self, uindex: u32, pname: *mut windows_core::BSTR, pvalue: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDefineCount(&self) -> windows_core::Result<u32>;
    fn GetDefine(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetTargetProfile(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetEntryPoint(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetMainFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHash(&self) -> windows_core::Result<IDxcBlob>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsFullPDB(&self) -> windows_core::BOOL;
    fn GetFullPDB(&self) -> windows_core::Result<IDxcBlob>;
    fn GetVersionInfo(&self) -> windows_core::Result<IDxcVersionInfo>;
    fn SetCompiler(&self, pcompiler: windows_core::Ref<IDxcCompiler3>) -> windows_core::Result<()>;
    fn CompileForFullPDB(&self) -> windows_core::Result<IDxcResult>;
    fn OverrideArgs(&self, pargpairs: *const DxcArgPair, unumargpairs: u32) -> windows_core::Result<()>;
    fn OverrideRootSignature(&self, prootsignature: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IDxcPdbUtils_Vtbl {
    pub const fn new<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Load<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdbordxil: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils_Impl::Load(this, core::mem::transmute_copy(&ppdbordxil)).into()
            }
        }
        unsafe extern "system" fn GetSourceCount<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetSourceCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSource<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetSource(this, core::mem::transmute_copy(&uindex)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSourceName<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, presult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetSourceName(this, core::mem::transmute_copy(&uindex)) {
                    Ok(ok__) => {
                        presult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFlagCount<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetFlagCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFlag<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, presult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetFlag(this, core::mem::transmute_copy(&uindex)) {
                    Ok(ok__) => {
                        presult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetArgCount<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetArgCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetArg<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, presult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetArg(this, core::mem::transmute_copy(&uindex)) {
                    Ok(ok__) => {
                        presult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetArgPairCount<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetArgPairCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetArgPair<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, pname: *mut *mut core::ffi::c_void, pvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils_Impl::GetArgPair(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&pname), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetDefineCount<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetDefineCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDefine<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, presult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetDefine(this, core::mem::transmute_copy(&uindex)) {
                    Ok(ok__) => {
                        presult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTargetProfile<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetTargetProfile(this) {
                    Ok(ok__) => {
                        presult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEntryPoint<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetEntryPoint(this) {
                    Ok(ok__) => {
                        presult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMainFileName<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetMainFileName(this) {
                    Ok(ok__) => {
                        presult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHash<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetHash(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetName<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetName(this) {
                    Ok(ok__) => {
                        presult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsFullPDB<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils_Impl::IsFullPDB(this)
            }
        }
        unsafe extern "system" fn GetFullPDB<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfullpdb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetFullPDB(this) {
                    Ok(ok__) => {
                        ppfullpdb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVersionInfo<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppversioninfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::GetVersionInfo(this) {
                    Ok(ok__) => {
                        ppversioninfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCompiler<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcompiler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils_Impl::SetCompiler(this, core::mem::transmute_copy(&pcompiler)).into()
            }
        }
        unsafe extern "system" fn CompileForFullPDB<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcPdbUtils_Impl::CompileForFullPDB(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OverrideArgs<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pargpairs: *const DxcArgPair, unumargpairs: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils_Impl::OverrideArgs(this, core::mem::transmute_copy(&pargpairs), core::mem::transmute_copy(&unumargpairs)).into()
            }
        }
        unsafe extern "system" fn OverrideRootSignature<Identity: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prootsignature: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcPdbUtils_Impl::OverrideRootSignature(this, core::mem::transmute(&prootsignature)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Load: Load::<Identity, OFFSET>,
            GetSourceCount: GetSourceCount::<Identity, OFFSET>,
            GetSource: GetSource::<Identity, OFFSET>,
            GetSourceName: GetSourceName::<Identity, OFFSET>,
            GetFlagCount: GetFlagCount::<Identity, OFFSET>,
            GetFlag: GetFlag::<Identity, OFFSET>,
            GetArgCount: GetArgCount::<Identity, OFFSET>,
            GetArg: GetArg::<Identity, OFFSET>,
            GetArgPairCount: GetArgPairCount::<Identity, OFFSET>,
            GetArgPair: GetArgPair::<Identity, OFFSET>,
            GetDefineCount: GetDefineCount::<Identity, OFFSET>,
            GetDefine: GetDefine::<Identity, OFFSET>,
            GetTargetProfile: GetTargetProfile::<Identity, OFFSET>,
            GetEntryPoint: GetEntryPoint::<Identity, OFFSET>,
            GetMainFileName: GetMainFileName::<Identity, OFFSET>,
            GetHash: GetHash::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            IsFullPDB: IsFullPDB::<Identity, OFFSET>,
            GetFullPDB: GetFullPDB::<Identity, OFFSET>,
            GetVersionInfo: GetVersionInfo::<Identity, OFFSET>,
            SetCompiler: SetCompiler::<Identity, OFFSET>,
            CompileForFullPDB: CompileForFullPDB::<Identity, OFFSET>,
            OverrideArgs: OverrideArgs::<Identity, OFFSET>,
            OverrideRootSignature: OverrideRootSignature::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcPdbUtils as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcPdbUtils {}
windows_core::imp::define_interface!(IDxcResult, IDxcResult_Vtbl, 0x58346cda_dde7_4497_9461_6f87af5e0659);
impl core::ops::Deref for IDxcResult {
    type Target = IDxcOperationResult;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDxcResult, windows_core::IUnknown, IDxcOperationResult);
impl IDxcResult {
    pub unsafe fn HasOutput(&self, dxcoutkind: DXC_OUT_KIND) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).HasOutput)(windows_core::Interface::as_raw(self), dxcoutkind) }
    }
    pub unsafe fn GetOutput<T>(&self, dxcoutkind: DXC_OUT_KIND, ppoutputname: *mut Option<IDxcBlobUtf16>, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).GetOutput)(windows_core::Interface::as_raw(self), dxcoutkind, &T::IID, result__ as *mut _ as *mut _, core::mem::transmute(ppoutputname)).ok() }
    }
    pub unsafe fn GetNumOutputs(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetNumOutputs)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetOutputByIndex(&self, index: u32) -> DXC_OUT_KIND {
        unsafe { (windows_core::Interface::vtable(self).GetOutputByIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn PrimaryOutput(&self) -> DXC_OUT_KIND {
        unsafe { (windows_core::Interface::vtable(self).PrimaryOutput)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcResult_Vtbl {
    pub base__: IDxcOperationResult_Vtbl,
    pub HasOutput: unsafe extern "system" fn(*mut core::ffi::c_void, DXC_OUT_KIND) -> windows_core::BOOL,
    pub GetOutput: unsafe extern "system" fn(*mut core::ffi::c_void, DXC_OUT_KIND, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumOutputs: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetOutputByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> DXC_OUT_KIND,
    pub PrimaryOutput: unsafe extern "system" fn(*mut core::ffi::c_void) -> DXC_OUT_KIND,
}
pub trait IDxcResult_Impl: IDxcOperationResult_Impl {
    fn HasOutput(&self, dxcoutkind: DXC_OUT_KIND) -> windows_core::BOOL;
    fn GetOutput(&self, dxcoutkind: DXC_OUT_KIND, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void, ppoutputname: windows_core::OutRef<IDxcBlobUtf16>) -> windows_core::Result<()>;
    fn GetNumOutputs(&self) -> u32;
    fn GetOutputByIndex(&self, index: u32) -> DXC_OUT_KIND;
    fn PrimaryOutput(&self) -> DXC_OUT_KIND;
}
impl IDxcResult_Vtbl {
    pub const fn new<Identity: IDxcResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HasOutput<Identity: IDxcResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxcoutkind: DXC_OUT_KIND) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcResult_Impl::HasOutput(this, core::mem::transmute_copy(&dxcoutkind))
            }
        }
        unsafe extern "system" fn GetOutput<Identity: IDxcResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxcoutkind: DXC_OUT_KIND, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void, ppoutputname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcResult_Impl::GetOutput(this, core::mem::transmute_copy(&dxcoutkind), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvobject), core::mem::transmute_copy(&ppoutputname)).into()
            }
        }
        unsafe extern "system" fn GetNumOutputs<Identity: IDxcResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcResult_Impl::GetNumOutputs(this)
            }
        }
        unsafe extern "system" fn GetOutputByIndex<Identity: IDxcResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> DXC_OUT_KIND {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcResult_Impl::GetOutputByIndex(this, core::mem::transmute_copy(&index))
            }
        }
        unsafe extern "system" fn PrimaryOutput<Identity: IDxcResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DXC_OUT_KIND {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcResult_Impl::PrimaryOutput(this)
            }
        }
        Self {
            base__: IDxcOperationResult_Vtbl::new::<Identity, OFFSET>(),
            HasOutput: HasOutput::<Identity, OFFSET>,
            GetOutput: GetOutput::<Identity, OFFSET>,
            GetNumOutputs: GetNumOutputs::<Identity, OFFSET>,
            GetOutputByIndex: GetOutputByIndex::<Identity, OFFSET>,
            PrimaryOutput: PrimaryOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcResult as windows_core::Interface>::IID || iid == &<IDxcOperationResult as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcResult {}
windows_core::imp::define_interface!(IDxcUtils, IDxcUtils_Vtbl, 0x4605c4cb_2019_492a_ada4_65f20bb7d67f);
windows_core::imp::interface_hierarchy!(IDxcUtils, windows_core::IUnknown);
impl IDxcUtils {
    pub unsafe fn CreateBlobFromBlob<P0>(&self, pblob: P0, offset: u32, length: u32) -> windows_core::Result<IDxcBlob>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlobFromBlob)(windows_core::Interface::as_raw(self), pblob.param().abi(), offset, length, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBlobFromPinned(&self, pdata: *const core::ffi::c_void, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlobFromPinned)(windows_core::Interface::as_raw(self), pdata, size, codepage, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoveToBlob<P1>(&self, pdata: *const core::ffi::c_void, pimalloc: P1, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding>
    where
        P1: windows_core::Param<super::super::super::System::Com::IMalloc>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveToBlob)(windows_core::Interface::as_raw(self), pdata, pimalloc.param().abi(), size, codepage, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBlob(&self, pdata: *const core::ffi::c_void, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlob)(windows_core::Interface::as_raw(self), pdata, size, codepage, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn LoadFile<P0>(&self, pfilename: P0, pcodepage: Option<*const DXC_CP>) -> windows_core::Result<IDxcBlobEncoding>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadFile)(windows_core::Interface::as_raw(self), pfilename.param().abi(), pcodepage.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReadOnlyStreamFromBlob<P0>(&self, pblob: P0) -> windows_core::Result<super::super::super::System::Com::IStream>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateReadOnlyStreamFromBlob)(windows_core::Interface::as_raw(self), pblob.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDefaultIncludeHandler(&self) -> windows_core::Result<IDxcIncludeHandler> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDefaultIncludeHandler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBlobAsUtf8<P0>(&self, pblob: P0) -> windows_core::Result<IDxcBlobUtf8>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBlobAsUtf8)(windows_core::Interface::as_raw(self), pblob.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBlobAsUtf16<P0>(&self, pblob: P0) -> windows_core::Result<IDxcBlobUtf16>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBlobAsUtf16)(windows_core::Interface::as_raw(self), pblob.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDxilContainerPart(&self, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut core::ffi::c_void, ppartsizeinbytes: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDxilContainerPart)(windows_core::Interface::as_raw(self), pshader, dxcpart, pppartdata as _, ppartsizeinbytes as _).ok() }
    }
    pub unsafe fn CreateReflection(&self, pdata: *const DxcBuffer, iid: *const windows_core::GUID, ppvreflection: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateReflection)(windows_core::Interface::as_raw(self), pdata, iid, ppvreflection as _).ok() }
    }
    pub unsafe fn BuildArguments<P0, P1, P2>(&self, psourcename: P0, pentrypoint: P1, ptargetprofile: P2, parguments: Option<&[windows_core::PCWSTR]>, pdefines: &[DxcDefine]) -> windows_core::Result<IDxcCompilerArgs>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BuildArguments)(windows_core::Interface::as_raw(self), psourcename.param().abi(), pentrypoint.param().abi(), ptargetprofile.param().abi(), core::mem::transmute(parguments.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), parguments.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pdefines.as_ptr()), pdefines.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPDBContents<P0>(&self, ppdbblob: P0, pphash: *mut Option<IDxcBlob>, ppcontainer: *mut Option<IDxcBlob>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetPDBContents)(windows_core::Interface::as_raw(self), ppdbblob.param().abi(), core::mem::transmute(pphash), core::mem::transmute(ppcontainer)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcUtils_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateBlobFromBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBlobFromPinned: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, DXC_CP, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MoveToBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut core::ffi::c_void, u32, DXC_CP, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MoveToBlob: usize,
    pub CreateBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, DXC_CP, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const DXC_CP, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReadOnlyStreamFromBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReadOnlyStreamFromBlob: usize,
    pub CreateDefaultIncludeHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBlobAsUtf8: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBlobAsUtf16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDxilContainerPart: unsafe extern "system" fn(*mut core::ffi::c_void, *const DxcBuffer, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CreateReflection: unsafe extern "system" fn(*mut core::ffi::c_void, *const DxcBuffer, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BuildArguments: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::PCWSTR, u32, *const DxcDefine, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPDBContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDxcUtils_Impl: windows_core::IUnknownImpl {
    fn CreateBlobFromBlob(&self, pblob: windows_core::Ref<IDxcBlob>, offset: u32, length: u32) -> windows_core::Result<IDxcBlob>;
    fn CreateBlobFromPinned(&self, pdata: *const core::ffi::c_void, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding>;
    fn MoveToBlob(&self, pdata: *const core::ffi::c_void, pimalloc: windows_core::Ref<super::super::super::System::Com::IMalloc>, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateBlob(&self, pdata: *const core::ffi::c_void, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding>;
    fn LoadFile(&self, pfilename: &windows_core::PCWSTR, pcodepage: *const DXC_CP) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateReadOnlyStreamFromBlob(&self, pblob: windows_core::Ref<IDxcBlob>) -> windows_core::Result<super::super::super::System::Com::IStream>;
    fn CreateDefaultIncludeHandler(&self) -> windows_core::Result<IDxcIncludeHandler>;
    fn GetBlobAsUtf8(&self, pblob: windows_core::Ref<IDxcBlob>) -> windows_core::Result<IDxcBlobUtf8>;
    fn GetBlobAsUtf16(&self, pblob: windows_core::Ref<IDxcBlob>) -> windows_core::Result<IDxcBlobUtf16>;
    fn GetDxilContainerPart(&self, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut core::ffi::c_void, ppartsizeinbytes: *mut u32) -> windows_core::Result<()>;
    fn CreateReflection(&self, pdata: *const DxcBuffer, iid: *const windows_core::GUID, ppvreflection: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn BuildArguments(&self, psourcename: &windows_core::PCWSTR, pentrypoint: &windows_core::PCWSTR, ptargetprofile: &windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32) -> windows_core::Result<IDxcCompilerArgs>;
    fn GetPDBContents(&self, ppdbblob: windows_core::Ref<IDxcBlob>, pphash: windows_core::OutRef<IDxcBlob>, ppcontainer: windows_core::OutRef<IDxcBlob>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IDxcUtils_Vtbl {
    pub const fn new<Identity: IDxcUtils_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateBlobFromBlob<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, offset: u32, length: u32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::CreateBlobFromBlob(this, core::mem::transmute_copy(&pblob), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&length)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBlobFromPinned<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::CreateBlobFromPinned(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveToBlob<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, pimalloc: *mut core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::MoveToBlob(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&pimalloc), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBlob<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::CreateBlob(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoadFile<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilename: windows_core::PCWSTR, pcodepage: *const DXC_CP, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::LoadFile(this, core::mem::transmute(&pfilename), core::mem::transmute_copy(&pcodepage)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateReadOnlyStreamFromBlob<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::CreateReadOnlyStreamFromBlob(this, core::mem::transmute_copy(&pblob)) {
                    Ok(ok__) => {
                        ppstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDefaultIncludeHandler<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::CreateDefaultIncludeHandler(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBlobAsUtf8<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::GetBlobAsUtf8(this, core::mem::transmute_copy(&pblob)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBlobAsUtf16<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::GetBlobAsUtf16(this, core::mem::transmute_copy(&pblob)) {
                    Ok(ok__) => {
                        pblobencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDxilContainerPart<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut core::ffi::c_void, ppartsizeinbytes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcUtils_Impl::GetDxilContainerPart(this, core::mem::transmute_copy(&pshader), core::mem::transmute_copy(&dxcpart), core::mem::transmute_copy(&pppartdata), core::mem::transmute_copy(&ppartsizeinbytes)).into()
            }
        }
        unsafe extern "system" fn CreateReflection<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const DxcBuffer, iid: *const windows_core::GUID, ppvreflection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcUtils_Impl::CreateReflection(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvreflection)).into()
            }
        }
        unsafe extern "system" fn BuildArguments<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcename: windows_core::PCWSTR, pentrypoint: windows_core::PCWSTR, ptargetprofile: windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, ppargs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcUtils_Impl::BuildArguments(this, core::mem::transmute(&psourcename), core::mem::transmute(&pentrypoint), core::mem::transmute(&ptargetprofile), core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount), core::mem::transmute_copy(&pdefines), core::mem::transmute_copy(&definecount)) {
                    Ok(ok__) => {
                        ppargs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPDBContents<Identity: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdbblob: *mut core::ffi::c_void, pphash: *mut *mut core::ffi::c_void, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcUtils_Impl::GetPDBContents(this, core::mem::transmute_copy(&ppdbblob), core::mem::transmute_copy(&pphash), core::mem::transmute_copy(&ppcontainer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateBlobFromBlob: CreateBlobFromBlob::<Identity, OFFSET>,
            CreateBlobFromPinned: CreateBlobFromPinned::<Identity, OFFSET>,
            MoveToBlob: MoveToBlob::<Identity, OFFSET>,
            CreateBlob: CreateBlob::<Identity, OFFSET>,
            LoadFile: LoadFile::<Identity, OFFSET>,
            CreateReadOnlyStreamFromBlob: CreateReadOnlyStreamFromBlob::<Identity, OFFSET>,
            CreateDefaultIncludeHandler: CreateDefaultIncludeHandler::<Identity, OFFSET>,
            GetBlobAsUtf8: GetBlobAsUtf8::<Identity, OFFSET>,
            GetBlobAsUtf16: GetBlobAsUtf16::<Identity, OFFSET>,
            GetDxilContainerPart: GetDxilContainerPart::<Identity, OFFSET>,
            CreateReflection: CreateReflection::<Identity, OFFSET>,
            BuildArguments: BuildArguments::<Identity, OFFSET>,
            GetPDBContents: GetPDBContents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcUtils as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDxcUtils {}
windows_core::imp::define_interface!(IDxcValidator, IDxcValidator_Vtbl, 0xa6e82bd2_1fd7_4826_9811_2857e797f49a);
windows_core::imp::interface_hierarchy!(IDxcValidator, windows_core::IUnknown);
impl IDxcValidator {
    pub unsafe fn Validate<P0>(&self, pshader: P0, flags: u32) -> windows_core::Result<IDxcOperationResult>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Validate)(windows_core::Interface::as_raw(self), pshader.param().abi(), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Validate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxcValidator_Impl: windows_core::IUnknownImpl {
    fn Validate(&self, pshader: windows_core::Ref<IDxcBlob>, flags: u32) -> windows_core::Result<IDxcOperationResult>;
}
impl IDxcValidator_Vtbl {
    pub const fn new<Identity: IDxcValidator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Validate<Identity: IDxcValidator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void, flags: u32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcValidator_Impl::Validate(this, core::mem::transmute_copy(&pshader), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Validate: Validate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcValidator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcValidator {}
windows_core::imp::define_interface!(IDxcValidator2, IDxcValidator2_Vtbl, 0x458e1fd1_b1b2_4750_a6e1_9c10f03bed92);
impl core::ops::Deref for IDxcValidator2 {
    type Target = IDxcValidator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDxcValidator2, windows_core::IUnknown, IDxcValidator);
impl IDxcValidator2 {
    pub unsafe fn ValidateWithDebug<P0>(&self, pshader: P0, flags: u32, poptdebugbitcode: Option<*const DxcBuffer>) -> windows_core::Result<IDxcOperationResult>
    where
        P0: windows_core::Param<IDxcBlob>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValidateWithDebug)(windows_core::Interface::as_raw(self), pshader.param().abi(), flags, poptdebugbitcode.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator2_Vtbl {
    pub base__: IDxcValidator_Vtbl,
    pub ValidateWithDebug: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const DxcBuffer, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDxcValidator2_Impl: IDxcValidator_Impl {
    fn ValidateWithDebug(&self, pshader: windows_core::Ref<IDxcBlob>, flags: u32, poptdebugbitcode: *const DxcBuffer) -> windows_core::Result<IDxcOperationResult>;
}
impl IDxcValidator2_Vtbl {
    pub const fn new<Identity: IDxcValidator2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ValidateWithDebug<Identity: IDxcValidator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void, flags: u32, poptdebugbitcode: *const DxcBuffer, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcValidator2_Impl::ValidateWithDebug(this, core::mem::transmute_copy(&pshader), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&poptdebugbitcode)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDxcValidator_Vtbl::new::<Identity, OFFSET>(), ValidateWithDebug: ValidateWithDebug::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcValidator2 as windows_core::Interface>::IID || iid == &<IDxcValidator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcValidator2 {}
windows_core::imp::define_interface!(IDxcVersionInfo, IDxcVersionInfo_Vtbl, 0xb04f5b50_2059_4f12_a8ff_a1e0cde1cc7e);
windows_core::imp::interface_hierarchy!(IDxcVersionInfo, windows_core::IUnknown);
impl IDxcVersionInfo {
    pub unsafe fn GetVersion(&self, pmajor: *mut u32, pminor: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), pmajor as _, pminor as _).ok() }
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IDxcVersionInfo_Impl: windows_core::IUnknownImpl {
    fn GetVersion(&self, pmajor: *mut u32, pminor: *mut u32) -> windows_core::Result<()>;
    fn GetFlags(&self) -> windows_core::Result<u32>;
}
impl IDxcVersionInfo_Vtbl {
    pub const fn new<Identity: IDxcVersionInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVersion<Identity: IDxcVersionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmajor: *mut u32, pminor: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcVersionInfo_Impl::GetVersion(this, core::mem::transmute_copy(&pmajor), core::mem::transmute_copy(&pminor)).into()
            }
        }
        unsafe extern "system" fn GetFlags<Identity: IDxcVersionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcVersionInfo_Impl::GetFlags(this) {
                    Ok(ok__) => {
                        pflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVersion: GetVersion::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcVersionInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcVersionInfo {}
windows_core::imp::define_interface!(IDxcVersionInfo2, IDxcVersionInfo2_Vtbl, 0xfb6904c4_42f0_4b62_9c46_983af7da7c83);
impl core::ops::Deref for IDxcVersionInfo2 {
    type Target = IDxcVersionInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDxcVersionInfo2, windows_core::IUnknown, IDxcVersionInfo);
impl IDxcVersionInfo2 {
    pub unsafe fn GetCommitInfo(&self, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCommitInfo)(windows_core::Interface::as_raw(self), pcommitcount as _, pcommithash as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo2_Vtbl {
    pub base__: IDxcVersionInfo_Vtbl,
    pub GetCommitInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut i8) -> windows_core::HRESULT,
}
pub trait IDxcVersionInfo2_Impl: IDxcVersionInfo_Impl {
    fn GetCommitInfo(&self, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> windows_core::Result<()>;
}
impl IDxcVersionInfo2_Vtbl {
    pub const fn new<Identity: IDxcVersionInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCommitInfo<Identity: IDxcVersionInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDxcVersionInfo2_Impl::GetCommitInfo(this, core::mem::transmute_copy(&pcommitcount), core::mem::transmute_copy(&pcommithash)).into()
            }
        }
        Self { base__: IDxcVersionInfo_Vtbl::new::<Identity, OFFSET>(), GetCommitInfo: GetCommitInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcVersionInfo2 as windows_core::Interface>::IID || iid == &<IDxcVersionInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcVersionInfo2 {}
windows_core::imp::define_interface!(IDxcVersionInfo3, IDxcVersionInfo3_Vtbl, 0x5e13e843_9d25_473c_9ad2_03b2d0b44b1e);
windows_core::imp::interface_hierarchy!(IDxcVersionInfo3, windows_core::IUnknown);
impl IDxcVersionInfo3 {
    pub unsafe fn GetCustomVersionString(&self) -> windows_core::Result<*mut i8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustomVersionString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCustomVersionString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut i8) -> windows_core::HRESULT,
}
pub trait IDxcVersionInfo3_Impl: windows_core::IUnknownImpl {
    fn GetCustomVersionString(&self) -> windows_core::Result<*mut i8>;
}
impl IDxcVersionInfo3_Vtbl {
    pub const fn new<Identity: IDxcVersionInfo3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCustomVersionString<Identity: IDxcVersionInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversionstring: *mut *mut i8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDxcVersionInfo3_Impl::GetCustomVersionString(this) {
                    Ok(ok__) => {
                        pversionstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCustomVersionString: GetCustomVersionString::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcVersionInfo3 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDxcVersionInfo3 {}
