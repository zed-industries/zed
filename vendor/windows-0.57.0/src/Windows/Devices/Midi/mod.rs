windows_core::imp::define_interface!(IMidiChannelPressureMessage, IMidiChannelPressureMessage_Vtbl, 0xbe1fa860_62b4_4d52_a37e_92e54d35b909);
impl windows_core::RuntimeType for IMidiChannelPressureMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiChannelPressureMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Pressure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiChannelPressureMessageFactory, IMidiChannelPressureMessageFactory_Vtbl, 0x6218ed2f_2284_412a_94cf_10fb04842c6c);
impl windows_core::RuntimeType for IMidiChannelPressureMessageFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiChannelPressureMessageFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateMidiChannelPressureMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u8, u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiControlChangeMessage, IMidiControlChangeMessage_Vtbl, 0xb7e15f83_780d_405f_b781_3e1598c97f40);
impl windows_core::RuntimeType for IMidiControlChangeMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiControlChangeMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Controller: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub ControlValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiControlChangeMessageFactory, IMidiControlChangeMessageFactory_Vtbl, 0x2ab14321_956c_46ad_9752_f87f55052fe3);
impl windows_core::RuntimeType for IMidiControlChangeMessageFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiControlChangeMessageFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateMidiControlChangeMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u8, u8, u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiInPort, IMidiInPort_Vtbl, 0xd5c1d9db_971a_4eaf_a23d_ea19fe607ff9);
impl windows_core::RuntimeType for IMidiInPort {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiInPort_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiInPortStatics, IMidiInPortStatics_Vtbl, 0x44c439dc_67ff_4a6e_8bac_fdb6610cf296);
impl windows_core::RuntimeType for IMidiInPortStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiInPortStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiMessage, IMidiMessage_Vtbl, 0x79767945_1094_4283_9be0_289fc0ee8334);
impl core::ops::Deref for IMidiMessage {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMidiMessage, windows_core::IUnknown, windows_core::IInspectable);
impl IMidiMessage {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IMidiMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub RawData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RawData: usize,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MidiMessageType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiMessageReceivedEventArgs, IMidiMessageReceivedEventArgs_Vtbl, 0x76566e56_f328_4b51_907d_b3a8ce96bf80);
impl windows_core::RuntimeType for IMidiMessageReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiMessageReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiNoteOffMessage, IMidiNoteOffMessage_Vtbl, 0x16fd8af4_198e_4d8f_a654_d305a293548f);
impl windows_core::RuntimeType for IMidiNoteOffMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiNoteOffMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Note: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Velocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiNoteOffMessageFactory, IMidiNoteOffMessageFactory_Vtbl, 0xa6b240e0_a749_425f_8af4_a4d979cc15b5);
impl windows_core::RuntimeType for IMidiNoteOffMessageFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiNoteOffMessageFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateMidiNoteOffMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u8, u8, u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiNoteOnMessage, IMidiNoteOnMessage_Vtbl, 0xe0224af5_6181_46dd_afa2_410004c057aa);
impl windows_core::RuntimeType for IMidiNoteOnMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiNoteOnMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Note: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Velocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiNoteOnMessageFactory, IMidiNoteOnMessageFactory_Vtbl, 0x9b4280a0_59c1_420e_b517_15a10aa9606b);
impl windows_core::RuntimeType for IMidiNoteOnMessageFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiNoteOnMessageFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateMidiNoteOnMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u8, u8, u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiOutPort, IMidiOutPort_Vtbl, 0x931d6d9f_57a2_4a3a_adb8_4640886f6693);
impl core::ops::Deref for IMidiOutPort {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMidiOutPort, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IMidiOutPort, super::super::Foundation::IClosable);
impl IMidiOutPort {
    pub fn SendMessage<P0>(&self, midimessage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMidiMessage>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SendMessage)(windows_core::Interface::as_raw(this), midimessage.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SendBuffer<P0>(&self, mididata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SendBuffer)(windows_core::Interface::as_raw(this), mididata.param().abi()).ok() }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for IMidiOutPort {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiOutPort_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SendMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SendBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SendBuffer: usize,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiOutPortStatics, IMidiOutPortStatics_Vtbl, 0x065cc3e9_0f88_448b_9b64_a95826c65b8f);
impl windows_core::RuntimeType for IMidiOutPortStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiOutPortStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiPitchBendChangeMessage, IMidiPitchBendChangeMessage_Vtbl, 0x29df4cb1_2e9f_4faf_8c2b_9cb82a9079ca);
impl windows_core::RuntimeType for IMidiPitchBendChangeMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiPitchBendChangeMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Bend: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiPitchBendChangeMessageFactory, IMidiPitchBendChangeMessageFactory_Vtbl, 0xf5eedf55_cfc8_4926_b30e_a3622393306c);
impl windows_core::RuntimeType for IMidiPitchBendChangeMessageFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiPitchBendChangeMessageFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateMidiPitchBendChangeMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u8, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiPolyphonicKeyPressureMessage, IMidiPolyphonicKeyPressureMessage_Vtbl, 0x1f7337fe_ace8_48a0_868e_7cdbf20f04d6);
impl windows_core::RuntimeType for IMidiPolyphonicKeyPressureMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiPolyphonicKeyPressureMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Note: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Pressure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiPolyphonicKeyPressureMessageFactory, IMidiPolyphonicKeyPressureMessageFactory_Vtbl, 0xe98f483e_c4b3_4dd2_917c_e349815a1b3b);
impl windows_core::RuntimeType for IMidiPolyphonicKeyPressureMessageFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiPolyphonicKeyPressureMessageFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateMidiPolyphonicKeyPressureMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u8, u8, u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiProgramChangeMessage, IMidiProgramChangeMessage_Vtbl, 0x9cbb3c78_7a3e_4327_aa98_20b8e4485af8);
impl windows_core::RuntimeType for IMidiProgramChangeMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiProgramChangeMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Program: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiProgramChangeMessageFactory, IMidiProgramChangeMessageFactory_Vtbl, 0xd6b04387_524b_4104_9c99_6572bfd2e261);
impl windows_core::RuntimeType for IMidiProgramChangeMessageFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiProgramChangeMessageFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateMidiProgramChangeMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u8, u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiSongPositionPointerMessage, IMidiSongPositionPointerMessage_Vtbl, 0x4ca50c56_ec5e_4ae4_a115_88dc57cc2b79);
impl windows_core::RuntimeType for IMidiSongPositionPointerMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiSongPositionPointerMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Beats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiSongPositionPointerMessageFactory, IMidiSongPositionPointerMessageFactory_Vtbl, 0x9c00e996_f10b_4fea_b395_f5d6cf80f64e);
impl windows_core::RuntimeType for IMidiSongPositionPointerMessageFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiSongPositionPointerMessageFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateMidiSongPositionPointerMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiSongSelectMessage, IMidiSongSelectMessage_Vtbl, 0x49f0f27f_6d83_4741_a5bf_4629f6be974f);
impl windows_core::RuntimeType for IMidiSongSelectMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiSongSelectMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Song: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiSongSelectMessageFactory, IMidiSongSelectMessageFactory_Vtbl, 0x848878e4_8748_4129_a66c_a05493f75daa);
impl windows_core::RuntimeType for IMidiSongSelectMessageFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiSongSelectMessageFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateMidiSongSelectMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiSynthesizer, IMidiSynthesizer_Vtbl, 0xf0da155e_db90_405f_b8ae_21d2e17f2e45);
impl windows_core::RuntimeType for IMidiSynthesizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiSynthesizer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub AudioDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    AudioDevice: usize,
    pub Volume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiSynthesizerStatics, IMidiSynthesizerStatics_Vtbl, 0x4224eaa8_6629_4d6b_aa8f_d4521a5a31ce);
impl windows_core::RuntimeType for IMidiSynthesizerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiSynthesizerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub CreateFromAudioDeviceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    CreateFromAudioDeviceAsync: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub IsSynthesizer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    IsSynthesizer: usize,
}
windows_core::imp::define_interface!(IMidiSystemExclusiveMessageFactory, IMidiSystemExclusiveMessageFactory_Vtbl, 0x083de222_3b74_4320_9b42_0ca8545f8a24);
impl windows_core::RuntimeType for IMidiSystemExclusiveMessageFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiSystemExclusiveMessageFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateMidiSystemExclusiveMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateMidiSystemExclusiveMessage: usize,
}
windows_core::imp::define_interface!(IMidiTimeCodeMessage, IMidiTimeCodeMessage_Vtbl, 0x0bf7087d_fa63_4a1c_8deb_c0e87796a6d7);
impl windows_core::RuntimeType for IMidiTimeCodeMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiTimeCodeMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FrameType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Values: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMidiTimeCodeMessageFactory, IMidiTimeCodeMessageFactory_Vtbl, 0xeb3099c5_771c_40de_b961_175a7489a85e);
impl windows_core::RuntimeType for IMidiTimeCodeMessageFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMidiTimeCodeMessageFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateMidiTimeCodeMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u8, u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiActiveSensingMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiActiveSensingMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiActiveSensingMessage, IMidiMessage);
impl MidiActiveSensingMessage {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiActiveSensingMessage, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MidiActiveSensingMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiMessage>();
}
unsafe impl windows_core::Interface for MidiActiveSensingMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiActiveSensingMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiActiveSensingMessage";
}
unsafe impl Send for MidiActiveSensingMessage {}
unsafe impl Sync for MidiActiveSensingMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiChannelPressureMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiChannelPressureMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiChannelPressureMessage, IMidiMessage);
impl MidiChannelPressureMessage {
    pub fn Channel(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Channel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Pressure(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pressure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateMidiChannelPressureMessage(channel: u8, pressure: u8) -> windows_core::Result<MidiChannelPressureMessage> {
        Self::IMidiChannelPressureMessageFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMidiChannelPressureMessage)(windows_core::Interface::as_raw(this), channel, pressure, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[doc(hidden)]
    pub fn IMidiChannelPressureMessageFactory<R, F: FnOnce(&IMidiChannelPressureMessageFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiChannelPressureMessage, IMidiChannelPressureMessageFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MidiChannelPressureMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiChannelPressureMessage>();
}
unsafe impl windows_core::Interface for MidiChannelPressureMessage {
    type Vtable = IMidiChannelPressureMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiChannelPressureMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiChannelPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiChannelPressureMessage";
}
unsafe impl Send for MidiChannelPressureMessage {}
unsafe impl Sync for MidiChannelPressureMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiContinueMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiContinueMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiContinueMessage, IMidiMessage);
impl MidiContinueMessage {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiContinueMessage, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MidiContinueMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiMessage>();
}
unsafe impl windows_core::Interface for MidiContinueMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiContinueMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiContinueMessage";
}
unsafe impl Send for MidiContinueMessage {}
unsafe impl Sync for MidiContinueMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiControlChangeMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiControlChangeMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiControlChangeMessage, IMidiMessage);
impl MidiControlChangeMessage {
    pub fn Channel(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Channel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Controller(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Controller)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ControlValue(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ControlValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateMidiControlChangeMessage(channel: u8, controller: u8, controlvalue: u8) -> windows_core::Result<MidiControlChangeMessage> {
        Self::IMidiControlChangeMessageFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMidiControlChangeMessage)(windows_core::Interface::as_raw(this), channel, controller, controlvalue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[doc(hidden)]
    pub fn IMidiControlChangeMessageFactory<R, F: FnOnce(&IMidiControlChangeMessageFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiControlChangeMessage, IMidiControlChangeMessageFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MidiControlChangeMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiControlChangeMessage>();
}
unsafe impl windows_core::Interface for MidiControlChangeMessage {
    type Vtable = IMidiControlChangeMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiControlChangeMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiControlChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiControlChangeMessage";
}
unsafe impl Send for MidiControlChangeMessage {}
unsafe impl Sync for MidiControlChangeMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiInPort(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiInPort, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiInPort, super::super::Foundation::IClosable);
impl MidiInPort {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MessageReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MidiInPort, MidiMessageReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMessageReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMessageReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MidiInPort>> {
        Self::IMidiInPortStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IMidiInPortStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMidiInPortStatics<R, F: FnOnce(&IMidiInPortStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiInPort, IMidiInPortStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MidiInPort {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiInPort>();
}
unsafe impl windows_core::Interface for MidiInPort {
    type Vtable = IMidiInPort_Vtbl;
    const IID: windows_core::GUID = <IMidiInPort as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiInPort {
    const NAME: &'static str = "Windows.Devices.Midi.MidiInPort";
}
unsafe impl Send for MidiInPort {}
unsafe impl Sync for MidiInPort {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiMessageReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiMessageReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MidiMessageReceivedEventArgs {
    pub fn Message(&self) -> windows_core::Result<IMidiMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MidiMessageReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiMessageReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MidiMessageReceivedEventArgs {
    type Vtable = IMidiMessageReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMidiMessageReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Midi.MidiMessageReceivedEventArgs";
}
unsafe impl Send for MidiMessageReceivedEventArgs {}
unsafe impl Sync for MidiMessageReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiNoteOffMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiNoteOffMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiNoteOffMessage, IMidiMessage);
impl MidiNoteOffMessage {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Channel(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Channel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Note(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Note)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Velocity(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Velocity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateMidiNoteOffMessage(channel: u8, note: u8, velocity: u8) -> windows_core::Result<MidiNoteOffMessage> {
        Self::IMidiNoteOffMessageFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMidiNoteOffMessage)(windows_core::Interface::as_raw(this), channel, note, velocity, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMidiNoteOffMessageFactory<R, F: FnOnce(&IMidiNoteOffMessageFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiNoteOffMessage, IMidiNoteOffMessageFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MidiNoteOffMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiNoteOffMessage>();
}
unsafe impl windows_core::Interface for MidiNoteOffMessage {
    type Vtable = IMidiNoteOffMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiNoteOffMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiNoteOffMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiNoteOffMessage";
}
unsafe impl Send for MidiNoteOffMessage {}
unsafe impl Sync for MidiNoteOffMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiNoteOnMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiNoteOnMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiNoteOnMessage, IMidiMessage);
impl MidiNoteOnMessage {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Channel(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Channel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Note(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Note)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Velocity(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Velocity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateMidiNoteOnMessage(channel: u8, note: u8, velocity: u8) -> windows_core::Result<MidiNoteOnMessage> {
        Self::IMidiNoteOnMessageFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMidiNoteOnMessage)(windows_core::Interface::as_raw(this), channel, note, velocity, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMidiNoteOnMessageFactory<R, F: FnOnce(&IMidiNoteOnMessageFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiNoteOnMessage, IMidiNoteOnMessageFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MidiNoteOnMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiNoteOnMessage>();
}
unsafe impl windows_core::Interface for MidiNoteOnMessage {
    type Vtable = IMidiNoteOnMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiNoteOnMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiNoteOnMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiNoteOnMessage";
}
unsafe impl Send for MidiNoteOnMessage {}
unsafe impl Sync for MidiNoteOnMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiOutPort(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiOutPort, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiOutPort, super::super::Foundation::IClosable, IMidiOutPort);
impl MidiOutPort {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SendMessage<P0>(&self, midimessage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMidiMessage>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SendMessage)(windows_core::Interface::as_raw(this), midimessage.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SendBuffer<P0>(&self, mididata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SendBuffer)(windows_core::Interface::as_raw(this), mididata.param().abi()).ok() }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<IMidiOutPort>> {
        Self::IMidiOutPortStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IMidiOutPortStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMidiOutPortStatics<R, F: FnOnce(&IMidiOutPortStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiOutPort, IMidiOutPortStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MidiOutPort {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiOutPort>();
}
unsafe impl windows_core::Interface for MidiOutPort {
    type Vtable = IMidiOutPort_Vtbl;
    const IID: windows_core::GUID = <IMidiOutPort as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiOutPort {
    const NAME: &'static str = "Windows.Devices.Midi.MidiOutPort";
}
unsafe impl Send for MidiOutPort {}
unsafe impl Sync for MidiOutPort {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiPitchBendChangeMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiPitchBendChangeMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiPitchBendChangeMessage, IMidiMessage);
impl MidiPitchBendChangeMessage {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Channel(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Channel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Bend(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bend)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateMidiPitchBendChangeMessage(channel: u8, bend: u16) -> windows_core::Result<MidiPitchBendChangeMessage> {
        Self::IMidiPitchBendChangeMessageFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMidiPitchBendChangeMessage)(windows_core::Interface::as_raw(this), channel, bend, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMidiPitchBendChangeMessageFactory<R, F: FnOnce(&IMidiPitchBendChangeMessageFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiPitchBendChangeMessage, IMidiPitchBendChangeMessageFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MidiPitchBendChangeMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiPitchBendChangeMessage>();
}
unsafe impl windows_core::Interface for MidiPitchBendChangeMessage {
    type Vtable = IMidiPitchBendChangeMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiPitchBendChangeMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiPitchBendChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiPitchBendChangeMessage";
}
unsafe impl Send for MidiPitchBendChangeMessage {}
unsafe impl Sync for MidiPitchBendChangeMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiPolyphonicKeyPressureMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiPolyphonicKeyPressureMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiPolyphonicKeyPressureMessage, IMidiMessage);
impl MidiPolyphonicKeyPressureMessage {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Channel(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Channel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Note(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Note)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Pressure(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pressure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateMidiPolyphonicKeyPressureMessage(channel: u8, note: u8, pressure: u8) -> windows_core::Result<MidiPolyphonicKeyPressureMessage> {
        Self::IMidiPolyphonicKeyPressureMessageFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMidiPolyphonicKeyPressureMessage)(windows_core::Interface::as_raw(this), channel, note, pressure, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMidiPolyphonicKeyPressureMessageFactory<R, F: FnOnce(&IMidiPolyphonicKeyPressureMessageFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiPolyphonicKeyPressureMessage, IMidiPolyphonicKeyPressureMessageFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MidiPolyphonicKeyPressureMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiPolyphonicKeyPressureMessage>();
}
unsafe impl windows_core::Interface for MidiPolyphonicKeyPressureMessage {
    type Vtable = IMidiPolyphonicKeyPressureMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiPolyphonicKeyPressureMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiPolyphonicKeyPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiPolyphonicKeyPressureMessage";
}
unsafe impl Send for MidiPolyphonicKeyPressureMessage {}
unsafe impl Sync for MidiPolyphonicKeyPressureMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiProgramChangeMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiProgramChangeMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiProgramChangeMessage, IMidiMessage);
impl MidiProgramChangeMessage {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Channel(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Channel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Program(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Program)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateMidiProgramChangeMessage(channel: u8, program: u8) -> windows_core::Result<MidiProgramChangeMessage> {
        Self::IMidiProgramChangeMessageFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMidiProgramChangeMessage)(windows_core::Interface::as_raw(this), channel, program, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMidiProgramChangeMessageFactory<R, F: FnOnce(&IMidiProgramChangeMessageFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiProgramChangeMessage, IMidiProgramChangeMessageFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MidiProgramChangeMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiProgramChangeMessage>();
}
unsafe impl windows_core::Interface for MidiProgramChangeMessage {
    type Vtable = IMidiProgramChangeMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiProgramChangeMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiProgramChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiProgramChangeMessage";
}
unsafe impl Send for MidiProgramChangeMessage {}
unsafe impl Sync for MidiProgramChangeMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiSongPositionPointerMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiSongPositionPointerMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiSongPositionPointerMessage, IMidiMessage);
impl MidiSongPositionPointerMessage {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Beats(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Beats)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateMidiSongPositionPointerMessage(beats: u16) -> windows_core::Result<MidiSongPositionPointerMessage> {
        Self::IMidiSongPositionPointerMessageFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMidiSongPositionPointerMessage)(windows_core::Interface::as_raw(this), beats, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMidiSongPositionPointerMessageFactory<R, F: FnOnce(&IMidiSongPositionPointerMessageFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiSongPositionPointerMessage, IMidiSongPositionPointerMessageFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MidiSongPositionPointerMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiSongPositionPointerMessage>();
}
unsafe impl windows_core::Interface for MidiSongPositionPointerMessage {
    type Vtable = IMidiSongPositionPointerMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiSongPositionPointerMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiSongPositionPointerMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSongPositionPointerMessage";
}
unsafe impl Send for MidiSongPositionPointerMessage {}
unsafe impl Sync for MidiSongPositionPointerMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiSongSelectMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiSongSelectMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiSongSelectMessage, IMidiMessage);
impl MidiSongSelectMessage {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Song(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Song)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateMidiSongSelectMessage(song: u8) -> windows_core::Result<MidiSongSelectMessage> {
        Self::IMidiSongSelectMessageFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMidiSongSelectMessage)(windows_core::Interface::as_raw(this), song, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMidiSongSelectMessageFactory<R, F: FnOnce(&IMidiSongSelectMessageFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiSongSelectMessage, IMidiSongSelectMessageFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MidiSongSelectMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiSongSelectMessage>();
}
unsafe impl windows_core::Interface for MidiSongSelectMessage {
    type Vtable = IMidiSongSelectMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiSongSelectMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiSongSelectMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSongSelectMessage";
}
unsafe impl Send for MidiSongSelectMessage {}
unsafe impl Sync for MidiSongSelectMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiStartMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiStartMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiStartMessage, IMidiMessage);
impl MidiStartMessage {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiStartMessage, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MidiStartMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiMessage>();
}
unsafe impl windows_core::Interface for MidiStartMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiStartMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiStartMessage";
}
unsafe impl Send for MidiStartMessage {}
unsafe impl Sync for MidiStartMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiStopMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiStopMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiStopMessage, IMidiMessage);
impl MidiStopMessage {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiStopMessage, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MidiStopMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiMessage>();
}
unsafe impl windows_core::Interface for MidiStopMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiStopMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiStopMessage";
}
unsafe impl Send for MidiStopMessage {}
unsafe impl Sync for MidiStopMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiSynthesizer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiSynthesizer, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiSynthesizer, super::super::Foundation::IClosable, IMidiOutPort);
impl MidiSynthesizer {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SendMessage<P0>(&self, midimessage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMidiMessage>,
    {
        let this = &windows_core::Interface::cast::<IMidiOutPort>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SendMessage)(windows_core::Interface::as_raw(this), midimessage.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SendBuffer<P0>(&self, mididata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IMidiOutPort>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SendBuffer)(windows_core::Interface::as_raw(this), mididata.param().abi()).ok() }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMidiOutPort>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn AudioDevice(&self) -> windows_core::Result<super::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioDevice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Volume(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Volume)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVolume(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVolume)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>> {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn CreateFromAudioDeviceAsync<P0>(audiodevice: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>>
    where
        P0: windows_core::Param<super::Enumeration::DeviceInformation>,
    {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromAudioDeviceAsync)(windows_core::Interface::as_raw(this), audiodevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn IsSynthesizer<P0>(mididevice: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::Enumeration::DeviceInformation>,
    {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSynthesizer)(windows_core::Interface::as_raw(this), mididevice.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiSynthesizerStatics<R, F: FnOnce(&IMidiSynthesizerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiSynthesizer, IMidiSynthesizerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MidiSynthesizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiSynthesizer>();
}
unsafe impl windows_core::Interface for MidiSynthesizer {
    type Vtable = IMidiSynthesizer_Vtbl;
    const IID: windows_core::GUID = <IMidiSynthesizer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiSynthesizer {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSynthesizer";
}
unsafe impl Send for MidiSynthesizer {}
unsafe impl Sync for MidiSynthesizer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiSystemExclusiveMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiSystemExclusiveMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiSystemExclusiveMessage, IMidiMessage);
impl MidiSystemExclusiveMessage {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateMidiSystemExclusiveMessage<P0>(rawdata: P0) -> windows_core::Result<MidiSystemExclusiveMessage>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::IMidiSystemExclusiveMessageFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMidiSystemExclusiveMessage)(windows_core::Interface::as_raw(this), rawdata.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMidiSystemExclusiveMessageFactory<R, F: FnOnce(&IMidiSystemExclusiveMessageFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiSystemExclusiveMessage, IMidiSystemExclusiveMessageFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MidiSystemExclusiveMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiMessage>();
}
unsafe impl windows_core::Interface for MidiSystemExclusiveMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiSystemExclusiveMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSystemExclusiveMessage";
}
unsafe impl Send for MidiSystemExclusiveMessage {}
unsafe impl Sync for MidiSystemExclusiveMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiSystemResetMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiSystemResetMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiSystemResetMessage, IMidiMessage);
impl MidiSystemResetMessage {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiSystemResetMessage, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MidiSystemResetMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiMessage>();
}
unsafe impl windows_core::Interface for MidiSystemResetMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiSystemResetMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSystemResetMessage";
}
unsafe impl Send for MidiSystemResetMessage {}
unsafe impl Sync for MidiSystemResetMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiTimeCodeMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiTimeCodeMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiTimeCodeMessage, IMidiMessage);
impl MidiTimeCodeMessage {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = &windows_core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FrameType(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Values(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Values)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateMidiTimeCodeMessage(frametype: u8, values: u8) -> windows_core::Result<MidiTimeCodeMessage> {
        Self::IMidiTimeCodeMessageFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMidiTimeCodeMessage)(windows_core::Interface::as_raw(this), frametype, values, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMidiTimeCodeMessageFactory<R, F: FnOnce(&IMidiTimeCodeMessageFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiTimeCodeMessage, IMidiTimeCodeMessageFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MidiTimeCodeMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiTimeCodeMessage>();
}
unsafe impl windows_core::Interface for MidiTimeCodeMessage {
    type Vtable = IMidiTimeCodeMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiTimeCodeMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiTimeCodeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTimeCodeMessage";
}
unsafe impl Send for MidiTimeCodeMessage {}
unsafe impl Sync for MidiTimeCodeMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiTimingClockMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiTimingClockMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiTimingClockMessage, IMidiMessage);
impl MidiTimingClockMessage {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiTimingClockMessage, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MidiTimingClockMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiMessage>();
}
unsafe impl windows_core::Interface for MidiTimingClockMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiTimingClockMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTimingClockMessage";
}
unsafe impl Send for MidiTimingClockMessage {}
unsafe impl Sync for MidiTimingClockMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MidiTuneRequestMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MidiTuneRequestMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MidiTuneRequestMessage, IMidiMessage);
impl MidiTuneRequestMessage {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MidiTuneRequestMessage, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MidiTuneRequestMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMidiMessage>();
}
unsafe impl windows_core::Interface for MidiTuneRequestMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: windows_core::GUID = <IMidiMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MidiTuneRequestMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTuneRequestMessage";
}
unsafe impl Send for MidiTuneRequestMessage {}
unsafe impl Sync for MidiTuneRequestMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MidiMessageType(pub i32);
impl MidiMessageType {
    pub const None: Self = Self(0i32);
    pub const NoteOff: Self = Self(128i32);
    pub const NoteOn: Self = Self(144i32);
    pub const PolyphonicKeyPressure: Self = Self(160i32);
    pub const ControlChange: Self = Self(176i32);
    pub const ProgramChange: Self = Self(192i32);
    pub const ChannelPressure: Self = Self(208i32);
    pub const PitchBendChange: Self = Self(224i32);
    pub const SystemExclusive: Self = Self(240i32);
    pub const MidiTimeCode: Self = Self(241i32);
    pub const SongPositionPointer: Self = Self(242i32);
    pub const SongSelect: Self = Self(243i32);
    pub const TuneRequest: Self = Self(246i32);
    pub const EndSystemExclusive: Self = Self(247i32);
    pub const TimingClock: Self = Self(248i32);
    pub const Start: Self = Self(250i32);
    pub const Continue: Self = Self(251i32);
    pub const Stop: Self = Self(252i32);
    pub const ActiveSensing: Self = Self(254i32);
    pub const SystemReset: Self = Self(255i32);
}
impl windows_core::TypeKind for MidiMessageType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MidiMessageType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MidiMessageType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MidiMessageType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Midi.MidiMessageType;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
