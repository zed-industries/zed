/* This file is generated, please don't edit it directly.  */
#ifndef KRB5_KRB5_H_INCLUDED
#define KRB5_KRB5_H_INCLUDED
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* General definitions for Kerberos version 5. */
/*
 * Copyright 1989, 1990, 1995, 2001, 2003, 2007, 2011 by the Massachusetts
 * Institute of Technology.  All Rights Reserved.
 *
 * Export of this software from the United States of America may
 *   require a specific license from the United States Government.
 *   It is the responsibility of any person or organization contemplating
 *   export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of M.I.T. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 * M.I.T. makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 */
/*
 * Copyright (C) 1998 by the FundsXpress, INC.
 *
 * All rights reserved.
 *
 * Export of this software from the United States of America may require
 * a specific license from the United States Government.  It is the
 * responsibility of any person or organization contemplating export to
 * obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of FundsXpress. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  FundsXpress makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */

#ifndef KRB5_GENERAL__
#define KRB5_GENERAL__

/** @defgroup KRB5_H krb5 library API
 * @{
 */

 /* By default, do not expose deprecated interfaces. */
#ifndef KRB5_DEPRECATED
#define KRB5_DEPRECATED 0
#endif

#if defined(__MACH__) && defined(__APPLE__)
#       include <TargetConditionals.h>
#    if TARGET_RT_MAC_CFM
#       error "Use KfM 4.0 SDK headers for CFM compilation."
#    endif
#endif

#if defined(_MSDOS) || defined(_WIN32)
#include <win-mac.h>
#endif

#ifndef KRB5_CONFIG__
#ifndef KRB5_CALLCONV
#define KRB5_CALLCONV
#define KRB5_CALLCONV_C
#endif /* !KRB5_CALLCONV */
#endif /* !KRB5_CONFIG__ */

#ifndef KRB5_CALLCONV_WRONG
#define KRB5_CALLCONV_WRONG
#endif

#ifndef THREEPARAMOPEN
#define THREEPARAMOPEN(x,y,z) open(x,y,z)
#endif

#define KRB5_OLD_CRYPTO

#include <stdlib.h>
#include <limits.h>             /* for *_MAX */
#include <stdarg.h>
#include <stdint.h>

#ifndef KRB5INT_BEGIN_DECLS
#if defined(__cplusplus)
#define KRB5INT_BEGIN_DECLS     extern "C" {
#define KRB5INT_END_DECLS       }
#else
#define KRB5INT_BEGIN_DECLS
#define KRB5INT_END_DECLS
#endif
#endif

KRB5INT_BEGIN_DECLS

#if defined(TARGET_OS_MAC) && TARGET_OS_MAC
#    pragma pack(push,2)
#endif

#if (__GNUC__ * 10000 + __GNUC_MINOR__ * 100 + __GNUC_PATCHLEVEL__) >= 30203
# define KRB5_ATTR_DEPRECATED __attribute__((deprecated))
#elif defined _WIN32
# define KRB5_ATTR_DEPRECATED __declspec(deprecated)
#else
# define KRB5_ATTR_DEPRECATED
#endif

/* from profile.h */
struct _profile_t;
/* typedef struct _profile_t *profile_t; */

/*
 * begin wordsize.h
 */

/*
 * Word-size related definition.
 */

typedef uint8_t krb5_octet;
typedef int16_t krb5_int16;
typedef uint16_t krb5_ui_2;
typedef int32_t krb5_int32;
typedef uint32_t krb5_ui_4;

#define VALID_INT_BITS    INT_MAX
#define VALID_UINT_BITS   UINT_MAX

#define KRB5_INT32_MAX  2147483647
/* this strange form is necessary since - is a unary operator, not a sign
   indicator */
#define KRB5_INT32_MIN  (-KRB5_INT32_MAX-1)

#define KRB5_INT16_MAX 65535
/* this strange form is necessary since - is a unary operator, not a sign
   indicator */
#define KRB5_INT16_MIN  (-KRB5_INT16_MAX-1)

/*
 * end wordsize.h
 */

/*
 * begin "base-defs.h"
 */

/*
 * Basic definitions for Kerberos V5 library
 */

#ifndef FALSE
#define FALSE   0
#endif
#ifndef TRUE
#define TRUE    1
#endif

typedef unsigned int krb5_boolean;
typedef unsigned int krb5_msgtype;
typedef unsigned int krb5_kvno;

typedef krb5_int32 krb5_addrtype;
typedef krb5_int32 krb5_enctype;
typedef krb5_int32 krb5_cksumtype;
typedef krb5_int32 krb5_authdatatype;
typedef krb5_int32 krb5_keyusage;
typedef krb5_int32 krb5_cryptotype;

typedef krb5_int32      krb5_preauthtype; /* This may change, later on */
typedef krb5_int32      krb5_flags;

/**
 * Represents a timestamp in seconds since the POSIX epoch.  This legacy type
 * is used frequently in the ABI, but cannot represent timestamps after 2038 as
 * a positive number.  Code which uses this type should cast values of it to
 * uint32_t so that negative values are treated as timestamps between 2038 and
 * 2106 on platforms with 64-bit time_t.
 */
typedef krb5_int32      krb5_timestamp;

typedef krb5_int32      krb5_deltat;

/**
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
typedef krb5_int32      krb5_error_code;

typedef krb5_error_code krb5_magic;

typedef struct _krb5_data {
    krb5_magic magic;
    unsigned int length;
    char *data;
} krb5_data;

/* Originally introduced for PKINIT; now unused.  Do not use this. */
typedef struct _krb5_octet_data {
    krb5_magic magic;
    unsigned int length;
    krb5_octet *data;
} krb5_octet_data;

/* Originally used to recognize AFS and default salts.  No longer used. */
#define SALT_TYPE_AFS_LENGTH UINT_MAX
#define SALT_TYPE_NO_LENGTH  UINT_MAX

typedef void * krb5_pointer;
typedef void const * krb5_const_pointer;

typedef struct krb5_principal_data {
    krb5_magic magic;
    krb5_data realm;
    krb5_data *data;            /**< An array of strings */
    krb5_int32 length;
    krb5_int32 type;
} krb5_principal_data;

typedef krb5_principal_data * krb5_principal;

/*
 * Per V5 spec on definition of principal types
 */

#define KRB5_NT_UNKNOWN        0 /**<  Name type not known */
#define KRB5_NT_PRINCIPAL      1 /**< Just the name of the principal
                                      as in DCE, or for users */
#define KRB5_NT_SRV_INST       2 /**< Service and other unique instance (krbtgt) */
#define KRB5_NT_SRV_HST        3 /**< Service with host name as instance
                                      (telnet, rcommands) */
#define KRB5_NT_SRV_XHST       4 /**< Service with host as remaining components */
#define KRB5_NT_UID            5 /**< Unique ID */
#define KRB5_NT_X500_PRINCIPAL 6 /**< PKINIT */
#define KRB5_NT_SMTP_NAME      7 /**< Name in form of SMTP email name */
#define KRB5_NT_ENTERPRISE_PRINCIPAL  10        /**< Windows 2000 UPN */
#define KRB5_NT_WELLKNOWN      11 /**< Well-known (special) principal */
#define KRB5_WELLKNOWN_NAMESTR "WELLKNOWN" /**< First component of
                                                NT_WELLKNOWN principals */
#define KRB5_NT_MS_PRINCIPAL         -128 /**< Windows 2000 UPN and SID */
#define KRB5_NT_MS_PRINCIPAL_AND_ID  -129 /**< NT 4 style name */
#define KRB5_NT_ENT_PRINCIPAL_AND_ID -130 /**< NT 4 style name and SID */

/** Constant version of krb5_principal_data */
typedef const krb5_principal_data *krb5_const_principal;

#define krb5_princ_realm(context, princ) (&(princ)->realm)
#define krb5_princ_set_realm(context, princ,value) ((princ)->realm = *(value))
#define krb5_princ_set_realm_length(context, princ,value) (princ)->realm.length = (value)
#define krb5_princ_set_realm_data(context, princ,value) (princ)->realm.data = (value)
#define krb5_princ_size(context, princ) (princ)->length
#define krb5_princ_type(context, princ) (princ)->type
#define krb5_princ_name(context, princ) (princ)->data
#define krb5_princ_component(context, princ,i)  \
    (((i) < krb5_princ_size(context, princ))    \
     ? (princ)->data + (i)                      \
     : NULL)

/** Constant for realm referrals. */
#define        KRB5_REFERRAL_REALM      ""

/*
 * Referral-specific functions.
 */

/**
 * Check for a match with KRB5_REFERRAL_REALM.
 *
 * @param [in] r                Realm to check
 *
 * @return @c TRUE if @a r is zero-length, @c FALSE otherwise
 */
krb5_boolean KRB5_CALLCONV
krb5_is_referral_realm(const krb5_data *r);

/**
 * Return an anonymous realm data.
 *
 * This function returns constant storage that must not be freed.
 *
 * @sa #KRB5_ANONYMOUS_REALMSTR
 */
const krb5_data *KRB5_CALLCONV
krb5_anonymous_realm(void);

/**
 * Build an anonymous principal.
 *
 * This function returns constant storage that must not be freed.
 *
 * @sa #KRB5_ANONYMOUS_PRINCSTR
 */
krb5_const_principal KRB5_CALLCONV
krb5_anonymous_principal(void);

#define KRB5_ANONYMOUS_REALMSTR "WELLKNOWN:ANONYMOUS" /**< Anonymous realm */
#define KRB5_ANONYMOUS_PRINCSTR "ANONYMOUS" /**< Anonymous principal name */
/*
 * end "base-defs.h"
 */

/*
 * begin "hostaddr.h"
 */

/** Structure for address */
typedef struct _krb5_address {
    krb5_magic magic;
    krb5_addrtype addrtype;
    unsigned int length;
    krb5_octet *contents;
} krb5_address;

/* per Kerberos v5 protocol spec */
#define ADDRTYPE_INET           0x0002
#define ADDRTYPE_CHAOS          0x0005
#define ADDRTYPE_XNS            0x0006
#define ADDRTYPE_ISO            0x0007
#define ADDRTYPE_DDP            0x0010
#define ADDRTYPE_NETBIOS        0x0014
#define ADDRTYPE_INET6          0x0018
/* not yet in the spec... */
#define ADDRTYPE_ADDRPORT       0x0100
#define ADDRTYPE_IPPORT         0x0101

/* macros to determine if a type is a local type */
#define ADDRTYPE_IS_LOCAL(addrtype) (addrtype & 0x8000)

/*
 * end "hostaddr.h"
 */


struct _krb5_context;
typedef struct _krb5_context * krb5_context;

struct _krb5_auth_context;
typedef struct _krb5_auth_context * krb5_auth_context;

struct _krb5_cryptosystem_entry;

/*
 * begin "encryption.h"
 */

/** Exposed contents of a key. */
typedef struct _krb5_keyblock {
    krb5_magic magic;
    krb5_enctype enctype;
    unsigned int length;
    krb5_octet *contents;
} krb5_keyblock;

struct krb5_key_st;
/**
 * Opaque identifier for a key.
 *
 * Use with the krb5_k APIs for better performance for repeated operations with
 * the same key and usage.  Key identifiers must not be used simultaneously
 * within multiple threads, as they may contain mutable internal state and are
 * not mutex-protected.
 */
typedef struct krb5_key_st *krb5_key;

#ifdef KRB5_OLD_CRYPTO
typedef struct _krb5_encrypt_block {
    krb5_magic magic;
    krb5_enctype crypto_entry;          /* to call krb5_encrypt_size, you need
                                           this.  it was a pointer, but it
                                           doesn't have to be.  gross. */
    krb5_keyblock *key;
} krb5_encrypt_block;
#endif

typedef struct _krb5_checksum {
    krb5_magic magic;
    krb5_cksumtype checksum_type;       /* checksum type */
    unsigned int length;
    krb5_octet *contents;
} krb5_checksum;

typedef struct _krb5_enc_data {
    krb5_magic magic;
    krb5_enctype enctype;
    krb5_kvno kvno;
    krb5_data ciphertext;
} krb5_enc_data;

/**
 * Structure to describe a region of text to be encrypted or decrypted.
 *
 * The @a flags member describes the type of the iov.
 * The @a data member points to the memory that will be manipulated.
 * All iov APIs take a pointer to the first element of an array of krb5_crypto_iov's
 * along with the size of that array. Buffer contents are manipulated in-place;
 * data is overwritten. Callers must allocate the right number of krb5_crypto_iov
 * structures before calling into an iov API.
 */
typedef struct _krb5_crypto_iov {
    krb5_cryptotype flags; /**< @ref KRB5_CRYPTO_TYPE type of the iov */
    krb5_data data;
} krb5_crypto_iov;

/* per Kerberos v5 protocol spec */
#define ENCTYPE_NULL            0x0000
#define ENCTYPE_DES_CBC_CRC     0x0001  /**< @deprecated no longer supported */
#define ENCTYPE_DES_CBC_MD4     0x0002  /**< @deprecated no longer supported */
#define ENCTYPE_DES_CBC_MD5     0x0003  /**< @deprecated no longer supported */
#define ENCTYPE_DES_CBC_RAW     0x0004  /**< @deprecated no longer supported */
#define ENCTYPE_DES3_CBC_SHA    0x0005  /**< @deprecated DES-3 cbc with SHA1 */
#define ENCTYPE_DES3_CBC_RAW    0x0006  /**< @deprecated DES-3 cbc mode raw */
#define ENCTYPE_DES_HMAC_SHA1   0x0008  /**< @deprecated no longer supported */
/* PKINIT */
#define ENCTYPE_DSA_SHA1_CMS    0x0009  /**< DSA with SHA1, CMS signature */
#define ENCTYPE_MD5_RSA_CMS     0x000a  /**< MD5 with RSA, CMS signature */
#define ENCTYPE_SHA1_RSA_CMS    0x000b  /**< SHA1 with RSA, CMS signature */
#define ENCTYPE_RC2_CBC_ENV     0x000c  /**< RC2 cbc mode, CMS enveloped data */
#define ENCTYPE_RSA_ENV         0x000d  /**< RSA encryption, CMS enveloped data */
#define ENCTYPE_RSA_ES_OAEP_ENV 0x000e  /**< RSA w/OEAP encryption, CMS enveloped data */
#define ENCTYPE_DES3_CBC_ENV    0x000f  /**< DES-3 cbc mode, CMS enveloped data */

#define ENCTYPE_DES3_CBC_SHA1               0x0010
#define ENCTYPE_AES128_CTS_HMAC_SHA1_96     0x0011 /**< RFC 3962 */
#define ENCTYPE_AES256_CTS_HMAC_SHA1_96     0x0012 /**< RFC 3962 */
#define ENCTYPE_AES128_CTS_HMAC_SHA256_128  0x0013 /**< RFC 8009 */
#define ENCTYPE_AES256_CTS_HMAC_SHA384_192  0x0014 /**< RFC 8009 */
#define ENCTYPE_ARCFOUR_HMAC                0x0017 /**< RFC 4757 */
#define ENCTYPE_ARCFOUR_HMAC_EXP            0x0018 /**< RFC 4757 */
#define ENCTYPE_CAMELLIA128_CTS_CMAC        0x0019 /**< RFC 6803 */
#define ENCTYPE_CAMELLIA256_CTS_CMAC        0x001a /**< RFC 6803 */
#define ENCTYPE_UNKNOWN                     0x01ff

#define CKSUMTYPE_CRC32         0x0001
#define CKSUMTYPE_RSA_MD4       0x0002
#define CKSUMTYPE_RSA_MD4_DES   0x0003
#define CKSUMTYPE_DESCBC        0x0004
/* des-mac-k */
/* rsa-md4-des-k */
#define CKSUMTYPE_RSA_MD5       0x0007
#define CKSUMTYPE_RSA_MD5_DES   0x0008
#define CKSUMTYPE_NIST_SHA      0x0009
#define CKSUMTYPE_HMAC_SHA1_DES3      0x000c
#define CKSUMTYPE_HMAC_SHA1_96_AES128 0x000f /**< RFC 3962. Used with
                                                ENCTYPE_AES128_CTS_HMAC_SHA1_96 */
#define CKSUMTYPE_HMAC_SHA1_96_AES256 0x0010 /**< RFC 3962. Used with
                                                ENCTYPE_AES256_CTS_HMAC_SHA1_96 */
#define CKSUMTYPE_HMAC_SHA256_128_AES128 0x0013 /**< RFC 8009 */
#define CKSUMTYPE_HMAC_SHA384_192_AES256 0x0014 /**< RFC 8009 */
#define CKSUMTYPE_CMAC_CAMELLIA128 0x0011 /**< RFC 6803 */
#define CKSUMTYPE_CMAC_CAMELLIA256 0x0012 /**< RFC 6803 */
#define CKSUMTYPE_MD5_HMAC_ARCFOUR -137 /* Microsoft netlogon */
#define CKSUMTYPE_HMAC_MD5_ARCFOUR -138 /**< RFC 4757 */

/*
 * The following are entropy source designations. Whenever
 * krb5_C_random_add_entropy is called, one of these source ids is passed in.
 * This allows the library to better estimate bits of entropy in the sample and
 * to keep track of what sources of entropy have contributed enough entropy.
 * Sources marked internal MUST NOT be used by applications outside the
 * Kerberos library
 */

enum {
    KRB5_C_RANDSOURCE_OLDAPI = 0, /*calls to krb5_C_RANDOM_SEED (INTERNAL)*/
    KRB5_C_RANDSOURCE_OSRAND = 1, /* /dev/random or equivalent (internal)*/
    KRB5_C_RANDSOURCE_TRUSTEDPARTY = 2, /* From KDC or other trusted party*/
    /*
     * This source should be used carefully; data in this category
     * should be from a third party trusted to give random bits
     * For example keys issued by the KDC in the application server.
     */
    KRB5_C_RANDSOURCE_TIMING = 3, /* Timing of operations*/
    KRB5_C_RANDSOURCE_EXTERNAL_PROTOCOL = 4, /*Protocol data possibly from attacker*/
    KRB5_C_RANDSOURCE_MAX = 5 /*Do not use; maximum source ID*/
};

#ifndef krb5_roundup
/* round x up to nearest multiple of y */
#define krb5_roundup(x, y) ((((x) + (y) - 1)/(y))*(y))
#endif /* roundup */

/* macro function definitions to help clean up code */

#if 1
#define krb5_x(ptr,args) ((ptr)?((*(ptr)) args):(abort(),1))
#define krb5_xc(ptr,args) ((ptr)?((*(ptr)) args):(abort(),(char*)0))
#else
#define krb5_x(ptr,args) ((*(ptr)) args)
#define krb5_xc(ptr,args) ((*(ptr)) args)
#endif

/**
 * Encrypt data using a key (operates on keyblock).
 *
 * @param [in]     context      Library context
 * @param [in]     key          Encryption key
 * @param [in]     usage        Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] cipher_state Cipher state; specify NULL if not needed
 * @param [in]     input        Data to be encrypted
 * @param [out]    output       Encrypted data
 *
 * This function encrypts the data block @a input and stores the output into @a
 * output.  The actual encryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the encryption operation, and
 * is updated with the state to be passed as input to the next operation.
 *
 * @note The caller must initialize @a output and allocate at least enough
 * space for the result (using krb5_c_encrypt_length() to determine the amount
 * of space needed).  @a output->length will be set to the actual length of the
 * ciphertext.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_encrypt(krb5_context context, const krb5_keyblock *key,
               krb5_keyusage usage, const krb5_data *cipher_state,
               const krb5_data *input, krb5_enc_data *output);

/**
 * Decrypt data using a key (operates on keyblock).
 *
 * @param [in]     context      Library context
 * @param [in]     key          Encryption key
 * @param [in]     usage        Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] cipher_state Cipher state; specify NULL if not needed
 * @param [in]     input        Encrypted data
 * @param [out]    output       Decrypted data
 *
 * This function decrypts the data block @a input and stores the output into @a
 * output. The actual decryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the decryption operation, and
 * is updated with the state to be passed as input to the next operation.
 *
 * @note The caller must initialize @a output and allocate at least enough
 * space for the result.  The usual practice is to allocate an output buffer as
 * long as the ciphertext, and let krb5_c_decrypt() trim @a output->length.
 * For some enctypes, the resulting @a output->length may include padding
 * bytes.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_decrypt(krb5_context context, const krb5_keyblock *key,
               krb5_keyusage usage, const krb5_data *cipher_state,
               const krb5_enc_data *input, krb5_data *output);

/**
 * Compute encrypted data length.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  inputlen        Length of the data to be encrypted
 * @param [out] length          Length of the encrypted data
 *
 * This function computes the length of the ciphertext produced by encrypting
 * @a inputlen bytes including padding, confounder, and checksum.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_encrypt_length(krb5_context context, krb5_enctype enctype,
                      size_t inputlen, size_t *length);

/**
 * Return cipher block size.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [out] blocksize       Block size for @a enctype
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_block_size(krb5_context context, krb5_enctype enctype,
                  size_t *blocksize);

/**
 * Return length of the specified key in bytes.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [out] keybytes        Number of bytes required to make a key
 * @param [out] keylength       Length of final key
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_keylengths(krb5_context context, krb5_enctype enctype,
                  size_t *keybytes, size_t *keylength);

/**
 * Initialize a new cipher state.
 *
 * @param [in]  context         Library context
 * @param [in]  key             Key
 * @param [in]  usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [out] new_state       New cipher state
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_init_state(krb5_context context, const krb5_keyblock *key,
                  krb5_keyusage usage, krb5_data *new_state);

/**
 * Free a cipher state previously allocated by krb5_c_init_state().
 *
 * @param [in] context          Library context
 * @param [in] key              Key
 * @param [in] state            Cipher state to be freed
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_free_state(krb5_context context, const krb5_keyblock *key,
                  krb5_data *state);

/**
 * Generate enctype-specific pseudo-random bytes.
 *
 * @param [in]  context         Library context
 * @param [in]  keyblock        Key
 * @param [in]  input           Input data
 * @param [out] output          Output data
 *
 * This function selects a pseudo-random function based on @a keyblock and
 * computes its value over @a input, placing the result into @a output.
 * The caller must preinitialize @a output and allocate space for the
 * result, using krb5_c_prf_length() to determine the required length.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_prf(krb5_context context, const krb5_keyblock *keyblock,
           krb5_data *input, krb5_data *output);

/**
 * Get the output length of pseudo-random functions for an encryption type.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [out] len             Length of PRF output
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_prf_length(krb5_context context, krb5_enctype enctype, size_t *len);

/**
 * Generate pseudo-random bytes using RFC 6113 PRF+.
 *
 * @param [in]  context         Library context
 * @param [in]  k               KDC contribution key
 * @param [in]  input           Input data
 * @param [out] output          Pseudo-random output buffer
 *
 * This function fills @a output with PRF+(k, input) as defined in RFC 6113
 * section 5.1.  The caller must preinitialize @a output and allocate the
 * desired amount of space.  The length of the pseudo-random output will match
 * the length of @a output.
 *
 * @note RFC 4402 defines a different PRF+ operation.  This function does not
 * implement that operation.
 *
 * @return 0 on success, @c E2BIG if output->length is too large for PRF+ to
 * generate, @c ENOMEM on allocation failure, or an error code from
 * krb5_c_prf()
 */
krb5_error_code KRB5_CALLCONV
krb5_c_prfplus(krb5_context context, const krb5_keyblock *k,
               const krb5_data *input, krb5_data *output);

/**
 * Derive a key using some input data (via RFC 6113 PRF+).
 *
 * @param [in]  context         Library context
 * @param [in]  k               KDC contribution key
 * @param [in]  input           Input string
 * @param [in]  enctype         Output key enctype (or @c ENCTYPE_NULL)
 * @param [out] out             Derived keyblock
 *
 * This function uses PRF+ as defined in RFC 6113 to derive a key from another
 * key and an input string.  If @a enctype is @c ENCTYPE_NULL, the output key
 * will have the same enctype as the input key.
 */
krb5_error_code KRB5_CALLCONV
krb5_c_derive_prfplus(krb5_context context, const krb5_keyblock *k,
                      const krb5_data *input, krb5_enctype enctype,
                      krb5_keyblock **out);

/**
 * Compute the KRB-FX-CF2 combination of two keys and pepper strings.
 *
 * @param [in]  context         Library context
 * @param [in]  k1              KDC contribution key
 * @param [in]  pepper1         String "PKINIT"
 * @param [in]  k2              Reply key
 * @param [in]  pepper2         String "KeyExchange"
 * @param [out] out             Output key
 *
 * This function computes the KRB-FX-CF2 function over its inputs and places
 * the results in a newly allocated keyblock.  This function is simple in that
 * it assumes that @a pepper1 and @a pepper2 are C strings with no internal
 * nulls and that the enctype of the result will be the same as that of @a k1.
 * @a k1 and @a k2 may be of different enctypes.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_fx_cf2_simple(krb5_context context,
                     const krb5_keyblock *k1, const char *pepper1,
                     const krb5_keyblock *k2, const char *pepper2,
                     krb5_keyblock **out);

/**
 * Generate an enctype-specific random encryption key.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type of the generated key
 * @param [out] k5_random_key   An allocated and initialized keyblock
 *
 * Use krb5_free_keyblock_contents() to free @a k5_random_key when
 * no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_make_random_key(krb5_context context, krb5_enctype enctype,
                       krb5_keyblock *k5_random_key);

/**
 * Generate an enctype-specific key from random data.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  random_data     Random input data
 * @param [out] k5_random_key   Resulting key
 *
 * This function takes random input data @a random_data and produces a valid
 * key @a k5_random_key for a given @a enctype.
 *
 * @note It is assumed that @a k5_random_key has already been initialized and
 * @a k5_random_key->contents has been allocated with the correct length.
 *
 * @sa krb5_c_keylengths()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_random_to_key(krb5_context context, krb5_enctype enctype,
                     krb5_data *random_data, krb5_keyblock *k5_random_key);

/**
 * Add entropy to the pseudo-random number generator.
 *
 * @param [in] context          Library context
 * @param [in] randsource       Entropy source (see KRB5_RANDSOURCE types)
 * @param [in] data             Data
 *
 * Contribute entropy to the PRNG used by krb5 crypto operations.  This may or
 * may not affect the output of the next crypto operation requiring random
 * data.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_random_add_entropy(krb5_context context, unsigned int randsource,
                          const krb5_data *data);

/**
 * Generate pseudo-random bytes.
 *
 * @param [in]  context         Library context
 * @param [out] data            Random data
 *
 * Fills in @a data with bytes from the PRNG used by krb5 crypto operations.
 * The caller must preinitialize @a data and allocate the desired amount of
 * space.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_random_make_octets(krb5_context context, krb5_data *data);

/**
 * Collect entropy from the OS if possible.
 *
 * @param [in]  context         Library context
 * @param [in]  strong          Strongest available source of entropy
 * @param [out] success         1 if OS provides entropy, 0 otherwise
 *
 * If @a strong is non-zero, this function attempts to use the strongest
 * available source of entropy.  Setting this flag may cause the function to
 * block on some operating systems.  Good uses include seeding the PRNG for
 * kadmind and realm setup.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_random_os_entropy(krb5_context context, int strong, int *success);

/** @deprecated Replaced by krb5_c_* API family. */
krb5_error_code KRB5_CALLCONV
krb5_c_random_seed(krb5_context context, krb5_data *data);

/**
 * Convert a string (such a password) to a key.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  string          String to be converted
 * @param [in]  salt            Salt value
 * @param [out] key             Generated key
 *
 * This function converts @a string to a @a key of encryption type @a enctype,
 * using the specified @a salt.  The newly created @a key must be released by
 * calling krb5_free_keyblock_contents() when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_string_to_key(krb5_context context, krb5_enctype enctype,
                     const krb5_data *string, const krb5_data *salt,
                     krb5_keyblock *key);

/**
 * Convert a string (such as a password) to a key with additional parameters.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  string          String to be converted
 * @param [in]  salt            Salt value
 * @param [in]  params          Parameters
 * @param [out] key             Generated key
 *
 * This function is similar to krb5_c_string_to_key(), but also takes
 * parameters which may affect the algorithm in an enctype-dependent way.  The
 * newly created @a key must be released by calling
 * krb5_free_keyblock_contents() when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_string_to_key_with_params(krb5_context context,
                                 krb5_enctype enctype,
                                 const krb5_data *string,
                                 const krb5_data *salt,
                                 const krb5_data *params,
                                 krb5_keyblock *key);

/**
 * Compare two encryption types.
 *
 * @param [in]  context         Library context
 * @param [in]  e1              First encryption type
 * @param [in]  e2              Second encryption type
 * @param [out] similar         @c TRUE if types are similar, @c FALSE if not
 *
 * This function determines whether two encryption types use the same kind of
 * keys.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_enctype_compare(krb5_context context, krb5_enctype e1, krb5_enctype e2,
                       krb5_boolean *similar);

/**
 * Compute a checksum (operates on keyblock).
 *
 * @param [in]  context         Library context
 * @param [in]  cksumtype       Checksum type (0 for mandatory type)
 * @param [in]  key             Encryption key for a keyed checksum
 * @param [in]  usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]  input           Input data
 * @param [out] cksum           Generated checksum
 *
 * This function computes a checksum of type @a cksumtype over @a input, using
 * @a key if the checksum type is a keyed checksum.  If @a cksumtype is 0 and
 * @a key is non-null, the checksum type will be the mandatory-to-implement
 * checksum type for the key's encryption type.  The actual checksum key will
 * be derived from @a key and @a usage if key derivation is specified for the
 * checksum type.  The newly created @a cksum must be released by calling
 * krb5_free_checksum_contents() when it is no longer needed.
 *
 * @note This function is similar to krb5_k_make_checksum(), but operates
 * on keyblock @a key.
 *
 * @sa krb5_c_verify_checksum()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_make_checksum(krb5_context context, krb5_cksumtype cksumtype,
                     const krb5_keyblock *key, krb5_keyusage usage,
                     const krb5_data *input, krb5_checksum *cksum);

/**
 * Verify a checksum (operates on keyblock).
 *
 * @param [in]  context         Library context
 * @param [in]  key             Encryption key for a keyed checksum
 * @param [in]  usage           @a key usage
 * @param [in]  data            Data to be used to compute a new checksum
 *                              using @a key to compare @a cksum against
 * @param [in]  cksum           Checksum to be verified
 * @param [out] valid           Non-zero for success, zero for failure
 *
 * This function verifies that @a cksum is a valid checksum for @a data.  If
 * the checksum type of @a cksum is a keyed checksum, @a key is used to verify
 * the checksum.  If the checksum type in @a cksum is 0 and @a key is not NULL,
 * the mandatory checksum type for @a key will be used.  The actual checksum
 * key will be derived from @a key and @a usage if key derivation is specified
 * for the checksum type.
 *
 * @note This function is similar to krb5_k_verify_checksum(), but operates
 * on keyblock @a key.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_verify_checksum(krb5_context context, const krb5_keyblock *key,
                       krb5_keyusage usage, const krb5_data *data,
                       const krb5_checksum *cksum, krb5_boolean *valid);

/**
 * Return the length of checksums for a checksum type.
 *
 * @param [in]  context         Library context
 * @param [in]  cksumtype       Checksum type
 * @param [out] length          Checksum length
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_checksum_length(krb5_context context, krb5_cksumtype cksumtype,
                       size_t *length);

/**
 * Return a list of keyed checksum types usable with an encryption type.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [out] count           Count of allowable checksum types
 * @param [out] cksumtypes      Array of allowable checksum types
 *
 * Use krb5_free_cksumtypes() to free @a cksumtypes when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_keyed_checksum_types(krb5_context context, krb5_enctype enctype,
                            unsigned int *count, krb5_cksumtype **cksumtypes);

/** @defgroup KRB5_KEYUSAGE KRB5_KEYUSAGE
 * @{
 */
#define KRB5_KEYUSAGE_AS_REQ_PA_ENC_TS          1
#define KRB5_KEYUSAGE_KDC_REP_TICKET            2
#define KRB5_KEYUSAGE_AS_REP_ENCPART            3
#define KRB5_KEYUSAGE_TGS_REQ_AD_SESSKEY        4
#define KRB5_KEYUSAGE_TGS_REQ_AD_SUBKEY         5
#define KRB5_KEYUSAGE_TGS_REQ_AUTH_CKSUM        6
#define KRB5_KEYUSAGE_TGS_REQ_AUTH              7
#define KRB5_KEYUSAGE_TGS_REP_ENCPART_SESSKEY   8
#define KRB5_KEYUSAGE_TGS_REP_ENCPART_SUBKEY    9
#define KRB5_KEYUSAGE_AP_REQ_AUTH_CKSUM         10
#define KRB5_KEYUSAGE_AP_REQ_AUTH               11
#define KRB5_KEYUSAGE_AP_REP_ENCPART            12
#define KRB5_KEYUSAGE_KRB_PRIV_ENCPART          13
#define KRB5_KEYUSAGE_KRB_CRED_ENCPART          14
#define KRB5_KEYUSAGE_KRB_SAFE_CKSUM            15
#define KRB5_KEYUSAGE_APP_DATA_ENCRYPT          16
#define KRB5_KEYUSAGE_APP_DATA_CKSUM            17
#define KRB5_KEYUSAGE_KRB_ERROR_CKSUM           18
#define KRB5_KEYUSAGE_AD_KDCISSUED_CKSUM        19
#define KRB5_KEYUSAGE_AD_MTE                    20
#define KRB5_KEYUSAGE_AD_ITE                    21

/* XXX need to register these */

#define KRB5_KEYUSAGE_GSS_TOK_MIC               22
#define KRB5_KEYUSAGE_GSS_TOK_WRAP_INTEG        23
#define KRB5_KEYUSAGE_GSS_TOK_WRAP_PRIV         24

/* Defined in Integrating SAM Mechanisms with Kerberos draft */
#define KRB5_KEYUSAGE_PA_SAM_CHALLENGE_CKSUM    25
/** Note conflict with @ref KRB5_KEYUSAGE_PA_S4U_X509_USER_REQUEST */
#define KRB5_KEYUSAGE_PA_SAM_CHALLENGE_TRACKID  26
/** Note conflict with @ref KRB5_KEYUSAGE_PA_S4U_X509_USER_REPLY */
#define KRB5_KEYUSAGE_PA_SAM_RESPONSE           27

/* Defined in [MS-SFU] */
/** Note conflict with @ref KRB5_KEYUSAGE_PA_SAM_CHALLENGE_TRACKID */
#define KRB5_KEYUSAGE_PA_S4U_X509_USER_REQUEST  26
/** Note conflict with @ref KRB5_KEYUSAGE_PA_SAM_RESPONSE */
#define KRB5_KEYUSAGE_PA_S4U_X509_USER_REPLY    27

/* unused */
#define KRB5_KEYUSAGE_PA_REFERRAL               26

#define KRB5_KEYUSAGE_AD_SIGNEDPATH             -21
#define KRB5_KEYUSAGE_IAKERB_FINISHED           42
#define KRB5_KEYUSAGE_PA_PKINIT_KX              44
#define KRB5_KEYUSAGE_PA_OTP_REQUEST  45  /**< See RFC 6560 section 4.2 */
/* define in draft-ietf-krb-wg-preauth-framework*/
#define KRB5_KEYUSAGE_FAST_REQ_CHKSUM 50
#define KRB5_KEYUSAGE_FAST_ENC 51
#define KRB5_KEYUSAGE_FAST_REP 52
#define KRB5_KEYUSAGE_FAST_FINISHED 53
#define KRB5_KEYUSAGE_ENC_CHALLENGE_CLIENT 54
#define KRB5_KEYUSAGE_ENC_CHALLENGE_KDC 55
#define KRB5_KEYUSAGE_AS_REQ 56
#define KRB5_KEYUSAGE_CAMMAC 64
#define KRB5_KEYUSAGE_SPAKE 65

/* Key usage values 512-1023 are reserved for uses internal to a Kerberos
 * implementation. */
#define KRB5_KEYUSAGE_PA_FX_COOKIE 513  /**< Used for encrypted FAST cookies */
#define KRB5_KEYUSAGE_PA_AS_FRESHNESS 514  /**< Used for freshness tokens */
/** @} */ /* end of KRB5_KEYUSAGE group */

/**
 * Verify that a specified encryption type is a valid Kerberos encryption type.
 *
 * @param [in] ktype            Encryption type
 *
 * @return @c TRUE if @a ktype is valid, @c FALSE if not
 */
krb5_boolean KRB5_CALLCONV
krb5_c_valid_enctype(krb5_enctype ktype);

/**
 * Verify that specified checksum type is a valid Kerberos checksum type.
 *
 * @param [in] ctype            Checksum type
 *
 * @return @c TRUE if @a ctype is valid, @c FALSE if not
 */
krb5_boolean KRB5_CALLCONV
krb5_c_valid_cksumtype(krb5_cksumtype ctype);

/**
 * Test whether a checksum type is collision-proof.
 *
 * @param [in] ctype            Checksum type
 *
 * @return @c TRUE if @a ctype is collision-proof, @c FALSE if it is not
 * collision-proof or not a valid checksum type.
 */
krb5_boolean KRB5_CALLCONV
krb5_c_is_coll_proof_cksum(krb5_cksumtype ctype);

/**
 * Test whether a checksum type is keyed.
 *
 * @param [in] ctype            Checksum type
 *
 * @return @c TRUE if @a ctype is a keyed checksum type, @c FALSE otherwise.
 */
krb5_boolean KRB5_CALLCONV
krb5_c_is_keyed_cksum(krb5_cksumtype ctype);

/* AEAD APIs */
/** @defgroup KRB5_CRYPTO_TYPE KRB5_CRYPTO_TYPE
 * @{
 */
#define KRB5_CRYPTO_TYPE_EMPTY      0   /**< [in] ignored */
#define KRB5_CRYPTO_TYPE_HEADER     1   /**< [out] header */
#define KRB5_CRYPTO_TYPE_DATA       2   /**< [in, out] plaintext */
#define KRB5_CRYPTO_TYPE_SIGN_ONLY  3   /**< [in] associated data */
#define KRB5_CRYPTO_TYPE_PADDING    4   /**< [out] padding */
#define KRB5_CRYPTO_TYPE_TRAILER    5   /**< [out] checksum for encrypt */
#define KRB5_CRYPTO_TYPE_CHECKSUM   6   /**< [out] checksum for MIC */
#define KRB5_CRYPTO_TYPE_STREAM     7   /**< [in] entire message without
                                           decomposing the structure into
                                           header, data and trailer buffers */
/** @} */ /* end of KRB5_CRYPTO_TYPE group */

/**
 * Fill in a checksum element in IOV array (operates on keyblock)
 *
 * @param [in]     context         Library context
 * @param [in]     cksumtype       Checksum type (0 for mandatory type)
 * @param [in]     key             Encryption key for a keyed checksum
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] data            IOV array
 * @param [in]     num_data        Size of @a data
 *
 * Create a checksum in the #KRB5_CRYPTO_TYPE_CHECKSUM element over
 * #KRB5_CRYPTO_TYPE_DATA and #KRB5_CRYPTO_TYPE_SIGN_ONLY chunks in @a data.
 * Only the #KRB5_CRYPTO_TYPE_CHECKSUM region is modified.
 *
 * @note This function is similar to krb5_k_make_checksum_iov(), but operates
 * on keyblock @a key.
 *
 * @sa krb5_c_verify_checksum_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_make_checksum_iov(krb5_context context, krb5_cksumtype cksumtype,
                         const krb5_keyblock *key, krb5_keyusage usage,
                         krb5_crypto_iov *data, size_t num_data);

/**
 * Validate a checksum element in IOV array (operates on keyblock).
 *
 * @param [in]     context         Library context
 * @param [in]     cksumtype       Checksum type (0 for mandatory type)
 * @param [in]     key             Encryption key for a keyed checksum
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     data            IOV array
 * @param [in]     num_data        Size of @a data
 * @param [out]    valid           Non-zero for success, zero for failure
 *
 * Confirm that the checksum in the #KRB5_CRYPTO_TYPE_CHECKSUM element is a
 * valid checksum of the #KRB5_CRYPTO_TYPE_DATA and #KRB5_CRYPTO_TYPE_SIGN_ONLY
 * regions in the iov.
 *
 * @note This function is similar to krb5_k_verify_checksum_iov(), but operates
 * on keyblock @a key.
 *
 * @sa krb5_c_make_checksum_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_verify_checksum_iov(krb5_context context, krb5_cksumtype cksumtype,
                           const krb5_keyblock *key, krb5_keyusage usage,
                           const krb5_crypto_iov *data, size_t num_data,
                           krb5_boolean *valid);

/**
 * Encrypt data in place supporting AEAD (operates on keyblock).
 *
 * @param [in]     context         Library context
 * @param [in]     keyblock        Encryption key
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     cipher_state    Cipher state; specify NULL if not needed
 * @param [in,out] data            IOV array. Modified in-place.
 * @param [in]     num_data        Size of @a data
 *
 * This function encrypts the data block @a data and stores the output in-place.
 * The actual encryption key will be derived from @a keyblock and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the encryption operation, and
 * is updated with the state to be passed as input to the next operation.
 * The caller must allocate the right number of krb5_crypto_iov
 * structures before calling into this API.
 *
 * @note On return from a krb5_c_encrypt_iov() call, the @a data->length in the
 * iov structure are adjusted to reflect actual lengths of the ciphertext used.
 * For example, if the padding length is too large, the length will be reduced.
 * Lengths are never increased.
 *
 * @note This function is similar to krb5_k_encrypt_iov(), but operates
 * on keyblock @a keyblock.
 *
 * @sa krb5_c_decrypt_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_encrypt_iov(krb5_context context, const krb5_keyblock *keyblock,
                   krb5_keyusage usage, const krb5_data *cipher_state,
                   krb5_crypto_iov *data, size_t num_data);

/**
 * Decrypt data in place supporting AEAD (operates on keyblock).
 *
 * @param [in]     context         Library context
 * @param [in]     keyblock        Encryption key
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     cipher_state    Cipher state; specify NULL if not needed
 * @param [in,out] data            IOV array. Modified in-place.
 * @param [in]     num_data        Size of @a data
 *
 * This function decrypts the data block @a data and stores the output in-place.
 * The actual decryption key will be derived from @a keyblock and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the decryption operation, and
 * is updated with the state to be passed as input to the next operation.
 * The caller must allocate the right number of krb5_crypto_iov
 * structures before calling into this API.
 *
 * @note On return from a krb5_c_decrypt_iov() call, the @a data->length in the
 * iov structure are adjusted to reflect actual lengths of the ciphertext used.
 * For example, if the padding length is too large, the length will be reduced.
 * Lengths are never increased.
 *
 * @note This function is similar to krb5_k_decrypt_iov(), but operates
 * on keyblock @a keyblock.
 *
 * @sa krb5_c_decrypt_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_decrypt_iov(krb5_context context, const krb5_keyblock *keyblock,
                   krb5_keyusage usage, const krb5_data *cipher_state,
                   krb5_crypto_iov *data, size_t num_data);

/**
 * Return a length of a message field specific to the encryption type.
 *
 * @param [in]  context      Library context
 * @param [in]  enctype      Encryption type
 * @param [in]  type         Type field (See @ref KRB5_CRYPTO_TYPE types)
 * @param [out] size         Length of the @a type specific to @a enctype
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_crypto_length(krb5_context context, krb5_enctype enctype,
                     krb5_cryptotype type, unsigned int *size);

/**
 * Fill in lengths for header, trailer and padding in a IOV array.
 *
 * @param [in]      context      Library context
 * @param [in]      enctype      Encryption type
 * @param [in,out]  data         IOV array
 * @param [in]      num_data     Size of @a data
 *
 * Padding is set to the actual padding required based on the provided
 * @a data buffers. Typically this API is used after setting up the data
 * buffers and #KRB5_CRYPTO_TYPE_SIGN_ONLY buffers, but before actually
 * allocating header, trailer and padding.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_c_crypto_length_iov(krb5_context context, krb5_enctype enctype,
                         krb5_crypto_iov *data, size_t num_data);

/**
 * Return a number of padding octets.
 *
 * @param [in]  context      Library context
 * @param [in]  enctype      Encryption type
 * @param [in]  data_length  Length of the plaintext to pad
 * @param [out] size         Number of padding octets
 *
 * This function returns the number of the padding octets required to pad
 * @a data_length octets of plaintext.
 *
 * @retval 0 Success; otherwise - KRB5_BAD_ENCTYPE
 */
krb5_error_code KRB5_CALLCONV
krb5_c_padding_length(krb5_context context, krb5_enctype enctype,
                      size_t data_length, unsigned int *size);

/**
 * Create a krb5_key from the enctype and key data in a keyblock.
 *
 * @param [in]  context      Library context
 * @param [in]  key_data     Keyblock
 * @param [out] out          Opaque key
 *
 * The reference count on a key @a out is set to 1.
 * Use krb5_k_free_key() to free @a out when it is no longer needed.
 *
 * @retval 0 Success; otherwise - KRB5_BAD_ENCTYPE
 */
krb5_error_code KRB5_CALLCONV
krb5_k_create_key(krb5_context context, const krb5_keyblock *key_data,
                  krb5_key *out);

/** Increment the reference count on a key. */
void KRB5_CALLCONV
krb5_k_reference_key(krb5_context context, krb5_key key);

/** Decrement the reference count on a key and free it if it hits zero. */
void KRB5_CALLCONV
krb5_k_free_key(krb5_context context, krb5_key key);

/** Retrieve a copy of the keyblock from a krb5_key structure. */
krb5_error_code KRB5_CALLCONV
krb5_k_key_keyblock(krb5_context context, krb5_key key,
                    krb5_keyblock **key_data);

/** Retrieve the enctype of a krb5_key structure. */
krb5_enctype KRB5_CALLCONV
krb5_k_key_enctype(krb5_context context, krb5_key key);

/**
 * Encrypt data using a key (operates on opaque key).
 *
 * @param [in]     context      Library context
 * @param [in]     key          Encryption key
 * @param [in]     usage        Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] cipher_state Cipher state; specify NULL if not needed
 * @param [in]     input        Data to be encrypted
 * @param [out]    output       Encrypted data
 *
 * This function encrypts the data block @a input and stores the output into @a
 * output.  The actual encryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the encryption operation, and
 * is updated with the state to be passed as input to the next operation.
 *
 * @note The caller must initialize @a output and allocate at least enough
 * space for the result (using krb5_c_encrypt_length() to determine the amount
 * of space needed).  @a output->length will be set to the actual length of the
 * ciphertext.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_k_encrypt(krb5_context context, krb5_key key, krb5_keyusage usage,
               const krb5_data *cipher_state, const krb5_data *input,
               krb5_enc_data *output);

/**
 * Encrypt data in place supporting AEAD (operates on opaque key).
 *
 * @param [in]     context         Library context
 * @param [in]     key             Encryption key
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     cipher_state    Cipher state; specify NULL if not needed
 * @param [in,out] data            IOV array. Modified in-place.
 * @param [in]     num_data        Size of @a data
 *
 * This function encrypts the data block @a data and stores the output in-place.
 * The actual encryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the encryption operation, and
 * is updated with the state to be passed as input to the next operation.
 * The caller must allocate the right number of krb5_crypto_iov
 * structures before calling into this API.
 *
 * @note On return from a krb5_c_encrypt_iov() call, the @a data->length in the
 * iov structure are adjusted to reflect actual lengths of the ciphertext used.
 * For example, if the padding length is too large, the length will be reduced.
 * Lengths are never increased.
 *
 * @note This function is similar to krb5_c_encrypt_iov(), but operates
 * on opaque key @a key.
 *
 * @sa krb5_k_decrypt_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_k_encrypt_iov(krb5_context context, krb5_key key, krb5_keyusage usage,
                   const krb5_data *cipher_state, krb5_crypto_iov *data,
                   size_t num_data);

/**
 * Decrypt data using a key (operates on opaque key).
 *
 * @param [in]     context      Library context
 * @param [in]     key          Encryption key
 * @param [in]     usage        Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] cipher_state Cipher state; specify NULL if not needed
 * @param [in]     input        Encrypted data
 * @param [out]    output       Decrypted data
 *
 * This function decrypts the data block @a input and stores the output into @a
 * output. The actual decryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the decryption operation, and
 * is updated with the state to be passed as input to the next operation.
 *
 * @note The caller must initialize @a output and allocate at least enough
 * space for the result.  The usual practice is to allocate an output buffer as
 * long as the ciphertext, and let krb5_c_decrypt() trim @a output->length.
 * For some enctypes, the resulting @a output->length may include padding
 * bytes.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_k_decrypt(krb5_context context, krb5_key key, krb5_keyusage usage,
               const krb5_data *cipher_state, const krb5_enc_data *input,
               krb5_data *output);

/**
 * Decrypt data in place supporting AEAD (operates on opaque key).
 *
 * @param [in]     context         Library context
 * @param [in]     key             Encryption key
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     cipher_state    Cipher state; specify NULL if not needed
 * @param [in,out] data            IOV array. Modified in-place.
 * @param [in]     num_data        Size of @a data
 *
 * This function decrypts the data block @a data and stores the output in-place.
 * The actual decryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the decryption operation, and
 * is updated with the state to be passed as input to the next operation.
 * The caller must allocate the right number of krb5_crypto_iov
 * structures before calling into this API.
 *
 * @note On return from a krb5_c_decrypt_iov() call, the @a data->length in the
 * iov structure are adjusted to reflect actual lengths of the ciphertext used.
 * For example, if the padding length is too large, the length will be reduced.
 * Lengths are never increased.
 *
 * @note This function is similar to krb5_c_decrypt_iov(), but operates
 * on opaque key @a key.
 *
 * @sa krb5_k_encrypt_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_k_decrypt_iov(krb5_context context, krb5_key key, krb5_keyusage usage,
                   const krb5_data *cipher_state, krb5_crypto_iov *data,
                   size_t num_data);
/**
 * Compute a checksum (operates on opaque key).
 *
 * @param [in]  context         Library context
 * @param [in]  cksumtype       Checksum type (0 for mandatory type)
 * @param [in]  key             Encryption key for a keyed checksum
 * @param [in]  usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]  input           Input data
 * @param [out] cksum           Generated checksum
 *
 * This function computes a checksum of type @a cksumtype over @a input, using
 * @a key if the checksum type is a keyed checksum.  If @a cksumtype is 0 and
 * @a key is non-null, the checksum type will be the mandatory-to-implement
 * checksum type for the key's encryption type.  The actual checksum key will
 * be derived from @a key and @a usage if key derivation is specified for the
 * checksum type.  The newly created @a cksum must be released by calling
 * krb5_free_checksum_contents() when it is no longer needed.
 *
 * @note This function is similar to krb5_c_make_checksum(), but operates
 * on opaque @a key.
 *
 * @sa krb5_c_verify_checksum()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_k_make_checksum(krb5_context context, krb5_cksumtype cksumtype,
                     krb5_key key, krb5_keyusage usage, const krb5_data *input,
                     krb5_checksum *cksum);

/**
 * Fill in a checksum element in IOV array (operates on opaque key)
 *
 * @param [in]     context         Library context
 * @param [in]     cksumtype       Checksum type (0 for mandatory type)
 * @param [in]     key             Encryption key for a keyed checksum
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] data            IOV array
 * @param [in]     num_data        Size of @a data
 *
 * Create a checksum in the #KRB5_CRYPTO_TYPE_CHECKSUM element over
 * #KRB5_CRYPTO_TYPE_DATA and #KRB5_CRYPTO_TYPE_SIGN_ONLY chunks in @a data.
 * Only the #KRB5_CRYPTO_TYPE_CHECKSUM region is modified.
 *
 * @note This function is similar to krb5_c_make_checksum_iov(), but operates
 * on opaque @a key.
 *
 * @sa krb5_k_verify_checksum_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_k_make_checksum_iov(krb5_context context, krb5_cksumtype cksumtype,
                         krb5_key key, krb5_keyusage usage,
                         krb5_crypto_iov *data, size_t num_data);

/**
 * Verify a checksum (operates on opaque key).
 *
 * @param [in]  context         Library context
 * @param [in]  key             Encryption key for a keyed checksum
 * @param [in]  usage           @a key usage
 * @param [in]  data            Data to be used to compute a new checksum
 *                              using @a key to compare @a cksum against
 * @param [in]  cksum           Checksum to be verified
 * @param [out] valid           Non-zero for success, zero for failure
 *
 * This function verifies that @a cksum is a valid checksum for @a data.  If
 * the checksum type of @a cksum is a keyed checksum, @a key is used to verify
 * the checksum.  If the checksum type in @a cksum is 0 and @a key is not NULL,
 * the mandatory checksum type for @a key will be used.  The actual checksum
 * key will be derived from @a key and @a usage if key derivation is specified
 * for the checksum type.
 *
 * @note This function is similar to krb5_c_verify_checksum(), but operates
 * on opaque @a key.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_k_verify_checksum(krb5_context context, krb5_key key, krb5_keyusage usage,
                       const krb5_data *data, const krb5_checksum *cksum,
                       krb5_boolean *valid);

/**
 * Validate a checksum element in IOV array (operates on opaque key).
 *
 * @param [in]     context         Library context
 * @param [in]     cksumtype       Checksum type (0 for mandatory type)
 * @param [in]     key             Encryption key for a keyed checksum
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     data            IOV array
 * @param [in]     num_data        Size of @a data
 * @param [out]    valid           Non-zero for success, zero for failure
 *
 * Confirm that the checksum in the #KRB5_CRYPTO_TYPE_CHECKSUM element is a
 * valid checksum of the #KRB5_CRYPTO_TYPE_DATA and #KRB5_CRYPTO_TYPE_SIGN_ONLY
 * regions in the iov.
 *
 * @note This function is similar to krb5_c_verify_checksum_iov(), but operates
 * on opaque @a key.
 *
 * @sa krb5_k_make_checksum_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_k_verify_checksum_iov(krb5_context context, krb5_cksumtype cksumtype,
                           krb5_key key, krb5_keyusage usage,
                           const krb5_crypto_iov *data, size_t num_data,
                           krb5_boolean *valid);

/**
 * Generate enctype-specific pseudo-random bytes (operates on opaque key).
 *
 * @param [in]  context      Library context
 * @param [in]  key          Key
 * @param [in]  input        Input data
 * @param [out] output       Output data
 *
 * This function selects a pseudo-random function based on @a key and
 * computes its value over @a input, placing the result into @a output.
 * The caller must preinitialize @a output and allocate space for the
 * result.
 *
 * @note This function is similar to krb5_c_prf(), but operates
 * on opaque @a key.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_k_prf(krb5_context context, krb5_key key, krb5_data *input, krb5_data *output);

#ifdef KRB5_OLD_CRYPTO
/*
 * old cryptosystem routine prototypes.  These are now layered
 * on top of the functions above.
 */
/** @deprecated Replaced by krb5_c_* API family.*/
krb5_error_code KRB5_CALLCONV
krb5_encrypt(krb5_context context, krb5_const_pointer inptr,
             krb5_pointer outptr, size_t size, krb5_encrypt_block *eblock,
             krb5_pointer ivec);

/** @deprecated Replaced by krb5_c_* API family. */
krb5_error_code KRB5_CALLCONV
krb5_decrypt(krb5_context context, krb5_const_pointer inptr,
             krb5_pointer outptr, size_t size, krb5_encrypt_block *eblock,
             krb5_pointer ivec);

/** @deprecated Replaced by krb5_c_* API family. */
krb5_error_code KRB5_CALLCONV
krb5_process_key(krb5_context context, krb5_encrypt_block *eblock,
                 const krb5_keyblock * key);

/** @deprecated Replaced by krb5_c_* API family. */
krb5_error_code KRB5_CALLCONV
krb5_finish_key(krb5_context context, krb5_encrypt_block * eblock);

/** @deprecated See krb5_c_string_to_key() */
krb5_error_code KRB5_CALLCONV
krb5_string_to_key(krb5_context context, const krb5_encrypt_block *eblock,
                   krb5_keyblock * keyblock, const krb5_data *data,
                   const krb5_data *salt);

/** @deprecated Replaced by krb5_c_* API family. */
krb5_error_code KRB5_CALLCONV
krb5_init_random_key(krb5_context context, const krb5_encrypt_block *eblock,
                     const krb5_keyblock *keyblock, krb5_pointer *ptr);

/** @deprecated Replaced by krb5_c_* API family. */
krb5_error_code KRB5_CALLCONV
krb5_finish_random_key(krb5_context context, const krb5_encrypt_block *eblock,
                       krb5_pointer *ptr);

/** @deprecated Replaced by krb5_c_* API family. */
krb5_error_code KRB5_CALLCONV
krb5_random_key(krb5_context context, const krb5_encrypt_block *eblock,
                krb5_pointer ptr, krb5_keyblock **keyblock);

/** @deprecated Replaced by krb5_c_* API family. */
krb5_enctype KRB5_CALLCONV
krb5_eblock_enctype(krb5_context context, const krb5_encrypt_block *eblock);

/** @deprecated Replaced by krb5_c_* API family. */
krb5_error_code KRB5_CALLCONV
krb5_use_enctype(krb5_context context, krb5_encrypt_block *eblock,
                 krb5_enctype enctype);

/** @deprecated Replaced by krb5_c_* API family. */
size_t KRB5_CALLCONV
krb5_encrypt_size(size_t length, krb5_enctype crypto);

/** @deprecated See krb5_c_checksum_length() */
size_t KRB5_CALLCONV
krb5_checksum_size(krb5_context context, krb5_cksumtype ctype);

/** @deprecated See krb5_c_make_checksum() */
krb5_error_code KRB5_CALLCONV
krb5_calculate_checksum(krb5_context context, krb5_cksumtype ctype,
                        krb5_const_pointer in, size_t in_length,
                        krb5_const_pointer seed, size_t seed_length,
                        krb5_checksum * outcksum);

/** @deprecated See krb5_c_verify_checksum() */
krb5_error_code KRB5_CALLCONV
krb5_verify_checksum(krb5_context context, krb5_cksumtype ctype,
                     const krb5_checksum * cksum, krb5_const_pointer in,
                     size_t in_length, krb5_const_pointer seed,
                     size_t seed_length);

#endif /* KRB5_OLD_CRYPTO */

/*
 * end "encryption.h"
 */

/*
 * begin "fieldbits.h"
 */

/* kdc_options for kdc_request */
/* options is 32 bits; each host is responsible to put the 4 bytes
   representing these bits into net order before transmission */
/* #define      KDC_OPT_RESERVED        0x80000000 */
#define KDC_OPT_FORWARDABLE             0x40000000
#define KDC_OPT_FORWARDED               0x20000000
#define KDC_OPT_PROXIABLE               0x10000000
#define KDC_OPT_PROXY                   0x08000000
#define KDC_OPT_ALLOW_POSTDATE          0x04000000
#define KDC_OPT_POSTDATED               0x02000000
/* #define      KDC_OPT_UNUSED          0x01000000 */
#define KDC_OPT_RENEWABLE               0x00800000
/* #define      KDC_OPT_UNUSED          0x00400000 */
/* #define      KDC_OPT_RESERVED        0x00200000 */
/* #define      KDC_OPT_RESERVED        0x00100000 */
/* #define      KDC_OPT_RESERVED        0x00080000 */
/* #define      KDC_OPT_RESERVED        0x00040000 */
#define KDC_OPT_CNAME_IN_ADDL_TKT       0x00020000
#define KDC_OPT_CANONICALIZE            0x00010000
#define KDC_OPT_REQUEST_ANONYMOUS       0x00008000
/* #define      KDC_OPT_RESERVED        0x00004000 */
/* #define      KDC_OPT_RESERVED        0x00002000 */
/* #define      KDC_OPT_RESERVED        0x00001000 */
/* #define      KDC_OPT_RESERVED        0x00000800 */
/* #define      KDC_OPT_RESERVED        0x00000400 */
/* #define      KDC_OPT_RESERVED        0x00000200 */
/* #define      KDC_OPT_RESERVED        0x00000100 */
/* #define      KDC_OPT_RESERVED        0x00000080 */
/* #define      KDC_OPT_RESERVED        0x00000040 */
#define KDC_OPT_DISABLE_TRANSITED_CHECK 0x00000020
#define KDC_OPT_RENEWABLE_OK            0x00000010
#define KDC_OPT_ENC_TKT_IN_SKEY         0x00000008
/* #define      KDC_OPT_UNUSED          0x00000004 */
#define KDC_OPT_RENEW                   0x00000002
#define KDC_OPT_VALIDATE                0x00000001

/*
 * Mask of ticket flags in the TGT which should be converted into KDC
 * options when using the TGT to get derivative tickets.
 *
 *  New mask = KDC_OPT_FORWARDABLE | KDC_OPT_PROXIABLE |
 *             KDC_OPT_ALLOW_POSTDATE | KDC_OPT_RENEWABLE
 */
#define KDC_TKT_COMMON_MASK             0x54800000

/* definitions for ap_options fields */

/** @defgroup AP_OPTS AP_OPTS
 *
 * ap_options are 32 bits; each host is responsible to put the 4 bytes
 * representing these bits into net order before transmission
 * @{
 */
#define AP_OPTS_RESERVED           0x80000000
#define AP_OPTS_USE_SESSION_KEY    0x40000000 /**< Use session key */
#define AP_OPTS_MUTUAL_REQUIRED    0x20000000 /**< Perform a mutual
                                                 authentication exchange */
#define AP_OPTS_ETYPE_NEGOTIATION  0x00000002
#define AP_OPTS_USE_SUBKEY         0x00000001 /**< Generate a subsession key
                                                 from the current session key
                                                 obtained from the
                                                 credentials */
/* #define      AP_OPTS_RESERVED        0x10000000 */
/* #define      AP_OPTS_RESERVED        0x08000000 */
/* #define      AP_OPTS_RESERVED        0x04000000 */
/* #define      AP_OPTS_RESERVED        0x02000000 */
/* #define      AP_OPTS_RESERVED        0x01000000 */
/* #define      AP_OPTS_RESERVED        0x00800000 */
/* #define      AP_OPTS_RESERVED        0x00400000 */
/* #define      AP_OPTS_RESERVED        0x00200000 */
/* #define      AP_OPTS_RESERVED        0x00100000 */
/* #define      AP_OPTS_RESERVED        0x00080000 */
/* #define      AP_OPTS_RESERVED        0x00040000 */
/* #define      AP_OPTS_RESERVED        0x00020000 */
/* #define      AP_OPTS_RESERVED        0x00010000 */
/* #define      AP_OPTS_RESERVED        0x00008000 */
/* #define      AP_OPTS_RESERVED        0x00004000 */
/* #define      AP_OPTS_RESERVED        0x00002000 */
/* #define      AP_OPTS_RESERVED        0x00001000 */
/* #define      AP_OPTS_RESERVED        0x00000800 */
/* #define      AP_OPTS_RESERVED        0x00000400 */
/* #define      AP_OPTS_RESERVED        0x00000200 */
/* #define      AP_OPTS_RESERVED        0x00000100 */
/* #define      AP_OPTS_RESERVED        0x00000080 */
/* #define      AP_OPTS_RESERVED        0x00000040 */
/* #define      AP_OPTS_RESERVED        0x00000020 */
/* #define      AP_OPTS_RESERVED        0x00000010 */
/* #define      AP_OPTS_RESERVED        0x00000008 */
/* #define      AP_OPTS_RESERVED        0x00000004 */


#define AP_OPTS_WIRE_MASK               0xfffffff0
/** @} */ /* end of AP_OPTS group */

/* definitions for ad_type fields. */
#define AD_TYPE_RESERVED        0x8000
#define AD_TYPE_EXTERNAL        0x4000
#define AD_TYPE_REGISTERED      0x2000

#define AD_TYPE_FIELD_TYPE_MASK 0x1fff

/* Ticket flags */
/* flags are 32 bits; each host is responsible to put the 4 bytes
   representing these bits into net order before transmission */
/* #define      TKT_FLG_RESERVED        0x80000000 */
#define TKT_FLG_FORWARDABLE             0x40000000
#define TKT_FLG_FORWARDED               0x20000000
#define TKT_FLG_PROXIABLE               0x10000000
#define TKT_FLG_PROXY                   0x08000000
#define TKT_FLG_MAY_POSTDATE            0x04000000
#define TKT_FLG_POSTDATED               0x02000000
#define TKT_FLG_INVALID                 0x01000000
#define TKT_FLG_RENEWABLE               0x00800000
#define TKT_FLG_INITIAL                 0x00400000
#define TKT_FLG_PRE_AUTH                0x00200000
#define TKT_FLG_HW_AUTH                 0x00100000
#define TKT_FLG_TRANSIT_POLICY_CHECKED  0x00080000
#define TKT_FLG_OK_AS_DELEGATE          0x00040000
#define TKT_FLG_ENC_PA_REP              0x00010000
#define TKT_FLG_ANONYMOUS               0x00008000
/* #define      TKT_FLG_RESERVED        0x00004000 */
/* #define      TKT_FLG_RESERVED        0x00002000 */
/* #define      TKT_FLG_RESERVED        0x00001000 */
/* #define      TKT_FLG_RESERVED        0x00000800 */
/* #define      TKT_FLG_RESERVED        0x00000400 */
/* #define      TKT_FLG_RESERVED        0x00000200 */
/* #define      TKT_FLG_RESERVED        0x00000100 */
/* #define      TKT_FLG_RESERVED        0x00000080 */
/* #define      TKT_FLG_RESERVED        0x00000040 */
/* #define      TKT_FLG_RESERVED        0x00000020 */
/* #define      TKT_FLG_RESERVED        0x00000010 */
/* #define      TKT_FLG_RESERVED        0x00000008 */
/* #define      TKT_FLG_RESERVED        0x00000004 */
/* #define      TKT_FLG_RESERVED        0x00000002 */
/* #define      TKT_FLG_RESERVED        0x00000001 */

/* definitions for lr_type fields. */
#define LR_TYPE_THIS_SERVER_ONLY        0x8000

#define LR_TYPE_INTERPRETATION_MASK     0x7fff

/* definitions for msec direction bit for KRB_SAFE, KRB_PRIV */
#define MSEC_DIRBIT             0x8000
#define MSEC_VAL_MASK           0x7fff

/*
 * end "fieldbits.h"
 */

/*
 * begin "proto.h"
 */

/** Protocol version number */
#define KRB5_PVNO       5

/* Message types */

#define KRB5_AS_REQ   ((krb5_msgtype)10) /**< Initial authentication request */
#define KRB5_AS_REP   ((krb5_msgtype)11) /**< Response to AS request */
#define KRB5_TGS_REQ  ((krb5_msgtype)12) /**< Ticket granting server request */
#define KRB5_TGS_REP  ((krb5_msgtype)13) /**< Response to TGS request */
#define KRB5_AP_REQ   ((krb5_msgtype)14) /**< Auth req to application server */
#define KRB5_AP_REP   ((krb5_msgtype)15) /**< Response to mutual AP request */
#define KRB5_SAFE     ((krb5_msgtype)20) /**< Safe application message */
#define KRB5_PRIV     ((krb5_msgtype)21) /**< Private application message */
#define KRB5_CRED     ((krb5_msgtype)22) /**< Cred forwarding message */
#define KRB5_ERROR    ((krb5_msgtype)30) /**< Error response */

/* LastReq types */
#define KRB5_LRQ_NONE                   0
#define KRB5_LRQ_ALL_LAST_TGT           1
#define KRB5_LRQ_ONE_LAST_TGT           (-1)
#define KRB5_LRQ_ALL_LAST_INITIAL       2
#define KRB5_LRQ_ONE_LAST_INITIAL       (-2)
#define KRB5_LRQ_ALL_LAST_TGT_ISSUED    3
#define KRB5_LRQ_ONE_LAST_TGT_ISSUED    (-3)
#define KRB5_LRQ_ALL_LAST_RENEWAL       4
#define KRB5_LRQ_ONE_LAST_RENEWAL       (-4)
#define KRB5_LRQ_ALL_LAST_REQ           5
#define KRB5_LRQ_ONE_LAST_REQ           (-5)
#define KRB5_LRQ_ALL_PW_EXPTIME         6
#define KRB5_LRQ_ONE_PW_EXPTIME         (-6)
#define KRB5_LRQ_ALL_ACCT_EXPTIME       7
#define KRB5_LRQ_ONE_ACCT_EXPTIME       (-7)

/* PADATA types */
#define KRB5_PADATA_NONE                0
#define KRB5_PADATA_AP_REQ              1
#define KRB5_PADATA_TGS_REQ             KRB5_PADATA_AP_REQ
#define KRB5_PADATA_ENC_TIMESTAMP       2 /**< RFC 4120 */
#define KRB5_PADATA_PW_SALT             3 /**< RFC 4120 */
#if 0                           /* Not used */
#define KRB5_PADATA_ENC_ENCKEY          4  /* Key encrypted within itself */
#endif
#define KRB5_PADATA_ENC_UNIX_TIME       5  /**< timestamp encrypted in key. RFC 4120 */
#define KRB5_PADATA_ENC_SANDIA_SECURID  6  /**< SecurId passcode. RFC 4120 */
#define KRB5_PADATA_SESAME              7  /**< Sesame project. RFC 4120 */
#define KRB5_PADATA_OSF_DCE             8  /**< OSF DCE. RFC 4120 */
#define KRB5_CYBERSAFE_SECUREID         9  /**< Cybersafe. RFC 4120 */
#define KRB5_PADATA_AFS3_SALT           10 /**< Cygnus. RFC 4120, 3961 */
#define KRB5_PADATA_ETYPE_INFO          11 /**< Etype info for preauth. RFC 4120 */
#define KRB5_PADATA_SAM_CHALLENGE       12 /**< SAM/OTP */
#define KRB5_PADATA_SAM_RESPONSE        13 /**< SAM/OTP */
#define KRB5_PADATA_PK_AS_REQ_OLD       14 /**< PKINIT */
#define KRB5_PADATA_PK_AS_REP_OLD       15 /**< PKINIT */
#define KRB5_PADATA_PK_AS_REQ           16 /**< PKINIT. RFC 4556 */
#define KRB5_PADATA_PK_AS_REP           17 /**< PKINIT. RFC 4556 */
#define KRB5_PADATA_ETYPE_INFO2         19 /**< RFC 4120 */
#define KRB5_PADATA_USE_SPECIFIED_KVNO  20 /**< RFC 4120 */
#define KRB5_PADATA_SVR_REFERRAL_INFO   20 /**< Windows 2000 referrals. RFC 6820 */
#define KRB5_PADATA_SAM_REDIRECT        21 /**< SAM/OTP. RFC 4120 */
#define KRB5_PADATA_GET_FROM_TYPED_DATA 22 /**< Embedded in typed data. RFC 4120 */
#define KRB5_PADATA_REFERRAL            25 /**< draft referral system */
#define KRB5_PADATA_SAM_CHALLENGE_2     30 /**< draft challenge system, updated */
#define KRB5_PADATA_SAM_RESPONSE_2      31 /**< draft challenge system, updated */
/* MS-KILE */
#define KRB5_PADATA_PAC_REQUEST         128 /**< include Windows PAC */
#define KRB5_PADATA_FOR_USER            129 /**< username protocol transition request */
#define KRB5_PADATA_S4U_X509_USER       130 /**< certificate protocol transition request */
#define KRB5_PADATA_AS_CHECKSUM         132 /**< AS checksum */
#define KRB5_PADATA_FX_COOKIE           133 /**< RFC 6113 */
#define KRB5_PADATA_FX_FAST             136 /**< RFC 6113 */
#define KRB5_PADATA_FX_ERROR            137 /**< RFC 6113 */
#define KRB5_PADATA_ENCRYPTED_CHALLENGE 138 /**< RFC 6113 */
#define KRB5_PADATA_OTP_CHALLENGE       141 /**< RFC 6560 section 4.1 */
#define KRB5_PADATA_OTP_REQUEST         142 /**< RFC 6560 section 4.2 */
#define KRB5_PADATA_OTP_PIN_CHANGE      144 /**< RFC 6560 section 4.3 */
#define KRB5_PADATA_PKINIT_KX           147 /**< RFC 6112 */
#define KRB5_ENCPADATA_REQ_ENC_PA_REP   149 /**< RFC 6806 */
#define KRB5_PADATA_AS_FRESHNESS        150 /**< RFC 8070 */
#define KRB5_PADATA_SPAKE               151
#define KRB5_PADATA_PAC_OPTIONS         167 /**< MS-KILE and MS-SFU */

#define KRB5_SAM_USE_SAD_AS_KEY         0x80000000
#define KRB5_SAM_SEND_ENCRYPTED_SAD     0x40000000
#define KRB5_SAM_MUST_PK_ENCRYPT_SAD    0x20000000 /**< currently must be zero */

/** Transited encoding types */
#define KRB5_DOMAIN_X500_COMPRESS               1

/** alternate authentication types */
#define KRB5_ALTAUTH_ATT_CHALLENGE_RESPONSE     64

/* authorization data types. See RFC 4120 section 5.2.6 */

/** @defgroup KRB5_AUTHDATA KRB5_AUTHDATA
 * @{
 */
#define KRB5_AUTHDATA_IF_RELEVANT   1
#define KRB5_AUTHDATA_KDC_ISSUED    4
#define KRB5_AUTHDATA_AND_OR        5
#define KRB5_AUTHDATA_MANDATORY_FOR_KDC 8
#define KRB5_AUTHDATA_INITIAL_VERIFIED_CAS      9
#define KRB5_AUTHDATA_OSF_DCE   64
#define KRB5_AUTHDATA_SESAME    65
#define KRB5_AUTHDATA_CAMMAC    96
#define KRB5_AUTHDATA_WIN2K_PAC 128
#define KRB5_AUTHDATA_ETYPE_NEGOTIATION 129     /**< RFC 4537 */
#define KRB5_AUTHDATA_SIGNTICKET        512     /**< formerly 142 in krb5 1.8 */
#define KRB5_AUTHDATA_FX_ARMOR 71
#define KRB5_AUTHDATA_AUTH_INDICATOR 97
/** @} */ /* end of KRB5_AUTHDATA group */

/* password change constants */
#define KRB5_KPASSWD_SUCCESS            0  /**< Success */
#define KRB5_KPASSWD_MALFORMED          1  /**< Malformed request */
#define KRB5_KPASSWD_HARDERROR          2  /**< Server error */
#define KRB5_KPASSWD_AUTHERROR          3  /**< Authentication error */
#define KRB5_KPASSWD_SOFTERROR          4  /**< Password change rejected */
/* These are Microsoft's extensions in RFC 3244, and it looks like
   they'll become standardized, possibly with other additions.  */
#define KRB5_KPASSWD_ACCESSDENIED       5  /**< Not authorized */
#define KRB5_KPASSWD_BAD_VERSION        6  /**< Unknown RPC version */
/** The presented credentials were not obtained using a password directly */
#define KRB5_KPASSWD_INITIAL_FLAG_NEEDED 7

/*
 * end "proto.h"
 */

/* Time set */
/** Ticket start time, end time, and renewal duration. */
typedef struct _krb5_ticket_times {
    krb5_timestamp authtime;    /**< Time at which KDC issued the initial ticket that corresponds to this ticket */
    /* XXX ? should ktime in KDC_REP == authtime
       in ticket? otherwise client can't get this */
    krb5_timestamp starttime;   /**< optional in ticket, if not present, use @a authtime */
    krb5_timestamp endtime;     /**< Ticket expiration time */
    krb5_timestamp renew_till;  /**< Latest time at which renewal of ticket can be valid */
} krb5_ticket_times;

/** Structure for auth data */
typedef struct _krb5_authdata {
    krb5_magic magic;
    krb5_authdatatype ad_type; /**< ADTYPE */
    unsigned int length;       /**< Length of data  */
    krb5_octet *contents;      /**< Data */
} krb5_authdata;

/** Structure for transited encoding */
typedef struct _krb5_transited {
    krb5_magic magic;
    krb5_octet tr_type;     /**< Transited encoding type */
    krb5_data tr_contents;  /**< Contents */
} krb5_transited;

/** Encrypted part of ticket. */
typedef struct _krb5_enc_tkt_part {
    krb5_magic magic;
    /* to-be-encrypted portion */
    krb5_flags flags;                   /**< flags */
    krb5_keyblock *session;             /**< session key: includes enctype */
    krb5_principal client;              /**< client name/realm */
    krb5_transited transited;           /**< list of transited realms */
    krb5_ticket_times times;            /**< auth, start, end, renew_till */
    krb5_address **caddrs;              /**< array of ptrs to addresses */
    krb5_authdata **authorization_data; /**< auth data */
} krb5_enc_tkt_part;

/**
 * Ticket structure.
 *
 * The C representation of the ticket message, with a pointer to the
 * C representation of the encrypted part.
 */
typedef struct _krb5_ticket {
    krb5_magic magic;
    /* cleartext portion */
    krb5_principal server;              /**< server name/realm */
    krb5_enc_data enc_part;             /**< encryption type, kvno, encrypted encoding */
    krb5_enc_tkt_part *enc_part2;       /**< ptr to decrypted version, if available */
} krb5_ticket;

/* the unencrypted version */
/**
 * Ticket authenticator.
 *
 * The C representation of an unencrypted authenticator.
 */
typedef struct _krb5_authenticator {
    krb5_magic magic;
    krb5_principal client;              /**< client name/realm */
    krb5_checksum *checksum;            /**< checksum, includes type, optional */
    krb5_int32 cusec;                   /**< client usec portion */
    krb5_timestamp ctime;               /**< client sec portion */
    krb5_keyblock *subkey;              /**< true session key, optional */
    krb5_ui_4 seq_number;               /**< sequence #, optional */
    krb5_authdata **authorization_data; /**< authoriazation data */
} krb5_authenticator;

/** Ticket authentication data. */
typedef struct _krb5_tkt_authent {
    krb5_magic magic;
    krb5_ticket *ticket;
    krb5_authenticator *authenticator;
    krb5_flags ap_options;
} krb5_tkt_authent;

/** Credentials structure including ticket, session key, and lifetime info. */
typedef struct _krb5_creds {
    krb5_magic magic;
    krb5_principal client;              /**< client's principal identifier */
    krb5_principal server;              /**< server's principal identifier */
    krb5_keyblock keyblock;             /**< session encryption key info */
    krb5_ticket_times times;            /**< lifetime info */
    krb5_boolean is_skey;               /**< true if ticket is encrypted in
                                           another ticket's skey */
    krb5_flags ticket_flags;            /**< flags in ticket */
    krb5_address **addresses;           /**< addrs in ticket */
    krb5_data ticket;                   /**< ticket string itself */
    krb5_data second_ticket;            /**< second ticket, if related to
                                           ticket (via DUPLICATE-SKEY or
                                           ENC-TKT-IN-SKEY) */
    krb5_authdata **authdata;           /**< authorization data */
} krb5_creds;

/** Last request entry */
typedef struct _krb5_last_req_entry {
    krb5_magic magic;
    krb5_int32 lr_type;         /**< LR type */
    krb5_timestamp value;       /**< Timestamp */
} krb5_last_req_entry;

/** Pre-authentication data */
typedef struct _krb5_pa_data {
    krb5_magic magic;
    krb5_preauthtype pa_type;   /**< Preauthentication data type */
    unsigned int length;        /**< Length of data */
    krb5_octet *contents;       /**< Data */
} krb5_pa_data;

/* Don't use this; use krb5_pa_data instead. */
typedef struct _krb5_typed_data {
    krb5_magic magic;
    krb5_int32 type;
    unsigned int length;
    krb5_octet *data;
} krb5_typed_data;

/** C representation of KDC-REQ protocol message, including KDC-REQ-BODY */
typedef struct _krb5_kdc_req {
    krb5_magic magic;
    krb5_msgtype msg_type;      /**< KRB5_AS_REQ or KRB5_TGS_REQ */
    krb5_pa_data **padata;      /**< Preauthentication data */
    /* real body */
    krb5_flags kdc_options;     /**< Requested options */
    krb5_principal client;      /**< Client principal and realm */
    krb5_principal server;      /**< Server principal and realm */
    krb5_timestamp from;        /**< Requested start time */
    krb5_timestamp till;        /**< Requested end time */
    krb5_timestamp rtime;       /**< Requested renewable end time */
    krb5_int32 nonce;           /**< Nonce to match request and response */
    int nktypes;                /**< Number of enctypes */
    krb5_enctype *ktype;        /**< Requested enctypes */
    krb5_address **addresses;   /**< Requested addresses (optional) */
    krb5_enc_data authorization_data;  /**< Encrypted authz data (optional) */
    krb5_authdata **unenc_authdata;    /**< Unencrypted authz data */
    krb5_ticket **second_ticket;       /**< Second ticket array (optional) */
} krb5_kdc_req;

/**
 * C representation of @c EncKDCRepPart protocol message.
 *
 * This is the cleartext message that is encrypted and inserted in @c KDC-REP.
 */
typedef struct _krb5_enc_kdc_rep_part {
    krb5_magic magic;
    /* encrypted part: */
    krb5_msgtype msg_type;           /**< krb5 message type */
    krb5_keyblock *session;          /**< Session key */
    krb5_last_req_entry **last_req;  /**< Array of pointers to entries */
    krb5_int32 nonce;                /**< Nonce from request */
    krb5_timestamp key_exp;          /**< Expiration date */
    krb5_flags flags;                /**< Ticket flags */
    krb5_ticket_times times;         /**< Lifetime info */
    krb5_principal server;           /**< Server's principal identifier */
    krb5_address **caddrs;           /**< Array of ptrs to addrs, optional */
    krb5_pa_data **enc_padata;       /**< Encrypted preauthentication data */
} krb5_enc_kdc_rep_part;

/** Representation of the @c KDC-REP protocol message. */
typedef struct _krb5_kdc_rep {
    krb5_magic magic;
    /* cleartext part: */
    krb5_msgtype msg_type;            /**< KRB5_AS_REP or KRB5_KDC_REP */
    krb5_pa_data **padata;            /**< Preauthentication data from KDC */
    krb5_principal client;            /**< Client principal and realm */
    krb5_ticket *ticket;              /**< Ticket */
    krb5_enc_data enc_part;           /**< Encrypted part of reply */
    krb5_enc_kdc_rep_part *enc_part2; /**< Unencrypted version, if available */
} krb5_kdc_rep;

/** Error message structure */
typedef struct _krb5_error {
    krb5_magic magic;
    /* some of these may be meaningless in certain contexts */
    krb5_timestamp ctime;       /**< Client sec portion; optional */
    krb5_int32 cusec;           /**< Client usec portion; optional */
    krb5_int32 susec;           /**< Server usec portion */
    krb5_timestamp stime;       /**< Server sec portion */
    krb5_ui_4 error;            /**< Error code (protocol error #'s) */
    krb5_principal client;      /**< Client principal and realm */
    krb5_principal server;      /**< Server principal and realm */
    krb5_data text;             /**< Descriptive text */
    krb5_data e_data;           /**< Additional error-describing data */
} krb5_error;

/** Authentication header. */
typedef struct _krb5_ap_req {
    krb5_magic magic;
    krb5_flags ap_options;        /**< Requested options */
    krb5_ticket *ticket;          /**< Ticket */
    krb5_enc_data authenticator;  /**< Encrypted authenticator */
} krb5_ap_req;

/**
 * C representaton of AP-REP message.
 *
 * The server's response to a client's request for mutual authentication.
 */
typedef struct _krb5_ap_rep {
    krb5_magic magic;
    krb5_enc_data enc_part;     /**< Ciphertext of ApRepEncPart */
} krb5_ap_rep;

/** Cleartext that is encrypted and put into @c _krb5_ap_rep.  */
typedef struct _krb5_ap_rep_enc_part {
    krb5_magic magic;
    krb5_timestamp ctime;       /**< Client time, seconds portion */
    krb5_int32 cusec;           /**< Client time, microseconds portion */
    krb5_keyblock *subkey;      /**< Subkey (optional) */
    krb5_ui_4 seq_number;       /**< Sequence number */
} krb5_ap_rep_enc_part;

/* Unused */
typedef struct _krb5_response {
    krb5_magic magic;
    krb5_octet message_type;
    krb5_data response;
    krb5_int32 expected_nonce;
    krb5_timestamp request_time;
} krb5_response;

/** Credentials information inserted into @c EncKrbCredPart. */
typedef struct _krb5_cred_info {
    krb5_magic magic;
    krb5_keyblock *session;     /**< Session key used to encrypt ticket */
    krb5_principal client;      /**< Client principal and realm */
    krb5_principal server;      /**< Server principal and realm */
    krb5_flags flags;           /**< Ticket flags */
    krb5_ticket_times times;    /**< Auth, start, end, renew_till */
    krb5_address **caddrs;      /**< Array of pointers to addrs (optional) */
} krb5_cred_info;

/** Cleartext credentials information.  */
typedef struct _krb5_cred_enc_part {
    krb5_magic magic;
    krb5_int32 nonce;           /**< Nonce (optional) */
    krb5_timestamp timestamp;   /**< Generation time, seconds portion */
    krb5_int32 usec;            /**< Generation time, microseconds portion */
    krb5_address *s_address;    /**< Sender address (optional) */
    krb5_address *r_address;    /**< Recipient address (optional) */
    krb5_cred_info **ticket_info;
} krb5_cred_enc_part;

/** Credentials data structure.*/
typedef struct _krb5_cred {
    krb5_magic magic;
    krb5_ticket **tickets;          /**< Tickets */
    krb5_enc_data enc_part;         /**< Encrypted part */
    krb5_cred_enc_part *enc_part2;  /**< Unencrypted version, if available */
} krb5_cred;

/* Unused, but here for API compatibility. */
typedef struct _passwd_phrase_element {
    krb5_magic magic;
    krb5_data *passwd;
    krb5_data *phrase;
} passwd_phrase_element;

/* Unused, but here for API compatibility. */
typedef struct _krb5_pwd_data {
    krb5_magic magic;
    int sequence_count;
    passwd_phrase_element **element;
} krb5_pwd_data;

/* Unused, but here for API compatibility. */
typedef struct _krb5_pa_svr_referral_data {
    /** Referred name, only realm is required */
    krb5_principal     principal;
} krb5_pa_svr_referral_data;

/* Unused, but here for API compatibility. */
typedef struct _krb5_pa_server_referral_data {
    krb5_data          *referred_realm;
    krb5_principal     true_principal_name;
    krb5_principal     requested_principal_name;
    krb5_timestamp     referral_valid_until;
    krb5_checksum      rep_cksum;
} krb5_pa_server_referral_data;

typedef struct _krb5_pa_pac_req {
    /** TRUE if a PAC should be included in TGS-REP */
    krb5_boolean       include_pac;
} krb5_pa_pac_req;

/*
 * begin "safepriv.h"
 */

/** @defgroup KRB5_AUTH_CONTEXT KRB5_AUTH_CONTEXT
 * @{
 */
/** Prevent replays with timestamps and replay cache. */
#define KRB5_AUTH_CONTEXT_DO_TIME       0x00000001
/** Save timestamps for application. */
#define KRB5_AUTH_CONTEXT_RET_TIME      0x00000002
/** Prevent replays with sequence numbers. */
#define KRB5_AUTH_CONTEXT_DO_SEQUENCE   0x00000004
/** Save sequence numbers for application. */
#define KRB5_AUTH_CONTEXT_RET_SEQUENCE  0x00000008
#define KRB5_AUTH_CONTEXT_PERMIT_ALL    0x00000010
#define KRB5_AUTH_CONTEXT_USE_SUBKEY    0x00000020
/** @} */ /* end of KRB5_AUTH_CONTEXT group */

/**
 * Replay data.
 *
 * Sequence number and timestamp information output by krb5_rd_priv() and
 * krb5_rd_safe().
 */
typedef struct krb5_replay_data {
    krb5_timestamp      timestamp;  /**< Timestamp, seconds portion */
    krb5_int32          usec;       /**< Timestamp, microseconds portion */
    krb5_ui_4           seq;        /**< Sequence number  */
} krb5_replay_data;

/* Flags for krb5_auth_con_genaddrs(). */

/** Generate the local network address. */
#define KRB5_AUTH_CONTEXT_GENERATE_LOCAL_ADDR       0x00000001
/** Generate the remote network address.  */
#define KRB5_AUTH_CONTEXT_GENERATE_REMOTE_ADDR      0x00000002
/** Generate the local network address and the local port. */
#define KRB5_AUTH_CONTEXT_GENERATE_LOCAL_FULL_ADDR  0x00000004
/** Generate the remote network address and the remote port. */
#define KRB5_AUTH_CONTEXT_GENERATE_REMOTE_FULL_ADDR 0x00000008

/** Type of function used as a callback to generate checksum data for mk_req */
typedef krb5_error_code
(KRB5_CALLCONV * krb5_mk_req_checksum_func)(krb5_context, krb5_auth_context,
                                            void *, krb5_data **);

/*
 * end "safepriv.h"
 */


/*
 * begin "ccache.h"
 */

/** Cursor for sequential lookup */
typedef krb5_pointer    krb5_cc_cursor;

struct _krb5_ccache;
typedef struct _krb5_ccache *krb5_ccache;
struct _krb5_cc_ops;
typedef struct _krb5_cc_ops krb5_cc_ops;

struct _krb5_cccol_cursor;
/** Cursor for iterating over all ccaches */
typedef struct _krb5_cccol_cursor *krb5_cccol_cursor;

/* Flags for krb5_cc_retrieve_cred. */
/** The requested lifetime must be at least as great as the time specified. */
#define KRB5_TC_MATCH_TIMES        0x00000001
/** The is_skey field must match exactly. */
#define KRB5_TC_MATCH_IS_SKEY      0x00000002
/** All the flags set in the match credentials must be set. */
#define KRB5_TC_MATCH_FLAGS        0x00000004
/** All the time fields must match exactly. */
#define KRB5_TC_MATCH_TIMES_EXACT  0x00000008
/** All the flags must match exactly. */
#define KRB5_TC_MATCH_FLAGS_EXACT  0x00000010
/** The authorization data must match. */
#define KRB5_TC_MATCH_AUTHDATA     0x00000020
/** Only the name portion of the principal name must match. */
#define KRB5_TC_MATCH_SRV_NAMEONLY 0x00000040
/** The second ticket must match. */
#define KRB5_TC_MATCH_2ND_TKT      0x00000080
/** The encryption key type must match. */
#define KRB5_TC_MATCH_KTYPE        0x00000100
/** The supported key types must match. */
#define KRB5_TC_SUPPORTED_KTYPES   0x00000200

/* Flags for krb5_cc_set_flags and similar. */
/** Open and close the file for each cache operation. */
#define KRB5_TC_OPENCLOSE          0x00000001 /**< @deprecated has no effect */
#define KRB5_TC_NOTICKET           0x00000002

/**
 * Retrieve the name, but not type of a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * @warning Returns the name of the credential cache.  The result is an alias
 * into @a cache and should not be freed or modified by the caller.  This name
 * does not include the cache type, so should not be used as input to
 * krb5_cc_resolve().
 *
 * @return
 * On success - the name of the credential cache.
 */
const char * KRB5_CALLCONV
krb5_cc_get_name(krb5_context context, krb5_ccache cache);

/**
 * Retrieve the full name of a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [out] fullname_out    Full name of cache
 *
 * Use krb5_free_string() to free @a fullname_out when it is no longer needed.
 *
 * @version New in 1.10
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_get_full_name(krb5_context context, krb5_ccache cache,
                      char **fullname_out);

#if KRB5_DEPRECATED
krb5_error_code KRB5_CALLCONV
krb5_cc_gen_new(krb5_context context, krb5_ccache *cache);
#endif /* KRB5_DEPRECATED */

/**
 * Initialize a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] principal        Default principal name
 *
 * Destroy any existing contents of @a cache and initialize it for the default
 * principal @a principal.
 *
 * @retval
 *  0  Success
 * @return
 *  System errors; Permission errors; Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_initialize(krb5_context context, krb5_ccache cache,
                   krb5_principal principal);

/**
 * Destroy a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * This function destroys any existing contents of @a cache and closes the
 * handle to it.
 *
 * @retval
 * 0  Success
 * @return
 * Permission errors
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_destroy(krb5_context context, krb5_ccache cache);

/**
 * Close a credential cache handle.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * This function closes a credential cache handle @a cache without affecting
 * the contents of the cache.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_close(krb5_context context, krb5_ccache cache);

/**
 * Store credentials in a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] creds            Credentials to be stored in cache
 *
 * This function stores @a creds into @a cache.  If @a creds->server and the
 * server in the decoded ticket @a creds->ticket differ, the credentials will
 * be stored under both server principal names.
 *
 * @retval
 *  0  Success
 * @return Permission errors; storage failure errors; Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_store_cred(krb5_context context, krb5_ccache cache, krb5_creds *creds);

/**
 * Retrieve a specified credentials from a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [in]  flags           Flags bit mask
 * @param [in]  mcreds          Credentials to match
 * @param [out] creds           Credentials matching the requested value
 *
 * This function searches a credential cache for credentials matching @a mcreds
 * and returns it if found.
 *
 * Valid values for @a flags are:
 *
 * @li #KRB5_TC_MATCH_TIMES        The requested lifetime must be at least as
 *                                 great as in @a mcreds .
 * @li #KRB5_TC_MATCH_IS_SKEY      The @a is_skey field much match exactly.
 * @li #KRB5_TC_MATCH_FLAGS        Flags set in @a mcreds must be set.
 * @li #KRB5_TC_MATCH_TIMES_EXACT  The requested lifetime must match exactly.
 * @li #KRB5_TC_MATCH_FLAGS_EXACT  Flags must match exactly.
 * @li #KRB5_TC_MATCH_AUTHDATA     The authorization data must match.
 * @li #KRB5_TC_MATCH_SRV_NAMEONLY Only the name portion of the principal
 *                                 name must match, not the realm.
 * @li #KRB5_TC_MATCH_2ND_TKT      The second tickets must match.
 * @li #KRB5_TC_MATCH_KTYPE        The encryption key types must match.
 * @li #KRB5_TC_SUPPORTED_KTYPES   Check all matching entries that have any
 *                                 supported encryption type and return the
 *                                 one with the encryption type listed earliest.
 *
 * Use krb5_free_cred_contents() to free @a creds when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_retrieve_cred(krb5_context context, krb5_ccache cache,
                      krb5_flags flags, krb5_creds *mcreds,
                      krb5_creds *creds);

/**
 * Get the default principal of a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [out] principal       Primary principal
 *
 * Returns the default client principal of a credential cache as set by
 * krb5_cc_initialize().
 *
 * Use krb5_free_principal() to free @a principal when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_get_principal(krb5_context context, krb5_ccache cache,
                      krb5_principal *principal);

/**
 * Prepare to sequentially read every credential in a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [out] cursor          Cursor
 *
 * krb5_cc_end_seq_get() must be called to complete the retrieve operation.
 *
 * @note If the cache represented by @a cache is modified between the time of
 * the call to this function and the time of the final krb5_cc_end_seq_get(),
 * these changes may not be reflected in the results of krb5_cc_next_cred()
 * calls.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_start_seq_get(krb5_context context, krb5_ccache cache,
                      krb5_cc_cursor *cursor);

/**
 * Retrieve the next entry from the credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [in]  cursor          Cursor
 * @param [out] creds           Next credential cache entry
 *
 * This function fills in @a creds with the next entry in @a cache and advances
 * @a cursor.
 *
 * Use krb5_free_cred_contents() to free @a creds when it is no longer needed.
 *
 * @sa krb5_cc_start_seq_get(), krb5_end_seq_get()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_next_cred(krb5_context context, krb5_ccache cache,
                  krb5_cc_cursor *cursor, krb5_creds *creds);

/**
 * Finish a series of sequential processing credential cache entries.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] cursor           Cursor
 *
 * This function finishes processing credential cache entries and invalidates
 * @a cursor.
 *
 * @sa krb5_cc_start_seq_get(), krb5_cc_next_cred()
 *
 * @retval 0 (always)
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_end_seq_get(krb5_context context, krb5_ccache cache,
                    krb5_cc_cursor *cursor);

/**
 * Remove credentials from a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] flags            Bitwise-ORed search flags
 * @param [in] creds            Credentials to be matched
 *
 * @warning This function is not implemented for some cache types.
 *
 * This function accepts the same flag values as krb5_cc_retrieve_cred().
 *
 * @retval KRB5_CC_NOSUPP Not implemented for this cache type
 * @return No matches found; Data cannot be deleted; Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_remove_cred(krb5_context context, krb5_ccache cache, krb5_flags flags,
                    krb5_creds *creds);

/**
 * Set options flags on a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] flags            Flag bit mask
 *
 * This function resets @a cache flags to @a flags.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_set_flags(krb5_context context, krb5_ccache cache, krb5_flags flags);

/**
 * Retrieve flags from a credential cache structure.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [out] flags           Flag bit mask
 *
 * @warning For memory credential cache always returns a flag mask of 0.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_get_flags(krb5_context context, krb5_ccache cache, krb5_flags *flags);

/**
 * Retrieve the type of a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * @return The type of a credential cache as an alias that must not be modified
 * or freed by the caller.
 */
const char * KRB5_CALLCONV
krb5_cc_get_type(krb5_context context, krb5_ccache cache);

/**
 * Move a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] src              The credential cache to move the content from
 * @param [in] dst              The credential cache to move the content to
 *
 * This function reinitializes @a dst and populates it with the credentials and
 * default principal of @a src; then, if successful, destroys @a src.
 *
 * @retval
 * 0 Success; @a src is closed.
 * @return
 * Kerberos error codes; @a src is still allocated.
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_move(krb5_context context, krb5_ccache src, krb5_ccache dst);

/**
 * Prepare to iterate over the collection of known credential caches.
 *
 * @param [in]  context         Library context
 * @param [out] cursor          Cursor
 *
 * Get a new cache iteration @a cursor that will iterate over all known
 * credential caches independent of type.
 *
 * Use krb5_cccol_cursor_free() to release @a cursor when it is no longer
 * needed.
 *
 * @sa krb5_cccol_cursor_next()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cccol_cursor_new(krb5_context context, krb5_cccol_cursor *cursor);

/**
 * Get the next credential cache in the collection.
 *
 * @param [in]  context         Library context
 * @param [in]  cursor          Cursor
 * @param [out] ccache          Credential cache handle
 *
 * @note When all caches are iterated over and the end of the list is reached,
 * @a ccache is set to NULL.
 *
 * Use krb5_cc_close() to close @a ccache when it is no longer needed.
 *
 * @sa krb5_cccol_cursor_new(), krb5_cccol_cursor_free()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cccol_cursor_next(krb5_context context, krb5_cccol_cursor cursor,
                       krb5_ccache *ccache);

/**
 * Free a credential cache collection cursor.
 *
 * @param [in] context          Library context
 * @param [in] cursor           Cursor
 *
 * @sa krb5_cccol_cursor_new(), krb5_cccol_cursor_next()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cccol_cursor_free(krb5_context context, krb5_cccol_cursor *cursor);

/**
 * Check if the credential cache collection contains any credentials.
 *
 * @param [in]  context         Library context
 *
 * @version New in 1.11
 *
 * @retval 0 Credentials are available in the collection
 * @retval KRB5_CC_NOTFOUND The collection contains no credentials
 */
krb5_error_code KRB5_CALLCONV
krb5_cccol_have_content(krb5_context context);

/**
 * Create a new credential cache of the specified type with a unique name.
 *
 * @param [in]  context         Library context
 * @param [in]  type            Credential cache type name
 * @param [in]  hint            Unused
 * @param [out] id              Credential cache handle
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_new_unique(krb5_context context, const char *type, const char *hint,
                   krb5_ccache *id);

/*
 * end "ccache.h"
 */

/*
 * begin "rcache.h"
 */

struct krb5_rc_st;
typedef struct krb5_rc_st *krb5_rcache;

/*
 * end "rcache.h"
 */

/*
 * begin "keytab.h"
 */


/* XXX */
#define MAX_KEYTAB_NAME_LEN 1100 /**< Long enough for MAXPATHLEN + some extra */

typedef krb5_pointer krb5_kt_cursor;

/** A key table entry. */
typedef struct krb5_keytab_entry_st {
    krb5_magic magic;
    krb5_principal principal;   /**< Principal of this key */
    krb5_timestamp timestamp;   /**< Time entry written to keytable */
    krb5_kvno vno;              /**< Key version number */
    krb5_keyblock key;          /**< The secret key */
} krb5_keytab_entry;

struct _krb5_kt;
typedef struct _krb5_kt *krb5_keytab;

/**
 * Return the type of a key table.
 *
 * @param [in] context          Library context
 * @param [in] keytab           Key table handle
 *
 * @return The type of a key table as an alias that must not be modified or
 * freed by the caller.
 */
const char * KRB5_CALLCONV
krb5_kt_get_type(krb5_context context, krb5_keytab keytab);

/**
 * Get a key table name.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] name            Key table name
 * @param [in]  namelen         Maximum length to fill in name
 *
 * Fill @a name with the name of @a keytab including the type and delimiter.
 *
 * @sa MAX_KEYTAB_NAME_LEN
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_KT_NAME_TOOLONG  Key table name does not fit in @a namelen bytes
 *
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_kt_get_name(krb5_context context, krb5_keytab keytab, char *name,
                 unsigned int namelen);

/**
 * Close a key table handle.
 *
 * @param [in] context          Library context
 * @param [in] keytab           Key table handle
 *
 * @retval 0
 */
krb5_error_code KRB5_CALLCONV
krb5_kt_close(krb5_context context, krb5_keytab keytab);

/**
 * Get an entry from a key table.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [in]  principal       Principal name
 * @param [in]  vno             Key version number (0 for highest available)
 * @param [in]  enctype         Encryption type (0 zero for any enctype)
 * @param [out] entry           Returned entry from key table
 *
 * Retrieve an entry from a key table which matches the @a keytab, @a
 * principal, @a vno, and @a enctype.  If @a vno is zero, retrieve the
 * highest-numbered kvno matching the other fields.  If @a enctype is 0, match
 * any enctype.
 *
 * Use krb5_free_keytab_entry_contents() to free @a entry when it is no longer
 * needed.
 *
 * @note If @a vno is zero, the function retrieves the highest-numbered-kvno
 * entry that matches the specified principal.
 *
 * @retval
 * 0 Success
 * @retval
 * Kerberos error codes on failure
 */
krb5_error_code KRB5_CALLCONV
krb5_kt_get_entry(krb5_context context, krb5_keytab keytab,
                  krb5_const_principal principal, krb5_kvno vno,
                  krb5_enctype enctype, krb5_keytab_entry *entry);

/**
 * Start a sequential retrieval of key table entries.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] cursor          Cursor
 *
 * Prepare to read sequentially every key in the specified key table.  Use
 * krb5_kt_end_seq_get() to release the cursor when it is no longer needed.
 *
 * @sa krb5_kt_next_entry(), krb5_kt_end_seq_get()
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_kt_start_seq_get(krb5_context context, krb5_keytab keytab,
                      krb5_kt_cursor *cursor);

/**
 * Retrieve the next entry from the key table.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] entry           Returned key table entry
 * @param [in]  cursor          Key table cursor
 *
 * Return the next sequential entry in @a keytab and advance @a cursor.
 * Callers must release the returned entry with krb5_kt_free_entry().
 *
 * @sa krb5_kt_start_seq_get(), krb5_kt_end_seq_get()
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_KT_END - if the last entry was reached
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_kt_next_entry(krb5_context context, krb5_keytab keytab,
                   krb5_keytab_entry *entry, krb5_kt_cursor *cursor);

/**
 * Release a keytab cursor.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] cursor          Cursor
 *
 * This function should be called to release the cursor created by
 * krb5_kt_start_seq_get().
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_kt_end_seq_get(krb5_context context, krb5_keytab keytab,
                    krb5_kt_cursor *cursor);

/**
 * Check if a keytab exists and contains entries.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 *
 * @version New in 1.11
 *
 * @retval 0 Keytab exists and contains entries
 * @retval KRB5_KT_NOTFOUND Keytab does not contain entries
 */
krb5_error_code KRB5_CALLCONV
krb5_kt_have_content(krb5_context context, krb5_keytab keytab);

/*
 * end "keytab.h"
 */

/*
 * begin "func-proto.h"
 */

#define KRB5_INIT_CONTEXT_SECURE 0x1 /**< Use secure context configuration */
#define KRB5_INIT_CONTEXT_KDC    0x2 /**< Use KDC configuration if available */

/**
 * Create a krb5 library context.
 *
 * @param [out] context         Library context
 *
 * The @a context must be released by calling krb5_free_context() when
 * it is no longer needed.
 *
 * @warning Any program or module that needs the Kerberos code to not trust the
 * environment must use krb5_init_secure_context(), or clean out the
 * environment.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_init_context(krb5_context *context);

/**
 * Create a krb5 library context using only configuration files.
 *
 * @param [out] context         Library context
 *
 * Create a context structure, using only system configuration files.  All
 * information passed through the environment variables is ignored.
 *
 * The @a context must be released by calling krb5_free_context() when
 * it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_init_secure_context(krb5_context *context);

/**
 * Create a krb5 library context using a specified profile.
 *
 * @param [in]  profile         Profile object (NULL to create default profile)
 * @param [in]  flags           Context initialization flags
 * @param [out] context         Library context
 *
 * Create a context structure, optionally using a specified profile and
 * initialization flags.  If @a profile is NULL, the default profile will be
 * created from config files.  If @a profile is non-null, a copy of it will be
 * made for the new context; the caller should still clean up its copy.  Valid
 * flag values are:
 *
 * @li #KRB5_INIT_CONTEXT_SECURE Ignore environment variables
 * @li #KRB5_INIT_CONTEXT_KDC    Use KDC configuration if creating profile
 */
krb5_error_code KRB5_CALLCONV
krb5_init_context_profile(struct _profile_t *profile, krb5_flags flags,
                          krb5_context *context);

/**
 * Free a krb5 library context.
 *
 * @param [in] context          Library context
 *
 * This function frees a @a context that was created by krb5_init_context()
 * or krb5_init_secure_context().
 */
void KRB5_CALLCONV
krb5_free_context(krb5_context context);

/**
 * Copy a krb5_context structure.
 *
 * @param [in]  ctx             Library context
 * @param [out] nctx_out        New context structure
 *
 * The newly created context must be released by calling krb5_free_context()
 * when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_copy_context(krb5_context ctx, krb5_context *nctx_out);

/**
 * Set default TGS encryption types in a krb5_context structure.
 *
 * @param [in] context          Library context
 * @param [in] etypes           Encryption type(s) to set
 *
 * This function sets the default enctype list for TGS requests
 * made using @a context to @a etypes.
 *
 * @note This overrides the default list (from config file or built-in).
 *
 * @retval
 *  0    Success
 * @retval
 *  KRB5_PROG_ETYPE_NOSUPP Program lacks support for encryption type
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_set_default_tgs_enctypes(krb5_context context, const krb5_enctype *etypes);

/**
 * Return a list of encryption types permitted for session keys.
 *
 * @param [in]  context         Library context
 * @param [out] ktypes          Zero-terminated list of encryption types
 *
 * This function returns the list of encryption types permitted for session
 * keys within @a context, as determined by configuration or by a previous call
 * to krb5_set_default_tgs_enctypes().
 *
 * Use krb5_free_enctypes() to free @a ktypes when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_get_permitted_enctypes(krb5_context context, krb5_enctype **ktypes);

/**
 * Test whether the Kerberos library was built with multithread support.
 *
 * @retval
 * TRUE if the library is threadsafe; FALSE otherwise
 */
krb5_boolean KRB5_CALLCONV
krb5_is_thread_safe(void);

/* libkrb.spec */

/**
 * Decrypt a ticket using the specified key table.
 *
 * @param [in] context          Library context
 * @param [in] kt               Key table
 * @param [in] ticket           Ticket to be decrypted
 *
 * This function takes a @a ticket as input and decrypts it using
 * key data from @a kt.  The result is placed into @a ticket->enc_part2.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_server_decrypt_ticket_keytab(krb5_context context, const krb5_keytab kt,
                                  krb5_ticket *ticket);

/**
 * Free an array of credential structures.
 *
 * @param [in] context          Library context
 * @param [in] tgts             Null-terminated array of credentials to free
 *
 * @note The last entry in the array @a tgts must be a NULL pointer.
 */
void KRB5_CALLCONV
krb5_free_tgt_creds(krb5_context context, krb5_creds **tgts);

/** @defgroup KRB5_GC  KRB5_GC
 * @{
 */
#define KRB5_GC_USER_USER    1  /**< Want user-user ticket */
#define KRB5_GC_CACHED       2  /**< Want cached ticket only */
#define KRB5_GC_CANONICALIZE 4  /**< Set canonicalize KDC option */
#define KRB5_GC_NO_STORE     8  /**< Do not store in credential cache */
#define KRB5_GC_FORWARDABLE             16  /**< Acquire forwardable tickets */
#define KRB5_GC_NO_TRANSIT_CHECK        32  /**< Disable transited check */
#define KRB5_GC_CONSTRAINED_DELEGATION  64  /**< Constrained delegation */
/** @} */ /* end of KRB5_GC group */

/**
 * Get an additional ticket.
 *
 * @param [in]  context         Library context
 * @param [in]  options         Options
 * @param [in]  ccache          Credential cache handle
 * @param [in]  in_creds        Input credentials
 * @param [out] out_creds       Output updated credentials
 *
 * Use @a ccache or a TGS exchange to get a service ticket matching @a
 * in_creds.
 *
 * Valid values for @a options are:
 * @li #KRB5_GC_CACHED     Search only credential cache for the ticket
 * @li #KRB5_GC_USER_USER  Return a user to user authentication ticket
 *
 * @a in_creds must be non-null.  @a in_creds->client and @a in_creds->server
 * must be filled in to specify the client and the server respectively.  If any
 * authorization data needs to be requested for the service ticket (such as
 * restrictions on how the ticket can be used), specify it in @a
 * in_creds->authdata; otherwise set @a in_creds->authdata to NULL.  The
 * session key type is specified in @a in_creds->keyblock.enctype, if it is
 * nonzero.
 *
 * The expiration date is specified in @a in_creds->times.endtime.
 * The KDC may return tickets with an earlier expiration date.
 * If @a in_creds->times.endtime is set to 0, the latest possible
 * expiration date will be requested.
 *
 * Any returned ticket and intermediate ticket-granting tickets are stored
 * in @a ccache.
 *
 * Use krb5_free_creds() to free @a out_creds when it is no longer needed.
 *
 * @retval
 *  0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_get_credentials(krb5_context context, krb5_flags options,
                     krb5_ccache ccache, krb5_creds *in_creds,
                     krb5_creds **out_creds);

/** @deprecated Replaced by krb5_get_validated_creds. */
krb5_error_code KRB5_CALLCONV
krb5_get_credentials_validate(krb5_context context, krb5_flags options,
                              krb5_ccache ccache, krb5_creds *in_creds,
                              krb5_creds **out_creds);

/** @deprecated Replaced by krb5_get_renewed_creds. */
krb5_error_code KRB5_CALLCONV
krb5_get_credentials_renew(krb5_context context, krb5_flags options,
                           krb5_ccache ccache, krb5_creds *in_creds,
                           krb5_creds **out_creds);

/**
 * Create a @c KRB_AP_REQ message.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     ap_req_options @ref AP_OPTS options
 * @param [in]     service        Service name, or NULL to use @c "host"
 * @param [in]     hostname       Host name, or NULL to use local hostname
 * @param [in]     in_data        Application data to be checksummed in the
 *                                authenticator, or NULL
 * @param [in]     ccache         Credential cache used to obtain credentials
 *                                for the desired service.
 * @param [out]    outbuf         @c AP-REQ message
 *
 * This function is similar to krb5_mk_req_extended() except that it uses a
 * given @a hostname, @a service, and @a ccache to construct a service
 * principal name and obtain credentials.
 *
 * Use krb5_free_data_contents() to free @a outbuf when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_mk_req(krb5_context context, krb5_auth_context *auth_context,
            krb5_flags ap_req_options, const char *service,
            const char *hostname, krb5_data *in_data, krb5_ccache ccache,
            krb5_data *outbuf);

/**
 * Create a @c KRB_AP_REQ message using supplied credentials.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     ap_req_options @ref AP_OPTS options
 * @param [in]     in_data        Application data to be checksummed in the
 *                                authenticator, or NULL
 * @param [in]     in_creds       Credentials for the service with valid ticket
 *                                and key
 * @param [out]    outbuf         @c AP-REQ message
 *
 * Valid @a ap_req_options are:
 * @li #AP_OPTS_USE_SESSION_KEY - Use the session key when creating the
 *                                request used for user to user
 *                                authentication.
 * @li #AP_OPTS_MUTUAL_REQUIRED - Request a mutual authentication packet from
 *                                the receiver.
 * @li #AP_OPTS_USE_SUBKEY      - Generate a subsession key from the current
 *                                session key obtained from the credentials.
 *
 * This function creates a KRB_AP_REQ message using supplied credentials @a
 * in_creds.  @a auth_context may point to an existing auth context or to NULL,
 * in which case a new one will be created.  If @a in_data is non-null, a
 * checksum of it will be included in the authenticator contained in the
 * KRB_AP_REQ message.  Use krb5_free_data_contents() to free @a outbuf when it
 * is no longer needed.
 *
 * On successful return, the authenticator is stored in @a auth_context with
 * the @a client and @a checksum fields nulled out.  (This is to prevent
 * pointer-sharing problems; the caller should not need these fields anyway,
 * since the caller supplied them.)
 *
 * @sa krb5_mk_req()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_mk_req_extended(krb5_context context, krb5_auth_context *auth_context,
                     krb5_flags ap_req_options, krb5_data *in_data,
                     krb5_creds *in_creds, krb5_data *outbuf);

/**
 * Format and encrypt a @c KRB_AP_REP message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] outbuf          @c AP-REP message
 *
 * This function fills in @a outbuf with an AP-REP message using information
 * from @a auth_context.
 *
 * If the flags in @a auth_context indicate that a sequence number should be
 * used (either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or
 * #KRB5_AUTH_CONTEXT_RET_SEQUENCE) and the local sequence number in @a
 * auth_context is 0, a new number will be generated with
 * krb5_generate_seq_number().
 *
 * Use krb5_free_data_contents() to free @a outbuf when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_mk_rep(krb5_context context, krb5_auth_context auth_context, krb5_data *outbuf);

/**
 * Format and encrypt a @c KRB_AP_REP message for DCE RPC.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] outbuf          @c AP-REP message
 *
 * Use krb5_free_data_contents() to free @a outbuf when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_mk_rep_dce(krb5_context context, krb5_auth_context auth_context, krb5_data *outbuf);

/**
 * Parse and decrypt a @c KRB_AP_REP message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  inbuf           AP-REP message
 * @param [out] repl            Decrypted reply message
 *
 * This function parses, decrypts and verifies a message from @a inbuf and
 * fills in @a repl with a pointer to allocated memory containing the fields
 * from the encrypted response.
 *
 * Use krb5_free_ap_rep_enc_part() to free @a repl when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_rd_rep(krb5_context context, krb5_auth_context auth_context,
            const krb5_data *inbuf, krb5_ap_rep_enc_part **repl);

/**
 * Parse and decrypt a @c KRB_AP_REP message for DCE RPC.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  inbuf           AP-REP message
 * @param [out] nonce           Sequence number from the decrypted reply
 *
 * This function parses, decrypts and verifies a message from @a inbuf and
 * fills in @a nonce with a decrypted reply sequence number.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_rd_rep_dce(krb5_context context, krb5_auth_context auth_context,
                const krb5_data *inbuf, krb5_ui_4 *nonce);

/**
 * Format and encode a @c KRB_ERROR message.
 *
 * @param [in]  context         Library context
 * @param [in]  dec_err         Error structure to be encoded
 * @param [out] enc_err         Encoded error structure
 *
 * This function creates a @c KRB_ERROR message in @a enc_err.  Use
 * krb5_free_data_contents() to free @a enc_err when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_mk_error(krb5_context context, const krb5_error *dec_err,
              krb5_data *enc_err);

/**
 * Decode a @c KRB-ERROR message.
 *
 * @param [in]  context         Library context
 * @param [in]  enc_errbuf      Encoded error message
 * @param [out] dec_error       Decoded error message
 *
 * This function processes @c KRB-ERROR message @a enc_errbuf and returns
 * an allocated structure @a dec_error containing the error message.
 * Use krb5_free_error() to free @a dec_error when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_rd_error(krb5_context context, const krb5_data *enc_errbuf,
              krb5_error **dec_error);

/**
 * Process @c KRB-SAFE message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  inbuf           @c KRB-SAFE message to be parsed
 * @param [out] userdata_out    Data parsed from @c KRB-SAFE message
 * @param [out] rdata_out       Replay data. Specify NULL if not needed
 *
 * This function parses a @c KRB-SAFE message, verifies its integrity, and
 * stores its data into @a userdata_out.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If @a auth_context has a remote address set, the address will be used to
 * verify the sender address in the KRB-SAFE message.  If @a auth_context has a
 * local address set, it will be used to verify the receiver address in the
 * KRB-SAFE message if the message contains one.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag is set in @a auth_context, the
 * sequence number of the KRB-SAFE message is checked against the remote
 * sequence number field of @a auth_context.  Otherwise, the sequence number is
 * not used.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, then the
 * timestamp in the message is verified to be within the permitted clock skew
 * of the current time, and the message is checked against an in-memory replay
 * cache to detect reflections or replays.
 *
 * Use krb5_free_data_contents() to free @a userdata_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_rd_safe(krb5_context context, krb5_auth_context auth_context,
             const krb5_data *inbuf, krb5_data *userdata_out,
             krb5_replay_data *rdata_out);

/**
 * Process a @c KRB-PRIV message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication structure
 * @param [in]  inbuf           @c KRB-PRIV message to be parsed
 * @param [out] userdata_out    Data parsed from @c KRB-PRIV message
 * @param [out] rdata_out       Replay data. Specify NULL if not needed
 *
 * This function parses a @c KRB-PRIV message, verifies its integrity, and
 * stores its unencrypted data into @a userdata_out.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If @a auth_context has a remote address set, the address will be used to
 * verify the sender address in the KRB-PRIV message.  If @a auth_context has a
 * local address set, it will be used to verify the receiver address in the
 * KRB-PRIV message if the message contains one.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag is set in @a auth_context, the
 * sequence number of the KRB-PRIV message is checked against the remote
 * sequence number field of @a auth_context.  Otherwise, the sequence number is
 * not used.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, then the
 * timestamp in the message is verified to be within the permitted clock skew
 * of the current time, and the message is checked against an in-memory replay
 * cache to detect reflections or replays.
 *
 * Use krb5_free_data_contents() to free @a userdata_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_rd_priv(krb5_context context, krb5_auth_context auth_context,
             const krb5_data *inbuf, krb5_data *userdata_out,
             krb5_replay_data *rdata_out);

/**
 * Convert a string principal name to a krb5_principal structure.
 *
 * @param [in]  context         Library context
 * @param [in]  name            String representation of a principal name
 * @param [out] principal_out   New principal
 *
 * Convert a string representation of a principal name to a krb5_principal
 * structure.
 *
 * A string representation of a Kerberos name consists of one or more principal
 * name components, separated by slashes, optionally followed by the \@
 * character and a realm name.  If the realm name is not specified, the local
 * realm is used.
 *
 * To use the slash and \@ symbols as part of a component (quoted) instead of
 * using them as a component separator or as a realm prefix), put a backslash
 * (\) character in front of the symbol.  Similarly, newline, tab, backspace,
 * and NULL characters can be included in a component by using @c n, @c t, @c b
 * or @c 0, respectively.
 *
 * @note The realm in a Kerberos @a name cannot contain slash, colon,
 * or NULL characters.
 *
 * Use krb5_free_principal() to free @a principal_out when it is no longer
 * needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_parse_name(krb5_context context, const char *name,
                krb5_principal *principal_out);

#define KRB5_PRINCIPAL_PARSE_NO_REALM      0x1 /**< Error if realm is present */
#define KRB5_PRINCIPAL_PARSE_REQUIRE_REALM 0x2 /**< Error if realm is not present */
#define KRB5_PRINCIPAL_PARSE_ENTERPRISE    0x4 /**< Create single-component
                                                  enterprise principle */
#define KRB5_PRINCIPAL_PARSE_IGNORE_REALM  0x8 /**< Ignore realm if present */

/**
 * Convert a string principal name to a krb5_principal with flags.
 *
 * @param [in]  context         Library context
 * @param [in]  name            String representation of a principal name
 * @param [in]  flags           Flag
 * @param [out] principal_out   New principal
 *
 * Similar to krb5_parse_name(), this function converts a single-string
 * representation of a principal name to a krb5_principal structure.
 *
 * The following flags are valid:
 * @li #KRB5_PRINCIPAL_PARSE_NO_REALM - no realm must be present in @a name
 * @li #KRB5_PRINCIPAL_PARSE_REQUIRE_REALM - realm must be present in @a name
 * @li #KRB5_PRINCIPAL_PARSE_ENTERPRISE - create single-component enterprise
 *                                        principal
 * @li #KRB5_PRINCIPAL_PARSE_IGNORE_REALM - ignore realm if present in @a name
 *
 * If @c KRB5_PRINCIPAL_PARSE_NO_REALM or @c KRB5_PRINCIPAL_PARSE_IGNORE_REALM
 * is specified in @a flags, the realm of the new principal will be empty.
 * Otherwise, the default realm for @a context will be used if @a name does not
 * specify a realm.
 *
 * Use krb5_free_principal() to free @a principal_out when it is no longer
 * needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_parse_name_flags(krb5_context context, const char *name,
                      int flags, krb5_principal *principal_out);

/**
 * Convert a krb5_principal structure to a string representation.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [out] name            String representation of principal name
 *
 * The resulting string representation uses the format and quoting conventions
 * described for krb5_parse_name().
 *
 * Use krb5_free_unparsed_name() to free @a name when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_unparse_name(krb5_context context, krb5_const_principal principal,
                  char **name);

/**
 * Convert krb5_principal structure to string and length.
 *
 * @param [in]     context      Library context
 * @param [in]     principal    Principal
 * @param [in,out] name         String representation of principal name
 * @param [in,out] size         Size of unparsed name
 *
 * This function is similar to krb5_unparse_name(), but allows the use of an
 * existing buffer for the result.  If size is not NULL, then @a name must
 * point to either NULL or an existing buffer of at least the size pointed to
 * by @a size.  The buffer will be allocated or resized if necessary, with the
 * new pointer stored into @a name.  Whether or not the buffer is resized, the
 * necessary space for the result, including null terminator, will be stored
 * into @a size.
 *
 * If size is NULL, this function behaves exactly as krb5_unparse_name().
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes. On failure @a name is set to NULL
 */
krb5_error_code KRB5_CALLCONV
krb5_unparse_name_ext(krb5_context context, krb5_const_principal principal,
                      char **name, unsigned int *size);

#define KRB5_PRINCIPAL_UNPARSE_SHORT  0x1 /**< Omit realm if it is the local realm */
#define KRB5_PRINCIPAL_UNPARSE_NO_REALM 0x2 /**< Omit realm always */
#define KRB5_PRINCIPAL_UNPARSE_DISPLAY  0x4 /**< Don't escape special characters */

/**
 * Convert krb5_principal structure to a string with flags.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [in]  flags           Flags
 * @param [out] name            String representation of principal name
 *
 * Similar to krb5_unparse_name(), this function converts a krb5_principal
 * structure to a string representation.
 *
 * The following flags are valid:
 * @li #KRB5_PRINCIPAL_UNPARSE_SHORT - omit realm if it is the local realm
 * @li #KRB5_PRINCIPAL_UNPARSE_NO_REALM - omit realm
 * @li #KRB5_PRINCIPAL_UNPARSE_DISPLAY - do not quote special characters
 *
 * Use krb5_free_unparsed_name() to free @a name when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes. On failure @a name is set to NULL
 */
krb5_error_code KRB5_CALLCONV
krb5_unparse_name_flags(krb5_context context, krb5_const_principal principal,
                        int flags, char **name);

/**
 * Convert krb5_principal structure to string format with flags.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [in]  flags           Flags
 * @param [out] name            Single string format of principal name
 * @param [out] size            Size of unparsed name buffer
 *
 * @sa krb5_unparse_name() krb5_unparse_name_flags() krb5_unparse_name_ext()
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes. On failure @a name is set to NULL
 */
krb5_error_code KRB5_CALLCONV
krb5_unparse_name_flags_ext(krb5_context context, krb5_const_principal principal,
                            int flags, char **name, unsigned int *size);

/**
 * Set the realm field of a principal
 *
 * @param [in] context          Library context
 * @param [in] principal        Principal name
 * @param [in] realm            Realm name
 *
 * Set the realm name part of @a principal to @a realm, overwriting the
 * previous realm.
 *
 * @retval
 * 0   Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_set_principal_realm(krb5_context context, krb5_principal principal,
                         const char *realm);

/**
 * Search a list of addresses for a specified address.
 *
 * @param [in] context          Library context
 * @param [in] addr             Address to search for
 * @param [in] addrlist         Address list to be searched (or NULL)
 *
 * @note If @a addrlist contains only a NetBIOS addresses, it will be treated
 *       as a null list.
 *
 * @return
 * TRUE if @a addr is listed in @a addrlist, or @c addrlist is NULL; FALSE
 * otherwise
 */
krb5_boolean KRB5_CALLCONV_WRONG
krb5_address_search(krb5_context context, const krb5_address *addr,
                    krb5_address *const *addrlist);

/**
 * Compare two Kerberos addresses.
 *
 * @param [in] context          Library context
 * @param [in] addr1            First address to be compared
 * @param [in] addr2            Second address to be compared
 *
 * @return
 * TRUE if the addresses are the same, FALSE otherwise
 */
krb5_boolean KRB5_CALLCONV
krb5_address_compare(krb5_context context, const krb5_address *addr1,
                     const krb5_address *addr2);

/**
 * Return an ordering of the specified addresses.
 *
 * @param [in] context          Library context
 * @param [in] addr1            First address
 * @param [in] addr2            Second address
 *
 * @retval
 *  0 The two addresses are the same
 * @retval
 *  \< 0 First address is less than second
 * @retval
 *  \> 0 First address is greater than second
 */
int KRB5_CALLCONV
krb5_address_order(krb5_context context, const krb5_address *addr1,
                   const krb5_address *addr2);

/**
 * Compare the realms of two principals.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * @retval
 * TRUE if the realm names are the same; FALSE otherwise
 */
krb5_boolean KRB5_CALLCONV
krb5_realm_compare(krb5_context context, krb5_const_principal princ1,
                   krb5_const_principal princ2);

/**
 * Compare two principals.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * @retval
 * TRUE if the principals are the same; FALSE otherwise
 */
krb5_boolean KRB5_CALLCONV
krb5_principal_compare(krb5_context context,
                       krb5_const_principal princ1,
                       krb5_const_principal princ2);

/**
 * Compare two principals ignoring realm components.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * Similar to krb5_principal_compare(), but do not compare the realm
 * components of the principals.
 *
 * @retval
 * TRUE if the principals are the same; FALSE otherwise
 */
krb5_boolean KRB5_CALLCONV
krb5_principal_compare_any_realm(krb5_context context,
                                 krb5_const_principal princ1,
                                 krb5_const_principal princ2);

#define KRB5_PRINCIPAL_COMPARE_IGNORE_REALM  1 /**< ignore realm component */
#define KRB5_PRINCIPAL_COMPARE_ENTERPRISE    2 /**< UPNs as real principals */
#define KRB5_PRINCIPAL_COMPARE_CASEFOLD      4 /**< case-insensitive */
#define KRB5_PRINCIPAL_COMPARE_UTF8          8 /**< treat principals as UTF-8 */

/**
 * Compare two principals with additional flags.
 *
 * @param [in] context           Library context
 * @param [in] princ1            First principal
 * @param [in] princ2            Second principal
 * @param [in] flags             Flags
 *
 * Valid flags are:
 * @li #KRB5_PRINCIPAL_COMPARE_IGNORE_REALM - ignore realm component
 * @li #KRB5_PRINCIPAL_COMPARE_ENTERPRISE - UPNs as real principals
 * @li #KRB5_PRINCIPAL_COMPARE_CASEFOLD case-insensitive
 * @li #KRB5_PRINCIPAL_COMPARE_UTF8 - treat principals as UTF-8
 *
 * @sa krb5_principal_compare()
 *
 * @retval
 * TRUE if the principal names are the same; FALSE otherwise
 */
krb5_boolean KRB5_CALLCONV
krb5_principal_compare_flags(krb5_context context,
                             krb5_const_principal princ1,
                             krb5_const_principal princ2,
                             int flags);

/**
 * Initialize an empty @c krb5_keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  length          Length of keyblock (or 0)
 * @param [out] out             New keyblock structure
 *
 * Initialize a new keyblock and allocate storage for the contents of the key.
 * It is legal to pass in a length of 0, in which case contents are left
 * unallocated.  Use krb5_free_keyblock() to free @a out when it is no longer
 * needed.
 *
 * @note If @a length is set to 0, contents are left unallocated.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_init_keyblock(krb5_context context, krb5_enctype enctype,
                   size_t length, krb5_keyblock **out);

/**
 * Copy a keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  from            Keyblock to be copied
 * @param [out] to              Copy of keyblock @a from
 *
 * This function creates a new keyblock with the same contents as @a from.  Use
 * krb5_free_keyblock() to free @a to when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_copy_keyblock(krb5_context context, const krb5_keyblock *from,
                   krb5_keyblock **to);

/**
 * Copy the contents of a keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  from            Key to be copied
 * @param [out] to              Output key
 *
 * This function copies the contents of @a from to @a to.  Use
 * krb5_free_keyblock_contents() to free @a to when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_copy_keyblock_contents(krb5_context context, const krb5_keyblock *from,
                            krb5_keyblock *to);

/**
 * Copy a krb5_creds structure.
 *
 * @param [in]  context         Library context
 * @param [in]  incred          Credentials structure to be copied
 * @param [out] outcred         Copy of @a incred
 *
 * This function creates a new credential with the contents of @a incred.  Use
 * krb5_free_creds() to free @a outcred when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_copy_creds(krb5_context context, const krb5_creds *incred, krb5_creds **outcred);

/**
 * Copy a krb5_data object.
 *
 * @param [in]  context           Library context
 * @param [in]  indata            Data object to be copied
 * @param [out] outdata           Copy of @a indata
 *
 * This function creates a new krb5_data object with the contents of @a indata.
 * Use krb5_free_data() to free @a outdata when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_copy_data(krb5_context context, const krb5_data *indata, krb5_data **outdata);

/**
 * Copy a principal.
 *
 * @param [in]  context         Library context
 * @param [in]  inprinc         Principal to be copied
 * @param [out] outprinc        Copy of @a inprinc
 *
 * This function creates a new principal structure with the contents of @a
 * inprinc.  Use krb5_free_principal() to free @a outprinc when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_copy_principal(krb5_context context, krb5_const_principal inprinc,
                    krb5_principal *outprinc);

/**
 * Copy an array of addresses.
 *
 * @param [in]  context         Library context
 * @param [in]  inaddr          Array of addresses to be copied
 * @param [out] outaddr         Copy of array of addresses
 *
 * This function creates a new address array containing a copy of @a inaddr.
 * Use krb5_free_addresses() to free @a outaddr when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_copy_addresses(krb5_context context, krb5_address *const *inaddr,
                    krb5_address ***outaddr);

/**
 * Copy a krb5_ticket structure.
 *
 * @param [in]  context         Library context
 * @param [in]  from            Ticket to be copied
 * @param [out] pto             Copy of ticket
 *
 * This function creates a new krb5_ticket structure containing the contents of
 * @a from.  Use krb5_free_ticket() to free @a pto when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_copy_ticket(krb5_context context, const krb5_ticket *from, krb5_ticket **pto);

/**
 * Copy an authorization data list.
 *
 * @param [in]  context         Library context
 * @param [in]  in_authdat      List of @a krb5_authdata structures
 * @param [out] out             New array of @a krb5_authdata structures
 *
 * This function creates a new authorization data list containing a copy of @a
 * in_authdat, which must be null-terminated.  Use krb5_free_authdata() to free
 * @a out when it is no longer needed.
 *
 * @note The last array entry in @a in_authdat must be a NULL pointer.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_copy_authdata(krb5_context context,
                   krb5_authdata *const *in_authdat, krb5_authdata ***out);

/**
 * Find authorization data elements.
 *
 * @param [in]  context         Library context
 * @param [in]  ticket_authdata Authorization data list from ticket
 * @param [in]  ap_req_authdata Authorization data list from AP request
 * @param [in]  ad_type         Authorization data type to find
 * @param [out] results         List of matching entries
 *
 * This function searches @a ticket_authdata and @a ap_req_authdata for
 * elements of type @a ad_type.  Either input list may be NULL, in which case
 * it will not be searched; otherwise, the input lists must be terminated by
 * NULL entries.  This function will search inside AD-IF-RELEVANT containers if
 * found in either list.  Use krb5_free_authdata() to free @a results when it
 * is no longer needed.
 *
 * @version New in 1.10
 */
krb5_error_code KRB5_CALLCONV
krb5_find_authdata(krb5_context context, krb5_authdata *const *ticket_authdata,
                   krb5_authdata *const *ap_req_authdata,
                   krb5_authdatatype ad_type, krb5_authdata ***results);

/**
 * Merge two authorization data lists into a new list.
 *
 * @param [in]  context         Library context
 * @param [in]  inauthdat1      First list of @a krb5_authdata structures
 * @param [in]  inauthdat2      Second list of @a krb5_authdata structures
 * @param [out] outauthdat      Merged list of @a krb5_authdata structures
 *
 * Merge two authdata arrays, such as the array from a ticket
 * and authenticator.
 * Use krb5_free_authdata() to free @a outauthdat when it is no longer needed.
 *
 * @note The last array entry in @a inauthdat1 and @a inauthdat2
 * must be a NULL pointer.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_merge_authdata(krb5_context context,
                    krb5_authdata *const *inauthdat1,
                    krb5_authdata * const *inauthdat2,
                    krb5_authdata ***outauthdat);

/**
 * Copy a krb5_authenticator structure.
 *
 * @param [in]  context         Library context
 * @param [in]  authfrom        krb5_authenticator structure to be copied
 * @param [out] authto          Copy of krb5_authenticator structure
 *
 * This function creates a new krb5_authenticator structure with the content of
 * @a authfrom.  Use krb5_free_authenticator() to free @a authto when it is no
 * longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_copy_authenticator(krb5_context context, const krb5_authenticator *authfrom,
                        krb5_authenticator **authto);

/**
 * Copy a krb5_checksum structure.
 *
 * @param [in]  context         Library context
 * @param [in]  ckfrom          Checksum to be copied
 * @param [out] ckto            Copy of krb5_checksum structure
 *
 * This function creates a new krb5_checksum structure with the contents of @a
 * ckfrom.  Use krb5_free_checksum() to free @a ckto when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_copy_checksum(krb5_context context, const krb5_checksum *ckfrom,
                   krb5_checksum **ckto);

/**
 * Generate a replay cache object for server use and open it.
 *
 * @param [in]  context         Library context
 * @param [in]  piece           Unused (replay cache identifier)
 * @param [out] rcptr           Handle to an open rcache
 *
 * This function creates a handle to the default replay cache.  Use
 * krb5_rc_close() to close @a rcptr when it is no longer needed.
 *
 * @version Prior to release 1.18, this function creates a handle to a
 * different replay cache for each unique value of @a piece.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_get_server_rcache(krb5_context context, const krb5_data *piece,
                       krb5_rcache *rcptr);

/**
 * Build a principal name using length-counted strings.
 *
 * @param [in]  context  Library context
 * @param [out] princ    Principal name
 * @param [in]  rlen     Realm name length
 * @param [in]  realm    Realm name
 * @param [in]  ...      List of unsigned int/char * components, followed by 0
 *
 * This function creates a principal from a length-counted string and a
 * variable-length list of length-counted components.  The list of components
 * ends with the first 0 length argument (so it is not possible to specify an
 * empty component with this function).  Call krb5_free_principal() to free
 * allocated memory for principal when it is no longer needed.
 *
 * @code
 * Example of how to build principal WELLKNOWN/ANONYMOUS@R
 *     krb5_build_principal_ext(context, &principal, strlen("R"), "R",
 *         (unsigned int)strlen(KRB5_WELLKNOWN_NAMESTR),
 *         KRB5_WELLKNOWN_NAMESTR,
 *         (unsigned int)strlen(KRB5_ANONYMOUS_PRINCSTR),
 *         KRB5_ANONYMOUS_PRINCSTR, 0);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV_C
krb5_build_principal_ext(krb5_context context,  krb5_principal * princ,
                         unsigned int rlen, const char * realm, ...);

/**
 * Build a principal name using null-terminated strings.
 *
 * @param [in]  context         Library context
 * @param [out] princ           Principal name
 * @param [in]  rlen            Realm name length
 * @param [in]  realm           Realm name
 * @param [in]  ...             List of char * components, ending with NULL
 *
 * Call krb5_free_principal() to free @a princ when it is no longer needed.
 *
 * @note krb5_build_principal() and krb5_build_principal_alloc_va() perform the
 * same task.  krb5_build_principal() takes variadic arguments.
 * krb5_build_principal_alloc_va() takes a pre-computed @a varargs pointer.
 *
 * @code
 * Example of how to build principal H/S@R
 *     krb5_build_principal(context, &principal,
 *                          strlen("R"), "R", "H", "S", (char*)NULL);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV_C
krb5_build_principal(krb5_context context,
                     krb5_principal * princ,
                     unsigned int rlen,
                     const char * realm, ...)
#if __GNUC__ >= 4
    __attribute__ ((sentinel))
#endif
    ;
#if KRB5_DEPRECATED
/** @deprecated Replaced by krb5_build_principal_alloc_va(). */
KRB5_ATTR_DEPRECATED krb5_error_code KRB5_CALLCONV
krb5_build_principal_va(krb5_context context,
                        krb5_principal princ,
                        unsigned int rlen,
                        const char *realm,
                        va_list ap);
#endif

/**
 * Build a principal name, using a precomputed variable argument list
 *
 * @param [in]  context         Library context
 * @param [out] princ           Principal structure
 * @param [in]  rlen            Realm name length
 * @param [in]  realm           Realm name
 * @param [in]  ap              List of char * components, ending with NULL
 *
 * Similar to krb5_build_principal(), this function builds a principal name,
 * but its name components are specified as a va_list.
 *
 * Use krb5_free_principal() to deallocate @a princ when it is no longer
 * needed.
 *
 * @code
 * Function usage example:
 *   va_list ap;
 *   va_start(ap, realm);
 *   krb5_build_principal_alloc_va(context, princ, rlen, realm, ap);
 *   va_end(ap);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_build_principal_alloc_va(krb5_context context,
                              krb5_principal *princ,
                              unsigned int rlen,
                              const char *realm,
                              va_list ap);

/**
 * Convert a Kerberos V4 principal to a Kerberos V5 principal.
 *
 * @param [in]  context         Library context
 * @param [in]  name            V4 name
 * @param [in]  instance        V4 instance
 * @param [in]  realm           Realm
 * @param [out] princ           V5 principal
 *
 * This function builds a @a princ from V4 specification based on given input
 * @a name.instance\@realm.
 *
 * Use krb5_free_principal() to free @a princ when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_425_conv_principal(krb5_context context, const char *name,
                        const char *instance, const char *realm,
                        krb5_principal *princ);

/**
 * Convert a Kerberos V5 principal to a Kerberos V4 principal.
 *
 * @param [in]  context         Library context
 * @param [in]  princ           V5 Principal
 * @param [out] name            V4 principal's name to be filled in
 * @param [out] inst            V4 principal's instance name to be filled in
 * @param [out] realm           Principal's realm name to be filled in
 *
 * This function separates a V5 principal @a princ into @a name, @a instance,
 * and @a realm.
 *
 * @retval
 *  0  Success
 * @retval
 *  KRB5_INVALID_PRINCIPAL   Invalid principal name
 * @retval
 *  KRB5_CONFIG_CANTOPEN     Can't open or find Kerberos configuration file
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_524_conv_principal(krb5_context context, krb5_const_principal princ,
                        char *name, char *inst, char *realm);
/**
 *@deprecated
 */
struct credentials;

/**
 * Convert a Kerberos V5 credentials to a Kerberos V4 credentials
 *
 * @note Not implemented
 *
 * @retval KRB524_KRB4_DISABLED (always)
 */
int KRB5_CALLCONV
krb5_524_convert_creds(krb5_context context, krb5_creds *v5creds,
                       struct credentials *v4creds);

#if KRB5_DEPRECATED
#define krb524_convert_creds_kdc krb5_524_convert_creds
#define krb524_init_ets(x) (0)
#endif

/* libkt.spec */

/**
 * Get a handle for a key table.
 *
 * @param [in]  context         Library context
 * @param [in]  name            Name of the key table
 * @param [out] ktid            Key table handle
 *
 * Resolve the key table name @a name and set @a ktid to a handle identifying
 * the key table.  Use krb5_kt_close() to free @a ktid when it is no longer
 * needed.
 *
 * @a name must be of the form @c type:residual, where @a type must be a type
 * known to the library and @a residual portion should be specific to the
 * particular keytab type.  If no @a type is given, the default is @c FILE.
 *
 * If @a name is of type @c FILE, the keytab file is not opened by this call.
 *
 * @code
 *  Example: krb5_kt_resolve(context, "FILE:/tmp/filename", &ktid);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_kt_resolve(krb5_context context, const char *name, krb5_keytab *ktid);

/**
 * Duplicate keytab handle.
 *
 * @param [in]  context         Library context
 * @param [in]  in              Key table handle to be duplicated
 * @param [out] out             Key table handle
 *
 * Create a new handle referring to the same key table as @a in.  The new
 * handle and @a in can be closed independently.
 *
 * @version New in 1.12
 */
krb5_error_code KRB5_CALLCONV
krb5_kt_dup(krb5_context context, krb5_keytab in, krb5_keytab *out);

/**
 * Get the default key table name.
 *
 * @param [in]     context      Library context
 * @param [out]    name         Default key table name
 * @param [in]     name_size    Space available in @a name
 *
 * Fill @a name with the name of the default key table for @a context.
 *
 * @sa MAX_KEYTAB_NAME_LEN
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_CONFIG_NOTENUFSPACE Buffer is too short
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_kt_default_name(krb5_context context, char *name, int name_size);

/**
 * Resolve the default key table.
 *
 * @param [in]  context         Library context
 * @param [out] id              Key table handle
 *
 * Set @a id to a handle to the default key table.  The key table is not
 * opened.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_kt_default(krb5_context context, krb5_keytab *id);

/**
 * Resolve the default client key table.
 *
 * @param [in]     context      Library context
 * @param [out]    keytab_out   Key table handle
 *
 * Fill @a keytab_out with a handle to the default client key table.
 *
 * @version New in 1.11
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_kt_client_default(krb5_context context, krb5_keytab *keytab_out);

/**
 * Free the contents of a key table entry.
 *
 * @param [in] context          Library context
 * @param [in] entry            Key table entry whose contents are to be freed
 *
 * @note The pointer is not freed.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_free_keytab_entry_contents(krb5_context context, krb5_keytab_entry *entry);

/** @deprecated Use krb5_free_keytab_entry_contents instead. */
krb5_error_code KRB5_CALLCONV
krb5_kt_free_entry(krb5_context context, krb5_keytab_entry *entry);


/* remove and add are functions, so that they can return NOWRITE
   if not a writable keytab */

/**
 * Remove an entry from a key table.
 *
 * @param [in] context          Library context
 * @param [in] id               Key table handle
 * @param [in] entry            Entry to remove from key table
 *
 * @retval
 * 0 Success
 * @retval
 *  KRB5_KT_NOWRITE     Key table is not writable
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_kt_remove_entry(krb5_context context, krb5_keytab id, krb5_keytab_entry *entry);

/**
 * Add a new entry to a key table.
 *
 * @param [in] context          Library context
 * @param [in] id               Key table handle
 * @param [in] entry            Entry to be added
 *
 * @retval
 * 0  Success
 * @retval
 *  ENOMEM    Insufficient memory
 * @retval
 *  KRB5_KT_NOWRITE  Key table is not writeable
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_kt_add_entry(krb5_context context, krb5_keytab id, krb5_keytab_entry *entry);

/**
 * Convert a principal name into the default salt for that principal.
 *
 * @param [in]  context         Library context
 * @param [in]  pr              Principal name
 * @param [out] ret             Default salt for @a pr to be filled in
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV_WRONG
krb5_principal2salt(krb5_context context,
                    krb5_const_principal pr, krb5_data *ret);
/* librc.spec--see rcache.h */

/* libcc.spec */

/**
 * Resolve a credential cache name.
 *
 * @param [in]  context         Library context
 * @param [in]  name            Credential cache name to be resolved
 * @param [out] cache           Credential cache handle
 *
 * Fills in @a cache with a @a cache handle that corresponds to the name in @a
 * name.  @a name should be of the form @c type:residual, and @a type must be a
 * type known to the library.  If the @a name does not contain a colon,
 * interpret it as a file name.
 *
 * @code
 * Example: krb5_cc_resolve(context, "MEMORY:C_", &cache);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_resolve(krb5_context context, const char *name, krb5_ccache *cache);

/**
 * Duplicate ccache handle.
 *
 * @param [in]  context         Library context
 * @param [in]  in              Credential cache handle to be duplicated
 * @param [out] out             Credential cache handle
 *
 * Create a new handle referring to the same cache as @a in.
 * The new handle and @a in can be closed independently.
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_dup(krb5_context context, krb5_ccache in, krb5_ccache *out);

/**
 * Return the name of the default credential cache.
 *
 * @param [in] context          Library context
 *
 * Return a pointer to the default credential cache name for @a context, as
 * determined by a prior call to krb5_cc_set_default_name(), by the KRB5CCNAME
 * environment variable, by the default_ccache_name profile variable, or by the
 * operating system or build-time default value.  The returned value must not
 * be modified or freed by the caller.  The returned value becomes invalid when
 * @a context is destroyed krb5_free_context() or if a subsequent call to
 * krb5_cc_set_default_name() is made on @a context.
 *
 * The default credential cache name is cached in @a context between calls to
 * this function, so if the value of KRB5CCNAME changes in the process
 * environment after the first call to this function on, that change will not
 * be reflected in later calls with the same context.  The caller can invoke
 * krb5_cc_set_default_name() with a NULL value of @a name to clear the cached
 * value and force the default name to be recomputed.
 *
 * @return
 * Name of default credential cache for the current user.
 */
const char *KRB5_CALLCONV
krb5_cc_default_name(krb5_context context);

/**
 * Set the default credential cache name.
 *
 * @param [in] context          Library context
 * @param [in] name             Default credential cache name or NULL
 *
 * Set the default credential cache name to @a name for future operations using
 * @a context.  If @a name is NULL, clear any previous application-set default
 * name and forget any cached value of the default name for @a context.
 *
 * Calls to this function invalidate the result of any previous calls to
 * krb5_cc_default_name() using @a context.
 *
 * @retval
 *  0  Success
 * @retval
 *  KV5M_CONTEXT          Bad magic number for @c _krb5_context structure
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_set_default_name(krb5_context context, const char *name);

/**
 * Resolve the default credential cache name.
 *
 * @param [in]  context         Library context
 * @param [out] ccache          Pointer to credential cache name
 *
 * Create a handle to the default credential cache as given by
 * krb5_cc_default_name().
 *
 * @retval
 * 0  Success
 * @retval
 * KV5M_CONTEXT            Bad magic number for @c _krb5_context structure
 * @retval
 * KRB5_FCC_INTERNAL       The name of the default credential cache cannot be
 *                         obtained
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_default(krb5_context context, krb5_ccache *ccache);

/**
 * Copy a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  incc            Credential cache to be copied
 * @param [out] outcc           Copy of credential cache to be filled in
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_copy_creds(krb5_context context, krb5_ccache incc, krb5_ccache outcc);

/**
 * Get a configuration value from a credential cache.
 *
 * @param [in]     context      Library context
 * @param [in]     id           Credential cache handle
 * @param [in]     principal    Configuration for this principal;
 *                              if NULL, global for the whole cache
 * @param [in]     key          Name of config variable
 * @param [out]    data         Data to be fetched
 *
 * Use krb5_free_data_contents() to free @a data when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_get_config(krb5_context context, krb5_ccache id,
                   krb5_const_principal principal,
                   const char *key, krb5_data *data);

/**
 * Store a configuration value in a credential cache.
 *
 * @param [in]     context      Library context
 * @param [in]     id           Credential cache handle
 * @param [in]     principal    Configuration for a specific principal;
 *                              if NULL, global for the whole cache
 * @param [in]     key          Name of config variable
 * @param [in]     data         Data to store, or NULL to remove
 *
 * @note Existing configuration under the same key is over-written.
 *
 * @warning Before version 1.10 @a data was assumed to be always non-null.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_set_config(krb5_context context, krb5_ccache id,
                   krb5_const_principal principal,
                   const char *key, krb5_data *data);

/**
 * Test whether a principal is a configuration principal.
 *
 * @param [in] context          Library context
 * @param [in] principal        Principal to check
 *
 * @return
 * @c TRUE if the principal is a configuration principal (generated part of
 * krb5_cc_set_config()); @c FALSE otherwise.
 */
krb5_boolean KRB5_CALLCONV
krb5_is_config_principal(krb5_context context, krb5_const_principal principal);

/**
 * Make a credential cache the primary cache for its collection.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * If the type of @a cache supports it, set @a cache to be the primary
 * credential cache for the collection it belongs to.
 *
 * @retval
 * 0  Success, or the type of @a cache doesn't support switching
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_switch(krb5_context context, krb5_ccache cache);

/**
 * Determine whether a credential cache type supports switching.
 *
 * @param [in] context          Library context
 * @param [in] type             Credential cache type
 *
 * @version New in 1.10
 *
 * @retval TRUE if @a type supports switching
 * @retval FALSE if it does not or is not a valid credential cache type.
 */
krb5_boolean KRB5_CALLCONV
krb5_cc_support_switch(krb5_context context, const char *type);

/**
 * Find a credential cache with a specified client principal.
 *
 * @param [in]  context         Library context
 * @param [in]  client          Client principal
 * @param [out] cache_out       Credential cache handle
 *
 * Find a cache within the collection whose default principal is @a client.
 * Use @a krb5_cc_close to close @a ccache when it is no longer needed.
 *
 * @retval 0 Success
 * @retval KRB5_CC_NOTFOUND
 *
 * @sa krb5_cccol_cursor_new
 *
 * @version New in 1.10
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_cache_match(krb5_context context, krb5_principal client,
                    krb5_ccache *cache_out);

/**
 * Select a credential cache to use with a server principal.
 *
 * @param [in]  context         Library context
 * @param [in]  server          Server principal
 * @param [out] cache_out       Credential cache handle
 * @param [out] princ_out       Client principal
 *
 * Select a cache within the collection containing credentials most appropriate
 * for use with @a server, according to configured rules and heuristics.
 *
 * Use krb5_cc_close() to release @a cache_out when it is no longer needed.
 * Use krb5_free_principal() to release @a princ_out when it is no longer
 * needed.  Note that @a princ_out is set in some error conditions.
 *
 * @return
 * If an appropriate cache is found, 0 is returned, @a cache_out is set to the
 * selected cache, and @a princ_out is set to the default principal of that
 * cache.
 *
 * If the appropriate client principal can be authoritatively determined but
 * the cache collection contains no credentials for that principal, then
 * KRB5_CC_NOTFOUND is returned, @a cache_out is set to NULL, and @a princ_out
 * is set to the appropriate client principal.
 *
 * If no configured mechanism can determine the appropriate cache or principal,
 * KRB5_CC_NOTFOUND is returned and @a cache_out and @a princ_out are set to
 * NULL.
 *
 * Any other error code indicates a fatal error in the processing of a cache
 * selection mechanism.
 *
 * @version New in 1.10
 */
krb5_error_code KRB5_CALLCONV
krb5_cc_select(krb5_context context, krb5_principal server,
               krb5_ccache *cache_out, krb5_principal *princ_out);

/* krb5_free.c */
/**
 * Free the storage assigned to a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Principal to be freed
 */
void KRB5_CALLCONV
krb5_free_principal(krb5_context context, krb5_principal val);

/**
 * Free a krb5_authenticator structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Authenticator structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
void KRB5_CALLCONV
krb5_free_authenticator(krb5_context context, krb5_authenticator *val);

/**
 * Free the data stored in array of addresses.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of addresses to be freed
 *
 * This function frees the contents of @a val and the array itself.
 *
 * @note The last entry in the array must be a NULL pointer.
 */
void KRB5_CALLCONV
krb5_free_addresses(krb5_context context, krb5_address **val);

/**
 * Free the storage assigned to array of authentication data.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of authentication data to be freed
 *
 * This function frees the contents of @a val and the array itself.
 *
 * @note The last entry in the array must be a NULL pointer.
 */
void KRB5_CALLCONV
krb5_free_authdata(krb5_context context, krb5_authdata **val);

/**
 * Free a ticket.
 *
 * @param [in] context          Library context
 * @param [in] val              Ticket to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
void KRB5_CALLCONV
krb5_free_ticket(krb5_context context, krb5_ticket *val);

/**
 * Free an error allocated by krb5_read_error() or krb5_sendauth().
 *
 * @param [in] context          Library context
 * @param [in] val              Error data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
void KRB5_CALLCONV
krb5_free_error(krb5_context context, krb5_error *val);

/**
 * Free a krb5_creds structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Credential structure to be freed.
 *
 * This function frees the contents of @a val and the structure itself.
 */
void KRB5_CALLCONV
krb5_free_creds(krb5_context context, krb5_creds *val);

/**
 * Free the contents of a krb5_creds structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Credential structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
void KRB5_CALLCONV
krb5_free_cred_contents(krb5_context context, krb5_creds *val);

/**
 * Free a krb5_checksum structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Checksum structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
void KRB5_CALLCONV
krb5_free_checksum(krb5_context context, krb5_checksum *val);

/**
 * Free the contents of a krb5_checksum structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Checksum structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
void KRB5_CALLCONV
krb5_free_checksum_contents(krb5_context context, krb5_checksum *val);

/**
 * Free a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Keyblock to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
void KRB5_CALLCONV
krb5_free_keyblock(krb5_context context, krb5_keyblock *val);

/**
 * Free the contents of a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] key              Keyblock to be freed
 *
 * This function frees the contents of @a key, but not the structure itself.
 */
void KRB5_CALLCONV
krb5_free_keyblock_contents(krb5_context context, krb5_keyblock *key);

/**
 * Free a krb5_ap_rep_enc_part structure.
 *
 * @param [in] context          Library context
 * @param [in] val              AP-REP enc part to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
void KRB5_CALLCONV
krb5_free_ap_rep_enc_part(krb5_context context, krb5_ap_rep_enc_part *val);

/**
 * Free a krb5_data structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
void KRB5_CALLCONV
krb5_free_data(krb5_context context, krb5_data *val);

/* Free a krb5_octet_data structure (should be unused). */
void KRB5_CALLCONV
krb5_free_octet_data(krb5_context context, krb5_octet_data *val);

/**
 * Free the contents of a krb5_data structure and zero the data field.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
void KRB5_CALLCONV
krb5_free_data_contents(krb5_context context, krb5_data *val);

/**
 * Free a string representation of a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Name string to be freed
 */
void KRB5_CALLCONV
krb5_free_unparsed_name(krb5_context context, char *val);

/**
 * Free a string allocated by a krb5 function.
 *
 * @param [in] context          Library context
 * @param [in] val              String to be freed
 *
 * @version New in 1.10
 */
void KRB5_CALLCONV
krb5_free_string(krb5_context context, char *val);

/**
 * Free an array of encryption types.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of enctypes to be freed
 *
 * @version New in 1.12
 */
void KRB5_CALLCONV
krb5_free_enctypes(krb5_context context, krb5_enctype *val);

/**
 * Free an array of checksum types.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of checksum types to be freed
 */
void KRB5_CALLCONV
krb5_free_cksumtypes(krb5_context context, krb5_cksumtype *val);

/* From krb5/os, but needed by the outside world */
/**
 * Retrieve the system time of day, in sec and ms, since the epoch.
 *
 * @param [in]  context         Library context
 * @param [out] seconds         System timeofday, seconds portion
 * @param [out] microseconds    System timeofday, microseconds portion
 *
 * This function retrieves the system time of day with the context
 * specific time offset adjustment.
 *
 * @sa krb5_crypto_us_timeofday()
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_us_timeofday(krb5_context context,
                  krb5_timestamp *seconds, krb5_int32 *microseconds);

/**
 * Retrieve the current time with context specific time offset adjustment.
 *
 * @param [in]  context         Library context
 * @param [out] timeret         Timestamp to fill in
 *
 * This function retrieves the system time of day with the context specific
 * time offset adjustment.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_timeofday(krb5_context context, krb5_timestamp *timeret);

/**
 * Check if a timestamp is within the allowed clock skew of the current time.
 *
 * @param [in]     context      Library context
 * @param [in]     date         Timestamp to check
 *
 * This function checks if @a date is close enough to the current time
 * according to the configured allowable clock skew.
 *
 * @version New in 1.10
 *
 * @retval 0 Success
 * @retval KRB5KRB_AP_ERR_SKEW @a date is not within allowable clock skew
 */
krb5_error_code KRB5_CALLCONV
krb5_check_clockskew(krb5_context context, krb5_timestamp date);

/**
 * Return all interface addresses for this host.
 *
 * @param [in]  context         Library context
 * @param [out] addr            Array of krb5_address pointers, ending with
 *                              NULL
 *
 * Use krb5_free_addresses() to free @a addr when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_os_localaddr(krb5_context context, krb5_address ***addr);

/**
 * Retrieve the default realm.
 *
 * @param [in]  context         Library context
 * @param [out] lrealm          Default realm name
 *
 * Retrieves the default realm to be used if no user-specified realm is
 * available.
 *
 * Use krb5_free_default_realm() to free @a lrealm when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_get_default_realm(krb5_context context, char **lrealm);

/**
 * Override the default realm for the specified context.
 *
 * @param [in]     context      Library context
 * @param [in]     lrealm       Realm name for the default realm
 *
 * If @a lrealm is NULL, clear the default realm setting.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_set_default_realm(krb5_context context, const char *lrealm);

/**
 * Free a default realm string returned by krb5_get_default_realm().
 *
 * @param [in] context          Library context
 * @param [in] lrealm           Realm to be freed
 */
void KRB5_CALLCONV
krb5_free_default_realm(krb5_context context, char *lrealm);

/**
 * Canonicalize a hostname, possibly using name service.
 *
 * @param [in]  context         Library context
 * @param [in]  host            Input hostname
 * @param [out] canonhost_out   Canonicalized hostname
 *
 * This function canonicalizes orig_hostname, possibly using name service
 * lookups if configuration permits.  Use krb5_free_string() to free @a
 * canonhost_out when it is no longer needed.
 *
 * @version New in 1.15
 */
krb5_error_code KRB5_CALLCONV
krb5_expand_hostname(krb5_context context, const char *host,
                     char **canonhost_out);

/**
 * Generate a full principal name from a service name.
 *
 * @param [in]  context         Library context
 * @param [in]  hostname        Host name, or NULL to use local host
 * @param [in]  sname           Service name, or NULL to use @c "host"
 * @param [in]  type            Principal type
 * @param [out] ret_princ       Generated principal
 *
 * This function converts a @a hostname and @a sname into @a krb5_principal
 * structure @a ret_princ.  The returned principal will be of the form @a
 * sname\/hostname\@REALM where REALM is determined by krb5_get_host_realm().
 * In some cases this may be the referral (empty) realm.
 *
 * The @a type can be one of the following:
 *
 * @li #KRB5_NT_SRV_HST canonicalizes the host name before looking up the
 * realm and generating the principal.
 *
 * @li #KRB5_NT_UNKNOWN accepts the hostname as given, and does not
 * canonicalize it.
 *
 * Use krb5_free_principal to free @a ret_princ when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_sname_to_principal(krb5_context context, const char *hostname, const char *sname,
                        krb5_int32 type, krb5_principal *ret_princ);

/**
 * Test whether a principal matches a matching principal.
 *
 * @param [in]  context         Library context
 * @param [in]  matching        Matching principal
 * @param [in]  princ           Principal to test
 *
 * @note A matching principal is a host-based principal with an empty realm
 * and/or second data component (hostname).  Profile configuration may cause
 * the hostname to be ignored even if it is present.  A principal matches a
 * matching principal if the former has the same non-empty (and non-ignored)
 * components of the latter.
 *
 * If @a matching is NULL, return TRUE.  If @a matching is not a matching
 * principal, return the value of krb5_principal_compare(context, matching,
 * princ).
 *
 * @return
 * TRUE if @a princ matches @a matching, FALSE otherwise.
 */
krb5_boolean KRB5_CALLCONV
krb5_sname_match(krb5_context context, krb5_const_principal matching,
                 krb5_const_principal princ);

/**
 * Change a password for an existing Kerberos account.
 *
 * @param [in]  context             Library context
 * @param [in]  creds               Credentials for kadmin/changepw service
 * @param [in]  newpw               New password
 * @param [out] result_code         Numeric error code from server
 * @param [out] result_code_string  String equivalent to @a result_code
 * @param [out] result_string       Change password response from the KDC
 *
 * Change the password for the existing principal identified by @a creds.
 *
 * The possible values of the output @a result_code are:
 *
 * @li #KRB5_KPASSWD_SUCCESS   (0) - success
 * @li #KRB5_KPASSWD_MALFORMED (1) - Malformed request error
 * @li #KRB5_KPASSWD_HARDERROR (2) - Server error
 * @li #KRB5_KPASSWD_AUTHERROR (3) - Authentication error
 * @li #KRB5_KPASSWD_SOFTERROR (4) - Password change rejected
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_change_password(krb5_context context, krb5_creds *creds,
                     const char *newpw, int *result_code,
                     krb5_data *result_code_string, krb5_data *result_string);

/**
 * Set a password for a principal using specified credentials.
 *
 * @param [in]  context              Library context
 * @param [in]  creds                Credentials for kadmin/changepw service
 * @param [in]  newpw                New password
 * @param [in]  change_password_for  Change the password for this principal
 * @param [out] result_code          Numeric error code from server
 * @param [out] result_code_string   String equivalent to @a result_code
 * @param [out] result_string        Data returned from the remote system
 *
 * This function uses the credentials @a creds to set the password @a newpw for
 * the principal @a change_password_for.  It implements the set password
 * operation of RFC 3244, for interoperability with Microsoft Windows
 * implementations.
 *
 * @note If @a change_password_for is NULL, the change is performed on the
 * current principal. If @a change_password_for is non-null, the change is
 * performed on the principal name passed in @a change_password_for.
 *
 * The error code and strings are returned in @a result_code,
 * @a result_code_string and @a result_string.
 *
 * @sa krb5_set_password_using_ccache()
 *
 * @retval
 * 0  Success and result_code is set to #KRB5_KPASSWD_SUCCESS.
 * @return
 * Kerberos error codes.
 */
krb5_error_code KRB5_CALLCONV
krb5_set_password(krb5_context context, krb5_creds *creds, const char *newpw,
                  krb5_principal change_password_for, int *result_code,
                  krb5_data *result_code_string, krb5_data *result_string);

/**
 * Set a password for a principal using cached credentials.
 *
 * @param [in]  context              Library context
 * @param [in]  ccache               Credential cache
 * @param [in]  newpw                New password
 * @param [in]  change_password_for  Change the password for this principal
 * @param [out] result_code          Numeric error code from server
 * @param [out] result_code_string   String equivalent to @a result_code
 * @param [out] result_string        Data returned from the remote system
 *
 * This function uses the cached credentials from @a ccache to set the password
 * @a newpw for the principal @a change_password_for.  It implements RFC 3244
 * set password operation (interoperable with MS Windows implementations) using
 * the credential cache.
 *
 * The error code and strings are returned in @a result_code,
 * @a result_code_string and @a result_string.
 *
 * @note If @a change_password_for is set to NULL, the change is performed on
 * the default principal in @a ccache. If @a change_password_for is non null,
 * the change is performed on the specified principal.
 *
 * @sa krb5_set_password()
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_set_password_using_ccache(krb5_context context, krb5_ccache ccache,
                               const char *newpw,
                               krb5_principal change_password_for,
                               int *result_code, krb5_data *result_code_string,
                               krb5_data *result_string);

/**
 * Get a result message for changing or setting a password.
 *
 * @param [in]  context            Library context
 * @param [in]  server_string      Data returned from the remote system
 * @param [out] message_out        A message displayable to the user
 *
 * This function processes the @a server_string returned in the @a
 * result_string parameter of krb5_change_password(), krb5_set_password(), and
 * related functions, and returns a displayable string.  If @a server_string
 * contains Active Directory structured policy information, it will be
 * converted into human-readable text.
 *
 * Use krb5_free_string() to free @a message_out when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 *
 * @version New in 1.11
 */
krb5_error_code KRB5_CALLCONV
krb5_chpw_message(krb5_context context, const krb5_data *server_string,
                  char **message_out);

/**
 * Retrieve configuration profile from the context.
 *
 * @param [in]  context         Library context
 * @param [out] profile         Pointer to data read from a configuration file
 *
 * This function creates a new @a profile object that reflects profile
 * in the supplied @a context.
 *
 * The @a profile object may be freed with profile_release() function.
 * See profile.h and profile API for more details.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_get_profile(krb5_context context, struct _profile_t ** profile);

#if KRB5_DEPRECATED
/** @deprecated Replaced by krb5_get_init_creds_password().*/
KRB5_ATTR_DEPRECATED krb5_error_code KRB5_CALLCONV
krb5_get_in_tkt_with_password(krb5_context context, krb5_flags options,
                              krb5_address *const *addrs, krb5_enctype *ktypes,
                              krb5_preauthtype *pre_auth_types,
                              const char *password, krb5_ccache ccache,
                              krb5_creds *creds, krb5_kdc_rep **ret_as_reply);

/** @deprecated Replaced by krb5_get_init_creds(). */
KRB5_ATTR_DEPRECATED krb5_error_code KRB5_CALLCONV
krb5_get_in_tkt_with_skey(krb5_context context, krb5_flags options,
                          krb5_address *const *addrs, krb5_enctype *ktypes,
                          krb5_preauthtype *pre_auth_types,
                          const krb5_keyblock *key, krb5_ccache ccache,
                          krb5_creds *creds, krb5_kdc_rep **ret_as_reply);

/** @deprecated Replaced by krb5_get_init_creds_keytab(). */
KRB5_ATTR_DEPRECATED krb5_error_code KRB5_CALLCONV
krb5_get_in_tkt_with_keytab(krb5_context context, krb5_flags options,
                            krb5_address *const *addrs, krb5_enctype *ktypes,
                            krb5_preauthtype *pre_auth_types,
                            krb5_keytab arg_keytab, krb5_ccache ccache,
                            krb5_creds *creds, krb5_kdc_rep **ret_as_reply);

#endif /* KRB5_DEPRECATED */

/**
 * Parse and decrypt a @c KRB_AP_REQ message.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     inbuf          AP-REQ message to be parsed
 * @param [in]     server         Matching principal for server, or NULL to
 *                                allow any principal in keytab
 * @param [in]     keytab         Key table, or NULL to use the default
 * @param [out]    ap_req_options If non-null, the AP-REQ flags on output
 * @param [out]    ticket         If non-null, ticket from the AP-REQ message
 *
 * This function parses, decrypts and verifies a AP-REQ message from @a inbuf
 * and stores the authenticator in @a auth_context.
 *
 * If a keyblock was specified in @a auth_context using
 * krb5_auth_con_setuseruserkey(), that key is used to decrypt the ticket in
 * AP-REQ message and @a keytab is ignored.  In this case, @a server should be
 * specified as a complete principal name to allow for proper transited-path
 * checking and replay cache selection.
 *
 * Otherwise, the decryption key is obtained from @a keytab, or from the
 * default keytab if it is NULL.  In this case, @a server may be a complete
 * principal name, a matching principal (see krb5_sname_match()), or NULL to
 * match any principal name.  The keys tried against the encrypted part of the
 * ticket are determined as follows:
 *
 * - If @a server is a complete principal name, then its entry in @a keytab is
 *   tried.
 * - Otherwise, if @a keytab is iterable, then all entries in @a keytab which
 *   match @a server are tried.
 * - Otherwise, the server principal in the ticket must match @a server, and
 *   its entry in @a keytab is tried.
 *
 * The client specified in the decrypted authenticator must match the client
 * specified in the decrypted ticket.
 *
 * If the @a remote_addr field of @a auth_context is set, the request must come
 * from that address.
 *
 * If a replay cache handle is provided in the @a auth_context, the
 * authenticator and ticket are verified against it.  If no conflict is found,
 * the new authenticator is then stored in the replay cache of @a auth_context.
 *
 * Various other checks are performed on the decoded data, including
 * cross-realm policy, clockskew, and ticket validation times.
 *
 * On success the authenticator, subkey, and remote sequence number of the
 * request are stored in @a auth_context. If the #AP_OPTS_MUTUAL_REQUIRED
 * bit is set, the local sequence number is XORed with the remote sequence
 * number in the request.
 *
 * Use krb5_free_ticket() to free @a ticket when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_rd_req(krb5_context context, krb5_auth_context *auth_context,
            const krb5_data *inbuf, krb5_const_principal server,
            krb5_keytab keytab, krb5_flags *ap_req_options,
            krb5_ticket **ticket);

/**
 * Retrieve a service key from a key table.
 *
 * @param [in]  context     Library context
 * @param [in]  keyprocarg  Name of a key table (NULL to use default name)
 * @param [in]  principal   Service principal
 * @param [in]  vno         Key version number (0 for highest available)
 * @param [in]  enctype     Encryption type (0 for any type)
 * @param [out] key         Service key from key table
 *
 * Open and search the specified key table for the entry identified by @a
 * principal, @a enctype, and @a vno. If no key is found, return an error code.
 *
 * The default key table is used, unless @a keyprocarg is non-null.
 * @a keyprocarg designates a specific key table.
 *
 * Use krb5_free_keyblock() to free @a key when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return Kerberos error code if not found or @a keyprocarg is invalid.
 */
krb5_error_code KRB5_CALLCONV
krb5_kt_read_service_key(krb5_context context, krb5_pointer keyprocarg,
                         krb5_principal principal, krb5_kvno vno,
                         krb5_enctype enctype, krb5_keyblock **key);

/**
 * Format a @c KRB-SAFE message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  userdata        User data in the message
 * @param [out] der_out         Formatted @c KRB-SAFE buffer
 * @param [out] rdata_out       Replay data. Specify NULL if not needed
 *
 * This function creates an integrity protected @c KRB-SAFE message
 * using data supplied by the application.
 *
 * Fields in @a auth_context specify the checksum type, the keyblock that
 * can be used to seed the checksum, full addresses (host and port) for
 * the sender and receiver, and @ref KRB5_AUTH_CONTEXT flags.
 *
 * The local address in @a auth_context must be set, and is used to form the
 * sender address used in the KRB-SAFE message.  The remote address is
 * optional; if specified, it will be used to form the receiver address used in
 * the message.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, a
 * timestamp is included in the KRB-SAFE message, and an entry for the message
 * is entered in an in-memory replay cache to detect if the message is
 * reflected by an attacker.  If #KRB5_AUTH_CONTEXT_DO_TIME is not set, no
 * replay cache is used.  If #KRB5_AUTH_CONTEXT_RET_TIME is set in @a
 * auth_context, a timestamp is included in the KRB-SAFE message and is stored
 * in @a rdata_out.
 *
 * If either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the @a auth_context local sequence number is included in the
 * KRB-SAFE message and then incremented.  If #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the sequence number used is stored in @a rdata_out.
 *
 * Use krb5_free_data_contents() to free @a der_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_mk_safe(krb5_context context, krb5_auth_context auth_context,
             const krb5_data *userdata, krb5_data *der_out,
             krb5_replay_data *rdata_out);

/**
 * Format a @c KRB-PRIV message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  userdata        User data for @c KRB-PRIV message
 * @param [out] der_out         Formatted @c KRB-PRIV message
 * @param [out] rdata_out       Replay data (NULL if not needed)
 *
 * This function is similar to krb5_mk_safe(), but the message is encrypted and
 * integrity-protected, not just integrity-protected.
 *
 * The local address in @a auth_context must be set, and is used to form the
 * sender address used in the KRB-PRIV message.  The remote address is
 * optional; if specified, it will be used to form the receiver address used in
 * the message.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, a
 * timestamp is included in the KRB-PRIV message, and an entry for the message
 * is entered in an in-memory replay cache to detect if the message is
 * reflected by an attacker.  If #KRB5_AUTH_CONTEXT_DO_TIME is not set, no
 * replay cache is used.  If #KRB5_AUTH_CONTEXT_RET_TIME is set in @a
 * auth_context, a timestamp is included in the KRB-PRIV message and is stored
 * in @a rdata_out.
 *
 * If either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the @a auth_context local sequence number is included in the
 * KRB-PRIV message and then incremented.  If #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the sequence number used is stored in @a rdata_out.
 *
 * Use krb5_free_data_contents() to free @a der_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_mk_priv(krb5_context context, krb5_auth_context auth_context,
             const krb5_data *userdata, krb5_data *der_out,
             krb5_replay_data *rdata_out);

/**
 * Client function for @c sendauth protocol.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     fd             File descriptor that describes network socket
 * @param [in]     appl_version   Application protocol version to be matched
 *                                with the receiver's application version
 * @param [in]     client         Client principal
 * @param [in]     server         Server principal
 * @param [in]     ap_req_options @ref AP_OPTS options
 * @param [in]     in_data        Data to be sent to the server
 * @param [in]     in_creds       Input credentials, or NULL to use @a ccache
 * @param [in]     ccache         Credential cache
 * @param [out]    error          If non-null, contains KRB_ERROR message
 *                                returned from server
 * @param [out]    rep_result     If non-null and @a ap_req_options is
 *                                #AP_OPTS_MUTUAL_REQUIRED, contains the result
 *                                of mutual authentication exchange
 * @param [out]    out_creds      If non-null, the retrieved credentials
 *
 * This function performs the client side of a sendauth/recvauth exchange by
 * sending and receiving messages over @a fd.
 *
 * Credentials may be specified in three ways:
 *
 * @li If @a in_creds is NULL, credentials are obtained with
 * krb5_get_credentials() using the principals @a client and @a server.  @a
 * server must be non-null; @a client may NULL to use the default principal of
 * @a ccache.
 *
 * @li If @a in_creds is non-null, but does not contain a ticket, credentials
 * for the exchange are obtained with krb5_get_credentials() using @a in_creds.
 * In this case, the values of @a client and @a server are unused.
 *
 * @li If @a in_creds is a complete credentials structure, it used directly.
 * In this case, the values of @a client, @a server, and @a ccache are unused.
 *
 * If the server is using a different application protocol than that specified
 * in @a appl_version, an error will be returned.
 *
 * Use krb5_free_creds() to free @a out_creds, krb5_free_ap_rep_enc_part() to
 * free @a rep_result, and krb5_free_error() to free @a error when they are no
 * longer needed.
 *
 * @sa krb5_recvauth()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_sendauth(krb5_context context, krb5_auth_context *auth_context,
              krb5_pointer fd, char *appl_version, krb5_principal client,
              krb5_principal server, krb5_flags ap_req_options,
              krb5_data *in_data, krb5_creds *in_creds, krb5_ccache ccache,
              krb5_error **error, krb5_ap_rep_enc_part **rep_result,
              krb5_creds **out_creds);

/**
 * Server function for @a sendauth protocol.
 *
 * @param [in]     context      Library context
 * @param [in,out] auth_context Pre-existing or newly created auth context
 * @param [in]     fd           File descriptor
 * @param [in]     appl_version Application protocol version to be matched
 *                              against the client's application version
 * @param [in]     server       Server principal (NULL for any in @a keytab)
 * @param [in]     flags        Additional specifications
 * @param [in]     keytab       Key table containing service keys
 * @param [out]    ticket       Ticket (NULL if not needed)
 *
 * This function performs the server side of a sendauth/recvauth exchange by
 * sending and receiving messages over @a fd.
 *
 * Use krb5_free_ticket() to free @a ticket when it is no longer needed.
 *
 * @sa krb5_sendauth()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_recvauth(krb5_context context, krb5_auth_context *auth_context,
              krb5_pointer fd, char *appl_version, krb5_principal server,
              krb5_int32 flags, krb5_keytab keytab, krb5_ticket **ticket);

/**
 * Server function for @a sendauth protocol with version parameter.
 *
 * @param [in]     context      Library context
 * @param [in,out] auth_context Pre-existing or newly created auth context
 * @param [in]     fd           File descriptor
 * @param [in]     server       Server principal (NULL for any in @a keytab)
 * @param [in]     flags        Additional specifications
 * @param [in]     keytab       Decryption key
 * @param [out]    ticket       Ticket (NULL if not needed)
 * @param [out]    version      sendauth protocol version (NULL if not needed)
 *
 * This function is similar to krb5_recvauth() with the additional output
 * information place into @a version.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_recvauth_version(krb5_context context,
                      krb5_auth_context *auth_context,
                      krb5_pointer fd,
                      krb5_principal server,
                      krb5_int32 flags,
                      krb5_keytab keytab,
                      krb5_ticket **ticket,
                      krb5_data *version);

/**
 * Format a @c KRB-CRED message for an array of credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  creds           Null-terminated array of credentials
 * @param [out] der_out         Encoded credentials
 * @param [out] rdata_out       Replay cache information (NULL if not needed)
 *
 * This function takes an array of credentials @a creds and formats
 * a @c KRB-CRED message @a der_out to pass to krb5_rd_cred().
 *
 * The local and remote addresses in @a auth_context are optional; if either is
 * specified, they are used to form the sender and receiver addresses in the
 * KRB-CRED message.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, an entry
 * for the message is entered in an in-memory replay cache to detect if the
 * message is reflected by an attacker.  If #KRB5_AUTH_CONTEXT_DO_TIME is not
 * set, no replay cache is used.  If #KRB5_AUTH_CONTEXT_RET_TIME is set in @a
 * auth_context, the timestamp used for the KRB-CRED message is stored in @a
 * rdata_out.
 *
 * If either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the @a auth_context local sequence number is included in the
 * KRB-CRED message and then incremented.  If #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the sequence number used is stored in @a rdata_out.
 *
 * Use krb5_free_data_contents() to free @a der_out when it is no longer
 * needed.
 *
 * The message will be encrypted using the send subkey of @a auth_context if it
 * is present, or the session key otherwise.  If neither key is present, the
 * credentials will not be encrypted, and the message should only be sent over
 * a secure channel.  No replay cache entry is used in this case.
 *
 * @retval
 *  0 Success
 * @retval
 *  ENOMEM Insufficient memory
 * @retval
 *   KRB5_RC_REQUIRED Message replay detection requires @a rcache parameter
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_mk_ncred(krb5_context context, krb5_auth_context auth_context,
              krb5_creds **creds, krb5_data **der_out,
              krb5_replay_data *rdata_out);

/**
 * Format a @c KRB-CRED message for a single set of credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  creds           Pointer to credentials
 * @param [out] der_out         Encoded credentials
 * @param [out] rdata_out       Replay cache data (NULL if not needed)
 *
 * This is a convenience function that calls krb5_mk_ncred() with a single set
 * of credentials.
 *
 * @retval
 * 0 Success
 * @retval
 *  ENOMEM Insufficient memory
 * @retval
 *  KRB5_RC_REQUIRED   Message replay detection requires @a rcache parameter
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_mk_1cred(krb5_context context, krb5_auth_context auth_context,
              krb5_creds *creds, krb5_data **der_out,
              krb5_replay_data *rdata_out);

/**
 * Read and validate a @c KRB-CRED message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  creddata        @c KRB-CRED message
 * @param [out] creds_out       Null-terminated array of forwarded credentials
 * @param [out] rdata_out       Replay data (NULL if not needed)
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.`
 *
 * @a creddata will be decrypted using the receiving subkey if it is present in
 * @a auth_context, or the session key if the receiving subkey is not present
 * or fails to decrypt the message.
 *
 * Use krb5_free_tgt_creds() to free @a creds_out when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_rd_cred(krb5_context context, krb5_auth_context auth_context,
             krb5_data *creddata, krb5_creds ***creds_out,
             krb5_replay_data *rdata_out);

/**
 * Get a forwarded TGT and format a @c KRB-CRED message.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] rhost            Remote host
 * @param [in] client           Client principal of TGT
 * @param [in] server           Principal of server to receive TGT
 * @param [in] cc               Credential cache handle (NULL to use default)
 * @param [in] forwardable      Whether TGT should be forwardable
 * @param [out] outbuf          KRB-CRED message
 *
 * Get a TGT for use at the remote host @a rhost and format it into a KRB-CRED
 * message.  If @a rhost is NULL and @a server is of type #KRB5_NT_SRV_HST,
 * the second component of @a server will be used.
 *
 * @retval
 *  0 Success
 * @retval
 *   ENOMEM Insufficient memory
 * @retval
 *   KRB5_PRINC_NOMATCH Requested principal and ticket do not match
 * @retval
 *   KRB5_NO_TKT_SUPPLIED Request did not supply a ticket
 * @retval
 *   KRB5_CC_BADNAME Credential cache name or principal name malformed
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_fwd_tgt_creds(krb5_context context, krb5_auth_context auth_context,
                   const char *rhost, krb5_principal client,
                   krb5_principal server, krb5_ccache cc, int forwardable,
                   krb5_data *outbuf);

/**
 * Create and initialize an authentication context.
 *
 * @param [in]  context         Library context
 * @param [out] auth_context    Authentication context
 *
 * This function creates an authentication context to hold configuration and
 * state relevant to krb5 functions for authenticating principals and
 * protecting messages once authentication has occurred.
 *
 * By default, flags for the context are set to enable the use of the replay
 * cache (#KRB5_AUTH_CONTEXT_DO_TIME), but not sequence numbers.  Use
 * krb5_auth_con_setflags() to change the flags.
 *
 * The allocated @a auth_context must be freed with krb5_auth_con_free() when
 * it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_init(krb5_context context, krb5_auth_context *auth_context);

/**
 * Free a krb5_auth_context structure.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context to be freed
 *
 * This function frees an auth context allocated by krb5_auth_con_init().
 *
 * @retval 0  (always)
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_free(krb5_context context, krb5_auth_context auth_context);

/**
 * Set a flags field in a krb5_auth_context structure.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] flags            Flags bit mask
 *
 * Valid values for @a flags are:
 * @li #KRB5_AUTH_CONTEXT_DO_TIME Use timestamps
 * @li #KRB5_AUTH_CONTEXT_RET_TIME Save timestamps
 * @li #KRB5_AUTH_CONTEXT_DO_SEQUENCE Use sequence numbers
 * @li #KRB5_AUTH_CONTEXT_RET_SEQUENCE Save sequence numbers
 *
 * @retval 0 (always)
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_setflags(krb5_context context, krb5_auth_context auth_context, krb5_int32 flags);

/**
 * Retrieve flags from a krb5_auth_context structure.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] flags           Flags bit mask
 *
 * Valid values for @a flags are:
 * @li #KRB5_AUTH_CONTEXT_DO_TIME Use timestamps
 * @li #KRB5_AUTH_CONTEXT_RET_TIME Save timestamps
 * @li #KRB5_AUTH_CONTEXT_DO_SEQUENCE Use sequence numbers
 * @li #KRB5_AUTH_CONTEXT_RET_SEQUENCE Save sequence numbers
 *
 * @retval 0 (always)
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_getflags(krb5_context context, krb5_auth_context auth_context,
                       krb5_int32 *flags);

/**
 * Set a checksum callback in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] func             Checksum callback
 * @param [in] data             Callback argument
 *
 * Set a callback to obtain checksum data in krb5_mk_req().  The callback will
 * be invoked after the subkey and local sequence number are stored in @a
 * auth_context.
 *
 * @retval 0 (always)
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_set_checksum_func( krb5_context context,
                                 krb5_auth_context  auth_context,
                                 krb5_mk_req_checksum_func func,
                                 void *data);

/**
 * Get the checksum callback from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] func            Checksum callback
 * @param [out] data            Callback argument
 *
 * @retval 0 (always)
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_get_checksum_func( krb5_context context,
                                 krb5_auth_context auth_context,
                                 krb5_mk_req_checksum_func *func,
                                 void **data);

/**
 * Set the local and remote addresses in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] local_addr       Local address
 * @param [in] remote_addr      Remote address
 *
 * This function releases the storage assigned to the contents of the local and
 * remote addresses of @a auth_context and then sets them to @a local_addr and
 * @a remote_addr respectively.
 *
 * @sa krb5_auth_con_genaddrs()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV_WRONG
krb5_auth_con_setaddrs(krb5_context context, krb5_auth_context auth_context,
                       krb5_address *local_addr, krb5_address *remote_addr);

/**
 * Retrieve address fields from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] local_addr      Local address (NULL if not needed)
 * @param [out] remote_addr     Remote address (NULL if not needed)
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_getaddrs(krb5_context context, krb5_auth_context auth_context,
                       krb5_address **local_addr, krb5_address **remote_addr);

/**
 * Set local and remote port fields in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] local_port       Local port
 * @param [in] remote_port      Remote port
 *
 * This function releases the storage assigned to the contents of the local and
 * remote ports of @a auth_context and then sets them to @a local_port and @a
 * remote_port respectively.
 *
 * @sa krb5_auth_con_genaddrs()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_setports(krb5_context context, krb5_auth_context auth_context,
                       krb5_address *local_port, krb5_address *remote_port);

/**
 * Set the session key in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] keyblock         User key
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_setuseruserkey(krb5_context context, krb5_auth_context auth_context,
                             krb5_keyblock *keyblock);

/**
 * Retrieve the session key from an auth context as a keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] keyblock        Session key
 *
 * This function creates a keyblock containing the session key from @a
 * auth_context.  Use krb5_free_keyblock() to free @a keyblock when it is no
 * longer needed
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_getkey(krb5_context context, krb5_auth_context auth_context,
                     krb5_keyblock **keyblock);

/**
 * Retrieve the session key from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] key             Session key
 *
 * This function sets @a key to the session key from @a auth_context.  Use
 * krb5_k_free_key() to release @a key when it is no longer needed.
 *
 * @retval 0 (always)
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_getkey_k(krb5_context context, krb5_auth_context auth_context,
                       krb5_key *key);

/**
 * Retrieve the send subkey from an auth context as a keyblock.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] keyblock        Send subkey
 *
 * This function creates a keyblock containing the send subkey from @a
 * auth_context.  Use krb5_free_keyblock() to free @a keyblock when it is no
 * longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_getsendsubkey(krb5_context ctx, krb5_auth_context ac, krb5_keyblock **keyblock);

/**
 * Retrieve the send subkey from an auth context.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] key             Send subkey
 *
 * This function sets @a key to the send subkey from @a auth_context.  Use
 * krb5_k_free_key() to release @a key when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_getsendsubkey_k(krb5_context ctx, krb5_auth_context ac,
                              krb5_key *key);

/**
 * Retrieve the receiving subkey from an auth context as a keyblock.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] keyblock        Receiving subkey
 *
 * This function creates a keyblock containing the receiving subkey from @a
 * auth_context.  Use krb5_free_keyblock() to free @a keyblock when it is no
 * longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_getrecvsubkey(krb5_context ctx, krb5_auth_context ac, krb5_keyblock **keyblock);

/**
 * Retrieve the receiving subkey from an auth context as a keyblock.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] key             Receiving subkey
 *
 * This function sets @a key to the receiving subkey from @a auth_context.  Use
 * krb5_k_free_key() to release @a key when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_getrecvsubkey_k(krb5_context ctx, krb5_auth_context ac, krb5_key *key);

/**
 * Set the send subkey in an auth context with a keyblock.
 *
 * @param [in] ctx              Library context
 * @param [in] ac               Authentication context
 * @param [in] keyblock         Send subkey
 *
 * This function sets the send subkey in @a ac to a copy of @a keyblock.
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_setsendsubkey(krb5_context ctx, krb5_auth_context ac,
                            krb5_keyblock *keyblock);

/**
 * Set the send subkey in an auth context.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] key             Send subkey
 *
 * This function sets the send subkey in @a ac to @a key, incrementing its
 * reference count.
 *
 * @version New in 1.9
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_setsendsubkey_k(krb5_context ctx, krb5_auth_context ac, krb5_key key);

/**
 * Set the receiving subkey in an auth context with a keyblock.
 *
 * @param [in] ctx              Library context
 * @param [in] ac               Authentication context
 * @param [in] keyblock         Receiving subkey
 *
 * This function sets the receiving subkey in @a ac to a copy of @a keyblock.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_setrecvsubkey(krb5_context ctx, krb5_auth_context ac,
                            krb5_keyblock *keyblock);

/**
 * Set the receiving subkey in an auth context.
 *
 * @param [in] ctx              Library context
 * @param [in] ac               Authentication context
 * @param [in] key              Receiving subkey
 *
 * This function sets the receiving subkey in @a ac to @a key, incrementing its
 * reference count.
 *
 * @version New in 1.9
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_setrecvsubkey_k(krb5_context ctx, krb5_auth_context ac,
                              krb5_key key);

#if KRB5_DEPRECATED
/** @deprecated Replaced by krb5_auth_con_getsendsubkey(). */
KRB5_ATTR_DEPRECATED krb5_error_code KRB5_CALLCONV
krb5_auth_con_getlocalsubkey(krb5_context context, krb5_auth_context auth_context,
                             krb5_keyblock **keyblock);

/** @deprecated Replaced by krb5_auth_con_getrecvsubkey(). */
KRB5_ATTR_DEPRECATED krb5_error_code KRB5_CALLCONV
krb5_auth_con_getremotesubkey(krb5_context context, krb5_auth_context auth_context,
                              krb5_keyblock **keyblock);
#endif

/**
 * Retrieve the local sequence number from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] seqnumber       Local sequence number
 *
 * Retrieve the local sequence number from @a auth_context and return it in @a
 * seqnumber.  The #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag must be set in @a
 * auth_context for this function to be useful.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_getlocalseqnumber(krb5_context context, krb5_auth_context auth_context,
                                krb5_int32 *seqnumber);

/**
 * Retrieve the remote sequence number from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] seqnumber       Remote sequence number
 *
 * Retrieve the remote sequence number from @a auth_context and return it in @a
 * seqnumber.  The #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag must be set in @a
 * auth_context for this function to be useful.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_getremoteseqnumber(krb5_context context, krb5_auth_context auth_context,
                                 krb5_int32 *seqnumber);

/**
 * Cause an auth context to use cipher state.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 *
 * Prepare @a auth_context to use cipher state when krb5_mk_priv() or
 * krb5_rd_priv() encrypt or decrypt data.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_initivector(krb5_context context, krb5_auth_context auth_context);

/**
 * Set the replay cache in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] rcache           Replay cache haddle
 *
 * This function sets the replay cache in @a auth_context to @a rcache.  @a
 * rcache will be closed when @a auth_context is freed, so the caller should
 * relinquish that responsibility.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_setrcache(krb5_context context, krb5_auth_context auth_context,
                        krb5_rcache rcache);

/**
 * Retrieve the replay cache from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] rcache          Replay cache handle
 *
 * This function fetches the replay cache from @a auth_context.  The caller
 * should not close @a rcache.
 *
 * @retval 0 (always)
 */
krb5_error_code KRB5_CALLCONV_WRONG
krb5_auth_con_getrcache(krb5_context context, krb5_auth_context auth_context,
                        krb5_rcache *rcache);

/**
 * Retrieve the authenticator from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] authenticator   Authenticator
 *
 * Use krb5_free_authenticator() to free @a authenticator when it is no longer
 * needed.
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_getauthenticator(krb5_context context, krb5_auth_context auth_context,
                               krb5_authenticator **authenticator);

/**
 * Set checksum type in an an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] cksumtype        Checksum type
 *
 * This function sets the checksum type in @a auth_context to be used by
 * krb5_mk_req() for the authenticator checksum.
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_set_req_cksumtype(krb5_context context, krb5_auth_context auth_context,
                                krb5_cksumtype cksumtype);

#define KRB5_REALM_BRANCH_CHAR '.'

/*
 * end "func-proto.h"
 */

/*
 * begin stuff from libos.h
 */

/**
 * @brief Read a password from keyboard input.
 *
 * @param [in]     context      Library context
 * @param [in]     prompt       First user prompt when reading password
 * @param [in]     prompt2      Second user prompt (NULL to prompt only once)
 * @param [out]    return_pwd   Returned password
 * @param [in,out] size_return  On input, maximum size of password; on output,
 *                              size of password read
 *
 * This function reads a password from keyboard input and stores it in @a
 * return_pwd.  @a size_return should be set by the caller to the amount of
 * storage space available in @a return_pwd; on successful return, it will be
 * set to the length of the password read.
 *
 * @a prompt is printed to the terminal, followed by ": ", and then a password
 * is read from the keyboard.
 *
 * If @a prompt2 is NULL, the password is read only once.  Otherwise, @a
 * prompt2 is printed to the terminal and a second password is read.  If the
 * two passwords entered are not identical, KRB5_LIBOS_BADPWDMATCH is returned.
 *
 * Echoing is turned off when the password is read.
 *
 * @retval
 *  0   Success
 * @return
 * Error in reading or verifying the password
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_read_password(krb5_context context,
                   const char *prompt, const char *prompt2,
                   char *return_pwd, unsigned int *size_return);

/**
 * Convert a principal name to a local name.
 *
 * @param [in]  context         Library context
 * @param [in]  aname           Principal name
 * @param [in]  lnsize_in       Space available in @a lname
 * @param [out] lname           Local name buffer to be filled in
 *
 * If @a aname does not correspond to any local account, KRB5_LNAME_NOTRANS is
 * returned.  If @a lnsize_in is too small for the local name,
 * KRB5_CONFIG_NOTENUFSPACE is returned.
 *
 * Local names, rather than principal names, can be used by programs that
 * translate to an environment-specific name (for example, a user account
 * name).
 *
 * @retval
 * 0  Success
 * @retval
 *  System errors
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_aname_to_localname(krb5_context context, krb5_const_principal aname,
                        int lnsize_in, char *lname);

/**
 * Get the Kerberos realm names for a host.
 *
 * @param [in]  context         Library context
 * @param [in]  host            Host name (or NULL)
 * @param [out] realmsp         Null-terminated list of realm names
 *
 * Fill in @a realmsp with a pointer to a null-terminated list of realm names.
 * If there are no known realms for the host, a list containing the referral
 * (empty) realm is returned.
 *
 * If @a host is NULL, the local host's realms are determined.
 *
 * Use krb5_free_host_realm() to release @a realmsp when it is no longer
 * needed.
 *
 * @retval
 *  0   Success
 * @retval
 *  ENOMEM  Insufficient memory
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_get_host_realm(krb5_context context, const char *host, char ***realmsp);

/**
 *
 * @param [in] context           Library context
 * @param [in] hdata             Host name (or NULL)
 * @param [out] realmsp          Null-terminated list of realm names
 *
 * Fill in @a realmsp with a pointer to a null-terminated list of realm names
 * obtained through heuristics or insecure resolution methods which have lower
 * priority than KDC referrals.
 *
 * If @a host is NULL, the local host's realms are determined.
 *
 * Use krb5_free_host_realm() to release @a realmsp when it is no longer
 * needed.
 */
krb5_error_code KRB5_CALLCONV
krb5_get_fallback_host_realm(krb5_context context,
                             krb5_data *hdata, char ***realmsp);

/**
 * Free the memory allocated by krb5_get_host_realm().
 *
 * @param [in] context          Library context
 * @param [in] realmlist        List of realm names to be released
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_free_host_realm(krb5_context context, char *const *realmlist);

/**
 * Determine if a principal is authorized to log in as a local user.
 *
 * @param [in] context          Library context
 * @param [in] principal        Principal name
 * @param [in] luser            Local username
 *
 * Determine whether @a principal is authorized to log in as a local user @a
 * luser.
 *
 * @retval
 * TRUE Principal is authorized to log in as user; FALSE otherwise.
 */
krb5_boolean KRB5_CALLCONV
krb5_kuserok(krb5_context context, krb5_principal principal, const char *luser);

/**
 * Generate auth context addresses from a connected socket.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] infd             Connected socket descriptor
 * @param [in] flags            Flags
 *
 * This function sets the local and/or remote addresses in @a auth_context
 * based on the local and remote endpoints of the socket @a infd.  The
 * following flags determine the operations performed:
 *
 * @li #KRB5_AUTH_CONTEXT_GENERATE_LOCAL_ADDR   Generate local address.
 * @li #KRB5_AUTH_CONTEXT_GENERATE_REMOTE_ADDR  Generate remote address.
 * @li #KRB5_AUTH_CONTEXT_GENERATE_LOCAL_FULL_ADDR  Generate local address and port.
 * @li #KRB5_AUTH_CONTEXT_GENERATE_REMOTE_FULL_ADDR Generate remote address and port.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_auth_con_genaddrs(krb5_context context, krb5_auth_context auth_context,
                       int infd, int flags);

/**
 * Set time offset field in a krb5_context structure.
 *
 * @param [in] context          Library context
 * @param [in] seconds          Real time, seconds portion
 * @param [in] microseconds     Real time, microseconds portion
 *
 * This function sets the time offset in @a context to the difference between
 * the system time and the real time as determined by @a seconds and @a
 * microseconds.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_set_real_time(krb5_context context, krb5_timestamp seconds,
                   krb5_int32 microseconds);

/**
 * Return the time offsets from the os context.
 *
 * @param [in]  context         Library context
 * @param [out] seconds         Time offset, seconds portion
 * @param [out] microseconds    Time offset, microseconds portion
 *
 * This function returns the time offsets in @a context.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_get_time_offsets(krb5_context context, krb5_timestamp *seconds, krb5_int32 *microseconds);

/* str_conv.c */
/**
 * Convert a string to an encryption type.
 *
 * @param [in]  string          String to convert to an encryption type
 * @param [out] enctypep        Encryption type
 *
 * @retval 0  Success; otherwise - EINVAL
 */
krb5_error_code KRB5_CALLCONV
krb5_string_to_enctype(char *string, krb5_enctype *enctypep);

/**
 * Convert a string to a salt type.
 *
 * @param [in]  string          String to convert to an encryption type
 * @param [out] salttypep       Salt type to be filled in
 *
 * @retval 0  Success; otherwise - EINVAL
 */
krb5_error_code KRB5_CALLCONV
krb5_string_to_salttype(char *string, krb5_int32 *salttypep);

/**
 * Convert a string to a checksum type.
 *
 * @param [in]  string          String to be converted
 * @param [out] cksumtypep      Checksum type to be filled in
 *
 * @retval 0  Success; otherwise - EINVAL
 */
krb5_error_code KRB5_CALLCONV
krb5_string_to_cksumtype(char *string, krb5_cksumtype *cksumtypep);

/**
 * Convert a string to a timestamp.
 *
 * @param [in]  string          String to be converted
 * @param [out] timestampp      Pointer to timestamp
 *
 * @retval 0  Success; otherwise - EINVAL
 */
krb5_error_code KRB5_CALLCONV
krb5_string_to_timestamp(char *string, krb5_timestamp *timestampp);

/**
 * Convert a string to a delta time value.
 *
 * @param [in]  string          String to be converted
 * @param [out] deltatp         Delta time to be filled in
 *
 * @retval 0  Success; otherwise - KRB5_DELTAT_BADFORMAT
 */
krb5_error_code KRB5_CALLCONV
krb5_string_to_deltat(char *string, krb5_deltat *deltatp);

/**
 * Convert an encryption type to a string.
 *
 * @param [in]  enctype         Encryption type
 * @param [out] buffer          Buffer to hold encryption type string
 * @param [in]  buflen          Storage available in @a buffer
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_enctype_to_string(krb5_enctype enctype, char *buffer, size_t buflen);

/**
 * Convert an encryption type to a name or alias.
 *
 * @param [in]  enctype         Encryption type
 * @param [in]  shortest        Flag
 * @param [out] buffer          Buffer to hold encryption type string
 * @param [in]  buflen          Storage available in @a buffer
 *
 * If @a shortest is FALSE, this function returns the enctype's canonical name
 * (like "aes128-cts-hmac-sha1-96").  If @a shortest is TRUE, it return the
 * enctype's shortest alias (like "aes128-cts").
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_enctype_to_name(krb5_enctype enctype, krb5_boolean shortest,
                     char *buffer, size_t buflen);

/**
 * Convert a salt type to a string.
 *
 * @param [in]  salttype        Salttype to convert
 * @param [out] buffer          Buffer to receive the converted string
 * @param [in]  buflen          Storage available in @a buffer
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_salttype_to_string(krb5_int32 salttype, char *buffer, size_t buflen);

/**
 * Convert a checksum type to a string.
 *
 * @param [in]  cksumtype       Checksum type
 * @param [out] buffer          Buffer to hold converted checksum type
 * @param [in]  buflen          Storage available in @a buffer
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_cksumtype_to_string(krb5_cksumtype cksumtype, char *buffer, size_t buflen);

/**
 * Convert a timestamp to a string.
 *
 * @param [in]  timestamp       Timestamp to convert
 * @param [out] buffer          Buffer to hold converted timestamp
 * @param [in]  buflen          Storage available in @a buffer
 *
 * The string is returned in the locale's appropriate date and time
 * representation.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_timestamp_to_string(krb5_timestamp timestamp, char *buffer, size_t buflen);

/**
 * Convert a timestamp to a string, with optional output padding
 *
 * @param [in]  timestamp       Timestamp to convert
 * @param [out] buffer          Buffer to hold the converted timestamp
 * @param [in]  buflen          Length of buffer
 * @param [in]  pad             Optional value to pad @a buffer if converted
 *                              timestamp does not fill it
 *
 * If @a pad is not NULL, @a buffer is padded out to @a buflen - 1 characters
 * with the value of *@a pad.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_timestamp_to_sfstring(krb5_timestamp timestamp, char *buffer,
                           size_t buflen, char *pad);

/**
 * Convert a relative time value to a string.
 *
 * @param [in]  deltat          Relative time value to convert
 * @param [out] buffer          Buffer to hold time string
 * @param [in]  buflen          Storage available in @a buffer
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_deltat_to_string(krb5_deltat deltat, char *buffer, size_t buflen);

/* The name of the Kerberos ticket granting service... and its size */
#define KRB5_TGS_NAME           "krbtgt"
#define KRB5_TGS_NAME_SIZE      6

/* flags for recvauth */
#define KRB5_RECVAUTH_SKIP_VERSION      0x0001
#define KRB5_RECVAUTH_BADAUTHVERS       0x0002
/* initial ticket api functions */

/** Text for prompt used in prompter callback function.  */
typedef struct _krb5_prompt {
    char *prompt;      /**< The prompt to show to the user */
    int hidden;        /**< Boolean; informative prompt or hidden (e.g. PIN) */
    krb5_data *reply;  /**< Must be allocated before call to  prompt routine */
} krb5_prompt;

/** Pointer to a prompter callback function. */
typedef krb5_error_code
(KRB5_CALLCONV *krb5_prompter_fct)(krb5_context context, void *data,
                                   const char *name, const char *banner,
                                   int num_prompts, krb5_prompt prompts[]);

/**
 * Prompt user for password.
 *
 * @param [in] context          Library context
 * @param      data             Unused (callback argument)
 * @param [in] name             Name to output during prompt
 * @param [in] banner           Banner to output during prompt
 * @param [in] num_prompts      Number of prompts in @a prompts
 * @param [in] prompts          Array of prompts and replies
 *
 * This function is intended to be used as a prompter callback for
 * krb5_get_init_creds_password() or krb5_init_creds_init().
 *
 * Writes @a name and @a banner to stdout, each followed by a newline, then
 * writes each prompt field in the @a prompts array, followed by ": ", and sets
 * the reply field of the entry to a line of input read from stdin.  If the
 * hidden flag is set for a prompt, then terminal echoing is turned off when
 * input is read.
 *
 * @retval
 *  0   Success
 * @return
 * Kerberos error codes
 *
 */
krb5_error_code KRB5_CALLCONV
krb5_prompter_posix(krb5_context context, void *data, const char *name,
                    const char *banner, int num_prompts,
                    krb5_prompt prompts[]);

/**
 * Long-term password responder question
 *
 * This question is asked when the long-term password is needed. It has no
 * challenge and the response is simply the password string.
 *
 * @version New in 1.11
 */
#define KRB5_RESPONDER_QUESTION_PASSWORD "password"

/**
 * OTP responder question
 *
 * The OTP responder question is asked when the KDC indicates that an OTP
 * value is required in order to complete the authentication.  The JSON format
 * of the challenge is:
 *
 *  @n {
 *  @n   "service": <string (optional)>,
 *  @n   "tokenInfo": [
 *  @n      {
 *  @n        "flags":     <number>,
 *  @n        "vendor":    <string (optional)>,
 *  @n        "challenge": <string (optional)>,
 *  @n        "length":    <number (optional)>,
 *  @n        "format":    <number (optional)>,
 *  @n        "tokenID":   <string (optional)>,
 *  @n        "algID":     <string (optional)>,
 *  @n      },
 *  @n      ...
 *  @n    ]
 *  @n  }
 *
 * The answer to the question MUST be JSON formatted:
 *
 * @n  {
 * @n    "tokeninfo": <number>,
 * @n    "value":     <string (optional)>,
 * @n    "pin":       <string (optional)>,
 * @n  }
 *
 * For more detail, please see RFC 6560.
 *
 * @version New in 1.11
 */
#define KRB5_RESPONDER_QUESTION_OTP "otp"

/**
 * These format constants identify the format of the token value.
 */
#define KRB5_RESPONDER_OTP_FORMAT_DECIMAL 0
#define KRB5_RESPONDER_OTP_FORMAT_HEXADECIMAL 1
#define KRB5_RESPONDER_OTP_FORMAT_ALPHANUMERIC 2

/**
 * This flag indicates that the token value MUST be collected.
 */
#define KRB5_RESPONDER_OTP_FLAGS_COLLECT_TOKEN 0x0001

/**
 * This flag indicates that the PIN value MUST be collected.
 */
#define KRB5_RESPONDER_OTP_FLAGS_COLLECT_PIN   0x0002

/**
 * This flag indicates that the token is now in re-synchronization mode with
 * the server.  The user is expected to reply with the next code displayed on
 * the token.
 */
#define KRB5_RESPONDER_OTP_FLAGS_NEXTOTP       0x0004

/**
 * This flag indicates that the PIN MUST be returned as a separate item. This
 * flag only takes effect if KRB5_RESPONDER_OTP_FLAGS_COLLECT_PIN is set. If
 * this flag is not set, the responder may either concatenate PIN + token value
 * and store it as "value" in the answer or it may return them separately. If
 * they are returned separately, they will be concatenated internally.
 */
#define KRB5_RESPONDER_OTP_FLAGS_SEPARATE_PIN  0x0008

/**
 * PKINIT responder question
 *
 * The PKINIT responder question is asked when the client needs a password
 * that's being used to protect key information, and is formatted as a JSON
 * object.  A specific identity's flags value, if not zero, is the bitwise-OR
 * of one or more of the KRB5_RESPONDER_PKINIT_FLAGS_TOKEN_* flags defined
 * below, and possibly other flags to be added later.  Any resemblance to
 * similarly-named CKF_* values in the PKCS#11 API should not be depended on.
 *
 *  @n {
 *  @n     identity <string> : flags <number>,
 *  @n     ...
 *  @n }
 *
 * The answer to the question MUST be JSON formatted:
 *
 *  @n {
 *  @n     identity <string> : password <string>,
 *  @n     ...
 *  @n }
 *
 * @version New in 1.12
 */
#define KRB5_RESPONDER_QUESTION_PKINIT "pkinit"

/**
 * This flag indicates that an incorrect PIN was supplied at least once since
 * the last time the correct PIN was supplied.
 */
#define KRB5_RESPONDER_PKINIT_FLAGS_TOKEN_USER_PIN_COUNT_LOW       (1 << 0)

/**
 * This flag indicates that supplying an incorrect PIN will cause the token to
 * lock itself.
 */
#define KRB5_RESPONDER_PKINIT_FLAGS_TOKEN_USER_PIN_FINAL_TRY       (1 << 1)

/**
 * This flag indicates that the user PIN is locked, and you can't log in to the
 * token with it.
 */
#define KRB5_RESPONDER_PKINIT_FLAGS_TOKEN_USER_PIN_LOCKED          (1 << 2)

/**
 * A container for a set of preauthentication questions and answers
 *
 * A responder context is supplied by the krb5 authentication system to a @ref
 * krb5_responder_fn callback.  It contains a list of questions and can receive
 * answers.  Questions contained in a responder context can be listed using
 * krb5_responder_list_questions(), retrieved using
 * krb5_responder_get_challenge(), or answered using
 * krb5_responder_set_answer().  The form of a question's challenge and
 * answer depend on the question name.
 *
 * @version New in 1.11
 */
typedef struct krb5_responder_context_st *krb5_responder_context;

/**
 * List the question names contained in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 *
 * Return a pointer to a null-terminated list of question names which are
 * present in @a rctx.  The pointer is an alias, valid only as long as the
 * lifetime of @a rctx, and should not be modified or freed by the caller.  A
 * question's challenge can be retrieved using krb5_responder_get_challenge()
 * and answered using krb5_responder_set_answer().
 *
 * @version New in 1.11
 */
const char * const * KRB5_CALLCONV
krb5_responder_list_questions(krb5_context ctx, krb5_responder_context rctx);

/**
 * Retrieve the challenge data for a given question in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] question         Question name
 *
 * Return a pointer to a C string containing the challenge for @a question
 * within @a rctx, or NULL if the question is not present in @a rctx.  The
 * structure of the question depends on the question name, but will always be
 * printable UTF-8 text.  The returned pointer is an alias, valid only as long
 * as the lifetime of @a rctx, and should not be modified or freed by the
 * caller.
 *
 * @version New in 1.11
 */
const char * KRB5_CALLCONV
krb5_responder_get_challenge(krb5_context ctx, krb5_responder_context rctx,
                             const char *question);

/**
 * Answer a named question in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] question         Question name
 * @param [in] answer           The string to set (MUST be printable UTF-8)
 *
 * This function supplies an answer to @a question within @a rctx.  The
 * appropriate form of the answer depends on the question name.
 *
 * @retval EINVAL @a question is not present within @a rctx
 *
 * @version New in 1.11
 */
krb5_error_code KRB5_CALLCONV
krb5_responder_set_answer(krb5_context ctx, krb5_responder_context rctx,
                          const char *question, const char *answer);

/**
 * Responder function for an initial credential exchange.
 *
 * @param [in] ctx              Library context
 * @param [in] data             Callback data
 * @param [in] rctx             Responder context
 *
 * A responder function is like a prompter function, but is used for handling
 * questions and answers as potentially complex data types.  Client
 * preauthentication modules will insert a set of named "questions" into
 * the responder context.  Each question may optionally contain a challenge.
 * This challenge is printable UTF-8, but may be an encoded value.  The
 * precise encoding and contents of the challenge are specific to the question
 * asked.  When the responder is called, it should answer all the questions it
 * understands.  Like the challenge, the answer MUST be printable UTF-8, but
 * may contain structured/encoded data formatted to the expected answer format
 * of the question.
 *
 * If a required question is unanswered, the prompter may be called.
 */
typedef krb5_error_code
(KRB5_CALLCONV *krb5_responder_fn)(krb5_context ctx, void *data,
                                   krb5_responder_context rctx);

typedef struct _krb5_responder_otp_tokeninfo {
    krb5_flags flags;
    krb5_int32 format; /* -1 when not specified. */
    krb5_int32 length; /* -1 when not specified. */
    char *vendor;
    char *challenge;
    char *token_id;
    char *alg_id;
} krb5_responder_otp_tokeninfo;

typedef struct _krb5_responder_otp_challenge {
    char *service;
    krb5_responder_otp_tokeninfo **tokeninfo;
} krb5_responder_otp_challenge;

/**
 * Decode the KRB5_RESPONDER_QUESTION_OTP to a C struct.
 *
 * A convenience function which parses the KRB5_RESPONDER_QUESTION_OTP
 * question challenge data, making it available in native C.  The main feature
 * of this function is the ability to interact with OTP tokens without parsing
 * the JSON.
 *
 * The returned value must be passed to krb5_responder_otp_challenge_free() to
 * be freed.
 *
 * @param [in]  ctx             Library context
 * @param [in]  rctx            Responder context
 * @param [out] chl             Challenge structure
 *
 * @version New in 1.11
 */
krb5_error_code KRB5_CALLCONV
krb5_responder_otp_get_challenge(krb5_context ctx,
                                 krb5_responder_context rctx,
                                 krb5_responder_otp_challenge **chl);

/**
 * Answer the KRB5_RESPONDER_QUESTION_OTP question.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] ti               The index of the tokeninfo selected
 * @param [in] value            The value to set, or NULL for none
 * @param [in] pin              The pin to set, or NULL for none
 *
 * @version New in 1.11
 */
krb5_error_code KRB5_CALLCONV
krb5_responder_otp_set_answer(krb5_context ctx, krb5_responder_context rctx,
                              size_t ti, const char *value, const char *pin);

/**
 * Free the value returned by krb5_responder_otp_get_challenge().
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] chl              The challenge to free
 *
 * @version New in 1.11
 */
void KRB5_CALLCONV
krb5_responder_otp_challenge_free(krb5_context ctx,
                                  krb5_responder_context rctx,
                                  krb5_responder_otp_challenge *chl);

typedef struct _krb5_responder_pkinit_identity {
    char *identity;
    krb5_int32 token_flags;     /* 0 when not specified or not applicable. */
} krb5_responder_pkinit_identity;

typedef struct _krb5_responder_pkinit_challenge {
    krb5_responder_pkinit_identity **identities;
} krb5_responder_pkinit_challenge;

/**
 * Decode the KRB5_RESPONDER_QUESTION_PKINIT to a C struct.
 *
 * A convenience function which parses the KRB5_RESPONDER_QUESTION_PKINIT
 * question challenge data, making it available in native C.  The main feature
 * of this function is the ability to read the challenge without parsing
 * the JSON.
 *
 * The returned value must be passed to krb5_responder_pkinit_challenge_free()
 * to be freed.
 *
 * @param [in]  ctx             Library context
 * @param [in]  rctx            Responder context
 * @param [out] chl_out         Challenge structure
 *
 * @version New in 1.12
 */
krb5_error_code KRB5_CALLCONV
krb5_responder_pkinit_get_challenge(krb5_context ctx,
                                    krb5_responder_context rctx,
                                    krb5_responder_pkinit_challenge **chl_out);

/**
 * Answer the KRB5_RESPONDER_QUESTION_PKINIT question for one identity.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] identity         The identity for which a PIN is being supplied
 * @param [in] pin              The provided PIN, or NULL for none
 *
 * @version New in 1.12
 */
krb5_error_code KRB5_CALLCONV
krb5_responder_pkinit_set_answer(krb5_context ctx, krb5_responder_context rctx,
                                 const char *identity, const char *pin);

/**
 * Free the value returned by krb5_responder_pkinit_get_challenge().
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] chl              The challenge to free
 *
 * @version New in 1.12
 */
void KRB5_CALLCONV
krb5_responder_pkinit_challenge_free(krb5_context ctx,
                                     krb5_responder_context rctx,
                                     krb5_responder_pkinit_challenge *chl);

/** Store options for @c _krb5_get_init_creds */
typedef struct _krb5_get_init_creds_opt {
    krb5_flags flags;
    krb5_deltat tkt_life;
    krb5_deltat renew_life;
    int forwardable;
    int proxiable;
    krb5_enctype *etype_list;
    int etype_list_length;
    krb5_address **address_list;
    krb5_preauthtype *preauth_list;
    int preauth_list_length;
    krb5_data *salt;
} krb5_get_init_creds_opt;

#define KRB5_GET_INIT_CREDS_OPT_TKT_LIFE        0x0001
#define KRB5_GET_INIT_CREDS_OPT_RENEW_LIFE      0x0002
#define KRB5_GET_INIT_CREDS_OPT_FORWARDABLE     0x0004
#define KRB5_GET_INIT_CREDS_OPT_PROXIABLE       0x0008
#define KRB5_GET_INIT_CREDS_OPT_ETYPE_LIST      0x0010
#define KRB5_GET_INIT_CREDS_OPT_ADDRESS_LIST    0x0020
#define KRB5_GET_INIT_CREDS_OPT_PREAUTH_LIST    0x0040
#define KRB5_GET_INIT_CREDS_OPT_SALT            0x0080
#define KRB5_GET_INIT_CREDS_OPT_CHG_PWD_PRMPT   0x0100
#define KRB5_GET_INIT_CREDS_OPT_CANONICALIZE    0x0200
#define KRB5_GET_INIT_CREDS_OPT_ANONYMOUS       0x0400


/**
 * Allocate a new initial credential options structure.
 *
 * @param [in]  context         Library context
 * @param [out] opt             New options structure
 *
 * This function is the preferred way to create an options structure for
 * getting initial credentials, and is required to make use of certain options.
 * Use krb5_get_init_creds_opt_free() to free @a opt when it is no longer
 * needed.
 *
 * @retval 0 - Success; Kerberos errors otherwise.
 */
krb5_error_code KRB5_CALLCONV
krb5_get_init_creds_opt_alloc(krb5_context context,
                              krb5_get_init_creds_opt **opt);

/**
 * Free initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure to free
 *
 * @sa krb5_get_init_creds_opt_alloc()
 */
void KRB5_CALLCONV
krb5_get_init_creds_opt_free(krb5_context context,
                             krb5_get_init_creds_opt *opt);

/** @deprecated Use krb5_get_init_creds_opt_alloc() instead. */
void KRB5_CALLCONV
krb5_get_init_creds_opt_init(krb5_get_init_creds_opt *opt);

/**
 * Set the ticket lifetime in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] tkt_life         Ticket lifetime
 */
void KRB5_CALLCONV
krb5_get_init_creds_opt_set_tkt_life(krb5_get_init_creds_opt *opt,
                                     krb5_deltat tkt_life);

/**
 * Set the ticket renewal lifetime in initial credential options.
 *
 * @param [in] opt              Pointer to @a options field
 * @param [in] renew_life       Ticket renewal lifetime
 */
void KRB5_CALLCONV
krb5_get_init_creds_opt_set_renew_life(krb5_get_init_creds_opt *opt,
                                       krb5_deltat renew_life);

/**
 * Set or unset the forwardable flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] forwardable      Whether credentials should be forwardable
 */
void KRB5_CALLCONV
krb5_get_init_creds_opt_set_forwardable(krb5_get_init_creds_opt *opt,
                                        int forwardable);

/**
 * Set or unset the proxiable flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] proxiable        Whether credentials should be proxiable
 */
void KRB5_CALLCONV
krb5_get_init_creds_opt_set_proxiable(krb5_get_init_creds_opt *opt,
                                      int proxiable);

/**
 * Set or unset the canonicalize flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] canonicalize     Whether to canonicalize client principal
 */
void KRB5_CALLCONV
krb5_get_init_creds_opt_set_canonicalize(krb5_get_init_creds_opt *opt,
                                         int canonicalize);

/**
 * Set or unset the anonymous flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] anonymous        Whether to make an anonymous request
 *
 * This function may be used to request anonymous credentials from the KDC by
 * setting @a anonymous to non-zero.  Note that anonymous credentials are only
 * a request; clients must verify that credentials are anonymous if that is a
 * requirement.
 */
void KRB5_CALLCONV
krb5_get_init_creds_opt_set_anonymous(krb5_get_init_creds_opt *opt,
                                      int anonymous);

/**
 * Set allowable encryption types in initial credential options.
 *
 * @param [in] opt               Options structure
 * @param [in] etype_list        Array of encryption types
 * @param [in] etype_list_length Length of @a etype_list
 */
void KRB5_CALLCONV
krb5_get_init_creds_opt_set_etype_list(krb5_get_init_creds_opt *opt,
                                       krb5_enctype *etype_list,
                                       int etype_list_length);

/**
 * Set address restrictions in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] addresses        Null-terminated array of addresses
 */
void KRB5_CALLCONV
krb5_get_init_creds_opt_set_address_list(krb5_get_init_creds_opt *opt,
                                         krb5_address **addresses);

/**
 * Set preauthentication types in initial credential options.
 *
 * @param [in] opt                 Options structure
 * @param [in] preauth_list        Array of preauthentication types
 * @param [in] preauth_list_length Length of @a preauth_list
 *
 * This function can be used to perform optimistic preauthentication when
 * getting initial credentials, in combination with
 * krb5_get_init_creds_opt_set_salt() and krb5_get_init_creds_opt_set_pa().
 */
void KRB5_CALLCONV
krb5_get_init_creds_opt_set_preauth_list(krb5_get_init_creds_opt *opt,
                                         krb5_preauthtype *preauth_list,
                                         int preauth_list_length);

/**
 * Set salt for optimistic preauthentication in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] salt             Salt data
 *
 * When getting initial credentials with a password, a salt string it used to
 * convert the password to a key.  Normally this salt is obtained from the
 * first KDC reply, but when performing optimistic preauthentication, the
 * client may need to supply the salt string with this function.
 */
void KRB5_CALLCONV
krb5_get_init_creds_opt_set_salt(krb5_get_init_creds_opt *opt,
                                 krb5_data *salt);

/**
 * Set or unset change-password-prompt flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] prompt           Whether to prompt to change password
 *
 * This flag is on by default.  It controls whether
 * krb5_get_init_creds_password() will react to an expired-password error by
 * prompting for a new password and attempting to change the old one.
 */
void KRB5_CALLCONV
krb5_get_init_creds_opt_set_change_password_prompt(krb5_get_init_creds_opt *opt,
                                                   int prompt);

/** Generic preauth option attribute/value pairs */
typedef struct _krb5_gic_opt_pa_data {
    char *attr;
    char *value;
} krb5_gic_opt_pa_data;

/**
 * Supply options for preauthentication in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] attr             Preauthentication option name
 * @param [in] value            Preauthentication option value
 *
 * This function allows the caller to supply options for preauthentication.
 * The values of @a attr and @a value are supplied to each preauthentication
 * module available within @a context.
 */
krb5_error_code KRB5_CALLCONV
krb5_get_init_creds_opt_set_pa(krb5_context context,
                               krb5_get_init_creds_opt *opt, const char *attr,
                               const char *value);

/**
 * Set location of FAST armor ccache in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] fast_ccache_name Credential cache name
 *
 * Sets the location of a credential cache containing an armor ticket to
 * protect an initial credential exchange using the FAST protocol extension.
 *
 * In version 1.7, setting an armor ccache requires that FAST be used for the
 * exchange.  In version 1.8 or later, setting the armor ccache causes FAST to
 * be used if the KDC supports it; krb5_get_init_creds_opt_set_fast_flags()
 * must be used to require that FAST be used.
 */
krb5_error_code KRB5_CALLCONV
krb5_get_init_creds_opt_set_fast_ccache_name(krb5_context context,
                                             krb5_get_init_creds_opt *opt,
                                             const char *fast_ccache_name);

/**
 * Set FAST armor cache in initial credential options.
 *
 * @param [in] context           Library context
 * @param [in] opt               Options
 * @param [in] ccache            Credential cache handle
 *
 * This function is similar to krb5_get_init_creds_opt_set_fast_ccache_name(),
 * but uses a credential cache handle instead of a name.
 *
 * @version New in 1.9
 */
krb5_error_code KRB5_CALLCONV
krb5_get_init_creds_opt_set_fast_ccache(krb5_context context,
                                        krb5_get_init_creds_opt *opt,
                                        krb5_ccache ccache);

/**
 * Set an input credential cache in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] ccache           Credential cache handle
 *
 * If an input credential cache is set, then the krb5_get_init_creds family of
 * APIs will read settings from it.  Setting an input ccache is desirable when
 * the application wishes to perform authentication in the same way (using the
 * same preauthentication mechanisms, and making the same non-security-
 * sensitive choices) as the previous authentication attempt, which stored
 * information in the passed-in ccache.
 *
 * @version New in 1.11
 */
krb5_error_code KRB5_CALLCONV
krb5_get_init_creds_opt_set_in_ccache(krb5_context context,
                                      krb5_get_init_creds_opt *opt,
                                      krb5_ccache ccache);

/**
 * Set an output credential cache in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] ccache           Credential cache handle
 *
 * If an output credential cache is set, then the krb5_get_init_creds family of
 * APIs will write credentials to it.  Setting an output ccache is desirable
 * both because it simplifies calling code and because it permits the
 * krb5_get_init_creds APIs to write out configuration information about the
 * realm to the ccache.
 */
krb5_error_code KRB5_CALLCONV
krb5_get_init_creds_opt_set_out_ccache(krb5_context context,
                                       krb5_get_init_creds_opt *opt,
                                       krb5_ccache ccache);

/**
 * @brief Ask the KDC to include or not include a PAC in the ticket
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] req_pac          Whether to request a PAC or not
 *
 * If this option is set, the AS request will include a PAC-REQUEST pa-data
 * item explicitly asking the KDC to either include or not include a privilege
 * attribute certificate in the ticket authorization data.  By default, no
 * request is made; typically the KDC will default to including a PAC if it
 * supports them.
 *
 * @version New in 1.15
 */
krb5_error_code KRB5_CALLCONV
krb5_get_init_creds_opt_set_pac_request(krb5_context context,
                                        krb5_get_init_creds_opt *opt,
                                        krb5_boolean req_pac);

/**
 * Set FAST flags in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] flags            FAST flags
 *
 * The following flag values are valid:
 * @li #KRB5_FAST_REQUIRED - Require FAST to be used
 *
 * @retval
 * 0 - Success; Kerberos errors otherwise.
 */
krb5_error_code KRB5_CALLCONV
krb5_get_init_creds_opt_set_fast_flags(krb5_context context,
                                       krb5_get_init_creds_opt *opt,
                                       krb5_flags flags);

/**
 * Retrieve FAST flags from initial credential options.
 *
 * @param [in]  context         Library context
 * @param [in]  opt             Options
 * @param [out] out_flags       FAST flags
 *
 * @retval
 * 0 - Success; Kerberos errors otherwise.
 */
krb5_error_code KRB5_CALLCONV
krb5_get_init_creds_opt_get_fast_flags(krb5_context context,
                                       krb5_get_init_creds_opt *opt,
                                       krb5_flags *out_flags);

/* Fast flags*/
#define KRB5_FAST_REQUIRED 0x0001 /**< Require KDC to support FAST*/

typedef void
(KRB5_CALLCONV *krb5_expire_callback_func)(krb5_context context, void *data,
                                           krb5_timestamp password_expiration,
                                           krb5_timestamp account_expiration,
                                           krb5_boolean is_last_req);

/**
 * Set an expiration callback in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] cb               Callback function
 * @param [in] data             Callback argument
 *
 * Set a callback to receive password and account expiration times.
 *
 * This option only applies to krb5_get_init_creds_password().  @a cb will be
 * invoked if and only if credentials are successfully acquired.  The callback
 * will receive the @a context from the krb5_get_init_creds_password() call and
 * the @a data argument supplied with this API.  The remaining arguments should
 * be interpreted as follows:
 *
 * If @a is_last_req is true, then the KDC reply contained last-req entries
 * which unambiguously indicated the password expiration, account expiration,
 * or both.  (If either value was not present, the corresponding argument will
 * be 0.)  Furthermore, a non-zero @a password_expiration should be taken as a
 * suggestion from the KDC that a warning be displayed.
 *
 * If @a is_last_req is false, then @a account_expiration will be 0 and @a
 * password_expiration will contain the expiration time of either the password
 * or account, or 0 if no expiration time was indicated in the KDC reply.  The
 * callback should independently decide whether to display a password
 * expiration warning.
 *
 * Note that @a cb may be invoked even if credentials are being acquired for
 * the kadmin/changepw service in order to change the password.  It is the
 * caller's responsibility to avoid displaying a password expiry warning in
 * this case.
 *
 * @warning Setting an expire callback with this API will cause
 * krb5_get_init_creds_password() not to send password expiry warnings to the
 * prompter, as it ordinarily may.
 *
 * @version New in 1.9
 */
krb5_error_code KRB5_CALLCONV
krb5_get_init_creds_opt_set_expire_callback(krb5_context context,
                                            krb5_get_init_creds_opt *opt,
                                            krb5_expire_callback_func cb,
                                            void *data);

/**
 * Set the responder function in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] responder        Responder function
 * @param [in] data             Responder data argument
 *
 * @version New in 1.11
 */
krb5_error_code KRB5_CALLCONV
krb5_get_init_creds_opt_set_responder(krb5_context context,
                                      krb5_get_init_creds_opt *opt,
                                      krb5_responder_fn responder, void *data);

/**
 * Get initial credentials using a password.
 *
 * @param [in]  context         Library context
 * @param [out] creds           New credentials
 * @param [in]  client          Client principal
 * @param [in]  password        Password (or NULL)
 * @param [in]  prompter        Prompter function
 * @param [in]  data            Prompter callback data
 * @param [in]  start_time      Time when ticket becomes valid (0 for now)
 * @param [in]  in_tkt_service  Service name of initial credentials (or NULL)
 * @param [in]  k5_gic_options  Initial credential options
 *
 * This function requests KDC for an initial credentials for @a client using @a
 * password.  If @a password is NULL, a password will be prompted for using @a
 * prompter if necessary.  If @a in_tkt_service is specified, it is parsed as a
 * principal name (with the realm ignored) and used as the service principal
 * for the request; otherwise the ticket-granting service is used.
 *
 * @sa krb5_verify_init_creds()
 *
 * @retval
 *  0    Success
 * @retval
 *  EINVAL Invalid argument
 * @retval
 *  KRB5_KDC_UNREACH Cannot contact any KDC for requested realm
 * @retval
 *  KRB5_PREAUTH_FAILED Generic Pre-athentication failure
 * @retval
 *  KRB5_LIBOS_PWDINTR Password read interrupted
 * @retval
 *  KRB5_REALM_CANT_RESOLVE Cannot resolve network address for KDC in requested realm
 * @retval
 *  KRB5KDC_ERR_KEY_EXP Password has expired
 * @retval
 *  KRB5_LIBOS_BADPWDMATCH Password mismatch
 * @retval
 *  KRB5_CHPW_PWDNULL New password cannot be zero length
 * @retval
 *  KRB5_CHPW_FAIL Password change failed
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_get_init_creds_password(krb5_context context, krb5_creds *creds,
                             krb5_principal client, const char *password,
                             krb5_prompter_fct prompter, void *data,
                             krb5_deltat start_time,
                             const char *in_tkt_service,
                             krb5_get_init_creds_opt *k5_gic_options);

/**
 * Retrieve enctype, salt and s2kparams from KDC
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal whose information is requested
 * @param [in]  opt             Initial credential options
 * @param [out] enctype_out     The enctype chosen by KDC
 * @param [out] salt_out        Salt returned from KDC
 * @param [out] s2kparams_out   String-to-key parameters returned from KDC
 *
 * Send an initial ticket request for @a principal and extract the encryption
 * type, salt type, and string-to-key parameters from the KDC response.  If the
 * KDC provides no etype-info, set @a enctype_out to @c ENCTYPE_NULL and set @a
 * salt_out and @a s2kparams_out to empty.  If the KDC etype-info provides no
 * salt, compute the default salt and place it in @a salt_out.  If the KDC
 * etype-info provides no string-to-key parameters, set @a s2kparams_out to
 * empty.
 *
 * @a opt may be used to specify options which affect the initial request, such
 * as request encryption types or a FAST armor cache (see
 * krb5_get_init_creds_opt_set_etype_list() and
 * krb5_get_init_creds_opt_set_fast_ccache_name()).
 *
 * Use krb5_free_data_contents() to free @a salt_out and @a s2kparams_out when
 * they are no longer needed.
 *
 * @version New in 1.17
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
krb5_error_code KRB5_CALLCONV
krb5_get_etype_info(krb5_context context, krb5_principal principal,
                    krb5_get_init_creds_opt *opt, krb5_enctype *enctype_out,
                    krb5_data *salt_out, krb5_data *s2kparams_out);

struct _krb5_init_creds_context;
typedef struct _krb5_init_creds_context *krb5_init_creds_context;

#define KRB5_INIT_CREDS_STEP_FLAG_CONTINUE 0x1  /**< More responses needed */

/**
 * Free an initial credentials context.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 *
 * @a context must be the same as the one passed to krb5_init_creds_init() for
 * this initial credentials context.
 */
void KRB5_CALLCONV
krb5_init_creds_free(krb5_context context, krb5_init_creds_context ctx);

/**
 * Acquire credentials using an initial credentials context.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 *
 * This function synchronously obtains credentials using a context created by
 * krb5_init_creds_init().  On successful return, the credentials can be
 * retrieved with krb5_init_creds_get_creds().
 *
 * @a context must be the same as the one passed to krb5_init_creds_init() for
 * this initial credentials context.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_init_creds_get(krb5_context context, krb5_init_creds_context ctx);

/**
 * Retrieve acquired credentials from an initial credentials context.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [out] creds           Acquired credentials
 *
 * This function copies the acquired initial credentials from @a ctx into @a
 * creds, after the successful completion of krb5_init_creds_get() or
 * krb5_init_creds_step().  Use krb5_free_cred_contents() to free @a creds when
 * it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_init_creds_get_creds(krb5_context context, krb5_init_creds_context ctx,
                          krb5_creds *creds);

/**
 * Get the last error from KDC from an initial credentials context.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [out] error           Error from KDC, or NULL if none was received
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_init_creds_get_error(krb5_context context, krb5_init_creds_context ctx,
                          krb5_error **error);

/**
 * Create a context for acquiring initial credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  client          Client principal to get initial creds for
 * @param [in]  prompter        Prompter callback
 * @param [in]  data            Prompter callback argument
 * @param [in]  start_time      Time when credentials become valid (0 for now)
 * @param [in]  options         Options structure (NULL for default)
 * @param [out] ctx             New initial credentials context
 *
 * This function creates a new context for acquiring initial credentials.  Use
 * krb5_init_creds_free() to free @a ctx when it is no longer needed.
 *
 * Any subsequent calls to krb5_init_creds_step(), krb5_init_creds_get(), or
 * krb5_init_creds_free() for this initial credentials context must use the
 * same @a context argument as the one passed to this function.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_init_creds_init(krb5_context context, krb5_principal client,
                     krb5_prompter_fct prompter, void *data,
                     krb5_deltat start_time, krb5_get_init_creds_opt *options,
                     krb5_init_creds_context *ctx);

/**
 * Specify a keytab to use for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] keytab           Key table handle
 *
 * This function supplies a keytab containing the client key for an initial
 * credentials request.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_init_creds_set_keytab(krb5_context context, krb5_init_creds_context ctx,
                           krb5_keytab keytab);

/**
 * Get the next KDC request for acquiring initial credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [in]  in              KDC response (empty on the first call)
 * @param [out] out             Next KDC request
 * @param [out] realm           Realm for next KDC request
 * @param [out] flags           Output flags
 *
 * This function constructs the next KDC request in an initial credential
 * exchange, allowing the caller to control the transport of KDC requests and
 * replies.  On the first call, @a in should be set to an empty buffer; on
 * subsequent calls, it should be set to the KDC's reply to the previous
 * request.
 *
 * If more requests are needed, @a flags will be set to
 * #KRB5_INIT_CREDS_STEP_FLAG_CONTINUE and the next request will be placed in
 * @a out.  If no more requests are needed, @a flags will not contain
 * #KRB5_INIT_CREDS_STEP_FLAG_CONTINUE and @a out will be empty.
 *
 * If this function returns @c KRB5KRB_ERR_RESPONSE_TOO_BIG, the caller should
 * transmit the next request using TCP rather than UDP.  If this function
 * returns any other error, the initial credential exchange has failed.
 *
 * @a context must be the same as the one passed to krb5_init_creds_init() for
 * this initial credentials context.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_init_creds_step(krb5_context context, krb5_init_creds_context ctx,
                     krb5_data *in, krb5_data *out, krb5_data *realm,
                     unsigned int *flags);

/**
 * Set a password for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] password         Password
 *
 * This function supplies a password to be used to construct the client key for
 * an initial credentials request.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_init_creds_set_password(krb5_context context, krb5_init_creds_context ctx,
                             const char *password);

/**
 * Specify a service principal for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] service          Service principal string
 *
 * This function supplies a service principal string to acquire initial
 * credentials for instead of the default krbtgt service.  @a service is parsed
 * as a principal name; any realm part is ignored.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_init_creds_set_service(krb5_context context, krb5_init_creds_context ctx,
                            const char *service);

/**
 * Retrieve ticket times from an initial credentials context.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [out] times           Ticket times for acquired credentials
 *
 * The initial credentials context must have completed obtaining credentials
 * via either krb5_init_creds_get() or krb5_init_creds_step().
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_init_creds_get_times(krb5_context context, krb5_init_creds_context ctx,
                          krb5_ticket_times *times);

struct _krb5_tkt_creds_context;
typedef struct _krb5_tkt_creds_context *krb5_tkt_creds_context;

/**
 * Create a context to get credentials from a KDC's Ticket Granting Service.
 *
 * @param[in]  context          Library context
 * @param[in]  ccache           Credential cache handle
 * @param[in]  creds            Input credentials
 * @param[in]  options          @ref KRB5_GC options for this request.
 * @param[out] ctx              New TGS request context
 *
 * This function prepares to obtain credentials matching @a creds, either by
 * retrieving them from @a ccache or by making requests to ticket-granting
 * services beginning with a ticket-granting ticket for the client principal's
 * realm.
 *
 * The resulting TGS acquisition context can be used asynchronously with
 * krb5_tkt_creds_step() or synchronously with krb5_tkt_creds_get().  See also
 * krb5_get_credentials() for synchronous use.
 *
 * Use krb5_tkt_creds_free() to free @a ctx when it is no longer needed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_tkt_creds_init(krb5_context context, krb5_ccache ccache,
                    krb5_creds *creds, krb5_flags options,
                    krb5_tkt_creds_context *ctx);

/**
 * Synchronously obtain credentials using a TGS request context.
 *
 * @param[in] context           Library context
 * @param[in] ctx               TGS request context
 *
 * This function synchronously obtains credentials using a context created by
 * krb5_tkt_creds_init().  On successful return, the credentials can be
 * retrieved with krb5_tkt_creds_get_creds().
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_tkt_creds_get(krb5_context context, krb5_tkt_creds_context ctx);

/**
 * Retrieve acquired credentials from a TGS request context.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[out] creds            Acquired credentials
 *
 * This function copies the acquired initial credentials from @a ctx into @a
 * creds, after the successful completion of krb5_tkt_creds_get() or
 * krb5_tkt_creds_step().  Use krb5_free_cred_contents() to free @a creds when
 * it is no longer needed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_tkt_creds_get_creds(krb5_context context, krb5_tkt_creds_context ctx,
                         krb5_creds *creds);

/**
 * Free a TGS request context.
 *
 * @param[in]  context  Library context
 * @param[in]  ctx      TGS request context
 *
 * @version New in 1.9
 */
void KRB5_CALLCONV
krb5_tkt_creds_free(krb5_context context, krb5_tkt_creds_context ctx);

#define KRB5_TKT_CREDS_STEP_FLAG_CONTINUE 0x1  /**< More responses needed */

/**
 * Get the next KDC request in a TGS exchange.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[in]  in               KDC response (empty on the first call)
 * @param[out] out              Next KDC request
 * @param[out] realm            Realm for next KDC request
 * @param[out] flags            Output flags
 *
 * This function constructs the next KDC request for a TGS exchange, allowing
 * the caller to control the transport of KDC requests and replies.  On the
 * first call, @a in should be set to an empty buffer; on subsequent calls, it
 * should be set to the KDC's reply to the previous request.
 *
 * If more requests are needed, @a flags will be set to
 * #KRB5_TKT_CREDS_STEP_FLAG_CONTINUE and the next request will be placed in @a
 * out.  If no more requests are needed, @a flags will not contain
 * #KRB5_TKT_CREDS_STEP_FLAG_CONTINUE and @a out will be empty.
 *
 * If this function returns @c KRB5KRB_ERR_RESPONSE_TOO_BIG, the caller should
 * transmit the next request using TCP rather than UDP.  If this function
 * returns any other error, the TGS exchange has failed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_tkt_creds_step(krb5_context context, krb5_tkt_creds_context ctx,
                    krb5_data *in, krb5_data *out, krb5_data *realm,
                    unsigned int *flags);

/**
 * Retrieve ticket times from a TGS request context.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[out] times            Ticket times for acquired credentials
 *
 * The TGS request context must have completed obtaining credentials via either
 * krb5_tkt_creds_get() or krb5_tkt_creds_step().
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_tkt_creds_get_times(krb5_context context, krb5_tkt_creds_context ctx,
                         krb5_ticket_times *times);

/**
 * Get initial credentials using a key table.
 *
 * @param [in]  context         Library context
 * @param [out] creds           New credentials
 * @param [in]  client          Client principal
 * @param [in]  arg_keytab      Key table handle
 * @param [in]  start_time      Time when ticket becomes valid (0 for now)
 * @param [in]  in_tkt_service  Service name of initial credentials (or NULL)
 * @param [in]  k5_gic_options  Initial credential options
 *
 * This function requests KDC for an initial credentials for @a client using a
 * client key stored in @a arg_keytab.  If @a in_tkt_service is specified, it
 * is parsed as a principal name (with the realm ignored) and used as the
 * service principal for the request; otherwise the ticket-granting service is
 * used.
 *
 * @sa krb5_verify_init_creds()
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_get_init_creds_keytab(krb5_context context, krb5_creds *creds,
                           krb5_principal client, krb5_keytab arg_keytab,
                           krb5_deltat start_time, const char *in_tkt_service,
                           krb5_get_init_creds_opt *k5_gic_options);

typedef struct _krb5_verify_init_creds_opt {
    krb5_flags flags;
    int ap_req_nofail; /**< boolean */
} krb5_verify_init_creds_opt;

#define KRB5_VERIFY_INIT_CREDS_OPT_AP_REQ_NOFAIL        0x0001

/**
 * Initialize a credential verification options structure.
 *
 * @param [in] k5_vic_options   Verification options structure
 */
void KRB5_CALLCONV
krb5_verify_init_creds_opt_init(krb5_verify_init_creds_opt *k5_vic_options);

/**
 * Set whether credential verification is required.
 *
 * @param [in] k5_vic_options   Verification options structure
 * @param [in] ap_req_nofail    Whether to require successful verification
 *
 * This function determines how krb5_verify_init_creds() behaves if no keytab
 * information is available.  If @a ap_req_nofail is @c FALSE, verification
 * will be skipped in this case and krb5_verify_init_creds() will return
 * successfully.  If @a ap_req_nofail is @c TRUE, krb5_verify_init_creds() will
 * not return successfully unless verification can be performed.
 *
 * If this function is not used, the behavior of krb5_verify_init_creds() is
 * determined through configuration.
 */
void KRB5_CALLCONV
krb5_verify_init_creds_opt_set_ap_req_nofail(krb5_verify_init_creds_opt * k5_vic_options,
                                             int ap_req_nofail);

/**
 * Verify initial credentials against a keytab.
 *
 * @param [in] context          Library context
 * @param [in] creds            Initial credentials to be verified
 * @param [in] server           Server principal (or NULL)
 * @param [in] keytab           Key table (NULL to use default keytab)
 * @param [in] ccache           Credential cache for fetched creds (or NULL)
 * @param [in] options          Verification options (NULL for default options)
 *
 * This function attempts to verify that @a creds were obtained from a KDC with
 * knowledge of a key in @a keytab, or the default keytab if @a keytab is NULL.
 * If @a server is provided, the highest-kvno key entry for that principal name
 * is used to verify the credentials; otherwise, all unique "host" service
 * principals in the keytab are tried.
 *
 * If the specified keytab does not exist, or is empty, or cannot be read, or
 * does not contain an entry for @a server, then credential verification may be
 * skipped unless configuration demands that it succeed.  The caller can
 * control this behavior by providing a verification options structure; see
 * krb5_verify_init_creds_opt_init() and
 * krb5_verify_init_creds_opt_set_ap_req_nofail().
 *
 * If @a ccache is NULL, any additional credentials fetched during the
 * verification process will be destroyed.  If @a ccache points to NULL, a
 * memory ccache will be created for the additional credentials and returned in
 * @a ccache.  If @a ccache points to a valid credential cache handle, the
 * additional credentials will be stored in that cache.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_verify_init_creds(krb5_context context, krb5_creds *creds,
                       krb5_principal server, krb5_keytab keytab,
                       krb5_ccache *ccache,
                       krb5_verify_init_creds_opt *options);

/**
 * Get validated credentials from the KDC.
 *
 * @param [in]  context         Library context
 * @param [out] creds           Validated credentials
 * @param [in]  client          Client principal name
 * @param [in]  ccache          Credential cache
 * @param [in]  in_tkt_service  Server principal string (or NULL)
 *
 * This function gets a validated credential using a postdated credential from
 * @a ccache.  If @a in_tkt_service is specified, it is parsed (with the realm
 * part ignored) and used as the server principal of the credential;
 * otherwise, the ticket-granting service is used.
 *
 * If successful, the validated credential is placed in @a creds.
 *
 * @sa krb5_get_renewed_creds()
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_NO_2ND_TKT Request missing second ticket
 * @retval
 * KRB5_NO_TKT_SUPPLIED Request did not supply a ticket
 * @retval
 * KRB5_PRINC_NOMATCH Requested principal and ticket do not match
 * @retval
 * KRB5_KDCREP_MODIFIED KDC reply did not match expectations
 * @retval
 * KRB5_KDCREP_SKEW Clock skew too great in KDC reply
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_get_validated_creds(krb5_context context, krb5_creds *creds,
                         krb5_principal client, krb5_ccache ccache,
                         const char *in_tkt_service);

/**
 * Get renewed credential from KDC using an existing credential.
 *
 * @param [in]  context         Library context
 * @param [out] creds           Renewed credentials
 * @param [in]  client          Client principal name
 * @param [in]  ccache          Credential cache
 * @param [in]  in_tkt_service  Server principal string (or NULL)
 *
 * This function gets a renewed credential using an existing one from @a
 * ccache.  If @a in_tkt_service is specified, it is parsed (with the realm
 * part ignored) and used as the server principal of the credential; otherwise,
 * the ticket-granting service is used.
 *
 * If successful, the renewed credential is placed in @a creds.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_get_renewed_creds(krb5_context context, krb5_creds *creds,
                       krb5_principal client, krb5_ccache ccache,
                       const char *in_tkt_service);

/**
 * Decode an ASN.1-formatted ticket.
 *
 * @param [in]  code            ASN.1-formatted ticket
 * @param [out] rep             Decoded ticket information
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_decode_ticket(const krb5_data *code, krb5_ticket **rep);

/**
 * Retrieve a string value from the appdefaults section of krb5.conf.
 *
 * @param [in]  context         Library context
 * @param [in]  appname         Application name
 * @param [in]  realm           Realm name
 * @param [in]  option          Option to be checked
 * @param [in]  default_value   Default value to return if no match is found
 * @param [out] ret_value       String value of @a option
 *
 * This function gets the application defaults for @a option based on the given
 * @a appname and/or @a realm.
 *
 * @sa krb5_appdefault_boolean()
 */
void KRB5_CALLCONV
krb5_appdefault_string(krb5_context context, const char *appname,
                       const krb5_data *realm, const char *option,
                       const char *default_value, char ** ret_value);

/**
 * Retrieve a boolean value from the appdefaults section of krb5.conf.
 *
 * @param [in]  context         Library context
 * @param [in]  appname         Application name
 * @param [in]  realm           Realm name
 * @param [in]  option          Option to be checked
 * @param [in]  default_value   Default value to return if no match is found
 * @param [out] ret_value       Boolean value of @a option
 *
 * This function gets the application defaults for @a option based on the given
 * @a appname and/or @a realm.
 *
 * @sa krb5_appdefault_string()
 */
void KRB5_CALLCONV
krb5_appdefault_boolean(krb5_context context, const char *appname,
                        const krb5_data *realm, const char *option,
                        int default_value, int *ret_value);

/*
 * Prompter enhancements
 */
/** Prompt for password */
#define KRB5_PROMPT_TYPE_PASSWORD            0x1
/** Prompt for new password (during password change) */
#define KRB5_PROMPT_TYPE_NEW_PASSWORD        0x2
/** Prompt for new password again */
#define KRB5_PROMPT_TYPE_NEW_PASSWORD_AGAIN  0x3
/** Prompt for preauthentication data (such as an OTP value) */
#define KRB5_PROMPT_TYPE_PREAUTH             0x4

typedef krb5_int32 krb5_prompt_type;

/**
 * Get prompt types array from a context.
 *
 * @param [in] context          Library context
 *
 * @return
 * Pointer to an array of prompt types corresponding to the prompter's @a
 * prompts arguments.  Each type has one of the following values:
 *  @li #KRB5_PROMPT_TYPE_PASSWORD
 *  @li #KRB5_PROMPT_TYPE_NEW_PASSWORD
 *  @li #KRB5_PROMPT_TYPE_NEW_PASSWORD_AGAIN
 *  @li #KRB5_PROMPT_TYPE_PREAUTH
 */
krb5_prompt_type* KRB5_CALLCONV
krb5_get_prompt_types(krb5_context context);

/* Error reporting */
/**
 * Set an extended error message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] ...              printf(3) style parameters
 */
void KRB5_CALLCONV_C
krb5_set_error_message(krb5_context ctx, krb5_error_code code, const char *fmt, ...)
#if !defined(__cplusplus) && (__GNUC__ > 2)
    __attribute__((__format__(__printf__, 3, 4)))
#endif
    ;

/**
 * Set an extended error message for an error code using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] args             List of vprintf(3) style arguments
 */
void KRB5_CALLCONV
krb5_vset_error_message(krb5_context  ctx, krb5_error_code code,
                        const char *fmt, va_list args)
#if !defined(__cplusplus) && (__GNUC__ > 2)
    __attribute__((__format__(__printf__, 3, 0)))
#endif
    ;

/**
 * Add a prefix to the message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] ...              printf(3) style parameters
 *
 * Format a message and prepend it to the current message for @a code.  The
 * prefix will be separated from the old message with a colon and space.
 */
void KRB5_CALLCONV_C
krb5_prepend_error_message(krb5_context ctx, krb5_error_code code,
                           const char *fmt, ...)
#if !defined(__cplusplus) && (__GNUC__ > 2)
    __attribute__((__format__(__printf__, 3, 4)))
#endif
    ;

/**
 * Add a prefix to the message for an error code using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] args             List of vprintf(3) style arguments
 *
 * This function is similar to krb5_prepend_error_message(), but uses a
 * va_list instead of variadic arguments.
 */
void KRB5_CALLCONV
krb5_vprepend_error_message(krb5_context  ctx, krb5_error_code code,
                        const char *fmt, va_list args)
#if !defined(__cplusplus) && (__GNUC__ > 2)
    __attribute__((__format__(__printf__, 3, 0)))
#endif
    ;

/**
 * Add a prefix to a different error code's message.
 *
 * @param [in] ctx              Library context
 * @param [in] old_code         Previous error code
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] ...              printf(3) style parameters
 *
 * Format a message and prepend it to the message for @a old_code.  The prefix
 * will be separated from the old message with a colon and space.  Set the
 * resulting message as the extended error message for @a code.
 */
void KRB5_CALLCONV_C
krb5_wrap_error_message(krb5_context ctx, krb5_error_code old_code,
                        krb5_error_code code, const char *fmt, ...)
#if !defined(__cplusplus) && (__GNUC__ > 2)
    __attribute__((__format__(__printf__, 4, 5)))
#endif
    ;

/**
 * Add a prefix to a different error code's message using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] old_code         Previous error code
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] args             List of vprintf(3) style arguments
 *
 * This function is similar to krb5_wrap_error_message(), but uses a
 * va_list instead of variadic arguments.
 */
void KRB5_CALLCONV
krb5_vwrap_error_message(krb5_context ctx, krb5_error_code old_code,
                         krb5_error_code code, const char *fmt, va_list args)
#if !defined(__cplusplus) && (__GNUC__ > 2)
    __attribute__((__format__(__printf__, 4, 0)))
#endif
    ;

/**
 * Copy the most recent extended error message from one context to another.
 *
 * @param [in] dest_ctx         Library context to copy message to
 * @param [in] src_ctx          Library context with current message
 */
void KRB5_CALLCONV
krb5_copy_error_message(krb5_context dest_ctx, krb5_context src_ctx);

/**
 * Get the (possibly extended) error message for a code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 *
 * The behavior of krb5_get_error_message() is only defined the first time it
 * is called after a failed call to a krb5 function using the same context, and
 * only when the error code passed in is the same as that returned by the krb5
 * function.
 *
 * This function never returns NULL, so its result may be used unconditionally
 * as a C string.
 *
 * The string returned by this function must be freed using
 * krb5_free_error_message()
 *
 * @note Future versions may return the same string for the second
 * and following calls.
 */
const char * KRB5_CALLCONV
krb5_get_error_message(krb5_context ctx, krb5_error_code code);

/**
 * Free an error message generated by krb5_get_error_message().
 *
 * @param [in] ctx              Library context
 * @param [in] msg              Pointer to error message
 */
void KRB5_CALLCONV
krb5_free_error_message(krb5_context ctx, const char *msg);

/**
 * Clear the extended error message in a context.
 *
 * @param [in] ctx              Library context
 *
 * This function unsets the extended error message in a context, to ensure that
 * it is not mistakenly applied to another occurrence of the same error code.
 */
void KRB5_CALLCONV
krb5_clear_error_message(krb5_context ctx);

/**
 * Unwrap authorization data.
 *
 * @param [in]  context         Library context
 * @param [in]  type            @ref KRB5_AUTHDATA type of @a container
 * @param [in]  container       Authorization data to be decoded
 * @param [out] authdata        List of decoded authorization data
 *
 * @sa krb5_encode_authdata_container()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_decode_authdata_container(krb5_context context,
                               krb5_authdatatype type,
                               const krb5_authdata *container,
                               krb5_authdata ***authdata);
/**
 * Wrap authorization data in a container.
 *
 * @param [in]  context         Library context
 * @param [in]  type            @ref KRB5_AUTHDATA type of @a container
 * @param [in]  authdata        List of authorization data to be encoded
 * @param [out] container       List of encoded authorization data
 *
 * The result is returned in @a container as a single-element list.
 *
 * @sa krb5_decode_authdata_container()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_encode_authdata_container(krb5_context context,
                               krb5_authdatatype type,
                               krb5_authdata * const*authdata,
                               krb5_authdata ***container);

/*
 * AD-KDCIssued
 */
/**
 * Encode and sign AD-KDCIssued authorization data.
 *
 * @param [in]  context         Library context
 * @param [in]  key             Session key
 * @param [in]  issuer          The name of the issuing principal
 * @param [in]  authdata        List of authorization data to be signed
 * @param [out] ad_kdcissued    List containing AD-KDCIssued authdata
 *
 * This function wraps a list of authorization data entries @a authdata in an
 * AD-KDCIssued container (see RFC 4120 section 5.2.6.2) signed with @a key.
 * The result is returned in @a ad_kdcissued as a single-element list.
 */
krb5_error_code KRB5_CALLCONV
krb5_make_authdata_kdc_issued(krb5_context context,
                              const krb5_keyblock *key,
                              krb5_const_principal issuer,
                              krb5_authdata *const *authdata,
                              krb5_authdata ***ad_kdcissued);

/**
 * Unwrap and verify AD-KDCIssued authorization data.
 *
 * @param [in] context          Library context
 * @param [in] key              Session key
 * @param [in] ad_kdcissued     AD-KDCIssued authorization data to be unwrapped
 * @param [out] issuer          Name of issuing principal (or NULL)
 * @param [out] authdata        Unwrapped list of authorization data
 *
 * This function unwraps an AD-KDCIssued authdatum (see RFC 4120 section
 * 5.2.6.2) and verifies its signature against @a key.  The issuer field of the
 * authdatum element is returned in @a issuer, and the unwrapped list of
 * authdata is returned in @a authdata.
 */
krb5_error_code KRB5_CALLCONV
krb5_verify_authdata_kdc_issued(krb5_context context,
                                const krb5_keyblock *key,
                                const krb5_authdata *ad_kdcissued,
                                krb5_principal *issuer,
                                krb5_authdata ***authdata);

/*
 * Windows PAC
 */

/* Microsoft defined types of data */
#define KRB5_PAC_LOGON_INFO        1  /**< Logon information */
#define KRB5_PAC_CREDENTIALS_INFO  2  /**< Credentials information */
#define KRB5_PAC_SERVER_CHECKSUM   6  /**< Server checksum */
#define KRB5_PAC_PRIVSVR_CHECKSUM  7  /**< KDC checksum */
#define KRB5_PAC_CLIENT_INFO       10 /**< Client name and ticket info */
#define KRB5_PAC_DELEGATION_INFO   11 /**< Constrained delegation info */
#define KRB5_PAC_UPN_DNS_INFO      12 /**< User principal name and DNS info */

struct krb5_pac_data;
/** PAC data structure to convey authorization information */
typedef struct krb5_pac_data *krb5_pac;

/**
 * Add a buffer to a PAC handle.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] type             Buffer type
 * @param [in] data             contents
 *
 * This function adds a buffer of type @a type and contents @a data to @a pac
 * if there isn't already a buffer of this type present.
 *
 * The valid values of @a type is one of the following:
 * @li #KRB5_PAC_LOGON_INFO         -  Logon information
 * @li #KRB5_PAC_CREDENTIALS_INFO   -  Credentials information
 * @li #KRB5_PAC_SERVER_CHECKSUM    -  Server checksum
 * @li #KRB5_PAC_PRIVSVR_CHECKSUM   -  KDC checksum
 * @li #KRB5_PAC_CLIENT_INFO        -  Client name and ticket information
 * @li #KRB5_PAC_DELEGATION_INFO    -  Constrained delegation information
 * @li #KRB5_PAC_UPN_DNS_INFO       -  User principal name and DNS information
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_pac_add_buffer(krb5_context context, krb5_pac pac, krb5_ui_4 type,
                    const krb5_data *data);

/**
 * Free a PAC handle.
 *
 * @param [in] context         Library context
 * @param [in] pac             PAC to be freed
 *
 * This function frees the contents of @a pac and the structure itself.
 */
void KRB5_CALLCONV
krb5_pac_free(krb5_context context, krb5_pac pac);

/**
 * Retrieve a buffer value from a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  type            Type of buffer to retrieve
 * @param [out] data            Buffer value
 *
 * Use krb5_free_data_contents() to free @a data when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_pac_get_buffer(krb5_context context, krb5_pac pac, krb5_ui_4 type,
                    krb5_data *data);

/**
 * Return an array of buffer types in a PAC handle.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [out] len             Number of entries in @a types
 * @param [out] types           Array of buffer types
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_pac_get_types(krb5_context context, krb5_pac pac, size_t *len,
                   krb5_ui_4 **types);

/**
 * Create an empty Privilege Attribute Certificate (PAC) handle.
 *
 * @param [in]  context         Library context
 * @param [out] pac             New PAC handle
 *
 * Use krb5_pac_free() to free @a pac when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_pac_init(krb5_context context, krb5_pac *pac);

/**
 * Unparse an encoded PAC into a new handle.
 *
 * @param [in]  context         Library context
 * @param [in]  ptr             PAC buffer
 * @param [in]  len             Length of @a ptr
 * @param [out] pac             PAC handle
 *
 * Use krb5_pac_free() to free @a pac when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_pac_parse(krb5_context context, const void *ptr, size_t len,
               krb5_pac *pac);

/**
 * Verify a PAC.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] authtime         Expected timestamp
 * @param [in] principal        Expected principal name (or NULL)
 * @param [in] server           Key to validate server checksum (or NULL)
 * @param [in] privsvr          Key to validate KDC checksum (or NULL)
 *
 * This function validates @a pac against the supplied @a server, @a privsvr,
 * @a principal and @a authtime.  If @a principal is NULL, the principal and
 * authtime are not verified.  If @a server or @a privsvr is NULL, the
 * corresponding checksum is not verified.
 *
 * If successful, @a pac is marked as verified.
 *
 * @note A checksum mismatch can occur if the PAC was copied from a cross-realm
 * TGT by an ignorant KDC; also macOS Server Open Directory (as of 10.6)
 * generates PACs with no server checksum at all.  One should consider not
 * failing the whole authentication because of this reason, but, instead,
 * treating the ticket as if it did not contain a PAC or marking the PAC
 * information as non-verified.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
krb5_error_code KRB5_CALLCONV
krb5_pac_verify(krb5_context context, const krb5_pac pac,
                krb5_timestamp authtime, krb5_const_principal principal,
                const krb5_keyblock *server, const krb5_keyblock *privsvr);

/**
 * Verify a PAC, possibly from a specified realm.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] authtime         Expected timestamp
 * @param [in] principal        Expected principal name (or NULL)
 * @param [in] server           Key to validate server checksum (or NULL)
 * @param [in] privsvr          Key to validate KDC checksum (or NULL)
 * @param [in] with_realm       If true, expect the realm of @a principal
 *
 * This function is similar to krb5_pac_verify(), but adds a parameter
 * @a with_realm.  If @a with_realm is true, the PAC_CLIENT_INFO field is
 * expected to include the realm of @a principal as well as the name.  This
 * flag is necessary to verify PACs in cross-realm S4U2Self referral TGTs.
 *
 * @version New in 1.17
 */
krb5_error_code KRB5_CALLCONV
krb5_pac_verify_ext(krb5_context context, const krb5_pac pac,
                    krb5_timestamp authtime, krb5_const_principal principal,
                    const krb5_keyblock *server, const krb5_keyblock *privsvr,
                    krb5_boolean with_realm);

/**
 * Sign a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  authtime        Expected timestamp
 * @param [in]  principal       Expected principal name (or NULL)
 * @param [in]  server_key      Key for server checksum
 * @param [in]  privsvr_key     Key for KDC checksum
 * @param [out] data            Signed PAC encoding
 *
 * This function signs @a pac using the keys @a server_key and @a privsvr_key
 * and returns the signed encoding in @a data.  @a pac is modified to include
 * the server and KDC checksum buffers.  Use krb5_free_data_contents() to free
 * @a data when it is no longer needed.
 *
 * @version New in 1.10
 */
krb5_error_code KRB5_CALLCONV
krb5_pac_sign(krb5_context context, krb5_pac pac, krb5_timestamp authtime,
              krb5_const_principal principal, const krb5_keyblock *server_key,
              const krb5_keyblock *privsvr_key, krb5_data *data);

/**
 * Sign a PAC, possibly with a specified realm.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  authtime        Expected timestamp
 * @param [in]  principal       Principal name (or NULL)
 * @param [in]  server_key      Key for server checksum
 * @param [in]  privsvr_key     Key for KDC checksum
 * @param [in]  with_realm      If true, include the realm of @a principal
 * @param [out] data            Signed PAC encoding
 *
 * This function is similar to krb5_pac_sign(), but adds a parameter
 * @a with_realm.  If @a with_realm is true, the PAC_CLIENT_INFO field of the
 * signed PAC will include the realm of @a principal as well as the name.  This
 * flag is necessary to generate PACs for cross-realm S4U2Self referrals.
 *
 * @version New in 1.17
 */
krb5_error_code KRB5_CALLCONV
krb5_pac_sign_ext(krb5_context context, krb5_pac pac, krb5_timestamp authtime,
                  krb5_const_principal principal,
                  const krb5_keyblock *server_key,
                  const krb5_keyblock *privsvr_key, krb5_boolean with_realm,
                  krb5_data *data);


/*
 * Read client information from a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [out] authtime_out    Authentication timestamp (NULL if not needed)
 * @param [out] princname_out   Client account name
 *
 * Read the PAC_CLIENT_INFO buffer in @a pac.  Place the client account name as
 * a string in @a princname_out.  If @a authtime_out is not NULL, place the
 * initial authentication timestamp in @a authtime_out.
 *
 * @retval 0 on success, ENOENT if no PAC_CLIENT_INFO buffer is present in @a
 * pac, ERANGE if the buffer contains invalid lengths.
 *
 * @version New in 1.18
 */
krb5_error_code KRB5_CALLCONV
krb5_pac_get_client_info(krb5_context context, const krb5_pac pac,
                         krb5_timestamp *authtime_out, char **princname_out);

/**
 * Allow the application to override the profile's allow_weak_crypto setting.
 *
 * @param [in] context          Library context
 * @param [in] enable           Boolean flag
 *
 * This function allows an application to override the allow_weak_crypto
 * setting.  It is primarily for use by aklog.
 *
 * @retval 0  (always)
 */
krb5_error_code KRB5_CALLCONV
krb5_allow_weak_crypto(krb5_context context, krb5_boolean enable);

/**
 * A wrapper for passing information to a @c krb5_trace_callback.
 *
 * Currently, it only contains the formatted message as determined
 * the the format string and arguments of the tracing macro, but it
 * may be extended to contain more fields in the future.
 */
typedef struct _krb5_trace_info {
    const char *message;
} krb5_trace_info;

typedef void
(KRB5_CALLCONV *krb5_trace_callback)(krb5_context context,
                                     const krb5_trace_info *info,
                                     void *cb_data);

/**
 * Specify a callback function for trace events.
 *
 * @param [in] context          Library context
 * @param [in] fn               Callback function
 * @param [in] cb_data          Callback data
 *
 * Specify a callback for trace events occurring in krb5 operations performed
 * within @a context.  @a fn will be invoked with @a context as the first
 * argument, @a cb_data as the last argument, and a pointer to a
 * krb5_trace_info as the second argument.  If the trace callback is reset via
 * this function or @a context is destroyed, @a fn will be invoked with a NULL
 * second argument so it can clean up @a cb_data.  Supply a NULL value for @a
 * fn to disable trace callbacks within @a context.
 *
 * @note This function overrides the information passed through the
 * @a KRB5_TRACE environment variable.
 *
 * @version New in 1.9
 *
 * @return Returns KRB5_TRACE_NOSUPP if tracing is not supported in the library
 * (unless @a fn is NULL).
 */
krb5_error_code KRB5_CALLCONV
krb5_set_trace_callback(krb5_context context, krb5_trace_callback fn,
                        void *cb_data);

/**
 * Specify a file name for directing trace events.
 *
 * @param [in] context          Library context
 * @param [in] filename         File name
 *
 * Open @a filename for appending (creating it, if necessary) and set up a
 * callback to write trace events to it.
 *
 * @note This function overrides the information passed through the
 * @a KRB5_TRACE environment variable.
 *
 * @version New in 1.9
 *
 * @retval KRB5_TRACE_NOSUPP Tracing is not supported in the library.
 */
krb5_error_code KRB5_CALLCONV
krb5_set_trace_filename(krb5_context context, const char *filename);


/**
 * Hook function for inspecting or modifying messages sent to KDCs.
 *
 * @param [in]  context         Library context
 * @param [in]  data            Callback data
 * @param [in]  realm           The realm the message will be sent to
 * @param [in]  message         The original message to be sent to the KDC
 * @param [out] new_message_out Optional replacement message to be sent
 * @param [out] reply_out       Optional synthetic reply
 *
 * If the hook function returns an error code, the KDC communication will be
 * aborted and the error code will be returned to the library operation which
 * initiated the communication.
 *
 * If the hook function sets @a reply_out, @a message will not be sent to the
 * KDC, and the given reply will used instead.
 *
 * If the hook function sets @a new_message_out, the given message will be sent
 * to the KDC in place of @a message.
 *
 * If the hook function returns successfully without setting either output,
 * @a message will be sent to the KDC normally.
 *
 * The hook function should use krb5_copy_data() to construct the value for
 * @a new_message_out or @a reply_out, to ensure that it can be freed correctly
 * by the library.
 *
 * @version New in 1.15
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
typedef krb5_error_code
(KRB5_CALLCONV *krb5_pre_send_fn)(krb5_context context, void *data,
                                  const krb5_data *realm,
                                  const krb5_data *message,
                                  krb5_data **new_message_out,
                                  krb5_data **new_reply_out);

/**
 * Hook function for inspecting or overriding KDC replies.
 *
 * @param [in]  context         Library context
 * @param [in]  data            Callback data
 * @param [in]  code            Status of KDC communication
 * @param [in]  realm           The realm the reply was received from
 * @param [in]  message         The message sent to the realm's KDC
 * @param [in]  reply           The reply received from the KDC
 * @param [out] new_reply_out   Optional replacement reply
 *
 * If @a code is zero, @a reply contains the reply received from the KDC.  The
 * hook function may return an error code to simulate an error, may synthesize
 * a different reply by setting @a new_reply_out, or may simply return
 * successfully to do nothing.
 *
 * If @a code is non-zero, KDC communication failed and @a reply should be
 * ignored.  The hook function may return @a code or a different error code, or
 * may synthesize a reply by setting @a new_reply_out and return successfully.
 *
 * The hook function should use krb5_copy_data() to construct the value for
 * @a new_reply_out, to ensure that it can be freed correctly by the library.
 *
 * @version New in 1.15
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
typedef krb5_error_code
(KRB5_CALLCONV *krb5_post_recv_fn)(krb5_context context, void *data,
                                   krb5_error_code code,
                                   const krb5_data *realm,
                                   const krb5_data *message,
                                   const krb5_data *reply,
                                   krb5_data **new_reply_out);

/**
 * Set a KDC pre-send hook function.
 *
 * @param [in] context          Library context
 * @param [in] send_hook        Hook function (or NULL to disable the hook)
 * @param [in] data             Callback data to be passed to @a send_hook
 *
 * @a send_hook will be called before messages are sent to KDCs by library
 * functions such as krb5_get_credentials().  The hook function may inspect,
 * override, or synthesize its own reply to the message.
 *
 * @version New in 1.15
 */
void KRB5_CALLCONV
krb5_set_kdc_send_hook(krb5_context context, krb5_pre_send_fn send_hook,
                       void *data);

/**
 * Set a KDC post-receive hook function.
 *
 * @param [in] context          The library context.
 * @param [in] recv_hook        Hook function (or NULL to disable the hook)
 * @param [in] data             Callback data to be passed to @a recv_hook
 *
 * @a recv_hook will be called after a reply is received from a KDC during a
 * call to a library function such as krb5_get_credentials().  The hook
 * function may inspect or override the reply.  This hook will not be executed
 * if the pre-send hook returns a synthetic reply.
 *
 * @version New in 1.15
 */
void KRB5_CALLCONV
krb5_set_kdc_recv_hook(krb5_context context, krb5_post_recv_fn recv_hook,
                       void *data);

#if defined(TARGET_OS_MAC) && TARGET_OS_MAC
#    pragma pack(pop)
#endif

KRB5INT_END_DECLS

/* Don't use this!  We're going to phase it out.  It's just here to keep
   applications from breaking right away.  */
#define krb5_const const

#undef KRB5_ATTR_DEPRECATED

/** @} */ /* end of KRB5_H group */

#endif /* KRB5_GENERAL__ */
/*
 * et-h-krb5_err.h:
 * This file is automatically generated; please do not edit it.
 */

#include <et/com_err.h>

#define KRB5KDC_ERR_NONE                         (-1765328384L)
#define KRB5KDC_ERR_NAME_EXP                     (-1765328383L)
#define KRB5KDC_ERR_SERVICE_EXP                  (-1765328382L)
#define KRB5KDC_ERR_BAD_PVNO                     (-1765328381L)
#define KRB5KDC_ERR_C_OLD_MAST_KVNO              (-1765328380L)
#define KRB5KDC_ERR_S_OLD_MAST_KVNO              (-1765328379L)
#define KRB5KDC_ERR_C_PRINCIPAL_UNKNOWN          (-1765328378L)
#define KRB5KDC_ERR_S_PRINCIPAL_UNKNOWN          (-1765328377L)
#define KRB5KDC_ERR_PRINCIPAL_NOT_UNIQUE         (-1765328376L)
#define KRB5KDC_ERR_NULL_KEY                     (-1765328375L)
#define KRB5KDC_ERR_CANNOT_POSTDATE              (-1765328374L)
#define KRB5KDC_ERR_NEVER_VALID                  (-1765328373L)
#define KRB5KDC_ERR_POLICY                       (-1765328372L)
#define KRB5KDC_ERR_BADOPTION                    (-1765328371L)
#define KRB5KDC_ERR_ETYPE_NOSUPP                 (-1765328370L)
#define KRB5KDC_ERR_SUMTYPE_NOSUPP               (-1765328369L)
#define KRB5KDC_ERR_PADATA_TYPE_NOSUPP           (-1765328368L)
#define KRB5KDC_ERR_TRTYPE_NOSUPP                (-1765328367L)
#define KRB5KDC_ERR_CLIENT_REVOKED               (-1765328366L)
#define KRB5KDC_ERR_SERVICE_REVOKED              (-1765328365L)
#define KRB5KDC_ERR_TGT_REVOKED                  (-1765328364L)
#define KRB5KDC_ERR_CLIENT_NOTYET                (-1765328363L)
#define KRB5KDC_ERR_SERVICE_NOTYET               (-1765328362L)
#define KRB5KDC_ERR_KEY_EXP                      (-1765328361L)
#define KRB5KDC_ERR_PREAUTH_FAILED               (-1765328360L)
#define KRB5KDC_ERR_PREAUTH_REQUIRED             (-1765328359L)
#define KRB5KDC_ERR_SERVER_NOMATCH               (-1765328358L)
#define KRB5KDC_ERR_MUST_USE_USER2USER           (-1765328357L)
#define KRB5KDC_ERR_PATH_NOT_ACCEPTED            (-1765328356L)
#define KRB5KDC_ERR_SVC_UNAVAILABLE              (-1765328355L)
#define KRB5PLACEHOLD_30                         (-1765328354L)
#define KRB5KRB_AP_ERR_BAD_INTEGRITY             (-1765328353L)
#define KRB5KRB_AP_ERR_TKT_EXPIRED               (-1765328352L)
#define KRB5KRB_AP_ERR_TKT_NYV                   (-1765328351L)
#define KRB5KRB_AP_ERR_REPEAT                    (-1765328350L)
#define KRB5KRB_AP_ERR_NOT_US                    (-1765328349L)
#define KRB5KRB_AP_ERR_BADMATCH                  (-1765328348L)
#define KRB5KRB_AP_ERR_SKEW                      (-1765328347L)
#define KRB5KRB_AP_ERR_BADADDR                   (-1765328346L)
#define KRB5KRB_AP_ERR_BADVERSION                (-1765328345L)
#define KRB5KRB_AP_ERR_MSG_TYPE                  (-1765328344L)
#define KRB5KRB_AP_ERR_MODIFIED                  (-1765328343L)
#define KRB5KRB_AP_ERR_BADORDER                  (-1765328342L)
#define KRB5KRB_AP_ERR_ILL_CR_TKT                (-1765328341L)
#define KRB5KRB_AP_ERR_BADKEYVER                 (-1765328340L)
#define KRB5KRB_AP_ERR_NOKEY                     (-1765328339L)
#define KRB5KRB_AP_ERR_MUT_FAIL                  (-1765328338L)
#define KRB5KRB_AP_ERR_BADDIRECTION              (-1765328337L)
#define KRB5KRB_AP_ERR_METHOD                    (-1765328336L)
#define KRB5KRB_AP_ERR_BADSEQ                    (-1765328335L)
#define KRB5KRB_AP_ERR_INAPP_CKSUM               (-1765328334L)
#define KRB5KRB_AP_PATH_NOT_ACCEPTED             (-1765328333L)
#define KRB5KRB_ERR_RESPONSE_TOO_BIG             (-1765328332L)
#define KRB5PLACEHOLD_53                         (-1765328331L)
#define KRB5PLACEHOLD_54                         (-1765328330L)
#define KRB5PLACEHOLD_55                         (-1765328329L)
#define KRB5PLACEHOLD_56                         (-1765328328L)
#define KRB5PLACEHOLD_57                         (-1765328327L)
#define KRB5PLACEHOLD_58                         (-1765328326L)
#define KRB5PLACEHOLD_59                         (-1765328325L)
#define KRB5KRB_ERR_GENERIC                      (-1765328324L)
#define KRB5KRB_ERR_FIELD_TOOLONG                (-1765328323L)
#define KRB5KDC_ERR_CLIENT_NOT_TRUSTED           (-1765328322L)
#define KRB5KDC_ERR_KDC_NOT_TRUSTED              (-1765328321L)
#define KRB5KDC_ERR_INVALID_SIG                  (-1765328320L)
#define KRB5KDC_ERR_DH_KEY_PARAMETERS_NOT_ACCEPTED (-1765328319L)
#define KRB5KDC_ERR_CERTIFICATE_MISMATCH         (-1765328318L)
#define KRB5KRB_AP_ERR_NO_TGT                    (-1765328317L)
#define KRB5KDC_ERR_WRONG_REALM                  (-1765328316L)
#define KRB5KRB_AP_ERR_USER_TO_USER_REQUIRED     (-1765328315L)
#define KRB5KDC_ERR_CANT_VERIFY_CERTIFICATE      (-1765328314L)
#define KRB5KDC_ERR_INVALID_CERTIFICATE          (-1765328313L)
#define KRB5KDC_ERR_REVOKED_CERTIFICATE          (-1765328312L)
#define KRB5KDC_ERR_REVOCATION_STATUS_UNKNOWN    (-1765328311L)
#define KRB5KDC_ERR_REVOCATION_STATUS_UNAVAILABLE (-1765328310L)
#define KRB5KDC_ERR_CLIENT_NAME_MISMATCH         (-1765328309L)
#define KRB5KDC_ERR_KDC_NAME_MISMATCH            (-1765328308L)
#define KRB5KDC_ERR_INCONSISTENT_KEY_PURPOSE     (-1765328307L)
#define KRB5KDC_ERR_DIGEST_IN_CERT_NOT_ACCEPTED  (-1765328306L)
#define KRB5KDC_ERR_PA_CHECKSUM_MUST_BE_INCLUDED (-1765328305L)
#define KRB5KDC_ERR_DIGEST_IN_SIGNED_DATA_NOT_ACCEPTED (-1765328304L)
#define KRB5KDC_ERR_PUBLIC_KEY_ENCRYPTION_NOT_SUPPORTED (-1765328303L)
#define KRB5PLACEHOLD_82                         (-1765328302L)
#define KRB5PLACEHOLD_83                         (-1765328301L)
#define KRB5PLACEHOLD_84                         (-1765328300L)
#define KRB5KRB_AP_ERR_IAKERB_KDC_NOT_FOUND      (-1765328299L)
#define KRB5KRB_AP_ERR_IAKERB_KDC_NO_RESPONSE    (-1765328298L)
#define KRB5PLACEHOLD_87                         (-1765328297L)
#define KRB5PLACEHOLD_88                         (-1765328296L)
#define KRB5PLACEHOLD_89                         (-1765328295L)
#define KRB5KDC_ERR_PREAUTH_EXPIRED              (-1765328294L)
#define KRB5KDC_ERR_MORE_PREAUTH_DATA_REQUIRED   (-1765328293L)
#define KRB5PLACEHOLD_92                         (-1765328292L)
#define KRB5KDC_ERR_UNKNOWN_CRITICAL_FAST_OPTION (-1765328291L)
#define KRB5PLACEHOLD_94                         (-1765328290L)
#define KRB5PLACEHOLD_95                         (-1765328289L)
#define KRB5PLACEHOLD_96                         (-1765328288L)
#define KRB5PLACEHOLD_97                         (-1765328287L)
#define KRB5PLACEHOLD_98                         (-1765328286L)
#define KRB5PLACEHOLD_99                         (-1765328285L)
#define KRB5KDC_ERR_NO_ACCEPTABLE_KDF            (-1765328284L)
#define KRB5PLACEHOLD_101                        (-1765328283L)
#define KRB5PLACEHOLD_102                        (-1765328282L)
#define KRB5PLACEHOLD_103                        (-1765328281L)
#define KRB5PLACEHOLD_104                        (-1765328280L)
#define KRB5PLACEHOLD_105                        (-1765328279L)
#define KRB5PLACEHOLD_106                        (-1765328278L)
#define KRB5PLACEHOLD_107                        (-1765328277L)
#define KRB5PLACEHOLD_108                        (-1765328276L)
#define KRB5PLACEHOLD_109                        (-1765328275L)
#define KRB5PLACEHOLD_110                        (-1765328274L)
#define KRB5PLACEHOLD_111                        (-1765328273L)
#define KRB5PLACEHOLD_112                        (-1765328272L)
#define KRB5PLACEHOLD_113                        (-1765328271L)
#define KRB5PLACEHOLD_114                        (-1765328270L)
#define KRB5PLACEHOLD_115                        (-1765328269L)
#define KRB5PLACEHOLD_116                        (-1765328268L)
#define KRB5PLACEHOLD_117                        (-1765328267L)
#define KRB5PLACEHOLD_118                        (-1765328266L)
#define KRB5PLACEHOLD_119                        (-1765328265L)
#define KRB5PLACEHOLD_120                        (-1765328264L)
#define KRB5PLACEHOLD_121                        (-1765328263L)
#define KRB5PLACEHOLD_122                        (-1765328262L)
#define KRB5PLACEHOLD_123                        (-1765328261L)
#define KRB5PLACEHOLD_124                        (-1765328260L)
#define KRB5PLACEHOLD_125                        (-1765328259L)
#define KRB5PLACEHOLD_126                        (-1765328258L)
#define KRB5PLACEHOLD_127                        (-1765328257L)
#define KRB5_ERR_RCSID                           (-1765328256L)
#define KRB5_LIBOS_BADLOCKFLAG                   (-1765328255L)
#define KRB5_LIBOS_CANTREADPWD                   (-1765328254L)
#define KRB5_LIBOS_BADPWDMATCH                   (-1765328253L)
#define KRB5_LIBOS_PWDINTR                       (-1765328252L)
#define KRB5_PARSE_ILLCHAR                       (-1765328251L)
#define KRB5_PARSE_MALFORMED                     (-1765328250L)
#define KRB5_CONFIG_CANTOPEN                     (-1765328249L)
#define KRB5_CONFIG_BADFORMAT                    (-1765328248L)
#define KRB5_CONFIG_NOTENUFSPACE                 (-1765328247L)
#define KRB5_BADMSGTYPE                          (-1765328246L)
#define KRB5_CC_BADNAME                          (-1765328245L)
#define KRB5_CC_UNKNOWN_TYPE                     (-1765328244L)
#define KRB5_CC_NOTFOUND                         (-1765328243L)
#define KRB5_CC_END                              (-1765328242L)
#define KRB5_NO_TKT_SUPPLIED                     (-1765328241L)
#define KRB5KRB_AP_WRONG_PRINC                   (-1765328240L)
#define KRB5KRB_AP_ERR_TKT_INVALID               (-1765328239L)
#define KRB5_PRINC_NOMATCH                       (-1765328238L)
#define KRB5_KDCREP_MODIFIED                     (-1765328237L)
#define KRB5_KDCREP_SKEW                         (-1765328236L)
#define KRB5_IN_TKT_REALM_MISMATCH               (-1765328235L)
#define KRB5_PROG_ETYPE_NOSUPP                   (-1765328234L)
#define KRB5_PROG_KEYTYPE_NOSUPP                 (-1765328233L)
#define KRB5_WRONG_ETYPE                         (-1765328232L)
#define KRB5_PROG_SUMTYPE_NOSUPP                 (-1765328231L)
#define KRB5_REALM_UNKNOWN                       (-1765328230L)
#define KRB5_SERVICE_UNKNOWN                     (-1765328229L)
#define KRB5_KDC_UNREACH                         (-1765328228L)
#define KRB5_NO_LOCALNAME                        (-1765328227L)
#define KRB5_MUTUAL_FAILED                       (-1765328226L)
#define KRB5_RC_TYPE_EXISTS                      (-1765328225L)
#define KRB5_RC_MALLOC                           (-1765328224L)
#define KRB5_RC_TYPE_NOTFOUND                    (-1765328223L)
#define KRB5_RC_UNKNOWN                          (-1765328222L)
#define KRB5_RC_REPLAY                           (-1765328221L)
#define KRB5_RC_IO                               (-1765328220L)
#define KRB5_RC_NOIO                             (-1765328219L)
#define KRB5_RC_PARSE                            (-1765328218L)
#define KRB5_RC_IO_EOF                           (-1765328217L)
#define KRB5_RC_IO_MALLOC                        (-1765328216L)
#define KRB5_RC_IO_PERM                          (-1765328215L)
#define KRB5_RC_IO_IO                            (-1765328214L)
#define KRB5_RC_IO_UNKNOWN                       (-1765328213L)
#define KRB5_RC_IO_SPACE                         (-1765328212L)
#define KRB5_TRANS_CANTOPEN                      (-1765328211L)
#define KRB5_TRANS_BADFORMAT                     (-1765328210L)
#define KRB5_LNAME_CANTOPEN                      (-1765328209L)
#define KRB5_LNAME_NOTRANS                       (-1765328208L)
#define KRB5_LNAME_BADFORMAT                     (-1765328207L)
#define KRB5_CRYPTO_INTERNAL                     (-1765328206L)
#define KRB5_KT_BADNAME                          (-1765328205L)
#define KRB5_KT_UNKNOWN_TYPE                     (-1765328204L)
#define KRB5_KT_NOTFOUND                         (-1765328203L)
#define KRB5_KT_END                              (-1765328202L)
#define KRB5_KT_NOWRITE                          (-1765328201L)
#define KRB5_KT_IOERR                            (-1765328200L)
#define KRB5_NO_TKT_IN_RLM                       (-1765328199L)
#define KRB5DES_BAD_KEYPAR                       (-1765328198L)
#define KRB5DES_WEAK_KEY                         (-1765328197L)
#define KRB5_BAD_ENCTYPE                         (-1765328196L)
#define KRB5_BAD_KEYSIZE                         (-1765328195L)
#define KRB5_BAD_MSIZE                           (-1765328194L)
#define KRB5_CC_TYPE_EXISTS                      (-1765328193L)
#define KRB5_KT_TYPE_EXISTS                      (-1765328192L)
#define KRB5_CC_IO                               (-1765328191L)
#define KRB5_FCC_PERM                            (-1765328190L)
#define KRB5_FCC_NOFILE                          (-1765328189L)
#define KRB5_FCC_INTERNAL                        (-1765328188L)
#define KRB5_CC_WRITE                            (-1765328187L)
#define KRB5_CC_NOMEM                            (-1765328186L)
#define KRB5_CC_FORMAT                           (-1765328185L)
#define KRB5_CC_NOT_KTYPE                        (-1765328184L)
#define KRB5_INVALID_FLAGS                       (-1765328183L)
#define KRB5_NO_2ND_TKT                          (-1765328182L)
#define KRB5_NOCREDS_SUPPLIED                    (-1765328181L)
#define KRB5_SENDAUTH_BADAUTHVERS                (-1765328180L)
#define KRB5_SENDAUTH_BADAPPLVERS                (-1765328179L)
#define KRB5_SENDAUTH_BADRESPONSE                (-1765328178L)
#define KRB5_SENDAUTH_REJECTED                   (-1765328177L)
#define KRB5_PREAUTH_BAD_TYPE                    (-1765328176L)
#define KRB5_PREAUTH_NO_KEY                      (-1765328175L)
#define KRB5_PREAUTH_FAILED                      (-1765328174L)
#define KRB5_RCACHE_BADVNO                       (-1765328173L)
#define KRB5_CCACHE_BADVNO                       (-1765328172L)
#define KRB5_KEYTAB_BADVNO                       (-1765328171L)
#define KRB5_PROG_ATYPE_NOSUPP                   (-1765328170L)
#define KRB5_RC_REQUIRED                         (-1765328169L)
#define KRB5_ERR_BAD_HOSTNAME                    (-1765328168L)
#define KRB5_ERR_HOST_REALM_UNKNOWN              (-1765328167L)
#define KRB5_SNAME_UNSUPP_NAMETYPE               (-1765328166L)
#define KRB5KRB_AP_ERR_V4_REPLY                  (-1765328165L)
#define KRB5_REALM_CANT_RESOLVE                  (-1765328164L)
#define KRB5_TKT_NOT_FORWARDABLE                 (-1765328163L)
#define KRB5_FWD_BAD_PRINCIPAL                   (-1765328162L)
#define KRB5_GET_IN_TKT_LOOP                     (-1765328161L)
#define KRB5_CONFIG_NODEFREALM                   (-1765328160L)
#define KRB5_SAM_UNSUPPORTED                     (-1765328159L)
#define KRB5_SAM_INVALID_ETYPE                   (-1765328158L)
#define KRB5_SAM_NO_CHECKSUM                     (-1765328157L)
#define KRB5_SAM_BAD_CHECKSUM                    (-1765328156L)
#define KRB5_KT_NAME_TOOLONG                     (-1765328155L)
#define KRB5_KT_KVNONOTFOUND                     (-1765328154L)
#define KRB5_APPL_EXPIRED                        (-1765328153L)
#define KRB5_LIB_EXPIRED                         (-1765328152L)
#define KRB5_CHPW_PWDNULL                        (-1765328151L)
#define KRB5_CHPW_FAIL                           (-1765328150L)
#define KRB5_KT_FORMAT                           (-1765328149L)
#define KRB5_NOPERM_ETYPE                        (-1765328148L)
#define KRB5_CONFIG_ETYPE_NOSUPP                 (-1765328147L)
#define KRB5_OBSOLETE_FN                         (-1765328146L)
#define KRB5_EAI_FAIL                            (-1765328145L)
#define KRB5_EAI_NODATA                          (-1765328144L)
#define KRB5_EAI_NONAME                          (-1765328143L)
#define KRB5_EAI_SERVICE                         (-1765328142L)
#define KRB5_ERR_NUMERIC_REALM                   (-1765328141L)
#define KRB5_ERR_BAD_S2K_PARAMS                  (-1765328140L)
#define KRB5_ERR_NO_SERVICE                      (-1765328139L)
#define KRB5_CC_READONLY                         (-1765328138L)
#define KRB5_CC_NOSUPP                           (-1765328137L)
#define KRB5_DELTAT_BADFORMAT                    (-1765328136L)
#define KRB5_PLUGIN_NO_HANDLE                    (-1765328135L)
#define KRB5_PLUGIN_OP_NOTSUPP                   (-1765328134L)
#define KRB5_ERR_INVALID_UTF8                    (-1765328133L)
#define KRB5_ERR_FAST_REQUIRED                   (-1765328132L)
#define KRB5_LOCAL_ADDR_REQUIRED                 (-1765328131L)
#define KRB5_REMOTE_ADDR_REQUIRED                (-1765328130L)
#define KRB5_TRACE_NOSUPP                        (-1765328129L)
extern const struct error_table et_krb5_error_table;
extern void initialize_krb5_error_table(void);

/* For compatibility with Heimdal */
extern void initialize_krb5_error_table_r(struct et_list **list);

#define ERROR_TABLE_BASE_krb5 (-1765328384L)

/* for compatibility with older versions... */
#define init_krb5_err_tbl initialize_krb5_error_table
#define krb5_err_base ERROR_TABLE_BASE_krb5
/*
 * et-h-k5e1_err.h:
 * This file is automatically generated; please do not edit it.
 */

#include <et/com_err.h>

#define KRB5_PLUGIN_VER_NOTSUPP                  (-1750600192L)
#define KRB5_PLUGIN_BAD_MODULE_SPEC              (-1750600191L)
#define KRB5_PLUGIN_NAME_NOTFOUND                (-1750600190L)
#define KRB5KDC_ERR_DISCARD                      (-1750600189L)
#define KRB5_DCC_CANNOT_CREATE                   (-1750600188L)
#define KRB5_KCC_INVALID_ANCHOR                  (-1750600187L)
#define KRB5_KCC_UNKNOWN_VERSION                 (-1750600186L)
#define KRB5_KCC_INVALID_UID                     (-1750600185L)
#define KRB5_KCM_MALFORMED_REPLY                 (-1750600184L)
#define KRB5_KCM_RPC_ERROR                       (-1750600183L)
#define KRB5_KCM_REPLY_TOO_BIG                   (-1750600182L)
#define KRB5_KCM_NO_SERVER                       (-1750600181L)
extern const struct error_table et_k5e1_error_table;
extern void initialize_k5e1_error_table(void);

/* For compatibility with Heimdal */
extern void initialize_k5e1_error_table_r(struct et_list **list);

#define ERROR_TABLE_BASE_k5e1 (-1750600192L)

/* for compatibility with older versions... */
#define init_k5e1_err_tbl initialize_k5e1_error_table
#define k5e1_err_base ERROR_TABLE_BASE_k5e1
/*
 * et-h-kdb5_err.h:
 * This file is automatically generated; please do not edit it.
 */

#include <et/com_err.h>

#define KRB5_KDB_RCSID                           (-1780008448L)
#define KRB5_KDB_INUSE                           (-1780008447L)
#define KRB5_KDB_UK_SERROR                       (-1780008446L)
#define KRB5_KDB_UK_RERROR                       (-1780008445L)
#define KRB5_KDB_UNAUTH                          (-1780008444L)
#define KRB5_KDB_NOENTRY                         (-1780008443L)
#define KRB5_KDB_ILL_WILDCARD                    (-1780008442L)
#define KRB5_KDB_DB_INUSE                        (-1780008441L)
#define KRB5_KDB_DB_CHANGED                      (-1780008440L)
#define KRB5_KDB_TRUNCATED_RECORD                (-1780008439L)
#define KRB5_KDB_RECURSIVELOCK                   (-1780008438L)
#define KRB5_KDB_NOTLOCKED                       (-1780008437L)
#define KRB5_KDB_BADLOCKMODE                     (-1780008436L)
#define KRB5_KDB_DBNOTINITED                     (-1780008435L)
#define KRB5_KDB_DBINITED                        (-1780008434L)
#define KRB5_KDB_ILLDIRECTION                    (-1780008433L)
#define KRB5_KDB_NOMASTERKEY                     (-1780008432L)
#define KRB5_KDB_BADMASTERKEY                    (-1780008431L)
#define KRB5_KDB_INVALIDKEYSIZE                  (-1780008430L)
#define KRB5_KDB_CANTREAD_STORED                 (-1780008429L)
#define KRB5_KDB_BADSTORED_MKEY                  (-1780008428L)
#define KRB5_KDB_NOACTMASTERKEY                  (-1780008427L)
#define KRB5_KDB_KVNONOMATCH                     (-1780008426L)
#define KRB5_KDB_STORED_MKEY_NOTCURRENT          (-1780008425L)
#define KRB5_KDB_CANTLOCK_DB                     (-1780008424L)
#define KRB5_KDB_DB_CORRUPT                      (-1780008423L)
#define KRB5_KDB_BAD_VERSION                     (-1780008422L)
#define KRB5_KDB_BAD_SALTTYPE                    (-1780008421L)
#define KRB5_KDB_BAD_ENCTYPE                     (-1780008420L)
#define KRB5_KDB_BAD_CREATEFLAGS                 (-1780008419L)
#define KRB5_KDB_NO_PERMITTED_KEY                (-1780008418L)
#define KRB5_KDB_NO_MATCHING_KEY                 (-1780008417L)
#define KRB5_KDB_DBTYPE_NOTFOUND                 (-1780008416L)
#define KRB5_KDB_DBTYPE_NOSUP                    (-1780008415L)
#define KRB5_KDB_DBTYPE_INIT                     (-1780008414L)
#define KRB5_KDB_SERVER_INTERNAL_ERR             (-1780008413L)
#define KRB5_KDB_ACCESS_ERROR                    (-1780008412L)
#define KRB5_KDB_INTERNAL_ERROR                  (-1780008411L)
#define KRB5_KDB_CONSTRAINT_VIOLATION            (-1780008410L)
#define KRB5_LOG_CONV                            (-1780008409L)
#define KRB5_LOG_UNSTABLE                        (-1780008408L)
#define KRB5_LOG_CORRUPT                         (-1780008407L)
#define KRB5_LOG_ERROR                           (-1780008406L)
#define KRB5_KDB_DBTYPE_MISMATCH                 (-1780008405L)
#define KRB5_KDB_POLICY_REF                      (-1780008404L)
#define KRB5_KDB_STRINGS_TOOLONG                 (-1780008403L)
extern const struct error_table et_kdb5_error_table;
extern void initialize_kdb5_error_table(void);

/* For compatibility with Heimdal */
extern void initialize_kdb5_error_table_r(struct et_list **list);

#define ERROR_TABLE_BASE_kdb5 (-1780008448L)

/* for compatibility with older versions... */
#define init_kdb5_err_tbl initialize_kdb5_error_table
#define kdb5_err_base ERROR_TABLE_BASE_kdb5
/*
 * et-h-kv5m_err.h:
 * This file is automatically generated; please do not edit it.
 */

#include <et/com_err.h>

#define KV5M_NONE                                (-1760647424L)
#define KV5M_PRINCIPAL                           (-1760647423L)
#define KV5M_DATA                                (-1760647422L)
#define KV5M_KEYBLOCK                            (-1760647421L)
#define KV5M_CHECKSUM                            (-1760647420L)
#define KV5M_ENCRYPT_BLOCK                       (-1760647419L)
#define KV5M_ENC_DATA                            (-1760647418L)
#define KV5M_CRYPTOSYSTEM_ENTRY                  (-1760647417L)
#define KV5M_CS_TABLE_ENTRY                      (-1760647416L)
#define KV5M_CHECKSUM_ENTRY                      (-1760647415L)
#define KV5M_AUTHDATA                            (-1760647414L)
#define KV5M_TRANSITED                           (-1760647413L)
#define KV5M_ENC_TKT_PART                        (-1760647412L)
#define KV5M_TICKET                              (-1760647411L)
#define KV5M_AUTHENTICATOR                       (-1760647410L)
#define KV5M_TKT_AUTHENT                         (-1760647409L)
#define KV5M_CREDS                               (-1760647408L)
#define KV5M_LAST_REQ_ENTRY                      (-1760647407L)
#define KV5M_PA_DATA                             (-1760647406L)
#define KV5M_KDC_REQ                             (-1760647405L)
#define KV5M_ENC_KDC_REP_PART                    (-1760647404L)
#define KV5M_KDC_REP                             (-1760647403L)
#define KV5M_ERROR                               (-1760647402L)
#define KV5M_AP_REQ                              (-1760647401L)
#define KV5M_AP_REP                              (-1760647400L)
#define KV5M_AP_REP_ENC_PART                     (-1760647399L)
#define KV5M_RESPONSE                            (-1760647398L)
#define KV5M_SAFE                                (-1760647397L)
#define KV5M_PRIV                                (-1760647396L)
#define KV5M_PRIV_ENC_PART                       (-1760647395L)
#define KV5M_CRED                                (-1760647394L)
#define KV5M_CRED_INFO                           (-1760647393L)
#define KV5M_CRED_ENC_PART                       (-1760647392L)
#define KV5M_PWD_DATA                            (-1760647391L)
#define KV5M_ADDRESS                             (-1760647390L)
#define KV5M_KEYTAB_ENTRY                        (-1760647389L)
#define KV5M_CONTEXT                             (-1760647388L)
#define KV5M_OS_CONTEXT                          (-1760647387L)
#define KV5M_ALT_METHOD                          (-1760647386L)
#define KV5M_ETYPE_INFO_ENTRY                    (-1760647385L)
#define KV5M_DB_CONTEXT                          (-1760647384L)
#define KV5M_AUTH_CONTEXT                        (-1760647383L)
#define KV5M_KEYTAB                              (-1760647382L)
#define KV5M_RCACHE                              (-1760647381L)
#define KV5M_CCACHE                              (-1760647380L)
#define KV5M_PREAUTH_OPS                         (-1760647379L)
#define KV5M_SAM_CHALLENGE                       (-1760647378L)
#define KV5M_SAM_CHALLENGE_2                     (-1760647377L)
#define KV5M_SAM_KEY                             (-1760647376L)
#define KV5M_ENC_SAM_RESPONSE_ENC                (-1760647375L)
#define KV5M_ENC_SAM_RESPONSE_ENC_2              (-1760647374L)
#define KV5M_SAM_RESPONSE                        (-1760647373L)
#define KV5M_SAM_RESPONSE_2                      (-1760647372L)
#define KV5M_PREDICTED_SAM_RESPONSE              (-1760647371L)
#define KV5M_PASSWD_PHRASE_ELEMENT               (-1760647370L)
#define KV5M_GSS_OID                             (-1760647369L)
#define KV5M_GSS_QUEUE                           (-1760647368L)
#define KV5M_FAST_ARMORED_REQ                    (-1760647367L)
#define KV5M_FAST_REQ                            (-1760647366L)
#define KV5M_FAST_RESPONSE                       (-1760647365L)
#define KV5M_AUTHDATA_CONTEXT                    (-1760647364L)
extern const struct error_table et_kv5m_error_table;
extern void initialize_kv5m_error_table(void);

/* For compatibility with Heimdal */
extern void initialize_kv5m_error_table_r(struct et_list **list);

#define ERROR_TABLE_BASE_kv5m (-1760647424L)

/* for compatibility with older versions... */
#define init_kv5m_err_tbl initialize_kv5m_error_table
#define kv5m_err_base ERROR_TABLE_BASE_kv5m
/*
 * et-h-krb524_err.h:
 * This file is automatically generated; please do not edit it.
 */

#include <et/com_err.h>

#define KRB524_BADKEY                            (-1750206208L)
#define KRB524_BADADDR                           (-1750206207L)
#define KRB524_BADPRINC                          (-1750206206L)
#define KRB524_BADREALM                          (-1750206205L)
#define KRB524_V4ERR                             (-1750206204L)
#define KRB524_ENCFULL                           (-1750206203L)
#define KRB524_DECEMPTY                          (-1750206202L)
#define KRB524_NOTRESP                           (-1750206201L)
#define KRB524_KRB4_DISABLED                     (-1750206200L)
extern const struct error_table et_k524_error_table;
extern void initialize_k524_error_table(void);

/* For compatibility with Heimdal */
extern void initialize_k524_error_table_r(struct et_list **list);

#define ERROR_TABLE_BASE_k524 (-1750206208L)

/* for compatibility with older versions... */
#define init_k524_err_tbl initialize_k524_error_table
#define k524_err_base ERROR_TABLE_BASE_k524
/*
 * et-h-asn1_err.h:
 * This file is automatically generated; please do not edit it.
 */

#include <et/com_err.h>

#define ASN1_BAD_TIMEFORMAT                      (1859794432L)
#define ASN1_MISSING_FIELD                       (1859794433L)
#define ASN1_MISPLACED_FIELD                     (1859794434L)
#define ASN1_TYPE_MISMATCH                       (1859794435L)
#define ASN1_OVERFLOW                            (1859794436L)
#define ASN1_OVERRUN                             (1859794437L)
#define ASN1_BAD_ID                              (1859794438L)
#define ASN1_BAD_LENGTH                          (1859794439L)
#define ASN1_BAD_FORMAT                          (1859794440L)
#define ASN1_PARSE_ERROR                         (1859794441L)
#define ASN1_BAD_GMTIME                          (1859794442L)
#define ASN1_MISMATCH_INDEF                      (1859794443L)
#define ASN1_MISSING_EOC                         (1859794444L)
#define ASN1_OMITTED                             (1859794445L)
extern const struct error_table et_asn1_error_table;
extern void initialize_asn1_error_table(void);

/* For compatibility with Heimdal */
extern void initialize_asn1_error_table_r(struct et_list **list);

#define ERROR_TABLE_BASE_asn1 (1859794432L)

/* for compatibility with older versions... */
#define init_asn1_err_tbl initialize_asn1_error_table
#define asn1_err_base ERROR_TABLE_BASE_asn1
#endif /* KRB5_KRB5_H_INCLUDED */
