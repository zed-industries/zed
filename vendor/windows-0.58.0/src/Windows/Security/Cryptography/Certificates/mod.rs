windows_core::imp::define_interface!(ICertificate, ICertificate_Vtbl, 0x333f740c_04d8_43b3_b278_8c5fcc9be5a0);
impl windows_core::RuntimeType for ICertificate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub BuildChainAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BuildChainAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub BuildChainWithParametersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BuildChainWithParametersAsync: usize,
    pub SerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetHashValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetHashValueWithAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetCertificateBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetCertificateBlob: usize,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Issuer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HasPrivateKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsStronglyProtected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ValidFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub ValidTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EnhancedKeyUsages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnhancedKeyUsages: usize,
    pub SetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificate2, ICertificate2_Vtbl, 0x17b8374c_8a25_4d96_a492_8fc29ac4fda6);
impl windows_core::RuntimeType for ICertificate2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificate2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSecurityDeviceBound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub KeyUsages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KeyAlgorithmName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SignatureAlgorithmName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SignatureHashAlgorithmName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SubjectAlternativeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificate3, ICertificate3_Vtbl, 0xbe51a966_ae5f_4652_ace7_c6d7e7724cf3);
impl windows_core::RuntimeType for ICertificate3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificate3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPerUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub StoreName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub KeyStorageProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificateChain, ICertificateChain_Vtbl, 0x20bf5385_3691_4501_a62c_fd97278b31ee);
impl windows_core::RuntimeType for ICertificateChain {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateChain_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Validate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ChainValidationResult) -> windows_core::HRESULT,
    pub ValidateWithParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut ChainValidationResult) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCertificates: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCertificates: usize,
}
windows_core::imp::define_interface!(ICertificateEnrollmentManagerStatics, ICertificateEnrollmentManagerStatics_Vtbl, 0x8846ef3f_a986_48fb_9fd7_9aec06935bf1);
impl windows_core::RuntimeType for ICertificateEnrollmentManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateEnrollmentManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateRequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallCertificateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, InstallOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImportPfxDataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, ExportOption, KeyProtectionLevel, InstallOptions, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificateEnrollmentManagerStatics2, ICertificateEnrollmentManagerStatics2_Vtbl, 0xdc5b1c33_6429_4014_999c_5d9735802d1d);
impl windows_core::RuntimeType for ICertificateEnrollmentManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateEnrollmentManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UserCertificateEnrollmentManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImportPfxDataToKspAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, ExportOption, KeyProtectionLevel, InstallOptions, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificateEnrollmentManagerStatics3, ICertificateEnrollmentManagerStatics3_Vtbl, 0xfdec82be_617c_425a_b72d_398b26ac7264);
impl windows_core::RuntimeType for ICertificateEnrollmentManagerStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateEnrollmentManagerStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ImportPfxDataToKspWithParametersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificateExtension, ICertificateExtension_Vtbl, 0x84cf0656_a9e6_454d_8e45_2ea7c4bcd53b);
impl windows_core::RuntimeType for ICertificateExtension {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateExtension_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsCritical: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCritical: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub EncodeValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificateFactory, ICertificateFactory_Vtbl, 0x17b4221c_4baf_44a2_9608_04fb62b16942);
impl windows_core::RuntimeType for ICertificateFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCertificate: usize,
}
windows_core::imp::define_interface!(ICertificateKeyUsages, ICertificateKeyUsages_Vtbl, 0x6ac6206f_e1cf_486a_b485_a69c83e46fd1);
impl windows_core::RuntimeType for ICertificateKeyUsages {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateKeyUsages_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EncipherOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetEncipherOnly: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CrlSign: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCrlSign: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub KeyCertificateSign: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetKeyCertificateSign: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub KeyAgreement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetKeyAgreement: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub DataEncipherment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDataEncipherment: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub KeyEncipherment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetKeyEncipherment: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub NonRepudiation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetNonRepudiation: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub DigitalSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDigitalSignature: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificateQuery, ICertificateQuery_Vtbl, 0x5b082a31_a728_4916_b5ee_ffcb8acf2417);
impl windows_core::RuntimeType for ICertificateQuery {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateQuery_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EnhancedKeyUsages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnhancedKeyUsages: usize,
    pub IssuerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetIssuerName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Thumbprint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub SetThumbprint: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub HardwareOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHardwareOnly: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificateQuery2, ICertificateQuery2_Vtbl, 0x935a0af7_0bd9_4f75_b8c2_e27a7f74eecd);
impl windows_core::RuntimeType for ICertificateQuery2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateQuery2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IncludeDuplicates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIncludeDuplicates: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IncludeExpiredCertificates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIncludeExpiredCertificates: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub StoreName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetStoreName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificateRequestProperties, ICertificateRequestProperties_Vtbl, 0x487e84f6_94e2_4dce_8833_1a700a37a29a);
impl windows_core::RuntimeType for ICertificateRequestProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateRequestProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub KeyAlgorithmName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetKeyAlgorithmName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub KeySize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetKeySize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HashAlgorithmName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetHashAlgorithmName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Exportable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ExportOption) -> windows_core::HRESULT,
    pub SetExportable: unsafe extern "system" fn(*mut core::ffi::c_void, ExportOption) -> windows_core::HRESULT,
    pub KeyUsages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EnrollKeyUsages) -> windows_core::HRESULT,
    pub SetKeyUsages: unsafe extern "system" fn(*mut core::ffi::c_void, EnrollKeyUsages) -> windows_core::HRESULT,
    pub KeyProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut KeyProtectionLevel) -> windows_core::HRESULT,
    pub SetKeyProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, KeyProtectionLevel) -> windows_core::HRESULT,
    pub KeyStorageProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetKeyStorageProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificateRequestProperties2, ICertificateRequestProperties2_Vtbl, 0x3da0c954_d73f_4ff3_a0a6_0677c0ada05b);
impl windows_core::RuntimeType for ICertificateRequestProperties2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateRequestProperties2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SmartcardReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSmartcardReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SigningCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSigningCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AttestationCredentialCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAttestationCredentialCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificateRequestProperties3, ICertificateRequestProperties3_Vtbl, 0xe687f616_734d_46b1_9d4c_6edfdbfc845b);
impl windows_core::RuntimeType for ICertificateRequestProperties3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateRequestProperties3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CurveName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCurveName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CurveParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub SetCurveParameters: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub ContainerNamePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetContainerNamePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ContainerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetContainerName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub UseExistingKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetUseExistingKey: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificateRequestProperties4, ICertificateRequestProperties4_Vtbl, 0x4e429ad2_1c61_4fea_b8fe_135fb19cdce4);
impl windows_core::RuntimeType for ICertificateRequestProperties4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateRequestProperties4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SuppressedDefaults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SuppressedDefaults: usize,
    pub SubjectAlternativeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Extensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Extensions: usize,
}
windows_core::imp::define_interface!(ICertificateStore, ICertificateStore_Vtbl, 0xb0bff720_344e_4331_af14_a7f7a7ebc93a);
impl windows_core::RuntimeType for ICertificateStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateStore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificateStore2, ICertificateStore2_Vtbl, 0xc7e68e4a_417d_4d1a_babd_15687e549974);
impl windows_core::RuntimeType for ICertificateStore2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateStore2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificateStoresStatics, ICertificateStoresStatics_Vtbl, 0xfbecc739_c6fe_4de7_99cf_74c3e596e032);
impl windows_core::RuntimeType for ICertificateStoresStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateStoresStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllWithQueryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllWithQueryAsync: usize,
    pub TrustedRootCertificationAuthorities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IntermediateCertificationAuthorities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStoreByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificateStoresStatics2, ICertificateStoresStatics2_Vtbl, 0xfa900b79_a0d4_4b8c_bc55_c0a37eb141ed);
impl windows_core::RuntimeType for ICertificateStoresStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICertificateStoresStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetUserStoreByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChainBuildingParameters, IChainBuildingParameters_Vtbl, 0x422ba922_7c8d_47b7_b59b_b12703733ac3);
impl windows_core::RuntimeType for IChainBuildingParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChainBuildingParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EnhancedKeyUsages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnhancedKeyUsages: usize,
    pub ValidationTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub SetValidationTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub RevocationCheckEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRevocationCheckEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub NetworkRetrievalEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetNetworkRetrievalEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AuthorityInformationAccessEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAuthorityInformationAccessEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CurrentTimeValidationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCurrentTimeValidationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExclusiveTrustRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExclusiveTrustRoots: usize,
}
windows_core::imp::define_interface!(IChainValidationParameters, IChainValidationParameters_Vtbl, 0xc4743b4a_7eb0_4b56_a040_b9c8e655ddf3);
impl windows_core::RuntimeType for IChainValidationParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChainValidationParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CertificateChainPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CertificateChainPolicy) -> windows_core::HRESULT,
    pub SetCertificateChainPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, CertificateChainPolicy) -> windows_core::HRESULT,
    #[cfg(feature = "Networking")]
    pub ServerDnsName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    ServerDnsName: usize,
    #[cfg(feature = "Networking")]
    pub SetServerDnsName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    SetServerDnsName: usize,
}
windows_core::imp::define_interface!(ICmsAttachedSignature, ICmsAttachedSignature_Vtbl, 0x61899d9d_3757_4ecb_bddc_0ca357d7a936);
impl windows_core::RuntimeType for ICmsAttachedSignature {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICmsAttachedSignature_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Certificates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Certificates: usize,
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Signers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Signers: usize,
    pub VerifySignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SignatureValidationResult) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICmsAttachedSignatureFactory, ICmsAttachedSignatureFactory_Vtbl, 0xd0c8fc15_f757_4c64_a362_52cc1c77cffb);
impl windows_core::RuntimeType for ICmsAttachedSignatureFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICmsAttachedSignatureFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCmsAttachedSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCmsAttachedSignature: usize,
}
windows_core::imp::define_interface!(ICmsAttachedSignatureStatics, ICmsAttachedSignatureStatics_Vtbl, 0x87989c8e_b0ad_498d_a7f5_78b59bce4b36);
impl windows_core::RuntimeType for ICmsAttachedSignatureStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICmsAttachedSignatureStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GenerateSignatureAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GenerateSignatureAsync: usize,
}
windows_core::imp::define_interface!(ICmsDetachedSignature, ICmsDetachedSignature_Vtbl, 0x0f1ef154_f65e_4536_8339_5944081db2ca);
impl windows_core::RuntimeType for ICmsDetachedSignature {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICmsDetachedSignature_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Certificates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Certificates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Signers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Signers: usize,
    #[cfg(feature = "Storage_Streams")]
    pub VerifySignatureAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VerifySignatureAsync: usize,
}
windows_core::imp::define_interface!(ICmsDetachedSignatureFactory, ICmsDetachedSignatureFactory_Vtbl, 0xc4ab3503_ae7f_4387_ad19_00f150e48ebb);
impl windows_core::RuntimeType for ICmsDetachedSignatureFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICmsDetachedSignatureFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCmsDetachedSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCmsDetachedSignature: usize,
}
windows_core::imp::define_interface!(ICmsDetachedSignatureStatics, ICmsDetachedSignatureStatics_Vtbl, 0x3d114cfd_bf9b_4682_9be6_91f57c053808);
impl windows_core::RuntimeType for ICmsDetachedSignatureStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICmsDetachedSignatureStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GenerateSignatureAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GenerateSignatureAsync: usize,
}
windows_core::imp::define_interface!(ICmsSignerInfo, ICmsSignerInfo_Vtbl, 0x50d020db_1d2f_4c1a_b5c5_d0188ff91f47);
impl windows_core::RuntimeType for ICmsSignerInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICmsSignerInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Certificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HashAlgorithmName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetHashAlgorithmName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TimestampInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICmsTimestampInfo, ICmsTimestampInfo_Vtbl, 0x2f5f00f2_2c18_4f88_8435_c534086076f5);
impl windows_core::RuntimeType for ICmsTimestampInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICmsTimestampInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SigningCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Certificates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Certificates: usize,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyAlgorithmNamesStatics, IKeyAlgorithmNamesStatics_Vtbl, 0x479065d7_7ac7_4581_8c3b_d07027140448);
impl windows_core::RuntimeType for IKeyAlgorithmNamesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKeyAlgorithmNamesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Rsa: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Dsa: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Ecdh256: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Ecdh384: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Ecdh521: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Ecdsa256: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Ecdsa384: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Ecdsa521: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyAlgorithmNamesStatics2, IKeyAlgorithmNamesStatics2_Vtbl, 0xc99b5686_e1fd_4a4a_893d_a26f33dd8bb4);
impl windows_core::RuntimeType for IKeyAlgorithmNamesStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKeyAlgorithmNamesStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Ecdsa: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Ecdh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyAttestationHelperStatics, IKeyAttestationHelperStatics_Vtbl, 0x1648e246_f644_4326_88be_3af102d30e0c);
impl windows_core::RuntimeType for IKeyAttestationHelperStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKeyAttestationHelperStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DecryptTpmAttestationCredentialAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTpmAttestationCredentialId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyAttestationHelperStatics2, IKeyAttestationHelperStatics2_Vtbl, 0x9c590b2c_a6c6_4a5e_9e64_e85d5279df97);
impl windows_core::RuntimeType for IKeyAttestationHelperStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKeyAttestationHelperStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DecryptTpmAttestationCredentialWithContainerNameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyStorageProviderNamesStatics, IKeyStorageProviderNamesStatics_Vtbl, 0xaf186ae0_5529_4602_bd94_0aab91957b5c);
impl windows_core::RuntimeType for IKeyStorageProviderNamesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKeyStorageProviderNamesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SoftwareKeyStorageProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SmartcardKeyStorageProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PlatformKeyStorageProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyStorageProviderNamesStatics2, IKeyStorageProviderNamesStatics2_Vtbl, 0x262d743d_9c2e_41cc_8812_c4d971dd7c60);
impl windows_core::RuntimeType for IKeyStorageProviderNamesStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKeyStorageProviderNamesStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PassportKeyStorageProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPfxImportParameters, IPfxImportParameters_Vtbl, 0x680d3511_9a08_47c8_864a_2edd4d8eb46c);
impl windows_core::RuntimeType for IPfxImportParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPfxImportParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Exportable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ExportOption) -> windows_core::HRESULT,
    pub SetExportable: unsafe extern "system" fn(*mut core::ffi::c_void, ExportOption) -> windows_core::HRESULT,
    pub KeyProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut KeyProtectionLevel) -> windows_core::HRESULT,
    pub SetKeyProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, KeyProtectionLevel) -> windows_core::HRESULT,
    pub InstallOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InstallOptions) -> windows_core::HRESULT,
    pub SetInstallOptions: unsafe extern "system" fn(*mut core::ffi::c_void, InstallOptions) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub KeyStorageProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetKeyStorageProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ContainerNamePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetContainerNamePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetReaderName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStandardCertificateStoreNamesStatics, IStandardCertificateStoreNamesStatics_Vtbl, 0x0c154adb_a496_41f8_8fe5_9e96f36efbf8);
impl windows_core::RuntimeType for IStandardCertificateStoreNamesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStandardCertificateStoreNamesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Personal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TrustedRootCertificationAuthorities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IntermediateCertificationAuthorities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISubjectAlternativeNameInfo, ISubjectAlternativeNameInfo_Vtbl, 0x582859f1_569d_4c20_be7b_4e1c9a0bc52b);
impl windows_core::RuntimeType for ISubjectAlternativeNameInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISubjectAlternativeNameInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EmailName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EmailName: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub IPAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    IPAddress: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Url: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Url: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DnsName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DnsName: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DistinguishedName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DistinguishedName: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PrincipalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PrincipalName: usize,
}
windows_core::imp::define_interface!(ISubjectAlternativeNameInfo2, ISubjectAlternativeNameInfo2_Vtbl, 0x437a78c6_1c51_41ea_b34a_3d654398a370);
impl windows_core::RuntimeType for ISubjectAlternativeNameInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISubjectAlternativeNameInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EmailNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EmailNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub IPAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    IPAddresses: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Urls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Urls: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DnsNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DnsNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DistinguishedNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DistinguishedNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PrincipalNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PrincipalNames: usize,
    pub Extension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserCertificateEnrollmentManager, IUserCertificateEnrollmentManager_Vtbl, 0x96313718_22e1_4819_b20b_ab46a6eca06e);
impl windows_core::RuntimeType for IUserCertificateEnrollmentManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserCertificateEnrollmentManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateRequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallCertificateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, InstallOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImportPfxDataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, ExportOption, KeyProtectionLevel, InstallOptions, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImportPfxDataToKspAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, ExportOption, KeyProtectionLevel, InstallOptions, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserCertificateEnrollmentManager2, IUserCertificateEnrollmentManager2_Vtbl, 0x0dad9cb1_65de_492a_b86d_fc5c482c3747);
impl windows_core::RuntimeType for IUserCertificateEnrollmentManager2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserCertificateEnrollmentManager2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ImportPfxDataToKspWithParametersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserCertificateStore, IUserCertificateStore_Vtbl, 0xc9fb1d83_789f_4b4e_9180_045a757aac6d);
impl windows_core::RuntimeType for IUserCertificateStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserCertificateStore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAddAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestDeleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Certificate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Certificate, windows_core::IUnknown, windows_core::IInspectable);
impl Certificate {
    #[cfg(feature = "Foundation_Collections")]
    pub fn BuildChainAsync<P0>(&self, certificates: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<CertificateChain>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<Certificate>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BuildChainAsync)(windows_core::Interface::as_raw(this), certificates.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn BuildChainWithParametersAsync<P0, P1>(&self, certificates: P0, parameters: P1) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<CertificateChain>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<Certificate>>,
        P1: windows_core::Param<ChainBuildingParameters>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BuildChainWithParametersAsync)(windows_core::Interface::as_raw(this), certificates.param().abi(), parameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SerialNumber(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).SerialNumber)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn GetHashValue(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetHashValue)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn GetHashValueWithAlgorithm(&self, hashalgorithmname: &windows_core::HSTRING) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetHashValueWithAlgorithm)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(hashalgorithmname), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetCertificateBlob(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCertificateBlob)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Subject(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subject)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Issuer(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Issuer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasPrivateKey(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasPrivateKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStronglyProtected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStronglyProtected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ValidFrom(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValidFrom)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ValidTo(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValidTo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnhancedKeyUsages(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnhancedKeyUsages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFriendlyName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFriendlyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FriendlyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsSecurityDeviceBound(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSecurityDeviceBound)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn KeyUsages(&self) -> windows_core::Result<CertificateKeyUsages> {
        let this = &windows_core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyUsages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn KeyAlgorithmName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyAlgorithmName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SignatureAlgorithmName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignatureAlgorithmName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SignatureHashAlgorithmName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignatureHashAlgorithmName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SubjectAlternativeName(&self) -> windows_core::Result<SubjectAlternativeNameInfo> {
        let this = &windows_core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubjectAlternativeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPerUser(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICertificate3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPerUser)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StoreName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICertificate3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StoreName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn KeyStorageProviderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICertificate3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyStorageProviderName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCertificate<P0>(certblob: P0) -> windows_core::Result<Certificate>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        Self::ICertificateFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCertificate)(windows_core::Interface::as_raw(this), certblob.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICertificateFactory<R, F: FnOnce(&ICertificateFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Certificate, ICertificateFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Certificate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICertificate>();
}
unsafe impl windows_core::Interface for Certificate {
    type Vtable = ICertificate_Vtbl;
    const IID: windows_core::GUID = <ICertificate as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Certificate {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.Certificate";
}
unsafe impl Send for Certificate {}
unsafe impl Sync for Certificate {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CertificateChain(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CertificateChain, windows_core::IUnknown, windows_core::IInspectable);
impl CertificateChain {
    pub fn Validate(&self) -> windows_core::Result<ChainValidationResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Validate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ValidateWithParameters<P0>(&self, parameter: P0) -> windows_core::Result<ChainValidationResult>
    where
        P0: windows_core::Param<ChainValidationParameters>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValidateWithParameters)(windows_core::Interface::as_raw(this), parameter.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCertificates(&self, includeroot: bool) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCertificates)(windows_core::Interface::as_raw(this), includeroot, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CertificateChain {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICertificateChain>();
}
unsafe impl windows_core::Interface for CertificateChain {
    type Vtable = ICertificateChain_Vtbl;
    const IID: windows_core::GUID = <ICertificateChain as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CertificateChain {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateChain";
}
unsafe impl Send for CertificateChain {}
unsafe impl Sync for CertificateChain {}
pub struct CertificateEnrollmentManager;
impl CertificateEnrollmentManager {
    pub fn CreateRequestAsync<P0>(request: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<CertificateRequestProperties>,
    {
        Self::ICertificateEnrollmentManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateRequestAsync)(windows_core::Interface::as_raw(this), request.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn InstallCertificateAsync(certificate: &windows_core::HSTRING, installoption: InstallOptions) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ICertificateEnrollmentManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallCertificateAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(certificate), installoption, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ImportPfxDataAsync(pfxdata: &windows_core::HSTRING, password: &windows_core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ICertificateEnrollmentManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImportPfxDataAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(pfxdata), core::mem::transmute_copy(password), exportable, keyprotectionlevel, installoption, core::mem::transmute_copy(friendlyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn UserCertificateEnrollmentManager() -> windows_core::Result<UserCertificateEnrollmentManager> {
        Self::ICertificateEnrollmentManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserCertificateEnrollmentManager)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ImportPfxDataToKspAsync(pfxdata: &windows_core::HSTRING, password: &windows_core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &windows_core::HSTRING, keystorageprovider: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ICertificateEnrollmentManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImportPfxDataToKspAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(pfxdata), core::mem::transmute_copy(password), exportable, keyprotectionlevel, installoption, core::mem::transmute_copy(friendlyname), core::mem::transmute_copy(keystorageprovider), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ImportPfxDataToKspWithParametersAsync<P0>(pfxdata: &windows_core::HSTRING, password: &windows_core::HSTRING, pfximportparameters: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<PfxImportParameters>,
    {
        Self::ICertificateEnrollmentManagerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImportPfxDataToKspWithParametersAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(pfxdata), core::mem::transmute_copy(password), pfximportparameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICertificateEnrollmentManagerStatics<R, F: FnOnce(&ICertificateEnrollmentManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CertificateEnrollmentManager, ICertificateEnrollmentManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICertificateEnrollmentManagerStatics2<R, F: FnOnce(&ICertificateEnrollmentManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CertificateEnrollmentManager, ICertificateEnrollmentManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICertificateEnrollmentManagerStatics3<R, F: FnOnce(&ICertificateEnrollmentManagerStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CertificateEnrollmentManager, ICertificateEnrollmentManagerStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for CertificateEnrollmentManager {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateEnrollmentManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CertificateExtension(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CertificateExtension, windows_core::IUnknown, windows_core::IInspectable);
impl CertificateExtension {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CertificateExtension, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ObjectId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ObjectId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetObjectId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetObjectId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsCritical(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCritical)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCritical(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsCritical)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EncodeValue(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).EncodeValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn SetValue(&self, value: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
}
impl windows_core::RuntimeType for CertificateExtension {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICertificateExtension>();
}
unsafe impl windows_core::Interface for CertificateExtension {
    type Vtable = ICertificateExtension_Vtbl;
    const IID: windows_core::GUID = <ICertificateExtension as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CertificateExtension {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateExtension";
}
unsafe impl Send for CertificateExtension {}
unsafe impl Sync for CertificateExtension {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CertificateKeyUsages(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CertificateKeyUsages, windows_core::IUnknown, windows_core::IInspectable);
impl CertificateKeyUsages {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CertificateKeyUsages, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn EncipherOnly(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncipherOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEncipherOnly(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEncipherOnly)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CrlSign(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CrlSign)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCrlSign(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCrlSign)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyCertificateSign(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyCertificateSign)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyCertificateSign(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeyCertificateSign)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyAgreement(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyAgreement)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyAgreement(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeyAgreement)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DataEncipherment(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataEncipherment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDataEncipherment(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDataEncipherment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyEncipherment(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyEncipherment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyEncipherment(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeyEncipherment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NonRepudiation(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NonRepudiation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNonRepudiation(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNonRepudiation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DigitalSignature(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DigitalSignature)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDigitalSignature(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDigitalSignature)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for CertificateKeyUsages {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICertificateKeyUsages>();
}
unsafe impl windows_core::Interface for CertificateKeyUsages {
    type Vtable = ICertificateKeyUsages_Vtbl;
    const IID: windows_core::GUID = <ICertificateKeyUsages as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CertificateKeyUsages {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateKeyUsages";
}
unsafe impl Send for CertificateKeyUsages {}
unsafe impl Sync for CertificateKeyUsages {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CertificateQuery(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CertificateQuery, windows_core::IUnknown, windows_core::IInspectable);
impl CertificateQuery {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CertificateQuery, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnhancedKeyUsages(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnhancedKeyUsages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IssuerName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IssuerName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIssuerName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIssuerName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FriendlyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFriendlyName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFriendlyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Thumbprint(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).Thumbprint)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn SetThumbprint(&self, value: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetThumbprint)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn HardwareOnly(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHardwareOnly(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHardwareOnly)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeDuplicates(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeDuplicates)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIncludeDuplicates(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIncludeDuplicates)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeExpiredCertificates(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeExpiredCertificates)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIncludeExpiredCertificates(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIncludeExpiredCertificates)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StoreName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StoreName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStoreName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICertificateQuery2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStoreName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for CertificateQuery {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICertificateQuery>();
}
unsafe impl windows_core::Interface for CertificateQuery {
    type Vtable = ICertificateQuery_Vtbl;
    const IID: windows_core::GUID = <ICertificateQuery as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CertificateQuery {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateQuery";
}
unsafe impl Send for CertificateQuery {}
unsafe impl Sync for CertificateQuery {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CertificateRequestProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CertificateRequestProperties, windows_core::IUnknown, windows_core::IInspectable);
impl CertificateRequestProperties {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CertificateRequestProperties, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Subject(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subject)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSubject(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSubject)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn KeyAlgorithmName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyAlgorithmName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyAlgorithmName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeyAlgorithmName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn KeySize(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeySize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeySize(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeySize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FriendlyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFriendlyName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFriendlyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn HashAlgorithmName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HashAlgorithmName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetHashAlgorithmName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHashAlgorithmName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Exportable(&self) -> windows_core::Result<ExportOption> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Exportable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExportable(&self, value: ExportOption) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExportable)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyUsages(&self) -> windows_core::Result<EnrollKeyUsages> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyUsages)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyUsages(&self, value: EnrollKeyUsages) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeyUsages)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyProtectionLevel(&self) -> windows_core::Result<KeyProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyProtectionLevel(&self, value: KeyProtectionLevel) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeyProtectionLevel)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyStorageProviderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyStorageProviderName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyStorageProviderName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeyStorageProviderName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SmartcardReaderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmartcardReaderName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSmartcardReaderName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSmartcardReaderName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SigningCertificate(&self) -> windows_core::Result<Certificate> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SigningCertificate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSigningCertificate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Certificate>,
    {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSigningCertificate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AttestationCredentialCertificate(&self) -> windows_core::Result<Certificate> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttestationCredentialCertificate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAttestationCredentialCertificate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Certificate>,
    {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAttestationCredentialCertificate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CurveName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurveName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCurveName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCurveName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CurveParameters(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).CurveParameters)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn SetCurveParameters(&self, value: &[u8]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCurveParameters)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn ContainerNamePrefix(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContainerNamePrefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContainerNamePrefix(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetContainerNamePrefix)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContainerName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContainerName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContainerName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetContainerName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn UseExistingKey(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseExistingKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseExistingKey(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseExistingKey)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SuppressedDefaults(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuppressedDefaults)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SubjectAlternativeName(&self) -> windows_core::Result<SubjectAlternativeNameInfo> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubjectAlternativeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Extensions(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<CertificateExtension>> {
        let this = &windows_core::Interface::cast::<ICertificateRequestProperties4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Extensions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CertificateRequestProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICertificateRequestProperties>();
}
unsafe impl windows_core::Interface for CertificateRequestProperties {
    type Vtable = ICertificateRequestProperties_Vtbl;
    const IID: windows_core::GUID = <ICertificateRequestProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CertificateRequestProperties {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateRequestProperties";
}
unsafe impl Send for CertificateRequestProperties {}
unsafe impl Sync for CertificateRequestProperties {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CertificateStore(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CertificateStore, windows_core::IUnknown, windows_core::IInspectable);
impl CertificateStore {
    pub fn Add<P0>(&self, certificate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Certificate>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Add)(windows_core::Interface::as_raw(this), certificate.param().abi()).ok() }
    }
    pub fn Delete<P0>(&self, certificate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Certificate>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Delete)(windows_core::Interface::as_raw(this), certificate.param().abi()).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICertificateStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CertificateStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICertificateStore>();
}
unsafe impl windows_core::Interface for CertificateStore {
    type Vtable = ICertificateStore_Vtbl;
    const IID: windows_core::GUID = <ICertificateStore as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CertificateStore {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateStore";
}
unsafe impl Send for CertificateStore {}
unsafe impl Sync for CertificateStore {}
pub struct CertificateStores;
impl CertificateStores {
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync() -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<Certificate>>> {
        Self::ICertificateStoresStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllWithQueryAsync<P0>(query: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<Certificate>>>
    where
        P0: windows_core::Param<CertificateQuery>,
    {
        Self::ICertificateStoresStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllWithQueryAsync)(windows_core::Interface::as_raw(this), query.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TrustedRootCertificationAuthorities() -> windows_core::Result<CertificateStore> {
        Self::ICertificateStoresStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrustedRootCertificationAuthorities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IntermediateCertificationAuthorities() -> windows_core::Result<CertificateStore> {
        Self::ICertificateStoresStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IntermediateCertificationAuthorities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetStoreByName(storename: &windows_core::HSTRING) -> windows_core::Result<CertificateStore> {
        Self::ICertificateStoresStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStoreByName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(storename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetUserStoreByName(storename: &windows_core::HSTRING) -> windows_core::Result<UserCertificateStore> {
        Self::ICertificateStoresStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUserStoreByName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(storename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICertificateStoresStatics<R, F: FnOnce(&ICertificateStoresStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CertificateStores, ICertificateStoresStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICertificateStoresStatics2<R, F: FnOnce(&ICertificateStoresStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CertificateStores, ICertificateStoresStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for CertificateStores {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateStores";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChainBuildingParameters(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChainBuildingParameters, windows_core::IUnknown, windows_core::IInspectable);
impl ChainBuildingParameters {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ChainBuildingParameters, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnhancedKeyUsages(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnhancedKeyUsages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ValidationTimestamp(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValidationTimestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetValidationTimestamp(&self, value: super::super::super::Foundation::DateTime) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValidationTimestamp)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RevocationCheckEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RevocationCheckEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRevocationCheckEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRevocationCheckEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NetworkRetrievalEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkRetrievalEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNetworkRetrievalEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNetworkRetrievalEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AuthorityInformationAccessEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthorityInformationAccessEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAuthorityInformationAccessEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAuthorityInformationAccessEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CurrentTimeValidationEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentTimeValidationEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCurrentTimeValidationEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCurrentTimeValidationEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExclusiveTrustRoots(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExclusiveTrustRoots)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ChainBuildingParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChainBuildingParameters>();
}
unsafe impl windows_core::Interface for ChainBuildingParameters {
    type Vtable = IChainBuildingParameters_Vtbl;
    const IID: windows_core::GUID = <IChainBuildingParameters as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChainBuildingParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ChainBuildingParameters";
}
unsafe impl Send for ChainBuildingParameters {}
unsafe impl Sync for ChainBuildingParameters {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChainValidationParameters(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChainValidationParameters, windows_core::IUnknown, windows_core::IInspectable);
impl ChainValidationParameters {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ChainValidationParameters, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CertificateChainPolicy(&self) -> windows_core::Result<CertificateChainPolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CertificateChainPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCertificateChainPolicy(&self, value: CertificateChainPolicy) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCertificateChainPolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Networking")]
    pub fn ServerDnsName(&self) -> windows_core::Result<super::super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerDnsName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Networking")]
    pub fn SetServerDnsName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Networking::HostName>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetServerDnsName)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for ChainValidationParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChainValidationParameters>();
}
unsafe impl windows_core::Interface for ChainValidationParameters {
    type Vtable = IChainValidationParameters_Vtbl;
    const IID: windows_core::GUID = <IChainValidationParameters as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChainValidationParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ChainValidationParameters";
}
unsafe impl Send for ChainValidationParameters {}
unsafe impl Sync for ChainValidationParameters {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CmsAttachedSignature(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CmsAttachedSignature, windows_core::IUnknown, windows_core::IInspectable);
impl CmsAttachedSignature {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Certificates(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Certificates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Content(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Signers(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<CmsSignerInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Signers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VerifySignature(&self) -> windows_core::Result<SignatureValidationResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerifySignature)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCmsAttachedSignature<P0>(inputblob: P0) -> windows_core::Result<CmsAttachedSignature>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        Self::ICmsAttachedSignatureFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCmsAttachedSignature)(windows_core::Interface::as_raw(this), inputblob.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GenerateSignatureAsync<P0, P1, P2>(data: P0, signers: P1, certificates: P2) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<super::super::super::Foundation::Collections::IIterable<CmsSignerInfo>>,
        P2: windows_core::Param<super::super::super::Foundation::Collections::IIterable<Certificate>>,
    {
        Self::ICmsAttachedSignatureStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GenerateSignatureAsync)(windows_core::Interface::as_raw(this), data.param().abi(), signers.param().abi(), certificates.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICmsAttachedSignatureFactory<R, F: FnOnce(&ICmsAttachedSignatureFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CmsAttachedSignature, ICmsAttachedSignatureFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICmsAttachedSignatureStatics<R, F: FnOnce(&ICmsAttachedSignatureStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CmsAttachedSignature, ICmsAttachedSignatureStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CmsAttachedSignature {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICmsAttachedSignature>();
}
unsafe impl windows_core::Interface for CmsAttachedSignature {
    type Vtable = ICmsAttachedSignature_Vtbl;
    const IID: windows_core::GUID = <ICmsAttachedSignature as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CmsAttachedSignature {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CmsAttachedSignature";
}
unsafe impl Send for CmsAttachedSignature {}
unsafe impl Sync for CmsAttachedSignature {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CmsDetachedSignature(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CmsDetachedSignature, windows_core::IUnknown, windows_core::IInspectable);
impl CmsDetachedSignature {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Certificates(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Certificates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Signers(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<CmsSignerInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Signers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn VerifySignatureAsync<P0>(&self, data: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<SignatureValidationResult>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerifySignatureAsync)(windows_core::Interface::as_raw(this), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCmsDetachedSignature<P0>(inputblob: P0) -> windows_core::Result<CmsDetachedSignature>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        Self::ICmsDetachedSignatureFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCmsDetachedSignature)(windows_core::Interface::as_raw(this), inputblob.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GenerateSignatureAsync<P0, P1, P2>(data: P0, signers: P1, certificates: P2) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IInputStream>,
        P1: windows_core::Param<super::super::super::Foundation::Collections::IIterable<CmsSignerInfo>>,
        P2: windows_core::Param<super::super::super::Foundation::Collections::IIterable<Certificate>>,
    {
        Self::ICmsDetachedSignatureStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GenerateSignatureAsync)(windows_core::Interface::as_raw(this), data.param().abi(), signers.param().abi(), certificates.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICmsDetachedSignatureFactory<R, F: FnOnce(&ICmsDetachedSignatureFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CmsDetachedSignature, ICmsDetachedSignatureFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICmsDetachedSignatureStatics<R, F: FnOnce(&ICmsDetachedSignatureStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CmsDetachedSignature, ICmsDetachedSignatureStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CmsDetachedSignature {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICmsDetachedSignature>();
}
unsafe impl windows_core::Interface for CmsDetachedSignature {
    type Vtable = ICmsDetachedSignature_Vtbl;
    const IID: windows_core::GUID = <ICmsDetachedSignature as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CmsDetachedSignature {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CmsDetachedSignature";
}
unsafe impl Send for CmsDetachedSignature {}
unsafe impl Sync for CmsDetachedSignature {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CmsSignerInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CmsSignerInfo, windows_core::IUnknown, windows_core::IInspectable);
impl CmsSignerInfo {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CmsSignerInfo, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Certificate(&self) -> windows_core::Result<Certificate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Certificate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCertificate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Certificate>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCertificate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn HashAlgorithmName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HashAlgorithmName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetHashAlgorithmName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHashAlgorithmName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn TimestampInfo(&self) -> windows_core::Result<CmsTimestampInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimestampInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CmsSignerInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICmsSignerInfo>();
}
unsafe impl windows_core::Interface for CmsSignerInfo {
    type Vtable = ICmsSignerInfo_Vtbl;
    const IID: windows_core::GUID = <ICmsSignerInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CmsSignerInfo {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CmsSignerInfo";
}
unsafe impl Send for CmsSignerInfo {}
unsafe impl Sync for CmsSignerInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CmsTimestampInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CmsTimestampInfo, windows_core::IUnknown, windows_core::IInspectable);
impl CmsTimestampInfo {
    pub fn SigningCertificate(&self) -> windows_core::Result<Certificate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SigningCertificate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Certificates(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Certificates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CmsTimestampInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICmsTimestampInfo>();
}
unsafe impl windows_core::Interface for CmsTimestampInfo {
    type Vtable = ICmsTimestampInfo_Vtbl;
    const IID: windows_core::GUID = <ICmsTimestampInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CmsTimestampInfo {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CmsTimestampInfo";
}
unsafe impl Send for CmsTimestampInfo {}
unsafe impl Sync for CmsTimestampInfo {}
pub struct KeyAlgorithmNames;
impl KeyAlgorithmNames {
    pub fn Rsa() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rsa)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Dsa() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dsa)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Ecdh256() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ecdh256)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Ecdh384() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ecdh384)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Ecdh521() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ecdh521)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Ecdsa256() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ecdsa256)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Ecdsa384() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ecdsa384)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Ecdsa521() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ecdsa521)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Ecdsa() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ecdsa)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Ecdh() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKeyAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Ecdh)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IKeyAlgorithmNamesStatics<R, F: FnOnce(&IKeyAlgorithmNamesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KeyAlgorithmNames, IKeyAlgorithmNamesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IKeyAlgorithmNamesStatics2<R, F: FnOnce(&IKeyAlgorithmNamesStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KeyAlgorithmNames, IKeyAlgorithmNamesStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for KeyAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.KeyAlgorithmNames";
}
pub struct KeyAttestationHelper;
impl KeyAttestationHelper {
    pub fn DecryptTpmAttestationCredentialAsync(credential: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        Self::IKeyAttestationHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DecryptTpmAttestationCredentialAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(credential), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetTpmAttestationCredentialId(credential: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        Self::IKeyAttestationHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTpmAttestationCredentialId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(credential), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DecryptTpmAttestationCredentialWithContainerNameAsync(credential: &windows_core::HSTRING, containername: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        Self::IKeyAttestationHelperStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DecryptTpmAttestationCredentialWithContainerNameAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(credential), core::mem::transmute_copy(containername), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IKeyAttestationHelperStatics<R, F: FnOnce(&IKeyAttestationHelperStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KeyAttestationHelper, IKeyAttestationHelperStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IKeyAttestationHelperStatics2<R, F: FnOnce(&IKeyAttestationHelperStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KeyAttestationHelper, IKeyAttestationHelperStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for KeyAttestationHelper {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.KeyAttestationHelper";
}
pub struct KeyStorageProviderNames;
impl KeyStorageProviderNames {
    pub fn SoftwareKeyStorageProvider() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKeyStorageProviderNamesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SoftwareKeyStorageProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SmartcardKeyStorageProvider() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKeyStorageProviderNamesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmartcardKeyStorageProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PlatformKeyStorageProvider() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKeyStorageProviderNamesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlatformKeyStorageProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PassportKeyStorageProvider() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKeyStorageProviderNamesStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PassportKeyStorageProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IKeyStorageProviderNamesStatics<R, F: FnOnce(&IKeyStorageProviderNamesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KeyStorageProviderNames, IKeyStorageProviderNamesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IKeyStorageProviderNamesStatics2<R, F: FnOnce(&IKeyStorageProviderNamesStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KeyStorageProviderNames, IKeyStorageProviderNamesStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for KeyStorageProviderNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.KeyStorageProviderNames";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PfxImportParameters(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PfxImportParameters, windows_core::IUnknown, windows_core::IInspectable);
impl PfxImportParameters {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PfxImportParameters, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Exportable(&self) -> windows_core::Result<ExportOption> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Exportable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExportable(&self, value: ExportOption) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExportable)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyProtectionLevel(&self) -> windows_core::Result<KeyProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyProtectionLevel(&self, value: KeyProtectionLevel) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeyProtectionLevel)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallOptions(&self) -> windows_core::Result<InstallOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallOptions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInstallOptions(&self, value: InstallOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInstallOptions)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FriendlyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFriendlyName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFriendlyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn KeyStorageProviderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyStorageProviderName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetKeyStorageProviderName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeyStorageProviderName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContainerNamePrefix(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContainerNamePrefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContainerNamePrefix(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContainerNamePrefix)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ReaderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReaderName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetReaderName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReaderName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for PfxImportParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPfxImportParameters>();
}
unsafe impl windows_core::Interface for PfxImportParameters {
    type Vtable = IPfxImportParameters_Vtbl;
    const IID: windows_core::GUID = <IPfxImportParameters as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PfxImportParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.PfxImportParameters";
}
unsafe impl Send for PfxImportParameters {}
unsafe impl Sync for PfxImportParameters {}
pub struct StandardCertificateStoreNames;
impl StandardCertificateStoreNames {
    pub fn Personal() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardCertificateStoreNamesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Personal)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TrustedRootCertificationAuthorities() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardCertificateStoreNamesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrustedRootCertificationAuthorities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IntermediateCertificationAuthorities() -> windows_core::Result<windows_core::HSTRING> {
        Self::IStandardCertificateStoreNamesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IntermediateCertificationAuthorities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IStandardCertificateStoreNamesStatics<R, F: FnOnce(&IStandardCertificateStoreNamesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StandardCertificateStoreNames, IStandardCertificateStoreNamesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for StandardCertificateStoreNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.StandardCertificateStoreNames";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SubjectAlternativeNameInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SubjectAlternativeNameInfo, windows_core::IUnknown, windows_core::IInspectable);
impl SubjectAlternativeNameInfo {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SubjectAlternativeNameInfo, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EmailName(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IPAddress(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IPAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Url(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Url)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DnsName(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DnsName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DistinguishedName(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DistinguishedName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PrincipalName(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrincipalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EmailNames(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IPAddresses(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IPAddresses)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Urls(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Urls)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DnsNames(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DnsNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DistinguishedNames(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DistinguishedNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PrincipalNames(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrincipalNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Extension(&self) -> windows_core::Result<CertificateExtension> {
        let this = &windows_core::Interface::cast::<ISubjectAlternativeNameInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Extension)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SubjectAlternativeNameInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISubjectAlternativeNameInfo>();
}
unsafe impl windows_core::Interface for SubjectAlternativeNameInfo {
    type Vtable = ISubjectAlternativeNameInfo_Vtbl;
    const IID: windows_core::GUID = <ISubjectAlternativeNameInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SubjectAlternativeNameInfo {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.SubjectAlternativeNameInfo";
}
unsafe impl Send for SubjectAlternativeNameInfo {}
unsafe impl Sync for SubjectAlternativeNameInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserCertificateEnrollmentManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserCertificateEnrollmentManager, windows_core::IUnknown, windows_core::IInspectable);
impl UserCertificateEnrollmentManager {
    pub fn CreateRequestAsync<P0>(&self, request: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<CertificateRequestProperties>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateRequestAsync)(windows_core::Interface::as_raw(this), request.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InstallCertificateAsync(&self, certificate: &windows_core::HSTRING, installoption: InstallOptions) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallCertificateAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(certificate), installoption, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ImportPfxDataAsync(&self, pfxdata: &windows_core::HSTRING, password: &windows_core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImportPfxDataAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(pfxdata), core::mem::transmute_copy(password), exportable, keyprotectionlevel, installoption, core::mem::transmute_copy(friendlyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ImportPfxDataToKspAsync(&self, pfxdata: &windows_core::HSTRING, password: &windows_core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &windows_core::HSTRING, keystorageprovider: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImportPfxDataToKspAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(pfxdata), core::mem::transmute_copy(password), exportable, keyprotectionlevel, installoption, core::mem::transmute_copy(friendlyname), core::mem::transmute_copy(keystorageprovider), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ImportPfxDataToKspWithParametersAsync<P0>(&self, pfxdata: &windows_core::HSTRING, password: &windows_core::HSTRING, pfximportparameters: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<PfxImportParameters>,
    {
        let this = &windows_core::Interface::cast::<IUserCertificateEnrollmentManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImportPfxDataToKspWithParametersAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(pfxdata), core::mem::transmute_copy(password), pfximportparameters.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserCertificateEnrollmentManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserCertificateEnrollmentManager>();
}
unsafe impl windows_core::Interface for UserCertificateEnrollmentManager {
    type Vtable = IUserCertificateEnrollmentManager_Vtbl;
    const IID: windows_core::GUID = <IUserCertificateEnrollmentManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserCertificateEnrollmentManager {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.UserCertificateEnrollmentManager";
}
unsafe impl Send for UserCertificateEnrollmentManager {}
unsafe impl Sync for UserCertificateEnrollmentManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserCertificateStore(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserCertificateStore, windows_core::IUnknown, windows_core::IInspectable);
impl UserCertificateStore {
    pub fn RequestAddAsync<P0>(&self, certificate: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<Certificate>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAddAsync)(windows_core::Interface::as_raw(this), certificate.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestDeleteAsync<P0>(&self, certificate: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<Certificate>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestDeleteAsync)(windows_core::Interface::as_raw(this), certificate.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserCertificateStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserCertificateStore>();
}
unsafe impl windows_core::Interface for UserCertificateStore {
    type Vtable = IUserCertificateStore_Vtbl;
    const IID: windows_core::GUID = <IUserCertificateStore as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserCertificateStore {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.UserCertificateStore";
}
unsafe impl Send for UserCertificateStore {}
unsafe impl Sync for UserCertificateStore {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CertificateChainPolicy(pub i32);
impl CertificateChainPolicy {
    pub const Base: Self = Self(0i32);
    pub const Ssl: Self = Self(1i32);
    pub const NTAuthentication: Self = Self(2i32);
    pub const MicrosoftRoot: Self = Self(3i32);
}
impl windows_core::TypeKind for CertificateChainPolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CertificateChainPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CertificateChainPolicy").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CertificateChainPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.CertificateChainPolicy;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ChainValidationResult(pub i32);
impl ChainValidationResult {
    pub const Success: Self = Self(0i32);
    pub const Untrusted: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Expired: Self = Self(3i32);
    pub const IncompleteChain: Self = Self(4i32);
    pub const InvalidSignature: Self = Self(5i32);
    pub const WrongUsage: Self = Self(6i32);
    pub const InvalidName: Self = Self(7i32);
    pub const InvalidCertificateAuthorityPolicy: Self = Self(8i32);
    pub const BasicConstraintsError: Self = Self(9i32);
    pub const UnknownCriticalExtension: Self = Self(10i32);
    pub const RevocationInformationMissing: Self = Self(11i32);
    pub const RevocationFailure: Self = Self(12i32);
    pub const OtherErrors: Self = Self(13i32);
}
impl windows_core::TypeKind for ChainValidationResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ChainValidationResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ChainValidationResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ChainValidationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.ChainValidationResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EnrollKeyUsages(pub u32);
impl EnrollKeyUsages {
    pub const None: Self = Self(0u32);
    pub const Decryption: Self = Self(1u32);
    pub const Signing: Self = Self(2u32);
    pub const KeyAgreement: Self = Self(4u32);
    pub const All: Self = Self(16777215u32);
}
impl windows_core::TypeKind for EnrollKeyUsages {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EnrollKeyUsages {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EnrollKeyUsages").field(&self.0).finish()
    }
}
impl EnrollKeyUsages {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for EnrollKeyUsages {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for EnrollKeyUsages {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for EnrollKeyUsages {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for EnrollKeyUsages {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for EnrollKeyUsages {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for EnrollKeyUsages {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.EnrollKeyUsages;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ExportOption(pub i32);
impl ExportOption {
    pub const NotExportable: Self = Self(0i32);
    pub const Exportable: Self = Self(1i32);
}
impl windows_core::TypeKind for ExportOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ExportOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ExportOption").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ExportOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.ExportOption;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InstallOptions(pub u32);
impl InstallOptions {
    pub const None: Self = Self(0u32);
    pub const DeleteExpired: Self = Self(1u32);
}
impl windows_core::TypeKind for InstallOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InstallOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InstallOptions").field(&self.0).finish()
    }
}
impl InstallOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for InstallOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for InstallOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for InstallOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for InstallOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for InstallOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for InstallOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.InstallOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct KeyProtectionLevel(pub i32);
impl KeyProtectionLevel {
    pub const NoConsent: Self = Self(0i32);
    pub const ConsentOnly: Self = Self(1i32);
    pub const ConsentWithPassword: Self = Self(2i32);
    pub const ConsentWithFingerprint: Self = Self(3i32);
}
impl windows_core::TypeKind for KeyProtectionLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for KeyProtectionLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KeyProtectionLevel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for KeyProtectionLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.KeyProtectionLevel;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct KeySize(pub i32);
impl KeySize {
    pub const Invalid: Self = Self(0i32);
    pub const Rsa2048: Self = Self(2048i32);
    pub const Rsa4096: Self = Self(4096i32);
}
impl windows_core::TypeKind for KeySize {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for KeySize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KeySize").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for KeySize {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.KeySize;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SignatureValidationResult(pub i32);
impl SignatureValidationResult {
    pub const Success: Self = Self(0i32);
    pub const InvalidParameter: Self = Self(1i32);
    pub const BadMessage: Self = Self(2i32);
    pub const InvalidSignature: Self = Self(3i32);
    pub const OtherErrors: Self = Self(4i32);
}
impl windows_core::TypeKind for SignatureValidationResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SignatureValidationResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SignatureValidationResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SignatureValidationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Certificates.SignatureValidationResult;i4)");
}
