windows_core::imp::define_interface!(IVpnAppId, IVpnAppId_Vtbl, 0x7b06a635_5c58_41d9_94a7_bfbcf1d8ca54);
impl windows_core::RuntimeType for IVpnAppId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnAppId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VpnAppIdType) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, VpnAppIdType) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnAppIdFactory, IVpnAppIdFactory_Vtbl, 0x46adfd2a_0aab_4fdb_821d_d3ddc919788b);
impl windows_core::RuntimeType for IVpnAppIdFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnAppIdFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, VpnAppIdType, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnChannel, IVpnChannel_Vtbl, 0x4ac78d07_d1a8_4303_a091_c8d2e0915bc3);
impl windows_core::RuntimeType for IVpnChannel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnChannel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AssociateTransport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, bool, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Start: usize,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub RequestCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, VpnCredentialType, bool, bool, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    RequestCredentials: usize,
    pub RequestVpnPacketBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, VpnDataPathType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LogDiagnosticMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivityChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveActivityChange: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SetPlugInContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlugInContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SystemHealth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestCustomPrompt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestCustomPrompt: usize,
    pub SetErrorMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAllowedSslTlsVersions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnChannel2, IVpnChannel2_Vtbl, 0x2255d165_993b_4629_ad60_f1c3f3537f50);
impl windows_core::RuntimeType for IVpnChannel2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnChannel2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub StartWithMainTransport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, bool, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartWithMainTransport: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StartExistingTransports: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartExistingTransports: usize,
    pub ActivityStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveActivityStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub GetVpnSendPacketBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVpnReceivePacketBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestCustomPromptAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestCustomPromptAsync: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub RequestCredentialsWithCertificateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, VpnCredentialType, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    RequestCredentialsWithCertificateAsync: usize,
    pub RequestCredentialsWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, VpnCredentialType, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestCredentialsSimpleAsync: unsafe extern "system" fn(*mut core::ffi::c_void, VpnCredentialType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TerminateConnection: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StartWithTrafficFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, bool, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartWithTrafficFilter: usize,
}
windows_core::imp::define_interface!(IVpnChannel4, IVpnChannel4_Vtbl, 0xd7266ede_2937_419d_9570_486aebb81803);
impl windows_core::RuntimeType for IVpnChannel4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnChannel4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddAndAssociateTransport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StartWithMultipleTransports: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, bool, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartWithMultipleTransports: usize,
    pub ReplaceAndAssociateTransport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartReconnectingTransport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub GetSlotTypeForTransportContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Sockets::ControlChannelTriggerStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    GetSlotTypeForTransportContext: usize,
    pub CurrentRequestTransportContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnChannel5, IVpnChannel5_Vtbl, 0xde7a0992_8384_4fbc_882c_1fd23124cd3b);
impl windows_core::RuntimeType for IVpnChannel5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnChannel5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppendVpnReceivePacketBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppendVpnSendPacketBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FlushVpnReceivePacketBuffers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FlushVpnSendPacketBuffers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnChannel6, IVpnChannel6_Vtbl, 0x55843696_bd63_49c5_abca_5da77885551a);
impl windows_core::RuntimeType for IVpnChannel6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnChannel6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ActivateForeground: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActivateForeground: usize,
}
windows_core::imp::define_interface!(IVpnChannelActivityEventArgs, IVpnChannelActivityEventArgs_Vtbl, 0xa36c88f2_afdc_4775_855d_d4ac0a35fc55);
impl windows_core::RuntimeType for IVpnChannelActivityEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnChannelActivityEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VpnChannelActivityEventType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnChannelActivityStateChangedArgs, IVpnChannelActivityStateChangedArgs_Vtbl, 0x3d750565_fdc0_4bbe_a23b_45fffc6d97a1);
impl windows_core::RuntimeType for IVpnChannelActivityStateChangedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnChannelActivityStateChangedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivityState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VpnChannelActivityEventType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnChannelConfiguration, IVpnChannelConfiguration_Vtbl, 0x0e2ddca2_2012_4fe4_b179_8c652c6d107e);
impl windows_core::RuntimeType for IVpnChannelConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnChannelConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServerServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ServerHostNameList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServerHostNameList: usize,
    pub CustomField: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnChannelConfiguration2, IVpnChannelConfiguration2_Vtbl, 0xf30b574c_7824_471c_a118_63dbc93ae4c7);
impl windows_core::RuntimeType for IVpnChannelConfiguration2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnChannelConfiguration2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ServerUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServerUris: usize,
}
windows_core::imp::define_interface!(IVpnChannelStatics, IVpnChannelStatics_Vtbl, 0x88eb062d_e818_4ffd_98a6_363e3736c95d);
impl core::ops::Deref for IVpnChannelStatics {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVpnChannelStatics, windows_core::IUnknown, windows_core::IInspectable);
impl IVpnChannelStatics {
    pub fn ProcessEventAsync<P0, P1>(&self, thirdpartyplugin: P0, event: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ProcessEventAsync)(windows_core::Interface::as_raw(this), thirdpartyplugin.param().abi(), event.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for IVpnChannelStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnChannelStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProcessEventAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnCredential, IVpnCredential_Vtbl, 0xb7e78af3_a46d_404b_8729_1832522853ac);
impl core::ops::Deref for IVpnCredential {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVpnCredential, windows_core::IUnknown, windows_core::IInspectable);
impl IVpnCredential {
    #[cfg(feature = "Security_Credentials")]
    pub fn PasskeyCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PasskeyCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn CertificateCredential(&self) -> windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CertificateCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AdditionalPin(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdditionalPin)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn OldPasswordCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OldPasswordCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IVpnCredential {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnCredential_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub PasskeyCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    PasskeyCredential: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub CertificateCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    CertificateCredential: usize,
    pub AdditionalPin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub OldPasswordCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    OldPasswordCredential: usize,
}
windows_core::imp::define_interface!(IVpnCustomCheckBox, IVpnCustomCheckBox_Vtbl, 0x43878753_03c5_4e61_93d7_a957714c4282);
impl windows_core::RuntimeType for IVpnCustomCheckBox {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnCustomCheckBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetInitialCheckState: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub InitialCheckState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Checked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnCustomComboBox, IVpnCustomComboBox_Vtbl, 0x9a24158e_dba1_4c6f_8270_dcf3c9761c4c);
impl windows_core::RuntimeType for IVpnCustomComboBox {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnCustomComboBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetOptionsText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetOptionsText: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionsText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionsText: usize,
    pub Selected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnCustomEditBox, IVpnCustomEditBox_Vtbl, 0x3002d9a0_cfbf_4c0b_8f3c_66f503c20b39);
impl windows_core::RuntimeType for IVpnCustomEditBox {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnCustomEditBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetDefaultText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DefaultText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetNoEcho: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub NoEcho: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnCustomErrorBox, IVpnCustomErrorBox_Vtbl, 0x9ec4efb2_c942_42af_b223_588b48328721);
impl windows_core::RuntimeType for IVpnCustomErrorBox {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnCustomErrorBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IVpnCustomPrompt, IVpnCustomPrompt_Vtbl, 0x9b2ebe7b_87d5_433c_b4f6_eee6aa68a244);
impl core::ops::Deref for IVpnCustomPrompt {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVpnCustomPrompt, windows_core::IUnknown, windows_core::IInspectable);
impl IVpnCustomPrompt {
    pub fn SetLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLabel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Label)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCompulsory)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compulsory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBordered)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bordered(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bordered)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IVpnCustomPrompt {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnCustomPrompt_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCompulsory: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Compulsory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetBordered: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Bordered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnCustomPromptBooleanInput, IVpnCustomPromptBooleanInput_Vtbl, 0xc4c9a69e_ff47_4527_9f27_a49292019979);
impl windows_core::RuntimeType for IVpnCustomPromptBooleanInput {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnCustomPromptBooleanInput_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetInitialValue: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub InitialValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnCustomPromptElement, IVpnCustomPromptElement_Vtbl, 0x73bd5638_6f04_404d_93dd_50a44924a38b);
impl core::ops::Deref for IVpnCustomPromptElement {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVpnCustomPromptElement, windows_core::IUnknown, windows_core::IInspectable);
impl IVpnCustomPromptElement {
    pub fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCompulsory)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compulsory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEmphasized(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEmphasized)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Emphasized(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Emphasized)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IVpnCustomPromptElement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnCustomPromptElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCompulsory: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Compulsory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetEmphasized: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Emphasized: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnCustomPromptOptionSelector, IVpnCustomPromptOptionSelector_Vtbl, 0x3b8f34d9_8ec1_4e95_9a4e_7ba64d38f330);
impl windows_core::RuntimeType for IVpnCustomPromptOptionSelector {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnCustomPromptOptionSelector_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Options: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Options: usize,
    pub SelectedIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnCustomPromptText, IVpnCustomPromptText_Vtbl, 0x3bc8bdee_3a42_49a3_abdd_07b2edea752d);
impl windows_core::RuntimeType for IVpnCustomPromptText {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnCustomPromptText_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnCustomPromptTextInput, IVpnCustomPromptTextInput_Vtbl, 0xc9da9c75_913c_47d5_88ba_48fc48930235);
impl windows_core::RuntimeType for IVpnCustomPromptTextInput {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnCustomPromptTextInput_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetPlaceholderText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PlaceholderText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetIsTextHidden: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsTextHidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnCustomTextBox, IVpnCustomTextBox_Vtbl, 0xdaa4c3ca_8f23_4d36_91f1_76d937827942);
impl windows_core::RuntimeType for IVpnCustomTextBox {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnCustomTextBox_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetDisplayText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnDomainNameAssignment, IVpnDomainNameAssignment_Vtbl, 0x4135b141_ccdb_49b5_9401_039a8ae767e9);
impl windows_core::RuntimeType for IVpnDomainNameAssignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnDomainNameAssignment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub DomainNameList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DomainNameList: usize,
    pub SetProxyAutoConfigurationUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProxyAutoConfigurationUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnDomainNameInfo, IVpnDomainNameInfo_Vtbl, 0xad2eb82f_ea8e_4f7a_843e_1a87e32e1b9a);
impl windows_core::RuntimeType for IVpnDomainNameInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnDomainNameInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetDomainName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DomainName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDomainNameType: unsafe extern "system" fn(*mut core::ffi::c_void, VpnDomainNameType) -> windows_core::HRESULT,
    pub DomainNameType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VpnDomainNameType) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DnsServers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DnsServers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WebProxyServers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebProxyServers: usize,
}
windows_core::imp::define_interface!(IVpnDomainNameInfo2, IVpnDomainNameInfo2_Vtbl, 0xab871151_6c53_4828_9883_d886de104407);
impl windows_core::RuntimeType for IVpnDomainNameInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnDomainNameInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub WebProxyUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebProxyUris: usize,
}
windows_core::imp::define_interface!(IVpnDomainNameInfoFactory, IVpnDomainNameInfoFactory_Vtbl, 0x2507bb75_028f_4688_8d3a_c4531df37da8);
impl core::ops::Deref for IVpnDomainNameInfoFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVpnDomainNameInfoFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IVpnDomainNameInfoFactory {
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateVpnDomainNameInfo<P0, P1>(&self, name: &windows_core::HSTRING, nametype: VpnDomainNameType, dnsserverlist: P0, proxyserverlist: P1) -> windows_core::Result<VpnDomainNameInfo>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<super::HostName>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<super::HostName>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateVpnDomainNameInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), nametype, dnsserverlist.param().abi(), proxyserverlist.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IVpnDomainNameInfoFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnDomainNameInfoFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateVpnDomainNameInfo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, VpnDomainNameType, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateVpnDomainNameInfo: usize,
}
windows_core::imp::define_interface!(IVpnForegroundActivatedEventArgs, IVpnForegroundActivatedEventArgs_Vtbl, 0x85b465b0_cadb_4d70_ac92_543a24dc9ebc);
impl windows_core::RuntimeType for IVpnForegroundActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnForegroundActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProfileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SharedContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SharedContext: usize,
    pub ActivationOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnForegroundActivationOperation, IVpnForegroundActivationOperation_Vtbl, 0x9e010d57_f17a_4bd5_9b6d_f984f1297d3c);
impl windows_core::RuntimeType for IVpnForegroundActivationOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnForegroundActivationOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Complete: usize,
}
windows_core::imp::define_interface!(IVpnInterfaceId, IVpnInterfaceId_Vtbl, 0x9e2ddca2_1712_4ce4_b179_8c652c6d1011);
impl windows_core::RuntimeType for IVpnInterfaceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnInterfaceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAddressInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnInterfaceIdFactory, IVpnInterfaceIdFactory_Vtbl, 0x9e2ddca2_1712_4ce4_b179_8c652c6d1000);
impl core::ops::Deref for IVpnInterfaceIdFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVpnInterfaceIdFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IVpnInterfaceIdFactory {
    pub fn CreateVpnInterfaceId(&self, address: &[u8]) -> windows_core::Result<VpnInterfaceId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateVpnInterfaceId)(windows_core::Interface::as_raw(this), address.len().try_into().unwrap(), address.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IVpnInterfaceIdFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnInterfaceIdFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateVpnInterfaceId: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnManagementAgent, IVpnManagementAgent_Vtbl, 0x193696cd_a5c4_4abe_852b_785be4cb3e34);
impl windows_core::RuntimeType for IVpnManagementAgent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnManagementAgent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddProfileFromXmlAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddProfileFromObjectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateProfileFromXmlAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateProfileFromObjectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetProfilesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetProfilesAsync: usize,
    pub DeleteProfileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectProfileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub ConnectProfileWithPasswordCredentialAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ConnectProfileWithPasswordCredentialAsync: usize,
    pub DisconnectProfileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnNamespaceAssignment, IVpnNamespaceAssignment_Vtbl, 0xd7f7db18_307d_4c0e_bd62_8fa270bbadd6);
impl windows_core::RuntimeType for IVpnNamespaceAssignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnNamespaceAssignment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetNamespaceList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetNamespaceList: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NamespaceList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NamespaceList: usize,
    pub SetProxyAutoConfigUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProxyAutoConfigUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnNamespaceInfo, IVpnNamespaceInfo_Vtbl, 0x30edfb43_444f_44c5_8167_a35a91f1af94);
impl windows_core::RuntimeType for IVpnNamespaceInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnNamespaceInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Namespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetDnsServers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetDnsServers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DnsServers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DnsServers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetWebProxyServers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetWebProxyServers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WebProxyServers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebProxyServers: usize,
}
windows_core::imp::define_interface!(IVpnNamespaceInfoFactory, IVpnNamespaceInfoFactory_Vtbl, 0xcb3e951a_b0ce_442b_acbb_5f99b202c31c);
impl core::ops::Deref for IVpnNamespaceInfoFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVpnNamespaceInfoFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IVpnNamespaceInfoFactory {
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateVpnNamespaceInfo<P0, P1>(&self, name: &windows_core::HSTRING, dnsserverlist: P0, proxyserverlist: P1) -> windows_core::Result<VpnNamespaceInfo>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVector<super::HostName>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IVector<super::HostName>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateVpnNamespaceInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), dnsserverlist.param().abi(), proxyserverlist.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IVpnNamespaceInfoFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnNamespaceInfoFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateVpnNamespaceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateVpnNamespaceInfo: usize,
}
windows_core::imp::define_interface!(IVpnNativeProfile, IVpnNativeProfile_Vtbl, 0xa4aee29e_6417_4333_9842_f0a66db69802);
impl windows_core::RuntimeType for IVpnNativeProfile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnNativeProfile_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Servers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Servers: usize,
    pub RoutingPolicyType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VpnRoutingPolicyType) -> windows_core::HRESULT,
    pub SetRoutingPolicyType: unsafe extern "system" fn(*mut core::ffi::c_void, VpnRoutingPolicyType) -> windows_core::HRESULT,
    pub NativeProtocolType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VpnNativeProtocolType) -> windows_core::HRESULT,
    pub SetNativeProtocolType: unsafe extern "system" fn(*mut core::ffi::c_void, VpnNativeProtocolType) -> windows_core::HRESULT,
    pub UserAuthenticationMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VpnAuthenticationMethod) -> windows_core::HRESULT,
    pub SetUserAuthenticationMethod: unsafe extern "system" fn(*mut core::ffi::c_void, VpnAuthenticationMethod) -> windows_core::HRESULT,
    pub TunnelAuthenticationMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VpnAuthenticationMethod) -> windows_core::HRESULT,
    pub SetTunnelAuthenticationMethod: unsafe extern "system" fn(*mut core::ffi::c_void, VpnAuthenticationMethod) -> windows_core::HRESULT,
    pub EapConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetEapConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnNativeProfile2, IVpnNativeProfile2_Vtbl, 0x0fec2467_cdb5_4ac7_b5a3_0afb5ec47682);
impl windows_core::RuntimeType for IVpnNativeProfile2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnNativeProfile2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequireVpnClientAppUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRequireVpnClientAppUI: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ConnectionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VpnManagementConnectionStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnPacketBuffer, IVpnPacketBuffer_Vtbl, 0xc2f891fc_4d5c_4a63_b70d_4e307eacce55);
impl windows_core::RuntimeType for IVpnPacketBuffer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnPacketBuffer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, VpnPacketBufferStatus) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VpnPacketBufferStatus) -> windows_core::HRESULT,
    pub SetTransportAffinity: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub TransportAffinity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnPacketBuffer2, IVpnPacketBuffer2_Vtbl, 0x665e91f0_8805_4bf5_a619_2e84882e6b4f);
impl windows_core::RuntimeType for IVpnPacketBuffer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnPacketBuffer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnPacketBuffer3, IVpnPacketBuffer3_Vtbl, 0xe256072f_107b_4c40_b127_5bc53e0ad960);
impl windows_core::RuntimeType for IVpnPacketBuffer3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnPacketBuffer3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetTransportContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TransportContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnPacketBufferFactory, IVpnPacketBufferFactory_Vtbl, 0x9e2ddca2_1712_4ce4_b179_8c652c6d9999);
impl core::ops::Deref for IVpnPacketBufferFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVpnPacketBufferFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IVpnPacketBufferFactory {
    pub fn CreateVpnPacketBuffer<P0>(&self, parentbuffer: P0, offset: u32, length: u32) -> windows_core::Result<VpnPacketBuffer>
    where
        P0: windows_core::Param<VpnPacketBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateVpnPacketBuffer)(windows_core::Interface::as_raw(this), parentbuffer.param().abi(), offset, length, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IVpnPacketBufferFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnPacketBufferFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateVpnPacketBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnPacketBufferList, IVpnPacketBufferList_Vtbl, 0xc2f891fc_4d5c_4a63_b70d_4e307eacce77);
impl windows_core::RuntimeType for IVpnPacketBufferList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnPacketBufferList_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddAtBegin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAtBegin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, VpnPacketBufferStatus) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VpnPacketBufferStatus) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnPacketBufferList2, IVpnPacketBufferList2_Vtbl, 0x3e7acfe5_ea1e_482a_8d98_c065f57d89ea);
impl windows_core::RuntimeType for IVpnPacketBufferList2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnPacketBufferList2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddLeadingPacket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveLeadingPacket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddTrailingPacket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveTrailingPacket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnPickedCredential, IVpnPickedCredential_Vtbl, 0x9a793ac7_8854_4e52_ad97_24dd9a842bce);
impl windows_core::RuntimeType for IVpnPickedCredential {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnPickedCredential_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub PasskeyCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    PasskeyCredential: usize,
    pub AdditionalPin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub OldPasswordCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    OldPasswordCredential: usize,
}
windows_core::imp::define_interface!(IVpnPlugIn, IVpnPlugIn_Vtbl, 0xceb78d07_d0a8_4703_a091_c8c2c0915bc4);
impl core::ops::Deref for IVpnPlugIn {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVpnPlugIn, windows_core::IUnknown, windows_core::IInspectable);
impl IVpnPlugIn {
    pub fn Connect<P0>(&self, channel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<VpnChannel>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Connect)(windows_core::Interface::as_raw(this), channel.param().abi()).ok() }
    }
    pub fn Disconnect<P0>(&self, channel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<VpnChannel>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Disconnect)(windows_core::Interface::as_raw(this), channel.param().abi()).ok() }
    }
    pub fn GetKeepAlivePayload<P0>(&self, channel: P0, keepalivepacket: &mut Option<VpnPacketBuffer>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<VpnChannel>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetKeepAlivePayload)(windows_core::Interface::as_raw(this), channel.param().abi(), keepalivepacket as *mut _ as _).ok() }
    }
    pub fn Encapsulate<P0, P1, P2>(&self, channel: P0, packets: P1, encapulatedpackets: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<VpnChannel>,
        P1: windows_core::Param<VpnPacketBufferList>,
        P2: windows_core::Param<VpnPacketBufferList>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Encapsulate)(windows_core::Interface::as_raw(this), channel.param().abi(), packets.param().abi(), encapulatedpackets.param().abi()).ok() }
    }
    pub fn Decapsulate<P0, P1, P2, P3>(&self, channel: P0, encapbuffer: P1, decapsulatedpackets: P2, controlpacketstosend: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<VpnChannel>,
        P1: windows_core::Param<VpnPacketBuffer>,
        P2: windows_core::Param<VpnPacketBufferList>,
        P3: windows_core::Param<VpnPacketBufferList>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Decapsulate)(windows_core::Interface::as_raw(this), channel.param().abi(), encapbuffer.param().abi(), decapsulatedpackets.param().abi(), controlpacketstosend.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for IVpnPlugIn {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnPlugIn_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetKeepAlivePayload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Encapsulate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Decapsulate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnPlugInProfile, IVpnPlugInProfile_Vtbl, 0x0edf0da4_4f00_4589_8d7b_4bf988f6542c);
impl windows_core::RuntimeType for IVpnPlugInProfile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnPlugInProfile_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ServerUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServerUris: usize,
    pub CustomConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCustomConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub VpnPluginPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetVpnPluginPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnPlugInProfile2, IVpnPlugInProfile2_Vtbl, 0x611c4892_cf94_4ad6_ba99_00f4ff34565e);
impl windows_core::RuntimeType for IVpnPlugInProfile2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnPlugInProfile2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequireVpnClientAppUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRequireVpnClientAppUI: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ConnectionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VpnManagementConnectionStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnPlugInReconnectTransport, IVpnPlugInReconnectTransport_Vtbl, 0x9d5a1092_bb46_4d34_9d88_f217893076f4);
impl core::ops::Deref for IVpnPlugInReconnectTransport {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVpnPlugInReconnectTransport, windows_core::IUnknown, windows_core::IInspectable);
impl IVpnPlugInReconnectTransport {
    pub fn ReconnectTransport<P0, P1>(&self, channel: P0, context: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<VpnChannel>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReconnectTransport)(windows_core::Interface::as_raw(this), channel.param().abi(), context.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for IVpnPlugInReconnectTransport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnPlugInReconnectTransport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReconnectTransport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnProfile, IVpnProfile_Vtbl, 0x7875b751_b0d7_43db_8a93_d3fe2479e56a);
impl core::ops::Deref for IVpnProfile {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVpnProfile, windows_core::IUnknown, windows_core::IInspectable);
impl IVpnProfile {
    pub fn ProfileName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProfileName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProfileName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProfileName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppTriggers(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnAppId>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppTriggers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Routes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Routes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DomainNameInfoList(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainNameInfoList)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrafficFilters(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrafficFilters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RememberCredentials(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RememberCredentials)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRememberCredentials(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRememberCredentials)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlwaysOn(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlwaysOn)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAlwaysOn(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAlwaysOn)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for IVpnProfile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnProfile_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProfileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetProfileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AppTriggers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppTriggers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Routes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Routes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DomainNameInfoList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DomainNameInfoList: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TrafficFilters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TrafficFilters: usize,
    pub RememberCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRememberCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AlwaysOn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAlwaysOn: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnRoute, IVpnRoute_Vtbl, 0xb5731b83_0969_4699_938e_7776db29cfb3);
impl windows_core::RuntimeType for IVpnRoute {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnRoute_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrefixSize: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub PrefixSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnRouteAssignment, IVpnRouteAssignment_Vtbl, 0xdb64de22_ce39_4a76_9550_f61039f80e48);
impl windows_core::RuntimeType for IVpnRouteAssignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnRouteAssignment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetIpv4InclusionRoutes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetIpv4InclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetIpv6InclusionRoutes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetIpv6InclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Ipv4InclusionRoutes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ipv4InclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Ipv6InclusionRoutes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ipv6InclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetIpv4ExclusionRoutes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetIpv4ExclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetIpv6ExclusionRoutes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetIpv6ExclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Ipv4ExclusionRoutes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ipv4ExclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Ipv6ExclusionRoutes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ipv6ExclusionRoutes: usize,
    pub SetExcludeLocalSubnets: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ExcludeLocalSubnets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnRouteFactory, IVpnRouteFactory_Vtbl, 0xbdeab5ff_45cf_4b99_83fb_db3bc2672b02);
impl core::ops::Deref for IVpnRouteFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVpnRouteFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IVpnRouteFactory {
    pub fn CreateVpnRoute<P0>(&self, address: P0, prefixsize: u8) -> windows_core::Result<VpnRoute>
    where
        P0: windows_core::Param<super::HostName>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateVpnRoute)(windows_core::Interface::as_raw(this), address.param().abi(), prefixsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IVpnRouteFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnRouteFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateVpnRoute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnSystemHealth, IVpnSystemHealth_Vtbl, 0x99a8f8af_c0ee_4e75_817a_f231aee5123d);
impl windows_core::RuntimeType for IVpnSystemHealth {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnSystemHealth_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub StatementOfHealth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StatementOfHealth: usize,
}
windows_core::imp::define_interface!(IVpnTrafficFilter, IVpnTrafficFilter_Vtbl, 0x2f691b60_6c9f_47f5_ac36_bb1b042e2c50);
impl windows_core::RuntimeType for IVpnTrafficFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnTrafficFilter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAppId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AppClaims: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppClaims: usize,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VpnIPProtocol) -> windows_core::HRESULT,
    pub SetProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, VpnIPProtocol) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub LocalPortRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LocalPortRanges: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemotePortRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemotePortRanges: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LocalAddressRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LocalAddressRanges: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoteAddressRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoteAddressRanges: usize,
    pub RoutingPolicyType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VpnRoutingPolicyType) -> windows_core::HRESULT,
    pub SetRoutingPolicyType: unsafe extern "system" fn(*mut core::ffi::c_void, VpnRoutingPolicyType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnTrafficFilterAssignment, IVpnTrafficFilterAssignment_Vtbl, 0x56ccd45c_e664_471e_89cd_601603b9e0f3);
impl windows_core::RuntimeType for IVpnTrafficFilterAssignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnTrafficFilterAssignment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub TrafficFilterList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TrafficFilterList: usize,
    pub AllowOutbound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowOutbound: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AllowInbound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowInbound: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVpnTrafficFilterFactory, IVpnTrafficFilterFactory_Vtbl, 0x480d41d5_7f99_474c_86ee_96df168318f1);
impl windows_core::RuntimeType for IVpnTrafficFilterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVpnTrafficFilterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnAppId(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnAppId, windows_core::IUnknown, windows_core::IInspectable);
impl VpnAppId {
    pub fn Type(&self) -> windows_core::Result<VpnAppIdType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetType(&self, value: VpnAppIdType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetValue(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Create(r#type: VpnAppIdType, value: &windows_core::HSTRING) -> windows_core::Result<VpnAppId> {
        Self::IVpnAppIdFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), r#type, core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVpnAppIdFactory<R, F: FnOnce(&IVpnAppIdFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnAppId, IVpnAppIdFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VpnAppId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnAppId>();
}
unsafe impl windows_core::Interface for VpnAppId {
    type Vtable = IVpnAppId_Vtbl;
    const IID: windows_core::GUID = <IVpnAppId as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnAppId {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnAppId";
}
unsafe impl Send for VpnAppId {}
unsafe impl Sync for VpnAppId {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnChannel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnChannel, windows_core::IUnknown, windows_core::IInspectable);
impl VpnChannel {
    pub fn AssociateTransport<P0, P1>(&self, mainoutertunneltransport: P0, optionaloutertunneltransport: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AssociateTransport)(windows_core::Interface::as_raw(this), mainoutertunneltransport.param().abi(), optionaloutertunneltransport.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Start<P0, P1, P2, P3, P4, P5, P6>(&self, assignedclientipv4list: P0, assignedclientipv6list: P1, vpninterfaceid: P2, routescope: P3, namespacescope: P4, mtusize: u32, maxframesize: u32, optimizeforlowcostnetwork: bool, mainoutertunneltransport: P5, optionaloutertunneltransport: P6) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVectorView<super::HostName>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IVectorView<super::HostName>>,
        P2: windows_core::Param<VpnInterfaceId>,
        P3: windows_core::Param<VpnRouteAssignment>,
        P4: windows_core::Param<VpnNamespaceAssignment>,
        P5: windows_core::Param<windows_core::IInspectable>,
        P6: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this), assignedclientipv4list.param().abi(), assignedclientipv6list.param().abi(), vpninterfaceid.param().abi(), routescope.param().abi(), namespacescope.param().abi(), mtusize, maxframesize, optimizeforlowcostnetwork, mainoutertunneltransport.param().abi(), optionaloutertunneltransport.param().abi()).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn RequestCredentials<P0>(&self, credtype: VpnCredentialType, isretry: bool, issinglesignoncredential: bool, certificate: P0) -> windows_core::Result<VpnPickedCredential>
    where
        P0: windows_core::Param<super::super::Security::Cryptography::Certificates::Certificate>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestCredentials)(windows_core::Interface::as_raw(this), credtype, isretry, issinglesignoncredential, certificate.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestVpnPacketBuffer(&self, r#type: VpnDataPathType, vpnpacketbuffer: &mut Option<VpnPacketBuffer>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RequestVpnPacketBuffer)(windows_core::Interface::as_raw(this), r#type, vpnpacketbuffer as *mut _ as _).ok() }
    }
    pub fn LogDiagnosticMessage(&self, message: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).LogDiagnosticMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(message)).ok() }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Configuration(&self) -> windows_core::Result<VpnChannelConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivityChange<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivityChange)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveActivityChange(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveActivityChange)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetPlugInContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPlugInContext)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PlugInContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlugInContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SystemHealth(&self) -> windows_core::Result<VpnSystemHealth> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemHealth)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestCustomPrompt<P0>(&self, customprompt: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVectorView<IVpnCustomPrompt>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RequestCustomPrompt)(windows_core::Interface::as_raw(this), customprompt.param().abi()).ok() }
    }
    pub fn SetErrorMessage(&self, message: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetErrorMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(message)).ok() }
    }
    pub fn SetAllowedSslTlsVersions<P0>(&self, tunneltransport: P0, usetls12: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowedSslTlsVersions)(windows_core::Interface::as_raw(this), tunneltransport.param().abi(), usetls12).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartWithMainTransport<P0, P1, P2, P3, P4, P5>(&self, assignedclientipv4list: P0, assignedclientipv6list: P1, vpninterfaceid: P2, assignedroutes: P3, assigneddomainname: P4, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: P5) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVectorView<super::HostName>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IVectorView<super::HostName>>,
        P2: windows_core::Param<VpnInterfaceId>,
        P3: windows_core::Param<VpnRouteAssignment>,
        P4: windows_core::Param<VpnDomainNameAssignment>,
        P5: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartWithMainTransport)(windows_core::Interface::as_raw(this), assignedclientipv4list.param().abi(), assignedclientipv6list.param().abi(), vpninterfaceid.param().abi(), assignedroutes.param().abi(), assigneddomainname.param().abi(), mtusize, maxframesize, reserved, mainoutertunneltransport.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartExistingTransports<P0, P1, P2, P3, P4>(&self, assignedclientipv4list: P0, assignedclientipv6list: P1, vpninterfaceid: P2, assignedroutes: P3, assigneddomainname: P4, mtusize: u32, maxframesize: u32, reserved: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVectorView<super::HostName>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IVectorView<super::HostName>>,
        P2: windows_core::Param<VpnInterfaceId>,
        P3: windows_core::Param<VpnRouteAssignment>,
        P4: windows_core::Param<VpnDomainNameAssignment>,
    {
        let this = &windows_core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartExistingTransports)(windows_core::Interface::as_raw(this), assignedclientipv4list.param().abi(), assignedclientipv6list.param().abi(), vpninterfaceid.param().abi(), assignedroutes.param().abi(), assigneddomainname.param().abi(), mtusize, maxframesize, reserved).ok() }
    }
    pub fn ActivityStateChange<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityStateChangedArgs>>,
    {
        let this = &windows_core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivityStateChange)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveActivityStateChange(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveActivityStateChange)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetVpnSendPacketBuffer(&self) -> windows_core::Result<VpnPacketBuffer> {
        let this = &windows_core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetVpnSendPacketBuffer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetVpnReceivePacketBuffer(&self) -> windows_core::Result<VpnPacketBuffer> {
        let this = &windows_core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetVpnReceivePacketBuffer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestCustomPromptAsync<P0>(&self, custompromptelement: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVectorView<IVpnCustomPromptElement>>,
    {
        let this = &windows_core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestCustomPromptAsync)(windows_core::Interface::as_raw(this), custompromptelement.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn RequestCredentialsWithCertificateAsync<P0>(&self, credtype: VpnCredentialType, credoptions: u32, certificate: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>>
    where
        P0: windows_core::Param<super::super::Security::Cryptography::Certificates::Certificate>,
    {
        let this = &windows_core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestCredentialsWithCertificateAsync)(windows_core::Interface::as_raw(this), credtype, credoptions, certificate.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestCredentialsWithOptionsAsync(&self, credtype: VpnCredentialType, credoptions: u32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>> {
        let this = &windows_core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestCredentialsWithOptionsAsync)(windows_core::Interface::as_raw(this), credtype, credoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestCredentialsSimpleAsync(&self, credtype: VpnCredentialType) -> windows_core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>> {
        let this = &windows_core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestCredentialsSimpleAsync)(windows_core::Interface::as_raw(this), credtype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TerminateConnection(&self, message: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TerminateConnection)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(message)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartWithTrafficFilter<P0, P1, P2, P3, P4, P5, P6, P7>(&self, assignedclientipv4list: P0, assignedclientipv6list: P1, vpninterfaceid: P2, assignedroutes: P3, assignednamespace: P4, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: P5, optionaloutertunneltransport: P6, assignedtrafficfilters: P7) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVectorView<super::HostName>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IVectorView<super::HostName>>,
        P2: windows_core::Param<VpnInterfaceId>,
        P3: windows_core::Param<VpnRouteAssignment>,
        P4: windows_core::Param<VpnDomainNameAssignment>,
        P5: windows_core::Param<windows_core::IInspectable>,
        P6: windows_core::Param<windows_core::IInspectable>,
        P7: windows_core::Param<VpnTrafficFilterAssignment>,
    {
        let this = &windows_core::Interface::cast::<IVpnChannel2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartWithTrafficFilter)(windows_core::Interface::as_raw(this), assignedclientipv4list.param().abi(), assignedclientipv6list.param().abi(), vpninterfaceid.param().abi(), assignedroutes.param().abi(), assignednamespace.param().abi(), mtusize, maxframesize, reserved, mainoutertunneltransport.param().abi(), optionaloutertunneltransport.param().abi(), assignedtrafficfilters.param().abi()).ok() }
    }
    pub fn AddAndAssociateTransport<P0, P1>(&self, transport: P0, context: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IVpnChannel4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AddAndAssociateTransport)(windows_core::Interface::as_raw(this), transport.param().abi(), context.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartWithMultipleTransports<P0, P1, P2, P3, P4, P5, P6>(&self, assignedclientipv4addresses: P0, assignedclientipv6addresses: P1, vpninterfaceid: P2, assignedroutes: P3, assignednamespace: P4, mtusize: u32, maxframesize: u32, reserved: bool, transports: P5, assignedtrafficfilters: P6) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<super::HostName>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<super::HostName>>,
        P2: windows_core::Param<VpnInterfaceId>,
        P3: windows_core::Param<VpnRouteAssignment>,
        P4: windows_core::Param<VpnDomainNameAssignment>,
        P5: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::IInspectable>>,
        P6: windows_core::Param<VpnTrafficFilterAssignment>,
    {
        let this = &windows_core::Interface::cast::<IVpnChannel4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartWithMultipleTransports)(windows_core::Interface::as_raw(this), assignedclientipv4addresses.param().abi(), assignedclientipv6addresses.param().abi(), vpninterfaceid.param().abi(), assignedroutes.param().abi(), assignednamespace.param().abi(), mtusize, maxframesize, reserved, transports.param().abi(), assignedtrafficfilters.param().abi()).ok() }
    }
    pub fn ReplaceAndAssociateTransport<P0, P1>(&self, transport: P0, context: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IVpnChannel4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAndAssociateTransport)(windows_core::Interface::as_raw(this), transport.param().abi(), context.param().abi()).ok() }
    }
    pub fn StartReconnectingTransport<P0, P1>(&self, transport: P0, context: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IVpnChannel4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartReconnectingTransport)(windows_core::Interface::as_raw(this), transport.param().abi(), context.param().abi()).ok() }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn GetSlotTypeForTransportContext<P0>(&self, context: P0) -> windows_core::Result<super::Sockets::ControlChannelTriggerStatus>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IVpnChannel4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSlotTypeForTransportContext)(windows_core::Interface::as_raw(this), context.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn CurrentRequestTransportContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IVpnChannel4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentRequestTransportContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendVpnReceivePacketBuffer<P0>(&self, decapsulatedpacketbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<VpnPacketBuffer>,
    {
        let this = &windows_core::Interface::cast::<IVpnChannel5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AppendVpnReceivePacketBuffer)(windows_core::Interface::as_raw(this), decapsulatedpacketbuffer.param().abi()).ok() }
    }
    pub fn AppendVpnSendPacketBuffer<P0>(&self, encapsulatedpacketbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<VpnPacketBuffer>,
    {
        let this = &windows_core::Interface::cast::<IVpnChannel5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AppendVpnSendPacketBuffer)(windows_core::Interface::as_raw(this), encapsulatedpacketbuffer.param().abi()).ok() }
    }
    pub fn FlushVpnReceivePacketBuffers(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnChannel5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).FlushVpnReceivePacketBuffers)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FlushVpnSendPacketBuffers(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnChannel5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).FlushVpnSendPacketBuffers)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ActivateForeground<P0>(&self, packagerelativeappid: &windows_core::HSTRING, sharedcontext: P0) -> windows_core::Result<super::super::Foundation::Collections::ValueSet>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::ValueSet>,
    {
        let this = &windows_core::Interface::cast::<IVpnChannel6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivateForeground)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagerelativeappid), sharedcontext.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessEventAsync<P0, P1>(thirdpartyplugin: P0, event: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        Self::IVpnChannelStatics(|this| unsafe { (windows_core::Interface::vtable(this).ProcessEventAsync)(windows_core::Interface::as_raw(this), thirdpartyplugin.param().abi(), event.param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IVpnChannelStatics<R, F: FnOnce(&IVpnChannelStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnChannel, IVpnChannelStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VpnChannel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnChannel>();
}
unsafe impl windows_core::Interface for VpnChannel {
    type Vtable = IVpnChannel_Vtbl;
    const IID: windows_core::GUID = <IVpnChannel as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnChannel {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnChannel";
}
unsafe impl Send for VpnChannel {}
unsafe impl Sync for VpnChannel {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnChannelActivityEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnChannelActivityEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl VpnChannelActivityEventArgs {
    pub fn Type(&self) -> windows_core::Result<VpnChannelActivityEventType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VpnChannelActivityEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnChannelActivityEventArgs>();
}
unsafe impl windows_core::Interface for VpnChannelActivityEventArgs {
    type Vtable = IVpnChannelActivityEventArgs_Vtbl;
    const IID: windows_core::GUID = <IVpnChannelActivityEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnChannelActivityEventArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnChannelActivityEventArgs";
}
unsafe impl Send for VpnChannelActivityEventArgs {}
unsafe impl Sync for VpnChannelActivityEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnChannelActivityStateChangedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnChannelActivityStateChangedArgs, windows_core::IUnknown, windows_core::IInspectable);
impl VpnChannelActivityStateChangedArgs {
    pub fn ActivityState(&self) -> windows_core::Result<VpnChannelActivityEventType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivityState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VpnChannelActivityStateChangedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnChannelActivityStateChangedArgs>();
}
unsafe impl windows_core::Interface for VpnChannelActivityStateChangedArgs {
    type Vtable = IVpnChannelActivityStateChangedArgs_Vtbl;
    const IID: windows_core::GUID = <IVpnChannelActivityStateChangedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnChannelActivityStateChangedArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnChannelActivityStateChangedArgs";
}
unsafe impl Send for VpnChannelActivityStateChangedArgs {}
unsafe impl Sync for VpnChannelActivityStateChangedArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnChannelConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnChannelConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl VpnChannelConfiguration {
    pub fn ServerServiceName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerServiceName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServerHostNameList(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerHostNameList)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CustomField(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomField)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServerUris(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>> {
        let this = &windows_core::Interface::cast::<IVpnChannelConfiguration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VpnChannelConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnChannelConfiguration>();
}
unsafe impl windows_core::Interface for VpnChannelConfiguration {
    type Vtable = IVpnChannelConfiguration_Vtbl;
    const IID: windows_core::GUID = <IVpnChannelConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnChannelConfiguration {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnChannelConfiguration";
}
unsafe impl Send for VpnChannelConfiguration {}
unsafe impl Sync for VpnChannelConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnCredential(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnCredential, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VpnCredential, IVpnCredential);
impl VpnCredential {
    #[cfg(feature = "Security_Credentials")]
    pub fn PasskeyCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PasskeyCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn CertificateCredential(&self) -> windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CertificateCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AdditionalPin(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdditionalPin)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn OldPasswordCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OldPasswordCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VpnCredential {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnCredential>();
}
unsafe impl windows_core::Interface for VpnCredential {
    type Vtable = IVpnCredential_Vtbl;
    const IID: windows_core::GUID = <IVpnCredential as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnCredential {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCredential";
}
unsafe impl Send for VpnCredential {}
unsafe impl Sync for VpnCredential {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnCustomCheckBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnCustomCheckBox, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VpnCustomCheckBox, IVpnCustomPrompt);
impl VpnCustomCheckBox {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnCustomCheckBox, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetInitialCheckState(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInitialCheckState)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InitialCheckState(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InitialCheckState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Checked(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Checked)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLabel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Label)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCompulsory)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compulsory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBordered)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bordered(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bordered)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VpnCustomCheckBox {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnCustomCheckBox>();
}
unsafe impl windows_core::Interface for VpnCustomCheckBox {
    type Vtable = IVpnCustomCheckBox_Vtbl;
    const IID: windows_core::GUID = <IVpnCustomCheckBox as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnCustomCheckBox {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomCheckBox";
}
unsafe impl Send for VpnCustomCheckBox {}
unsafe impl Sync for VpnCustomCheckBox {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnCustomComboBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnCustomComboBox, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VpnCustomComboBox, IVpnCustomPrompt);
impl VpnCustomComboBox {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnCustomComboBox, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetOptionsText<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOptionsText)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionsText(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptionsText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Selected(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Selected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLabel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Label)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCompulsory)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compulsory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBordered)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bordered(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bordered)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VpnCustomComboBox {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnCustomComboBox>();
}
unsafe impl windows_core::Interface for VpnCustomComboBox {
    type Vtable = IVpnCustomComboBox_Vtbl;
    const IID: windows_core::GUID = <IVpnCustomComboBox as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnCustomComboBox {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomComboBox";
}
unsafe impl Send for VpnCustomComboBox {}
unsafe impl Sync for VpnCustomComboBox {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnCustomEditBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnCustomEditBox, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VpnCustomEditBox, IVpnCustomPrompt);
impl VpnCustomEditBox {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnCustomEditBox, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetDefaultText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DefaultText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNoEcho(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNoEcho)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NoEcho(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NoEcho)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLabel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Label)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCompulsory)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compulsory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBordered)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bordered(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bordered)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VpnCustomEditBox {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnCustomEditBox>();
}
unsafe impl windows_core::Interface for VpnCustomEditBox {
    type Vtable = IVpnCustomEditBox_Vtbl;
    const IID: windows_core::GUID = <IVpnCustomEditBox as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnCustomEditBox {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomEditBox";
}
unsafe impl Send for VpnCustomEditBox {}
unsafe impl Sync for VpnCustomEditBox {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnCustomErrorBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnCustomErrorBox, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VpnCustomErrorBox, IVpnCustomPrompt);
impl VpnCustomErrorBox {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnCustomErrorBox, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLabel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Label)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCompulsory)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compulsory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBordered)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bordered(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bordered)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VpnCustomErrorBox {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnCustomErrorBox>();
}
unsafe impl windows_core::Interface for VpnCustomErrorBox {
    type Vtable = IVpnCustomErrorBox_Vtbl;
    const IID: windows_core::GUID = <IVpnCustomErrorBox as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnCustomErrorBox {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomErrorBox";
}
unsafe impl Send for VpnCustomErrorBox {}
unsafe impl Sync for VpnCustomErrorBox {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnCustomPromptBooleanInput(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnCustomPromptBooleanInput, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VpnCustomPromptBooleanInput, IVpnCustomPromptElement);
impl VpnCustomPromptBooleanInput {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnCustomPromptBooleanInput, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetInitialValue(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInitialValue)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InitialValue(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InitialValue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Value(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCompulsory)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compulsory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEmphasized(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetEmphasized)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Emphasized(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Emphasized)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VpnCustomPromptBooleanInput {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnCustomPromptBooleanInput>();
}
unsafe impl windows_core::Interface for VpnCustomPromptBooleanInput {
    type Vtable = IVpnCustomPromptBooleanInput_Vtbl;
    const IID: windows_core::GUID = <IVpnCustomPromptBooleanInput as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnCustomPromptBooleanInput {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomPromptBooleanInput";
}
unsafe impl Send for VpnCustomPromptBooleanInput {}
unsafe impl Sync for VpnCustomPromptBooleanInput {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnCustomPromptOptionSelector(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnCustomPromptOptionSelector, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VpnCustomPromptOptionSelector, IVpnCustomPromptElement);
impl VpnCustomPromptOptionSelector {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnCustomPromptOptionSelector, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCompulsory)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compulsory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEmphasized(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetEmphasized)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Emphasized(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Emphasized)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Options(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Options)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectedIndex(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VpnCustomPromptOptionSelector {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnCustomPromptOptionSelector>();
}
unsafe impl windows_core::Interface for VpnCustomPromptOptionSelector {
    type Vtable = IVpnCustomPromptOptionSelector_Vtbl;
    const IID: windows_core::GUID = <IVpnCustomPromptOptionSelector as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnCustomPromptOptionSelector {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomPromptOptionSelector";
}
unsafe impl Send for VpnCustomPromptOptionSelector {}
unsafe impl Sync for VpnCustomPromptOptionSelector {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnCustomPromptText(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnCustomPromptText, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VpnCustomPromptText, IVpnCustomPromptElement);
impl VpnCustomPromptText {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnCustomPromptText, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCompulsory)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compulsory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEmphasized(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetEmphasized)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Emphasized(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Emphasized)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VpnCustomPromptText {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnCustomPromptText>();
}
unsafe impl windows_core::Interface for VpnCustomPromptText {
    type Vtable = IVpnCustomPromptText_Vtbl;
    const IID: windows_core::GUID = <IVpnCustomPromptText as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnCustomPromptText {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomPromptText";
}
unsafe impl Send for VpnCustomPromptText {}
unsafe impl Sync for VpnCustomPromptText {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnCustomPromptTextInput(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnCustomPromptTextInput, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VpnCustomPromptTextInput, IVpnCustomPromptElement);
impl VpnCustomPromptTextInput {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnCustomPromptTextInput, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCompulsory)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compulsory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEmphasized(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetEmphasized)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Emphasized(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Emphasized)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPlaceholderText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPlaceholderText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn PlaceholderText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaceholderText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIsTextHidden(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsTextHidden)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsTextHidden(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTextHidden)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VpnCustomPromptTextInput {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnCustomPromptTextInput>();
}
unsafe impl windows_core::Interface for VpnCustomPromptTextInput {
    type Vtable = IVpnCustomPromptTextInput_Vtbl;
    const IID: windows_core::GUID = <IVpnCustomPromptTextInput as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnCustomPromptTextInput {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomPromptTextInput";
}
unsafe impl Send for VpnCustomPromptTextInput {}
unsafe impl Sync for VpnCustomPromptTextInput {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnCustomTextBox(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnCustomTextBox, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VpnCustomTextBox, IVpnCustomPrompt);
impl VpnCustomTextBox {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnCustomTextBox, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLabel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Label)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCompulsory)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compulsory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBordered)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bordered(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bordered)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDisplayText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VpnCustomTextBox {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnCustomTextBox>();
}
unsafe impl windows_core::Interface for VpnCustomTextBox {
    type Vtable = IVpnCustomTextBox_Vtbl;
    const IID: windows_core::GUID = <IVpnCustomTextBox as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnCustomTextBox {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomTextBox";
}
unsafe impl Send for VpnCustomTextBox {}
unsafe impl Sync for VpnCustomTextBox {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnDomainNameAssignment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnDomainNameAssignment, windows_core::IUnknown, windows_core::IInspectable);
impl VpnDomainNameAssignment {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnDomainNameAssignment, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DomainNameList(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainNameList)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProxyAutoConfigurationUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProxyAutoConfigurationUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ProxyAutoConfigurationUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProxyAutoConfigurationUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VpnDomainNameAssignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnDomainNameAssignment>();
}
unsafe impl windows_core::Interface for VpnDomainNameAssignment {
    type Vtable = IVpnDomainNameAssignment_Vtbl;
    const IID: windows_core::GUID = <IVpnDomainNameAssignment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnDomainNameAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnDomainNameAssignment";
}
unsafe impl Send for VpnDomainNameAssignment {}
unsafe impl Sync for VpnDomainNameAssignment {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnDomainNameInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnDomainNameInfo, windows_core::IUnknown, windows_core::IInspectable);
impl VpnDomainNameInfo {
    pub fn SetDomainName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::HostName>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDomainName)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn DomainName(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDomainNameType(&self, value: VpnDomainNameType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDomainNameType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DomainNameType(&self) -> windows_core::Result<VpnDomainNameType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainNameType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DnsServers(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DnsServers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebProxyServers(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebProxyServers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebProxyUris(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = &windows_core::Interface::cast::<IVpnDomainNameInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebProxyUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateVpnDomainNameInfo<P0, P1>(name: &windows_core::HSTRING, nametype: VpnDomainNameType, dnsserverlist: P0, proxyserverlist: P1) -> windows_core::Result<VpnDomainNameInfo>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<super::HostName>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<super::HostName>>,
    {
        Self::IVpnDomainNameInfoFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateVpnDomainNameInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), nametype, dnsserverlist.param().abi(), proxyserverlist.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVpnDomainNameInfoFactory<R, F: FnOnce(&IVpnDomainNameInfoFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnDomainNameInfo, IVpnDomainNameInfoFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VpnDomainNameInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnDomainNameInfo>();
}
unsafe impl windows_core::Interface for VpnDomainNameInfo {
    type Vtable = IVpnDomainNameInfo_Vtbl;
    const IID: windows_core::GUID = <IVpnDomainNameInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnDomainNameInfo {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnDomainNameInfo";
}
unsafe impl Send for VpnDomainNameInfo {}
unsafe impl Sync for VpnDomainNameInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnForegroundActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnForegroundActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(VpnForegroundActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser);
impl VpnForegroundActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProfileName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProfileName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SharedContext(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SharedContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActivationOperation(&self) -> windows_core::Result<VpnForegroundActivationOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivationOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VpnForegroundActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnForegroundActivatedEventArgs>();
}
unsafe impl windows_core::Interface for VpnForegroundActivatedEventArgs {
    type Vtable = IVpnForegroundActivatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IVpnForegroundActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnForegroundActivatedEventArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnForegroundActivatedEventArgs";
}
unsafe impl Send for VpnForegroundActivatedEventArgs {}
unsafe impl Sync for VpnForegroundActivatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnForegroundActivationOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnForegroundActivationOperation, windows_core::IUnknown, windows_core::IInspectable);
impl VpnForegroundActivationOperation {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Complete<P0>(&self, result: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::ValueSet>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this), result.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for VpnForegroundActivationOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnForegroundActivationOperation>();
}
unsafe impl windows_core::Interface for VpnForegroundActivationOperation {
    type Vtable = IVpnForegroundActivationOperation_Vtbl;
    const IID: windows_core::GUID = <IVpnForegroundActivationOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnForegroundActivationOperation {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnForegroundActivationOperation";
}
unsafe impl Send for VpnForegroundActivationOperation {}
unsafe impl Sync for VpnForegroundActivationOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnInterfaceId(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnInterfaceId, windows_core::IUnknown, windows_core::IInspectable);
impl VpnInterfaceId {
    pub fn GetAddressInfo(&self, id: &mut windows_core::Array<u8>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetAddressInfo)(windows_core::Interface::as_raw(this), id.set_abi_len(), id as *mut _ as _).ok() }
    }
    pub fn CreateVpnInterfaceId(address: &[u8]) -> windows_core::Result<VpnInterfaceId> {
        Self::IVpnInterfaceIdFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateVpnInterfaceId)(windows_core::Interface::as_raw(this), address.len().try_into().unwrap(), address.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVpnInterfaceIdFactory<R, F: FnOnce(&IVpnInterfaceIdFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnInterfaceId, IVpnInterfaceIdFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VpnInterfaceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnInterfaceId>();
}
unsafe impl windows_core::Interface for VpnInterfaceId {
    type Vtable = IVpnInterfaceId_Vtbl;
    const IID: windows_core::GUID = <IVpnInterfaceId as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnInterfaceId {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnInterfaceId";
}
unsafe impl Send for VpnInterfaceId {}
unsafe impl Sync for VpnInterfaceId {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnManagementAgent(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnManagementAgent, windows_core::IUnknown, windows_core::IInspectable);
impl VpnManagementAgent {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnManagementAgent, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AddProfileFromXmlAsync(&self, xml: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddProfileFromXmlAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xml), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddProfileFromObjectAsync<P0>(&self, profile: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>
    where
        P0: windows_core::Param<IVpnProfile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddProfileFromObjectAsync)(windows_core::Interface::as_raw(this), profile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateProfileFromXmlAsync(&self, xml: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateProfileFromXmlAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xml), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateProfileFromObjectAsync<P0>(&self, profile: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>
    where
        P0: windows_core::Param<IVpnProfile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateProfileFromObjectAsync)(windows_core::Interface::as_raw(this), profile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetProfilesAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IVpnProfile>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetProfilesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteProfileAsync<P0>(&self, profile: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>
    where
        P0: windows_core::Param<IVpnProfile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteProfileAsync)(windows_core::Interface::as_raw(this), profile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectProfileAsync<P0>(&self, profile: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>
    where
        P0: windows_core::Param<IVpnProfile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectProfileAsync)(windows_core::Interface::as_raw(this), profile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ConnectProfileWithPasswordCredentialAsync<P0, P1>(&self, profile: P0, passwordcredential: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>
    where
        P0: windows_core::Param<IVpnProfile>,
        P1: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectProfileWithPasswordCredentialAsync)(windows_core::Interface::as_raw(this), profile.param().abi(), passwordcredential.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisconnectProfileAsync<P0>(&self, profile: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>
    where
        P0: windows_core::Param<IVpnProfile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisconnectProfileAsync)(windows_core::Interface::as_raw(this), profile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VpnManagementAgent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnManagementAgent>();
}
unsafe impl windows_core::Interface for VpnManagementAgent {
    type Vtable = IVpnManagementAgent_Vtbl;
    const IID: windows_core::GUID = <IVpnManagementAgent as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnManagementAgent {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnManagementAgent";
}
unsafe impl Send for VpnManagementAgent {}
unsafe impl Sync for VpnManagementAgent {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnNamespaceAssignment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnNamespaceAssignment, windows_core::IUnknown, windows_core::IInspectable);
impl VpnNamespaceAssignment {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnNamespaceAssignment, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetNamespaceList<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVector<VpnNamespaceInfo>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNamespaceList)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn NamespaceList(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnNamespaceInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceList)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProxyAutoConfigUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProxyAutoConfigUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ProxyAutoConfigUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProxyAutoConfigUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VpnNamespaceAssignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnNamespaceAssignment>();
}
unsafe impl windows_core::Interface for VpnNamespaceAssignment {
    type Vtable = IVpnNamespaceAssignment_Vtbl;
    const IID: windows_core::GUID = <IVpnNamespaceAssignment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnNamespaceAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnNamespaceAssignment";
}
unsafe impl Send for VpnNamespaceAssignment {}
unsafe impl Sync for VpnNamespaceAssignment {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnNamespaceInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnNamespaceInfo, windows_core::IUnknown, windows_core::IInspectable);
impl VpnNamespaceInfo {
    pub fn SetNamespace(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNamespace)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Namespace(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Namespace)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetDnsServers<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVector<super::HostName>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDnsServers)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DnsServers(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DnsServers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetWebProxyServers<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVector<super::HostName>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWebProxyServers)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebProxyServers(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WebProxyServers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateVpnNamespaceInfo<P0, P1>(name: &windows_core::HSTRING, dnsserverlist: P0, proxyserverlist: P1) -> windows_core::Result<VpnNamespaceInfo>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVector<super::HostName>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IVector<super::HostName>>,
    {
        Self::IVpnNamespaceInfoFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateVpnNamespaceInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), dnsserverlist.param().abi(), proxyserverlist.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVpnNamespaceInfoFactory<R, F: FnOnce(&IVpnNamespaceInfoFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnNamespaceInfo, IVpnNamespaceInfoFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VpnNamespaceInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnNamespaceInfo>();
}
unsafe impl windows_core::Interface for VpnNamespaceInfo {
    type Vtable = IVpnNamespaceInfo_Vtbl;
    const IID: windows_core::GUID = <IVpnNamespaceInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnNamespaceInfo {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnNamespaceInfo";
}
unsafe impl Send for VpnNamespaceInfo {}
unsafe impl Sync for VpnNamespaceInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnNativeProfile(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnNativeProfile, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VpnNativeProfile, IVpnProfile);
impl VpnNativeProfile {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnNativeProfile, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Servers(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Servers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RoutingPolicyType(&self) -> windows_core::Result<VpnRoutingPolicyType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoutingPolicyType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRoutingPolicyType(&self, value: VpnRoutingPolicyType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRoutingPolicyType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NativeProtocolType(&self) -> windows_core::Result<VpnNativeProtocolType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NativeProtocolType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNativeProtocolType(&self, value: VpnNativeProtocolType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNativeProtocolType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UserAuthenticationMethod(&self) -> windows_core::Result<VpnAuthenticationMethod> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserAuthenticationMethod)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUserAuthenticationMethod(&self, value: VpnAuthenticationMethod) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUserAuthenticationMethod)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TunnelAuthenticationMethod(&self) -> windows_core::Result<VpnAuthenticationMethod> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TunnelAuthenticationMethod)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTunnelAuthenticationMethod(&self, value: VpnAuthenticationMethod) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTunnelAuthenticationMethod)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EapConfiguration(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EapConfiguration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetEapConfiguration(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEapConfiguration)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn RequireVpnClientAppUI(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnNativeProfile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequireVpnClientAppUI)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRequireVpnClientAppUI(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnNativeProfile2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRequireVpnClientAppUI)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ConnectionStatus(&self) -> windows_core::Result<VpnManagementConnectionStatus> {
        let this = &windows_core::Interface::cast::<IVpnNativeProfile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProfileName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProfileName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProfileName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProfileName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppTriggers(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnAppId>> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppTriggers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Routes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Routes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DomainNameInfoList(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainNameInfoList)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrafficFilters(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrafficFilters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RememberCredentials(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RememberCredentials)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRememberCredentials(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRememberCredentials)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlwaysOn(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlwaysOn)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAlwaysOn(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAlwaysOn)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for VpnNativeProfile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnNativeProfile>();
}
unsafe impl windows_core::Interface for VpnNativeProfile {
    type Vtable = IVpnNativeProfile_Vtbl;
    const IID: windows_core::GUID = <IVpnNativeProfile as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnNativeProfile {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnNativeProfile";
}
unsafe impl Send for VpnNativeProfile {}
unsafe impl Sync for VpnNativeProfile {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnPacketBuffer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnPacketBuffer, windows_core::IUnknown, windows_core::IInspectable);
impl VpnPacketBuffer {
    #[cfg(feature = "Storage_Streams")]
    pub fn Buffer(&self) -> windows_core::Result<super::super::Storage::Streams::Buffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Buffer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStatus(&self, value: VpnPacketBufferStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Status(&self) -> windows_core::Result<VpnPacketBufferStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransportAffinity(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTransportAffinity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TransportAffinity(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportAffinity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppId(&self) -> windows_core::Result<VpnAppId> {
        let this = &windows_core::Interface::cast::<IVpnPacketBuffer2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTransportContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IVpnPacketBuffer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTransportContext)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn TransportContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IVpnPacketBuffer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateVpnPacketBuffer<P0>(parentbuffer: P0, offset: u32, length: u32) -> windows_core::Result<VpnPacketBuffer>
    where
        P0: windows_core::Param<VpnPacketBuffer>,
    {
        Self::IVpnPacketBufferFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateVpnPacketBuffer)(windows_core::Interface::as_raw(this), parentbuffer.param().abi(), offset, length, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVpnPacketBufferFactory<R, F: FnOnce(&IVpnPacketBufferFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnPacketBuffer, IVpnPacketBufferFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VpnPacketBuffer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnPacketBuffer>();
}
unsafe impl windows_core::Interface for VpnPacketBuffer {
    type Vtable = IVpnPacketBuffer_Vtbl;
    const IID: windows_core::GUID = <IVpnPacketBuffer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnPacketBuffer {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnPacketBuffer";
}
unsafe impl Send for VpnPacketBuffer {}
unsafe impl Sync for VpnPacketBuffer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnPacketBufferList(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnPacketBufferList, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(VpnPacketBufferList, super::super::Foundation::Collections::IIterable::<VpnPacketBuffer>);
impl VpnPacketBufferList {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::Foundation::Collections::IIterator<VpnPacketBuffer>> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IIterable<VpnPacketBuffer>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Append<P0>(&self, nextvpnpacketbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<VpnPacketBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), nextvpnpacketbuffer.param().abi()).ok() }
    }
    pub fn AddAtBegin<P0>(&self, nextvpnpacketbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<VpnPacketBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddAtBegin)(windows_core::Interface::as_raw(this), nextvpnpacketbuffer.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<VpnPacketBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveAtBegin(&self) -> windows_core::Result<VpnPacketBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveAtBegin)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetStatus(&self, value: VpnPacketBufferStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Status(&self) -> windows_core::Result<VpnPacketBufferStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VpnPacketBufferList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnPacketBufferList>();
}
unsafe impl windows_core::Interface for VpnPacketBufferList {
    type Vtable = IVpnPacketBufferList_Vtbl;
    const IID: windows_core::GUID = <IVpnPacketBufferList as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnPacketBufferList {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnPacketBufferList";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for VpnPacketBufferList {
    type Item = VpnPacketBuffer;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &VpnPacketBufferList {
    type Item = VpnPacketBuffer;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
unsafe impl Send for VpnPacketBufferList {}
unsafe impl Sync for VpnPacketBufferList {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnPickedCredential(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnPickedCredential, windows_core::IUnknown, windows_core::IInspectable);
impl VpnPickedCredential {
    #[cfg(feature = "Security_Credentials")]
    pub fn PasskeyCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PasskeyCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AdditionalPin(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdditionalPin)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn OldPasswordCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OldPasswordCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VpnPickedCredential {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnPickedCredential>();
}
unsafe impl windows_core::Interface for VpnPickedCredential {
    type Vtable = IVpnPickedCredential_Vtbl;
    const IID: windows_core::GUID = <IVpnPickedCredential as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnPickedCredential {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnPickedCredential";
}
unsafe impl Send for VpnPickedCredential {}
unsafe impl Sync for VpnPickedCredential {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnPlugInProfile(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnPlugInProfile, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VpnPlugInProfile, IVpnProfile);
impl VpnPlugInProfile {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnPlugInProfile, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServerUris(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CustomConfiguration(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomConfiguration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCustomConfiguration(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCustomConfiguration)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn VpnPluginPackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VpnPluginPackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetVpnPluginPackageFamilyName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVpnPluginPackageFamilyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn RequireVpnClientAppUI(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnPlugInProfile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequireVpnClientAppUI)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRequireVpnClientAppUI(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnPlugInProfile2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRequireVpnClientAppUI)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ConnectionStatus(&self) -> windows_core::Result<VpnManagementConnectionStatus> {
        let this = &windows_core::Interface::cast::<IVpnPlugInProfile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProfileName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProfileName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProfileName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProfileName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppTriggers(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnAppId>> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppTriggers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Routes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Routes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DomainNameInfoList(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainNameInfoList)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrafficFilters(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrafficFilters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RememberCredentials(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RememberCredentials)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRememberCredentials(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRememberCredentials)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlwaysOn(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlwaysOn)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAlwaysOn(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVpnProfile>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAlwaysOn)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for VpnPlugInProfile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnPlugInProfile>();
}
unsafe impl windows_core::Interface for VpnPlugInProfile {
    type Vtable = IVpnPlugInProfile_Vtbl;
    const IID: windows_core::GUID = <IVpnPlugInProfile as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnPlugInProfile {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnPlugInProfile";
}
unsafe impl Send for VpnPlugInProfile {}
unsafe impl Sync for VpnPlugInProfile {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnRoute(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnRoute, windows_core::IUnknown, windows_core::IInspectable);
impl VpnRoute {
    pub fn SetAddress<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::HostName>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAddress)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Address(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Address)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPrefixSize(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrefixSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PrefixSize(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrefixSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateVpnRoute<P0>(address: P0, prefixsize: u8) -> windows_core::Result<VpnRoute>
    where
        P0: windows_core::Param<super::HostName>,
    {
        Self::IVpnRouteFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateVpnRoute)(windows_core::Interface::as_raw(this), address.param().abi(), prefixsize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVpnRouteFactory<R, F: FnOnce(&IVpnRouteFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnRoute, IVpnRouteFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VpnRoute {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnRoute>();
}
unsafe impl windows_core::Interface for VpnRoute {
    type Vtable = IVpnRoute_Vtbl;
    const IID: windows_core::GUID = <IVpnRoute as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnRoute {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnRoute";
}
unsafe impl Send for VpnRoute {}
unsafe impl Sync for VpnRoute {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnRouteAssignment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnRouteAssignment, windows_core::IUnknown, windows_core::IInspectable);
impl VpnRouteAssignment {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnRouteAssignment, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetIpv4InclusionRoutes<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVector<VpnRoute>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIpv4InclusionRoutes)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetIpv6InclusionRoutes<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVector<VpnRoute>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIpv6InclusionRoutes)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ipv4InclusionRoutes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ipv4InclusionRoutes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ipv6InclusionRoutes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ipv6InclusionRoutes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetIpv4ExclusionRoutes<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVector<VpnRoute>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIpv4ExclusionRoutes)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetIpv6ExclusionRoutes<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVector<VpnRoute>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIpv6ExclusionRoutes)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ipv4ExclusionRoutes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ipv4ExclusionRoutes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ipv6ExclusionRoutes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ipv6ExclusionRoutes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetExcludeLocalSubnets(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExcludeLocalSubnets)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExcludeLocalSubnets(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExcludeLocalSubnets)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VpnRouteAssignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnRouteAssignment>();
}
unsafe impl windows_core::Interface for VpnRouteAssignment {
    type Vtable = IVpnRouteAssignment_Vtbl;
    const IID: windows_core::GUID = <IVpnRouteAssignment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnRouteAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnRouteAssignment";
}
unsafe impl Send for VpnRouteAssignment {}
unsafe impl Sync for VpnRouteAssignment {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnSystemHealth(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnSystemHealth, windows_core::IUnknown, windows_core::IInspectable);
impl VpnSystemHealth {
    #[cfg(feature = "Storage_Streams")]
    pub fn StatementOfHealth(&self) -> windows_core::Result<super::super::Storage::Streams::Buffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatementOfHealth)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VpnSystemHealth {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnSystemHealth>();
}
unsafe impl windows_core::Interface for VpnSystemHealth {
    type Vtable = IVpnSystemHealth_Vtbl;
    const IID: windows_core::GUID = <IVpnSystemHealth as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnSystemHealth {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnSystemHealth";
}
unsafe impl Send for VpnSystemHealth {}
unsafe impl Sync for VpnSystemHealth {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnTrafficFilter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnTrafficFilter, windows_core::IUnknown, windows_core::IInspectable);
impl VpnTrafficFilter {
    pub fn AppId(&self) -> windows_core::Result<VpnAppId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAppId<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<VpnAppId>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAppId)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppClaims(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppClaims)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Protocol(&self) -> windows_core::Result<VpnIPProtocol> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Protocol)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProtocol(&self, value: VpnIPProtocol) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProtocol)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn LocalPortRanges(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalPortRanges)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemotePortRanges(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemotePortRanges)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn LocalAddressRanges(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalAddressRanges)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoteAddressRanges(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteAddressRanges)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RoutingPolicyType(&self) -> windows_core::Result<VpnRoutingPolicyType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoutingPolicyType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRoutingPolicyType(&self, value: VpnRoutingPolicyType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRoutingPolicyType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<P0>(appid: P0) -> windows_core::Result<VpnTrafficFilter>
    where
        P0: windows_core::Param<VpnAppId>,
    {
        Self::IVpnTrafficFilterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), appid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVpnTrafficFilterFactory<R, F: FnOnce(&IVpnTrafficFilterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnTrafficFilter, IVpnTrafficFilterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VpnTrafficFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnTrafficFilter>();
}
unsafe impl windows_core::Interface for VpnTrafficFilter {
    type Vtable = IVpnTrafficFilter_Vtbl;
    const IID: windows_core::GUID = <IVpnTrafficFilter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnTrafficFilter {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnTrafficFilter";
}
unsafe impl Send for VpnTrafficFilter {}
unsafe impl Sync for VpnTrafficFilter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VpnTrafficFilterAssignment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VpnTrafficFilterAssignment, windows_core::IUnknown, windows_core::IInspectable);
impl VpnTrafficFilterAssignment {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VpnTrafficFilterAssignment, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrafficFilterList(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrafficFilterList)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AllowOutbound(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowOutbound)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowOutbound(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowOutbound)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowInbound(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowInbound)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowInbound(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowInbound)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for VpnTrafficFilterAssignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVpnTrafficFilterAssignment>();
}
unsafe impl windows_core::Interface for VpnTrafficFilterAssignment {
    type Vtable = IVpnTrafficFilterAssignment_Vtbl;
    const IID: windows_core::GUID = <IVpnTrafficFilterAssignment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VpnTrafficFilterAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnTrafficFilterAssignment";
}
unsafe impl Send for VpnTrafficFilterAssignment {}
unsafe impl Sync for VpnTrafficFilterAssignment {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VpnAppIdType(pub i32);
impl VpnAppIdType {
    pub const PackageFamilyName: Self = Self(0i32);
    pub const FullyQualifiedBinaryName: Self = Self(1i32);
    pub const FilePath: Self = Self(2i32);
}
impl windows_core::TypeKind for VpnAppIdType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VpnAppIdType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VpnAppIdType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VpnAppIdType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnAppIdType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VpnAuthenticationMethod(pub i32);
impl VpnAuthenticationMethod {
    pub const Mschapv2: Self = Self(0i32);
    pub const Eap: Self = Self(1i32);
    pub const Certificate: Self = Self(2i32);
    pub const PresharedKey: Self = Self(3i32);
}
impl windows_core::TypeKind for VpnAuthenticationMethod {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VpnAuthenticationMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VpnAuthenticationMethod").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VpnAuthenticationMethod {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnAuthenticationMethod;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VpnChannelActivityEventType(pub i32);
impl VpnChannelActivityEventType {
    pub const Idle: Self = Self(0i32);
    pub const Active: Self = Self(1i32);
}
impl windows_core::TypeKind for VpnChannelActivityEventType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VpnChannelActivityEventType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VpnChannelActivityEventType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VpnChannelActivityEventType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnChannelActivityEventType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VpnChannelRequestCredentialsOptions(pub u32);
impl VpnChannelRequestCredentialsOptions {
    pub const None: Self = Self(0u32);
    pub const Retrying: Self = Self(1u32);
    pub const UseForSingleSignIn: Self = Self(2u32);
}
impl windows_core::TypeKind for VpnChannelRequestCredentialsOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VpnChannelRequestCredentialsOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VpnChannelRequestCredentialsOptions").field(&self.0).finish()
    }
}
impl VpnChannelRequestCredentialsOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VpnChannelRequestCredentialsOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VpnChannelRequestCredentialsOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VpnChannelRequestCredentialsOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for VpnChannelRequestCredentialsOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for VpnChannelRequestCredentialsOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for VpnChannelRequestCredentialsOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnChannelRequestCredentialsOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VpnCredentialType(pub i32);
impl VpnCredentialType {
    pub const UsernamePassword: Self = Self(0i32);
    pub const UsernameOtpPin: Self = Self(1i32);
    pub const UsernamePasswordAndPin: Self = Self(2i32);
    pub const UsernamePasswordChange: Self = Self(3i32);
    pub const SmartCard: Self = Self(4i32);
    pub const ProtectedCertificate: Self = Self(5i32);
    pub const UnProtectedCertificate: Self = Self(6i32);
}
impl windows_core::TypeKind for VpnCredentialType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VpnCredentialType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VpnCredentialType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VpnCredentialType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnCredentialType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VpnDataPathType(pub i32);
impl VpnDataPathType {
    pub const Send: Self = Self(0i32);
    pub const Receive: Self = Self(1i32);
}
impl windows_core::TypeKind for VpnDataPathType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VpnDataPathType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VpnDataPathType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VpnDataPathType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnDataPathType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VpnDomainNameType(pub i32);
impl VpnDomainNameType {
    pub const Suffix: Self = Self(0i32);
    pub const FullyQualified: Self = Self(1i32);
    pub const Reserved: Self = Self(65535i32);
}
impl windows_core::TypeKind for VpnDomainNameType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VpnDomainNameType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VpnDomainNameType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VpnDomainNameType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnDomainNameType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VpnIPProtocol(pub i32);
impl VpnIPProtocol {
    pub const None: Self = Self(0i32);
    pub const Tcp: Self = Self(6i32);
    pub const Udp: Self = Self(17i32);
    pub const Icmp: Self = Self(1i32);
    pub const Ipv6Icmp: Self = Self(58i32);
    pub const Igmp: Self = Self(2i32);
    pub const Pgm: Self = Self(113i32);
}
impl windows_core::TypeKind for VpnIPProtocol {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VpnIPProtocol {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VpnIPProtocol").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VpnIPProtocol {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnIPProtocol;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VpnManagementConnectionStatus(pub i32);
impl VpnManagementConnectionStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Disconnecting: Self = Self(1i32);
    pub const Connected: Self = Self(2i32);
    pub const Connecting: Self = Self(3i32);
}
impl windows_core::TypeKind for VpnManagementConnectionStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VpnManagementConnectionStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VpnManagementConnectionStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VpnManagementConnectionStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnManagementConnectionStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VpnManagementErrorStatus(pub i32);
impl VpnManagementErrorStatus {
    pub const Ok: Self = Self(0i32);
    pub const Other: Self = Self(1i32);
    pub const InvalidXmlSyntax: Self = Self(2i32);
    pub const ProfileNameTooLong: Self = Self(3i32);
    pub const ProfileInvalidAppId: Self = Self(4i32);
    pub const AccessDenied: Self = Self(5i32);
    pub const CannotFindProfile: Self = Self(6i32);
    pub const AlreadyDisconnecting: Self = Self(7i32);
    pub const AlreadyConnected: Self = Self(8i32);
    pub const GeneralAuthenticationFailure: Self = Self(9i32);
    pub const EapFailure: Self = Self(10i32);
    pub const SmartCardFailure: Self = Self(11i32);
    pub const CertificateFailure: Self = Self(12i32);
    pub const ServerConfiguration: Self = Self(13i32);
    pub const NoConnection: Self = Self(14i32);
    pub const ServerConnection: Self = Self(15i32);
    pub const UserNamePassword: Self = Self(16i32);
    pub const DnsNotResolvable: Self = Self(17i32);
    pub const InvalidIP: Self = Self(18i32);
}
impl windows_core::TypeKind for VpnManagementErrorStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VpnManagementErrorStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VpnManagementErrorStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VpnManagementErrorStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnManagementErrorStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VpnNativeProtocolType(pub i32);
impl VpnNativeProtocolType {
    pub const Pptp: Self = Self(0i32);
    pub const L2tp: Self = Self(1i32);
    pub const IpsecIkev2: Self = Self(2i32);
}
impl windows_core::TypeKind for VpnNativeProtocolType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VpnNativeProtocolType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VpnNativeProtocolType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VpnNativeProtocolType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnNativeProtocolType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VpnPacketBufferStatus(pub i32);
impl VpnPacketBufferStatus {
    pub const Ok: Self = Self(0i32);
    pub const InvalidBufferSize: Self = Self(1i32);
}
impl windows_core::TypeKind for VpnPacketBufferStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VpnPacketBufferStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VpnPacketBufferStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VpnPacketBufferStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnPacketBufferStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VpnRoutingPolicyType(pub i32);
impl VpnRoutingPolicyType {
    pub const SplitRouting: Self = Self(0i32);
    pub const ForceAllTrafficOverVpn: Self = Self(1i32);
}
impl windows_core::TypeKind for VpnRoutingPolicyType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VpnRoutingPolicyType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VpnRoutingPolicyType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VpnRoutingPolicyType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnRoutingPolicyType;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
