windows_core::imp::define_interface!(IWCNConnectNotify, IWCNConnectNotify_Vtbl, 0xc100be9f_d33a_4a4b_bf23_bbef4663d017);
impl core::ops::Deref for IWCNConnectNotify {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWCNConnectNotify, windows_core::IUnknown);
impl IWCNConnectNotify {
    pub unsafe fn ConnectSucceeded(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConnectSucceeded)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ConnectFailed(&self, hrfailure: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConnectFailed)(windows_core::Interface::as_raw(self), hrfailure).ok()
    }
}
#[repr(C)]
pub struct IWCNConnectNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectSucceeded: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectFailed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWCNDevice, IWCNDevice_Vtbl, 0xc100be9c_d33a_4a4b_bf23_bbef4663d017);
impl core::ops::Deref for IWCNDevice {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWCNDevice, windows_core::IUnknown);
impl IWCNDevice {
    pub unsafe fn SetPassword(&self, r#type: WCN_PASSWORD_TYPE, pbpassword: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPassword)(windows_core::Interface::as_raw(self), r#type, pbpassword.len().try_into().unwrap(), core::mem::transmute(pbpassword.as_ptr())).ok()
    }
    pub unsafe fn Connect<P0>(&self, pnotify: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWCNConnectNotify>,
    {
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), pnotify.param().abi()).ok()
    }
    pub unsafe fn GetAttribute(&self, attributetype: WCN_ATTRIBUTE_TYPE, pbbuffer: &mut [u8], pdwbufferused: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAttribute)(windows_core::Interface::as_raw(self), attributetype, pbbuffer.len().try_into().unwrap(), core::mem::transmute(pbbuffer.as_ptr()), pdwbufferused).ok()
    }
    pub unsafe fn GetIntegerAttribute(&self, attributetype: WCN_ATTRIBUTE_TYPE) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIntegerAttribute)(windows_core::Interface::as_raw(self), attributetype, &mut result__).map(|| result__)
    }
    pub unsafe fn GetStringAttribute(&self, attributetype: WCN_ATTRIBUTE_TYPE, wszstring: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStringAttribute)(windows_core::Interface::as_raw(self), attributetype, wszstring.len().try_into().unwrap(), core::mem::transmute(wszstring.as_ptr())).ok()
    }
    pub unsafe fn GetNetworkProfile(&self, wszprofile: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNetworkProfile)(windows_core::Interface::as_raw(self), wszprofile.len().try_into().unwrap(), core::mem::transmute(wszprofile.as_ptr())).ok()
    }
    pub unsafe fn SetNetworkProfile<P0>(&self, pszprofilexml: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetNetworkProfile)(windows_core::Interface::as_raw(self), pszprofilexml.param().abi()).ok()
    }
    pub unsafe fn GetVendorExtension(&self, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, pbbuffer: &mut [u8], pdwbufferused: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVendorExtension)(windows_core::Interface::as_raw(self), pvendorextspec, pbbuffer.len().try_into().unwrap(), core::mem::transmute(pbbuffer.as_ptr()), pdwbufferused).ok()
    }
    pub unsafe fn SetVendorExtension(&self, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, pbbuffer: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVendorExtension)(windows_core::Interface::as_raw(self), pvendorextspec, pbbuffer.len().try_into().unwrap(), core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
    pub unsafe fn Unadvise(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetNFCPasswordParams(&self, r#type: WCN_PASSWORD_TYPE, dwoobpasswordid: u32, pbpassword: Option<&[u8]>, pbremotepublickeyhash: Option<&[u8]>, pbdhkeyblob: Option<&[u8]>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNFCPasswordParams)(
            windows_core::Interface::as_raw(self),
            r#type,
            dwoobpasswordid,
            pbpassword.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(pbpassword.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pbremotepublickeyhash.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(pbremotepublickeyhash.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pbdhkeyblob.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(pbdhkeyblob.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        )
        .ok()
    }
}
#[repr(C)]
pub struct IWCNDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, WCN_PASSWORD_TYPE, u32, *const u8) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, WCN_ATTRIBUTE_TYPE, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetIntegerAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, WCN_ATTRIBUTE_TYPE, *mut u32) -> windows_core::HRESULT,
    pub GetStringAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, WCN_ATTRIBUTE_TYPE, u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetNetworkProfile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetNetworkProfile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetVendorExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *const WCN_VENDOR_EXTENSION_SPEC, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SetVendorExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *const WCN_VENDOR_EXTENSION_SPEC, u32, *const u8) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNFCPasswordParams: unsafe extern "system" fn(*mut core::ffi::c_void, WCN_PASSWORD_TYPE, u32, u32, *const u8, u32, *const u8, u32, *const u8) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_DeviceType_Category: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b8b_4684_11da_a26a_0002b3988e81), pid: 16 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_DeviceType_SubCategory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b8b_4684_11da_a26a_0002b3988e81), pid: 18 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_DeviceType_SubCategoryOUI: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b8b_4684_11da_a26a_0002b3988e81), pid: 17 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_SSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x88190b8b_4684_11da_a26a_0002b3988e81), pid: 32 };
pub const SID_WcnProvider: windows_core::GUID = windows_core::GUID::from_u128(0xc100beca_d33a_4a4b_bf23_bbef4663d017);
pub const WCN_API_MAX_BUFFER_SIZE: u32 = 2096u32;
pub const WCN_E_AUTHENTICATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80043C02_u32 as _);
pub const WCN_E_CONNECTION_REJECTED: windows_core::HRESULT = windows_core::HRESULT(0x80043C03_u32 as _);
pub const WCN_E_PEER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80043C01_u32 as _);
pub const WCN_E_PROTOCOL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80043C05_u32 as _);
pub const WCN_E_SESSION_TIMEDOUT: windows_core::HRESULT = windows_core::HRESULT(0x80043C04_u32 as _);
pub const WCN_FLAG_AUTHENTICATED_VE: u32 = 2u32;
pub const WCN_FLAG_DISCOVERY_VE: u32 = 1u32;
pub const WCN_FLAG_ENCRYPTED_VE: u32 = 4u32;
pub const WCN_MICROSOFT_VENDOR_ID: u32 = 311u32;
pub const WCN_NO_SUBTYPE: u32 = 4294967294u32;
pub const WCN_NUM_ATTRIBUTE_TYPES: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(100i32);
pub const WCN_PASSWORD_TYPE_OOB_SPECIFIED: WCN_PASSWORD_TYPE = WCN_PASSWORD_TYPE(3i32);
pub const WCN_PASSWORD_TYPE_PIN: WCN_PASSWORD_TYPE = WCN_PASSWORD_TYPE(1i32);
pub const WCN_PASSWORD_TYPE_PIN_REGISTRAR_SPECIFIED: WCN_PASSWORD_TYPE = WCN_PASSWORD_TYPE(2i32);
pub const WCN_PASSWORD_TYPE_PUSH_BUTTON: WCN_PASSWORD_TYPE = WCN_PASSWORD_TYPE(0i32);
pub const WCN_PASSWORD_TYPE_WFDS: WCN_PASSWORD_TYPE = WCN_PASSWORD_TYPE(4i32);
pub const WCN_QUERY_CONSTRAINT_USE_SOFTAP: windows_core::PCWSTR = windows_core::w!("WCN.Discovery.SoftAP");
pub const WCN_SESSION_STATUS_FAILURE_GENERIC: WCN_SESSION_STATUS = WCN_SESSION_STATUS(1i32);
pub const WCN_SESSION_STATUS_FAILURE_TIMEOUT: WCN_SESSION_STATUS = WCN_SESSION_STATUS(2i32);
pub const WCN_SESSION_STATUS_SUCCESS: WCN_SESSION_STATUS = WCN_SESSION_STATUS(0i32);
pub const WCN_TYPE_802_1X_ENABLED: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(83i32);
pub const WCN_TYPE_APPLICATION_EXTENSION: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(79i32);
pub const WCN_TYPE_APPSESSIONKEY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(84i32);
pub const WCN_TYPE_AP_CHANNEL: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(0i32);
pub const WCN_TYPE_AP_SETUP_LOCKED: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(78i32);
pub const WCN_TYPE_ASSOCIATION_STATE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(1i32);
pub const WCN_TYPE_AUTHENTICATION_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(2i32);
pub const WCN_TYPE_AUTHENTICATION_TYPE_FLAGS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(3i32);
pub const WCN_TYPE_AUTHENTICATOR: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(4i32);
pub const WCN_TYPE_AUTHORIZED_MACS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(93i32);
pub const WCN_TYPE_BSSID: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(91i32);
pub const WCN_TYPE_CONFIGURATION_ERROR: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(6i32);
pub const WCN_TYPE_CONFIG_METHODS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(5i32);
pub const WCN_TYPE_CONFIRMATION_URL4: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(7i32);
pub const WCN_TYPE_CONFIRMATION_URL6: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(8i32);
pub const WCN_TYPE_CONNECTION_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(9i32);
pub const WCN_TYPE_CONNECTION_TYPE_FLAGS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(10i32);
pub const WCN_TYPE_CREDENTIAL: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(11i32);
pub const WCN_TYPE_CURRENT_SSID: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(90i32);
pub const WCN_TYPE_DEVICE_NAME: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(12i32);
pub const WCN_TYPE_DEVICE_PASSWORD_ID: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(13i32);
pub const WCN_TYPE_DOT11_MAC_ADDRESS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(92i32);
pub const WCN_TYPE_EAP_IDENTITY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(68i32);
pub const WCN_TYPE_EAP_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(80i32);
pub const WCN_TYPE_ENCRYPTED_SETTINGS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(18i32);
pub const WCN_TYPE_ENCRYPTION_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(19i32);
pub const WCN_TYPE_ENCRYPTION_TYPE_FLAGS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(20i32);
pub const WCN_TYPE_ENROLLEE_NONCE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(21i32);
pub const WCN_TYPE_E_HASH1: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(14i32);
pub const WCN_TYPE_E_HASH2: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(15i32);
pub const WCN_TYPE_E_SNONCE1: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(16i32);
pub const WCN_TYPE_E_SNONCE2: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(17i32);
pub const WCN_TYPE_FEATURE_ID: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(22i32);
pub const WCN_TYPE_IDENTITY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(23i32);
pub const WCN_TYPE_IDENTITY_PROOF: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(24i32);
pub const WCN_TYPE_INITIALIZATION_VECTOR: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(81i32);
pub const WCN_TYPE_KEY_IDENTIFIER: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(26i32);
pub const WCN_TYPE_KEY_LIFETIME: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(72i32);
pub const WCN_TYPE_KEY_PROVIDED_AUTOMATICALLY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(82i32);
pub const WCN_TYPE_KEY_WRAP_AUTHENTICATOR: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(25i32);
pub const WCN_TYPE_MAC_ADDRESS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(27i32);
pub const WCN_TYPE_MANUFACTURER: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(28i32);
pub const WCN_TYPE_MESSAGE_COUNTER: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(69i32);
pub const WCN_TYPE_MESSAGE_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(29i32);
pub const WCN_TYPE_MODEL_NAME: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(30i32);
pub const WCN_TYPE_MODEL_NUMBER: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(31i32);
pub const WCN_TYPE_NETWORK_INDEX: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(32i32);
pub const WCN_TYPE_NETWORK_KEY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(33i32);
pub const WCN_TYPE_NETWORK_KEY_INDEX: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(34i32);
pub const WCN_TYPE_NETWORK_KEY_SHAREABLE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(94i32);
pub const WCN_TYPE_NEW_DEVICE_NAME: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(35i32);
pub const WCN_TYPE_NEW_PASSWORD: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(36i32);
pub const WCN_TYPE_OOB_DEVICE_PASSWORD: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(37i32);
pub const WCN_TYPE_OS_VERSION: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(38i32);
pub const WCN_TYPE_PERMITTED_CONFIG_METHODS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(73i32);
pub const WCN_TYPE_PORTABLE_DEVICE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(77i32);
pub const WCN_TYPE_POWER_LEVEL: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(39i32);
pub const WCN_TYPE_PRIMARY_DEVICE_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(75i32);
pub const WCN_TYPE_PRIMARY_DEVICE_TYPE_CATEGORY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(87i32);
pub const WCN_TYPE_PRIMARY_DEVICE_TYPE_SUBCATEGORY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(89i32);
pub const WCN_TYPE_PRIMARY_DEVICE_TYPE_SUBCATEGORY_OUI: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(88i32);
pub const WCN_TYPE_PSK_CURRENT: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(40i32);
pub const WCN_TYPE_PSK_MAX: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(41i32);
pub const WCN_TYPE_PUBLIC_KEY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(42i32);
pub const WCN_TYPE_PUBLIC_KEY_HASH: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(70i32);
pub const WCN_TYPE_RADIO_ENABLED: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(43i32);
pub const WCN_TYPE_REBOOT: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(44i32);
pub const WCN_TYPE_REGISTRAR_CURRENT: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(45i32);
pub const WCN_TYPE_REGISTRAR_ESTABLISHED: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(46i32);
pub const WCN_TYPE_REGISTRAR_LIST: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(47i32);
pub const WCN_TYPE_REGISTRAR_MAX: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(48i32);
pub const WCN_TYPE_REGISTRAR_NONCE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(49i32);
pub const WCN_TYPE_REKEY_KEY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(71i32);
pub const WCN_TYPE_REQUESTED_DEVICE_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(96i32);
pub const WCN_TYPE_REQUEST_TO_ENROLL: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(95i32);
pub const WCN_TYPE_REQUEST_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(50i32);
pub const WCN_TYPE_RESPONSE_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(51i32);
pub const WCN_TYPE_RF_BANDS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(52i32);
pub const WCN_TYPE_R_HASH1: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(53i32);
pub const WCN_TYPE_R_HASH2: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(54i32);
pub const WCN_TYPE_R_SNONCE1: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(55i32);
pub const WCN_TYPE_R_SNONCE2: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(56i32);
pub const WCN_TYPE_SECONDARY_DEVICE_TYPE_LIST: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(76i32);
pub const WCN_TYPE_SELECTED_REGISTRAR: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(57i32);
pub const WCN_TYPE_SELECTED_REGISTRAR_CONFIG_METHODS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(74i32);
pub const WCN_TYPE_SERIAL_NUMBER: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(58i32);
pub const WCN_TYPE_SETTINGS_DELAY_TIME: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(97i32);
pub const WCN_TYPE_SSID: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(60i32);
pub const WCN_TYPE_TOTAL_NETWORKS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(61i32);
pub const WCN_TYPE_UUID: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(86i32);
pub const WCN_TYPE_UUID_E: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(62i32);
pub const WCN_TYPE_UUID_R: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(63i32);
pub const WCN_TYPE_VENDOR_EXTENSION: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(64i32);
pub const WCN_TYPE_VENDOR_EXTENSION_WFA: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(99i32);
pub const WCN_TYPE_VERSION: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(65i32);
pub const WCN_TYPE_VERSION2: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(98i32);
pub const WCN_TYPE_WEPTRANSMITKEY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(85i32);
pub const WCN_TYPE_WI_FI_PROTECTED_SETUP_STATE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(59i32);
pub const WCN_TYPE_X_509_CERTIFICATE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(67i32);
pub const WCN_TYPE_X_509_CERTIFICATE_REQUEST: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(66i32);
pub const WCN_VALUE_AS_ASSOCIATION_FAILURE: WCN_VALUE_TYPE_ASSOCIATION_STATE = WCN_VALUE_TYPE_ASSOCIATION_STATE(3i32);
pub const WCN_VALUE_AS_CONFIGURATION_FAILURE: WCN_VALUE_TYPE_ASSOCIATION_STATE = WCN_VALUE_TYPE_ASSOCIATION_STATE(2i32);
pub const WCN_VALUE_AS_CONNECTION_SUCCESS: WCN_VALUE_TYPE_ASSOCIATION_STATE = WCN_VALUE_TYPE_ASSOCIATION_STATE(1i32);
pub const WCN_VALUE_AS_IP_FAILURE: WCN_VALUE_TYPE_ASSOCIATION_STATE = WCN_VALUE_TYPE_ASSOCIATION_STATE(4i32);
pub const WCN_VALUE_AS_NOT_ASSOCIATED: WCN_VALUE_TYPE_ASSOCIATION_STATE = WCN_VALUE_TYPE_ASSOCIATION_STATE(0i32);
pub const WCN_VALUE_AT_OPEN: WCN_VALUE_TYPE_AUTHENTICATION_TYPE = WCN_VALUE_TYPE_AUTHENTICATION_TYPE(1i32);
pub const WCN_VALUE_AT_SHARED: WCN_VALUE_TYPE_AUTHENTICATION_TYPE = WCN_VALUE_TYPE_AUTHENTICATION_TYPE(4i32);
pub const WCN_VALUE_AT_WPA: WCN_VALUE_TYPE_AUTHENTICATION_TYPE = WCN_VALUE_TYPE_AUTHENTICATION_TYPE(8i32);
pub const WCN_VALUE_AT_WPA2: WCN_VALUE_TYPE_AUTHENTICATION_TYPE = WCN_VALUE_TYPE_AUTHENTICATION_TYPE(16i32);
pub const WCN_VALUE_AT_WPA2PSK: WCN_VALUE_TYPE_AUTHENTICATION_TYPE = WCN_VALUE_TYPE_AUTHENTICATION_TYPE(32i32);
pub const WCN_VALUE_AT_WPAPSK: WCN_VALUE_TYPE_AUTHENTICATION_TYPE = WCN_VALUE_TYPE_AUTHENTICATION_TYPE(2i32);
pub const WCN_VALUE_AT_WPAWPA2PSK_MIXED: WCN_VALUE_TYPE_AUTHENTICATION_TYPE = WCN_VALUE_TYPE_AUTHENTICATION_TYPE(34i32);
pub const WCN_VALUE_CE_2_4_CHANNEL_NOT_SUPPORTED: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(3i32);
pub const WCN_VALUE_CE_5_0_CHANNEL_NOT_SUPPORTED: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(4i32);
pub const WCN_VALUE_CE_COULD_NOT_CONNECT_TO_REGISTRAR: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(11i32);
pub const WCN_VALUE_CE_DECRYPTION_CRC_FAILURE: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(2i32);
pub const WCN_VALUE_CE_DEVICE_BUSY: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(14i32);
pub const WCN_VALUE_CE_DEVICE_PASSWORD_AUTH_FAILURE: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(18i32);
pub const WCN_VALUE_CE_FAILED_DHCP_CONFIG: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(9i32);
pub const WCN_VALUE_CE_IP_ADDRESS_CONFLICT: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(10i32);
pub const WCN_VALUE_CE_MESSAGE_TIMEOUT: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(16i32);
pub const WCN_VALUE_CE_MULTIPLE_PBC_SESSIONS_DETECTED: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(12i32);
pub const WCN_VALUE_CE_NETWORK_ASSOCIATION_FAILURE: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(7i32);
pub const WCN_VALUE_CE_NETWORK_AUTHENTICATION_FAILURE: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(6i32);
pub const WCN_VALUE_CE_NO_DHCP_RESPONSE: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(8i32);
pub const WCN_VALUE_CE_NO_ERROR: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(0i32);
pub const WCN_VALUE_CE_OOB_INTERFACE_READ_ERROR: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(1i32);
pub const WCN_VALUE_CE_REGISTRATION_SESSION_TIMEOUT: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(17i32);
pub const WCN_VALUE_CE_ROGUE_ACTIVITY_SUSPECTED: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(13i32);
pub const WCN_VALUE_CE_SETUP_LOCKED: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(15i32);
pub const WCN_VALUE_CE_SIGNAL_TOO_WEAK: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(5i32);
pub const WCN_VALUE_CM_DISPLAY: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(8i32);
pub const WCN_VALUE_CM_ETHERNET: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(2i32);
pub const WCN_VALUE_CM_EXTERNAL_NFC: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(16i32);
pub const WCN_VALUE_CM_INTEGRATED_NFC: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(32i32);
pub const WCN_VALUE_CM_KEYPAD: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(256i32);
pub const WCN_VALUE_CM_LABEL: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(4i32);
pub const WCN_VALUE_CM_NFC_INTERFACE: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(64i32);
pub const WCN_VALUE_CM_PHYS_DISPLAY: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(16392i32);
pub const WCN_VALUE_CM_PHYS_PUSHBUTTON: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(1152i32);
pub const WCN_VALUE_CM_PUSHBUTTON: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(128i32);
pub const WCN_VALUE_CM_USBA: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(1i32);
pub const WCN_VALUE_CM_VIRT_DISPLAY: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(8200i32);
pub const WCN_VALUE_CM_VIRT_PUSHBUTTON: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(640i32);
pub const WCN_VALUE_CT_ESS: WCN_VALUE_TYPE_CONNECTION_TYPE = WCN_VALUE_TYPE_CONNECTION_TYPE(1i32);
pub const WCN_VALUE_CT_IBSS: WCN_VALUE_TYPE_CONNECTION_TYPE = WCN_VALUE_TYPE_CONNECTION_TYPE(2i32);
pub const WCN_VALUE_DP_DEFAULT: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(0i32);
pub const WCN_VALUE_DP_MACHINE_SPECIFIED: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(2i32);
pub const WCN_VALUE_DP_NFC_CONNECTION_HANDOVER: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(7i32);
pub const WCN_VALUE_DP_OUTOFBAND_MAX: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(65535i32);
pub const WCN_VALUE_DP_OUTOFBAND_MIN: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(16i32);
pub const WCN_VALUE_DP_PUSHBUTTON: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(4i32);
pub const WCN_VALUE_DP_REGISTRAR_SPECIFIED: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(5i32);
pub const WCN_VALUE_DP_REKEY: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(3i32);
pub const WCN_VALUE_DP_USER_SPECIFIED: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(1i32);
pub const WCN_VALUE_DP_WFD_SERVICES: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(8i32);
pub const WCN_VALUE_DT_CATEGORY_AUDIO_DEVICE: u32 = 11u32;
pub const WCN_VALUE_DT_CATEGORY_CAMERA: u32 = 4u32;
pub const WCN_VALUE_DT_CATEGORY_COMPUTER: u32 = 1u32;
pub const WCN_VALUE_DT_CATEGORY_DISPLAY: u32 = 7u32;
pub const WCN_VALUE_DT_CATEGORY_GAMING_DEVICE: u32 = 9u32;
pub const WCN_VALUE_DT_CATEGORY_INPUT_DEVICE: u32 = 2u32;
pub const WCN_VALUE_DT_CATEGORY_MULTIMEDIA_DEVICE: u32 = 8u32;
pub const WCN_VALUE_DT_CATEGORY_NETWORK_INFRASTRUCTURE: u32 = 6u32;
pub const WCN_VALUE_DT_CATEGORY_OTHER: u32 = 255u32;
pub const WCN_VALUE_DT_CATEGORY_PRINTER: u32 = 3u32;
pub const WCN_VALUE_DT_CATEGORY_STORAGE: u32 = 5u32;
pub const WCN_VALUE_DT_CATEGORY_TELEPHONE: u32 = 10u32;
pub const WCN_VALUE_DT_SUBTYPE_AUDIO_DEVICE__HEADPHONES: u32 = 5u32;
pub const WCN_VALUE_DT_SUBTYPE_AUDIO_DEVICE__HEADSET: u32 = 4u32;
pub const WCN_VALUE_DT_SUBTYPE_AUDIO_DEVICE__HOMETHEATER: u32 = 7u32;
pub const WCN_VALUE_DT_SUBTYPE_AUDIO_DEVICE__MICROPHONE: u32 = 6u32;
pub const WCN_VALUE_DT_SUBTYPE_AUDIO_DEVICE__PMP: u32 = 3u32;
pub const WCN_VALUE_DT_SUBTYPE_AUDIO_DEVICE__SPEAKERS: u32 = 2u32;
pub const WCN_VALUE_DT_SUBTYPE_AUDIO_DEVICE__TUNER_RECEIVER: u32 = 1u32;
pub const WCN_VALUE_DT_SUBTYPE_CAMERA__SECURITY_CAMERA: u32 = 4u32;
pub const WCN_VALUE_DT_SUBTYPE_CAMERA__STILL_CAMERA: u32 = 1u32;
pub const WCN_VALUE_DT_SUBTYPE_CAMERA__VIDEO_CAMERA: u32 = 2u32;
pub const WCN_VALUE_DT_SUBTYPE_CAMERA__WEB_CAMERA: u32 = 3u32;
pub const WCN_VALUE_DT_SUBTYPE_COMPUTER__DESKTOP: u32 = 6u32;
pub const WCN_VALUE_DT_SUBTYPE_COMPUTER__MEDIACENTER: u32 = 3u32;
pub const WCN_VALUE_DT_SUBTYPE_COMPUTER__MID: u32 = 7u32;
pub const WCN_VALUE_DT_SUBTYPE_COMPUTER__NETBOOK: u32 = 8u32;
pub const WCN_VALUE_DT_SUBTYPE_COMPUTER__NOTEBOOK: u32 = 5u32;
pub const WCN_VALUE_DT_SUBTYPE_COMPUTER__PC: u32 = 1u32;
pub const WCN_VALUE_DT_SUBTYPE_COMPUTER__SERVER: u32 = 2u32;
pub const WCN_VALUE_DT_SUBTYPE_COMPUTER__ULTRAMOBILEPC: u32 = 4u32;
pub const WCN_VALUE_DT_SUBTYPE_DISPLAY__MONITOR: u32 = 4u32;
pub const WCN_VALUE_DT_SUBTYPE_DISPLAY__PICTURE_FRAME: u32 = 2u32;
pub const WCN_VALUE_DT_SUBTYPE_DISPLAY__PROJECTOR: u32 = 3u32;
pub const WCN_VALUE_DT_SUBTYPE_DISPLAY__TELEVISION: u32 = 1u32;
pub const WCN_VALUE_DT_SUBTYPE_GAMING_DEVICE__CONSOLE_ADAPT: u32 = 4u32;
pub const WCN_VALUE_DT_SUBTYPE_GAMING_DEVICE__PLAYSTATION: u32 = 3u32;
pub const WCN_VALUE_DT_SUBTYPE_GAMING_DEVICE__PORTABLE: u32 = 5u32;
pub const WCN_VALUE_DT_SUBTYPE_GAMING_DEVICE__XBOX: u32 = 1u32;
pub const WCN_VALUE_DT_SUBTYPE_GAMING_DEVICE__XBOX360: u32 = 2u32;
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__BARCODEREADER: u32 = 9u32;
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__BIOMETRICREADER: u32 = 8u32;
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__GAMECONTROLLER: u32 = 5u32;
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__JOYSTICK: u32 = 3u32;
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__KEYBOARD: u32 = 1u32;
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__MOUSE: u32 = 2u32;
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__REMOTE: u32 = 6u32;
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__TOUCHSCREEN: u32 = 7u32;
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__TRACKBALL: u32 = 4u32;
pub const WCN_VALUE_DT_SUBTYPE_MULTIMEDIA_DEVICE__DAR: u32 = 1u32;
pub const WCN_VALUE_DT_SUBTYPE_MULTIMEDIA_DEVICE__MCX: u32 = 3u32;
pub const WCN_VALUE_DT_SUBTYPE_MULTIMEDIA_DEVICE__MEDIA_SERVER_ADAPT_EXT: u32 = 5u32;
pub const WCN_VALUE_DT_SUBTYPE_MULTIMEDIA_DEVICE__PVP: u32 = 6u32;
pub const WCN_VALUE_DT_SUBTYPE_MULTIMEDIA_DEVICE__PVR: u32 = 2u32;
pub const WCN_VALUE_DT_SUBTYPE_MULTIMEDIA_DEVICE__SETTOPBOX: u32 = 4u32;
pub const WCN_VALUE_DT_SUBTYPE_NETWORK_INFRASTRUCUTURE__AP: u32 = 1u32;
pub const WCN_VALUE_DT_SUBTYPE_NETWORK_INFRASTRUCUTURE__BRIDGE: u32 = 5u32;
pub const WCN_VALUE_DT_SUBTYPE_NETWORK_INFRASTRUCUTURE__GATEWAY: u32 = 4u32;
pub const WCN_VALUE_DT_SUBTYPE_NETWORK_INFRASTRUCUTURE__ROUTER: u32 = 2u32;
pub const WCN_VALUE_DT_SUBTYPE_NETWORK_INFRASTRUCUTURE__SWITCH: u32 = 3u32;
pub const WCN_VALUE_DT_SUBTYPE_PRINTER__ALLINONE: u32 = 5u32;
pub const WCN_VALUE_DT_SUBTYPE_PRINTER__COPIER: u32 = 4u32;
pub const WCN_VALUE_DT_SUBTYPE_PRINTER__FAX: u32 = 3u32;
pub const WCN_VALUE_DT_SUBTYPE_PRINTER__PRINTER: u32 = 1u32;
pub const WCN_VALUE_DT_SUBTYPE_PRINTER__SCANNER: u32 = 2u32;
pub const WCN_VALUE_DT_SUBTYPE_STORAGE__NAS: u32 = 1u32;
pub const WCN_VALUE_DT_SUBTYPE_TELEPHONE__PHONE_DUALMODE: u32 = 3u32;
pub const WCN_VALUE_DT_SUBTYPE_TELEPHONE__PHONE_SINGLEMODE: u32 = 2u32;
pub const WCN_VALUE_DT_SUBTYPE_TELEPHONE__SMARTPHONE_DUALMODE: u32 = 5u32;
pub const WCN_VALUE_DT_SUBTYPE_TELEPHONE__SMARTPHONE_SINGLEMODE: u32 = 4u32;
pub const WCN_VALUE_DT_SUBTYPE_TELEPHONE__WINDOWS_MOBILE: u32 = 1u32;
pub const WCN_VALUE_DT_SUBTYPE_WIFI_OUI: u32 = 5304836u32;
pub const WCN_VALUE_ET_AES: WCN_VALUE_TYPE_ENCRYPTION_TYPE = WCN_VALUE_TYPE_ENCRYPTION_TYPE(8i32);
pub const WCN_VALUE_ET_NONE: WCN_VALUE_TYPE_ENCRYPTION_TYPE = WCN_VALUE_TYPE_ENCRYPTION_TYPE(1i32);
pub const WCN_VALUE_ET_TKIP: WCN_VALUE_TYPE_ENCRYPTION_TYPE = WCN_VALUE_TYPE_ENCRYPTION_TYPE(4i32);
pub const WCN_VALUE_ET_TKIP_AES_MIXED: WCN_VALUE_TYPE_ENCRYPTION_TYPE = WCN_VALUE_TYPE_ENCRYPTION_TYPE(12i32);
pub const WCN_VALUE_ET_WEP: WCN_VALUE_TYPE_ENCRYPTION_TYPE = WCN_VALUE_TYPE_ENCRYPTION_TYPE(2i32);
pub const WCN_VALUE_FALSE: WCN_VALUE_TYPE_BOOLEAN = WCN_VALUE_TYPE_BOOLEAN(0i32);
pub const WCN_VALUE_MT_ACK: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(13i32);
pub const WCN_VALUE_MT_BEACON: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(1i32);
pub const WCN_VALUE_MT_DONE: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(15i32);
pub const WCN_VALUE_MT_M1: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(4i32);
pub const WCN_VALUE_MT_M2: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(5i32);
pub const WCN_VALUE_MT_M2D: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(6i32);
pub const WCN_VALUE_MT_M3: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(7i32);
pub const WCN_VALUE_MT_M4: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(8i32);
pub const WCN_VALUE_MT_M5: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(9i32);
pub const WCN_VALUE_MT_M6: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(10i32);
pub const WCN_VALUE_MT_M7: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(11i32);
pub const WCN_VALUE_MT_M8: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(12i32);
pub const WCN_VALUE_MT_NACK: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(14i32);
pub const WCN_VALUE_MT_PROBE_REQUEST: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(2i32);
pub const WCN_VALUE_MT_PROBE_RESPONSE: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(3i32);
pub const WCN_VALUE_RB_24GHZ: WCN_VALUE_TYPE_RF_BANDS = WCN_VALUE_TYPE_RF_BANDS(1i32);
pub const WCN_VALUE_RB_50GHZ: WCN_VALUE_TYPE_RF_BANDS = WCN_VALUE_TYPE_RF_BANDS(2i32);
pub const WCN_VALUE_ReqT_ENROLLEE_INFO: WCN_VALUE_TYPE_REQUEST_TYPE = WCN_VALUE_TYPE_REQUEST_TYPE(0i32);
pub const WCN_VALUE_ReqT_ENROLLEE_OPEN_1X: WCN_VALUE_TYPE_REQUEST_TYPE = WCN_VALUE_TYPE_REQUEST_TYPE(1i32);
pub const WCN_VALUE_ReqT_MANAGER_REGISTRAR: WCN_VALUE_TYPE_REQUEST_TYPE = WCN_VALUE_TYPE_REQUEST_TYPE(3i32);
pub const WCN_VALUE_ReqT_REGISTRAR: WCN_VALUE_TYPE_REQUEST_TYPE = WCN_VALUE_TYPE_REQUEST_TYPE(2i32);
pub const WCN_VALUE_RspT_AP: WCN_VALUE_TYPE_RESPONSE_TYPE = WCN_VALUE_TYPE_RESPONSE_TYPE(3i32);
pub const WCN_VALUE_RspT_ENROLLEE_INFO: WCN_VALUE_TYPE_RESPONSE_TYPE = WCN_VALUE_TYPE_RESPONSE_TYPE(0i32);
pub const WCN_VALUE_RspT_ENROLLEE_OPEN_1X: WCN_VALUE_TYPE_RESPONSE_TYPE = WCN_VALUE_TYPE_RESPONSE_TYPE(1i32);
pub const WCN_VALUE_RspT_REGISTRAR: WCN_VALUE_TYPE_RESPONSE_TYPE = WCN_VALUE_TYPE_RESPONSE_TYPE(2i32);
pub const WCN_VALUE_SS_CONFIGURED: WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE = WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE(2i32);
pub const WCN_VALUE_SS_NOT_CONFIGURED: WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE = WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE(1i32);
pub const WCN_VALUE_SS_RESERVED00: WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE = WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE(0i32);
pub const WCN_VALUE_TRUE: WCN_VALUE_TYPE_BOOLEAN = WCN_VALUE_TYPE_BOOLEAN(1i32);
pub const WCN_VALUE_VERSION_1_0: WCN_VALUE_TYPE_VERSION = WCN_VALUE_TYPE_VERSION(16i32);
pub const WCN_VALUE_VERSION_2_0: WCN_VALUE_TYPE_VERSION = WCN_VALUE_TYPE_VERSION(32i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_ATTRIBUTE_TYPE(pub i32);
impl windows_core::TypeKind for WCN_ATTRIBUTE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_ATTRIBUTE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_PASSWORD_TYPE(pub i32);
impl windows_core::TypeKind for WCN_PASSWORD_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_PASSWORD_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_PASSWORD_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_SESSION_STATUS(pub i32);
impl windows_core::TypeKind for WCN_SESSION_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_SESSION_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_SESSION_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_VALUE_TYPE_ASSOCIATION_STATE(pub i32);
impl windows_core::TypeKind for WCN_VALUE_TYPE_ASSOCIATION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_VALUE_TYPE_ASSOCIATION_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_ASSOCIATION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_VALUE_TYPE_AUTHENTICATION_TYPE(pub i32);
impl windows_core::TypeKind for WCN_VALUE_TYPE_AUTHENTICATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_VALUE_TYPE_AUTHENTICATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_AUTHENTICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_VALUE_TYPE_BOOLEAN(pub i32);
impl windows_core::TypeKind for WCN_VALUE_TYPE_BOOLEAN {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_VALUE_TYPE_BOOLEAN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_BOOLEAN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_VALUE_TYPE_CONFIGURATION_ERROR(pub i32);
impl windows_core::TypeKind for WCN_VALUE_TYPE_CONFIGURATION_ERROR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_VALUE_TYPE_CONFIGURATION_ERROR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_CONFIGURATION_ERROR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_VALUE_TYPE_CONFIG_METHODS(pub i32);
impl windows_core::TypeKind for WCN_VALUE_TYPE_CONFIG_METHODS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_VALUE_TYPE_CONFIG_METHODS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_CONFIG_METHODS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_VALUE_TYPE_CONNECTION_TYPE(pub i32);
impl windows_core::TypeKind for WCN_VALUE_TYPE_CONNECTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_VALUE_TYPE_CONNECTION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_CONNECTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(pub i32);
impl windows_core::TypeKind for WCN_VALUE_TYPE_DEVICE_PASSWORD_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_VALUE_TYPE_DEVICE_PASSWORD_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_DEVICE_PASSWORD_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_VALUE_TYPE_ENCRYPTION_TYPE(pub i32);
impl windows_core::TypeKind for WCN_VALUE_TYPE_ENCRYPTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_VALUE_TYPE_ENCRYPTION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_ENCRYPTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_VALUE_TYPE_MESSAGE_TYPE(pub i32);
impl windows_core::TypeKind for WCN_VALUE_TYPE_MESSAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_VALUE_TYPE_MESSAGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_MESSAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_VALUE_TYPE_REQUEST_TYPE(pub i32);
impl windows_core::TypeKind for WCN_VALUE_TYPE_REQUEST_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_VALUE_TYPE_REQUEST_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_REQUEST_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_VALUE_TYPE_RESPONSE_TYPE(pub i32);
impl windows_core::TypeKind for WCN_VALUE_TYPE_RESPONSE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_VALUE_TYPE_RESPONSE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_RESPONSE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_VALUE_TYPE_RF_BANDS(pub i32);
impl windows_core::TypeKind for WCN_VALUE_TYPE_RF_BANDS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_VALUE_TYPE_RF_BANDS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_RF_BANDS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_VALUE_TYPE_VERSION(pub i32);
impl windows_core::TypeKind for WCN_VALUE_TYPE_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_VALUE_TYPE_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE(pub i32);
impl windows_core::TypeKind for WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE").field(&self.0).finish()
    }
}
pub const WCNDeviceObject: windows_core::GUID = windows_core::GUID::from_u128(0xc100bea7_d33a_4a4b_bf23_bbef4663d017);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WCN_VALUE_TYPE_PRIMARY_DEVICE_TYPE {
    pub Category: u16,
    pub SubCategoryOUI: u32,
    pub SubCategory: u16,
}
impl windows_core::TypeKind for WCN_VALUE_TYPE_PRIMARY_DEVICE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl Default for WCN_VALUE_TYPE_PRIMARY_DEVICE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WCN_VENDOR_EXTENSION_SPEC {
    pub VendorId: u32,
    pub SubType: u32,
    pub Index: u32,
    pub Flags: u32,
}
impl windows_core::TypeKind for WCN_VENDOR_EXTENSION_SPEC {
    type TypeKind = windows_core::CopyType;
}
impl Default for WCN_VENDOR_EXTENSION_SPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
