/* pkcs11.h
   Copyright 2006, 2007 g10 Code GmbH
   Copyright 2006 Andreas Jellinghaus
   Copyright 2017 Red Hat, Inc.

   This file is free software; as a special exception the author gives
   unlimited permission to copy and/or distribute it, with or without
   modifications, as long as this notice is preserved.

   This file is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY, to the extent permitted by law; without even
   the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
   PURPOSE.  */

/* Please submit any changes back to the p11-kit project at
   https://github.com/p11-glue/p11-kit/, so that
   they can be picked up by other projects from there as well.  */

/* This file is a modified implementation of the PKCS #11 standard by
   OASIS group.  It is mostly a drop-in replacement, with the
   following change:

   This header file does not require any macro definitions by the user
   (like CK_DEFINE_FUNCTION etc).  In fact, it defines those macros
   for you (if useful, some are missing, let me know if you need
   more).

   There is an additional API available that does comply better to the
   GNU coding standard.  It can be switched on by defining
   CRYPTOKI_GNU before including this header file.  For this, the
   following changes are made to the specification:

   All structure types are changed to a "struct ck_foo" where CK_FOO
   is the type name in PKCS #11.

   All non-structure types are changed to ck_foo_t where CK_FOO is the
   lowercase version of the type name in PKCS #11.  The basic types
   (CK_ULONG et al.) are removed without substitute.

   All members of structures are modified in the following way: Type
   indication prefixes are removed, and underscore characters are
   inserted before words.  Then the result is lowercased.

   Note that function names are still in the original case, as they
   need for ABI compatibility.

   CK_FALSE, CK_TRUE and NULL_PTR are removed without substitute.  Use
   <stdbool.h>.

   If CRYPTOKI_COMPAT is defined before including this header file,
   then none of the API changes above take place, and the API is the
   one defined by the PKCS #11 standard.  */

#ifndef PKCS11_H
#define PKCS11_H 1

#if defined(__cplusplus)
extern "C" {
#endif


/* The version of cryptoki we implement.  The revision is changed with
   each modification of this file.  */
#define CRYPTOKI_VERSION_MAJOR		2
#define CRYPTOKI_VERSION_MINOR		40
#define P11_KIT_CRYPTOKI_VERSION_REVISION	0


/* Compatibility interface is default, unless CRYPTOKI_GNU is
   given.  */
#ifndef CRYPTOKI_GNU
#ifndef CRYPTOKI_COMPAT
#define CRYPTOKI_COMPAT 1
#endif
#endif

/* System dependencies.  */

#if defined(_WIN32) || defined(CRYPTOKI_FORCE_WIN32)

/* There is a matching pop below.  */
#pragma pack(push, cryptoki, 1)

#ifdef CRYPTOKI_EXPORTS
#define CK_SPEC __declspec(dllexport)
#else
#define CK_SPEC __declspec(dllimport)
#endif

#else

#define CK_SPEC

#endif


#ifdef CRYPTOKI_COMPAT
  /* If we are in compatibility mode, switch all exposed names to the
     PKCS #11 variant.  There are corresponding #undefs below.  */

#define ck_flags_t CK_FLAGS
#define ck_version _CK_VERSION

#define ck_info _CK_INFO
#define cryptoki_version cryptokiVersion
#define manufacturer_id manufacturerID
#define library_description libraryDescription
#define library_version libraryVersion

#define ck_notification_t CK_NOTIFICATION
#define ck_slot_id_t CK_SLOT_ID

#define ck_slot_info _CK_SLOT_INFO
#define slot_description slotDescription
#define hardware_version hardwareVersion
#define firmware_version firmwareVersion

#define ck_token_info _CK_TOKEN_INFO
#define serial_number serialNumber
#define max_session_count ulMaxSessionCount
#define session_count ulSessionCount
#define max_rw_session_count ulMaxRwSessionCount
#define rw_session_count ulRwSessionCount
#define max_pin_len ulMaxPinLen
#define min_pin_len ulMinPinLen
#define total_public_memory ulTotalPublicMemory
#define free_public_memory ulFreePublicMemory
#define total_private_memory ulTotalPrivateMemory
#define free_private_memory ulFreePrivateMemory
#define utc_time utcTime

#define ck_session_handle_t CK_SESSION_HANDLE
#define ck_user_type_t CK_USER_TYPE
#define ck_state_t CK_STATE

#define ck_session_info _CK_SESSION_INFO
#define slot_id slotID
#define device_error ulDeviceError

#define ck_object_handle_t CK_OBJECT_HANDLE
#define ck_object_class_t CK_OBJECT_CLASS
#define ck_hw_feature_type_t CK_HW_FEATURE_TYPE
#define ck_key_type_t CK_KEY_TYPE
#define ck_certificate_type_t CK_CERTIFICATE_TYPE
#define ck_attribute_type_t CK_ATTRIBUTE_TYPE

#define ck_attribute _CK_ATTRIBUTE
#define value pValue
#define value_len ulValueLen

#define count ulCount

#define ck_date _CK_DATE

#define ck_mechanism_type_t CK_MECHANISM_TYPE

#define ck_mechanism _CK_MECHANISM
#define parameter pParameter
#define parameter_len ulParameterLen

#define params pParams

#define ck_mechanism_info _CK_MECHANISM_INFO
#define min_key_size ulMinKeySize
#define max_key_size ulMaxKeySize

#define ck_param_type CK_PARAM_TYPE
#define ck_otp_param CK_OTP_PARAM
#define ck_otp_params CK_OTP_PARAMS
#define ck_otp_signature_info CK_OTP_SIGNATURE_INFO

#define ck_rv_t CK_RV
#define ck_notify_t CK_NOTIFY

#define ck_function_list _CK_FUNCTION_LIST

#define ck_createmutex_t CK_CREATEMUTEX
#define ck_destroymutex_t CK_DESTROYMUTEX
#define ck_lockmutex_t CK_LOCKMUTEX
#define ck_unlockmutex_t CK_UNLOCKMUTEX

#define ck_c_initialize_args _CK_C_INITIALIZE_ARGS
#define create_mutex CreateMutex
#define destroy_mutex DestroyMutex
#define lock_mutex LockMutex
#define unlock_mutex UnlockMutex
#define reserved pReserved

#define ck_rsa_pkcs_mgf_type_t CK_RSA_PKCS_MGF_TYPE
#define ck_rsa_pkcs_oaep_source_type_t CK_RSA_PKCS_OAEP_SOURCE_TYPE
#define hash_alg hashAlg
#define s_len sLen
#define source_data pSourceData
#define source_data_len ulSourceDataLen

#define counter_bits ulCounterBits
#define iv_ptr pIv
#define iv_len ulIvLen
#define iv_bits ulIvBits
#define aad_ptr pAAD
#define aad_len ulAADLen
#define tag_bits ulTagBits
#define shared_data_len ulSharedDataLen
#define shared_data pSharedData
#define public_data_len ulPublicDataLen
#define public_data pPublicData
#define string_data pData
#define string_data_len ulLen
#define data_params pData
#endif	/* CRYPTOKI_COMPAT */



typedef unsigned long ck_flags_t;

struct ck_version
{
  unsigned char major;
  unsigned char minor;
};


struct ck_info
{
  struct ck_version cryptoki_version;
  unsigned char manufacturer_id[32];
  ck_flags_t flags;
  unsigned char library_description[32];
  struct ck_version library_version;
};


typedef unsigned long ck_notification_t;

#define CKN_SURRENDER	(0UL)


typedef unsigned long ck_slot_id_t;


struct ck_slot_info
{
  unsigned char slot_description[64];
  unsigned char manufacturer_id[32];
  ck_flags_t flags;
  struct ck_version hardware_version;
  struct ck_version firmware_version;
};


#define CKF_TOKEN_PRESENT	(1UL << 0)
#define CKF_REMOVABLE_DEVICE	(1UL << 1)
#define CKF_HW_SLOT		(1UL << 2)
#define CKF_ARRAY_ATTRIBUTE	(1UL << 30)


struct ck_token_info
{
  unsigned char label[32];
  unsigned char manufacturer_id[32];
  unsigned char model[16];
  unsigned char serial_number[16];
  ck_flags_t flags;
  unsigned long max_session_count;
  unsigned long session_count;
  unsigned long max_rw_session_count;
  unsigned long rw_session_count;
  unsigned long max_pin_len;
  unsigned long min_pin_len;
  unsigned long total_public_memory;
  unsigned long free_public_memory;
  unsigned long total_private_memory;
  unsigned long free_private_memory;
  struct ck_version hardware_version;
  struct ck_version firmware_version;
  unsigned char utc_time[16];
};


#define CKF_RNG					(1UL << 0)
#define CKF_WRITE_PROTECTED			(1UL << 1)
#define CKF_LOGIN_REQUIRED			(1UL << 2)
#define CKF_USER_PIN_INITIALIZED		(1UL << 3)
#define CKF_RESTORE_KEY_NOT_NEEDED		(1UL << 5)
#define CKF_CLOCK_ON_TOKEN			(1UL << 6)
#define CKF_PROTECTED_AUTHENTICATION_PATH	(1UL << 8)
#define CKF_DUAL_CRYPTO_OPERATIONS		(1UL << 9)
#define CKF_TOKEN_INITIALIZED			(1UL << 10)
#define CKF_SECONDARY_AUTHENTICATION		(1UL << 11)
#define CKF_USER_PIN_COUNT_LOW			(1UL << 16)
#define CKF_USER_PIN_FINAL_TRY			(1UL << 17)
#define CKF_USER_PIN_LOCKED			(1UL << 18)
#define CKF_USER_PIN_TO_BE_CHANGED		(1UL << 19)
#define CKF_SO_PIN_COUNT_LOW			(1UL << 20)
#define CKF_SO_PIN_FINAL_TRY			(1UL << 21)
#define CKF_SO_PIN_LOCKED			(1UL << 22)
#define CKF_SO_PIN_TO_BE_CHANGED		(1UL << 23)

#define CK_UNAVAILABLE_INFORMATION	((unsigned long)-1L)
#define CK_EFFECTIVELY_INFINITE		(0UL)


typedef unsigned long ck_session_handle_t;

#define CK_INVALID_HANDLE	(0UL)


typedef unsigned long ck_user_type_t;

#define CKU_SO			(0UL)
#define CKU_USER		(1UL)
#define CKU_CONTEXT_SPECIFIC	(2UL)


typedef unsigned long ck_state_t;

#define CKS_RO_PUBLIC_SESSION	(0UL)
#define CKS_RO_USER_FUNCTIONS	(1UL)
#define CKS_RW_PUBLIC_SESSION	(2UL)
#define CKS_RW_USER_FUNCTIONS	(3UL)
#define CKS_RW_SO_FUNCTIONS	(4UL)


struct ck_session_info
{
  ck_slot_id_t slot_id;
  ck_state_t state;
  ck_flags_t flags;
  unsigned long device_error;
};

#define CKF_RW_SESSION		(1UL << 1)
#define CKF_SERIAL_SESSION	(1UL << 2)


typedef unsigned long ck_object_handle_t;


typedef unsigned long ck_object_class_t;

#define CKO_DATA		(0UL)
#define CKO_CERTIFICATE		(1UL)
#define CKO_PUBLIC_KEY		(2UL)
#define CKO_PRIVATE_KEY		(3UL)
#define CKO_SECRET_KEY		(4UL)
#define CKO_HW_FEATURE		(5UL)
#define CKO_DOMAIN_PARAMETERS	(6UL)
#define CKO_MECHANISM		(7UL)
#define CKO_OTP_KEY		(8UL)
#define CKO_VENDOR_DEFINED	((unsigned long) (1UL << 31))


typedef unsigned long ck_hw_feature_type_t;

#define CKH_MONOTONIC_COUNTER	(1UL)
#define CKH_CLOCK		(2UL)
#define CKH_USER_INTERFACE	(3UL)
#define CKH_VENDOR_DEFINED	((unsigned long) (1UL << 31))


typedef unsigned long ck_key_type_t;

#define CKK_RSA			(0UL)
#define CKK_DSA			(1UL)
#define CKK_DH			(2UL)
#define CKK_ECDSA		(3UL)
#define CKK_EC			(3UL)
#define CKK_X9_42_DH		(4UL)
#define CKK_KEA			(5UL)
#define CKK_GENERIC_SECRET	(0x10UL)
#define CKK_RC2			(0x11UL)
#define CKK_RC4			(0x12UL)
#define CKK_DES			(0x13UL)
#define CKK_DES2		(0x14UL)
#define CKK_DES3		(0x15UL)
#define CKK_CAST		(0x16UL)
#define CKK_CAST3		(0x17UL)
#define CKK_CAST128		(0x18UL)
#define CKK_RC5			(0x19UL)
#define CKK_IDEA		(0x1aUL)
#define CKK_SKIPJACK		(0x1bUL)
#define CKK_BATON		(0x1cUL)
#define CKK_JUNIPER		(0x1dUL)
#define CKK_CDMF		(0x1eUL)
#define CKK_AES			(0x1fUL)
#define CKK_BLOWFISH		(0x20UL)
#define CKK_TWOFISH		(0x21UL)
#define CKK_SECURID		(0x22UL)
#define CKK_HOTP		(0x23UL)
#define CKK_ACTI		(0x24UL)
#define CKK_CAMELLIA		(0x25UL)
#define CKK_ARIA		(0x26UL)
#define CKK_MD5_HMAC		(0x27UL)
#define CKK_SHA_1_HMAC		(0x28UL)
#define CKK_RIPEMD128_HMAC	(0x29UL)
#define CKK_RIPEMD160_HMAC	(0x2aUL)
#define CKK_SHA256_HMAC		(0x2bUL)
#define CKK_SHA384_HMAC		(0x2cUL)
#define CKK_SHA512_HMAC		(0x2dUL)
#define CKK_SHA224_HMAC		(0x2eUL)
#define CKK_SEED		(0x2fUL)
#define CKK_GOSTR3410		(0x30UL)
#define CKK_GOSTR3411		(0x31UL)
#define CKK_GOST28147		(0x32UL)
#define CKK_EC_EDWARDS		(0x40UL)
#define CKK_VENDOR_DEFINED	((unsigned long) (1UL << 31))


typedef unsigned long ck_certificate_type_t;

#define CKC_X_509		(0UL)
#define CKC_X_509_ATTR_CERT	(1UL)
#define CKC_WTLS		(2UL)
#define CKC_VENDOR_DEFINED	((unsigned long) (1UL << 31))

#define CKC_OPENPGP		(CKC_VENDOR_DEFINED|0x504750UL)

typedef unsigned long ck_attribute_type_t;

#define CKA_CLASS			(0UL)
#define CKA_TOKEN			(1UL)
#define CKA_PRIVATE			(2UL)
#define CKA_LABEL			(3UL)
#define CKA_APPLICATION			(0x10UL)
#define CKA_VALUE			(0x11UL)
#define CKA_OBJECT_ID			(0x12UL)
#define CKA_CERTIFICATE_TYPE		(0x80UL)
#define CKA_ISSUER			(0x81UL)
#define CKA_SERIAL_NUMBER		(0x82UL)
#define CKA_AC_ISSUER			(0x83UL)
#define CKA_OWNER			(0x84UL)
#define CKA_ATTR_TYPES			(0x85UL)
#define CKA_TRUSTED			(0x86UL)
#define CKA_CERTIFICATE_CATEGORY	(0x87UL)
#define CKA_JAVA_MIDP_SECURITY_DOMAIN	(0x88UL)
#define CKA_URL				(0x89UL)
#define CKA_HASH_OF_SUBJECT_PUBLIC_KEY	(0x8aUL)
#define CKA_HASH_OF_ISSUER_PUBLIC_KEY	(0x8bUL)
#define CKA_NAME_HASH_ALGORITHM         (0x8cUL)
#define CKA_CHECK_VALUE			(0x90UL)
#define CKA_KEY_TYPE			(0x100UL)
#define CKA_SUBJECT			(0x101UL)
#define CKA_ID				(0x102UL)
#define CKA_SENSITIVE			(0x103UL)
#define CKA_ENCRYPT			(0x104UL)
#define CKA_DECRYPT			(0x105UL)
#define CKA_WRAP			(0x106UL)
#define CKA_UNWRAP			(0x107UL)
#define CKA_SIGN			(0x108UL)
#define CKA_SIGN_RECOVER		(0x109UL)
#define CKA_VERIFY			(0x10aUL)
#define CKA_VERIFY_RECOVER		(0x10bUL)
#define CKA_DERIVE			(0x10cUL)
#define CKA_START_DATE			(0x110UL)
#define CKA_END_DATE			(0x111UL)
#define CKA_MODULUS			(0x120UL)
#define CKA_MODULUS_BITS		(0x121UL)
#define CKA_PUBLIC_EXPONENT		(0x122UL)
#define CKA_PRIVATE_EXPONENT		(0x123UL)
#define CKA_PRIME_1			(0x124UL)
#define CKA_PRIME_2			(0x125UL)
#define CKA_EXPONENT_1			(0x126UL)
#define CKA_EXPONENT_2			(0x127UL)
#define CKA_COEFFICIENT			(0x128UL)
#define CKA_PUBLIC_KEY_INFO		(0x129UL)
#define CKA_PRIME			(0x130UL)
#define CKA_SUBPRIME			(0x131UL)
#define CKA_BASE			(0x132UL)
#define CKA_PRIME_BITS			(0x133UL)
#define CKA_SUB_PRIME_BITS		(0x134UL)
#define CKA_VALUE_BITS			(0x160UL)
#define CKA_VALUE_LEN			(0x161UL)
#define CKA_EXTRACTABLE			(0x162UL)
#define CKA_LOCAL			(0x163UL)
#define CKA_NEVER_EXTRACTABLE		(0x164UL)
#define CKA_ALWAYS_SENSITIVE		(0x165UL)
#define CKA_KEY_GEN_MECHANISM		(0x166UL)
#define CKA_MODIFIABLE			(0x170UL)
#define CKA_COPYABLE			(0x171UL)
#define CKA_DESTROYABLE			(0x172UL)
#define CKA_ECDSA_PARAMS		(0x180UL)
#define CKA_EC_PARAMS			(0x180UL)
#define CKA_EC_POINT			(0x181UL)
#define CKA_SECONDARY_AUTH		(0x200UL)
#define CKA_AUTH_PIN_FLAGS		(0x201UL)
#define CKA_ALWAYS_AUTHENTICATE		(0x202UL)
#define CKA_WRAP_WITH_TRUSTED		(0x210UL)
#define CKA_OTP_FORMAT			(0x220UL)
#define CKA_OTP_LENGTH			(0x221UL)
#define CKA_OTP_TIME_INTERVAL		(0x222UL)
#define CKA_OTP_USER_FRIENDLY_MODE	(0x223UL)
#define CKA_OTP_CHALLENGE_REQUIREMENT	(0x224UL)
#define CKA_OTP_TIME_REQUIREMENT	(0x225UL)
#define CKA_OTP_COUNTER_REQUIREMENT	(0x226UL)
#define CKA_OTP_PIN_REQUIREMENT		(0x227UL)
#define CKA_OTP_USER_IDENTIFIER		(0x22AUL)
#define CKA_OTP_SERVICE_IDENTIFIER	(0x22BUL)
#define CKA_OTP_SERVICE_LOGO		(0x22CUL)
#define CKA_OTP_SERVICE_LOGO_TYPE	(0x22DUL)
#define CKA_OTP_COUNTER			(0x22EUL)
#define CKA_OTP_TIME                    (0x22FUL)
#define CKA_GOSTR3410_PARAMS		(0x250UL)
#define CKA_GOSTR3411_PARAMS		(0x251UL)
#define CKA_GOST28147_PARAMS		(0x252UL)
#define CKA_HW_FEATURE_TYPE		(0x300UL)
#define CKA_RESET_ON_INIT		(0x301UL)
#define CKA_HAS_RESET			(0x302UL)
#define CKA_PIXEL_X			(0x400UL)
#define CKA_PIXEL_Y			(0x401UL)
#define CKA_RESOLUTION			(0x402UL)
#define CKA_CHAR_ROWS			(0x403UL)
#define CKA_CHAR_COLUMNS		(0x404UL)
#define CKA_COLOR			(0x405UL)
#define CKA_BITS_PER_PIXEL		(0x406UL)
#define CKA_CHAR_SETS			(0x480UL)
#define CKA_ENCODING_METHODS		(0x481UL)
#define CKA_MIME_TYPES			(0x482UL)
#define CKA_MECHANISM_TYPE		(0x500UL)
#define CKA_REQUIRED_CMS_ATTRIBUTES	(0x501UL)
#define CKA_DEFAULT_CMS_ATTRIBUTES	(0x502UL)
#define CKA_SUPPORTED_CMS_ATTRIBUTES	(0x503UL)
#define CKA_WRAP_TEMPLATE		(CKF_ARRAY_ATTRIBUTE | 0x211UL)
#define CKA_UNWRAP_TEMPLATE		(CKF_ARRAY_ATTRIBUTE | 0x212UL)
#define CKA_DERIVE_TEMPLATE		(CKF_ARRAY_ATTRIBUTE | 0x213UL)
#define CKA_ALLOWED_MECHANISMS		(CKF_ARRAY_ATTRIBUTE | 0x600UL)
#define CKA_VENDOR_DEFINED		((unsigned long) (1UL << 31))


struct ck_attribute
{
  ck_attribute_type_t type;
  void *value;
  unsigned long value_len;
};


struct ck_date
{
  unsigned char year[4];
  unsigned char month[2];
  unsigned char day[2];
};


typedef unsigned long ck_mechanism_type_t;

#define CKM_RSA_PKCS_KEY_PAIR_GEN	(0UL)
#define CKM_RSA_PKCS			(1UL)
#define CKM_RSA_9796			(2UL)
#define CKM_RSA_X_509			(3UL)
#define CKM_MD2_RSA_PKCS		(4UL)
#define CKM_MD5_RSA_PKCS		(5UL)
#define CKM_SHA1_RSA_PKCS		(6UL)
#define CKM_RIPEMD128_RSA_PKCS		(7UL)
#define CKM_RIPEMD160_RSA_PKCS		(8UL)
#define CKM_RSA_PKCS_OAEP		(9UL)
#define CKM_RSA_X9_31_KEY_PAIR_GEN	(0xaUL)
#define CKM_RSA_X9_31			(0xbUL)
#define CKM_SHA1_RSA_X9_31		(0xcUL)
#define CKM_RSA_PKCS_PSS		(0xdUL)
#define CKM_SHA1_RSA_PKCS_PSS		(0xeUL)
#define CKM_DSA_KEY_PAIR_GEN		(0x10UL)
#define	CKM_DSA				(0x11UL)
#define CKM_DSA_SHA1			(0x12UL)
#define CKM_DSA_SHA224			(0x13UL)
#define CKM_DSA_SHA256			(0x14UL)
#define CKM_DSA_SHA384			(0x15UL)
#define CKM_DSA_SHA512			(0x16UL)
#define CKM_DH_PKCS_KEY_PAIR_GEN	(0x20UL)
#define CKM_DH_PKCS_DERIVE		(0x21UL)
#define	CKM_X9_42_DH_KEY_PAIR_GEN	(0x30UL)
#define CKM_X9_42_DH_DERIVE		(0x31UL)
#define CKM_X9_42_DH_HYBRID_DERIVE	(0x32UL)
#define CKM_X9_42_MQV_DERIVE		(0x33UL)
#define CKM_SHA256_RSA_PKCS		(0x40UL)
#define CKM_SHA384_RSA_PKCS		(0x41UL)
#define CKM_SHA512_RSA_PKCS		(0x42UL)
#define CKM_SHA256_RSA_PKCS_PSS		(0x43UL)
#define CKM_SHA384_RSA_PKCS_PSS		(0x44UL)
#define CKM_SHA512_RSA_PKCS_PSS		(0x45UL)
#define CKM_SHA512_224			(0x48UL)
#define CKM_SHA512_224_HMAC		(0x49UL)
#define CKM_SHA512_224_HMAC_GENERAL	(0x4aUL)
#define CKM_SHA512_224_KEY_DERIVATION	(0x4bUL)
#define CKM_SHA512_256			(0x4cUL)
#define CKM_SHA512_256_HMAC		(0x4dUL)
#define CKM_SHA512_256_HMAC_GENERAL	(0x4eUL)
#define CKM_SHA512_256_KEY_DERIVATION	(0x4fUL)
#define CKM_SHA512_T			(0x50UL)
#define CKM_SHA512_T_HMAC		(0x51UL)
#define CKM_SHA512_T_HMAC_GENERAL	(0x52UL)
#define CKM_SHA512_T_KEY_DERIVATION	(0x53UL)
#define CKM_RC2_KEY_GEN			(0x100UL)
#define CKM_RC2_ECB			(0x101UL)
#define	CKM_RC2_CBC			(0x102UL)
#define	CKM_RC2_MAC			(0x103UL)
#define CKM_RC2_MAC_GENERAL		(0x104UL)
#define CKM_RC2_CBC_PAD			(0x105UL)
#define CKM_RC4_KEY_GEN			(0x110UL)
#define CKM_RC4				(0x111UL)
#define CKM_DES_KEY_GEN			(0x120UL)
#define CKM_DES_ECB			(0x121UL)
#define CKM_DES_CBC			(0x122UL)
#define CKM_DES_MAC			(0x123UL)
#define CKM_DES_MAC_GENERAL		(0x124UL)
#define CKM_DES_CBC_PAD			(0x125UL)
#define CKM_DES2_KEY_GEN		(0x130UL)
#define CKM_DES3_KEY_GEN		(0x131UL)
#define CKM_DES3_ECB			(0x132UL)
#define CKM_DES3_CBC			(0x133UL)
#define CKM_DES3_MAC			(0x134UL)
#define CKM_DES3_MAC_GENERAL		(0x135UL)
#define CKM_DES3_CBC_PAD		(0x136UL)
#define CKM_DES3_CMAC_GENERAL		(0x137UL)
#define CKM_DES3_CMAC			(0x138UL)
#define CKM_CDMF_KEY_GEN		(0x140UL)
#define CKM_CDMF_ECB			(0x141UL)
#define CKM_CDMF_CBC			(0x142UL)
#define CKM_CDMF_MAC			(0x143UL)
#define CKM_CDMF_MAC_GENERAL		(0x144UL)
#define CKM_CDMF_CBC_PAD		(0x145UL)
#define CKM_DES_OFB64			(0x150UL)
#define CKM_DES_OFB8			(0x151UL)
#define CKM_DES_CFB64			(0x152UL)
#define CKM_DES_CFB8			(0x153UL)
#define CKM_MD2				(0x200UL)
#define CKM_MD2_HMAC			(0x201UL)
#define CKM_MD2_HMAC_GENERAL		(0x202UL)
#define CKM_MD5				(0x210UL)
#define CKM_MD5_HMAC			(0x211UL)
#define CKM_MD5_HMAC_GENERAL		(0x212UL)
#define CKM_SHA_1			(0x220UL)
#define CKM_SHA_1_HMAC			(0x221UL)
#define CKM_SHA_1_HMAC_GENERAL		(0x222UL)
#define CKM_RIPEMD128			(0x230UL)
#define CKM_RIPEMD128_HMAC		(0x231UL)
#define CKM_RIPEMD128_HMAC_GENERAL	(0x232UL)
#define CKM_RIPEMD160			(0x240UL)
#define CKM_RIPEMD160_HMAC		(0x241UL)
#define CKM_RIPEMD160_HMAC_GENERAL	(0x242UL)
#define CKM_SHA256			(0x250UL)
#define CKM_SHA256_HMAC			(0x251UL)
#define CKM_SHA256_HMAC_GENERAL		(0x252UL)
#define CKM_SHA384			(0x260UL)
#define CKM_SHA384_HMAC			(0x261UL)
#define CKM_SHA384_HMAC_GENERAL		(0x262UL)
#define CKM_SHA512			(0x270UL)
#define CKM_SHA512_HMAC			(0x271UL)
#define CKM_SHA512_HMAC_GENERAL		(0x272UL)
#define CKM_SECURID_KEY_GEN             (0x280UL)
#define CKM_SECURID                     (0x282UL)
#define CKM_HOTP_KEY_GEN                (0x290UL)
#define CKM_HOTP                        (0x291UL)
#define CKM_ACTI                        (0x2a0UL)
#define CKM_ACTI_KEY_GEN                (0x2a1UL)
#define CKM_CAST_KEY_GEN		(0x300UL)
#define CKM_CAST_ECB			(0x301UL)
#define CKM_CAST_CBC			(0x302UL)
#define CKM_CAST_MAC			(0x303UL)
#define CKM_CAST_MAC_GENERAL		(0x304UL)
#define CKM_CAST_CBC_PAD		(0x305UL)
#define CKM_CAST3_KEY_GEN		(0x310UL)
#define CKM_CAST3_ECB			(0x311UL)
#define CKM_CAST3_CBC			(0x312UL)
#define CKM_CAST3_MAC			(0x313UL)
#define CKM_CAST3_MAC_GENERAL		(0x314UL)
#define CKM_CAST3_CBC_PAD		(0x315UL)
#define CKM_CAST5_KEY_GEN		(0x320UL)
#define CKM_CAST128_KEY_GEN		(0x320UL)
#define CKM_CAST5_ECB			(0x321UL)
#define CKM_CAST128_ECB			(0x321UL)
#define CKM_CAST5_CBC			(0x322UL)
#define CKM_CAST128_CBC			(0x322UL)
#define CKM_CAST5_MAC			(0x323UL)
#define	CKM_CAST128_MAC			(0x323UL)
#define CKM_CAST5_MAC_GENERAL		(0x324UL)
#define CKM_CAST128_MAC_GENERAL		(0x324UL)
#define CKM_CAST5_CBC_PAD		(0x325UL)
#define CKM_CAST128_CBC_PAD		(0x325UL)
#define CKM_RC5_KEY_GEN			(0x330UL)
#define CKM_RC5_ECB			(0x331UL)
#define CKM_RC5_CBC			(0x332UL)
#define CKM_RC5_MAC			(0x333UL)
#define CKM_RC5_MAC_GENERAL		(0x334UL)
#define CKM_RC5_CBC_PAD			(0x335UL)
#define CKM_IDEA_KEY_GEN		(0x340UL)
#define CKM_IDEA_ECB			(0x341UL)
#define	CKM_IDEA_CBC			(0x342UL)
#define CKM_IDEA_MAC			(0x343UL)
#define CKM_IDEA_MAC_GENERAL		(0x344UL)
#define CKM_IDEA_CBC_PAD		(0x345UL)
#define CKM_GENERIC_SECRET_KEY_GEN	(0x350UL)
#define CKM_CONCATENATE_BASE_AND_KEY	(0x360UL)
#define CKM_CONCATENATE_BASE_AND_DATA	(0x362UL)
#define CKM_CONCATENATE_DATA_AND_BASE	(0x363UL)
#define CKM_XOR_BASE_AND_DATA		(0x364UL)
#define CKM_EXTRACT_KEY_FROM_KEY	(0x365UL)
#define CKM_SSL3_PRE_MASTER_KEY_GEN	(0x370UL)
#define CKM_SSL3_MASTER_KEY_DERIVE	(0x371UL)
#define CKM_SSL3_KEY_AND_MAC_DERIVE	(0x372UL)
#define CKM_SSL3_MASTER_KEY_DERIVE_DH	(0x373UL)
#define CKM_TLS_PRE_MASTER_KEY_GEN	(0x374UL)
#define CKM_TLS_MASTER_KEY_DERIVE	(0x375UL)
#define CKM_TLS_KEY_AND_MAC_DERIVE	(0x376UL)
#define CKM_TLS_MASTER_KEY_DERIVE_DH	(0x377UL)
#define CKM_TLS_PRF			(0x378UL)
#define CKM_SSL3_MD5_MAC		(0x380UL)
#define CKM_SSL3_SHA1_MAC		(0x381UL)
#define CKM_MD5_KEY_DERIVATION		(0x390UL)
#define CKM_MD2_KEY_DERIVATION		(0x391UL)
#define CKM_SHA1_KEY_DERIVATION		(0x392UL)
#define CKM_SHA256_KEY_DERIVATION	(0x393UL)
#define CKM_SHA384_KEY_DERIVATION	(0x394UL)
#define CKM_SHA512_KEY_DERIVATION	(0x395UL)
#define CKM_PBE_MD2_DES_CBC		(0x3a0UL)
#define CKM_PBE_MD5_DES_CBC		(0x3a1UL)
#define CKM_PBE_MD5_CAST_CBC		(0x3a2UL)
#define CKM_PBE_MD5_CAST3_CBC		(0x3a3UL)
#define CKM_PBE_MD5_CAST5_CBC		(0x3a4UL)
#define CKM_PBE_MD5_CAST128_CBC		(0x3a4UL)
#define CKM_PBE_SHA1_CAST5_CBC		(0x3a5UL)
#define CKM_PBE_SHA1_CAST128_CBC	(0x3a5UL)
#define CKM_PBE_SHA1_RC4_128		(0x3a6UL)
#define CKM_PBE_SHA1_RC4_40		(0x3a7UL)
#define CKM_PBE_SHA1_DES3_EDE_CBC	(0x3a8UL)
#define CKM_PBE_SHA1_DES2_EDE_CBC	(0x3a9UL)
#define CKM_PBE_SHA1_RC2_128_CBC	(0x3aaUL)
#define CKM_PBE_SHA1_RC2_40_CBC		(0x3abUL)
#define CKM_PKCS5_PBKD2			(0x3b0UL)
#define CKM_PBA_SHA1_WITH_SHA1_HMAC	(0x3c0UL)
#define CKM_WTLS_PRE_MASTER_KEY_GEN	(0x3d0UL)
#define CKM_WTLS_MASTER_KEY_DERIVE	(0x3d1UL)
#define CKM_WTLS_MASTER_KEY_DERIVE_DH_ECC (0x3d2UL)
#define CKM_WTLS_PRF			(0x3d3UL)
#define CKM_WTLS_SERVER_KEY_AND_MAC_DERIVE (0x3d4UL)
#define CKM_WTLS_CLIENT_KEY_AND_MAC_DERIVE (0x3d5UL)
#define CKM_TLS10_MAC_SERVER		(0x3d6UL)
#define CKM_TLS10_MAC_CLIENT		(0x3d7UL)
#define CKM_TLS12_MAC			(0x3d8UL)
#define CKM_TLS12_KDF			(0x3d9UL)
#define CKM_TLS12_MASTER_KEY_DERIVE	(0x3e0UL)
#define CKM_TLS12_KEY_AND_MAC_DERIVE	(0x3e1UL)
#define CKM_TLS12_MASTER_KEY_DERIVE_DH	(0x3e2UL)
#define CKM_TLS12_KEY_SAFE_DERIVE	(0x3e3UL)
#define CKM_TLS_MAC			(0x3e4UL)
#define CKM_TLS_KDF			(0x3e5UL)
#define CKM_KEY_WRAP_LYNKS		(0x400UL)
#define CKM_KEY_WRAP_SET_OAEP		(0x401UL)
#define CKM_CMS_SIG			(0x500UL)
#define CKM_KIP_DERIVE			(0x510UL)
#define CKM_KIP_WRAP			(0x511UL)
#define CKM_KIP_MAC			(0x512UL)
#define CKM_ARIA_KEY_GEN		(0x560UL)
#define CKM_ARIA_ECB			(0x561UL)
#define CKM_ARIA_CBC			(0x562UL)
#define CKM_ARIA_MAC			(0x563UL)
#define CKM_ARIA_MAC_GENERAL		(0x564UL)
#define CKM_ARIA_CBC_PAD		(0x565UL)
#define CKM_ARIA_ECB_ENCRYPT_DATA	(0x566UL)
#define CKM_ARIA_CBC_ENCRYPT_DATA	(0x567UL)
#define CKM_SEED_KEY_GEN		(0x650UL)
#define CKM_SEED_ECB			(0x651UL)
#define CKM_SEED_CBC			(0x652UL)
#define CKM_SEED_MAC			(0x653UL)
#define CKM_SEED_MAC_GENERAL		(0x654UL)
#define CKM_SEED_CBC_PAD		(0x655UL)
#define CKM_SEED_ECB_ENCRYPT_DATA	(0x656UL)
#define CKM_SEED_CBC_ENCRYPT_DATA	(0x657UL)
#define CKM_SKIPJACK_KEY_GEN		(0x1000UL)
#define CKM_SKIPJACK_ECB64		(0x1001UL)
#define CKM_SKIPJACK_CBC64		(0x1002UL)
#define CKM_SKIPJACK_OFB64		(0x1003UL)
#define CKM_SKIPJACK_CFB64		(0x1004UL)
#define CKM_SKIPJACK_CFB32		(0x1005UL)
#define CKM_SKIPJACK_CFB16		(0x1006UL)
#define CKM_SKIPJACK_CFB8		(0x1007UL)
#define CKM_SKIPJACK_WRAP		(0x1008UL)
#define CKM_SKIPJACK_PRIVATE_WRAP	(0x1009UL)
#define CKM_SKIPJACK_RELAYX		(0x100aUL)
#define CKM_KEA_KEY_PAIR_GEN		(0x1010UL)
#define CKM_KEA_KEY_DERIVE		(0x1011UL)
#define CKM_FORTEZZA_TIMESTAMP		(0x1020UL)
#define CKM_BATON_KEY_GEN		(0x1030UL)
#define CKM_BATON_ECB128		(0x1031UL)
#define CKM_BATON_ECB96			(0x1032UL)
#define CKM_BATON_CBC128		(0x1033UL)
#define CKM_BATON_COUNTER		(0x1034UL)
#define CKM_BATON_SHUFFLE		(0x1035UL)
#define CKM_BATON_WRAP			(0x1036UL)
#define CKM_ECDSA_KEY_PAIR_GEN		(0x1040UL)
#define CKM_EC_KEY_PAIR_GEN		(0x1040UL)
#define CKM_ECDSA			(0x1041UL)
#define CKM_ECDSA_SHA1			(0x1042UL)
#define CKM_ECDSA_SHA224		(0x1043UL)
#define CKM_ECDSA_SHA256		(0x1044UL)
#define CKM_ECDSA_SHA384		(0x1045UL)
#define CKM_ECDSA_SHA512		(0x1046UL)
#define CKM_ECDH1_DERIVE		(0x1050UL)
#define CKM_ECDH1_COFACTOR_DERIVE	(0x1051UL)
#define CKM_ECMQV_DERIVE		(0x1052UL)
#define CKM_ECDH_AES_KEY_WRAP		(0x1053UL)
#define CKM_RSA_AES_KEY_WRAP		(0x1054UL)
#define CKM_JUNIPER_KEY_GEN		(0x1060UL)
#define CKM_JUNIPER_ECB128		(0x1061UL)
#define CKM_JUNIPER_CBC128		(0x1062UL)
#define CKM_JUNIPER_COUNTER		(0x1063UL)
#define CKM_JUNIPER_SHUFFLE		(0x1064UL)
#define CKM_JUNIPER_WRAP		(0x1065UL)
#define CKM_FASTHASH			(0x1070UL)
#define CKM_AES_KEY_GEN			(0x1080UL)
#define CKM_AES_ECB			(0x1081UL)
#define CKM_AES_CBC			(0x1082UL)
#define CKM_AES_MAC			(0x1083UL)
#define CKM_AES_MAC_GENERAL		(0x1084UL)
#define CKM_AES_CBC_PAD			(0x1085UL)
#define CKM_AES_CTR			(0x1086UL)
#define CKM_AES_GCM			(0x1087UL)
#define CKM_AES_CCM			(0x1088UL)
#define CKM_AES_CTS			(0x1089UL)
#define CKM_AES_CMAC			(0x108aUL)
#define CKM_AES_CMAC_GENERAL		(0x108bUL)
#define CKM_AES_XCBC_MAC		(0x108cUL)
#define CKM_AES_XCBC_MAC_96		(0x108dUL)
#define CKM_AES_GMAC			(0x108eUL)
#define CKM_BLOWFISH_KEY_GEN		(0x1090UL)
#define CKM_BLOWFISH_CBC		(0x1091UL)
#define CKM_TWOFISH_KEY_GEN		(0x1092UL)
#define CKM_TWOFISH_CBC			(0x1093UL)
#define CKM_BLOWFISH_CBC_PAD		(0x1094UL)
#define CKM_TWOFISH_CBC_PAD		(0x1095UL)
#define CKM_DES_ECB_ENCRYPT_DATA	(0x1100UL)
#define CKM_DES_CBC_ENCRYPT_DATA	(0x1101UL)
#define CKM_DES3_ECB_ENCRYPT_DATA	(0x1102UL)
#define CKM_DES3_CBC_ENCRYPT_DATA	(0x1103UL)
#define CKM_AES_ECB_ENCRYPT_DATA	(0x1104UL)
#define CKM_AES_CBC_ENCRYPT_DATA	(0x1105UL)
#define CKM_GOSTR3410_KEY_PAIR_GEN	(0x1200UL)
#define CKM_GOSTR3410			(0x1201UL)
#define CKM_GOSTR3410_WITH_GOSTR3411	(0x1202UL)
#define CKM_GOSTR3410_KEY_WRAP		(0x1203UL)
#define CKM_GOSTR3410_DERIVE		(0x1204UL)
#define CKM_GOSTR3411			(0x1210UL)
#define CKM_GOSTR3411_HMAC		(0x1211UL)
#define CKM_GOST28147_KEY_GEN		(0x1220UL)
#define CKM_GOST28147_ECB		(0x1221UL)
#define CKM_GOST28147			(0x1222UL)
#define CKM_GOST28147_MAC		(0x1223UL)
#define CKM_GOST28147_KEY_WRAP		(0x1224UL)
#define CKM_DSA_PARAMETER_GEN		(0x2000UL)
#define CKM_DH_PKCS_PARAMETER_GEN	(0x2001UL)
#define CKM_X9_42_DH_PARAMETER_GEN	(0x2002UL)
#define CKM_DSA_PROBABLISTIC_PARAMETER_GEN	(0x2003UL)
#define CKM_DSA_SHAWE_TAYLOR_PARAMETER_GEN	(0x2004UL)
#define CKM_AES_OFB			(0x2104UL)
#define CKM_AES_CFB64			(0x2105UL)
#define CKM_AES_CFB8			(0x2106UL)
#define CKM_AES_CFB128			(0x2107UL)
#define CKM_AES_CFB1			(0x2108UL)

#define CKM_VENDOR_DEFINED		((unsigned long) (1UL << 31))

/* Amendments */
#define CKM_SHA224			(0x255UL)
#define CKM_SHA224_HMAC			(0x256UL)
#define CKM_SHA224_HMAC_GENERAL		(0x257UL)
#define CKM_SHA224_RSA_PKCS		(0x46UL)
#define CKM_SHA224_RSA_PKCS_PSS		(0x47UL)
#define CKM_SHA224_KEY_DERIVATION	(0x396UL)

#define CKM_CAMELLIA_KEY_GEN		(0x550UL)
#define CKM_CAMELLIA_ECB		(0x551UL)
#define CKM_CAMELLIA_CBC		(0x552UL)
#define CKM_CAMELLIA_MAC		(0x553UL)
#define CKM_CAMELLIA_MAC_GENERAL	(0x554UL)
#define CKM_CAMELLIA_CBC_PAD		(0x555UL)
#define CKM_CAMELLIA_ECB_ENCRYPT_DATA	(0x556UL)
#define CKM_CAMELLIA_CBC_ENCRYPT_DATA	(0x557UL)
#define CKM_CAMELLIA_CTR		(0x558UL)

#define CKM_AES_KEY_WRAP		(0x2109UL)
#define CKM_AES_KEY_WRAP_PAD		(0x210aUL)

#define CKM_RSA_PKCS_TPM_1_1		(0x4001UL)
#define CKM_RSA_PKCS_OAEP_TPM_1_1	(0x4002UL)

/* From version 3.0 */
#define CKM_EC_EDWARDS_KEY_PAIR_GEN	(0x1055UL)
#define CKM_EDDSA			(0x1057UL)

/* Attribute and other constants related to OTP */
#define CK_OTP_FORMAT_DECIMAL		(0UL)
#define CK_OTP_FORMAT_HEXADECIMAL	(1UL)
#define CK_OTP_FORMAT_ALPHANUMERIC	(2UL)
#define CK_OTP_FORMAT_BINARY		(3UL)
#define CK_OTP_PARAM_IGNORED		(0UL)
#define CK_OTP_PARAM_OPTIONAL		(1UL)
#define CK_OTP_PARAM_MANDATORY		(2UL)

#define CK_OTP_VALUE			(0UL)
#define CK_OTP_PIN			(1UL)
#define CK_OTP_CHALLENGE		(2UL)
#define CK_OTP_TIME			(3UL)
#define CK_OTP_COUNTER			(4UL)
#define CK_OTP_FLAGS			(5UL)
#define CK_OTP_OUTPUT_LENGTH		(6UL)
#define CK_OTP_FORMAT			(7UL)

/* OTP mechanism flags */
#define CKF_NEXT_OTP			(0x01UL)
#define CKF_EXCLUDE_TIME		(0x02UL)
#define CKF_EXCLUDE_COUNTER		(0x04UL)
#define CKF_EXCLUDE_CHALLENGE		(0x08UL)
#define CKF_EXCLUDE_PIN			(0x10UL)
#define CKF_USER_FRIENDLY_OTP		(0x20UL)

#define CKN_OTP_CHANGED			(0x01UL)

struct ck_mechanism
{
  ck_mechanism_type_t mechanism;
  void *parameter;
  unsigned long parameter_len;
};


struct ck_mechanism_info
{
  unsigned long min_key_size;
  unsigned long max_key_size;
  ck_flags_t flags;
};

typedef unsigned long ck_param_type;

typedef struct ck_otp_param {
   ck_param_type type;
   void *value;
   unsigned long value_len;
} ck_otp_param;

typedef struct ck_otp_params {
   struct ck_otp_param *params;
   unsigned long count;
} ck_otp_params;

typedef struct ck_otp_signature_info
{
  struct ck_otp_param *params;
  unsigned long count;
} ck_otp_signature_info;

#define CKG_MGF1_SHA1 0x00000001UL
#define CKG_MGF1_SHA224 0x00000005UL
#define CKG_MGF1_SHA256 0x00000002UL
#define CKG_MGF1_SHA384 0x00000003UL
#define CKG_MGF1_SHA512 0x00000004UL

typedef unsigned long ck_rsa_pkcs_mgf_type_t;
typedef ck_rsa_pkcs_mgf_type_t * CK_RSA_PKCS_MGF_TYPE_PTR;

struct ck_rsa_pkcs_pss_params {
  ck_mechanism_type_t hash_alg;
  ck_rsa_pkcs_mgf_type_t mgf;
  unsigned long s_len;
};

typedef unsigned long ck_rsa_pkcs_oaep_source_type_t;

struct ck_rsa_pkcs_oaep_params {
  ck_mechanism_type_t hash_alg;
  ck_rsa_pkcs_mgf_type_t mgf;
  ck_rsa_pkcs_oaep_source_type_t source;
  void *source_data;
  unsigned long source_data_len;
};

struct ck_aes_ctr_params {
  unsigned long counter_bits;
  unsigned char cb[16];
};

struct ck_gcm_params {
  unsigned char *iv_ptr;
  unsigned long iv_len;
  unsigned long iv_bits;
  unsigned char *aad_ptr;
  unsigned long aad_len;
  unsigned long tag_bits;
};


/* The following EC Key Derivation Functions are defined */
#define CKD_NULL			(0x01UL)
#define CKD_SHA1_KDF			(0x02UL)

/* The following X9.42 DH key derivation functions are defined */
#define CKD_SHA1_KDF_ASN1		(0x03UL)
#define CKD_SHA1_KDF_CONCATENATE	(0x04UL)
#define CKD_SHA224_KDF			(0x05UL)
#define CKD_SHA256_KDF			(0x06UL)
#define CKD_SHA384_KDF			(0x07UL)
#define CKD_SHA512_KDF			(0x08UL)
#define CKD_CPDIVERSIFY_KDF		(0x09UL)

typedef unsigned long ck_ec_kdf_t;

struct ck_ecdh1_derive_params {
  ck_ec_kdf_t kdf;
  unsigned long shared_data_len;
  unsigned char *shared_data;
  unsigned long public_data_len;
  unsigned char *public_data;
};

struct ck_key_derivation_string_data {
  unsigned char *string_data;
  unsigned long string_data_len;
};

struct ck_des_cbc_encrypt_data_params {
  unsigned char iv[8];
  unsigned char *data_params;
  unsigned long length;
};

struct ck_aes_cbc_encrypt_data_params {
  unsigned char iv[16];
  unsigned char *data_params;
  unsigned long length;
};

#define CKF_HW			(1UL << 0)
#define CKF_ENCRYPT		(1UL << 8)
#define CKF_DECRYPT		(1UL << 9)
#define CKF_DIGEST		(1UL << 10)
#define CKF_SIGN		(1UL << 11)
#define CKF_SIGN_RECOVER	(1UL << 12)
#define CKF_VERIFY		(1UL << 13)
#define CKF_VERIFY_RECOVER	(1UL << 14)
#define CKF_GENERATE		(1UL << 15)
#define CKF_GENERATE_KEY_PAIR	(1UL << 16)
#define CKF_WRAP		(1UL << 17)
#define CKF_UNWRAP		(1UL << 18)
#define CKF_DERIVE		(1UL << 19)
#define CKF_EXTENSION		((unsigned long) (1UL << 31))

#define CKF_EC_F_P		(1UL << 20)
#define CKF_EC_NAMEDCURVE	(1UL << 23)
#define CKF_EC_UNCOMPRESS	(1UL << 24)
#define CKF_EC_COMPRESS		(1UL << 25)


/* Flags for C_WaitForSlotEvent.  */
#define CKF_DONT_BLOCK				(1UL)


typedef unsigned long ck_rv_t;


typedef ck_rv_t (*ck_notify_t) (ck_session_handle_t session,
				ck_notification_t event, void *application);

/* Forward reference.  */
struct ck_function_list;

#define _CK_DECLARE_FUNCTION(name, args)	\
typedef ck_rv_t (*CK_ ## name) args;		\
ck_rv_t CK_SPEC name args

_CK_DECLARE_FUNCTION (C_Initialize, (void *init_args));
_CK_DECLARE_FUNCTION (C_Finalize, (void *reserved));
_CK_DECLARE_FUNCTION (C_GetInfo, (struct ck_info *info));
_CK_DECLARE_FUNCTION (C_GetFunctionList,
		      (struct ck_function_list **function_list));

_CK_DECLARE_FUNCTION (C_GetSlotList,
		      (unsigned char token_present, ck_slot_id_t *slot_list,
		       unsigned long *count));
_CK_DECLARE_FUNCTION (C_GetSlotInfo,
		      (ck_slot_id_t slot_id, struct ck_slot_info *info));
_CK_DECLARE_FUNCTION (C_GetTokenInfo,
		      (ck_slot_id_t slot_id, struct ck_token_info *info));
_CK_DECLARE_FUNCTION (C_WaitForSlotEvent,
		      (ck_flags_t flags, ck_slot_id_t *slot, void *reserved));
_CK_DECLARE_FUNCTION (C_GetMechanismList,
		      (ck_slot_id_t slot_id,
		       ck_mechanism_type_t *mechanism_list,
		       unsigned long *count));
_CK_DECLARE_FUNCTION (C_GetMechanismInfo,
		      (ck_slot_id_t slot_id, ck_mechanism_type_t type,
		       struct ck_mechanism_info *info));
_CK_DECLARE_FUNCTION (C_InitToken,
		      (ck_slot_id_t slot_id, unsigned char *pin,
		       unsigned long pin_len, unsigned char *label));
_CK_DECLARE_FUNCTION (C_InitPIN,
		      (ck_session_handle_t session, unsigned char *pin,
		       unsigned long pin_len));
_CK_DECLARE_FUNCTION (C_SetPIN,
		      (ck_session_handle_t session, unsigned char *old_pin,
		       unsigned long old_len, unsigned char *new_pin,
		       unsigned long new_len));

_CK_DECLARE_FUNCTION (C_OpenSession,
		      (ck_slot_id_t slot_id, ck_flags_t flags,
		       void *application, ck_notify_t notify,
		       ck_session_handle_t *session));
_CK_DECLARE_FUNCTION (C_CloseSession, (ck_session_handle_t session));
_CK_DECLARE_FUNCTION (C_CloseAllSessions, (ck_slot_id_t slot_id));
_CK_DECLARE_FUNCTION (C_GetSessionInfo,
		      (ck_session_handle_t session,
		       struct ck_session_info *info));
_CK_DECLARE_FUNCTION (C_GetOperationState,
		      (ck_session_handle_t session,
		       unsigned char *operation_state,
		       unsigned long *operation_state_len));
_CK_DECLARE_FUNCTION (C_SetOperationState,
		      (ck_session_handle_t session,
		       unsigned char *operation_state,
		       unsigned long operation_state_len,
		       ck_object_handle_t encryption_key,
		       ck_object_handle_t authentiation_key));
_CK_DECLARE_FUNCTION (C_Login,
		      (ck_session_handle_t session, ck_user_type_t user_type,
		       unsigned char *pin, unsigned long pin_len));
_CK_DECLARE_FUNCTION (C_Logout, (ck_session_handle_t session));

_CK_DECLARE_FUNCTION (C_CreateObject,
		      (ck_session_handle_t session,
		       struct ck_attribute *templ,
		       unsigned long count, ck_object_handle_t *object));
_CK_DECLARE_FUNCTION (C_CopyObject,
		      (ck_session_handle_t session, ck_object_handle_t object,
		       struct ck_attribute *templ, unsigned long count,
		       ck_object_handle_t *new_object));
_CK_DECLARE_FUNCTION (C_DestroyObject,
		      (ck_session_handle_t session,
		       ck_object_handle_t object));
_CK_DECLARE_FUNCTION (C_GetObjectSize,
		      (ck_session_handle_t session,
		       ck_object_handle_t object,
		       unsigned long *size));
_CK_DECLARE_FUNCTION (C_GetAttributeValue,
		      (ck_session_handle_t session,
		       ck_object_handle_t object,
		       struct ck_attribute *templ,
		       unsigned long count));
_CK_DECLARE_FUNCTION (C_SetAttributeValue,
		      (ck_session_handle_t session,
		       ck_object_handle_t object,
		       struct ck_attribute *templ,
		       unsigned long count));
_CK_DECLARE_FUNCTION (C_FindObjectsInit,
		      (ck_session_handle_t session,
		       struct ck_attribute *templ,
		       unsigned long count));
_CK_DECLARE_FUNCTION (C_FindObjects,
		      (ck_session_handle_t session,
		       ck_object_handle_t *object,
		       unsigned long max_object_count,
		       unsigned long *object_count));
_CK_DECLARE_FUNCTION (C_FindObjectsFinal,
		      (ck_session_handle_t session));

_CK_DECLARE_FUNCTION (C_EncryptInit,
		      (ck_session_handle_t session,
		       struct ck_mechanism *mechanism,
		       ck_object_handle_t key));
_CK_DECLARE_FUNCTION (C_Encrypt,
		      (ck_session_handle_t session,
		       unsigned char *data, unsigned long data_len,
		       unsigned char *encrypted_data,
		       unsigned long *encrypted_data_len));
_CK_DECLARE_FUNCTION (C_EncryptUpdate,
		      (ck_session_handle_t session,
		       unsigned char *part, unsigned long part_len,
		       unsigned char *encrypted_part,
		       unsigned long *encrypted_part_len));
_CK_DECLARE_FUNCTION (C_EncryptFinal,
		      (ck_session_handle_t session,
		       unsigned char *last_encrypted_part,
		       unsigned long *last_encrypted_part_len));

_CK_DECLARE_FUNCTION (C_DecryptInit,
		      (ck_session_handle_t session,
		       struct ck_mechanism *mechanism,
		       ck_object_handle_t key));
_CK_DECLARE_FUNCTION (C_Decrypt,
		      (ck_session_handle_t session,
		       unsigned char *encrypted_data,
		       unsigned long encrypted_data_len,
		       unsigned char *data, unsigned long *data_len));
_CK_DECLARE_FUNCTION (C_DecryptUpdate,
		      (ck_session_handle_t session,
		       unsigned char *encrypted_part,
		       unsigned long encrypted_part_len,
		       unsigned char *part, unsigned long *part_len));
_CK_DECLARE_FUNCTION (C_DecryptFinal,
		      (ck_session_handle_t session,
		       unsigned char *last_part,
		       unsigned long *last_part_len));

_CK_DECLARE_FUNCTION (C_DigestInit,
		      (ck_session_handle_t session,
		       struct ck_mechanism *mechanism));
_CK_DECLARE_FUNCTION (C_Digest,
		      (ck_session_handle_t session,
		       unsigned char *data, unsigned long data_len,
		       unsigned char *digest,
		       unsigned long *digest_len));
_CK_DECLARE_FUNCTION (C_DigestUpdate,
		      (ck_session_handle_t session,
		       unsigned char *part, unsigned long part_len));
_CK_DECLARE_FUNCTION (C_DigestKey,
		      (ck_session_handle_t session, ck_object_handle_t key));
_CK_DECLARE_FUNCTION (C_DigestFinal,
		      (ck_session_handle_t session,
		       unsigned char *digest,
		       unsigned long *digest_len));

_CK_DECLARE_FUNCTION (C_SignInit,
		      (ck_session_handle_t session,
		       struct ck_mechanism *mechanism,
		       ck_object_handle_t key));
_CK_DECLARE_FUNCTION (C_Sign,
		      (ck_session_handle_t session,
		       unsigned char *data, unsigned long data_len,
		       unsigned char *signature,
		       unsigned long *signature_len));
_CK_DECLARE_FUNCTION (C_SignUpdate,
		      (ck_session_handle_t session,
		       unsigned char *part, unsigned long part_len));
_CK_DECLARE_FUNCTION (C_SignFinal,
		      (ck_session_handle_t session,
		       unsigned char *signature,
		       unsigned long *signature_len));
_CK_DECLARE_FUNCTION (C_SignRecoverInit,
		      (ck_session_handle_t session,
		       struct ck_mechanism *mechanism,
		       ck_object_handle_t key));
_CK_DECLARE_FUNCTION (C_SignRecover,
		      (ck_session_handle_t session,
		       unsigned char *data, unsigned long data_len,
		       unsigned char *signature,
		       unsigned long *signature_len));

_CK_DECLARE_FUNCTION (C_VerifyInit,
		      (ck_session_handle_t session,
		       struct ck_mechanism *mechanism,
		       ck_object_handle_t key));
_CK_DECLARE_FUNCTION (C_Verify,
		      (ck_session_handle_t session,
		       unsigned char *data, unsigned long data_len,
		       unsigned char *signature,
		       unsigned long signature_len));
_CK_DECLARE_FUNCTION (C_VerifyUpdate,
		      (ck_session_handle_t session,
		       unsigned char *part, unsigned long part_len));
_CK_DECLARE_FUNCTION (C_VerifyFinal,
		      (ck_session_handle_t session,
		       unsigned char *signature,
		       unsigned long signature_len));
_CK_DECLARE_FUNCTION (C_VerifyRecoverInit,
		      (ck_session_handle_t session,
		       struct ck_mechanism *mechanism,
		       ck_object_handle_t key));
_CK_DECLARE_FUNCTION (C_VerifyRecover,
		      (ck_session_handle_t session,
		       unsigned char *signature,
		       unsigned long signature_len,
		       unsigned char *data,
		       unsigned long *data_len));

_CK_DECLARE_FUNCTION (C_DigestEncryptUpdate,
		      (ck_session_handle_t session,
		       unsigned char *part, unsigned long part_len,
		       unsigned char *encrypted_part,
		       unsigned long *encrypted_part_len));
_CK_DECLARE_FUNCTION (C_DecryptDigestUpdate,
		      (ck_session_handle_t session,
		       unsigned char *encrypted_part,
		       unsigned long encrypted_part_len,
		       unsigned char *part,
		       unsigned long *part_len));
_CK_DECLARE_FUNCTION (C_SignEncryptUpdate,
		      (ck_session_handle_t session,
		       unsigned char *part, unsigned long part_len,
		       unsigned char *encrypted_part,
		       unsigned long *encrypted_part_len));
_CK_DECLARE_FUNCTION (C_DecryptVerifyUpdate,
		      (ck_session_handle_t session,
		       unsigned char *encrypted_part,
		       unsigned long encrypted_part_len,
		       unsigned char *part,
		       unsigned long *part_len));

_CK_DECLARE_FUNCTION (C_GenerateKey,
		      (ck_session_handle_t session,
		       struct ck_mechanism *mechanism,
		       struct ck_attribute *templ,
		       unsigned long count,
		       ck_object_handle_t *key));
_CK_DECLARE_FUNCTION (C_GenerateKeyPair,
		      (ck_session_handle_t session,
		       struct ck_mechanism *mechanism,
		       struct ck_attribute *public_key_template,
		       unsigned long public_key_attribute_count,
		       struct ck_attribute *private_key_template,
		       unsigned long private_key_attribute_count,
		       ck_object_handle_t *public_key,
		       ck_object_handle_t *private_key));
_CK_DECLARE_FUNCTION (C_WrapKey,
		      (ck_session_handle_t session,
		       struct ck_mechanism *mechanism,
		       ck_object_handle_t wrapping_key,
		       ck_object_handle_t key,
		       unsigned char *wrapped_key,
		       unsigned long *wrapped_key_len));
_CK_DECLARE_FUNCTION (C_UnwrapKey,
		      (ck_session_handle_t session,
		       struct ck_mechanism *mechanism,
		       ck_object_handle_t unwrapping_key,
		       unsigned char *wrapped_key,
		       unsigned long wrapped_key_len,
		       struct ck_attribute *templ,
		       unsigned long attribute_count,
		       ck_object_handle_t *key));
_CK_DECLARE_FUNCTION (C_DeriveKey,
		      (ck_session_handle_t session,
		       struct ck_mechanism *mechanism,
		       ck_object_handle_t base_key,
		       struct ck_attribute *templ,
		       unsigned long attribute_count,
		       ck_object_handle_t *key));

_CK_DECLARE_FUNCTION (C_SeedRandom,
		      (ck_session_handle_t session, unsigned char *seed,
		       unsigned long seed_len));
_CK_DECLARE_FUNCTION (C_GenerateRandom,
		      (ck_session_handle_t session,
		       unsigned char *random_data,
		       unsigned long random_len));

_CK_DECLARE_FUNCTION (C_GetFunctionStatus, (ck_session_handle_t session));
_CK_DECLARE_FUNCTION (C_CancelFunction, (ck_session_handle_t session));


struct ck_function_list
{
  struct ck_version version;
  CK_C_Initialize C_Initialize;
  CK_C_Finalize C_Finalize;
  CK_C_GetInfo C_GetInfo;
  CK_C_GetFunctionList C_GetFunctionList;
  CK_C_GetSlotList C_GetSlotList;
  CK_C_GetSlotInfo C_GetSlotInfo;
  CK_C_GetTokenInfo C_GetTokenInfo;
  CK_C_GetMechanismList C_GetMechanismList;
  CK_C_GetMechanismInfo C_GetMechanismInfo;
  CK_C_InitToken C_InitToken;
  CK_C_InitPIN C_InitPIN;
  CK_C_SetPIN C_SetPIN;
  CK_C_OpenSession C_OpenSession;
  CK_C_CloseSession C_CloseSession;
  CK_C_CloseAllSessions C_CloseAllSessions;
  CK_C_GetSessionInfo C_GetSessionInfo;
  CK_C_GetOperationState C_GetOperationState;
  CK_C_SetOperationState C_SetOperationState;
  CK_C_Login C_Login;
  CK_C_Logout C_Logout;
  CK_C_CreateObject C_CreateObject;
  CK_C_CopyObject C_CopyObject;
  CK_C_DestroyObject C_DestroyObject;
  CK_C_GetObjectSize C_GetObjectSize;
  CK_C_GetAttributeValue C_GetAttributeValue;
  CK_C_SetAttributeValue C_SetAttributeValue;
  CK_C_FindObjectsInit C_FindObjectsInit;
  CK_C_FindObjects C_FindObjects;
  CK_C_FindObjectsFinal C_FindObjectsFinal;
  CK_C_EncryptInit C_EncryptInit;
  CK_C_Encrypt C_Encrypt;
  CK_C_EncryptUpdate C_EncryptUpdate;
  CK_C_EncryptFinal C_EncryptFinal;
  CK_C_DecryptInit C_DecryptInit;
  CK_C_Decrypt C_Decrypt;
  CK_C_DecryptUpdate C_DecryptUpdate;
  CK_C_DecryptFinal C_DecryptFinal;
  CK_C_DigestInit C_DigestInit;
  CK_C_Digest C_Digest;
  CK_C_DigestUpdate C_DigestUpdate;
  CK_C_DigestKey C_DigestKey;
  CK_C_DigestFinal C_DigestFinal;
  CK_C_SignInit C_SignInit;
  CK_C_Sign C_Sign;
  CK_C_SignUpdate C_SignUpdate;
  CK_C_SignFinal C_SignFinal;
  CK_C_SignRecoverInit C_SignRecoverInit;
  CK_C_SignRecover C_SignRecover;
  CK_C_VerifyInit C_VerifyInit;
  CK_C_Verify C_Verify;
  CK_C_VerifyUpdate C_VerifyUpdate;
  CK_C_VerifyFinal C_VerifyFinal;
  CK_C_VerifyRecoverInit C_VerifyRecoverInit;
  CK_C_VerifyRecover C_VerifyRecover;
  CK_C_DigestEncryptUpdate C_DigestEncryptUpdate;
  CK_C_DecryptDigestUpdate C_DecryptDigestUpdate;
  CK_C_SignEncryptUpdate C_SignEncryptUpdate;
  CK_C_DecryptVerifyUpdate C_DecryptVerifyUpdate;
  CK_C_GenerateKey C_GenerateKey;
  CK_C_GenerateKeyPair C_GenerateKeyPair;
  CK_C_WrapKey C_WrapKey;
  CK_C_UnwrapKey C_UnwrapKey;
  CK_C_DeriveKey C_DeriveKey;
  CK_C_SeedRandom C_SeedRandom;
  CK_C_GenerateRandom C_GenerateRandom;
  CK_C_GetFunctionStatus C_GetFunctionStatus;
  CK_C_CancelFunction C_CancelFunction;
  CK_C_WaitForSlotEvent C_WaitForSlotEvent;
};


typedef ck_rv_t (*ck_createmutex_t) (void **mutex);
typedef ck_rv_t (*ck_destroymutex_t) (void *mutex);
typedef ck_rv_t (*ck_lockmutex_t) (void *mutex);
typedef ck_rv_t (*ck_unlockmutex_t) (void *mutex);


struct ck_c_initialize_args
{
  ck_createmutex_t create_mutex;
  ck_destroymutex_t destroy_mutex;
  ck_lockmutex_t lock_mutex;
  ck_unlockmutex_t unlock_mutex;
  ck_flags_t flags;
  void *reserved;
};


#define CKF_LIBRARY_CANT_CREATE_OS_THREADS	(1UL << 0)
#define CKF_OS_LOCKING_OK			(1UL << 1)

#define CKR_OK					(0UL)
#define CKR_CANCEL				(1UL)
#define CKR_HOST_MEMORY				(2UL)
#define CKR_SLOT_ID_INVALID			(3UL)
#define CKR_GENERAL_ERROR			(5UL)
#define CKR_FUNCTION_FAILED			(6UL)
#define CKR_ARGUMENTS_BAD			(7UL)
#define CKR_NO_EVENT				(8UL)
#define CKR_NEED_TO_CREATE_THREADS		(9UL)
#define CKR_CANT_LOCK				(0xaUL)
#define CKR_ATTRIBUTE_READ_ONLY			(0x10UL)
#define CKR_ATTRIBUTE_SENSITIVE			(0x11UL)
#define CKR_ATTRIBUTE_TYPE_INVALID		(0x12UL)
#define CKR_ATTRIBUTE_VALUE_INVALID		(0x13UL)
#define CKR_ACTION_PROHIBITED			(0x1BUL)
#define CKR_DATA_INVALID			(0x20UL)
#define CKR_DATA_LEN_RANGE			(0x21UL)
#define CKR_DEVICE_ERROR			(0x30UL)
#define CKR_DEVICE_MEMORY			(0x31UL)
#define CKR_DEVICE_REMOVED			(0x32UL)
#define CKR_ENCRYPTED_DATA_INVALID		(0x40UL)
#define CKR_ENCRYPTED_DATA_LEN_RANGE		(0x41UL)
#define CKR_FUNCTION_CANCELED			(0x50UL)
#define CKR_FUNCTION_NOT_PARALLEL		(0x51UL)
#define CKR_FUNCTION_NOT_SUPPORTED		(0x54UL)
#define CKR_KEY_HANDLE_INVALID			(0x60UL)
#define CKR_KEY_SIZE_RANGE			(0x62UL)
#define CKR_KEY_TYPE_INCONSISTENT		(0x63UL)
#define CKR_KEY_NOT_NEEDED			(0x64UL)
#define CKR_KEY_CHANGED				(0x65UL)
#define CKR_KEY_NEEDED				(0x66UL)
#define CKR_KEY_INDIGESTIBLE			(0x67UL)
#define CKR_KEY_FUNCTION_NOT_PERMITTED		(0x68UL)
#define CKR_KEY_NOT_WRAPPABLE			(0x69UL)
#define CKR_KEY_UNEXTRACTABLE			(0x6aUL)
#define CKR_MECHANISM_INVALID			(0x70UL)
#define CKR_MECHANISM_PARAM_INVALID		(0x71UL)
#define CKR_OBJECT_HANDLE_INVALID		(0x82UL)
#define CKR_OPERATION_ACTIVE			(0x90UL)
#define CKR_OPERATION_NOT_INITIALIZED		(0x91UL)
#define CKR_PIN_INCORRECT			(0xa0UL)
#define CKR_PIN_INVALID				(0xa1UL)
#define CKR_PIN_LEN_RANGE			(0xa2UL)
#define CKR_PIN_EXPIRED				(0xa3UL)
#define CKR_PIN_LOCKED				(0xa4UL)
#define CKR_SESSION_CLOSED			(0xb0UL)
#define CKR_SESSION_COUNT			(0xb1UL)
#define CKR_SESSION_HANDLE_INVALID		(0xb3UL)
#define CKR_SESSION_PARALLEL_NOT_SUPPORTED	(0xb4UL)
#define CKR_SESSION_READ_ONLY			(0xb5UL)
#define CKR_SESSION_EXISTS			(0xb6UL)
#define CKR_SESSION_READ_ONLY_EXISTS		(0xb7UL)
#define CKR_SESSION_READ_WRITE_SO_EXISTS	(0xb8UL)
#define CKR_SIGNATURE_INVALID			(0xc0UL)
#define CKR_SIGNATURE_LEN_RANGE			(0xc1UL)
#define CKR_TEMPLATE_INCOMPLETE			(0xd0UL)
#define CKR_TEMPLATE_INCONSISTENT		(0xd1UL)
#define CKR_TOKEN_NOT_PRESENT			(0xe0UL)
#define CKR_TOKEN_NOT_RECOGNIZED		(0xe1UL)
#define CKR_TOKEN_WRITE_PROTECTED		(0xe2UL)
#define	CKR_UNWRAPPING_KEY_HANDLE_INVALID	(0xf0UL)
#define CKR_UNWRAPPING_KEY_SIZE_RANGE		(0xf1UL)
#define CKR_UNWRAPPING_KEY_TYPE_INCONSISTENT	(0xf2UL)
#define CKR_USER_ALREADY_LOGGED_IN		(0x100UL)
#define CKR_USER_NOT_LOGGED_IN			(0x101UL)
#define CKR_USER_PIN_NOT_INITIALIZED		(0x102UL)
#define CKR_USER_TYPE_INVALID			(0x103UL)
#define CKR_USER_ANOTHER_ALREADY_LOGGED_IN	(0x104UL)
#define CKR_USER_TOO_MANY_TYPES			(0x105UL)
#define CKR_WRAPPED_KEY_INVALID			(0x110UL)
#define CKR_WRAPPED_KEY_LEN_RANGE		(0x112UL)
#define CKR_WRAPPING_KEY_HANDLE_INVALID		(0x113UL)
#define CKR_WRAPPING_KEY_SIZE_RANGE		(0x114UL)
#define CKR_WRAPPING_KEY_TYPE_INCONSISTENT	(0x115UL)
#define CKR_RANDOM_SEED_NOT_SUPPORTED		(0x120UL)
#define CKR_RANDOM_NO_RNG			(0x121UL)
#define CKR_DOMAIN_PARAMS_INVALID		(0x130UL)
#define CKR_CURVE_NOT_SUPPORTED			(0x140UL)
#define CKR_BUFFER_TOO_SMALL			(0x150UL)
#define CKR_SAVED_STATE_INVALID			(0x160UL)
#define CKR_INFORMATION_SENSITIVE		(0x170UL)
#define CKR_STATE_UNSAVEABLE			(0x180UL)
#define CKR_CRYPTOKI_NOT_INITIALIZED		(0x190UL)
#define CKR_CRYPTOKI_ALREADY_INITIALIZED	(0x191UL)
#define CKR_MUTEX_BAD				(0x1a0UL)
#define CKR_MUTEX_NOT_LOCKED			(0x1a1UL)
#define CKR_NEW_PIN_MODE			(0x1b0UL)
#define CKR_NEXT_OTP				(0x1b1UL)
#define CKR_EXCEEDED_MAX_ITERATIONS		(0x1c0UL)
#define CKR_FIPS_SELF_TEST_FAILED		(0x1c1UL)
#define CKR_LIBRARY_LOAD_FAILED			(0x1c2UL)
#define CKR_PIN_TOO_WEAK			(0x1c3UL)
#define CKR_PUBLIC_KEY_INVALID			(0x1c4UL)
#define CKR_FUNCTION_REJECTED			(0x200UL)
#define CKR_VENDOR_DEFINED			((unsigned long) (1UL << 31))


#define CKZ_DATA_SPECIFIED			(0x01UL)



/* Compatibility layer.  */

#ifdef CRYPTOKI_COMPAT

#undef CK_DEFINE_FUNCTION
#define CK_DEFINE_FUNCTION(retval, name) retval CK_SPEC name

/* For NULL.  */
#include <stddef.h>

typedef unsigned char CK_BYTE;
typedef unsigned char CK_CHAR;
typedef unsigned char CK_UTF8CHAR;
typedef unsigned char CK_BBOOL;
typedef unsigned long int CK_ULONG;
typedef long int CK_LONG;
typedef CK_BYTE *CK_BYTE_PTR;
typedef CK_CHAR *CK_CHAR_PTR;
typedef CK_UTF8CHAR *CK_UTF8CHAR_PTR;
typedef CK_ULONG *CK_ULONG_PTR;
typedef void *CK_VOID_PTR;
typedef void **CK_VOID_PTR_PTR;
#define CK_FALSE 0
#define CK_TRUE 1
#ifndef CK_DISABLE_TRUE_FALSE
#ifndef FALSE
#define FALSE 0
#endif
#ifndef TRUE
#define TRUE 1
#endif
#endif

typedef struct ck_version CK_VERSION;
typedef struct ck_version *CK_VERSION_PTR;

typedef struct ck_info CK_INFO;
typedef struct ck_info *CK_INFO_PTR;

typedef ck_slot_id_t *CK_SLOT_ID_PTR;

typedef struct ck_slot_info CK_SLOT_INFO;
typedef struct ck_slot_info *CK_SLOT_INFO_PTR;

typedef struct ck_token_info CK_TOKEN_INFO;
typedef struct ck_token_info *CK_TOKEN_INFO_PTR;

typedef ck_session_handle_t *CK_SESSION_HANDLE_PTR;

typedef struct ck_session_info CK_SESSION_INFO;
typedef struct ck_session_info *CK_SESSION_INFO_PTR;

typedef ck_object_handle_t *CK_OBJECT_HANDLE_PTR;

typedef ck_object_class_t *CK_OBJECT_CLASS_PTR;

typedef struct ck_attribute CK_ATTRIBUTE;
typedef struct ck_attribute *CK_ATTRIBUTE_PTR;

typedef struct ck_date CK_DATE;
typedef struct ck_date *CK_DATE_PTR;

typedef ck_mechanism_type_t *CK_MECHANISM_TYPE_PTR;

typedef struct ck_mechanism CK_MECHANISM;
typedef struct ck_mechanism *CK_MECHANISM_PTR;

typedef struct ck_mechanism_info CK_MECHANISM_INFO;
typedef struct ck_mechanism_info *CK_MECHANISM_INFO_PTR;

typedef struct ck_otp_mechanism_info CK_OTP_MECHANISM_INFO;
typedef struct ck_otp_mechanism_info *CK_OTP_MECHANISM_INFO_PTR;

typedef struct ck_function_list CK_FUNCTION_LIST;
typedef struct ck_function_list *CK_FUNCTION_LIST_PTR;
typedef struct ck_function_list **CK_FUNCTION_LIST_PTR_PTR;

typedef struct ck_c_initialize_args CK_C_INITIALIZE_ARGS;
typedef struct ck_c_initialize_args *CK_C_INITIALIZE_ARGS_PTR;

typedef struct ck_rsa_pkcs_pss_params CK_RSA_PKCS_PSS_PARAMS;
typedef struct ck_rsa_pkcs_pss_params *CK_RSA_PKCS_PSS_PARAMS_PTR;

typedef struct ck_rsa_pkcs_oaep_params CK_RSA_PKCS_OAEP_PARAMS;
typedef struct ck_rsa_pkcs_oaep_params *CK_RSA_PKCS_OAEP_PARAMS_PTR;

typedef struct ck_aes_ctr_params CK_AES_CTR_PARAMS;
typedef struct ck_aes_ctr_params *CK_AES_CTR_PARAMS_PTR;

typedef struct ck_gcm_params CK_GCM_PARAMS;
typedef struct ck_gcm_params *CK_GCM_PARAMS_PTR;

typedef struct ck_ecdh1_derive_params CK_ECDH1_DERIVE_PARAMS;
typedef struct ck_ecdh1_derive_params *CK_ECDH1_DERIVE_PARAMS_PTR;

typedef struct ck_key_derivation_string_data CK_KEY_DERIVATION_STRING_DATA;
typedef struct ck_key_derivation_string_data *CK_KEY_DERIVATION_STRING_DATA_PTR;

typedef struct ck_des_cbc_encrypt_data_params CK_DES_CBC_ENCRYPT_DATA_PARAMS;
typedef struct ck_des_cbc_encrypt_data_params *CK_DES_CBC_ENCRYPT_DATA_PARAMS_PTR;

typedef struct ck_aes_cbc_encrypt_data_params CK_AES_CBC_ENCRYPT_DATA_PARAMS;
typedef struct ck_aes_cbc_encrypt_data_params *CK_AES_CBC_ENCRYPT_DATA_PARAMS_PTR;

#ifndef NULL_PTR
#define NULL_PTR NULL
#endif

/* Delete the helper macros defined at the top of the file.  */
#undef ck_flags_t
#undef ck_version

#undef ck_info
#undef cryptoki_version
#undef manufacturer_id
#undef library_description
#undef library_version

#undef ck_notification_t
#undef ck_slot_id_t

#undef ck_slot_info
#undef slot_description
#undef hardware_version
#undef firmware_version

#undef ck_token_info
#undef serial_number
#undef max_session_count
#undef session_count
#undef max_rw_session_count
#undef rw_session_count
#undef max_pin_len
#undef min_pin_len
#undef total_public_memory
#undef free_public_memory
#undef total_private_memory
#undef free_private_memory
#undef utc_time

#undef ck_session_handle_t
#undef ck_user_type_t
#undef ck_state_t

#undef ck_session_info
#undef slot_id
#undef device_error

#undef ck_object_handle_t
#undef ck_object_class_t
#undef ck_hw_feature_type_t
#undef ck_key_type_t
#undef ck_certificate_type_t
#undef ck_attribute_type_t

#undef ck_attribute
#undef value
#undef value_len

#undef params
#undef count

#undef ck_date

#undef ck_mechanism_type_t

#undef ck_mechanism
#undef parameter
#undef parameter_len

#undef ck_mechanism_info

#undef ck_param_type
#undef ck_otp_param
#undef ck_otp_params
#undef ck_otp_signature_info

#undef min_key_size
#undef max_key_size

#undef ck_rv_t
#undef ck_notify_t

#undef ck_function_list

#undef ck_createmutex_t
#undef ck_destroymutex_t
#undef ck_lockmutex_t
#undef ck_unlockmutex_t

#undef ck_c_initialize_args
#undef create_mutex
#undef destroy_mutex
#undef lock_mutex
#undef unlock_mutex
#undef reserved

#endif	/* CRYPTOKI_COMPAT */


/* System dependencies.  */
#if defined(_WIN32) || defined(CRYPTOKI_FORCE_WIN32)
#pragma pack(pop, cryptoki)
#endif

#if defined(__cplusplus)
}
#endif

#endif	/* PKCS11_H */
