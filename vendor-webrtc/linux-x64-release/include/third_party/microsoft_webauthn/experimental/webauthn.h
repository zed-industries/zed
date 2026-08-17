/*

MIT License

Copyright (c) Microsoft Corporation. All rights reserved.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE

*/

#pragma once

#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#ifdef __cplusplus
extern "C" {
#endif

#ifndef WINAPI
#define WINAPI __stdcall
#endif

#ifndef INITGUID
#define INITGUID
#include <guiddef.h>
#undef INITGUID
#else
#include <guiddef.h>
#endif

//+------------------------------------------------------------------------------------------
// API Version Information.
// Caller should check for WebAuthNGetApiVersionNumber to check the presence of
// relevant APIs and features for their usage.
//-------------------------------------------------------------------------------------------

#define WEBAUTHN_API_VERSION_1 1
// WEBAUTHN_API_VERSION_1 : Baseline Version
//      Data Structures and their sub versions:
//          - WEBAUTHN_RP_ENTITY_INFORMATION                    :   1
//          - WEBAUTHN_USER_ENTITY_INFORMATION                  :   1
//          - WEBAUTHN_CLIENT_DATA                              :   1
//          - WEBAUTHN_COSE_CREDENTIAL_PARAMETER                :   1
//          - WEBAUTHN_COSE_CREDENTIAL_PARAMETERS               :   Not
//          Applicable
//          - WEBAUTHN_CREDENTIAL                               :   1
//          - WEBAUTHN_CREDENTIALS                              :   Not
//          Applicable
//          - WEBAUTHN_CREDENTIAL_EX                            :   1
//          - WEBAUTHN_CREDENTIAL_LIST                          :   Not
//          Applicable
//          - WEBAUTHN_EXTENSION                                :   Not
//          Applicable
//          - WEBAUTHN_EXTENSIONS                               :   Not
//          Applicable
//          - WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS    :   3
//          - WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS      :   4
//          - WEBAUTHN_COMMON_ATTESTATION                       :   1
//          - WEBAUTHN_CREDENTIAL_ATTESTATION                   :   3
//          - WEBAUTHN_ASSERTION                                :   1
//      Extensions:
//          - WEBAUTHN_EXTENSIONS_IDENTIFIER_HMAC_SECRET
//      APIs:
//          - WebAuthNGetApiVersionNumber
//          - WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable
//          - WebAuthNAuthenticatorMakeCredential
//          - WebAuthNAuthenticatorGetAssertion
//          - WebAuthNFreeCredentialAttestation
//          - WebAuthNFreeAssertion
//          - WebAuthNGetCancellationId
//          - WebAuthNCancelCurrentOperation
//          - WebAuthNGetErrorName
//          - WebAuthNGetW3CExceptionDOMError
//      Transports:
//          - WEBAUTHN_CTAP_TRANSPORT_USB
//          - WEBAUTHN_CTAP_TRANSPORT_NFC
//          - WEBAUTHN_CTAP_TRANSPORT_BLE
//          - WEBAUTHN_CTAP_TRANSPORT_INTERNAL

#define WEBAUTHN_API_VERSION_2 2
// WEBAUTHN_API_VERSION_2 : Delta From WEBAUTHN_API_VERSION_1
//      Added Extensions:
//          - WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_PROTECT
//

#define WEBAUTHN_API_VERSION_3 3
// WEBAUTHN_API_VERSION_3 : Delta From WEBAUTHN_API_VERSION_2
//      Data Structures and their sub versions:
//          - WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS    :   4
//          - WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS      :   5
//          - WEBAUTHN_CREDENTIAL_ATTESTATION                   :   4
//          - WEBAUTHN_ASSERTION                                :   2
//      Added Extensions:
//          - WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_BLOB
//          - WEBAUTHN_EXTENSIONS_IDENTIFIER_MIN_PIN_LENGTH
//

#define WEBAUTHN_API_VERSION_4 4
// WEBAUTHN_API_VERSION_4 : Delta From WEBAUTHN_API_VERSION_3
//      Data Structures and their sub versions:
//          - WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS    :   5
//          - WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS      :   6
//          - WEBAUTHN_ASSERTION                                :   3
//          - WEBAUTHN_CREDENTIAL_DETAILS                       :   1
//      APIs:
//          - WebAuthNGetPlatformCredentialList
//          - WebAuthNFreePlatformCredentialList
//          - WebAuthNDeletePlatformCredential
//

#define WEBAUTHN_API_VERSION_5 5
// WEBAUTHN_API_VERSION_5 : Delta From WEBAUTHN_API_VERSION_4
//      Data Structures and their sub versions:
//          - WEBAUTHN_CREDENTIAL_DETAILS                       :   2
//      Extension Changes:
//          - Enabled LARGE_BLOB Support
//

#define WEBAUTHN_API_VERSION_6 6
// WEBAUTHN_API_VERSION_6 : Delta From WEBAUTHN_API_VERSION_5
//      Data Structures and their sub versions:
//          - WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS    :   6
//          - WEBAUTHN_CREDENTIAL_ATTESTATION                   :   5
//          - WEBAUTHN_ASSERTION                                :   4
//      Transports:
//          - WEBAUTHN_CTAP_TRANSPORT_HYBRID

#define WEBAUTHN_API_VERSION_7 7
// WEBAUTHN_API_VERSION_7 : Delta From WEBAUTHN_API_VERSION_6
//      Data Structures and their sub versions:
//          - WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS    :   7
//          - WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS      :   7
//          - WEBAUTHN_CREDENTIAL_ATTESTATION                   :   6
//          - WEBAUTHN_ASSERTION                                :   5

// ***************************************************************************************************************************
// DISCLAIMER: All APIs, fields, and data types introduced as part of
// EXPERIMENTAL_WEBAUTHN_API_VERSION_8 are unstable, may not be documented and
// are subject to change at any time in the future without any notice.
// ***************************************************************************************************************************

#define EXPERIMENTAL_WEBAUTHN_API_VERSION_8 1008
// EXPERIMENTAL_WEBAUTHN_API_VERSION_8 : Delta From WEBAUTHN_API_VERSION_7
//      Data Structures and their sub versions:
//          - WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS    :   1008
//          (EXPERIMENTAL)
//          - WEBAUTHN_CREDENTIAL_DETAILS                       :   1003
//          (EXPERIMENTAL)
//          - WEBAUTHN_CREDENTIAL_ATTESTATION                   :   1007
//          (EXPERIMENTAL)
//          - WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS      :   1008
//          (EXPERIMENTAL)
//      APIs:
//          -
//          EXPERIMENTAL_WebAuthNIsUserVerifyingNativePlatformAuthenticatorAvailable
//          - EXPERIMENTAL_WebAuthNPluginGetAuthenticatorState
//          - EXPERIMENTAL_WebAuthNPluginAddAuthenticator
//          - EXPERIMENTAL_WebAuthNPluginFreeAddAuthenticatorResponse
//          - EXPERIMENTAL_WebAuthNPluginRemoveAuthenticator
//          - EXPERIMENTAL_WebAuthNPluginUpdateAuthenticatorDetails
//          - EXPERIMENTAL_WebAuthNPluginFreeUpdateAuthenticatorDetailsResponse
//          - EXPERIMENTAL_WebAuthNPluginAuthenticatorAddCredentials
//          - EXPERIMENTAL_WebAuthNPluginFreeAddCredentialsResponse
//          - EXPERIMENTAL_WebAuthNPluginAuthenticatorRemoveCredentials
//          - EXPERIMENTAL_WebAuthNPluginFreeRemoveCredentialsResponse
//          - EXPERIMENTAL_WebAuthNPluginAuthenticatorRemoveAllCredentials
//          - EXPERIMENTAL_WebAuthNPluginFreeRemoveAllCredentialsResponse
//          - EXPERIMENTAL_WebAuthNPluginAuthenticatorGetAllCredentials
//          - EXPERIMENTAL_WebAuthNPluginPerformUv
//          - EXPERIMENTAL_WebAuthNPluginFreePerformUvResponse
//          - EXPERIMENTAL_WebAuthNEncodeMakeCredentialResponse
//          - EXPERIMENTAL_WebAuthNDecodeMakeCredentialRequest
//          - EXPERIMENTAL_WebAuthNFreeDecodedMakeCredentialRequest
//          - EXPERIMENTAL_WebAuthNDecodeGetAssertionRequest
//          - EXPERIMENTAL_WebAuthNFreeDecodedGetAssertionRequest
//          - EXPERIMENTAL_WebAuthNEncodeGetAssertionResponse

#define WEBAUTHN_API_CURRENT_VERSION WEBAUTHN_API_VERSION_7

//+------------------------------------------------------------------------------------------
// Information about an RP Entity
//-------------------------------------------------------------------------------------------

#define WEBAUTHN_RP_ENTITY_INFORMATION_CURRENT_VERSION 1

#ifdef __midl
typedef[string] wchar_t* PWSTR;
typedef[string] wchar_t* const PCWSTR;
typedef unsigned char* PBYTE;
#endif

typedef struct _WEBAUTHN_RP_ENTITY_INFORMATION {
  // Version of this structure, to allow for modifications in the future.
  // This field is required and should be set to CURRENT_VERSION above.
  DWORD dwVersion;

  // Identifier for the RP. This field is required.
  PCWSTR pwszId;

  // Contains the friendly name of the Relying Party, such as "Acme
  // Corporation", "Widgets Inc" or "Awesome Site". This field is required.
  PCWSTR pwszName;

  // Optional URL pointing to RP's logo.
  PCWSTR pwszIcon;
} WEBAUTHN_RP_ENTITY_INFORMATION, *PWEBAUTHN_RP_ENTITY_INFORMATION;
typedef const WEBAUTHN_RP_ENTITY_INFORMATION* PCWEBAUTHN_RP_ENTITY_INFORMATION;

//+------------------------------------------------------------------------------------------
// Information about an User Entity
//-------------------------------------------------------------------------------------------
#define WEBAUTHN_MAX_USER_ID_LENGTH 64

#define WEBAUTHN_USER_ENTITY_INFORMATION_CURRENT_VERSION 1

typedef struct _WEBAUTHN_USER_ENTITY_INFORMATION {
  // Version of this structure, to allow for modifications in the future.
  // This field is required and should be set to CURRENT_VERSION above.
  DWORD dwVersion;

  // Identifier for the User. This field is required.
  DWORD cbId;

#ifdef __midl
  [size_is(cbId)]
#else
  _Field_size_bytes_(cbId)
#endif
      PBYTE pbId;

  // Contains a detailed name for this account, such as
  // "john.p.smith@example.com".
  PCWSTR pwszName;

  // Optional URL that can be used to retrieve an image containing the user's
  // current avatar, or a data URI that contains the image data.
  PCWSTR pwszIcon;

  // For User: Contains the friendly name associated with the user account by
  // the Relying Party, such as "John P. Smith".
  PCWSTR pwszDisplayName;
} WEBAUTHN_USER_ENTITY_INFORMATION, *PWEBAUTHN_USER_ENTITY_INFORMATION;
typedef const WEBAUTHN_USER_ENTITY_INFORMATION*
    PCWEBAUTHN_USER_ENTITY_INFORMATION;

#ifndef __midl

//+------------------------------------------------------------------------------------------
// Information about client data.
//-------------------------------------------------------------------------------------------

#define WEBAUTHN_HASH_ALGORITHM_SHA_256 L"SHA-256"
#define WEBAUTHN_HASH_ALGORITHM_SHA_384 L"SHA-384"
#define WEBAUTHN_HASH_ALGORITHM_SHA_512 L"SHA-512"

#define WEBAUTHN_CLIENT_DATA_CURRENT_VERSION 1

typedef struct _WEBAUTHN_CLIENT_DATA {
  // Version of this structure, to allow for modifications in the future.
  // This field is required and should be set to CURRENT_VERSION above.
  DWORD dwVersion;

  // Size of the pbClientDataJSON field.
  DWORD cbClientDataJSON;
  // UTF-8 encoded JSON serialization of the client data.
  _Field_size_bytes_(cbClientDataJSON) PBYTE pbClientDataJSON;

  // Hash algorithm ID used to hash the pbClientDataJSON field.
  LPCWSTR pwszHashAlgId;
} WEBAUTHN_CLIENT_DATA, *PWEBAUTHN_CLIENT_DATA;
typedef const WEBAUTHN_CLIENT_DATA* PCWEBAUTHN_CLIENT_DATA;

#endif  //__midl

//+------------------------------------------------------------------------------------------
// Information about credential parameters.
//-------------------------------------------------------------------------------------------

#define WEBAUTHN_CREDENTIAL_TYPE_PUBLIC_KEY L"public-key"

#define WEBAUTHN_COSE_ALGORITHM_ECDSA_P256_WITH_SHA256 -7
#define WEBAUTHN_COSE_ALGORITHM_ECDSA_P384_WITH_SHA384 -35
#define WEBAUTHN_COSE_ALGORITHM_ECDSA_P521_WITH_SHA512 -36

#define WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA256 -257
#define WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA384 -258
#define WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA512 -259

#define WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA256 -37
#define WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA384 -38
#define WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA512 -39

#define WEBAUTHN_COSE_CREDENTIAL_PARAMETER_CURRENT_VERSION 1

typedef struct _WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
  // Version of this structure, to allow for modifications in the future.
  DWORD dwVersion;

// Well-known credential type specifying a credential to create.
#ifdef __midl
  PCWSTR pwszCredentialType;
#else
  LPCWSTR pwszCredentialType;
#endif

  // Well-known COSE algorithm specifying the algorithm to use for the
  // credential.
  LONG lAlg;
} WEBAUTHN_COSE_CREDENTIAL_PARAMETER, *PWEBAUTHN_COSE_CREDENTIAL_PARAMETER;
typedef const WEBAUTHN_COSE_CREDENTIAL_PARAMETER*
    PCWEBAUTHN_COSE_CREDENTIAL_PARAMETER;

typedef struct _WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
  DWORD cCredentialParameters;
#ifdef __midl
  [size_is(cCredentialParameters)]
#else
  _Field_size_(cCredentialParameters)
#endif
      PWEBAUTHN_COSE_CREDENTIAL_PARAMETER pCredentialParameters;
} WEBAUTHN_COSE_CREDENTIAL_PARAMETERS, *PWEBAUTHN_COSE_CREDENTIAL_PARAMETERS;
typedef const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS*
    PCWEBAUTHN_COSE_CREDENTIAL_PARAMETERS;

//+------------------------------------------------------------------------------------------
// Information about credential.
//-------------------------------------------------------------------------------------------
#define WEBAUTHN_CREDENTIAL_CURRENT_VERSION 1

typedef struct _WEBAUTHN_CREDENTIAL {
  // Version of this structure, to allow for modifications in the future.
  DWORD dwVersion;

  // Unique ID for this particular credential.
  DWORD cbId;
#ifdef __midl
  [size_is(cbId)]
#else
  _Field_size_bytes_(cbId) PBYTE pbId;
#endif

// Well-known credential type specifying what this particular credential is.
#ifdef __midl
      PWSTR pwszCredentialType;
#else
  PCWSTR pwszCredentialType;
#endif

} WEBAUTHN_CREDENTIAL, *PWEBAUTHN_CREDENTIAL;
typedef const WEBAUTHN_CREDENTIAL* PCWEBAUTHN_CREDENTIAL;

typedef struct _WEBAUTHN_CREDENTIALS {
  DWORD cCredentials;
#ifdef __midl
  [size_is(cCredentials)]
#else
  _Field_size_(cCredentials)
#endif
      PWEBAUTHN_CREDENTIAL pCredentials;
} WEBAUTHN_CREDENTIALS, *PWEBAUTHN_CREDENTIALS;
typedef const WEBAUTHN_CREDENTIALS* PCWEBAUTHN_CREDENTIALS;

//+------------------------------------------------------------------------------------------
// Information about credential with extra information, such as, dwTransports
//-------------------------------------------------------------------------------------------

#define WEBAUTHN_CTAP_TRANSPORT_USB 0x00000001
#define WEBAUTHN_CTAP_TRANSPORT_NFC 0x00000002
#define WEBAUTHN_CTAP_TRANSPORT_BLE 0x00000004
#define WEBAUTHN_CTAP_TRANSPORT_TEST 0x00000008
#define WEBAUTHN_CTAP_TRANSPORT_INTERNAL 0x00000010
#define WEBAUTHN_CTAP_TRANSPORT_HYBRID 0x00000020
#define WEBAUTHN_CTAP_TRANSPORT_FLAGS_MASK 0x0000003F

#define WEBAUTHN_CREDENTIAL_EX_CURRENT_VERSION 1

typedef struct _WEBAUTHN_CREDENTIAL_EX {
  // Version of this structure, to allow for modifications in the future.
  DWORD dwVersion;

  // Size of pbID.
  DWORD cbId;
#ifdef __midl
  [size_is(cbId)]
#else
  _Field_size_bytes_(cbId)
#endif
      PBYTE pbId;

// Well-known credential type specifying what this particular credential is.
#ifdef __midl
  PCWSTR pwszCredentialType;
#else
  LPCWSTR pwszCredentialType;
#endif

  // Transports. 0 implies no transport restrictions.
  DWORD dwTransports;
} WEBAUTHN_CREDENTIAL_EX, *PWEBAUTHN_CREDENTIAL_EX;
typedef const WEBAUTHN_CREDENTIAL_EX* PCWEBAUTHN_CREDENTIAL_EX;

//+------------------------------------------------------------------------------------------
// Information about credential list with extra information
//-------------------------------------------------------------------------------------------

typedef struct _WEBAUTHN_CREDENTIAL_LIST {
  DWORD cCredentials;
#ifdef __midl
  [size_is(cCredentials)]
#else
  _Field_size_(cCredentials)
#endif
      PWEBAUTHN_CREDENTIAL_EX* ppCredentials;
} WEBAUTHN_CREDENTIAL_LIST, *PWEBAUTHN_CREDENTIAL_LIST;
typedef const WEBAUTHN_CREDENTIAL_LIST* PCWEBAUTHN_CREDENTIAL_LIST;

#ifndef __midl

//+------------------------------------------------------------------------------------------
// Information about linked devices
//-------------------------------------------------------------------------------------------

#define CTAPCBOR_HYBRID_STORAGE_LINKED_DATA_VERSION_1 1
#define CTAPCBOR_HYBRID_STORAGE_LINKED_DATA_CURRENT_VERSION \
  CTAPCBOR_HYBRID_STORAGE_LINKED_DATA_VERSION_1

typedef struct _CTAPCBOR_HYBRID_STORAGE_LINKED_DATA {
  // Version
  DWORD dwVersion;

  // Contact Id
  DWORD cbContactId;
  _Field_size_bytes_(cbContactId) PBYTE pbContactId;

  // Link Id
  DWORD cbLinkId;
  _Field_size_bytes_(cbLinkId) PBYTE pbLinkId;

  // Link secret
  DWORD cbLinkSecret;
  _Field_size_bytes_(cbLinkSecret) PBYTE pbLinkSecret;

  // Authenticator Public Key
  DWORD cbPublicKey;
  _Field_size_bytes_(cbPublicKey) PBYTE pbPublicKey;

  // Authenticator Name
  PCWSTR pwszAuthenticatorName;

  // Tunnel server domain
  WORD wEncodedTunnelServerDomain;
} CTAPCBOR_HYBRID_STORAGE_LINKED_DATA, *PCTAPCBOR_HYBRID_STORAGE_LINKED_DATA;
typedef const CTAPCBOR_HYBRID_STORAGE_LINKED_DATA*
    PCCTAPCBOR_HYBRID_STORAGE_LINKED_DATA;

#endif  //__midl
//+------------------------------------------------------------------------------------------
// Credential Information for WebAuthNGetPlatformCredentialList API
//-------------------------------------------------------------------------------------------

#define WEBAUTHN_CREDENTIAL_DETAILS_VERSION_1 1
#define WEBAUTHN_CREDENTIAL_DETAILS_VERSION_2 2
#define EXPERIMENTAL_WEBAUTHN_CREDENTIAL_DETAILS_VERSION_3 1003
#define WEBAUTHN_CREDENTIAL_DETAILS_CURRENT_VERSION \
  WEBAUTHN_CREDENTIAL_DETAILS_VERSION_2

typedef struct _WEBAUTHN_CREDENTIAL_DETAILS {
  // Version of this structure, to allow for modifications in the future.
  DWORD dwVersion;

  // Size of pbCredentialID.
  DWORD cbCredentialID;

#ifdef __midl
  [size_is(cbCredentialID)]
#else
  _Field_size_bytes_(cbCredentialID)
#endif
      PBYTE pbCredentialID;

// RP Info
#ifdef __midl
  [unique]
#endif
      PWEBAUTHN_RP_ENTITY_INFORMATION pRpInformation;

// User Info
#ifdef __midl
  [unique]
#endif
      PWEBAUTHN_USER_ENTITY_INFORMATION pUserInformation;

  // Removable or not.
  BOOL bRemovable;

  //
  // The following fields have been added in
  // WEBAUTHN_CREDENTIAL_DETAILS_VERSION_2
  //

  // Backed Up or not.
  BOOL bBackedUp;

  //
  // The following fields have been added in
  // EXPERIMENTAL_WEBAUTHN_CREDENTIAL_DETAILS_VERSION_3
  //
  PCWSTR EXPERIMENTAL_pwszAuthenticatorName;

  // The logo is expected to be in the svg format
  DWORD EXPERIMENTAL_cbAuthenticatorLogo;

#ifdef __midl
  [size_is(EXPERIMENTAL_cbAuthenticatorLogo)]
#else
  _Field_size_bytes_(EXPERIMENTAL_cbAuthenticatorLogo)
#endif
      PBYTE EXPERIMENTAL_pbAuthenticatorLogo;

  // ThirdPartyPayment Credential or not.
  BOOL EXPERIMENTAL_bThirdPartyPayment;

} WEBAUTHN_CREDENTIAL_DETAILS, *PWEBAUTHN_CREDENTIAL_DETAILS;
typedef const WEBAUTHN_CREDENTIAL_DETAILS* PCWEBAUTHN_CREDENTIAL_DETAILS;

typedef struct _WEBAUTHN_CREDENTIAL_DETAILS_LIST {
  DWORD cCredentialDetails;
#ifdef __midl
  [size_is(cCredentialDetails)]
#else
  _Field_size_(cCredentialDetails)
#endif
      PWEBAUTHN_CREDENTIAL_DETAILS* ppCredentialDetails;
} WEBAUTHN_CREDENTIAL_DETAILS_LIST, *PWEBAUTHN_CREDENTIAL_DETAILS_LIST;
typedef const WEBAUTHN_CREDENTIAL_DETAILS_LIST*
    PCWEBAUTHN_CREDENTIAL_DETAILS_LIST;

#define WEBAUTHN_GET_CREDENTIALS_OPTIONS_VERSION_1 1
#define WEBAUTHN_GET_CREDENTIALS_OPTIONS_CURRENT_VERSION \
  WEBAUTHN_GET_CREDENTIALS_OPTIONS_VERSION_1

typedef struct _WEBAUTHN_GET_CREDENTIALS_OPTIONS {
  // Version of this structure, to allow for modifications in the future.
  DWORD dwVersion;

  // Optional.
  LPCWSTR pwszRpId;

  // Optional. BrowserInPrivate Mode. Defaulting to FALSE.
  BOOL bBrowserInPrivateMode;
} WEBAUTHN_GET_CREDENTIALS_OPTIONS, *PWEBAUTHN_GET_CREDENTIALS_OPTIONS;
typedef const WEBAUTHN_GET_CREDENTIALS_OPTIONS*
    PCWEBAUTHN_GET_CREDENTIALS_OPTIONS;

//+------------------------------------------------------------------------------------------
// PRF values.
//-------------------------------------------------------------------------------------------

#define WEBAUTHN_CTAP_ONE_HMAC_SECRET_LENGTH 32

// SALT values below by default are converted into RAW Hmac-Secret values as per
// PRF extension.
//   - SHA-256(UTF8Encode("WebAuthn PRF") || 0x00 || Value)
//
// Set WEBAUTHN_CTAP_HMAC_SECRET_VALUES_FLAG in dwFlags in
// WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS,
//   if caller wants to provide RAW Hmac-Secret SALT values directly. In that
//   case, values if provided MUST be of WEBAUTHN_CTAP_ONE_HMAC_SECRET_LENGTH
//   size.

typedef struct _WEBAUTHN_HMAC_SECRET_SALT {
  // Size of pbFirst.
  DWORD cbFirst;
#ifdef __midl
  [size_is(cbFirst)]
#else
  _Field_size_bytes_(cbFirst)
#endif
      PBYTE pbFirst;  // Required

  // Size of pbSecond.
  DWORD cbSecond;
#ifdef __midl
  [size_is(cbSecond)]
#else
  _Field_size_bytes_(cbSecond)
#endif
      PBYTE pbSecond;
} WEBAUTHN_HMAC_SECRET_SALT, *PWEBAUTHN_HMAC_SECRET_SALT;
typedef const WEBAUTHN_HMAC_SECRET_SALT* PCWEBAUTHN_HMAC_SECRET_SALT;

#ifndef __midl

typedef struct _WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT {
  // Size of pbCredID.
  DWORD cbCredID;
  _Field_size_bytes_(cbCredID) PBYTE pbCredID;  // Required

  // PRF Values for above credential
  PWEBAUTHN_HMAC_SECRET_SALT pHmacSecretSalt;  // Required
} WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT, *PWEBAUTHN_CRED_WITH_HMAC_SECRET_SALT;
typedef const WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT*
    PCWEBAUTHN_CRED_WITH_HMAC_SECRET_SALT;

typedef struct _WEBAUTHN_HMAC_SECRET_SALT_VALUES {
  PWEBAUTHN_HMAC_SECRET_SALT pGlobalHmacSalt;

  DWORD cCredWithHmacSecretSaltList;
  _Field_size_(cCredWithHmacSecretSaltList)
      PWEBAUTHN_CRED_WITH_HMAC_SECRET_SALT pCredWithHmacSecretSaltList;
} WEBAUTHN_HMAC_SECRET_SALT_VALUES, *PWEBAUTHN_HMAC_SECRET_SALT_VALUES;
typedef const WEBAUTHN_HMAC_SECRET_SALT_VALUES*
    PCWEBAUTHN_HMAC_SECRET_SALT_VALUES;

//+------------------------------------------------------------------------------------------
// Hmac-Secret extension
//-------------------------------------------------------------------------------------------

#define WEBAUTHN_EXTENSIONS_IDENTIFIER_HMAC_SECRET L"hmac-secret"
// Below type definitions is for WEBAUTHN_EXTENSIONS_IDENTIFIER_HMAC_SECRET
// MakeCredential Input Type:   BOOL.
//      - pvExtension must point to a BOOL with the value TRUE.
//      - cbExtension must contain the sizeof(BOOL).
// MakeCredential Output Type:  BOOL.
//      - pvExtension will point to a BOOL with the value TRUE if credential
//        was successfully created with HMAC_SECRET.
//      - cbExtension will contain the sizeof(BOOL).
// GetAssertion Input Type:     Not Supported
// GetAssertion Output Type:    Not Supported

//+------------------------------------------------------------------------------------------
//  credProtect  extension
//-------------------------------------------------------------------------------------------

#define WEBAUTHN_USER_VERIFICATION_ANY 0
#define WEBAUTHN_USER_VERIFICATION_OPTIONAL 1
#define WEBAUTHN_USER_VERIFICATION_OPTIONAL_WITH_CREDENTIAL_ID_LIST 2
#define WEBAUTHN_USER_VERIFICATION_REQUIRED 3

typedef struct _WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
  // One of the above WEBAUTHN_USER_VERIFICATION_* values
  DWORD dwCredProtect;
  // Set the following to TRUE to require authenticator support for the
  // credProtect extension
  BOOL bRequireCredProtect;
} WEBAUTHN_CRED_PROTECT_EXTENSION_IN, *PWEBAUTHN_CRED_PROTECT_EXTENSION_IN;
typedef const WEBAUTHN_CRED_PROTECT_EXTENSION_IN*
    PCWEBAUTHN_CRED_PROTECT_EXTENSION_IN;

#define WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_PROTECT L"credProtect"
// Below type definitions is for WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_PROTECT
// MakeCredential Input Type:   WEBAUTHN_CRED_PROTECT_EXTENSION_IN.
//      - pvExtension must point to a WEBAUTHN_CRED_PROTECT_EXTENSION_IN struct
//      - cbExtension will contain the
//      sizeof(WEBAUTHN_CRED_PROTECT_EXTENSION_IN).
// MakeCredential Output Type:  DWORD.
//      - pvExtension will point to a DWORD with one of the above
//      WEBAUTHN_USER_VERIFICATION_* values
//        if credential was successfully created with CRED_PROTECT.
//      - cbExtension will contain the sizeof(DWORD).
// GetAssertion Input Type:     Not Supported
// GetAssertion Output Type:    Not Supported

//+------------------------------------------------------------------------------------------
//  credBlob  extension
//-------------------------------------------------------------------------------------------

typedef struct _WEBAUTHN_CRED_BLOB_EXTENSION {
  // Size of pbCredBlob.
  DWORD cbCredBlob;
  _Field_size_bytes_(cbCredBlob) PBYTE pbCredBlob;
} WEBAUTHN_CRED_BLOB_EXTENSION, *PWEBAUTHN_CRED_BLOB_EXTENSION;
typedef const WEBAUTHN_CRED_BLOB_EXTENSION* PCWEBAUTHN_CRED_BLOB_EXTENSION;

#define WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_BLOB L"credBlob"
// Below type definitions is for WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_BLOB
// MakeCredential Input Type:   WEBAUTHN_CRED_BLOB_EXTENSION.
//      - pvExtension must point to a WEBAUTHN_CRED_BLOB_EXTENSION struct
//      - cbExtension must contain the sizeof(WEBAUTHN_CRED_BLOB_EXTENSION).
// MakeCredential Output Type:  BOOL.
//      - pvExtension will point to a BOOL with the value TRUE if credBlob was
//      successfully created
//      - cbExtension will contain the sizeof(BOOL).
// GetAssertion Input Type:     BOOL.
//      - pvExtension must point to a BOOL with the value TRUE to request the
//      credBlob.
//      - cbExtension must contain the sizeof(BOOL).
// GetAssertion Output Type:    WEBAUTHN_CRED_BLOB_EXTENSION.
//      - pvExtension will point to a WEBAUTHN_CRED_BLOB_EXTENSION struct if the
//      authenticator
//        returns the credBlob in the signed extensions
//      - cbExtension will contain the sizeof(WEBAUTHN_CRED_BLOB_EXTENSION).

//+------------------------------------------------------------------------------------------
//  minPinLength  extension
//-------------------------------------------------------------------------------------------

#define WEBAUTHN_EXTENSIONS_IDENTIFIER_MIN_PIN_LENGTH L"minPinLength"
// Below type definitions is for WEBAUTHN_EXTENSIONS_IDENTIFIER_MIN_PIN_LENGTH
// MakeCredential Input Type:   BOOL.
//      - pvExtension must point to a BOOL with the value TRUE to request the
//      minPinLength.
//      - cbExtension must contain the sizeof(BOOL).
// MakeCredential Output Type:  DWORD.
//      - pvExtension will point to a DWORD with the minimum pin length if
//      returned by the authenticator
//      - cbExtension will contain the sizeof(DWORD).
// GetAssertion Input Type:     Not Supported
// GetAssertion Output Type:    Not Supported

#endif  //__midl

//+------------------------------------------------------------------------------------------
// Information about Extensions.
//-------------------------------------------------------------------------------------------
typedef struct _WEBAUTHN_EXTENSION {
#ifdef __midl
  PWSTR pwszExtensionIdentifier;
#else
  LPCWSTR pwszExtensionIdentifier;
#endif

  DWORD cbExtension;
#ifdef __midl
  [size_is(cbExtension)] PBYTE pvExtension;
#else
  PVOID pvExtension;
#endif
} WEBAUTHN_EXTENSION, *PWEBAUTHN_EXTENSION;
typedef const WEBAUTHN_EXTENSION* PCWEBAUTHN_EXTENSION;

typedef struct _WEBAUTHN_EXTENSIONS {
  DWORD cExtensions;
#ifdef __midl
  [size_is(cExtensions)]
#else
  _Field_size_(cExtensions)
#endif
      PWEBAUTHN_EXTENSION pExtensions;
} WEBAUTHN_EXTENSIONS, *PWEBAUTHN_EXTENSIONS;
typedef const WEBAUTHN_EXTENSIONS* PCWEBAUTHN_EXTENSIONS;

#ifndef __midl

//+------------------------------------------------------------------------------------------
// Options.
//-------------------------------------------------------------------------------------------

#define WEBAUTHN_AUTHENTICATOR_ATTACHMENT_ANY 0
#define WEBAUTHN_AUTHENTICATOR_ATTACHMENT_PLATFORM 1
#define WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM 2
#define WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM_U2F_V2 3

#define WEBAUTHN_USER_VERIFICATION_REQUIREMENT_ANY 0
#define WEBAUTHN_USER_VERIFICATION_REQUIREMENT_REQUIRED 1
#define WEBAUTHN_USER_VERIFICATION_REQUIREMENT_PREFERRED 2
#define WEBAUTHN_USER_VERIFICATION_REQUIREMENT_DISCOURAGED 3

#define WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_ANY 0
#define WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_NONE 1
#define WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_INDIRECT 2
#define WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_DIRECT 3

#define WEBAUTHN_ENTERPRISE_ATTESTATION_NONE 0
#define WEBAUTHN_ENTERPRISE_ATTESTATION_VENDOR_FACILITATED 1
#define WEBAUTHN_ENTERPRISE_ATTESTATION_PLATFORM_MANAGED 2

#define WEBAUTHN_LARGE_BLOB_SUPPORT_NONE 0
#define WEBAUTHN_LARGE_BLOB_SUPPORT_REQUIRED 1
#define WEBAUTHN_LARGE_BLOB_SUPPORT_PREFERRED 2

#define WEBAUTHN_CREDENTIAL_HINT_SECURITY_KEY L"security-key"
#define WEBAUTHN_CREDENTIAL_HINT_CLIENT_DEVICE L"client-device"
#define WEBAUTHN_CREDENTIAL_HINT_HYBRID L"hybrid"

#define WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_1 1
#define WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_2 2
#define WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_3 3
#define WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_4 4
#define WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_5 5
#define WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_6 6
#define WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_7 7
#define EXPERIMENTAL_WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_8 \
  1008
#define WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_CURRENT_VERSION \
  WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_7

typedef struct _WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
  // Version of this structure, to allow for modifications in the future.
  DWORD dwVersion;

  // Time that the operation is expected to complete within.
  // This is used as guidance, and can be overridden by the platform.
  DWORD dwTimeoutMilliseconds;

  // Credentials used for exclusion.
  WEBAUTHN_CREDENTIALS CredentialList;

  // Optional extensions to parse when performing the operation.
  WEBAUTHN_EXTENSIONS Extensions;

  // Optional. Platform vs Cross-Platform Authenticators.
  DWORD dwAuthenticatorAttachment;

  // Optional. Require key to be resident or not. Defaulting to FALSE.
  BOOL bRequireResidentKey;

  // User Verification Requirement.
  DWORD dwUserVerificationRequirement;

  // Attestation Conveyance Preference.
  DWORD dwAttestationConveyancePreference;

  // Reserved for future Use
  DWORD dwFlags;

  //
  // The following fields have been added in
  // WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_2
  //

  // Cancellation Id - Optional - See WebAuthNGetCancellationId
  GUID* pCancellationId;

  //
  // The following fields have been added in
  // WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_3
  //

  // Exclude Credential List. If present, "CredentialList" will be ignored.
  PWEBAUTHN_CREDENTIAL_LIST pExcludeCredentialList;

  //
  // The following fields have been added in
  // WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_4
  //

  // Enterprise Attestation
  DWORD dwEnterpriseAttestation;

  // Large Blob Support: none, required or preferred
  //
  // NTE_INVALID_PARAMETER when large blob required or preferred and
  //   bRequireResidentKey isn't set to TRUE
  DWORD dwLargeBlobSupport;

  // Optional. Prefer key to be resident. Defaulting to FALSE. When TRUE,
  // overrides the above bRequireResidentKey.
  BOOL bPreferResidentKey;

  //
  // The following fields have been added in
  // WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_5
  //

  // Optional. BrowserInPrivate Mode. Defaulting to FALSE.
  BOOL bBrowserInPrivateMode;

  //
  // The following fields have been added in
  // WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_6
  //

  // Enable PRF
  BOOL bEnablePrf;

  //
  // The following fields have been added in
  // WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_7
  //

  // Optional. Linked Device Connection Info.
  PCTAPCBOR_HYBRID_STORAGE_LINKED_DATA pLinkedDevice;

  // Size of pbJsonExt
  DWORD cbJsonExt;
  _Field_size_bytes_(cbJsonExt) PBYTE pbJsonExt;

  //
  // The following fields have been added in
  // EXPERIMENTAL_WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_8
  //

  // PRF extension "eval" values which will be converted into HMAC-SECRET values
  // according to WebAuthn Spec. Set WEBAUTHN_CTAP_HMAC_SECRET_VALUES_FLAG in
  // dwFlags above, if caller wants to provide RAW Hmac-Secret SALT values
  // directly. In that case, values provided MUST be of
  // WEBAUTHN_CTAP_ONE_HMAC_SECRET_LENGTH size.
  PWEBAUTHN_HMAC_SECRET_SALT EXPERIMENTAL_pPRFGlobalEval;

  // PublicKeyCredentialHints (https://w3c.github.io/webauthn/#enum-hints)
  DWORD EXPERIMENTAL_cCredentialHints;

#ifdef __midl
  [size_is(
      EXPERIMENTAL_cCredentialHints)] PCWSTR* EXPERIMENTAL_ppwszCredentialHints;
#else
  _Field_size_(
      EXPERIMENTAL_cCredentialHints) LPCWSTR* EXPERIMENTAL_ppwszCredentialHints;
#endif

  // Enable ThirdPartyPayment
  BOOL EXPERIMENTAL_bThirdPartyPayment;

} WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS,
    *PWEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS;
typedef const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS*
    PCWEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS;

#define WEBAUTHN_CRED_LARGE_BLOB_OPERATION_NONE 0
#define WEBAUTHN_CRED_LARGE_BLOB_OPERATION_GET 1
#define WEBAUTHN_CRED_LARGE_BLOB_OPERATION_SET 2
#define WEBAUTHN_CRED_LARGE_BLOB_OPERATION_DELETE 3

#define WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_1 1
#define WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_2 2
#define WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_3 3
#define WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_4 4
#define WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_5 5
#define WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_6 6
#define WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_7 7
#define EXPERIMENTAL_WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_8 1008
#define WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_CURRENT_VERSION \
  WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_7

/*
    Information about flags.
*/

#define WEBAUTHN_AUTHENTICATOR_HMAC_SECRET_VALUES_FLAG 0x00100000

typedef struct _WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
  // Version of this structure, to allow for modifications in the future.
  DWORD dwVersion;

  // Time that the operation is expected to complete within.
  // This is used as guidance, and can be overridden by the platform.
  DWORD dwTimeoutMilliseconds;

  // Allowed Credentials List.
  WEBAUTHN_CREDENTIALS CredentialList;

  // Optional extensions to parse when performing the operation.
  WEBAUTHN_EXTENSIONS Extensions;

  // Optional. Platform vs Cross-Platform Authenticators.
  DWORD dwAuthenticatorAttachment;

  // User Verification Requirement.
  DWORD dwUserVerificationRequirement;

  // Flags
  DWORD dwFlags;

  //
  // The following fields have been added in
  // WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_2
  //

  // Optional identifier for the U2F AppId. Converted to UTF8 before being
  // hashed. Not lower cased.
  PCWSTR pwszU2fAppId;

  // If the following is non-NULL, then, set to TRUE if the above pwszU2fAppid
  // was used instead of PCWSTR pwszRpId;
  BOOL* pbU2fAppId;

  //
  // The following fields have been added in
  // WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_3
  //

  // Cancellation Id - Optional - See WebAuthNGetCancellationId
  GUID* pCancellationId;

  //
  // The following fields have been added in
  // WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_4
  //

  // Allow Credential List. If present, "CredentialList" will be ignored.
  PWEBAUTHN_CREDENTIAL_LIST pAllowCredentialList;

  //
  // The following fields have been added in
  // WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_5
  //

  DWORD dwCredLargeBlobOperation;

  // Size of pbCredLargeBlob
  DWORD cbCredLargeBlob;
  _Field_size_bytes_(cbCredLargeBlob) PBYTE pbCredLargeBlob;

  //
  // The following fields have been added in
  // WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_6
  //

  // PRF values which will be converted into HMAC-SECRET values according to
  // WebAuthn Spec.
  PWEBAUTHN_HMAC_SECRET_SALT_VALUES pHmacSecretSaltValues;

  // Optional. BrowserInPrivate Mode. Defaulting to FALSE.
  BOOL bBrowserInPrivateMode;

  //
  // The following fields have been added in
  // WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_7
  //

  // Optional. Linked Device Connection Info.
  PCTAPCBOR_HYBRID_STORAGE_LINKED_DATA pLinkedDevice;

  // Optional. Allowlist MUST contain 1 credential applicable for Hybrid
  // transport.
  BOOL bAutoFill;

  // Size of pbJsonExt
  DWORD cbJsonExt;
  _Field_size_bytes_(cbJsonExt) PBYTE pbJsonExt;

  //
  // The following fields have been added in
  // EXPERIMENTAL_WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_8
  //

  // PublicKeyCredentialHints (https://w3c.github.io/webauthn/#enum-hints)
  DWORD EXPERIMENTAL_cCredentialHints;

#ifdef __midl
  [size_is(
      EXPERIMENTAL_cCredentialHints)] PCWSTR* EXPERIMENTAL_ppwszCredentialHints;
#else
  _Field_size_(
      EXPERIMENTAL_cCredentialHints) LPCWSTR* EXPERIMENTAL_ppwszCredentialHints;
#endif

} WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS,
    *PWEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS;
typedef const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS*
    PCWEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS;

//+------------------------------------------------------------------------------------------
// Attestation Info.
//
//-------------------------------------------------------------------------------------------
#define WEBAUTHN_ATTESTATION_DECODE_NONE 0
#define WEBAUTHN_ATTESTATION_DECODE_COMMON 1
// WEBAUTHN_ATTESTATION_DECODE_COMMON supports format types
//  L"packed"
//  L"fido-u2f"

#define WEBAUTHN_ATTESTATION_VER_TPM_2_0 L"2.0"

typedef struct _WEBAUTHN_X5C {
  // Length of X.509 encoded certificate
  DWORD cbData;
  // X.509 encoded certificate bytes
  _Field_size_bytes_(cbData) PBYTE pbData;
} WEBAUTHN_X5C, *PWEBAUTHN_X5C;

// Supports either Self or Full Basic Attestation

// Note, new fields will be added to the following data structure to
// support additional attestation format types, such as, TPM.
// When fields are added, the dwVersion will be incremented.
//
// Therefore, your code must make the following check:
//  "if (dwVersion >= WEBAUTHN_COMMON_ATTESTATION_CURRENT_VERSION)"

#define WEBAUTHN_COMMON_ATTESTATION_CURRENT_VERSION 1

typedef struct _WEBAUTHN_COMMON_ATTESTATION {
  // Version of this structure, to allow for modifications in the future.
  DWORD dwVersion;

  // Hash and Padding Algorithm
  //
  // The following won't be set for "fido-u2f" which assumes "ES256".
  PCWSTR pwszAlg;
  LONG lAlg;  // COSE algorithm

  // Signature that was generated for this attestation.
  DWORD cbSignature;
  _Field_size_bytes_(cbSignature) PBYTE pbSignature;

  // Following is set for Full Basic Attestation. If not, set then, this is Self
  // Attestation. Array of X.509 DER encoded certificates. The first certificate
  // is the signer, leaf certificate.
  DWORD cX5c;
  _Field_size_(cX5c) PWEBAUTHN_X5C pX5c;

  // Following are also set for tpm
  PCWSTR pwszVer;  // L"2.0"
  DWORD cbCertInfo;
  _Field_size_bytes_(cbCertInfo) PBYTE pbCertInfo;
  DWORD cbPubArea;
  _Field_size_bytes_(cbPubArea) PBYTE pbPubArea;
} WEBAUTHN_COMMON_ATTESTATION, *PWEBAUTHN_COMMON_ATTESTATION;
typedef const WEBAUTHN_COMMON_ATTESTATION* PCWEBAUTHN_COMMON_ATTESTATION;

#endif  //__midl

#define WEBAUTHN_ATTESTATION_TYPE_PACKED L"packed"
#define WEBAUTHN_ATTESTATION_TYPE_U2F L"fido-u2f"
#define WEBAUTHN_ATTESTATION_TYPE_TPM L"tpm"
#define WEBAUTHN_ATTESTATION_TYPE_NONE L"none"

#define WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_1 1
#define WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_2 2
#define WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_3 3
#define WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_4 4
#define WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_5 5
#define WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_6 6
#define EXPERIMENTAL_WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_7 1007
#define WEBAUTHN_CREDENTIAL_ATTESTATION_CURRENT_VERSION \
  WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_6

typedef struct _WEBAUTHN_CREDENTIAL_ATTESTATION {
  // Version of this structure, to allow for modifications in the future.
  DWORD dwVersion;

// Attestation format type
#ifdef __midl
  PWSTR pwszFormatType;
#else
  PCWSTR pwszFormatType;
#endif

  // Size of cbAuthenticatorData.
  DWORD cbAuthenticatorData;
// Authenticator data that was created for this credential.
#ifdef __midl
  [size_is(cbAuthenticatorData)]
#else
  _Field_size_bytes_(cbAuthenticatorData)
#endif
      PBYTE pbAuthenticatorData;

  // Size of CBOR encoded attestation information
  // 0 => encoded as CBOR null value.
  DWORD cbAttestation;
// Encoded CBOR attestation information
#ifdef __midl
  [size_is(cbAttestation)]
#else
  _Field_size_bytes_(cbAttestation)
#endif
      PBYTE pbAttestation;

  DWORD dwAttestationDecodeType;
// Following depends on the dwAttestationDecodeType
//  WEBAUTHN_ATTESTATION_DECODE_NONE
//      NULL - not able to decode the CBOR attestation information
//  WEBAUTHN_ATTESTATION_DECODE_COMMON
//      PWEBAUTHN_COMMON_ATTESTATION;
#ifdef __midl
  PBYTE pvAttestationDecode;
#else
  PVOID pvAttestationDecode;
#endif

  // The CBOR encoded Attestation Object to be returned to the RP.
  DWORD cbAttestationObject;
#ifdef __midl
  [size_is(cbAttestationObject)]
#else
  _Field_size_bytes_(cbAttestationObject)
#endif
      PBYTE pbAttestationObject;

  // The CredentialId bytes extracted from the Authenticator Data.
  // Used by Edge to return to the RP.
  DWORD cbCredentialId;
#ifdef __midl
  [size_is(cbCredentialId)]
#else
  _Field_size_bytes_(cbCredentialId)
#endif
      PBYTE pbCredentialId;

  //
  // Following fields have been added in
  // WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_2
  //

  WEBAUTHN_EXTENSIONS Extensions;

  //
  // Following fields have been added in
  // WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_3
  //

  // One of the WEBAUTHN_CTAP_TRANSPORT_* bits will be set corresponding to
  // the transport that was used.
  DWORD dwUsedTransport;

  //
  // Following fields have been added in
  // WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_4
  //

  BOOL bEpAtt;
  BOOL bLargeBlobSupported;
  BOOL bResidentKey;

  //
  // Following fields have been added in
  // WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_5
  //

  BOOL bPrfEnabled;

  //
  // Following fields have been added in
  // WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_6
  //

  DWORD cbUnsignedExtensionOutputs;
#ifdef __midl
  [size_is(cbUnsignedExtensionOutputs)]
#else
  _Field_size_bytes_(cbUnsignedExtensionOutputs)
#endif
      PBYTE pbUnsignedExtensionOutputs;

  //
  // Following fields have been added in
  // EXPERIMENTAL_WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_7
  //

  PWEBAUTHN_HMAC_SECRET_SALT EXPERIMENTAL_pHmacSecret;

  // ThirdPartyPayment Credential or not.
  BOOL EXPERIMENTAL_bThirdPartyPayment;

} WEBAUTHN_CREDENTIAL_ATTESTATION, *PWEBAUTHN_CREDENTIAL_ATTESTATION;
typedef const WEBAUTHN_CREDENTIAL_ATTESTATION*
    PCWEBAUTHN_CREDENTIAL_ATTESTATION;

//+------------------------------------------------------------------------------------------
// authenticatorGetAssertion output.
//-------------------------------------------------------------------------------------------

#define WEBAUTHN_CRED_LARGE_BLOB_STATUS_NONE 0
#define WEBAUTHN_CRED_LARGE_BLOB_STATUS_SUCCESS 1
#define WEBAUTHN_CRED_LARGE_BLOB_STATUS_NOT_SUPPORTED 2
#define WEBAUTHN_CRED_LARGE_BLOB_STATUS_INVALID_DATA 3
#define WEBAUTHN_CRED_LARGE_BLOB_STATUS_INVALID_PARAMETER 4
#define WEBAUTHN_CRED_LARGE_BLOB_STATUS_NOT_FOUND 5
#define WEBAUTHN_CRED_LARGE_BLOB_STATUS_MULTIPLE_CREDENTIALS 6
#define WEBAUTHN_CRED_LARGE_BLOB_STATUS_LACK_OF_SPACE 7
#define WEBAUTHN_CRED_LARGE_BLOB_STATUS_PLATFORM_ERROR 8
#define WEBAUTHN_CRED_LARGE_BLOB_STATUS_AUTHENTICATOR_ERROR 9

#define WEBAUTHN_ASSERTION_VERSION_1 1
#define WEBAUTHN_ASSERTION_VERSION_2 2
#define WEBAUTHN_ASSERTION_VERSION_3 3
#define WEBAUTHN_ASSERTION_VERSION_4 4
#define WEBAUTHN_ASSERTION_VERSION_5 5
#define WEBAUTHN_ASSERTION_CURRENT_VERSION WEBAUTHN_ASSERTION_VERSION_5

typedef struct _WEBAUTHN_ASSERTION {
  // Version of this structure, to allow for modifications in the future.
  DWORD dwVersion;

  // Authenticator data that was created for this assertion.
  DWORD cbAuthenticatorData;
#ifdef __midl
  [size_is(cbAuthenticatorData)]
#else
  _Field_size_bytes_(cbAuthenticatorData)
#endif
      PBYTE pbAuthenticatorData;

  // Signature that was generated for this assertion.
  DWORD cbSignature;
#ifdef __midl
  [size_is(cbSignature)]
#else
  _Field_size_bytes_(cbSignature)
#endif
      PBYTE pbSignature;

  // Credential that was used for this assertion.
  WEBAUTHN_CREDENTIAL Credential;

  // UserId
  DWORD cbUserId;
#ifdef __midl
  [size_is(cbUserId)]
#else
  _Field_size_bytes_(cbUserId)
#endif
      PBYTE pbUserId;

  //
  // Following fields have been added in WEBAUTHN_ASSERTION_VERSION_2
  //

  WEBAUTHN_EXTENSIONS Extensions;

  // Size of pbCredLargeBlob
  DWORD cbCredLargeBlob;
#ifdef __midl
  [size_is(cbCredLargeBlob)]
#else
  _Field_size_bytes_(cbCredLargeBlob)
#endif
      PBYTE pbCredLargeBlob;

  DWORD dwCredLargeBlobStatus;

  //
  // Following fields have been added in WEBAUTHN_ASSERTION_VERSION_3
  //

#ifdef __midl
  [unique]
#endif
      PWEBAUTHN_HMAC_SECRET_SALT pHmacSecret;

  //
  // Following fields have been added in WEBAUTHN_ASSERTION_VERSION_4
  //

  // One of the WEBAUTHN_CTAP_TRANSPORT_* bits will be set corresponding to
  // the transport that was used.
  DWORD dwUsedTransport;

  //
  // Following fields have been added in WEBAUTHN_ASSERTION_VERSION_5
  //

  DWORD cbUnsignedExtensionOutputs;
#ifdef __midl
  [size_is(cbUnsignedExtensionOutputs)]
#else
  _Field_size_bytes_(cbUnsignedExtensionOutputs)
#endif
      PBYTE pbUnsignedExtensionOutputs;

} WEBAUTHN_ASSERTION, *PWEBAUTHN_ASSERTION;
typedef const WEBAUTHN_ASSERTION* PCWEBAUTHN_ASSERTION;

#ifndef __midl

//+------------------------------------------------------------------------------------------
// APIs.
//-------------------------------------------------------------------------------------------

DWORD
WINAPI
WebAuthNGetApiVersionNumber();

HRESULT
WINAPI
WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable(
    _Out_ BOOL* pbIsUserVerifyingPlatformAuthenticatorAvailable);

HRESULT
WINAPI
EXPERIMENTAL_WebAuthNIsUserVerifyingNativePlatformAuthenticatorAvailable(
    _Out_ BOOL* pbIsUserVerifyingNativePlatformAuthenticatorAvailable);

HRESULT
WINAPI
WebAuthNAuthenticatorMakeCredential(
    _In_ HWND hWnd,
    _In_ PCWEBAUTHN_RP_ENTITY_INFORMATION pRpInformation,
    _In_ PCWEBAUTHN_USER_ENTITY_INFORMATION pUserInformation,
    _In_ PCWEBAUTHN_COSE_CREDENTIAL_PARAMETERS pPubKeyCredParams,
    _In_ PCWEBAUTHN_CLIENT_DATA pWebAuthNClientData,
    _In_opt_ PCWEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS
        pWebAuthNMakeCredentialOptions,
    _Outptr_result_maybenull_ PWEBAUTHN_CREDENTIAL_ATTESTATION*
        ppWebAuthNCredentialAttestation);

HRESULT
WINAPI
WebAuthNAuthenticatorGetAssertion(
    _In_ HWND hWnd,
    _In_ LPCWSTR pwszRpId,
    _In_ PCWEBAUTHN_CLIENT_DATA pWebAuthNClientData,
    _In_opt_ PCWEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS
        pWebAuthNGetAssertionOptions,
    _Outptr_result_maybenull_ PWEBAUTHN_ASSERTION* ppWebAuthNAssertion);

void WINAPI WebAuthNFreeCredentialAttestation(
    _In_opt_ PWEBAUTHN_CREDENTIAL_ATTESTATION pWebAuthNCredentialAttestation);

void WINAPI WebAuthNFreeAssertion(_In_ PWEBAUTHN_ASSERTION pWebAuthNAssertion);

HRESULT
WINAPI
WebAuthNGetCancellationId(_Out_ GUID* pCancellationId);

HRESULT
WINAPI
WebAuthNCancelCurrentOperation(_In_ const GUID* pCancellationId);

// Returns NTE_NOT_FOUND when credentials are not found.
HRESULT
WINAPI
WebAuthNGetPlatformCredentialList(
    _In_ PCWEBAUTHN_GET_CREDENTIALS_OPTIONS pGetCredentialsOptions,
    _Outptr_result_maybenull_ PWEBAUTHN_CREDENTIAL_DETAILS_LIST*
        ppCredentialDetailsList);

void WINAPI WebAuthNFreePlatformCredentialList(
    _In_ PWEBAUTHN_CREDENTIAL_DETAILS_LIST pCredentialDetailsList);

HRESULT
WINAPI
WebAuthNDeletePlatformCredential(_In_ DWORD cbCredentialId,
                                 _In_reads_bytes_(cbCredentialId)
                                     const BYTE* pbCredentialId);

//
// Returns the following Error Names:
//  L"Success"              - S_OK
//  L"InvalidStateError"    - NTE_EXISTS
//  L"ConstraintError"      - HRESULT_FROM_WIN32(ERROR_NOT_SUPPORTED),
//                            NTE_NOT_SUPPORTED,
//                            NTE_TOKEN_KEYSET_STORAGE_FULL
//  L"NotSupportedError"    - NTE_INVALID_PARAMETER
//  L"NotAllowedError"      - NTE_DEVICE_NOT_FOUND,
//                            NTE_NOT_FOUND,
//                            HRESULT_FROM_WIN32(ERROR_CANCELLED),
//                            NTE_USER_CANCELLED,
//                            HRESULT_FROM_WIN32(ERROR_TIMEOUT)
//  L"UnknownError"         - All other hr values
//
PCWSTR
WINAPI
WebAuthNGetErrorName(_In_ HRESULT hr);

HRESULT
WINAPI
WebAuthNGetW3CExceptionDOMError(_In_ HRESULT hr);

typedef enum _EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE {
  PluginAuthenticatorState_Unknown = 0,
  PluginAuthenticatorState_Disabled,
  PluginAuthenticatorState_Enabled
} EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE;

//
// Plugin Authenticator API: WebAuthNPluginGetAuthenticatorState: Get Plugin
// Authenticator State
//
HRESULT
WINAPI
EXPERIMENTAL_WebAuthNPluginGetAuthenticatorState(
    _In_ LPCWSTR pwszPluginClsId,
    _Out_ EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE* pluginAuthenticatorState);

//
// Plugin Authenticator API: WebAuthNAddPluginAuthenticator: Add Plugin
// Authenticator
//

typedef struct _EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS {
  // Authenticator Name
  LPCWSTR pwszAuthenticatorName;

  // Plugin COM ClsId
  LPCWSTR pwszPluginClsId;

  // Plugin RPID (Optional. Required for a nested WebAuthN call originating from
  // a plugin)
  LPCWSTR pwszPluginRpId;

  // Plugin Authenticator Logo for the Light themes. base64 svg (Optional)
  LPCWSTR pwszLightThemeLogo;

  // Plugin Authenticator Logo for the Dark themes. base64 svg (Optional)
  LPCWSTR pwszDarkThemeLogo;

  // CTAP CBOR encoded authenticatorGetInfo
  DWORD cbAuthenticatorInfo;
  _Field_size_bytes_(cbAuthenticatorInfo) PBYTE pbAuthenticatorInfo;

} EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS,
    *EXPERIMENTAL_PWEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS;
typedef const EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS*
    EXPERIMENTAL_PCWEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS;

typedef struct _EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE {
  // Plugin operation signing Public Key - Used to sign the request in the
  // EXPERIMENTAL_PluginPerformOperation. Refer pluginauthenticator.h.
  DWORD cbOpSignPubKey;
  _Field_size_bytes_(cbOpSignPubKey) PBYTE pbOpSignPubKey;

} EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE,
    *EXPERIMENTAL_PWEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE;
typedef const EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE*
    EXPERIMENTAL_PCWEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE;

HRESULT
WINAPI
EXPERIMENTAL_WebAuthNPluginAddAuthenticator(
    _In_ EXPERIMENTAL_PCWEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS
        pPluginAddAuthenticatorOptions,
    _Outptr_result_maybenull_ EXPERIMENTAL_PWEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE*
        ppPluginAddAuthenticatorResponse);

void WINAPI EXPERIMENTAL_WebAuthNPluginFreeAddAuthenticatorResponse(
    _In_opt_ EXPERIMENTAL_PWEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE
        pPluginAddAuthenticatorResponse);

//
// Plugin Authenticator API: WebAuthNRemovePluginAuthenticator: Remove Plugin
// Authenticator
//

HRESULT
WINAPI
EXPERIMENTAL_WebAuthNPluginRemoveAuthenticator(_In_ LPCWSTR pwszPluginClsId);

//
// Plugin Authenticator API: WebAuthNPluginAuthenticatorUpdateDetails: Update
// Credential Metadata for Browser AutoFill Scenarios
//

typedef struct _EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS {
  // Authenticator Name (Optional)
  LPCWSTR pwszAuthenticatorName;

  // Plugin COM ClsId
  LPCWSTR pwszPluginClsId;

  // Plugin COM New ClsId (Optional)
  LPCWSTR pwszNewPluginClsId;

  // Plugin Authenticator Logo for the Light themes. base64 svg (Optional)
  LPCWSTR pwszLightThemeLogo;

  // Plugin Authenticator Logo for the Dark themes. base64 svg (Optional)
  LPCWSTR pwszDarkThemeLogo;

  // CTAP CBOR encoded authenticatorGetInfo (Optional)
  DWORD cbAuthenticatorInfo;
  _Field_size_bytes_(cbAuthenticatorInfo) PBYTE pbAuthenticatorInfo;

} EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS,
    *EXPERIMENTAL_PWEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS;
typedef const EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS*
    EXPERIMENTAL_PCWEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS;

HRESULT
WINAPI
EXPERIMENTAL_WebAuthNPluginUpdateAuthenticatorDetails(
    _In_ EXPERIMENTAL_PCWEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS
        pPluginUpdateAuthenticatorDetails);

#endif  //__midl

//
// Plugin Authenticator API: WebAuthNPluginAuthenticatorAddCredentials: Add
// Credential Metadata for Browser AutoFill Scenarios
//

typedef struct _EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS {
  // Size of pbCredentialId.
  DWORD cbCredentialId;

// Credential Identifier bytes. This field is required.
#ifdef __midl
  [size_is(cbCredentialId)]
#else
  _Field_size_bytes_(cbCredentialId)
#endif
      PBYTE pbCredentialId;

  // Identifier for the RP. This field is required.
  PWSTR pwszRpId;

  // Contains the friendly name of the Relying Party, such as "Acme
  // Corporation", "Widgets Inc" or "Awesome Site". This field is required.
  PWSTR pwszRpName;

  // Identifier for the User. This field is required.
  DWORD cbUserId;

// User Identifier bytes. This field is required.
#ifdef __midl
  [size_is(cbUserId)]
#else
  _Field_size_bytes_(cbUserId)
#endif
      PBYTE pbUserId;

  // Contains a detailed name for this account, such as
  // "john.p.smith@example.com".
  PWSTR pwszUserName;

  // For User: Contains the friendly name associated with the user account such
  // as "John P. Smith".
  PWSTR pwszUserDisplayName;

} EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS,
    *EXPERIMENTAL_PWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS;
typedef const EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS*
    EXPERIMENTAL_PCWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS;

typedef struct _EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST {
  // Plugin COM ClsId
  PWSTR pwszPluginClsId;

  // count of credentials
  DWORD cCredentialDetails;

#ifdef __midl
  [size_is(cCredentialDetails)]
#else
  _Field_size_(cCredentialDetails)
#endif
      EXPERIMENTAL_PWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS* pCredentialDetails;

} EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST,
    *EXPERIMENTAL_PWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST;
typedef const EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST*
    EXPERIMENTAL_PCWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST;

#ifndef __midl

HRESULT
WINAPI
EXPERIMENTAL_WebAuthNPluginAuthenticatorAddCredentials(
    _In_ EXPERIMENTAL_PWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST
        pCredentialDetailsList);

//
// Plugin Authenticator API: WebAuthNPluginAuthenticatorRemoveCredentials:
// Remove Credential Metadata for Browser AutoFill Scenarios
//

HRESULT
WINAPI
EXPERIMENTAL_WebAuthNPluginAuthenticatorRemoveCredentials(
    _In_ EXPERIMENTAL_PWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST
        pCredentialDetailsList);

//
// Plugin Authenticator API: WebAuthNPluginAuthenticatorRemoveCredentials:
// Remove All Credential Metadata for Browser AutoFill Scenarios
//

HRESULT
WINAPI
EXPERIMENTAL_WebAuthNPluginAuthenticatorRemoveAllCredentials(
    _In_ LPCWSTR pwszPluginClsId);

//
// Plugin Authenticator API: WebAuthNPluginAuthenticatorGetAllCredentials: Get
// All Credential Metadata cached for Browser AutoFill Scenarios
//
HRESULT
WINAPI
EXPERIMENTAL_WebAuthNPluginAuthenticatorGetAllCredentials(
    _In_ LPCWSTR pwszPluginClsId,
    _Outptr_result_maybenull_ EXPERIMENTAL_PWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST*
        ppCredentialDetailsList);

//
// Hello UV API for Plugin: WebAuthNPluginPerformUv: Perform Hello UV related
// operations
//

typedef enum _EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE {
  PerformUv = 1,
  GetUvCount,
  GetPubKey
} EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE;

typedef struct _EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV {
  HWND hwnd;
  GUID* transactionId;
  EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE type;
  PCWSTR pwszUsername;
  PCWSTR pwszContext;
} EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV,
    *EXPERIMENTAL_PWEBAUTHN_PLUGIN_PERFROM_UV;
typedef const EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV*
    EXPERIMENTAL_PCWEBAUTHN_PLUGIN_PERFORM_UV;

typedef struct _EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE {
  DWORD cbResponse;
  PBYTE pbResponse;
} EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE,
    *EXPERIMENTAL_PWEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE;
typedef const EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE*
    EXPERIMENTAL_PCWEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE;

HRESULT
WINAPI
EXPERIMENTAL_WebAuthNPluginPerformUv(
    _In_ EXPERIMENTAL_PCWEBAUTHN_PLUGIN_PERFORM_UV pPluginPerformUv,
    _Outptr_result_maybenull_ EXPERIMENTAL_PWEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE*
        ppPluginPerformUvRespose);

void WINAPI EXPERIMENTAL_WebAuthNPluginFreePerformUvResponse(
    _In_opt_ EXPERIMENTAL_PWEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE
        ppPluginPerformUvResponse);

#define EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_VERSION_1 1
#define EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_CURRENT_VERSION \
  EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_VERSION_1
typedef struct _EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS {
  // Version of this structure, to allow for modifications in the future.
  DWORD dwVersion;

  // Following have following values:
  //  +1 - TRUE
  //   0 - Not defined
  //  -1 - FALSE
  // up: "true" | "false"
  LONG lUp;
  // uv: "true" | "false"
  LONG lUv;
  // rk: "true" | "false"
  LONG lRequireResidentKey;
} EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS,
    *EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS;
typedef const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS*
    EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS;

#define EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY_VERSION_1 1
#define EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY_CURRENT_VERSION \
  EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY_VERSION_1
typedef struct _EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY {
  // Version of this structure, to allow for modifications in the future.
  DWORD dwVersion;

  // Key type
  LONG lKty;

  // Hash Algorithm: ES256, ES384, ES512
  LONG lAlg;

  // Curve
  LONG lCrv;

  // Size of "x" (X Coordinate)
  DWORD cbX;

  //"x" (X Coordinate) data. Big Endian.
  PBYTE pbX;

  // Size of "y" (Y Coordinate)
  DWORD cbY;

  //"y" (Y Coordinate) data. Big Endian.
  PBYTE pbY;
} EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY,
    *EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY;
typedef const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY*
    EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY;

#define EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION_VERSION_1 1
#define EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION_CURRENT_VERSION \
  EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION_VERSION_1
typedef struct _EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION {
  // Version of this structure, to allow for modifications in the future.
  DWORD dwVersion;

  // Platform's key agreement public key
  EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY pKeyAgreement;

  DWORD cbEncryptedSalt;
  PBYTE pbEncryptedSalt;

  DWORD cbSaltAuth;
  PBYTE pbSaltAuth;
} EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION,
    *EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION;
typedef const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION*
    EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION;

#define EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST_VERSION_1 1
#define EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST_CURRENT_VERSION \
  EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST_VERSION_1
typedef struct _EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST {
  // Version of this structure, to allow for modifications in the future.
  DWORD dwVersion;

  // Input RP ID. Raw UTF8 bytes before conversion.
  // These are the bytes to be hashed in the Authenticator Data.
  DWORD cbRpId;
  PBYTE pbRpId;

  // Client Data Hash
  DWORD cbClientDataHash;
  PBYTE pbClientDataHash;

  // RP Information
  PCWEBAUTHN_RP_ENTITY_INFORMATION pRpInformation;

  // User Information
  PCWEBAUTHN_USER_ENTITY_INFORMATION pUserInformation;

  // Crypto Parameters
  WEBAUTHN_COSE_CREDENTIAL_PARAMETERS WebAuthNCredentialParameters;

  // Credentials used for exclusion
  WEBAUTHN_CREDENTIAL_LIST CredentialList;

  // Optional extensions to parse when performing the operation.
  DWORD cbCborExtensionsMap;
  PBYTE pbCborExtensionsMap;

  // Authenticator Options (Optional)
  EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS pAuthenticatorOptions;

  // Pin Auth (Optional)
  BOOL fEmptyPinAuth;  // Zero length PinAuth is included in the request
  DWORD cbPinAuth;
  PBYTE pbPinAuth;

  //"hmac-secret": true extension
  LONG lHmacSecretExt;

  // "hmac-secret-mc" extension
  EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION pHmacSecretMcExtension;

  //"prf" extension
  LONG lPrfExt;
  DWORD cbHmacSecretSaltValues;
  PBYTE pbHmacSecretSaltValues;

  //"credProtect" extension. Nonzero if present
  DWORD dwCredProtect;

  // Nonzero if present
  DWORD dwPinProtocol;

  // Nonzero if present
  DWORD dwEnterpriseAttestation;

  //"credBlob" extension. Nonzero if present
  DWORD cbCredBlobExt;
  PBYTE pbCredBlobExt;

  //"largeBlobKey": true extension
  LONG lLargeBlobKeyExt;

  //"largeBlob": extension
  DWORD dwLargeBlobSupport;

  //"minPinLength": true extension
  LONG lMinPinLengthExt;

  // "json" extension. Nonzero if present
  DWORD cbJsonExt;
  PBYTE pbJsonExt;
} EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST,
    *EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST;
typedef const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST*
    EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST;

_Success_(return == S_OK) HRESULT WINAPI
    EXPERIMENTAL_WebAuthNEncodeMakeCredentialResponse(
        _In_ PCWEBAUTHN_CREDENTIAL_ATTESTATION pCredentialAttestation,
        _Out_ DWORD* pcbResp,
        _Outptr_result_buffer_maybenull_(*pcbResp) BYTE** ppbResp);

_Success_(return == S_OK) HRESULT WINAPI
    EXPERIMENTAL_WebAuthNDecodeMakeCredentialRequest(
        _In_ DWORD cbEncoded,
        _In_reads_bytes_(cbEncoded) const BYTE* pbEncoded,
        _Outptr_ EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST*
            ppMakeCredentialRequest);

void WINAPI EXPERIMENTAL_WebAuthNFreeDecodedMakeCredentialRequest(
    _In_opt_ EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST
        pMakeCredentialRequest);

#define EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST_VERSION_1 1
#define EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST_CURRENT_VERSION \
  EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST_VERSION_1
typedef struct _EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST {
  // Version of this structure, to allow for modifications in the future.
  DWORD dwVersion;

  // RP ID. After UTF8 to Unicode conversion,
  PCWSTR pwszRpId;

  // Input RP ID. Raw UTF8 bytes before conversion.
  // These are the bytes to be hashed in the Authenticator Data.
  DWORD cbRpId;
  PBYTE pbRpId;

  // Client Data Hash
  DWORD cbClientDataHash;
  PBYTE pbClientDataHash;

  // Credentials used for inclusion
  WEBAUTHN_CREDENTIAL_LIST CredentialList;

  // Optional extensions to parse when performing the operation.
  DWORD cbCborExtensionsMap;
  PBYTE pbCborExtensionsMap;

  // Authenticator Options (Optional)
  EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS pAuthenticatorOptions;

  // Pin Auth (Optional)
  BOOL fEmptyPinAuth;  // Zero length PinAuth is included in the request
  DWORD cbPinAuth;
  PBYTE pbPinAuth;

  // HMAC Salt Extension (Optional)
  EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION pHmacSaltExtension;

  // PRF Extension
  DWORD cbHmacSecretSaltValues;
  PBYTE pbHmacSecretSaltValues;

  DWORD dwPinProtocol;

  //"credBlob": true  extension
  LONG lCredBlobExt;

  //"largeBlobKey": true extension
  LONG lLargeBlobKeyExt;

  //"largeBlob" extension
  DWORD dwCredLargeBlobOperation;
  DWORD cbCredLargeBlobCompressed;
  PBYTE pbCredLargeBlobCompressed;
  DWORD dwCredLargeBlobOriginalSize;

  // "json" extension. Nonzero if present
  DWORD cbJsonExt;
  PBYTE pbJsonExt;
} EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST,
    *EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST;
typedef const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST*
    EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST;

_Success_(return == S_OK) HRESULT WINAPI
    EXPERIMENTAL_WebAuthNDecodeGetAssertionRequest(
        _In_ DWORD cbEncoded,
        _In_reads_bytes_(cbEncoded) const BYTE* pbEncoded,
        _Outptr_ EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST*
            ppGetAssertionRequest);

void WINAPI EXPERIMENTAL_WebAuthNFreeDecodedGetAssertionRequest(
    _In_opt_ EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST
        pGetAssertionRequest);

typedef struct _EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE {
  // [1] credential (optional)
  // [2] authenticatorData
  // [3] signature
  WEBAUTHN_ASSERTION WebAuthNAssertion;

  // [4] user (optional)
  PCWEBAUTHN_USER_ENTITY_INFORMATION pUserInformation;

  // [5] numberOfCredentials (optional)
  DWORD dwNumberOfCredentials;

  // [6] userSelected (optional)
  LONG lUserSelected;

  // [7] largeBlobKey (optional)
  DWORD cbLargeBlobKey;
  PBYTE pbLargeBlobKey;

  // [8] unsignedExtensionOutputs
  DWORD cbUnsignedExtensionOutputs;
  PBYTE pbUnsignedExtensionOutputs;
} EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE,
    *EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE;
typedef const EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE*
    EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE;

_Success_(return == S_OK) HRESULT WINAPI
    EXPERIMENTAL_WebAuthNEncodeGetAssertionResponse(
        _In_ EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE
            pGetAssertionResponse,
        _Out_ DWORD* pcbResp,
        _Outptr_result_buffer_maybenull_(*pcbResp) BYTE** ppbResp);

#endif  //__midl

#ifdef __cplusplus
}  // Balance extern "C" above
#endif

#endif  // WINAPI_FAMILY_PARTITION
#pragma endregion
