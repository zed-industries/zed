#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Direct3DBindings(pub u32);
impl Direct3DBindings {
    pub const VertexBuffer: Self = Self(1u32);
    pub const IndexBuffer: Self = Self(2u32);
    pub const ConstantBuffer: Self = Self(4u32);
    pub const ShaderResource: Self = Self(8u32);
    pub const StreamOutput: Self = Self(16u32);
    pub const RenderTarget: Self = Self(32u32);
    pub const DepthStencil: Self = Self(64u32);
    pub const UnorderedAccess: Self = Self(128u32);
    pub const Decoder: Self = Self(512u32);
    pub const VideoEncoder: Self = Self(1024u32);
}
impl windows_core::TypeKind for Direct3DBindings {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Direct3DBindings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.Direct3D11.Direct3DBindings;u4)");
}
impl Direct3DBindings {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for Direct3DBindings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for Direct3DBindings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for Direct3DBindings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for Direct3DBindings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for Direct3DBindings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Direct3DMultisampleDescription {
    pub Count: i32,
    pub Quality: i32,
}
impl windows_core::TypeKind for Direct3DMultisampleDescription {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Direct3DMultisampleDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.DirectX.Direct3D11.Direct3DMultisampleDescription;i4;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Direct3DSurfaceDescription {
    pub Width: i32,
    pub Height: i32,
    pub Format: super::DirectXPixelFormat,
    pub MultisampleDescription: Direct3DMultisampleDescription,
}
impl windows_core::TypeKind for Direct3DSurfaceDescription {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Direct3DSurfaceDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.DirectX.Direct3D11.Direct3DSurfaceDescription;i4;i4;enum(Windows.Graphics.DirectX.DirectXPixelFormat;i4);struct(Windows.Graphics.DirectX.Direct3D11.Direct3DMultisampleDescription;i4;i4))");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Direct3DUsage(pub i32);
impl Direct3DUsage {
    pub const Default: Self = Self(0i32);
    pub const Immutable: Self = Self(1i32);
    pub const Dynamic: Self = Self(2i32);
    pub const Staging: Self = Self(3i32);
}
impl windows_core::TypeKind for Direct3DUsage {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Direct3DUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.Direct3D11.Direct3DUsage;i4)");
}
windows_core::imp::define_interface!(IDirect3DDevice, IDirect3DDevice_Vtbl, 0xa37624ab_8d5f_4650_9d3e_9eae3d9bc670);
impl windows_core::RuntimeType for IDirect3DDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IDirect3DDevice, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IDirect3DDevice, super::super::super::Foundation::IClosable);
impl IDirect3DDevice {
    pub fn Trim(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Trim)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for IDirect3DDevice {
    const NAME: &'static str = "Windows.Graphics.DirectX.Direct3D11.IDirect3DDevice";
}
pub trait IDirect3DDevice_Impl: super::super::super::Foundation::IClosable_Impl {
    fn Trim(&self) -> windows_core::Result<()>;
}
impl IDirect3DDevice_Vtbl {
    pub const fn new<Identity: IDirect3DDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Trim<Identity: IDirect3DDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice_Impl::Trim(this).into()
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDirect3DDevice, OFFSET>(), Trim: Trim::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DDevice as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Trim: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirect3DSurface, IDirect3DSurface_Vtbl, 0x0bf4a146_13c1_4694_bee3_7abf15eaf586);
impl windows_core::RuntimeType for IDirect3DSurface {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IDirect3DSurface, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IDirect3DSurface, super::super::super::Foundation::IClosable);
impl IDirect3DSurface {
    pub fn Description(&self) -> windows_core::Result<Direct3DSurfaceDescription> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for IDirect3DSurface {
    const NAME: &'static str = "Windows.Graphics.DirectX.Direct3D11.IDirect3DSurface";
}
pub trait IDirect3DSurface_Impl: super::super::super::Foundation::IClosable_Impl {
    fn Description(&self) -> windows_core::Result<Direct3DSurfaceDescription>;
}
impl IDirect3DSurface_Vtbl {
    pub const fn new<Identity: IDirect3DSurface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Description<Identity: IDirect3DSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Direct3DSurfaceDescription) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DSurface_Impl::Description(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDirect3DSurface, OFFSET>(), Description: Description::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DSurface as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DSurface_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Direct3DSurfaceDescription) -> windows_core::HRESULT,
}
