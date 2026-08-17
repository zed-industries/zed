#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
pub trait IAudioInputNode_Impl: Sized + IAudioNode_Impl + super::super::Foundation::IClosable_Impl {
    fn OutgoingConnections(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>;
    fn AddOutgoingConnection(&self, destination: Option<&IAudioNode>) -> windows_core::Result<()>;
    fn AddOutgoingConnectionWithGain(&self, destination: Option<&IAudioNode>, gain: f64) -> windows_core::Result<()>;
    fn RemoveOutgoingConnection(&self, destination: Option<&IAudioNode>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl windows_core::RuntimeName for IAudioInputNode {
    const NAME: &'static str = "Windows.Media.Audio.IAudioInputNode";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl IAudioInputNode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputNode_Impl, const OFFSET: isize>() -> IAudioInputNode_Vtbl {
        unsafe extern "system" fn OutgoingConnections<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioInputNode_Impl::OutgoingConnections(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOutgoingConnection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioInputNode_Impl::AddOutgoingConnection(this, windows_core::from_raw_borrowed(&destination)).into()
        }
        unsafe extern "system" fn AddOutgoingConnectionWithGain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destination: *mut core::ffi::c_void, gain: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioInputNode_Impl::AddOutgoingConnectionWithGain(this, windows_core::from_raw_borrowed(&destination), gain).into()
        }
        unsafe extern "system" fn RemoveOutgoingConnection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioInputNode_Impl::RemoveOutgoingConnection(this, windows_core::from_raw_borrowed(&destination)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAudioInputNode, OFFSET>(),
            OutgoingConnections: OutgoingConnections::<Identity, Impl, OFFSET>,
            AddOutgoingConnection: AddOutgoingConnection::<Identity, Impl, OFFSET>,
            AddOutgoingConnectionWithGain: AddOutgoingConnectionWithGain::<Identity, Impl, OFFSET>,
            RemoveOutgoingConnection: RemoveOutgoingConnection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioInputNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
pub trait IAudioInputNode2_Impl: Sized + IAudioInputNode_Impl + IAudioNode_Impl + super::super::Foundation::IClosable_Impl {
    fn Emitter(&self) -> windows_core::Result<AudioNodeEmitter>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl windows_core::RuntimeName for IAudioInputNode2 {
    const NAME: &'static str = "Windows.Media.Audio.IAudioInputNode2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl IAudioInputNode2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputNode2_Impl, const OFFSET: isize>() -> IAudioInputNode2_Vtbl {
        unsafe extern "system" fn Emitter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputNode2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioInputNode2_Impl::Emitter(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IAudioInputNode2, OFFSET>(), Emitter: Emitter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioInputNode2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
pub trait IAudioNode_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn EffectDefinitions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>;
    fn SetOutgoingGain(&self, value: f64) -> windows_core::Result<()>;
    fn OutgoingGain(&self) -> windows_core::Result<f64>;
    fn EncodingProperties(&self) -> windows_core::Result<super::MediaProperties::AudioEncodingProperties>;
    fn ConsumeInput(&self) -> windows_core::Result<bool>;
    fn SetConsumeInput(&self, value: bool) -> windows_core::Result<()>;
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn DisableEffectsByDefinition(&self, definition: Option<&super::Effects::IAudioEffectDefinition>) -> windows_core::Result<()>;
    fn EnableEffectsByDefinition(&self, definition: Option<&super::Effects::IAudioEffectDefinition>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl windows_core::RuntimeName for IAudioNode {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNode";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl IAudioNode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>() -> IAudioNode_Vtbl {
        unsafe extern "system" fn EffectDefinitions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioNode_Impl::EffectDefinitions(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingGain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioNode_Impl::SetOutgoingGain(this, value).into()
        }
        unsafe extern "system" fn OutgoingGain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioNode_Impl::OutgoingGain(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodingProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioNode_Impl::EncodingProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConsumeInput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioNode_Impl::ConsumeInput(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConsumeInput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioNode_Impl::SetConsumeInput(this, value).into()
        }
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioNode_Impl::Start(this).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioNode_Impl::Stop(this).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioNode_Impl::Reset(this).into()
        }
        unsafe extern "system" fn DisableEffectsByDefinition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, definition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioNode_Impl::DisableEffectsByDefinition(this, windows_core::from_raw_borrowed(&definition)).into()
        }
        unsafe extern "system" fn EnableEffectsByDefinition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, definition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioNode_Impl::EnableEffectsByDefinition(this, windows_core::from_raw_borrowed(&definition)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAudioNode, OFFSET>(),
            EffectDefinitions: EffectDefinitions::<Identity, Impl, OFFSET>,
            SetOutgoingGain: SetOutgoingGain::<Identity, Impl, OFFSET>,
            OutgoingGain: OutgoingGain::<Identity, Impl, OFFSET>,
            EncodingProperties: EncodingProperties::<Identity, Impl, OFFSET>,
            ConsumeInput: ConsumeInput::<Identity, Impl, OFFSET>,
            SetConsumeInput: SetConsumeInput::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            DisableEffectsByDefinition: DisableEffectsByDefinition::<Identity, Impl, OFFSET>,
            EnableEffectsByDefinition: EnableEffectsByDefinition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
pub trait IAudioNodeWithListener_Impl: Sized + IAudioNode_Impl + super::super::Foundation::IClosable_Impl {
    fn SetListener(&self, value: Option<&AudioNodeListener>) -> windows_core::Result<()>;
    fn Listener(&self) -> windows_core::Result<AudioNodeListener>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl windows_core::RuntimeName for IAudioNodeWithListener {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNodeWithListener";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl IAudioNodeWithListener_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioNodeWithListener_Impl, const OFFSET: isize>() -> IAudioNodeWithListener_Vtbl {
        unsafe extern "system" fn SetListener<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioNodeWithListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioNodeWithListener_Impl::SetListener(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Listener<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioNodeWithListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioNodeWithListener_Impl::Listener(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAudioNodeWithListener, OFFSET>(),
            SetListener: SetListener::<Identity, Impl, OFFSET>,
            Listener: Listener::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioNodeWithListener as windows_core::Interface>::IID
    }
}
