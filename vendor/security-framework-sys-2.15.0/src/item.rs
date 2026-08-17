use core_foundation_sys::string::CFStringRef;

extern "C" {
    pub static kSecClass: CFStringRef;
    pub static kSecClassInternetPassword: CFStringRef;
    pub static kSecClassGenericPassword: CFStringRef;
    pub static kSecClassCertificate: CFStringRef;
    pub static kSecClassKey: CFStringRef;
    pub static kSecClassIdentity: CFStringRef;

    pub static kSecMatchLimit: CFStringRef;
    pub static kSecMatchLimitAll: CFStringRef;

    pub static kSecMatchTrustedOnly: CFStringRef;
    pub static kSecMatchCaseInsensitive: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecMatchSubjectWholeString: CFStringRef;

    pub static kSecReturnData: CFStringRef;
    pub static kSecReturnAttributes: CFStringRef;
    pub static kSecReturnRef: CFStringRef;
    pub static kSecReturnPersistentRef: CFStringRef;

    pub static kSecMatchSearchList: CFStringRef;

    pub static kSecAttrApplicationLabel: CFStringRef;
    pub static kSecAttrKeyType: CFStringRef;
    pub static kSecAttrLabel: CFStringRef;
    pub static kSecAttrIsPermanent: CFStringRef;
    pub static kSecAttrPublicKeyHash: CFStringRef;
    pub static kSecAttrSerialNumber: CFStringRef;
    pub static kSecPrivateKeyAttrs: CFStringRef;
    pub static kSecPublicKeyAttrs: CFStringRef;

    pub static kSecAttrKeyClass: CFStringRef;
    pub static kSecAttrKeyClassPublic: CFStringRef;
    pub static kSecAttrKeyClassPrivate: CFStringRef;
    pub static kSecAttrKeyClassSymmetric: CFStringRef;

    pub static kSecUseKeychain: CFStringRef;
    #[cfg(any(feature = "OSX_10_15", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub static kSecUseDataProtectionKeychain: CFStringRef;
    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub static kSecAttrTokenID: CFStringRef;
    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub static kSecAttrTokenIDSecureEnclave: CFStringRef;
    #[cfg(any(feature = "OSX_10_13", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub static kSecUseAuthenticationContext: CFStringRef;
    #[cfg(any(feature = "OSX_10_11", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub static kSecUseAuthenticationUI: CFStringRef;
    #[cfg(any(feature = "OSX_10_11", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub static kSecUseAuthenticationUISkip: CFStringRef;
    #[cfg(any(feature = "OSX_10_9", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub static kSecAttrSynchronizable: CFStringRef;
    #[cfg(any(feature = "OSX_10_9", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub static kSecAttrSynchronizableAny: CFStringRef;

    pub static kSecAttrKeySizeInBits: CFStringRef;

    pub static kSecAttrKeyTypeECSECPrimeRandom: CFStringRef;
    pub static kSecAttrKeyTypeRSA: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecAttrKeyTypeDSA: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecAttrKeyTypeAES: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecAttrKeyTypeDES: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecAttrKeyType3DES: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecAttrKeyTypeRC4: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecAttrKeyTypeRC2: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecAttrKeyTypeCAST: CFStringRef;
    pub static kSecAttrKeyTypeEC: CFStringRef;

    pub static kSecAttrAccessGroup: CFStringRef;
    pub static kSecAttrAccessGroupToken: CFStringRef;

    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub static kSecKeyKeyExchangeParameterRequestedSize: CFStringRef;
    #[cfg(any(feature = "OSX_10_12", target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "visionos"))]
    pub static kSecKeyKeyExchangeParameterSharedInfo: CFStringRef;

    pub static kSecAttrAuthenticationType: CFStringRef;
    pub static kSecAttrComment: CFStringRef;
    pub static kSecAttrDescription: CFStringRef;
    pub static kSecAttrPath: CFStringRef;
    pub static kSecAttrPort: CFStringRef;
    pub static kSecAttrProtocol: CFStringRef;
    pub static kSecAttrSecurityDomain: CFStringRef;
    pub static kSecAttrServer: CFStringRef;
    pub static kSecAttrService: CFStringRef;
    pub static kSecAttrAccessControl: CFStringRef;
    pub static kSecAttrAccount: CFStringRef;
    pub static kSecValueData: CFStringRef;
    pub static kSecValueRef: CFStringRef;
}
