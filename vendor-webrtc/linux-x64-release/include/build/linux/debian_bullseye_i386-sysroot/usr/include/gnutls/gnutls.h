/* -*- c -*-
 * Copyright (C) 2000-2016 Free Software Foundation, Inc.
 * Copyright (C) 2015-2017 Red Hat, Inc.
 *
 * Author: Nikos Mavrogiannopoulos
 *
 * This file is part of GnuTLS.
 *
 * The GnuTLS is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>
 *
 */

/* This file contains the types and prototypes for all the
 * high level functionality of the gnutls main library.
 *
 * If the optional C++ binding was built, it is available in
 * gnutls/gnutlsxx.h.
 *
 * The openssl compatibility layer (which is under the GNU GPL
 * license) is in gnutls/openssl.h.
 *
 * The low level cipher functionality is in gnutls/crypto.h.
 */

#ifndef GNUTLS_GNUTLS_H
#define GNUTLS_GNUTLS_H

/* Get ssize_t. */
#include <sys/types.h>

/* Get size_t. */
#include <stddef.h>

/* Get time_t. */
#include <time.h>

/* *INDENT-OFF* */
#ifdef __cplusplus
extern "C" {
#endif
/* *INDENT-ON* */

#define GNUTLS_VERSION "3.7.1"

#define GNUTLS_VERSION_MAJOR 3
#define GNUTLS_VERSION_MINOR 7
#define GNUTLS_VERSION_PATCH 1

#define GNUTLS_VERSION_NUMBER 0x030701

#define GNUTLS_CIPHER_RIJNDAEL_128_CBC GNUTLS_CIPHER_AES_128_CBC
#define GNUTLS_CIPHER_RIJNDAEL_256_CBC GNUTLS_CIPHER_AES_256_CBC
#define GNUTLS_CIPHER_RIJNDAEL_CBC GNUTLS_CIPHER_AES_128_CBC
#define GNUTLS_CIPHER_ARCFOUR GNUTLS_CIPHER_ARCFOUR_128

#if !defined(GNUTLS_INTERNAL_BUILD) && defined(_WIN32)
# define _SYM_EXPORT __declspec(dllimport)
#else
# define _SYM_EXPORT
#endif

#ifdef __GNUC__
# define __GNUTLS_CONST__  __attribute__((const))
# define __GNUTLS_PURE__  __attribute__((pure))
#else
# define __GNUTLS_CONST__
# define __GNUTLS_PURE__
#endif


/* Use the following definition globally in your program to disable
 * implicit initialization of gnutls. */
#define GNUTLS_SKIP_GLOBAL_INIT int _gnutls_global_init_skip(void); \
    int _gnutls_global_init_skip(void) {return 1;}

/**
 * gnutls_cipher_algorithm_t:
 * @GNUTLS_CIPHER_UNKNOWN: Value to identify an unknown/unsupported algorithm.
 * @GNUTLS_CIPHER_NULL: The NULL (identity) encryption algorithm.
 * @GNUTLS_CIPHER_ARCFOUR_128: ARCFOUR stream cipher with 128-bit keys.
 * @GNUTLS_CIPHER_3DES_CBC: 3DES in CBC mode.
 * @GNUTLS_CIPHER_AES_128_CBC: AES in CBC mode with 128-bit keys.
 * @GNUTLS_CIPHER_AES_192_CBC: AES in CBC mode with 192-bit keys.
 * @GNUTLS_CIPHER_AES_256_CBC: AES in CBC mode with 256-bit keys.
 * @GNUTLS_CIPHER_AES_128_CFB8: AES in CFB8 mode with 128-bit keys.
 * @GNUTLS_CIPHER_AES_192_CFB8: AES in CFB8 mode with 192-bit keys.
 * @GNUTLS_CIPHER_AES_256_CFB8: AES in CFB8 mode with 256-bit keys.
 * @GNUTLS_CIPHER_ARCFOUR_40: ARCFOUR stream cipher with 40-bit keys.
 * @GNUTLS_CIPHER_CAMELLIA_128_CBC: Camellia in CBC mode with 128-bit keys.
 * @GNUTLS_CIPHER_CAMELLIA_192_CBC: Camellia in CBC mode with 192-bit keys.
 * @GNUTLS_CIPHER_CAMELLIA_256_CBC: Camellia in CBC mode with 256-bit keys.
 * @GNUTLS_CIPHER_RC2_40_CBC: RC2 in CBC mode with 40-bit keys.
 * @GNUTLS_CIPHER_DES_CBC: DES in CBC mode (56-bit keys).
 * @GNUTLS_CIPHER_AES_128_GCM: AES in GCM mode with 128-bit keys (AEAD).
 * @GNUTLS_CIPHER_AES_256_GCM: AES in GCM mode with 256-bit keys (AEAD).
 * @GNUTLS_CIPHER_AES_128_CCM: AES in CCM mode with 128-bit keys (AEAD).
 * @GNUTLS_CIPHER_AES_256_CCM: AES in CCM mode with 256-bit keys (AEAD).
 * @GNUTLS_CIPHER_AES_128_CCM_8: AES in CCM mode with 64-bit tag and 128-bit keys (AEAD).
 * @GNUTLS_CIPHER_AES_256_CCM_8: AES in CCM mode with 64-bit tag and 256-bit keys (AEAD).
 * @GNUTLS_CIPHER_CAMELLIA_128_GCM: CAMELLIA in GCM mode with 128-bit keys (AEAD).
 * @GNUTLS_CIPHER_CAMELLIA_256_GCM: CAMELLIA in GCM mode with 256-bit keys (AEAD).
 * @GNUTLS_CIPHER_SALSA20_256: Salsa20 with 256-bit keys.
 * @GNUTLS_CIPHER_ESTREAM_SALSA20_256: Estream's Salsa20 variant with 256-bit keys.
 * @GNUTLS_CIPHER_CHACHA20_32: Chacha20 cipher with 96-bit nonces and 32-bit block counters.
 * @GNUTLS_CIPHER_CHACHA20_64: Chacha20 cipher with 64-bit nonces and 64-bit block counters.
 * @GNUTLS_CIPHER_CHACHA20_POLY1305: The Chacha20 cipher with the Poly1305 authenticator (AEAD).
 * @GNUTLS_CIPHER_GOST28147_TC26Z_CFB: GOST 28147-89 (Magma) cipher in CFB mode with TC26 Z S-box.
 * @GNUTLS_CIPHER_GOST28147_CPA_CFB: GOST 28147-89 (Magma) cipher in CFB mode with CryptoPro A S-box.
 * @GNUTLS_CIPHER_GOST28147_CPB_CFB: GOST 28147-89 (Magma) cipher in CFB mode with CryptoPro B S-box.
 * @GNUTLS_CIPHER_GOST28147_CPC_CFB: GOST 28147-89 (Magma) cipher in CFB mode with CryptoPro C S-box.
 * @GNUTLS_CIPHER_GOST28147_CPD_CFB: GOST 28147-89 (Magma) cipher in CFB mode with CryptoPro D S-box.
 * @GNUTLS_CIPHER_AES_128_XTS: AES in XTS mode with 128-bit key + 128bit tweak key.
 * @GNUTLS_CIPHER_AES_256_XTS: AES in XTS mode with 256-bit key + 256bit tweak key.
 *                             Note that the XTS ciphers are message oriented.
 *                             The whole message needs to be provided with a single call, because
 *                             cipher-stealing requires to know where the message actually terminates
 *                             in order to be able to compute where the stealing occurs.
 * @GNUTLS_CIPHER_GOST28147_TC26Z_CNT: GOST 28147-89 (Magma) cipher in CNT mode with TC26 Z S-box.
 * @GNUTLS_CIPHER_MAGMA_CTR_ACPKM: GOST R 34.12-2015 (Magma) cipher in CTR-ACPKM mode.
 * @GNUTLS_CIPHER_KUZNYECHIK_CTR_ACPKM: GOST R 34.12-2015 (Kuznyechik) cipher in CTR-ACPKM mode.
 * @GNUTLS_CIPHER_IDEA_PGP_CFB: IDEA in CFB mode (placeholder - unsupported).
 * @GNUTLS_CIPHER_3DES_PGP_CFB: 3DES in CFB mode (placeholder - unsupported).
 * @GNUTLS_CIPHER_CAST5_PGP_CFB: CAST5 in CFB mode (placeholder - unsupported).
 * @GNUTLS_CIPHER_BLOWFISH_PGP_CFB: Blowfish in CFB mode (placeholder - unsupported).
 * @GNUTLS_CIPHER_SAFER_SK128_PGP_CFB: Safer-SK in CFB mode with 128-bit keys (placeholder - unsupported).
 * @GNUTLS_CIPHER_AES128_PGP_CFB: AES in CFB mode with 128-bit keys (placeholder - unsupported).
 * @GNUTLS_CIPHER_AES192_PGP_CFB: AES in CFB mode with 192-bit keys (placeholder - unsupported).
 * @GNUTLS_CIPHER_AES256_PGP_CFB: AES in CFB mode with 256-bit keys (placeholder - unsupported).
 * @GNUTLS_CIPHER_TWOFISH_PGP_CFB: Twofish in CFB mode (placeholder - unsupported).
 * @GNUTLS_CIPHER_AES_128_SIV: AES in SIV mode with 128-bit key.
 * @GNUTLS_CIPHER_AES_256_SIV: AES in SIV mode with 256-bit key.
 *                             Note that the SIV ciphers can only be used with
 *                             the AEAD interface, and the IV plays a role as
 *                             the authentication tag while it is prepended to
 *                             the cipher text.
 * @GNUTLS_CIPHER_AES_192_GCM: AES in GCM mode with 192-bit keys (AEAD).
 *
 * Enumeration of different symmetric encryption algorithms.
 */
typedef enum gnutls_cipher_algorithm {
	GNUTLS_CIPHER_UNKNOWN = 0,
	GNUTLS_CIPHER_NULL = 1,
	GNUTLS_CIPHER_ARCFOUR_128 = 2,
	GNUTLS_CIPHER_3DES_CBC = 3,
	GNUTLS_CIPHER_AES_128_CBC = 4,
	GNUTLS_CIPHER_AES_256_CBC = 5,
	GNUTLS_CIPHER_ARCFOUR_40 = 6,
	GNUTLS_CIPHER_CAMELLIA_128_CBC = 7,
	GNUTLS_CIPHER_CAMELLIA_256_CBC = 8,
	GNUTLS_CIPHER_AES_192_CBC = 9,
	GNUTLS_CIPHER_AES_128_GCM = 10,
	GNUTLS_CIPHER_AES_256_GCM = 11,
	GNUTLS_CIPHER_CAMELLIA_192_CBC = 12,
	GNUTLS_CIPHER_SALSA20_256 = 13,
	GNUTLS_CIPHER_ESTREAM_SALSA20_256 = 14,
	GNUTLS_CIPHER_CAMELLIA_128_GCM = 15,
	GNUTLS_CIPHER_CAMELLIA_256_GCM = 16,
	GNUTLS_CIPHER_RC2_40_CBC = 17,
	GNUTLS_CIPHER_DES_CBC = 18,
	GNUTLS_CIPHER_AES_128_CCM = 19,
	GNUTLS_CIPHER_AES_256_CCM = 20,
	GNUTLS_CIPHER_AES_128_CCM_8 = 21,
	GNUTLS_CIPHER_AES_256_CCM_8 = 22,
	GNUTLS_CIPHER_CHACHA20_POLY1305 = 23,
	GNUTLS_CIPHER_GOST28147_TC26Z_CFB = 24,
	GNUTLS_CIPHER_GOST28147_CPA_CFB = 25,
	GNUTLS_CIPHER_GOST28147_CPB_CFB = 26,
	GNUTLS_CIPHER_GOST28147_CPC_CFB = 27,
	GNUTLS_CIPHER_GOST28147_CPD_CFB = 28,
	GNUTLS_CIPHER_AES_128_CFB8 = 29,
	GNUTLS_CIPHER_AES_192_CFB8 = 30,
	GNUTLS_CIPHER_AES_256_CFB8 = 31,
	GNUTLS_CIPHER_AES_128_XTS = 32,
	GNUTLS_CIPHER_AES_256_XTS = 33,
	GNUTLS_CIPHER_GOST28147_TC26Z_CNT = 34,
	GNUTLS_CIPHER_CHACHA20_64 = 35,
	GNUTLS_CIPHER_CHACHA20_32 = 36,
	GNUTLS_CIPHER_AES_128_SIV = 37,
	GNUTLS_CIPHER_AES_256_SIV = 38,
	GNUTLS_CIPHER_AES_192_GCM = 39,
	GNUTLS_CIPHER_MAGMA_CTR_ACPKM = 40,
	GNUTLS_CIPHER_KUZNYECHIK_CTR_ACPKM = 41,

	/* used only for PGP internals. Ignored in TLS/SSL
	 */
	GNUTLS_CIPHER_IDEA_PGP_CFB = 200,
	GNUTLS_CIPHER_3DES_PGP_CFB = 201,
	GNUTLS_CIPHER_CAST5_PGP_CFB = 202,
	GNUTLS_CIPHER_BLOWFISH_PGP_CFB = 203,
	GNUTLS_CIPHER_SAFER_SK128_PGP_CFB = 204,
	GNUTLS_CIPHER_AES128_PGP_CFB = 205,
	GNUTLS_CIPHER_AES192_PGP_CFB = 206,
	GNUTLS_CIPHER_AES256_PGP_CFB = 207,
	GNUTLS_CIPHER_TWOFISH_PGP_CFB = 208
} gnutls_cipher_algorithm_t;

/**
 * gnutls_kx_algorithm_t:
 * @GNUTLS_KX_UNKNOWN: Unknown key-exchange algorithm.
 * @GNUTLS_KX_RSA: RSA key-exchange algorithm.
 * @GNUTLS_KX_DHE_DSS: DHE-DSS key-exchange algorithm.
 * @GNUTLS_KX_DHE_RSA: DHE-RSA key-exchange algorithm.
 * @GNUTLS_KX_ECDHE_RSA: ECDHE-RSA key-exchange algorithm.
 * @GNUTLS_KX_ECDHE_ECDSA: ECDHE-ECDSA key-exchange algorithm.
 * @GNUTLS_KX_ANON_DH: Anon-DH key-exchange algorithm.
 * @GNUTLS_KX_ANON_ECDH: Anon-ECDH key-exchange algorithm.
 * @GNUTLS_KX_SRP: SRP key-exchange algorithm.
 * @GNUTLS_KX_RSA_EXPORT: RSA-EXPORT key-exchange algorithm (defunc).
 * @GNUTLS_KX_SRP_RSA: SRP-RSA key-exchange algorithm.
 * @GNUTLS_KX_SRP_DSS: SRP-DSS key-exchange algorithm.
 * @GNUTLS_KX_PSK: PSK key-exchange algorithm.
 * @GNUTLS_KX_DHE_PSK: DHE-PSK key-exchange algorithm.
 * @GNUTLS_KX_ECDHE_PSK: ECDHE-PSK key-exchange algorithm.
 * @GNUTLS_KX_RSA_PSK: RSA-PSK key-exchange algorithm.
 * @GNUTLS_KX_VKO_GOST_12: VKO GOST R 34.10-2012 key-exchange algorithm.
 *
 * Enumeration of different key exchange algorithms.
 */
typedef enum {
	GNUTLS_KX_UNKNOWN = 0,
	GNUTLS_KX_RSA = 1,
	GNUTLS_KX_DHE_DSS = 2,
	GNUTLS_KX_DHE_RSA = 3,
	GNUTLS_KX_ANON_DH = 4,
	GNUTLS_KX_SRP = 5,
	GNUTLS_KX_RSA_EXPORT = 6,
	GNUTLS_KX_SRP_RSA = 7,
	GNUTLS_KX_SRP_DSS = 8,
	GNUTLS_KX_PSK = 9,
	GNUTLS_KX_DHE_PSK = 10,
	GNUTLS_KX_ANON_ECDH = 11,
	GNUTLS_KX_ECDHE_RSA = 12,
	GNUTLS_KX_ECDHE_ECDSA = 13,
	GNUTLS_KX_ECDHE_PSK = 14,
	GNUTLS_KX_RSA_PSK = 15,
	GNUTLS_KX_VKO_GOST_12 = 16
} gnutls_kx_algorithm_t;

/**
 * gnutls_params_type_t:
 * @GNUTLS_PARAMS_RSA_EXPORT: Session RSA-EXPORT parameters (defunc).
 * @GNUTLS_PARAMS_DH: Session Diffie-Hellman parameters.
 * @GNUTLS_PARAMS_ECDH: Session Elliptic-Curve Diffie-Hellman parameters.
 *
 * Enumeration of different TLS session parameter types.
 */
typedef enum {
	GNUTLS_PARAMS_RSA_EXPORT = 1,
	GNUTLS_PARAMS_DH = 2,
	GNUTLS_PARAMS_ECDH = 3
} gnutls_params_type_t;

/**
 * gnutls_credentials_type_t:
 * @GNUTLS_CRD_CERTIFICATE: Certificate credential.
 * @GNUTLS_CRD_ANON: Anonymous credential.
 * @GNUTLS_CRD_SRP: SRP credential.
 * @GNUTLS_CRD_PSK: PSK credential.
 * @GNUTLS_CRD_IA: IA credential.
 *
 * Enumeration of different credential types.
 */
typedef enum {
	GNUTLS_CRD_CERTIFICATE = 1,
	GNUTLS_CRD_ANON,
	GNUTLS_CRD_SRP,
	GNUTLS_CRD_PSK,
	GNUTLS_CRD_IA
} gnutls_credentials_type_t;

#define GNUTLS_MAC_SHA GNUTLS_MAC_SHA1
#define GNUTLS_DIG_SHA GNUTLS_DIG_SHA1

/**
 * gnutls_mac_algorithm_t:
 * @GNUTLS_MAC_UNKNOWN: Unknown MAC algorithm.
 * @GNUTLS_MAC_NULL: NULL MAC algorithm (empty output).
 * @GNUTLS_MAC_MD5: HMAC-MD5 algorithm.
 * @GNUTLS_MAC_SHA1: HMAC-SHA-1 algorithm.
 * @GNUTLS_MAC_RMD160: HMAC-RMD160 algorithm.
 * @GNUTLS_MAC_MD2: HMAC-MD2 algorithm.
 * @GNUTLS_MAC_SHA256: HMAC-SHA-256 algorithm.
 * @GNUTLS_MAC_SHA384: HMAC-SHA-384 algorithm.
 * @GNUTLS_MAC_SHA512: HMAC-SHA-512 algorithm.
 * @GNUTLS_MAC_SHA224: HMAC-SHA-224 algorithm.
 * @GNUTLS_MAC_MD5_SHA1: Combined MD5+SHA1 MAC placeholder.
 * @GNUTLS_MAC_GOSTR_94: HMAC GOST R 34.11-94 algorithm.
 * @GNUTLS_MAC_STREEBOG_256: HMAC GOST R 34.11-2001 (Streebog) algorithm, 256 bit.
 * @GNUTLS_MAC_STREEBOG_512: HMAC GOST R 34.11-2001 (Streebog) algorithm, 512 bit.
 * @GNUTLS_MAC_AEAD: MAC implicit through AEAD cipher.
 * @GNUTLS_MAC_UMAC_96: The UMAC-96 MAC algorithm (requires nonce).
 * @GNUTLS_MAC_UMAC_128: The UMAC-128 MAC algorithm (requires nonce).
 * @GNUTLS_MAC_AES_CMAC_128: The AES-CMAC-128 MAC algorithm.
 * @GNUTLS_MAC_AES_CMAC_256: The AES-CMAC-256 MAC algorithm.
 * @GNUTLS_MAC_AES_GMAC_128: The AES-GMAC-128 MAC algorithm (requires nonce).
 * @GNUTLS_MAC_AES_GMAC_192: The AES-GMAC-192 MAC algorithm (requires nonce).
 * @GNUTLS_MAC_AES_GMAC_256: The AES-GMAC-256 MAC algorithm (requires nonce).
 * @GNUTLS_MAC_SHA3_224: Reserved; unimplemented.
 * @GNUTLS_MAC_SHA3_256: Reserved; unimplemented.
 * @GNUTLS_MAC_SHA3_384: Reserved; unimplemented.
 * @GNUTLS_MAC_SHA3_512: Reserved; unimplemented.
 * @GNUTLS_MAC_GOST28147_TC26Z_IMIT: The GOST 28147-89 working in IMIT mode with TC26 Z S-box.
 * @GNUTLS_MAC_SHAKE_128: Reserved; unimplemented.
 * @GNUTLS_MAC_SHAKE_256: Reserved; unimplemented.
 * @GNUTLS_MAC_MAGMA_OMAC: GOST R 34.12-2015 (Magma) in OMAC (CMAC) mode.
 * @GNUTLS_MAC_KUZNYECHIK_OMAC: GOST R 34.12-2015 (Kuznyechik) in OMAC (CMAC) mode.
 *
 * Enumeration of different Message Authentication Code (MAC)
 * algorithms.
 */
typedef enum {
	GNUTLS_MAC_UNKNOWN = 0,
	GNUTLS_MAC_NULL = 1,
	GNUTLS_MAC_MD5 = 2,
	GNUTLS_MAC_SHA1 = 3,
	GNUTLS_MAC_RMD160 = 4,
	GNUTLS_MAC_MD2 = 5,
	GNUTLS_MAC_SHA256 = 6,
	GNUTLS_MAC_SHA384 = 7,
	GNUTLS_MAC_SHA512 = 8,
	GNUTLS_MAC_SHA224 = 9,
	GNUTLS_MAC_SHA3_224 = 10, /* reserved: no implementation */
	GNUTLS_MAC_SHA3_256 = 11, /* reserved: no implementation */
	GNUTLS_MAC_SHA3_384 = 12, /* reserved: no implementation */
	GNUTLS_MAC_SHA3_512 = 13, /* reserved: no implementation */
	GNUTLS_MAC_MD5_SHA1 = 14, /* reserved: no implementation */
	GNUTLS_MAC_GOSTR_94 = 15,
	GNUTLS_MAC_STREEBOG_256 = 16,
	GNUTLS_MAC_STREEBOG_512 = 17,
	/* If you add anything here, make sure you align with
	   gnutls_digest_algorithm_t. */
	GNUTLS_MAC_AEAD = 200,	/* indicates that MAC is on the cipher */
	GNUTLS_MAC_UMAC_96 = 201,
	GNUTLS_MAC_UMAC_128 = 202,
	GNUTLS_MAC_AES_CMAC_128 = 203,
	GNUTLS_MAC_AES_CMAC_256 = 204,
	GNUTLS_MAC_AES_GMAC_128 = 205,
	GNUTLS_MAC_AES_GMAC_192 = 206,
	GNUTLS_MAC_AES_GMAC_256 = 207,
	GNUTLS_MAC_GOST28147_TC26Z_IMIT = 208,
	GNUTLS_MAC_SHAKE_128 = 209,
	GNUTLS_MAC_SHAKE_256 = 210,
	GNUTLS_MAC_MAGMA_OMAC = 211,
	GNUTLS_MAC_KUZNYECHIK_OMAC = 212
} gnutls_mac_algorithm_t;

/**
 * gnutls_digest_algorithm_t:
 * @GNUTLS_DIG_UNKNOWN: Unknown hash algorithm.
 * @GNUTLS_DIG_NULL: NULL hash algorithm (empty output).
 * @GNUTLS_DIG_MD5: MD5 algorithm.
 * @GNUTLS_DIG_SHA1: SHA-1 algorithm.
 * @GNUTLS_DIG_RMD160: RMD160 algorithm.
 * @GNUTLS_DIG_MD2: MD2 algorithm.
 * @GNUTLS_DIG_SHA256: SHA-256 algorithm.
 * @GNUTLS_DIG_SHA384: SHA-384 algorithm.
 * @GNUTLS_DIG_SHA512: SHA-512 algorithm.
 * @GNUTLS_DIG_SHA224: SHA-224 algorithm.
 * @GNUTLS_DIG_SHA3_224: SHA3-224 algorithm.
 * @GNUTLS_DIG_SHA3_256: SHA3-256 algorithm.
 * @GNUTLS_DIG_SHA3_384: SHA3-384 algorithm.
 * @GNUTLS_DIG_SHA3_512: SHA3-512 algorithm.
 * @GNUTLS_DIG_MD5_SHA1: Combined MD5+SHA1 algorithm.
 * @GNUTLS_DIG_GOSTR_94: GOST R 34.11-94 algorithm.
 * @GNUTLS_DIG_STREEBOG_256: GOST R 34.11-2001 (Streebog) algorithm, 256 bit.
 * @GNUTLS_DIG_STREEBOG_512: GOST R 34.11-2001 (Streebog) algorithm, 512 bit.
 * @GNUTLS_DIG_SHAKE_128: Reserved; unimplemented.
 * @GNUTLS_DIG_SHAKE_256: Reserved; unimplemented.
 *
 * Enumeration of different digest (hash) algorithms.
 */
typedef enum {
	GNUTLS_DIG_UNKNOWN = GNUTLS_MAC_UNKNOWN,
	GNUTLS_DIG_NULL = GNUTLS_MAC_NULL,
	GNUTLS_DIG_MD5 = GNUTLS_MAC_MD5,
	GNUTLS_DIG_SHA1 = GNUTLS_MAC_SHA1,
	GNUTLS_DIG_RMD160 = GNUTLS_MAC_RMD160,
	GNUTLS_DIG_MD2 = GNUTLS_MAC_MD2,
	GNUTLS_DIG_SHA256 = GNUTLS_MAC_SHA256,
	GNUTLS_DIG_SHA384 = GNUTLS_MAC_SHA384,
	GNUTLS_DIG_SHA512 = GNUTLS_MAC_SHA512,
	GNUTLS_DIG_SHA224 = GNUTLS_MAC_SHA224,
	GNUTLS_DIG_SHA3_224 = GNUTLS_MAC_SHA3_224,
	GNUTLS_DIG_SHA3_256 = GNUTLS_MAC_SHA3_256,
	GNUTLS_DIG_SHA3_384 = GNUTLS_MAC_SHA3_384,
	GNUTLS_DIG_SHA3_512 = GNUTLS_MAC_SHA3_512,
	GNUTLS_DIG_MD5_SHA1 = GNUTLS_MAC_MD5_SHA1,
	GNUTLS_DIG_GOSTR_94 = GNUTLS_MAC_GOSTR_94,
	GNUTLS_DIG_STREEBOG_256 = GNUTLS_MAC_STREEBOG_256,
	GNUTLS_DIG_STREEBOG_512 = GNUTLS_MAC_STREEBOG_512,
	GNUTLS_DIG_SHAKE_128 = GNUTLS_MAC_SHAKE_128,
	GNUTLS_DIG_SHAKE_256 = GNUTLS_MAC_SHAKE_256
	    /* If you add anything here, make sure you align with
	       gnutls_mac_algorithm_t. */
} gnutls_digest_algorithm_t;

  /* exported for other gnutls headers. This is the maximum number of
   * algorithms (ciphers, kx or macs).
   */
#define GNUTLS_MAX_ALGORITHM_NUM 64
#define GNUTLS_MAX_SESSION_ID_SIZE 32


/**
 * gnutls_compression_method_t:
 * @GNUTLS_COMP_UNKNOWN: Unknown compression method.
 * @GNUTLS_COMP_NULL: The NULL compression method (no compression).
 * @GNUTLS_COMP_DEFLATE: The DEFLATE compression method from zlib.
 * @GNUTLS_COMP_ZLIB: Same as %GNUTLS_COMP_DEFLATE.
 *
 * Enumeration of different TLS compression methods.
 */
typedef enum {
	GNUTLS_COMP_UNKNOWN = 0,
	GNUTLS_COMP_NULL = 1,
	GNUTLS_COMP_DEFLATE = 2,
	GNUTLS_COMP_ZLIB = GNUTLS_COMP_DEFLATE
} gnutls_compression_method_t;


/**
 * gnutls_init_flags_t:
 *
 * @GNUTLS_SERVER: Connection end is a server.
 * @GNUTLS_CLIENT: Connection end is a client.
 * @GNUTLS_DATAGRAM: Connection is datagram oriented (DTLS). Since 3.0.0.
 * @GNUTLS_NONBLOCK: Connection should not block. Since 3.0.0.
 * @GNUTLS_NO_SIGNAL: In systems where SIGPIPE is delivered on send, it will be disabled. That flag has effect in systems which support the MSG_NOSIGNAL sockets flag (since 3.4.2).
 * @GNUTLS_NO_EXTENSIONS: Do not enable any TLS extensions by default (since 3.1.2). As TLS 1.2 and later require extensions this option is considered obsolete and should not be used.
 * @GNUTLS_NO_REPLAY_PROTECTION: Disable any replay protection in DTLS. This must only be used if  replay protection is achieved using other means. Since 3.2.2.
 * @GNUTLS_ALLOW_ID_CHANGE: Allow the peer to replace its certificate, or change its ID during a rehandshake. This change is often used in attacks and thus prohibited by default. Since 3.5.0.
 * @GNUTLS_ENABLE_FALSE_START: Enable the TLS false start on client side if the negotiated ciphersuites allow it. This will enable sending data prior to the handshake being complete, and may introduce a risk of crypto failure when combined with certain key exchanged; for that GnuTLS may not enable that option in ciphersuites that are known to be not safe for false start. Since 3.5.0.
 * @GNUTLS_ENABLE_EARLY_START: Under TLS1.3 allow the server to return earlier than the full handshake
 *   finish; similarly to false start the handshake will be completed once data are received by the
 *   client, while the server is able to transmit sooner. This is not enabled by default as it could
 *   break certain existing server assumptions and use-cases. Since 3.6.4.
 * @GNUTLS_ENABLE_EARLY_DATA: Under TLS1.3 allow the server to receive early data sent as part of the initial ClientHello (0-RTT). This is not enabled by default as early data has weaker security properties than other data. Since 3.6.5.
 * @GNUTLS_FORCE_CLIENT_CERT: When in client side and only a single cert is specified, send that certificate irrespective of the issuers expected by the server. Since 3.5.0.
 * @GNUTLS_NO_TICKETS: Flag to indicate that the session should not use resumption with session tickets.
 * @GNUTLS_KEY_SHARE_TOP3: Generate key shares for the top-3 different groups which are enabled.
 *   That is, as each group is associated with a key type (EC, finite field, x25519), generate
 *   three keys using %GNUTLS_PK_DH, %GNUTLS_PK_EC, %GNUTLS_PK_ECDH_X25519 if all of them are enabled.
 * @GNUTLS_KEY_SHARE_TOP2: Generate key shares for the top-2 different groups which are enabled.
 *   For example (ECDH + x25519). This is the default.
 * @GNUTLS_KEY_SHARE_TOP: Generate key share for the first group which is enabled.
 *   For example x25519. This option is the most performant for client (less CPU spent
 *   generating keys), but if the server doesn't support the advertized option it may
 *   result to more roundtrips needed to discover the server's choice.
 * @GNUTLS_NO_AUTO_REKEY: Disable auto-rekeying under TLS1.3. If this option is not specified
 *   gnutls will force a rekey after 2^24 records have been sent.
 * @GNUTLS_POST_HANDSHAKE_AUTH: Enable post handshake authentication for server and client. When set and
 *   a server requests authentication after handshake %GNUTLS_E_REAUTH_REQUEST will be returned
 *   by gnutls_record_recv(). A client should then call gnutls_reauth() to re-authenticate.
 * @GNUTLS_SAFE_PADDING_CHECK: Flag to indicate that the TLS 1.3 padding check will be done in a
 *   safe way which doesn't leak the pad size based on GnuTLS processing time. This is of use to
 *   applications which hide the length of transferred data via the TLS1.3 padding mechanism and
 *   are already taking steps to hide the data processing time. This comes at a performance
 *   penalty.
 * @GNUTLS_AUTO_REAUTH: Enable transparent re-authentication in client side when the server
 *    requests to. That is, reauthentication is handled within gnutls_record_recv(), and
 *    the %GNUTLS_E_REHANDSHAKE or %GNUTLS_E_REAUTH_REQUEST are not returned. This must be
 *    enabled with %GNUTLS_POST_HANDSHAKE_AUTH for TLS1.3. Enabling this flag requires to restore
 *    interrupted calls to gnutls_record_recv() based on the output of gnutls_record_get_direction(),
 *    since gnutls_record_recv() could be interrupted when sending when this flag is enabled.
 *    Note this flag may not be used if you are using the same session for sending and receiving
 *    in different threads.
 * @GNUTLS_ENABLE_EARLY_DATA: Under TLS1.3 allow the server to receive early data sent as part of the initial ClientHello (0-RTT). 
 *    This is not enabled by default as early data has weaker security properties than other data. Since 3.6.5.
 * @GNUTLS_ENABLE_RAWPK: Allows raw public-keys to be negotiated during the handshake. Since 3.6.6.
 * @GNUTLS_NO_AUTO_SEND_TICKET: Under TLS1.3 disable auto-sending of
 *    session tickets during the handshake.
 *
 * Enumeration of different flags for gnutls_init() function. All the flags
 * can be combined except @GNUTLS_SERVER and @GNUTLS_CLIENT which are mutually
 * exclusive.
 *
 * The key share options relate to the TLS 1.3 key share extension
 * which is a speculative key generation expecting that the server
 * would support the generated key.
 */
typedef enum {
	GNUTLS_SERVER = 1,
	GNUTLS_CLIENT = (1<<1),
	GNUTLS_DATAGRAM = (1<<2),
	GNUTLS_NONBLOCK = (1<<3),
	GNUTLS_NO_EXTENSIONS = (1<<4),
	GNUTLS_NO_REPLAY_PROTECTION = (1<<5),
	GNUTLS_NO_SIGNAL = (1<<6),
	GNUTLS_ALLOW_ID_CHANGE = (1<<7),
	GNUTLS_ENABLE_FALSE_START = (1<<8),
	GNUTLS_FORCE_CLIENT_CERT = (1<<9),
	GNUTLS_NO_TICKETS = (1<<10),
	GNUTLS_KEY_SHARE_TOP = (1<<11),
	GNUTLS_KEY_SHARE_TOP2 = (1<<12),
	GNUTLS_KEY_SHARE_TOP3 = (1<<13),
	GNUTLS_POST_HANDSHAKE_AUTH = (1<<14),
	GNUTLS_NO_AUTO_REKEY = (1<<15),
	GNUTLS_SAFE_PADDING_CHECK = (1<<16),
	GNUTLS_ENABLE_EARLY_START = (1<<17),
	GNUTLS_ENABLE_RAWPK = (1<<18),
	GNUTLS_AUTO_REAUTH = (1<<19),
	GNUTLS_ENABLE_EARLY_DATA = (1<<20),
	GNUTLS_NO_AUTO_SEND_TICKET = (1<<21)
} gnutls_init_flags_t;

/* compatibility defines (previous versions of gnutls
 * used defines instead of enumerated values). */
#define GNUTLS_SERVER (1)
#define GNUTLS_CLIENT (1<<1)
#define GNUTLS_DATAGRAM (1<<2)
#define GNUTLS_NONBLOCK (1<<3)
#define GNUTLS_NO_EXTENSIONS (1<<4)
#define GNUTLS_NO_REPLAY_PROTECTION (1<<5)
#define GNUTLS_NO_SIGNAL (1<<6)
#define GNUTLS_ALLOW_ID_CHANGE (1<<7)
#define GNUTLS_ENABLE_FALSE_START (1<<8)
#define GNUTLS_FORCE_CLIENT_CERT (1<<9)
#define GNUTLS_NO_TICKETS (1<<10)
#define GNUTLS_ENABLE_CERT_TYPE_NEG 0
	// Here for compatibility reasons

/**
 * gnutls_alert_level_t:
 * @GNUTLS_AL_WARNING: Alert of warning severity.
 * @GNUTLS_AL_FATAL: Alert of fatal severity.
 *
 * Enumeration of different TLS alert severities.
 */
typedef enum {
	GNUTLS_AL_WARNING = 1,
	GNUTLS_AL_FATAL
} gnutls_alert_level_t;

/**
 * gnutls_alert_description_t:
 * @GNUTLS_A_CLOSE_NOTIFY: Close notify.
 * @GNUTLS_A_UNEXPECTED_MESSAGE: Unexpected message.
 * @GNUTLS_A_BAD_RECORD_MAC: Bad record MAC.
 * @GNUTLS_A_DECRYPTION_FAILED: Decryption failed.
 * @GNUTLS_A_RECORD_OVERFLOW: Record overflow.
 * @GNUTLS_A_DECOMPRESSION_FAILURE: Decompression failed.
 * @GNUTLS_A_HANDSHAKE_FAILURE: Handshake failed.
 * @GNUTLS_A_SSL3_NO_CERTIFICATE: No certificate.
 * @GNUTLS_A_BAD_CERTIFICATE: Certificate is bad.
 * @GNUTLS_A_UNSUPPORTED_CERTIFICATE: Certificate is not supported.
 * @GNUTLS_A_CERTIFICATE_REVOKED: Certificate was revoked.
 * @GNUTLS_A_CERTIFICATE_EXPIRED: Certificate is expired.
 * @GNUTLS_A_CERTIFICATE_UNKNOWN: Unknown certificate.
 * @GNUTLS_A_ILLEGAL_PARAMETER: Illegal parameter.
 * @GNUTLS_A_UNKNOWN_CA: CA is unknown.
 * @GNUTLS_A_ACCESS_DENIED: Access was denied.
 * @GNUTLS_A_DECODE_ERROR: Decode error.
 * @GNUTLS_A_DECRYPT_ERROR: Decrypt error.
 * @GNUTLS_A_EXPORT_RESTRICTION: Export restriction.
 * @GNUTLS_A_PROTOCOL_VERSION: Error in protocol version.
 * @GNUTLS_A_INSUFFICIENT_SECURITY: Insufficient security.
 * @GNUTLS_A_INTERNAL_ERROR: Internal error.
 * @GNUTLS_A_INAPPROPRIATE_FALLBACK: Inappropriate fallback,
 * @GNUTLS_A_USER_CANCELED: User canceled.
 * @GNUTLS_A_NO_RENEGOTIATION: No renegotiation is allowed.
 * @GNUTLS_A_MISSING_EXTENSION: An extension was expected but was not seen
 * @GNUTLS_A_UNSUPPORTED_EXTENSION: An unsupported extension was
 *   sent.
 * @GNUTLS_A_CERTIFICATE_UNOBTAINABLE: Could not retrieve the
 *   specified certificate.
 * @GNUTLS_A_UNRECOGNIZED_NAME: The server name sent was not
 *   recognized.
 * @GNUTLS_A_UNKNOWN_PSK_IDENTITY: The SRP/PSK username is missing
 *   or not known.
 * @GNUTLS_A_CERTIFICATE_REQUIRED: Certificate is required.
 * @GNUTLS_A_NO_APPLICATION_PROTOCOL: The ALPN protocol requested is
 *   not supported by the peer.
 *
 * Enumeration of different TLS alerts.
 */
typedef enum {
	GNUTLS_A_CLOSE_NOTIFY,
	GNUTLS_A_UNEXPECTED_MESSAGE = 10,
	GNUTLS_A_BAD_RECORD_MAC = 20,
	GNUTLS_A_DECRYPTION_FAILED,
	GNUTLS_A_RECORD_OVERFLOW,
	GNUTLS_A_DECOMPRESSION_FAILURE = 30,
	GNUTLS_A_HANDSHAKE_FAILURE = 40,
	GNUTLS_A_SSL3_NO_CERTIFICATE = 41,
	GNUTLS_A_BAD_CERTIFICATE = 42,
	GNUTLS_A_UNSUPPORTED_CERTIFICATE,
	GNUTLS_A_CERTIFICATE_REVOKED,
	GNUTLS_A_CERTIFICATE_EXPIRED,
	GNUTLS_A_CERTIFICATE_UNKNOWN,
	GNUTLS_A_ILLEGAL_PARAMETER,
	GNUTLS_A_UNKNOWN_CA,
	GNUTLS_A_ACCESS_DENIED,
	GNUTLS_A_DECODE_ERROR = 50,
	GNUTLS_A_DECRYPT_ERROR,
	GNUTLS_A_EXPORT_RESTRICTION = 60,
	GNUTLS_A_PROTOCOL_VERSION = 70,
	GNUTLS_A_INSUFFICIENT_SECURITY,
	GNUTLS_A_INTERNAL_ERROR = 80,
	GNUTLS_A_INAPPROPRIATE_FALLBACK = 86,
	GNUTLS_A_USER_CANCELED = 90,
	GNUTLS_A_NO_RENEGOTIATION = 100,
	GNUTLS_A_MISSING_EXTENSION = 109,
	GNUTLS_A_UNSUPPORTED_EXTENSION = 110,
	GNUTLS_A_CERTIFICATE_UNOBTAINABLE = 111,
	GNUTLS_A_UNRECOGNIZED_NAME = 112,
	GNUTLS_A_UNKNOWN_PSK_IDENTITY = 115,
	GNUTLS_A_CERTIFICATE_REQUIRED = 116,
	GNUTLS_A_NO_APPLICATION_PROTOCOL = 120,
	GNUTLS_A_MAX = GNUTLS_A_NO_APPLICATION_PROTOCOL
} gnutls_alert_description_t;

/**
 * gnutls_handshake_description_t:
 * @GNUTLS_HANDSHAKE_HELLO_REQUEST: Hello request.
 * @GNUTLS_HANDSHAKE_HELLO_VERIFY_REQUEST: DTLS Hello verify request.
 * @GNUTLS_HANDSHAKE_CLIENT_HELLO: Client hello.
 * @GNUTLS_HANDSHAKE_SERVER_HELLO: Server hello.
 * @GNUTLS_HANDSHAKE_END_OF_EARLY_DATA: End of early data.
 * @GNUTLS_HANDSHAKE_HELLO_RETRY_REQUEST: Hello retry request.
 * @GNUTLS_HANDSHAKE_NEW_SESSION_TICKET: New session ticket.
 * @GNUTLS_HANDSHAKE_CERTIFICATE_PKT: Certificate packet.
 * @GNUTLS_HANDSHAKE_SERVER_KEY_EXCHANGE: Server key exchange.
 * @GNUTLS_HANDSHAKE_CERTIFICATE_REQUEST: Certificate request.
 * @GNUTLS_HANDSHAKE_SERVER_HELLO_DONE: Server hello done.
 * @GNUTLS_HANDSHAKE_CERTIFICATE_VERIFY: Certificate verify.
 * @GNUTLS_HANDSHAKE_CLIENT_KEY_EXCHANGE: Client key exchange.
 * @GNUTLS_HANDSHAKE_FINISHED: Finished.
 * @GNUTLS_HANDSHAKE_CERTIFICATE_STATUS: Certificate status (OCSP).
 * @GNUTLS_HANDSHAKE_KEY_UPDATE: TLS1.3 key update message.
 * @GNUTLS_HANDSHAKE_SUPPLEMENTAL: Supplemental.
 * @GNUTLS_HANDSHAKE_CHANGE_CIPHER_SPEC: Change Cipher Spec.
 * @GNUTLS_HANDSHAKE_CLIENT_HELLO_V2: SSLv2 Client Hello.
 * @GNUTLS_HANDSHAKE_ENCRYPTED_EXTENSIONS: Encrypted extensions message.
 *
 * Enumeration of different TLS handshake packets.
 */
typedef enum {
	GNUTLS_HANDSHAKE_HELLO_REQUEST = 0,
	GNUTLS_HANDSHAKE_CLIENT_HELLO = 1,
	GNUTLS_HANDSHAKE_SERVER_HELLO = 2,
	GNUTLS_HANDSHAKE_HELLO_VERIFY_REQUEST = 3,
	GNUTLS_HANDSHAKE_NEW_SESSION_TICKET = 4,
	GNUTLS_HANDSHAKE_END_OF_EARLY_DATA = 5,
	GNUTLS_HANDSHAKE_ENCRYPTED_EXTENSIONS = 8,
	GNUTLS_HANDSHAKE_CERTIFICATE_PKT = 11,
	GNUTLS_HANDSHAKE_SERVER_KEY_EXCHANGE = 12,
	GNUTLS_HANDSHAKE_CERTIFICATE_REQUEST = 13,
	GNUTLS_HANDSHAKE_SERVER_HELLO_DONE = 14,
	GNUTLS_HANDSHAKE_CERTIFICATE_VERIFY = 15,
	GNUTLS_HANDSHAKE_CLIENT_KEY_EXCHANGE = 16,
	GNUTLS_HANDSHAKE_FINISHED = 20,
	GNUTLS_HANDSHAKE_CERTIFICATE_STATUS = 22,
	GNUTLS_HANDSHAKE_SUPPLEMENTAL = 23,
	GNUTLS_HANDSHAKE_KEY_UPDATE = 24,
	GNUTLS_HANDSHAKE_CHANGE_CIPHER_SPEC = 254,
	GNUTLS_HANDSHAKE_CLIENT_HELLO_V2 = 1024,
	GNUTLS_HANDSHAKE_HELLO_RETRY_REQUEST = 1025,
} gnutls_handshake_description_t;

#define GNUTLS_HANDSHAKE_ANY ((unsigned int)-1)

const char
    *gnutls_handshake_description_get_name(gnutls_handshake_description_t
					   type);

/**
 * gnutls_certificate_status_t:
 * @GNUTLS_CERT_INVALID: The certificate is not signed by one of the
 *   known authorities or the signature is invalid (deprecated by the flags 
 *   %GNUTLS_CERT_SIGNATURE_FAILURE and %GNUTLS_CERT_SIGNER_NOT_FOUND).
 * @GNUTLS_CERT_SIGNATURE_FAILURE: The signature verification failed.
 * @GNUTLS_CERT_REVOKED: Certificate is revoked by its authority.  In X.509 this will be
 *   set only if CRLs are checked.
 * @GNUTLS_CERT_SIGNER_NOT_FOUND: The certificate's issuer is not known. 
 *   This is the case if the issuer is not included in the trusted certificate list.
 * @GNUTLS_CERT_SIGNER_NOT_CA: The certificate's signer was not a CA. This
 *   may happen if this was a version 1 certificate, which is common with
 *   some CAs, or a version 3 certificate without the basic constrains extension.
 * @GNUTLS_CERT_SIGNER_CONSTRAINTS_FAILURE: The certificate's signer constraints were
 *   violated.
 * @GNUTLS_CERT_INSECURE_ALGORITHM:  The certificate was signed using an insecure
 *   algorithm such as MD2 or MD5. These algorithms have been broken and
 *   should not be trusted.
 * @GNUTLS_CERT_NOT_ACTIVATED: The certificate is not yet activated.
 * @GNUTLS_CERT_EXPIRED: The certificate has expired.
 * @GNUTLS_CERT_REVOCATION_DATA_SUPERSEDED: The revocation data are old and have been superseded.
 * @GNUTLS_CERT_REVOCATION_DATA_ISSUED_IN_FUTURE: The revocation data have a future issue date.
 * @GNUTLS_CERT_UNEXPECTED_OWNER: The owner is not the expected one.
 * @GNUTLS_CERT_MISMATCH: The certificate presented isn't the expected one (TOFU)
 * @GNUTLS_CERT_PURPOSE_MISMATCH: The certificate or an intermediate does not match the intended purpose (extended key usage).
 * @GNUTLS_CERT_MISSING_OCSP_STATUS: The certificate requires the server to send the certifiate status, but no status was received.
 * @GNUTLS_CERT_INVALID_OCSP_STATUS: The received OCSP status response is invalid.
 * @GNUTLS_CERT_UNKNOWN_CRIT_EXTENSIONS: The certificate has extensions marked as critical which are not supported.
 *
 * Enumeration of certificate status codes.  Note that the status
 * bits may have different meanings in OpenPGP keys and X.509
 * certificate verification.
 */
typedef enum {
	GNUTLS_CERT_INVALID = 1 << 1,
	GNUTLS_CERT_REVOKED = 1 << 5,
	GNUTLS_CERT_SIGNER_NOT_FOUND = 1 << 6,
	GNUTLS_CERT_SIGNER_NOT_CA = 1 << 7,
	GNUTLS_CERT_INSECURE_ALGORITHM = 1 << 8,
	GNUTLS_CERT_NOT_ACTIVATED = 1 << 9,
	GNUTLS_CERT_EXPIRED = 1 << 10,
	GNUTLS_CERT_SIGNATURE_FAILURE = 1 << 11,
	GNUTLS_CERT_REVOCATION_DATA_SUPERSEDED = 1 << 12,
	GNUTLS_CERT_UNEXPECTED_OWNER = 1 << 14,
	GNUTLS_CERT_REVOCATION_DATA_ISSUED_IN_FUTURE = 1 << 15,
	GNUTLS_CERT_SIGNER_CONSTRAINTS_FAILURE = 1 << 16,
	GNUTLS_CERT_MISMATCH = 1 << 17,
	GNUTLS_CERT_PURPOSE_MISMATCH = 1 << 18,
	GNUTLS_CERT_MISSING_OCSP_STATUS = 1 << 19,
	GNUTLS_CERT_INVALID_OCSP_STATUS = 1 << 20,
	GNUTLS_CERT_UNKNOWN_CRIT_EXTENSIONS = 1 << 21
} gnutls_certificate_status_t;

/**
 * gnutls_certificate_request_t:
 * @GNUTLS_CERT_IGNORE: Ignore certificate.
 * @GNUTLS_CERT_REQUEST: Request certificate.
 * @GNUTLS_CERT_REQUIRE: Require certificate.
 *
 * Enumeration of certificate request types.
 */
typedef enum {
	GNUTLS_CERT_IGNORE = 0,
	GNUTLS_CERT_REQUEST = 1,
	GNUTLS_CERT_REQUIRE = 2
} gnutls_certificate_request_t;

/**
 * gnutls_openpgp_crt_status_t:
 * @GNUTLS_OPENPGP_CERT: Send entire certificate.
 * @GNUTLS_OPENPGP_CERT_FINGERPRINT: Send only certificate fingerprint.
 *
 * Enumeration of ways to send OpenPGP certificate.
 */
typedef enum {
	GNUTLS_OPENPGP_CERT = 0,
	GNUTLS_OPENPGP_CERT_FINGERPRINT = 1
} gnutls_openpgp_crt_status_t;

/**
 * gnutls_close_request_t:
 * @GNUTLS_SHUT_RDWR: Disallow further receives/sends.
 * @GNUTLS_SHUT_WR: Disallow further sends.
 *
 * Enumeration of how TLS session should be terminated.  See gnutls_bye().
 */
typedef enum {
	GNUTLS_SHUT_RDWR = 0,
	GNUTLS_SHUT_WR = 1
} gnutls_close_request_t;

/**
 * gnutls_protocol_t:
 * @GNUTLS_SSL3: SSL version 3.0.
 * @GNUTLS_TLS1_0: TLS version 1.0.
 * @GNUTLS_TLS1: Same as %GNUTLS_TLS1_0.
 * @GNUTLS_TLS1_1: TLS version 1.1.
 * @GNUTLS_TLS1_2: TLS version 1.2.
 * @GNUTLS_TLS1_3: TLS version 1.3.
 * @GNUTLS_DTLS1_0: DTLS version 1.0.
 * @GNUTLS_DTLS1_2: DTLS version 1.2.
 * @GNUTLS_DTLS0_9: DTLS version 0.9 (Cisco AnyConnect / OpenSSL 0.9.8e).
 * @GNUTLS_TLS_VERSION_MAX: Maps to the highest supported TLS version.
 * @GNUTLS_DTLS_VERSION_MAX: Maps to the highest supported DTLS version.
 * @GNUTLS_VERSION_UNKNOWN: Unknown SSL/TLS version.
 *
 * Enumeration of different SSL/TLS protocol versions.
 */
typedef enum {
	GNUTLS_SSL3 = 1,
	GNUTLS_TLS1_0 = 2,
	GNUTLS_TLS1 = GNUTLS_TLS1_0,
	GNUTLS_TLS1_1 = 3,
	GNUTLS_TLS1_2 = 4,
	GNUTLS_TLS1_3 = 5,

	GNUTLS_DTLS0_9 = 200,
	GNUTLS_DTLS1_0 = 201,	/* 201 */
	GNUTLS_DTLS1_2 = 202,
	GNUTLS_DTLS_VERSION_MIN = GNUTLS_DTLS0_9,
	GNUTLS_DTLS_VERSION_MAX = GNUTLS_DTLS1_2,
	GNUTLS_TLS_VERSION_MAX = GNUTLS_TLS1_3,
	GNUTLS_VERSION_UNKNOWN = 0xff	/* change it to 0xffff */
} gnutls_protocol_t;

#define GNUTLS_CRT_RAW GNUTLS_CRT_RAWPK

/**
 * gnutls_certificate_type_t:
 * @GNUTLS_CRT_UNKNOWN: Unknown certificate type.
 * @GNUTLS_CRT_X509: X.509 Certificate.
 * @GNUTLS_CRT_OPENPGP: OpenPGP certificate.
 * @GNUTLS_CRT_RAWPK: Raw public-key (SubjectPublicKeyInfo)
 *
 * Enumeration of different certificate types.
 */
typedef enum {
	GNUTLS_CRT_UNKNOWN = 0,
	GNUTLS_CRT_X509 = 1,
	GNUTLS_CRT_OPENPGP = 2,
	GNUTLS_CRT_RAWPK = 3,
	GNUTLS_CRT_MAX = GNUTLS_CRT_RAWPK
} gnutls_certificate_type_t;

/**
 * gnutls_x509_crt_fmt_t:
 * @GNUTLS_X509_FMT_DER: X.509 certificate in DER format (binary).
 * @GNUTLS_X509_FMT_PEM: X.509 certificate in PEM format (text).
 *
 * Enumeration of different certificate encoding formats.
 */
typedef enum {
	GNUTLS_X509_FMT_DER = 0,
	GNUTLS_X509_FMT_PEM = 1
} gnutls_x509_crt_fmt_t;

/**
 * gnutls_certificate_print_formats_t:
 * @GNUTLS_CRT_PRINT_FULL: Full information about certificate.
 * @GNUTLS_CRT_PRINT_FULL_NUMBERS: Full information about certificate and include easy to parse public key parameters.
 * @GNUTLS_CRT_PRINT_COMPACT: Information about certificate name in one line, plus identification of the public key.
 * @GNUTLS_CRT_PRINT_ONELINE: Information about certificate in one line.
 * @GNUTLS_CRT_PRINT_UNSIGNED_FULL: All info for an unsigned certificate.
 *
 * Enumeration of different certificate printing variants.
 */
typedef enum gnutls_certificate_print_formats {
	GNUTLS_CRT_PRINT_FULL = 0,
	GNUTLS_CRT_PRINT_ONELINE = 1,
	GNUTLS_CRT_PRINT_UNSIGNED_FULL = 2,
	GNUTLS_CRT_PRINT_COMPACT = 3,
	GNUTLS_CRT_PRINT_FULL_NUMBERS = 4
} gnutls_certificate_print_formats_t;

#define GNUTLS_PK_ECC GNUTLS_PK_ECDSA
#define GNUTLS_PK_EC GNUTLS_PK_ECDSA

#define GNUTLS_PK_ECDHX GNUTLS_PK_ECDH_X25519
/**
 * gnutls_pk_algorithm_t:
 * @GNUTLS_PK_UNKNOWN: Unknown public-key algorithm.
 * @GNUTLS_PK_RSA: RSA public-key algorithm.
 * @GNUTLS_PK_RSA_PSS: RSA public-key algorithm, with PSS padding.
 * @GNUTLS_PK_DSA: DSA public-key algorithm.
 * @GNUTLS_PK_DH: Diffie-Hellman algorithm. Used to generate parameters.
 * @GNUTLS_PK_ECDSA: Elliptic curve algorithm. These parameters are compatible with the ECDSA and ECDH algorithm.
 * @GNUTLS_PK_ECDH_X25519: Elliptic curve algorithm, restricted to ECDH as per rfc7748.
 * @GNUTLS_PK_EDDSA_ED25519: Edwards curve Digital signature algorithm. Used with SHA512 on signatures.
 * @GNUTLS_PK_GOST_01: GOST R 34.10-2001 algorithm per rfc5832.
 * @GNUTLS_PK_GOST_12_256: GOST R 34.10-2012 algorithm, 256-bit key per rfc7091.
 * @GNUTLS_PK_GOST_12_512: GOST R 34.10-2012 algorithm, 512-bit key per rfc7091.
 * @GNUTLS_PK_ECDH_X448: Elliptic curve algorithm, restricted to ECDH as per rfc7748.
 * @GNUTLS_PK_EDDSA_ED448: Edwards curve Digital signature algorithm. Used with SHAKE256 on signatures.
 *
 * Enumeration of different public-key algorithms.
 */
typedef enum {
	GNUTLS_PK_UNKNOWN = 0,
	GNUTLS_PK_RSA = 1,
	GNUTLS_PK_DSA = 2,
	GNUTLS_PK_DH = 3,
	GNUTLS_PK_ECDSA = 4,
	GNUTLS_PK_ECDH_X25519 = 5,
	GNUTLS_PK_RSA_PSS = 6,
	GNUTLS_PK_EDDSA_ED25519 = 7,
	GNUTLS_PK_GOST_01 = 8,
	GNUTLS_PK_GOST_12_256 = 9,
	GNUTLS_PK_GOST_12_512 = 10,
	GNUTLS_PK_ECDH_X448 = 11,
	GNUTLS_PK_EDDSA_ED448 = 12,
	GNUTLS_PK_MAX = GNUTLS_PK_EDDSA_ED448
} gnutls_pk_algorithm_t;


const char *gnutls_pk_algorithm_get_name(gnutls_pk_algorithm_t algorithm);

/**
 * gnutls_sign_algorithm_t:
 * @GNUTLS_SIGN_UNKNOWN: Unknown signature algorithm.
 * @GNUTLS_SIGN_RSA_RAW: Digital signature algorithm RSA with DigestInfo formatted data
 * @GNUTLS_SIGN_RSA_SHA1: Digital signature algorithm RSA with SHA-1
 * @GNUTLS_SIGN_RSA_SHA: Same as %GNUTLS_SIGN_RSA_SHA1.
 * @GNUTLS_SIGN_DSA_SHA1: Digital signature algorithm DSA with SHA-1
 * @GNUTLS_SIGN_DSA_SHA224: Digital signature algorithm DSA with SHA-224
 * @GNUTLS_SIGN_DSA_SHA256: Digital signature algorithm DSA with SHA-256
 * @GNUTLS_SIGN_DSA_SHA384: Digital signature algorithm DSA with SHA-384
 * @GNUTLS_SIGN_DSA_SHA512: Digital signature algorithm DSA with SHA-512
 * @GNUTLS_SIGN_DSA_SHA: Same as %GNUTLS_SIGN_DSA_SHA1.
 * @GNUTLS_SIGN_RSA_MD5: Digital signature algorithm RSA with MD5.
 * @GNUTLS_SIGN_RSA_MD2: Digital signature algorithm RSA with MD2.
 * @GNUTLS_SIGN_RSA_RMD160: Digital signature algorithm RSA with RMD-160.
 * @GNUTLS_SIGN_RSA_SHA256: Digital signature algorithm RSA with SHA-256.
 * @GNUTLS_SIGN_RSA_SHA384: Digital signature algorithm RSA with SHA-384.
 * @GNUTLS_SIGN_RSA_SHA512: Digital signature algorithm RSA with SHA-512.
 * @GNUTLS_SIGN_RSA_SHA224: Digital signature algorithm RSA with SHA-224.
 * @GNUTLS_SIGN_ECDSA_SHA1: ECDSA with SHA1.
 * @GNUTLS_SIGN_ECDSA_SHA224: Digital signature algorithm ECDSA with SHA-224.
 * @GNUTLS_SIGN_ECDSA_SHA256: Digital signature algorithm ECDSA with SHA-256.
 * @GNUTLS_SIGN_ECDSA_SHA384: Digital signature algorithm ECDSA with SHA-384.
 * @GNUTLS_SIGN_ECDSA_SHA512: Digital signature algorithm ECDSA with SHA-512.
 * @GNUTLS_SIGN_ECDSA_SECP256R1_SHA256: Digital signature algorithm ECDSA-SECP256R1 with SHA-256 (used in TLS 1.3 but not PKIX).
 * @GNUTLS_SIGN_ECDSA_SECP384R1_SHA384: Digital signature algorithm ECDSA-SECP384R1 with SHA-384 (used in TLS 1.3 but not PKIX).
 * @GNUTLS_SIGN_ECDSA_SECP521R1_SHA512: Digital signature algorithm ECDSA-SECP521R1 with SHA-512 (used in TLS 1.3 but not PKIX).
 * @GNUTLS_SIGN_ECDSA_SHA3_224: Digital signature algorithm ECDSA with SHA3-224.
 * @GNUTLS_SIGN_ECDSA_SHA3_256: Digital signature algorithm ECDSA with SHA3-256.
 * @GNUTLS_SIGN_ECDSA_SHA3_384: Digital signature algorithm ECDSA with SHA3-384.
 * @GNUTLS_SIGN_ECDSA_SHA3_512: Digital signature algorithm ECDSA with SHA3-512.
 * @GNUTLS_SIGN_DSA_SHA3_224: Digital signature algorithm DSA with SHA3-224.
 * @GNUTLS_SIGN_DSA_SHA3_256: Digital signature algorithm DSA with SHA3-256.
 * @GNUTLS_SIGN_DSA_SHA3_384: Digital signature algorithm DSA with SHA3-384.
 * @GNUTLS_SIGN_DSA_SHA3_512: Digital signature algorithm DSA with SHA3-512.
 * @GNUTLS_SIGN_RSA_SHA3_224: Digital signature algorithm RSA with SHA3-224.
 * @GNUTLS_SIGN_RSA_SHA3_256: Digital signature algorithm RSA with SHA3-256.
 * @GNUTLS_SIGN_RSA_SHA3_384: Digital signature algorithm RSA with SHA3-384.
 * @GNUTLS_SIGN_RSA_SHA3_512: Digital signature algorithm RSA with SHA3-512.
 * @GNUTLS_SIGN_RSA_PSS_RSAE_SHA256: Digital signature algorithm RSA with SHA-256,
 *      with PSS padding (RSA PKCS#1 1.5 certificate). This signature is identical
 *      to #GNUTLS_SIGN_RSA_PSS_SHA256, but they are distinct as the TLS1.3 protocol
 *      treats them differently.
 * @GNUTLS_SIGN_RSA_PSS_RSAE_SHA384: Digital signature algorithm RSA with SHA-384,
 *      with PSS padding (RSA PKCS#1 1.5 certificate). This signature is identical
 *      to #GNUTLS_SIGN_RSA_PSS_SHA384, but they are distinct as the TLS1.3 protocol
 *      treats them differently.
 * @GNUTLS_SIGN_RSA_PSS_RSAE_SHA512: Digital signature algorithm RSA with SHA-512,
 *      with PSS padding (RSA PKCS#1 1.5 certificate). This signature is identical
 *      to #GNUTLS_SIGN_RSA_PSS_SHA512, but they are distinct as the TLS1.3 protocol
 *      treats them differently.
 * @GNUTLS_SIGN_RSA_PSS_SHA256: Digital signature algorithm RSA with SHA-256, with PSS padding (RSA-PSS certificate).
 * @GNUTLS_SIGN_RSA_PSS_SHA384: Digital signature algorithm RSA with SHA-384, with PSS padding (RSA-PSS certificate).
 * @GNUTLS_SIGN_RSA_PSS_SHA512: Digital signature algorithm RSA with SHA-512, with PSS padding (RSA-PSS certificate).
 * @GNUTLS_SIGN_EDDSA_ED25519: Digital signature algorithm EdDSA with Ed25519 curve.
 * @GNUTLS_SIGN_GOST_94: Digital signature algorithm GOST R 34.10-2001 with GOST R 34.11-94
 * @GNUTLS_SIGN_GOST_256: Digital signature algorithm GOST R 34.10-2012 with GOST R 34.11-2012 256 bit
 * @GNUTLS_SIGN_GOST_512: Digital signature algorithm GOST R 34.10-2012 with GOST R 34.11-2012 512 bit
 * @GNUTLS_SIGN_EDDSA_ED448: Digital signature algorithm EdDSA with Ed448 curve.
 *
 * Enumeration of different digital signature algorithms.
 */
typedef enum {
	GNUTLS_SIGN_UNKNOWN = 0,
	GNUTLS_SIGN_RSA_SHA1 = 1,
	GNUTLS_SIGN_RSA_SHA = GNUTLS_SIGN_RSA_SHA1,
	GNUTLS_SIGN_DSA_SHA1 = 2,
	GNUTLS_SIGN_DSA_SHA = GNUTLS_SIGN_DSA_SHA1,
	GNUTLS_SIGN_RSA_MD5 = 3,
	GNUTLS_SIGN_RSA_MD2 = 4,
	GNUTLS_SIGN_RSA_RMD160 = 5,
	GNUTLS_SIGN_RSA_SHA256 = 6,
	GNUTLS_SIGN_RSA_SHA384 = 7,
	GNUTLS_SIGN_RSA_SHA512 = 8,
	GNUTLS_SIGN_RSA_SHA224 = 9,
	GNUTLS_SIGN_DSA_SHA224 = 10,
	GNUTLS_SIGN_DSA_SHA256 = 11,
	GNUTLS_SIGN_ECDSA_SHA1 = 12,
	GNUTLS_SIGN_ECDSA_SHA224 = 13,
	GNUTLS_SIGN_ECDSA_SHA256 = 14,
	GNUTLS_SIGN_ECDSA_SHA384 = 15,
	GNUTLS_SIGN_ECDSA_SHA512 = 16,
	GNUTLS_SIGN_DSA_SHA384 = 17,
	GNUTLS_SIGN_DSA_SHA512 = 18,
	GNUTLS_SIGN_ECDSA_SHA3_224 = 20,
	GNUTLS_SIGN_ECDSA_SHA3_256 = 21,
	GNUTLS_SIGN_ECDSA_SHA3_384 = 22,
	GNUTLS_SIGN_ECDSA_SHA3_512 = 23,

	GNUTLS_SIGN_DSA_SHA3_224 = 24,
	GNUTLS_SIGN_DSA_SHA3_256 = 25,
	GNUTLS_SIGN_DSA_SHA3_384 = 26,
	GNUTLS_SIGN_DSA_SHA3_512 = 27,
	GNUTLS_SIGN_RSA_SHA3_224 = 28,
	GNUTLS_SIGN_RSA_SHA3_256 = 29,
	GNUTLS_SIGN_RSA_SHA3_384 = 30,
	GNUTLS_SIGN_RSA_SHA3_512 = 31,

	GNUTLS_SIGN_RSA_PSS_SHA256 = 32,
	GNUTLS_SIGN_RSA_PSS_SHA384 = 33,
	GNUTLS_SIGN_RSA_PSS_SHA512 = 34,
	GNUTLS_SIGN_EDDSA_ED25519 = 35,
	GNUTLS_SIGN_RSA_RAW = 36,

	GNUTLS_SIGN_ECDSA_SECP256R1_SHA256 = 37,
	GNUTLS_SIGN_ECDSA_SECP384R1_SHA384 = 38,
	GNUTLS_SIGN_ECDSA_SECP521R1_SHA512 = 39,

	GNUTLS_SIGN_RSA_PSS_RSAE_SHA256 = 40,
	GNUTLS_SIGN_RSA_PSS_RSAE_SHA384 = 41,
	GNUTLS_SIGN_RSA_PSS_RSAE_SHA512 = 42,

	GNUTLS_SIGN_GOST_94 = 43,
	GNUTLS_SIGN_GOST_256 = 44,
	GNUTLS_SIGN_GOST_512 = 45,
	GNUTLS_SIGN_EDDSA_ED448 = 46,
	GNUTLS_SIGN_MAX = GNUTLS_SIGN_EDDSA_ED448
} gnutls_sign_algorithm_t;

/**
 * gnutls_ecc_curve_t:
 * @GNUTLS_ECC_CURVE_INVALID: Cannot be known
 * @GNUTLS_ECC_CURVE_SECP192R1: the SECP192R1 curve
 * @GNUTLS_ECC_CURVE_SECP224R1: the SECP224R1 curve
 * @GNUTLS_ECC_CURVE_SECP256R1: the SECP256R1 curve
 * @GNUTLS_ECC_CURVE_SECP384R1: the SECP384R1 curve
 * @GNUTLS_ECC_CURVE_SECP521R1: the SECP521R1 curve
 * @GNUTLS_ECC_CURVE_X25519: the X25519 curve (ECDH only)
 * @GNUTLS_ECC_CURVE_ED25519: the Ed25519 curve
 * @GNUTLS_ECC_CURVE_GOST256CPA: GOST R 34.10 CryptoPro 256 A curve
 * @GNUTLS_ECC_CURVE_GOST256CPB: GOST R 34.10 CryptoPro 256 B curve
 * @GNUTLS_ECC_CURVE_GOST256CPC: GOST R 34.10 CryptoPro 256 C curve
 * @GNUTLS_ECC_CURVE_GOST256CPXA: GOST R 34.10 CryptoPro 256 XchA curve
 * @GNUTLS_ECC_CURVE_GOST256CPXB: GOST R 34.10 CryptoPro 256 XchB curve
 * @GNUTLS_ECC_CURVE_GOST512A: GOST R 34.10 TC26 512 A curve
 * @GNUTLS_ECC_CURVE_GOST512B: GOST R 34.10 TC26 512 B curve
 * @GNUTLS_ECC_CURVE_GOST512C: GOST R 34.10 TC26 512 C curve
 * @GNUTLS_ECC_CURVE_GOST256A: GOST R 34.10 TC26 256 A curve
 * @GNUTLS_ECC_CURVE_GOST256B: GOST R 34.10 TC26 256 B curve
 * @GNUTLS_ECC_CURVE_GOST256C: GOST R 34.10 TC26 256 C curve
 * @GNUTLS_ECC_CURVE_GOST256D: GOST R 34.10 TC26 256 D curve
 * @GNUTLS_ECC_CURVE_X448: the X448 curve (ECDH only)
 * @GNUTLS_ECC_CURVE_ED448: the Ed448 curve
 *
 * Enumeration of ECC curves.
 */
typedef enum {
	GNUTLS_ECC_CURVE_INVALID = 0,
	GNUTLS_ECC_CURVE_SECP224R1,
	GNUTLS_ECC_CURVE_SECP256R1,
	GNUTLS_ECC_CURVE_SECP384R1,
	GNUTLS_ECC_CURVE_SECP521R1,
	GNUTLS_ECC_CURVE_SECP192R1,
	GNUTLS_ECC_CURVE_X25519,
	GNUTLS_ECC_CURVE_ED25519,
	GNUTLS_ECC_CURVE_GOST256CPA,
	GNUTLS_ECC_CURVE_GOST256CPB,
	GNUTLS_ECC_CURVE_GOST256CPC,
	GNUTLS_ECC_CURVE_GOST256CPXA,
	GNUTLS_ECC_CURVE_GOST256CPXB,
	GNUTLS_ECC_CURVE_GOST512A,
	GNUTLS_ECC_CURVE_GOST512B,
	GNUTLS_ECC_CURVE_GOST512C,
	GNUTLS_ECC_CURVE_GOST256A,
	GNUTLS_ECC_CURVE_GOST256B,
	GNUTLS_ECC_CURVE_GOST256C,
	GNUTLS_ECC_CURVE_GOST256D,
	GNUTLS_ECC_CURVE_X448,
	GNUTLS_ECC_CURVE_ED448,
	GNUTLS_ECC_CURVE_MAX = GNUTLS_ECC_CURVE_ED448
} gnutls_ecc_curve_t;

/**
 * gnutls_group_t:
 * @GNUTLS_GROUP_INVALID: Indicates unknown/invalid group
 * @GNUTLS_GROUP_SECP192R1: the SECP192R1 curve group (legacy, only for TLS 1.2 compatibility)
 * @GNUTLS_GROUP_SECP224R1: the SECP224R1 curve group (legacy, only for TLS 1.2 compatibility)
 * @GNUTLS_GROUP_SECP256R1: the SECP256R1 curve group
 * @GNUTLS_GROUP_SECP384R1: the SECP384R1 curve group
 * @GNUTLS_GROUP_SECP521R1: the SECP521R1 curve group
 * @GNUTLS_GROUP_X25519: the X25519 curve group
 * @GNUTLS_GROUP_GC256A: the GOST R 34.10 TC26 256 A curve group
 * @GNUTLS_GROUP_GC256B: the GOST R 34.10 TC26 256 B curve group
 * @GNUTLS_GROUP_GC256C: the GOST R 34.10 TC26 256 C curve group
 * @GNUTLS_GROUP_GC256D: the GOST R 34.10 TC26 256 D curve group
 * @GNUTLS_GROUP_GC512A: the GOST R 34.10 TC26 512 A curve group
 * @GNUTLS_GROUP_GC512B: the GOST R 34.10 TC26 512 B curve group
 * @GNUTLS_GROUP_GC512C: the GOST R 34.10 TC26 512 C curve group
 * @GNUTLS_GROUP_FFDHE2048: the FFDHE2048 group
 * @GNUTLS_GROUP_FFDHE3072: the FFDHE3072 group
 * @GNUTLS_GROUP_FFDHE4096: the FFDHE4096 group
 * @GNUTLS_GROUP_FFDHE6144: the FFDHE6144 group
 * @GNUTLS_GROUP_FFDHE8192: the FFDHE8192 group
 * @GNUTLS_GROUP_X448: the X448 curve group
 *
 * Enumeration of supported groups. It is intended to be backwards
 * compatible with the enumerations in %gnutls_ecc_curve_t for the groups
 * which are valid elliptic curves.
 */
typedef enum {
	GNUTLS_GROUP_INVALID = 0,
	GNUTLS_GROUP_SECP192R1 = GNUTLS_ECC_CURVE_SECP192R1,
	GNUTLS_GROUP_SECP224R1 = GNUTLS_ECC_CURVE_SECP224R1,
	GNUTLS_GROUP_SECP256R1 = GNUTLS_ECC_CURVE_SECP256R1,
	GNUTLS_GROUP_SECP384R1 = GNUTLS_ECC_CURVE_SECP384R1,
	GNUTLS_GROUP_SECP521R1 = GNUTLS_ECC_CURVE_SECP521R1,
	GNUTLS_GROUP_X25519 = GNUTLS_ECC_CURVE_X25519,
	GNUTLS_GROUP_X448 = GNUTLS_ECC_CURVE_X448,

	GNUTLS_GROUP_GC256A = GNUTLS_ECC_CURVE_GOST256A,
	GNUTLS_GROUP_GC256B = GNUTLS_ECC_CURVE_GOST256B,
	GNUTLS_GROUP_GC256C = GNUTLS_ECC_CURVE_GOST256C,
	GNUTLS_GROUP_GC256D = GNUTLS_ECC_CURVE_GOST256D,
	GNUTLS_GROUP_GC512A = GNUTLS_ECC_CURVE_GOST512A,
	GNUTLS_GROUP_GC512B = GNUTLS_ECC_CURVE_GOST512B,
	GNUTLS_GROUP_GC512C = GNUTLS_ECC_CURVE_GOST512C,

	GNUTLS_GROUP_FFDHE2048 = 256,
	GNUTLS_GROUP_FFDHE3072,
	GNUTLS_GROUP_FFDHE4096,
	GNUTLS_GROUP_FFDHE8192,
	GNUTLS_GROUP_FFDHE6144,
	GNUTLS_GROUP_MAX = GNUTLS_GROUP_FFDHE6144,
} gnutls_group_t;

/* macros to allow specifying a specific curve in gnutls_privkey_generate()
 * and gnutls_x509_privkey_generate() */
#define GNUTLS_CURVE_TO_BITS(curve) (unsigned int)(((unsigned int)1<<31)|((unsigned int)(curve)))
#define GNUTLS_BITS_TO_CURVE(bits) (((unsigned int)(bits)) & 0x7FFFFFFF)
#define GNUTLS_BITS_ARE_CURVE(bits) (((unsigned int)(bits)) & 0x80000000)

/**
 * gnutls_sec_param_t:
 * @GNUTLS_SEC_PARAM_UNKNOWN: Cannot be known
 * @GNUTLS_SEC_PARAM_INSECURE: Less than 42 bits of security
 * @GNUTLS_SEC_PARAM_EXPORT: 42 bits of security
 * @GNUTLS_SEC_PARAM_VERY_WEAK: 64 bits of security
 * @GNUTLS_SEC_PARAM_WEAK: 72 bits of security
 * @GNUTLS_SEC_PARAM_LOW: 80 bits of security
 * @GNUTLS_SEC_PARAM_LEGACY: 96 bits of security
 * @GNUTLS_SEC_PARAM_MEDIUM: 112 bits of security (used to be %GNUTLS_SEC_PARAM_NORMAL)
 * @GNUTLS_SEC_PARAM_HIGH: 128 bits of security
 * @GNUTLS_SEC_PARAM_ULTRA: 192 bits of security
 * @GNUTLS_SEC_PARAM_FUTURE: 256 bits of security
 *
 * Enumeration of security parameters for passive attacks.
 */
typedef enum {
	GNUTLS_SEC_PARAM_UNKNOWN = 0,
	GNUTLS_SEC_PARAM_INSECURE = 5,
	GNUTLS_SEC_PARAM_EXPORT = 10,
	GNUTLS_SEC_PARAM_VERY_WEAK = 15,
	GNUTLS_SEC_PARAM_WEAK = 20,
	GNUTLS_SEC_PARAM_LOW = 25,
	GNUTLS_SEC_PARAM_LEGACY = 30,
	GNUTLS_SEC_PARAM_MEDIUM = 35,
	GNUTLS_SEC_PARAM_HIGH = 40,
	GNUTLS_SEC_PARAM_ULTRA = 45,
	GNUTLS_SEC_PARAM_FUTURE = 50,
	GNUTLS_SEC_PARAM_MAX = GNUTLS_SEC_PARAM_FUTURE
} gnutls_sec_param_t;

/* old name */
#define GNUTLS_SEC_PARAM_NORMAL GNUTLS_SEC_PARAM_MEDIUM

/**
 * gnutls_channel_binding_t:
 * @GNUTLS_CB_TLS_UNIQUE: "tls-unique" (RFC 5929) channel binding
 *
 * Enumeration of support channel binding types.
 */
typedef enum {
	GNUTLS_CB_TLS_UNIQUE
} gnutls_channel_binding_t;

/**
 * gnutls_gost_paramset_t:
 * @GNUTLS_GOST_PARAMSET_UNKNOWN: Unknown/default parameter set
 * @GNUTLS_GOST_PARAMSET_TC26_Z: Specified by TC26, see rfc7836
 * @GNUTLS_GOST_PARAMSET_CP_A: CryptoPro-A, see rfc4357
 * @GNUTLS_GOST_PARAMSET_CP_B: CryptoPro-B, see rfc4357
 * @GNUTLS_GOST_PARAMSET_CP_C: CryptoPro-C, see rfc4357
 * @GNUTLS_GOST_PARAMSET_CP_D: CryptoPro-D, see rfc4357
 *
 * Enumeration of different GOST 28147 parameter sets.
 */
typedef enum {
	GNUTLS_GOST_PARAMSET_UNKNOWN = 0,
	GNUTLS_GOST_PARAMSET_TC26_Z,
	GNUTLS_GOST_PARAMSET_CP_A,
	GNUTLS_GOST_PARAMSET_CP_B,
	GNUTLS_GOST_PARAMSET_CP_C,
	GNUTLS_GOST_PARAMSET_CP_D
} gnutls_gost_paramset_t;

/**
 * gnutls_ctype_target_t:
 * @GNUTLS_CTYPE_CLIENT: for requesting client certificate type values.
 * @GNUTLS_CTYPE_SERVER: for requesting server certificate type values.
 * @GNUTLS_CTYPE_OURS: for requesting our certificate type values.
 * @GNUTLS_CTYPE_PEERS: for requesting the peers' certificate type values.
 *
 * Enumeration of certificate type targets with respect to asymmetric
 * certificate types as specified in RFC7250 and P2P connection set up
 * as specified in draft-vanrein-tls-symmetry-02.
 */
typedef enum {
	GNUTLS_CTYPE_CLIENT,
	GNUTLS_CTYPE_SERVER,
	GNUTLS_CTYPE_OURS,
	GNUTLS_CTYPE_PEERS
} gnutls_ctype_target_t;

/* If you want to change this, then also change the define in
 * gnutls_int.h, and recompile.
 */
typedef void *gnutls_transport_ptr_t;

struct gnutls_session_int;
typedef struct gnutls_session_int *gnutls_session_t;

struct gnutls_dh_params_int;
typedef struct gnutls_dh_params_int *gnutls_dh_params_t;

  /* XXX ugly. */
struct gnutls_x509_privkey_int;
typedef struct gnutls_x509_privkey_int *gnutls_rsa_params_t;

struct gnutls_priority_st;
typedef struct gnutls_priority_st *gnutls_priority_t;

typedef struct {
	unsigned char *data;
	unsigned int size;
} gnutls_datum_t;


typedef struct gnutls_params_st {
	gnutls_params_type_t type;
	union params {
		gnutls_dh_params_t dh;
		gnutls_rsa_params_t rsa_export;
	} params;
	int deinit;
} gnutls_params_st;

typedef int gnutls_params_function(gnutls_session_t, gnutls_params_type_t,
				   gnutls_params_st *);

/* internal functions */

int gnutls_init(gnutls_session_t * session, unsigned int flags);
void gnutls_deinit(gnutls_session_t session);
#define _gnutls_deinit(x) gnutls_deinit(x)

int gnutls_bye(gnutls_session_t session, gnutls_close_request_t how);

int gnutls_handshake(gnutls_session_t session);

int gnutls_reauth(gnutls_session_t session, unsigned int flags);

#define GNUTLS_DEFAULT_HANDSHAKE_TIMEOUT ((unsigned int)-1)
#define GNUTLS_INDEFINITE_TIMEOUT ((unsigned int)-2)
void gnutls_handshake_set_timeout(gnutls_session_t session,
				  unsigned int ms);
int gnutls_rehandshake(gnutls_session_t session);

#define GNUTLS_KU_PEER 1
int gnutls_session_key_update(gnutls_session_t session, unsigned flags);

gnutls_alert_description_t gnutls_alert_get(gnutls_session_t session);
int gnutls_alert_send(gnutls_session_t session,
		      gnutls_alert_level_t level,
		      gnutls_alert_description_t desc);
int gnutls_alert_send_appropriate(gnutls_session_t session, int err);
const char *gnutls_alert_get_name(gnutls_alert_description_t alert);
const char *gnutls_alert_get_strname(gnutls_alert_description_t alert);

gnutls_sec_param_t gnutls_pk_bits_to_sec_param(gnutls_pk_algorithm_t algo,
					       unsigned int bits);
const char *gnutls_sec_param_get_name(gnutls_sec_param_t param);
unsigned int gnutls_sec_param_to_pk_bits(gnutls_pk_algorithm_t algo,
					 gnutls_sec_param_t param);
unsigned int
	gnutls_sec_param_to_symmetric_bits(gnutls_sec_param_t param) __GNUTLS_CONST__;

/* Elliptic curves */
const char *
	gnutls_ecc_curve_get_name(gnutls_ecc_curve_t curve) __GNUTLS_CONST__;
const char *
	gnutls_ecc_curve_get_oid(gnutls_ecc_curve_t curve) __GNUTLS_CONST__;

const char *
	gnutls_group_get_name(gnutls_group_t group) __GNUTLS_CONST__;

int
	gnutls_ecc_curve_get_size(gnutls_ecc_curve_t curve) __GNUTLS_CONST__;
gnutls_ecc_curve_t gnutls_ecc_curve_get(gnutls_session_t session);

gnutls_group_t gnutls_group_get(gnutls_session_t session);

/* get information on the current session */
gnutls_cipher_algorithm_t gnutls_cipher_get(gnutls_session_t session);
gnutls_kx_algorithm_t gnutls_kx_get(gnutls_session_t session);
gnutls_mac_algorithm_t gnutls_mac_get(gnutls_session_t session);
gnutls_digest_algorithm_t gnutls_prf_hash_get(const gnutls_session_t session);
gnutls_certificate_type_t
gnutls_certificate_type_get(gnutls_session_t session);
gnutls_certificate_type_t
gnutls_certificate_type_get2(gnutls_session_t session,
								gnutls_ctype_target_t target);

int gnutls_sign_algorithm_get(gnutls_session_t session);
int gnutls_sign_algorithm_get_client(gnutls_session_t session);

int gnutls_sign_algorithm_get_requested(gnutls_session_t session,
					size_t indx,
					gnutls_sign_algorithm_t * algo);

/* the name of the specified algorithms */
const char *
	gnutls_cipher_get_name(gnutls_cipher_algorithm_t algorithm) __GNUTLS_CONST__;
const char *
	gnutls_mac_get_name(gnutls_mac_algorithm_t algorithm) __GNUTLS_CONST__;

const char *
	gnutls_digest_get_name(gnutls_digest_algorithm_t algorithm) __GNUTLS_CONST__;
const char *
	gnutls_digest_get_oid(gnutls_digest_algorithm_t algorithm) __GNUTLS_CONST__;

const char *
	gnutls_kx_get_name(gnutls_kx_algorithm_t algorithm) __GNUTLS_CONST__;
const char *
	gnutls_certificate_type_get_name(gnutls_certificate_type_t
					     type) __GNUTLS_CONST__;
const char *
	gnutls_pk_get_name(gnutls_pk_algorithm_t algorithm) __GNUTLS_CONST__;
const char *
	gnutls_pk_get_oid(gnutls_pk_algorithm_t algorithm) __GNUTLS_CONST__;

const char *
	gnutls_sign_get_name(gnutls_sign_algorithm_t algorithm) __GNUTLS_CONST__;

const char *gnutls_sign_get_oid(gnutls_sign_algorithm_t sign) __GNUTLS_CONST__;

const char *
	gnutls_gost_paramset_get_name(gnutls_gost_paramset_t param) __GNUTLS_CONST__;
const char *
	gnutls_gost_paramset_get_oid(gnutls_gost_paramset_t param) __GNUTLS_CONST__;

size_t
	gnutls_cipher_get_key_size(gnutls_cipher_algorithm_t algorithm) __GNUTLS_CONST__;
size_t
	gnutls_mac_get_key_size(gnutls_mac_algorithm_t algorithm) __GNUTLS_CONST__;

unsigned gnutls_sign_is_secure(gnutls_sign_algorithm_t algorithm) __GNUTLS_CONST__;

/* It is possible that a signature algorithm is ok to use for short-lived
 * data (e.g., to sign a TLS session), but not for data that are long-lived
 * like certificates. This flag is about checking the security of the algorithm
 * for long-lived data. */
#define GNUTLS_SIGN_FLAG_SECURE_FOR_CERTS 1
unsigned gnutls_sign_is_secure2(gnutls_sign_algorithm_t algorithm, unsigned int flags) __GNUTLS_CONST__;

gnutls_digest_algorithm_t
	gnutls_sign_get_hash_algorithm(gnutls_sign_algorithm_t sign) __GNUTLS_CONST__;
gnutls_pk_algorithm_t
	gnutls_sign_get_pk_algorithm(gnutls_sign_algorithm_t sign) __GNUTLS_CONST__;
gnutls_sign_algorithm_t
	gnutls_pk_to_sign(gnutls_pk_algorithm_t pk,
		  gnutls_digest_algorithm_t hash) __GNUTLS_CONST__;

unsigned
gnutls_sign_supports_pk_algorithm(gnutls_sign_algorithm_t sign, gnutls_pk_algorithm_t pk) __GNUTLS_CONST__;

#define gnutls_sign_algorithm_get_name gnutls_sign_get_name

gnutls_mac_algorithm_t gnutls_mac_get_id(const char *name) __GNUTLS_CONST__;
gnutls_digest_algorithm_t gnutls_digest_get_id(const char *name) __GNUTLS_CONST__;

gnutls_cipher_algorithm_t
	gnutls_cipher_get_id(const char *name) __GNUTLS_CONST__;

gnutls_kx_algorithm_t
	gnutls_kx_get_id(const char *name) __GNUTLS_CONST__;
gnutls_protocol_t
	gnutls_protocol_get_id(const char *name) __GNUTLS_CONST__;
gnutls_certificate_type_t
	gnutls_certificate_type_get_id(const char *name) __GNUTLS_CONST__;
gnutls_pk_algorithm_t
	gnutls_pk_get_id(const char *name) __GNUTLS_CONST__;
gnutls_sign_algorithm_t
	gnutls_sign_get_id(const char *name) __GNUTLS_CONST__;
gnutls_ecc_curve_t gnutls_ecc_curve_get_id(const char *name)  __GNUTLS_CONST__;
gnutls_pk_algorithm_t gnutls_ecc_curve_get_pk(gnutls_ecc_curve_t curve) __GNUTLS_CONST__;
gnutls_group_t gnutls_group_get_id(const char *name);

gnutls_digest_algorithm_t
	gnutls_oid_to_digest(const char *oid)  __GNUTLS_CONST__;
gnutls_mac_algorithm_t
	gnutls_oid_to_mac(const char *oid)  __GNUTLS_CONST__;
gnutls_pk_algorithm_t
	gnutls_oid_to_pk(const char *oid) __GNUTLS_CONST__;
gnutls_sign_algorithm_t
	gnutls_oid_to_sign(const char *oid) __GNUTLS_CONST__;
gnutls_ecc_curve_t
	gnutls_oid_to_ecc_curve(const char *oid) __GNUTLS_CONST__;
gnutls_gost_paramset_t
	gnutls_oid_to_gost_paramset(const char *oid) __GNUTLS_CONST__;

  /* list supported algorithms */
const gnutls_ecc_curve_t *
	gnutls_ecc_curve_list(void)  __GNUTLS_PURE__;
const gnutls_group_t *
	gnutls_group_list(void)  __GNUTLS_PURE__;
const gnutls_cipher_algorithm_t *
	gnutls_cipher_list(void) __GNUTLS_PURE__;
const gnutls_mac_algorithm_t *
	gnutls_mac_list(void) __GNUTLS_PURE__;
const gnutls_digest_algorithm_t *
	gnutls_digest_list(void) __GNUTLS_PURE__;
const gnutls_protocol_t *
	gnutls_protocol_list(void) __GNUTLS_PURE__;
const gnutls_certificate_type_t *
	gnutls_certificate_type_list(void) __GNUTLS_PURE__;
const gnutls_kx_algorithm_t *
	gnutls_kx_list(void) __GNUTLS_PURE__;
const gnutls_pk_algorithm_t *
	gnutls_pk_list(void) __GNUTLS_PURE__;
const gnutls_sign_algorithm_t *
	gnutls_sign_list(void) __GNUTLS_PURE__;
const char *
	gnutls_cipher_suite_info(size_t idx,
			         unsigned char *cs_id,
				 gnutls_kx_algorithm_t * kx,
				 gnutls_cipher_algorithm_t * cipher,
				 gnutls_mac_algorithm_t * mac,
				 gnutls_protocol_t * min_version);

  /* error functions */
int gnutls_error_is_fatal(int error) __GNUTLS_CONST__;
int gnutls_error_to_alert(int err, int *level);

void gnutls_perror(int error);
const char * gnutls_strerror(int error) __GNUTLS_CONST__;
const char * gnutls_strerror_name(int error) __GNUTLS_CONST__;

/* Semi-internal functions.
 */
void gnutls_handshake_set_private_extensions(gnutls_session_t session,
					     int allow);
int gnutls_handshake_set_random(gnutls_session_t session,
				const gnutls_datum_t * random);

gnutls_handshake_description_t
gnutls_handshake_get_last_out(gnutls_session_t session);
gnutls_handshake_description_t
gnutls_handshake_get_last_in(gnutls_session_t session);

/* Record layer functions.
 */
#define GNUTLS_HEARTBEAT_WAIT 1
int gnutls_heartbeat_ping(gnutls_session_t session, size_t data_size,
			  unsigned int max_tries, unsigned int flags);
int gnutls_heartbeat_pong(gnutls_session_t session, unsigned int flags);

void gnutls_record_set_timeout(gnutls_session_t session, unsigned int ms);
void gnutls_record_disable_padding(gnutls_session_t session);

void gnutls_record_cork(gnutls_session_t session);
#define GNUTLS_RECORD_WAIT 1
int gnutls_record_uncork(gnutls_session_t session, unsigned int flags);
size_t gnutls_record_discard_queued(gnutls_session_t session);

int
gnutls_record_get_state(gnutls_session_t session,
			unsigned read,
			gnutls_datum_t *mac_key,
			gnutls_datum_t *IV,
			gnutls_datum_t *cipher_key,
			unsigned char seq_number[8]);

int
gnutls_record_set_state(gnutls_session_t session,
			unsigned read,
			const unsigned char seq_number[8]);

typedef struct {
	size_t low;
	size_t high;
} gnutls_range_st;

int gnutls_range_split(gnutls_session_t session,
		       const gnutls_range_st * orig,
		       gnutls_range_st * small_range,
		       gnutls_range_st * rem_range);

ssize_t gnutls_record_send(gnutls_session_t session, const void *data,
			   size_t data_size);
ssize_t gnutls_record_send2(gnutls_session_t session, const void *data,
			    size_t data_size, size_t pad, unsigned flags);
ssize_t gnutls_record_send_range(gnutls_session_t session,
				 const void *data, size_t data_size,
				 const gnutls_range_st * range);
ssize_t gnutls_record_recv(gnutls_session_t session, void *data,
			   size_t data_size);

typedef struct mbuffer_st *gnutls_packet_t;

ssize_t
gnutls_record_recv_packet(gnutls_session_t session,
			  gnutls_packet_t *packet);

void gnutls_packet_get(gnutls_packet_t packet, gnutls_datum_t *data, unsigned char *sequence);
void gnutls_packet_deinit(gnutls_packet_t packet);

#define gnutls_read gnutls_record_recv
#define gnutls_write gnutls_record_send
ssize_t gnutls_record_recv_seq(gnutls_session_t session, void *data,
			       size_t data_size, unsigned char *seq);

size_t gnutls_record_overhead_size(gnutls_session_t session);

size_t
	gnutls_est_record_overhead_size(gnutls_protocol_t version,
				        gnutls_cipher_algorithm_t cipher,
				        gnutls_mac_algorithm_t mac,
				        gnutls_compression_method_t comp,
				        unsigned int flags) __GNUTLS_CONST__;

void gnutls_session_enable_compatibility_mode(gnutls_session_t session);
#define gnutls_record_set_max_empty_records(session, x)

unsigned gnutls_record_can_use_length_hiding(gnutls_session_t session);

int gnutls_record_get_direction(gnutls_session_t session);

size_t gnutls_record_get_max_size(gnutls_session_t session);
ssize_t gnutls_record_set_max_size(gnutls_session_t session, size_t size);
ssize_t gnutls_record_set_max_recv_size(gnutls_session_t session, size_t size);

size_t gnutls_record_check_pending(gnutls_session_t session);
size_t gnutls_record_check_corked(gnutls_session_t session);

size_t gnutls_record_get_max_early_data_size(gnutls_session_t session);
int gnutls_record_set_max_early_data_size(gnutls_session_t session, size_t size);
ssize_t gnutls_record_send_early_data(gnutls_session_t session,
				      const void *data,
				      size_t length);
ssize_t gnutls_record_recv_early_data(gnutls_session_t session,
				      void *data,
				      size_t data_size);

void gnutls_session_force_valid(gnutls_session_t session);

int gnutls_prf(gnutls_session_t session,
	       size_t label_size, const char *label,
	       int server_random_first,
	       size_t extra_size, const char *extra,
	       size_t outsize, char *out);
int gnutls_prf_rfc5705(gnutls_session_t session,
	       size_t label_size, const char *label,
	       size_t context_size, const char *context,
	       size_t outsize, char *out);
int gnutls_prf_early(gnutls_session_t session,
		     size_t label_size, const char *label,
		     size_t context_size, const char *context,
		     size_t outsize, char *out);

int gnutls_prf_raw(gnutls_session_t session,
		   size_t label_size, const char *label,
		   size_t seed_size, const char *seed,
		   size_t outsize, char *out);

/**
 * gnutls_server_name_type_t:
 * @GNUTLS_NAME_DNS: Domain Name System name type.
 *
 * Enumeration of different server name types.
 */
typedef enum {
	GNUTLS_NAME_DNS = 1
} gnutls_server_name_type_t;

int gnutls_server_name_set(gnutls_session_t session,
			   gnutls_server_name_type_t type,
			   const void *name, size_t name_length);

int gnutls_server_name_get(gnutls_session_t session,
			   void *data, size_t * data_length,
			   unsigned int *type, unsigned int indx);

unsigned int gnutls_heartbeat_get_timeout(gnutls_session_t session);
void gnutls_heartbeat_set_timeouts(gnutls_session_t session,
				   unsigned int retrans_timeout,
				   unsigned int total_timeout);

#define GNUTLS_HB_PEER_ALLOWED_TO_SEND (1)
#define GNUTLS_HB_PEER_NOT_ALLOWED_TO_SEND (1<<1)

  /* Heartbeat */
void gnutls_heartbeat_enable(gnutls_session_t session, unsigned int type);

#define GNUTLS_HB_LOCAL_ALLOWED_TO_SEND (1<<2)
unsigned gnutls_heartbeat_allowed(gnutls_session_t session, unsigned int type);

  /* Safe renegotiation */
unsigned gnutls_safe_renegotiation_status(gnutls_session_t session);
unsigned gnutls_session_ext_master_secret_status(gnutls_session_t session);
unsigned gnutls_session_etm_status(gnutls_session_t session);

/**
 * gnutls_session_flags_t:
 * @GNUTLS_SFLAGS_SAFE_RENEGOTIATION: Safe renegotiation (RFC5746) was used
 * @GNUTLS_SFLAGS_EXT_MASTER_SECRET: The extended master secret (RFC7627) extension was used
 * @GNUTLS_SFLAGS_ETM: The encrypt then MAC (RFC7366) extension was used
 * @GNUTLS_SFLAGS_RFC7919: The RFC7919 Diffie-Hellman parameters were negotiated
 * @GNUTLS_SFLAGS_HB_LOCAL_SEND: The heartbeat negotiation allows the local side to send heartbeat messages
 * @GNUTLS_SFLAGS_HB_PEER_SEND: The heartbeat negotiation allows the peer to send heartbeat messages
 * @GNUTLS_SFLAGS_FALSE_START: False start was used in this client session.
 * @GNUTLS_SFLAGS_SESSION_TICKET: A session ticket has been received by the server.
 * @GNUTLS_SFLAGS_POST_HANDSHAKE_AUTH: Indicates client capability for post-handshake auth; set only on server side.
 * @GNUTLS_SFLAGS_EARLY_START: The TLS1.3 server session returned early.
 * @GNUTLS_SFLAGS_EARLY_DATA: The TLS1.3 early data has been received by the server.
 * @GNUTLS_SFLAGS_CLI_REQUESTED_OCSP: Set when the client has requested OCSP staple during handshake.
 * @GNUTLS_SFLAGS_SERV_REQUESTED_OCSP: Set when the server has requested OCSP staple during handshake.
 *
 * Enumeration of different session parameters.
 */
typedef enum {
	GNUTLS_SFLAGS_SAFE_RENEGOTIATION = 1,
	GNUTLS_SFLAGS_EXT_MASTER_SECRET = 1<<1,
	GNUTLS_SFLAGS_ETM = 1<<2,
	GNUTLS_SFLAGS_HB_LOCAL_SEND = 1<<3,
	GNUTLS_SFLAGS_HB_PEER_SEND = 1<<4,
	GNUTLS_SFLAGS_FALSE_START = 1<<5,
	GNUTLS_SFLAGS_RFC7919 = 1<<6,
	GNUTLS_SFLAGS_SESSION_TICKET = 1<<7,
	GNUTLS_SFLAGS_POST_HANDSHAKE_AUTH = 1<<8,
	GNUTLS_SFLAGS_EARLY_START = 1<<9,
	GNUTLS_SFLAGS_EARLY_DATA = 1<<10,
	GNUTLS_SFLAGS_CLI_REQUESTED_OCSP = 1<<11,
	GNUTLS_SFLAGS_SERV_REQUESTED_OCSP = 1<<12
} gnutls_session_flags_t;

unsigned gnutls_session_get_flags(gnutls_session_t session);

/**
 * gnutls_supplemental_data_format_type_t:
 * @GNUTLS_SUPPLEMENTAL_UNKNOWN: Unknown data format
 *
 * Enumeration of different supplemental data types (RFC 4680).
 */
typedef enum {
	GNUTLS_SUPPLEMENTAL_UNKNOWN = 0,
} gnutls_supplemental_data_format_type_t;

const char
*gnutls_supplemental_get_name(gnutls_supplemental_data_format_type_t type);

  /* SessionTicket, RFC 5077. */
int gnutls_session_ticket_key_generate(gnutls_datum_t * key);
int gnutls_session_ticket_enable_client(gnutls_session_t session);
int gnutls_session_ticket_enable_server(gnutls_session_t session,
					const gnutls_datum_t * key);

int gnutls_session_ticket_send(gnutls_session_t session, unsigned nr, unsigned flags);

  /* SRTP, RFC 5764 */

/**
 * gnutls_srtp_profile_t:
 * @GNUTLS_SRTP_AES128_CM_HMAC_SHA1_80: 128 bit AES with a 80 bit HMAC-SHA1
 * @GNUTLS_SRTP_AES128_CM_HMAC_SHA1_32: 128 bit AES with a 32 bit HMAC-SHA1
 * @GNUTLS_SRTP_NULL_HMAC_SHA1_80: NULL cipher with a 80 bit HMAC-SHA1
 * @GNUTLS_SRTP_NULL_HMAC_SHA1_32: NULL cipher with a 32 bit HMAC-SHA1
 *
 * Enumeration of different SRTP protection profiles.
 */
typedef enum {
	GNUTLS_SRTP_AES128_CM_HMAC_SHA1_80 = 0x0001,
	GNUTLS_SRTP_AES128_CM_HMAC_SHA1_32 = 0x0002,
	GNUTLS_SRTP_NULL_HMAC_SHA1_80 = 0x0005,
	GNUTLS_SRTP_NULL_HMAC_SHA1_32 = 0x0006
} gnutls_srtp_profile_t;

int gnutls_srtp_set_profile(gnutls_session_t session,
			    gnutls_srtp_profile_t profile);
int gnutls_srtp_set_profile_direct(gnutls_session_t session,
				   const char *profiles,
				   const char **err_pos);
int gnutls_srtp_get_selected_profile(gnutls_session_t session,
				     gnutls_srtp_profile_t * profile);

const char *gnutls_srtp_get_profile_name(gnutls_srtp_profile_t profile);
int gnutls_srtp_get_profile_id(const char *name,
			       gnutls_srtp_profile_t * profile);
int gnutls_srtp_get_keys(gnutls_session_t session,
			 void *key_material,
			 unsigned int key_material_size,
			 gnutls_datum_t * client_key,
			 gnutls_datum_t * client_salt,
			 gnutls_datum_t * server_key,
			 gnutls_datum_t * server_salt);

int gnutls_srtp_set_mki(gnutls_session_t session,
			const gnutls_datum_t * mki);
int gnutls_srtp_get_mki(gnutls_session_t session, gnutls_datum_t * mki);

/* ALPN TLS extension */

/**
 * gnutls_alpn_flags_t:
 * @GNUTLS_ALPN_MANDATORY: Require ALPN negotiation. The connection will be
 *   aborted if no matching ALPN protocol is found.
 * @GNUTLS_ALPN_SERVER_PRECEDENCE: The choices set by the server
 *   will take precedence over the client's.
 *
 * Enumeration of different ALPN flags. These are used by gnutls_alpn_set_protocols().
 */
typedef enum {
	GNUTLS_ALPN_MANDATORY = 1,
	GNUTLS_ALPN_SERVER_PRECEDENCE = (1<<1)
} gnutls_alpn_flags_t;

#define GNUTLS_ALPN_MAND GNUTLS_ALPN_MANDATORY
int gnutls_alpn_get_selected_protocol(gnutls_session_t session,
				      gnutls_datum_t * protocol);
int gnutls_alpn_set_protocols(gnutls_session_t session,
			      const gnutls_datum_t * protocols,
			      unsigned protocols_size, unsigned flags);

int gnutls_key_generate(gnutls_datum_t * key, unsigned int key_size);


#define GNUTLS_PRIORITY_INIT_DEF_APPEND 1
int gnutls_priority_init(gnutls_priority_t * priority_cache,
			 const char *priorities, const char **err_pos);
int gnutls_priority_init2(gnutls_priority_t * priority_cache,
			  const char *priorities, const char **err_pos,
			  unsigned flags);
void gnutls_priority_deinit(gnutls_priority_t priority_cache);
int gnutls_priority_get_cipher_suite_index(gnutls_priority_t pcache,
					   unsigned int idx,
					   unsigned int *sidx);

#define GNUTLS_PRIORITY_LIST_INIT_KEYWORDS 1
#define GNUTLS_PRIORITY_LIST_SPECIAL 2
const char *
gnutls_priority_string_list(unsigned iter, unsigned int flags);

int gnutls_priority_set(gnutls_session_t session,
			gnutls_priority_t priority);

int gnutls_priority_set_direct(gnutls_session_t session,
			       const char *priorities,
			       const char **err_pos);

int gnutls_priority_certificate_type_list(gnutls_priority_t pcache,
					  const unsigned int **list);
int gnutls_priority_certificate_type_list2(gnutls_priority_t pcache,
					  const unsigned int **list,
					  gnutls_ctype_target_t target);
int gnutls_priority_sign_list(gnutls_priority_t pcache,
			      const unsigned int **list);
int gnutls_priority_protocol_list(gnutls_priority_t pcache,
				  const unsigned int **list);
int gnutls_priority_ecc_curve_list(gnutls_priority_t pcache,
				   const unsigned int **list);
int
gnutls_priority_group_list(gnutls_priority_t pcache,
			   const unsigned int **list);

int gnutls_priority_kx_list(gnutls_priority_t pcache,
			    const unsigned int **list);
int gnutls_priority_cipher_list(gnutls_priority_t pcache,
				const unsigned int **list);
int gnutls_priority_mac_list(gnutls_priority_t pcache,
			     const unsigned int **list);

const char *gnutls_get_system_config_file(void);

int gnutls_set_default_priority(gnutls_session_t session);
int gnutls_set_default_priority_append(gnutls_session_t session,
				       const char *add_prio,
				       const char **err_pos,
				       unsigned flags);

/* Returns the name of a cipher suite */
const char *
	gnutls_cipher_suite_get_name(gnutls_kx_algorithm_t kx_algorithm,
				     gnutls_cipher_algorithm_t cipher_algorithm,
				     gnutls_mac_algorithm_t mac_algorithm) __GNUTLS_CONST__;

/* get the currently used protocol version */
gnutls_protocol_t gnutls_protocol_get_version(gnutls_session_t session);

const char *
	gnutls_protocol_get_name(gnutls_protocol_t version) __GNUTLS_CONST__;


/* get/set session
 */
int gnutls_session_set_data(gnutls_session_t session,
			    const void *session_data,
			    size_t session_data_size);
int gnutls_session_get_data(gnutls_session_t session, void *session_data,
			    size_t * session_data_size);
int gnutls_session_get_data2(gnutls_session_t session,
			     gnutls_datum_t * data);
void gnutls_session_get_random(gnutls_session_t session,
			       gnutls_datum_t * client,
			       gnutls_datum_t * server);

void gnutls_session_get_master_secret(gnutls_session_t session,
			              gnutls_datum_t * secret);

char *gnutls_session_get_desc(gnutls_session_t session);

typedef int gnutls_certificate_verify_function(gnutls_session_t);
void gnutls_session_set_verify_function(gnutls_session_t session, gnutls_certificate_verify_function * func);

/**
 * gnutls_vdata_types_t:
 * @GNUTLS_DT_UNKNOWN: Unknown data type.
 * @GNUTLS_DT_DNS_HOSTNAME: The data contain a null-terminated DNS hostname; the hostname will be 
 *   matched using the RFC6125 rules. If the data contain a textual IP (v4 or v6) address it will
 *   be marched against the IPAddress Alternative name, unless the verification flag %GNUTLS_VERIFY_DO_NOT_ALLOW_IP_MATCHES
 *   is specified.
 * @GNUTLS_DT_IP_ADDRESS: The data contain a raw IP address (4 or 16 bytes). If will be matched
 *   against the IPAddress Alternative name; option available since 3.6.0.
 * @GNUTLS_DT_RFC822NAME: The data contain a null-terminated email address; the email will be
 *   matched against the RFC822Name Alternative name of the certificate, or the EMAIL DN component if the
 *   former isn't available. Prior to matching the email address will be converted to ACE
 *   (ASCII-compatible-encoding).
 * @GNUTLS_DT_KEY_PURPOSE_OID: The data contain a null-terminated key purpose OID. It will be matched
 *   against the certificate's Extended Key Usage extension.
 *
 * Enumeration of different typed-data options. They are used as input to certificate
 * verification functions to provide information about the name and purpose of the
 * certificate. Only a single option of a type can be provided to the relevant functions
 * (i.e., options %GNUTLS_DT_DNS_HOSTNAME, %GNUTLS_DT_IP_ADDRESS and
 * %GNUTLS_DT_RFC822NAME cannot be combined).
 */
typedef enum {
	GNUTLS_DT_UNKNOWN = 0,
	GNUTLS_DT_DNS_HOSTNAME = 1,
	GNUTLS_DT_KEY_PURPOSE_OID = 2,
	GNUTLS_DT_RFC822NAME = 3,
	GNUTLS_DT_IP_ADDRESS = 4
} gnutls_vdata_types_t;

typedef struct {
	gnutls_vdata_types_t type;
	unsigned char *data;
	unsigned int size;
} gnutls_typed_vdata_st;

void gnutls_session_set_verify_cert(gnutls_session_t session,
			       const char *hostname, unsigned flags);

void
gnutls_session_set_verify_cert2(gnutls_session_t session,
				gnutls_typed_vdata_st * data,
				unsigned elements, unsigned flags);

unsigned int gnutls_session_get_verify_cert_status(gnutls_session_t);

int gnutls_session_set_premaster(gnutls_session_t session,
				 unsigned int entity,
				 gnutls_protocol_t version,
				 gnutls_kx_algorithm_t kx,
				 gnutls_cipher_algorithm_t cipher,
				 gnutls_mac_algorithm_t mac,
				 gnutls_compression_method_t comp,
				 const gnutls_datum_t * master,
				 const gnutls_datum_t * session_id);

/* returns the session ID */
#define GNUTLS_MAX_SESSION_ID 32
int gnutls_session_get_id(gnutls_session_t session, void *session_id,
			  size_t * session_id_size);
int gnutls_session_get_id2(gnutls_session_t session,
			   gnutls_datum_t * session_id);

int gnutls_session_set_id(gnutls_session_t session,
			  const gnutls_datum_t * sid);

int gnutls_session_channel_binding(gnutls_session_t session,
				   gnutls_channel_binding_t cbtype,
				   gnutls_datum_t * cb);

/* checks if this session is a resumed one
 */
int gnutls_session_is_resumed(gnutls_session_t session);
int gnutls_session_resumption_requested(gnutls_session_t session);

typedef int (*gnutls_db_store_func) (void *, gnutls_datum_t key,
				     gnutls_datum_t data);
typedef int (*gnutls_db_remove_func) (void *, gnutls_datum_t key);
typedef gnutls_datum_t(*gnutls_db_retr_func) (void *, gnutls_datum_t key);

void gnutls_db_set_cache_expiration(gnutls_session_t session, int seconds);
unsigned gnutls_db_get_default_cache_expiration(void);

void gnutls_db_remove_session(gnutls_session_t session);
void gnutls_db_set_retrieve_function(gnutls_session_t session,
				     gnutls_db_retr_func retr_func);
void gnutls_db_set_remove_function(gnutls_session_t session,
				   gnutls_db_remove_func rem_func);
void gnutls_db_set_store_function(gnutls_session_t session,
				  gnutls_db_store_func store_func);
void gnutls_db_set_ptr(gnutls_session_t session, void *ptr);
void *gnutls_db_get_ptr(gnutls_session_t session);
int gnutls_db_check_entry(gnutls_session_t session,
			  gnutls_datum_t session_entry);
time_t gnutls_db_check_entry_time(gnutls_datum_t * entry);
time_t gnutls_db_check_entry_expire_time(gnutls_datum_t * entry);

  /**
   * gnutls_handshake_hook_func:
   * @session: the current session
   * @htype: the type of the handshake message (%gnutls_handshake_description_t)
   * @when: non zero if this is a post-process/generation call and zero otherwise
   * @incoming: non zero if this is an incoming message and zero if this is an outgoing message
   * @msg: the (const) data of the handshake message without the handshake headers.
   *
   * Function prototype for handshake hooks. It is set using
   * gnutls_handshake_set_hook_function().
   *
   * Returns: Non zero on error.
   */
#define GNUTLS_HOOK_POST (1)
#define GNUTLS_HOOK_PRE (0)
#define GNUTLS_HOOK_BOTH (-1)

typedef int (*gnutls_handshake_hook_func) (gnutls_session_t,
					   unsigned int htype,
					   unsigned when,
					   unsigned int incoming,
					   const gnutls_datum_t *msg);
void gnutls_handshake_set_hook_function(gnutls_session_t session,
					unsigned int htype, int when,
					gnutls_handshake_hook_func func);

#define gnutls_handshake_post_client_hello_func gnutls_handshake_simple_hook_func
typedef int (*gnutls_handshake_simple_hook_func) (gnutls_session_t);
void
gnutls_handshake_set_post_client_hello_function(gnutls_session_t session,
						gnutls_handshake_simple_hook_func func);

void gnutls_handshake_set_max_packet_length(gnutls_session_t session,
					    size_t max);

/* returns libgnutls version (call it with a NULL argument)
 */
const char * gnutls_check_version(const char *req_version) __GNUTLS_CONST__;

/* A macro which will allow optimizing out calls to gnutls_check_version()
 * when the version being compiled with is sufficient.
 * Used as:
 *   if (gnutls_check_version_numerc(3,3,16)) {
 */
#define gnutls_check_version_numeric(a,b,c) \
	((GNUTLS_VERSION_MAJOR >= (a)) &&  \
	 ((GNUTLS_VERSION_NUMBER >= ( ((a) << 16) + ((b) << 8) + (c) )) || \
	 gnutls_check_version(#a "." #b "." #c)))

/* Functions for setting/clearing credentials
 */
void gnutls_credentials_clear(gnutls_session_t session);

/* cred is a structure defined by the kx algorithm
 */
int gnutls_credentials_set(gnutls_session_t session,
			   gnutls_credentials_type_t type, void *cred);
int gnutls_credentials_get(gnutls_session_t session,
			   gnutls_credentials_type_t type, void **cred);
#define gnutls_cred_set	gnutls_credentials_set

/* x.509 types */

struct gnutls_pubkey_st;
typedef struct gnutls_pubkey_st *gnutls_pubkey_t;

struct gnutls_privkey_st;
typedef struct gnutls_privkey_st *gnutls_privkey_t;

struct gnutls_x509_privkey_int;
typedef struct gnutls_x509_privkey_int *gnutls_x509_privkey_t;

struct gnutls_x509_crl_int;
typedef struct gnutls_x509_crl_int *gnutls_x509_crl_t;

struct gnutls_x509_crt_int;
typedef struct gnutls_x509_crt_int *gnutls_x509_crt_t;

struct gnutls_x509_crq_int;
typedef struct gnutls_x509_crq_int *gnutls_x509_crq_t;

struct gnutls_openpgp_keyring_int;
typedef struct gnutls_openpgp_keyring_int *gnutls_openpgp_keyring_t;


/* Credential structures - used in gnutls_credentials_set(); */

struct gnutls_certificate_credentials_st;
typedef struct gnutls_certificate_credentials_st
*gnutls_certificate_credentials_t;
typedef gnutls_certificate_credentials_t
    gnutls_certificate_server_credentials;
typedef gnutls_certificate_credentials_t
    gnutls_certificate_client_credentials;

typedef struct gnutls_anon_server_credentials_st
*gnutls_anon_server_credentials_t;
typedef struct gnutls_anon_client_credentials_st
*gnutls_anon_client_credentials_t;

void gnutls_anon_free_server_credentials(gnutls_anon_server_credentials_t
					 sc);
int
gnutls_anon_allocate_server_credentials(gnutls_anon_server_credentials_t
					* sc);

void gnutls_anon_set_server_dh_params(gnutls_anon_server_credentials_t res,
				      gnutls_dh_params_t dh_params);

int
gnutls_anon_set_server_known_dh_params(gnutls_anon_server_credentials_t res,
					gnutls_sec_param_t sec_param);

void
gnutls_anon_set_server_params_function(gnutls_anon_server_credentials_t
				       res, gnutls_params_function * func);

void
gnutls_anon_free_client_credentials(gnutls_anon_client_credentials_t sc);
int
gnutls_anon_allocate_client_credentials(gnutls_anon_client_credentials_t
					* sc);

/* CERTFILE is an x509 certificate in PEM form.
 * KEYFILE is a pkcs-1 private key in PEM form (for RSA keys).
 */
void
gnutls_certificate_free_credentials(gnutls_certificate_credentials_t sc);
int
gnutls_certificate_allocate_credentials(gnutls_certificate_credentials_t
					* res);

int
gnutls_certificate_get_issuer(gnutls_certificate_credentials_t sc,
			      gnutls_x509_crt_t cert,
			      gnutls_x509_crt_t * issuer,
			      unsigned int flags);

int gnutls_certificate_get_crt_raw(gnutls_certificate_credentials_t sc,
				   unsigned idx1, unsigned idx2,
				   gnutls_datum_t * cert);

void gnutls_certificate_free_keys(gnutls_certificate_credentials_t sc);
void gnutls_certificate_free_cas(gnutls_certificate_credentials_t sc);
void gnutls_certificate_free_ca_names(gnutls_certificate_credentials_t sc);
void gnutls_certificate_free_crls(gnutls_certificate_credentials_t sc);

void gnutls_certificate_set_dh_params(gnutls_certificate_credentials_t res,
				      gnutls_dh_params_t dh_params);

int gnutls_certificate_set_known_dh_params(gnutls_certificate_credentials_t res,
					   gnutls_sec_param_t sec_param);
void gnutls_certificate_set_verify_flags(gnutls_certificate_credentials_t
					 res, unsigned int flags);
unsigned int
gnutls_certificate_get_verify_flags(gnutls_certificate_credentials_t res);

/**
 * gnutls_certificate_flags:
 * @GNUTLS_CERTIFICATE_SKIP_KEY_CERT_MATCH: Skip the key and certificate matching check.
 * @GNUTLS_CERTIFICATE_API_V2: If set the gnutls_certificate_set_*key* functions will return an index of the added key pair instead of zero.
 * @GNUTLS_CERTIFICATE_SKIP_OCSP_RESPONSE_CHECK: If set, the gnutls_certificate_set_ocsp_status_request_file
 *    function, will not check whether the response set matches any of the certificates.
 * @GNUTLS_CERTIFICATE_VERIFY_CRLS: This will enable CRL verification when added in the certificate structure.
 *    When used, it requires CAs to be added before CRLs.
 *
 * Enumeration of different certificate credentials flags.
 */
typedef enum gnutls_certificate_flags {
	GNUTLS_CERTIFICATE_SKIP_KEY_CERT_MATCH = 1,
	GNUTLS_CERTIFICATE_API_V2 = (1<<1),
	GNUTLS_CERTIFICATE_SKIP_OCSP_RESPONSE_CHECK = (1<<2),
	GNUTLS_CERTIFICATE_VERIFY_CRLS = (1<<3)
} gnutls_certificate_flags;

void gnutls_certificate_set_flags(gnutls_certificate_credentials_t,
				  unsigned flags);

void gnutls_certificate_set_verify_limits(gnutls_certificate_credentials_t
					  res, unsigned int max_bits,
					  unsigned int max_depth);

int
gnutls_certificate_set_x509_system_trust(gnutls_certificate_credentials_t
					 cred);

int
gnutls_certificate_set_x509_trust_file(gnutls_certificate_credentials_t
				       cred, const char *cafile,
				       gnutls_x509_crt_fmt_t type);
int
gnutls_certificate_set_x509_trust_dir(gnutls_certificate_credentials_t cred,
				      const char *ca_dir,
				      gnutls_x509_crt_fmt_t type);

int gnutls_certificate_set_x509_trust_mem(gnutls_certificate_credentials_t
					  res, const gnutls_datum_t * ca,
					  gnutls_x509_crt_fmt_t type);

int
gnutls_certificate_set_x509_crl_file(gnutls_certificate_credentials_t
				     res, const char *crlfile,
				     gnutls_x509_crt_fmt_t type);
int gnutls_certificate_set_x509_crl_mem(gnutls_certificate_credentials_t
					res, const gnutls_datum_t * CRL,
					gnutls_x509_crt_fmt_t type);

int
gnutls_certificate_set_x509_key_file(gnutls_certificate_credentials_t
				     res, const char *certfile,
				     const char *keyfile,
				     gnutls_x509_crt_fmt_t type);

int
gnutls_certificate_set_x509_key_file2(gnutls_certificate_credentials_t
				      res, const char *certfile,
				      const char *keyfile,
				      gnutls_x509_crt_fmt_t type,
				      const char *pass,
				      unsigned int flags);

int gnutls_certificate_set_x509_key_mem(gnutls_certificate_credentials_t
					res, const gnutls_datum_t * cert,
					const gnutls_datum_t * key,
					gnutls_x509_crt_fmt_t type);

int gnutls_certificate_set_x509_key_mem2(gnutls_certificate_credentials_t
					 res, const gnutls_datum_t * cert,
					 const gnutls_datum_t * key,
					 gnutls_x509_crt_fmt_t type,
					 const char *pass,
					 unsigned int flags);

void gnutls_certificate_send_x509_rdn_sequence(gnutls_session_t session,
					       int status);

int
gnutls_certificate_set_x509_simple_pkcs12_file
(gnutls_certificate_credentials_t res, const char *pkcs12file,
 gnutls_x509_crt_fmt_t type, const char *password);
int
gnutls_certificate_set_x509_simple_pkcs12_mem
(gnutls_certificate_credentials_t res, const gnutls_datum_t * p12blob,
 gnutls_x509_crt_fmt_t type, const char *password);

/* New functions to allow setting already parsed X.509 stuff.
 */

int gnutls_certificate_set_x509_key(gnutls_certificate_credentials_t res,
				    gnutls_x509_crt_t * cert_list,
				    int cert_list_size,
				    gnutls_x509_privkey_t key);
int gnutls_certificate_set_x509_trust(gnutls_certificate_credentials_t res,
				      gnutls_x509_crt_t * ca_list,
				      int ca_list_size);
int gnutls_certificate_set_x509_crl(gnutls_certificate_credentials_t res,
				    gnutls_x509_crl_t * crl_list,
				    int crl_list_size);

int gnutls_certificate_get_x509_key(gnutls_certificate_credentials_t res,
                                    unsigned index,
                                    gnutls_x509_privkey_t *key);
int gnutls_certificate_get_x509_crt(gnutls_certificate_credentials_t res,
                                    unsigned index,
                                    gnutls_x509_crt_t **crt_list,
                                    unsigned *crt_list_size);

  /* OCSP status request extension, RFC 6066 */
typedef int (*gnutls_status_request_ocsp_func)
 (gnutls_session_t session, void *ptr, gnutls_datum_t *ocsp_response);

void
gnutls_certificate_set_ocsp_status_request_function
(gnutls_certificate_credentials_t res,
gnutls_status_request_ocsp_func ocsp_func, void *ptr);

int
gnutls_certificate_set_ocsp_status_request_function2
(gnutls_certificate_credentials_t res, unsigned idx,
gnutls_status_request_ocsp_func ocsp_func, void *ptr);

int
gnutls_certificate_set_ocsp_status_request_file
(gnutls_certificate_credentials_t res, const char *response_file,
 unsigned idx);

int
gnutls_certificate_set_ocsp_status_request_file2
(gnutls_certificate_credentials_t res, const char *response_file,
 unsigned idx, gnutls_x509_crt_fmt_t fmt);

int
gnutls_certificate_set_ocsp_status_request_mem
(gnutls_certificate_credentials_t res, const gnutls_datum_t *resp,
 unsigned idx, gnutls_x509_crt_fmt_t fmt);

typedef struct gnutls_ocsp_data_st {
	unsigned int version; /* must be zero */
	gnutls_datum_t response;
	time_t exptime;
	unsigned char padding[32];
} gnutls_ocsp_data_st;

time_t
gnutls_certificate_get_ocsp_expiration(gnutls_certificate_credentials_t sc,
				       unsigned idx,
				       int oidx,
				       unsigned flags);

int gnutls_ocsp_status_request_enable_client(gnutls_session_t session,
					     gnutls_datum_t * responder_id,
					     size_t responder_id_size,
					     gnutls_datum_t *
					     request_extensions);

int gnutls_ocsp_status_request_get(gnutls_session_t session,
				   gnutls_datum_t * response);

#define GNUTLS_OCSP_SR_IS_AVAIL 1
unsigned gnutls_ocsp_status_request_is_checked(gnutls_session_t session,
					       unsigned int flags);

int
gnutls_ocsp_status_request_get2(gnutls_session_t session,
			        unsigned idx,
			        gnutls_datum_t * response);

/* RAW public key functions (RFC7250) */
int gnutls_certificate_set_rawpk_key_mem(gnutls_certificate_credentials_t cred,
				    const gnutls_datum_t* spki,
				    const gnutls_datum_t* pkey,
				    gnutls_x509_crt_fmt_t format,
				    const char* pass,
				    unsigned int key_usage,
				    const char **names,
				    unsigned int names_length,
				    unsigned int flags);

int gnutls_certificate_set_rawpk_key_file(gnutls_certificate_credentials_t cred,
				      const char* rawpkfile,
				      const char* privkeyfile,
				      gnutls_x509_crt_fmt_t format,
				      const char *pass,
				      unsigned int key_usage,
				      const char **names,
				      unsigned int names_length,
				      unsigned int privkey_flags,
				      unsigned int pkcs11_flags);


/* global state functions
 */
int gnutls_global_init(void);
void gnutls_global_deinit(void);

  /**
   * gnutls_time_func:
   * @t: where to store time.
   *
   * Function prototype for time()-like function.  Set with
   * gnutls_global_set_time_function().
   *
   * Returns: Number of seconds since the epoch, or (time_t)-1 on errors.
   */
typedef time_t(*gnutls_time_func) (time_t * t);

typedef int (*mutex_init_func) (void **mutex);
typedef int (*mutex_lock_func) (void **mutex);
typedef int (*mutex_unlock_func) (void **mutex);
typedef int (*mutex_deinit_func) (void **mutex);

void gnutls_global_set_mutex(mutex_init_func init,
			     mutex_deinit_func deinit,
			     mutex_lock_func lock,
			     mutex_unlock_func unlock);

typedef void *(*gnutls_alloc_function) (size_t);
typedef void *(*gnutls_calloc_function) (size_t, size_t);
typedef int (*gnutls_is_secure_function) (const void *);
typedef void (*gnutls_free_function) (void *);
typedef void *(*gnutls_realloc_function) (void *, size_t);

void gnutls_global_set_time_function(gnutls_time_func time_func);

/* For use in callbacks */
extern _SYM_EXPORT gnutls_alloc_function gnutls_malloc;
extern _SYM_EXPORT gnutls_realloc_function gnutls_realloc;
extern _SYM_EXPORT gnutls_calloc_function gnutls_calloc;
extern _SYM_EXPORT gnutls_free_function gnutls_free;

#ifdef GNUTLS_INTERNAL_BUILD
#define gnutls_free(a) gnutls_free((void *) (a)), a=NULL
#endif

extern _SYM_EXPORT char *(*gnutls_strdup) (const char *);

/* a variant of memset that doesn't get optimized out */
void gnutls_memset(void *data, int c, size_t size);

/* constant time memcmp */
int gnutls_memcmp(const void *s1, const void *s2, size_t n);

typedef void (*gnutls_log_func) (int, const char *);
typedef void (*gnutls_audit_log_func) (gnutls_session_t, const char *);
void gnutls_global_set_log_function(gnutls_log_func log_func);
void gnutls_global_set_audit_log_function(gnutls_audit_log_func log_func);
void gnutls_global_set_log_level(int level);

  /**
   * gnutls_keylog_func:
   * @session: the current session
   * @label: the keylog label
   * @secret: the (const) data of the derived secret.
   *
   * Function prototype for keylog hooks. It is set using
   * gnutls_session_set_keylog_function().
   *
   * Returns: Non zero on error.
   * Since: 3.6.13
   */
typedef int (*gnutls_keylog_func) (gnutls_session_t session,
				   const char *label,
				   const gnutls_datum_t *secret);
gnutls_keylog_func gnutls_session_get_keylog_function(const gnutls_session_t session);
void gnutls_session_set_keylog_function(gnutls_session_t session,
					gnutls_keylog_func func);

/* Diffie-Hellman parameter handling.
 */
int gnutls_dh_params_init(gnutls_dh_params_t * dh_params);
void gnutls_dh_params_deinit(gnutls_dh_params_t dh_params);
int gnutls_dh_params_import_raw(gnutls_dh_params_t dh_params,
				const gnutls_datum_t * prime,
				const gnutls_datum_t * generator);
int gnutls_dh_params_import_dsa(gnutls_dh_params_t dh_params, gnutls_x509_privkey_t key);
int gnutls_dh_params_import_raw2(gnutls_dh_params_t dh_params,
				 const gnutls_datum_t * prime,
				 const gnutls_datum_t * generator,
				 unsigned key_bits);
int gnutls_dh_params_import_raw3(gnutls_dh_params_t dh_params,
				 const gnutls_datum_t * prime,
				 const gnutls_datum_t * q,
				 const gnutls_datum_t * generator);
int gnutls_dh_params_import_pkcs3(gnutls_dh_params_t params,
				  const gnutls_datum_t * pkcs3_params,
				  gnutls_x509_crt_fmt_t format);
int gnutls_dh_params_generate2(gnutls_dh_params_t params,
			       unsigned int bits);
int gnutls_dh_params_export_pkcs3(gnutls_dh_params_t params,
				  gnutls_x509_crt_fmt_t format,
				  unsigned char *params_data,
				  size_t * params_data_size);
int gnutls_dh_params_export2_pkcs3(gnutls_dh_params_t params,
				   gnutls_x509_crt_fmt_t format,
				   gnutls_datum_t * out);
int gnutls_dh_params_export_raw(gnutls_dh_params_t params,
				gnutls_datum_t * prime,
				gnutls_datum_t * generator,
				unsigned int *bits);
int gnutls_dh_params_cpy(gnutls_dh_params_t dst, gnutls_dh_params_t src);



/* Session stuff
 */
#include <sys/uio.h>
typedef struct iovec giovec_t;

typedef ssize_t(*gnutls_pull_func) (gnutls_transport_ptr_t, void *,
				    size_t);
typedef ssize_t(*gnutls_push_func) (gnutls_transport_ptr_t, const void *,
				    size_t);

int gnutls_system_recv_timeout(gnutls_transport_ptr_t ptr, unsigned int ms);
typedef int (*gnutls_pull_timeout_func) (gnutls_transport_ptr_t,
					 unsigned int ms);

typedef ssize_t(*gnutls_vec_push_func) (gnutls_transport_ptr_t,
					const giovec_t * iov, int iovcnt);

typedef int (*gnutls_errno_func) (gnutls_transport_ptr_t);

#if 0
 /* This will be defined as macro. */
  void gnutls_transport_set_int (gnutls_session_t session, int r);
#endif

void gnutls_transport_set_int2(gnutls_session_t session, int r, int s);
#define gnutls_transport_set_int(s, i) gnutls_transport_set_int2(s, i, i)

void gnutls_transport_get_int2(gnutls_session_t session, int *r, int *s);
int gnutls_transport_get_int(gnutls_session_t session);

void gnutls_transport_set_ptr(gnutls_session_t session,
			      gnutls_transport_ptr_t ptr);
void gnutls_transport_set_ptr2(gnutls_session_t session,
			       gnutls_transport_ptr_t recv_ptr,
			       gnutls_transport_ptr_t send_ptr);

gnutls_transport_ptr_t gnutls_transport_get_ptr(gnutls_session_t session);
void gnutls_transport_get_ptr2(gnutls_session_t session,
			       gnutls_transport_ptr_t * recv_ptr,
			       gnutls_transport_ptr_t * send_ptr);

void gnutls_transport_set_vec_push_function(gnutls_session_t session,
					    gnutls_vec_push_func vec_func);
void gnutls_transport_set_push_function(gnutls_session_t session,
					gnutls_push_func push_func);
void gnutls_transport_set_pull_function(gnutls_session_t session,
					gnutls_pull_func pull_func);

void gnutls_transport_set_pull_timeout_function(gnutls_session_t session,
						gnutls_pull_timeout_func
						func);

void gnutls_transport_set_errno_function(gnutls_session_t session,
					 gnutls_errno_func errno_func);

void gnutls_transport_set_errno(gnutls_session_t session, int err);

/* session specific
 */
void gnutls_session_set_ptr(gnutls_session_t session, void *ptr);
void *gnutls_session_get_ptr(gnutls_session_t session);

void gnutls_openpgp_send_cert(gnutls_session_t session,
			      gnutls_openpgp_crt_status_t status);

/* This function returns the hash of the given data.
 */
int gnutls_fingerprint(gnutls_digest_algorithm_t algo,
		       const gnutls_datum_t * data, void *result,
		       size_t * result_size);

  /**
   * gnutls_random_art_t:
   * @GNUTLS_RANDOM_ART_OPENSSH: OpenSSH-style random art.
   *
   * Enumeration of different random art types.
   */
typedef enum gnutls_random_art {
	GNUTLS_RANDOM_ART_OPENSSH = 1
} gnutls_random_art_t;

int gnutls_random_art(gnutls_random_art_t type,
		      const char *key_type, unsigned int key_size,
		      void *fpr, size_t fpr_size, gnutls_datum_t * art);

/* IDNA */
#define GNUTLS_IDNA_FORCE_2008 (1<<1)
int gnutls_idna_map(const char * input, unsigned ilen, gnutls_datum_t *out, unsigned flags);
int gnutls_idna_reverse_map(const char *input, unsigned ilen, gnutls_datum_t *out, unsigned flags);

/* SRP
 */

typedef struct gnutls_srp_server_credentials_st
*gnutls_srp_server_credentials_t;
typedef struct gnutls_srp_client_credentials_st
*gnutls_srp_client_credentials_t;

void
gnutls_srp_free_client_credentials(gnutls_srp_client_credentials_t sc);
int
gnutls_srp_allocate_client_credentials(gnutls_srp_client_credentials_t *
				       sc);
int gnutls_srp_set_client_credentials(gnutls_srp_client_credentials_t res,
				      const char *username,
				      const char *password);

void
gnutls_srp_free_server_credentials(gnutls_srp_server_credentials_t sc);
int
gnutls_srp_allocate_server_credentials(gnutls_srp_server_credentials_t *
				       sc);
int gnutls_srp_set_server_credentials_file(gnutls_srp_server_credentials_t
					   res, const char *password_file,
					   const char *password_conf_file);

const char *gnutls_srp_server_get_username(gnutls_session_t session);

void gnutls_srp_set_prime_bits(gnutls_session_t session,
                               unsigned int bits);

int gnutls_srp_verifier(const char *username,
			const char *password,
			const gnutls_datum_t * salt,
			const gnutls_datum_t * generator,
			const gnutls_datum_t * prime,
			gnutls_datum_t * res);

/* The static parameters defined in draft-ietf-tls-srp-05
 * Those should be used as input to gnutls_srp_verifier().
 */
extern _SYM_EXPORT const gnutls_datum_t gnutls_srp_8192_group_prime;
extern _SYM_EXPORT const gnutls_datum_t gnutls_srp_8192_group_generator;

extern _SYM_EXPORT const gnutls_datum_t gnutls_srp_4096_group_prime;
extern _SYM_EXPORT const gnutls_datum_t gnutls_srp_4096_group_generator;

extern _SYM_EXPORT const gnutls_datum_t gnutls_srp_3072_group_prime;
extern _SYM_EXPORT const gnutls_datum_t gnutls_srp_3072_group_generator;

extern _SYM_EXPORT const gnutls_datum_t gnutls_srp_2048_group_prime;
extern _SYM_EXPORT const gnutls_datum_t gnutls_srp_2048_group_generator;

extern _SYM_EXPORT const gnutls_datum_t gnutls_srp_1536_group_prime;
extern _SYM_EXPORT const gnutls_datum_t gnutls_srp_1536_group_generator;

extern _SYM_EXPORT const gnutls_datum_t gnutls_srp_1024_group_prime;
extern _SYM_EXPORT const gnutls_datum_t gnutls_srp_1024_group_generator;

/* The static parameters defined in rfc7919
 */

extern _SYM_EXPORT const gnutls_datum_t gnutls_ffdhe_8192_group_prime;
extern _SYM_EXPORT const gnutls_datum_t gnutls_ffdhe_8192_group_q;
extern _SYM_EXPORT const gnutls_datum_t gnutls_ffdhe_8192_group_generator;
extern _SYM_EXPORT const unsigned int gnutls_ffdhe_8192_key_bits;

extern _SYM_EXPORT const gnutls_datum_t gnutls_ffdhe_6144_group_prime;
extern _SYM_EXPORT const gnutls_datum_t gnutls_ffdhe_6144_group_q;
extern _SYM_EXPORT const gnutls_datum_t gnutls_ffdhe_6144_group_generator;
extern _SYM_EXPORT const unsigned int gnutls_ffdhe_6144_key_bits;

extern _SYM_EXPORT const gnutls_datum_t gnutls_ffdhe_4096_group_prime;
extern _SYM_EXPORT const gnutls_datum_t gnutls_ffdhe_4096_group_q;
extern _SYM_EXPORT const gnutls_datum_t gnutls_ffdhe_4096_group_generator;
extern _SYM_EXPORT const unsigned int gnutls_ffdhe_4096_key_bits;

extern _SYM_EXPORT const gnutls_datum_t gnutls_ffdhe_3072_group_prime;
extern _SYM_EXPORT const gnutls_datum_t gnutls_ffdhe_3072_group_q;
extern _SYM_EXPORT const gnutls_datum_t gnutls_ffdhe_3072_group_generator;
extern _SYM_EXPORT const unsigned int gnutls_ffdhe_3072_key_bits;

extern _SYM_EXPORT const gnutls_datum_t gnutls_ffdhe_2048_group_prime;
extern _SYM_EXPORT const gnutls_datum_t gnutls_ffdhe_2048_group_q;
extern _SYM_EXPORT const gnutls_datum_t gnutls_ffdhe_2048_group_generator;
extern _SYM_EXPORT const unsigned int gnutls_ffdhe_2048_key_bits;

typedef int gnutls_srp_server_credentials_function(gnutls_session_t,
						   const char *username,
						   gnutls_datum_t * salt,
						   gnutls_datum_t *
						   verifier,
						   gnutls_datum_t *
						   generator,
						   gnutls_datum_t * prime);
void
gnutls_srp_set_server_credentials_function(gnutls_srp_server_credentials_t
					   cred,
					   gnutls_srp_server_credentials_function
					   * func);

typedef int gnutls_srp_client_credentials_function(gnutls_session_t,
						   char **, char **);
void
gnutls_srp_set_client_credentials_function(gnutls_srp_client_credentials_t
					   cred,
					   gnutls_srp_client_credentials_function
					   * func);

int gnutls_srp_base64_encode(const gnutls_datum_t * data, char *result,
			     size_t * result_size);
int gnutls_srp_base64_encode2(const gnutls_datum_t * data,
				   gnutls_datum_t * result);

int gnutls_srp_base64_decode(const gnutls_datum_t * b64_data, char *result,
			     size_t * result_size);
int gnutls_srp_base64_decode2(const gnutls_datum_t * b64_data,
				   gnutls_datum_t * result);

#define gnutls_srp_base64_encode_alloc gnutls_srp_base64_encode2
#define gnutls_srp_base64_decode_alloc gnutls_srp_base64_decode2

void
gnutls_srp_set_server_fake_salt_seed(gnutls_srp_server_credentials_t
				     sc,
				     const gnutls_datum_t * seed,
				     unsigned int salt_length);

/* PSK stuff */
typedef struct gnutls_psk_server_credentials_st
*gnutls_psk_server_credentials_t;
typedef struct gnutls_psk_client_credentials_st
*gnutls_psk_client_credentials_t;

/**
 * gnutls_psk_key_flags:
 * @GNUTLS_PSK_KEY_RAW: PSK-key in raw format.
 * @GNUTLS_PSK_KEY_HEX: PSK-key in hex format.
 *
 * Enumeration of different PSK key flags.
 */
typedef enum gnutls_psk_key_flags {
	GNUTLS_PSK_KEY_RAW = 0,
	GNUTLS_PSK_KEY_HEX
} gnutls_psk_key_flags;

void
gnutls_psk_free_client_credentials(gnutls_psk_client_credentials_t sc);
int
gnutls_psk_allocate_client_credentials(gnutls_psk_client_credentials_t *
				       sc);
int gnutls_psk_set_client_credentials(gnutls_psk_client_credentials_t res,
				      const char *username,
				      const gnutls_datum_t * key,
				      gnutls_psk_key_flags flags);
int gnutls_psk_set_client_credentials2(gnutls_psk_client_credentials_t res,
				       const gnutls_datum_t *username,
				       const gnutls_datum_t *key,
				       gnutls_psk_key_flags flags);

void
gnutls_psk_free_server_credentials(gnutls_psk_server_credentials_t sc);
int
gnutls_psk_allocate_server_credentials(gnutls_psk_server_credentials_t *
				       sc);
int gnutls_psk_set_server_credentials_file(gnutls_psk_server_credentials_t
					   res, const char *password_file);

int
gnutls_psk_set_server_credentials_hint(gnutls_psk_server_credentials_t
				       res, const char *hint);

const char *gnutls_psk_server_get_username(gnutls_session_t session);
int gnutls_psk_server_get_username2(gnutls_session_t session,
				    gnutls_datum_t *out);
const char *gnutls_psk_client_get_hint(gnutls_session_t session);

typedef int gnutls_psk_server_credentials_function(gnutls_session_t,
						   const char *username,
						   gnutls_datum_t * key);
typedef int gnutls_psk_server_credentials_function2(gnutls_session_t,
						    const gnutls_datum_t *username,
						    gnutls_datum_t *key);
void
gnutls_psk_set_server_credentials_function(gnutls_psk_server_credentials_t
					   cred,
					   gnutls_psk_server_credentials_function
					   * func);
void
gnutls_psk_set_server_credentials_function2(gnutls_psk_server_credentials_t cred,
					    gnutls_psk_server_credentials_function2 *func);

typedef int gnutls_psk_client_credentials_function(gnutls_session_t,
						   char **username,
						   gnutls_datum_t * key);
typedef int gnutls_psk_client_credentials_function2(gnutls_session_t,
						    gnutls_datum_t *username,
						    gnutls_datum_t *key);
void
gnutls_psk_set_client_credentials_function(gnutls_psk_client_credentials_t
					   cred,
					   gnutls_psk_client_credentials_function
					   * func);
void
gnutls_psk_set_client_credentials_function2(gnutls_psk_client_credentials_t cred,
					    gnutls_psk_client_credentials_function2 *func);

int gnutls_hex_encode(const gnutls_datum_t * data, char *result,
		      size_t * result_size);
int gnutls_hex_decode(const gnutls_datum_t * hex_data, void *result,
		      size_t * result_size);

int gnutls_hex_encode2(const gnutls_datum_t * data, gnutls_datum_t *result);
int gnutls_hex_decode2(const gnutls_datum_t * data, gnutls_datum_t *result);

void
gnutls_psk_set_server_dh_params(gnutls_psk_server_credentials_t res,
				gnutls_dh_params_t dh_params);

int
gnutls_psk_set_server_known_dh_params(gnutls_psk_server_credentials_t res,
				      gnutls_sec_param_t sec_param);

void
gnutls_psk_set_server_params_function(gnutls_psk_server_credentials_t
				      res, gnutls_params_function * func);

/**
 * gnutls_x509_subject_alt_name_t:
 * @GNUTLS_SAN_DNSNAME: DNS-name SAN.
 * @GNUTLS_SAN_RFC822NAME: E-mail address SAN.
 * @GNUTLS_SAN_URI: URI SAN.
 * @GNUTLS_SAN_IPADDRESS: IP address SAN.
 * @GNUTLS_SAN_OTHERNAME: OtherName SAN.
 * @GNUTLS_SAN_DN: DN SAN.
 * @GNUTLS_SAN_REGISTERED_ID: RegisteredID.
 * @GNUTLS_SAN_OTHERNAME_XMPP: Virtual SAN, used by certain functions for convenience.
 * @GNUTLS_SAN_OTHERNAME_KRB5PRINCIPAL: Virtual SAN, used by certain functions for convenience.
 *
 * Enumeration of different subject alternative names types.
 */
typedef enum gnutls_x509_subject_alt_name_t {
	GNUTLS_SAN_DNSNAME = 1,
	GNUTLS_SAN_RFC822NAME = 2,
	GNUTLS_SAN_URI = 3,
	GNUTLS_SAN_IPADDRESS = 4,
	GNUTLS_SAN_OTHERNAME = 5,
	GNUTLS_SAN_DN = 6,
	GNUTLS_SAN_REGISTERED_ID = 7,
	GNUTLS_SAN_MAX = GNUTLS_SAN_REGISTERED_ID,
	/* The following are "virtual" subject alternative name types, in
	   that they are represented by an otherName value and an OID.
	   Used by gnutls_x509_crt_get_subject_alt_othername_oid.  */
	GNUTLS_SAN_OTHERNAME_XMPP = 1000,
	GNUTLS_SAN_OTHERNAME_KRB5PRINCIPAL
} gnutls_x509_subject_alt_name_t;

struct gnutls_openpgp_crt_int;
typedef struct gnutls_openpgp_crt_int *gnutls_openpgp_crt_t;

struct gnutls_openpgp_privkey_int;
typedef struct gnutls_openpgp_privkey_int *gnutls_openpgp_privkey_t;

struct gnutls_pkcs11_privkey_st;
typedef struct gnutls_pkcs11_privkey_st *gnutls_pkcs11_privkey_t;

/**
 * gnutls_privkey_type_t:
 * @GNUTLS_PRIVKEY_X509: X.509 private key, #gnutls_x509_privkey_t.
 * @GNUTLS_PRIVKEY_OPENPGP: OpenPGP private key, #gnutls_openpgp_privkey_t.
 * @GNUTLS_PRIVKEY_PKCS11: PKCS11 private key, #gnutls_pkcs11_privkey_t.
 * @GNUTLS_PRIVKEY_EXT: External private key, operating using callbacks.
 *
 * Enumeration of different private key types.
 */
typedef enum {
	GNUTLS_PRIVKEY_X509,
	GNUTLS_PRIVKEY_OPENPGP,
	GNUTLS_PRIVKEY_PKCS11,
	GNUTLS_PRIVKEY_EXT
} gnutls_privkey_type_t;

typedef struct gnutls_retr2_st {
	gnutls_certificate_type_t cert_type;
	gnutls_privkey_type_t key_type;

	union {
		gnutls_x509_crt_t *x509;
		gnutls_openpgp_crt_t pgp;
	} cert;
	unsigned int ncerts;	/* one for pgp keys */

	union {
		gnutls_x509_privkey_t x509;
		gnutls_openpgp_privkey_t pgp;
		gnutls_pkcs11_privkey_t pkcs11;
	} key;

	unsigned int deinit_all;	/* if non zero all keys will be deinited */
} gnutls_retr2_st;


  /* Functions that allow auth_info_t structures handling
   */

gnutls_credentials_type_t gnutls_auth_get_type(gnutls_session_t session);
gnutls_credentials_type_t
gnutls_auth_server_get_type(gnutls_session_t session);
gnutls_credentials_type_t
gnutls_auth_client_get_type(gnutls_session_t session);

  /* DH */

void gnutls_dh_set_prime_bits(gnutls_session_t session, unsigned int bits);
int gnutls_dh_get_secret_bits(gnutls_session_t session);
int gnutls_dh_get_peers_public_bits(gnutls_session_t session);
int gnutls_dh_get_prime_bits(gnutls_session_t session);

int gnutls_dh_get_group(gnutls_session_t session, gnutls_datum_t * raw_gen,
			gnutls_datum_t * raw_prime);
int gnutls_dh_get_pubkey(gnutls_session_t session,
			 gnutls_datum_t * raw_key);

  /* X509PKI */


  /* These are set on the credentials structure.
   */

  /* use gnutls_certificate_set_retrieve_function2() in abstract.h
   * instead. It's much more efficient.
   */

typedef int gnutls_certificate_retrieve_function(gnutls_session_t,
						 const
						 gnutls_datum_t *
						 req_ca_rdn,
						 int nreqs,
						 const
						 gnutls_pk_algorithm_t
						 * pk_algos,
						 int
						 pk_algos_length,
						 gnutls_retr2_st *);


void
gnutls_certificate_set_retrieve_function(gnutls_certificate_credentials_t
					 cred,
					 gnutls_certificate_retrieve_function
					 * func);

void
gnutls_certificate_set_verify_function(gnutls_certificate_credentials_t
				       cred,
				       gnutls_certificate_verify_function
				       * func);

void
gnutls_certificate_server_set_request(gnutls_session_t session,
				      gnutls_certificate_request_t req);

  /* get data from the session
   */
const gnutls_datum_t *gnutls_certificate_get_peers(gnutls_session_t
						   session, unsigned int
						   *list_size);
const gnutls_datum_t *gnutls_certificate_get_ours(gnutls_session_t
						  session);

int gnutls_certificate_get_peers_subkey_id(gnutls_session_t session,
					   gnutls_datum_t * id);

time_t gnutls_certificate_activation_time_peers(gnutls_session_t session);
time_t gnutls_certificate_expiration_time_peers(gnutls_session_t session);

unsigned gnutls_certificate_client_get_request_status(gnutls_session_t session);
int gnutls_certificate_verify_peers2(gnutls_session_t session,
				     unsigned int *status);
int gnutls_certificate_verify_peers3(gnutls_session_t session,
				     const char *hostname,
				     unsigned int *status);

int
gnutls_certificate_verify_peers(gnutls_session_t session,
				gnutls_typed_vdata_st * data,
				unsigned int elements,
				unsigned int *status);

int gnutls_certificate_verification_status_print(unsigned int status,
						 gnutls_certificate_type_t
						 type,
						 gnutls_datum_t * out,
						 unsigned int flags);

int gnutls_pem_base64_encode(const char *msg, const gnutls_datum_t * data,
			     char *result, size_t * result_size);
int gnutls_pem_base64_decode(const char *header,
			     const gnutls_datum_t * b64_data,
			     unsigned char *result, size_t * result_size);

int gnutls_pem_base64_encode2(const char *msg,
				   const gnutls_datum_t * data,
				   gnutls_datum_t * result);
int gnutls_pem_base64_decode2(const char *header,
				   const gnutls_datum_t * b64_data,
				   gnutls_datum_t * result);

int gnutls_base64_encode2(const gnutls_datum_t * data,
			  gnutls_datum_t * result);
int gnutls_base64_decode2(const gnutls_datum_t * b64_data,
			  gnutls_datum_t * result);

#define gnutls_pem_base64_encode_alloc gnutls_pem_base64_encode2
#define gnutls_pem_base64_decode_alloc gnutls_pem_base64_decode2

  /* key_usage will be an OR of the following values:
   */

  /* when the key is to be used for signing: */
#define GNUTLS_KEY_DIGITAL_SIGNATURE	128
#define GNUTLS_KEY_NON_REPUDIATION	64
  /* when the key is to be used for encryption: */
#define GNUTLS_KEY_KEY_ENCIPHERMENT	32
#define GNUTLS_KEY_DATA_ENCIPHERMENT	16
#define GNUTLS_KEY_KEY_AGREEMENT	8
#define GNUTLS_KEY_KEY_CERT_SIGN	4
#define GNUTLS_KEY_CRL_SIGN		2
#define GNUTLS_KEY_ENCIPHER_ONLY	1
#define GNUTLS_KEY_DECIPHER_ONLY	32768

void
gnutls_certificate_set_params_function(gnutls_certificate_credentials_t
				       res, gnutls_params_function * func);
void gnutls_anon_set_params_function(gnutls_anon_server_credentials_t res,
				     gnutls_params_function * func);
void gnutls_psk_set_params_function(gnutls_psk_server_credentials_t res,
				    gnutls_params_function * func);

int gnutls_hex2bin(const char *hex_data, size_t hex_size,
		   void *bin_data, size_t * bin_size);

  /* Trust on first use (or ssh like) functions */

  /* stores the provided information to a database
   */
typedef int (*gnutls_tdb_store_func) (const char *db_name,
				      const char *host,
				      const char *service,
				      time_t expiration,
				      const gnutls_datum_t * pubkey);

typedef int (*gnutls_tdb_store_commitment_func) (const char *db_name,
						 const char *host,
						 const char *service,
						 time_t expiration,
						 gnutls_digest_algorithm_t
						 hash_algo,
						 const gnutls_datum_t *
						 hash);

  /* searches for the provided host/service pair that match the
   * provided public key in the database. */
typedef int (*gnutls_tdb_verify_func) (const char *db_name,
				       const char *host,
				       const char *service,
				       const gnutls_datum_t * pubkey);


struct gnutls_tdb_int;
typedef struct gnutls_tdb_int *gnutls_tdb_t;

int gnutls_tdb_init(gnutls_tdb_t * tdb);
void gnutls_tdb_set_store_func(gnutls_tdb_t tdb,
			       gnutls_tdb_store_func store);
void gnutls_tdb_set_store_commitment_func(gnutls_tdb_t tdb,
					  gnutls_tdb_store_commitment_func
					  cstore);
void gnutls_tdb_set_verify_func(gnutls_tdb_t tdb,
				gnutls_tdb_verify_func verify);
void gnutls_tdb_deinit(gnutls_tdb_t tdb);

int gnutls_verify_stored_pubkey(const char *db_name,
				gnutls_tdb_t tdb,
				const char *host,
				const char *service,
				gnutls_certificate_type_t cert_type,
				const gnutls_datum_t * cert,
				unsigned int flags);

#define GNUTLS_SCOMMIT_FLAG_ALLOW_BROKEN 1
int gnutls_store_commitment(const char *db_name,
			    gnutls_tdb_t tdb,
			    const char *host,
			    const char *service,
			    gnutls_digest_algorithm_t hash_algo,
			    const gnutls_datum_t * hash,
			    time_t expiration, unsigned int flags);

int gnutls_store_pubkey(const char *db_name,
			gnutls_tdb_t tdb,
			const char *host,
			const char *service,
			gnutls_certificate_type_t cert_type,
			const gnutls_datum_t * cert,
			time_t expiration, unsigned int flags);

  /* Other helper functions */
int gnutls_load_file(const char *filename, gnutls_datum_t * data);

unsigned gnutls_url_is_supported(const char *url);

  /* PIN callback */

/**
 * gnutls_pin_flag_t:
 * @GNUTLS_PIN_USER: The PIN for the user.
 * @GNUTLS_PIN_SO: The PIN for the security officer (admin).
 * @GNUTLS_PIN_CONTEXT_SPECIFIC: The PIN is for a specific action and key like signing.
 * @GNUTLS_PIN_FINAL_TRY: This is the final try before blocking.
 * @GNUTLS_PIN_COUNT_LOW: Few tries remain before token blocks.
 * @GNUTLS_PIN_WRONG: Last given PIN was not correct.
 *
 * Enumeration of different flags that are input to the PIN function.
 */
typedef enum {
	GNUTLS_PIN_USER = (1 << 0),
	GNUTLS_PIN_SO = (1 << 1),
	GNUTLS_PIN_FINAL_TRY = (1 << 2),
	GNUTLS_PIN_COUNT_LOW = (1 << 3),
	GNUTLS_PIN_CONTEXT_SPECIFIC = (1 << 4),
	GNUTLS_PIN_WRONG = (1 << 5)
} gnutls_pin_flag_t;

#define GNUTLS_PKCS11_PIN_USER GNUTLS_PIN_USER
#define GNUTLS_PKCS11_PIN_SO GNUTLS_PIN_SO
#define GNUTLS_PKCS11_PIN_FINAL_TRY GNUTLS_PIN_FINAL_TRY
#define GNUTLS_PKCS11_PIN_COUNT_LOW  GNUTLS_PIN_COUNT_LOW
#define GNUTLS_PKCS11_PIN_CONTEXT_SPECIFIC GNUTLS_PIN_CONTEXT_SPECIFIC
#define GNUTLS_PKCS11_PIN_WRONG GNUTLS_PIN_WRONG

/**
 * gnutls_pin_callback_t:
 * @userdata: user-controlled data from gnutls_pkcs11_set_pin_function().
 * @attempt: pin-attempt counter, initially 0.
 * @token_url: URL of token.
 * @token_label: label of token.
 * @flags: a #gnutls_pin_flag_t flag.
 * @pin: buffer to hold PIN, of size @pin_max.
 * @pin_max: size of @pin buffer.
 *
 * Callback function type for PKCS#11 or TPM PIN entry.  It is set by
 * functions like gnutls_pkcs11_set_pin_function().
 *
 * The callback should provides the PIN code to unlock the token with
 * label @token_label, specified by the URL @token_url.
 *
 * The PIN code, as a NUL-terminated ASCII string, should be copied
 * into the @pin buffer (of maximum size @pin_max), and return 0 to
 * indicate success.  Alternatively, the callback may return a
 * negative gnutls error code to indicate failure and cancel PIN entry
 * (in which case, the contents of the @pin parameter are ignored).
 *
 * When a PIN is required, the callback will be invoked repeatedly
 * (and indefinitely) until either the returned PIN code is correct,
 * the callback returns failure, or the token refuses login (e.g. when
 * the token is locked due to too many incorrect PINs!).  For the
 * first such invocation, the @attempt counter will have value zero;
 * it will increase by one for each subsequent attempt.
 *
 * Returns: %GNUTLS_E_SUCCESS (0) on success or a negative error code on error.
 *
 * Since: 2.12.0
 **/
typedef int (*gnutls_pin_callback_t) (void *userdata, int attempt,
				      const char *token_url,
				      const char *token_label,
				      unsigned int flags,
				      char *pin, size_t pin_max);

void gnutls_certificate_set_pin_function(gnutls_certificate_credentials_t,
					 gnutls_pin_callback_t fn,
					 void *userdata);

/* Public string related functions */
typedef struct gnutls_buffer_st *gnutls_buffer_t;

int gnutls_buffer_append_data(gnutls_buffer_t, const void *data, size_t data_size);

#define GNUTLS_UTF8_IGNORE_ERRS 1
int gnutls_utf8_password_normalize(const unsigned char *password, unsigned password_len,
				   gnutls_datum_t *out, unsigned flags);

/* Public extensions related functions */

typedef void *gnutls_ext_priv_data_t;

void gnutls_ext_set_data(gnutls_session_t session, unsigned type,
			 gnutls_ext_priv_data_t);
int gnutls_ext_get_data(gnutls_session_t session, unsigned type,
			gnutls_ext_priv_data_t *);

unsigned gnutls_ext_get_current_msg(gnutls_session_t session);

typedef int (*gnutls_ext_recv_func) (gnutls_session_t session,
				     const unsigned char *data,
				     size_t len);

typedef int (*gnutls_ext_send_func) (gnutls_session_t session,
				     gnutls_buffer_t extdata);

typedef void (*gnutls_ext_deinit_data_func) (gnutls_ext_priv_data_t data);

typedef int (*gnutls_ext_pack_func) (gnutls_ext_priv_data_t data,
				     gnutls_buffer_t packed_data);

typedef int (*gnutls_ext_unpack_func) (gnutls_buffer_t packed_data,
				       gnutls_ext_priv_data_t *data);

#define GNUTLS_EXT_RAW_FLAG_TLS_CLIENT_HELLO 1
#define GNUTLS_EXT_RAW_FLAG_DTLS_CLIENT_HELLO (1<<1)
typedef int (*gnutls_ext_raw_process_func)(void *ctx, unsigned tls_id, const unsigned char *data, unsigned data_size);
int gnutls_ext_raw_parse(void *ctx, gnutls_ext_raw_process_func cb,
			 const gnutls_datum_t *data, unsigned int flags);

/**
 * gnutls_ext_parse_type_t:
 * @GNUTLS_EXT_NONE: Never to be parsed
 * @GNUTLS_EXT_ANY: Any extension type (should not be used as it is used only internally).
 * @GNUTLS_EXT_VERSION_NEG: Extensions to be parsed first for TLS version negotiation.
 * @GNUTLS_EXT_MANDATORY: Parsed after @GNUTLS_EXT_VERSION_NEG and even when resuming.
 * @GNUTLS_EXT_APPLICATION: Parsed after @GNUTLS_EXT_MANDATORY
 * @GNUTLS_EXT_TLS: TLS-internal extensions, parsed after @GNUTLS_EXT_APPLICATION.
 *
 * Enumeration of different TLS extension parsing phases.  The @gnutls_ext_parse_type_t
 * indicates the time/phase an extension is parsed during Client or Server hello parsing.
 *
 */
typedef enum {
  GNUTLS_EXT_ANY = 0,
  GNUTLS_EXT_APPLICATION = 1,
  GNUTLS_EXT_TLS = 2,
  GNUTLS_EXT_MANDATORY = 3,
  GNUTLS_EXT_NONE = 4,
  GNUTLS_EXT_VERSION_NEG = 5
} gnutls_ext_parse_type_t;

/**
 * gnutls_ext_flags_t:
 * @GNUTLS_EXT_FLAG_OVERRIDE_INTERNAL: If specified the extension registered will override the internal; this does not work with extensions existing prior to 3.6.0.
 * @GNUTLS_EXT_FLAG_CLIENT_HELLO: This extension can be present in a client hello
 * @GNUTLS_EXT_FLAG_TLS12_SERVER_HELLO: This extension can be present in a TLS1.2 or earlier server hello
 * @GNUTLS_EXT_FLAG_TLS13_SERVER_HELLO: This extension can be present in a TLS1.3 server hello
 * @GNUTLS_EXT_FLAG_EE: This extension can be present in encrypted extensions message
 * @GNUTLS_EXT_FLAG_HRR: This extension can be present in hello retry request message
 * @GNUTLS_EXT_FLAG_IGNORE_CLIENT_REQUEST: When flag is present, this extension will be send even if the client didn't advertise it. An extension of this type is the Cookie TLS1.3 extension.
 * @GNUTLS_EXT_FLAG_DTLS: This extension can be present under DTLS; otherwise ignored.
 * @GNUTLS_EXT_FLAG_TLS: This extension can be present under TLS; otherwise ignored.
 *
 * Enumeration of different TLS extension registration flags.
 */
typedef enum {
  GNUTLS_EXT_FLAG_OVERRIDE_INTERNAL = 1,
  GNUTLS_EXT_FLAG_CLIENT_HELLO = (1<<1),
  GNUTLS_EXT_FLAG_TLS12_SERVER_HELLO = (1<<2),
  GNUTLS_EXT_FLAG_TLS13_SERVER_HELLO = (1<<3),
  GNUTLS_EXT_FLAG_EE = (1<<4), /* ENCRYPTED */
  GNUTLS_EXT_FLAG_HRR = (1<<5),
  GNUTLS_EXT_FLAG_IGNORE_CLIENT_REQUEST = (1<<6),
  GNUTLS_EXT_FLAG_TLS = (1<<7),
  GNUTLS_EXT_FLAG_DTLS = (1<<8)
} gnutls_ext_flags_t;

/* Register a custom tls extension
 */
int gnutls_ext_register(const char *name, int type, gnutls_ext_parse_type_t parse_point,
				gnutls_ext_recv_func recv_func, gnutls_ext_send_func send_func, 
				gnutls_ext_deinit_data_func deinit_func, gnutls_ext_pack_func pack_func,
				gnutls_ext_unpack_func unpack_func);

int gnutls_session_ext_register(gnutls_session_t, const char *name, int type, gnutls_ext_parse_type_t parse_point,
				gnutls_ext_recv_func recv_func, gnutls_ext_send_func send_func, 
				gnutls_ext_deinit_data_func deinit_func, gnutls_ext_pack_func pack_func,
				gnutls_ext_unpack_func unpack_func, unsigned flags);

const char *gnutls_ext_get_name(unsigned int ext);
const char *gnutls_ext_get_name2(gnutls_session_t session, unsigned int tls_id,
				 gnutls_ext_parse_type_t parse_point);

/* Public supplemental data related functions */

typedef int (*gnutls_supp_recv_func) (gnutls_session_t session,
			       const unsigned char * data, size_t data_size);
typedef int (*gnutls_supp_send_func) (gnutls_session_t session,
			       gnutls_buffer_t buf);

int gnutls_supplemental_register(const char *name,
				gnutls_supplemental_data_format_type_t type,
				gnutls_supp_recv_func supp_recv_func,
				gnutls_supp_send_func supp_send_func);

int gnutls_session_supplemental_register(gnutls_session_t session, const char *name,
				gnutls_supplemental_data_format_type_t type,
				gnutls_supp_recv_func supp_recv_func,
				gnutls_supp_send_func supp_send_func,
				unsigned int flags);

void gnutls_supplemental_recv(gnutls_session_t session, unsigned do_recv_supplemental);

void gnutls_supplemental_send(gnutls_session_t session, unsigned do_send_supplemental);

/* Anti-replay related functions */

typedef struct gnutls_anti_replay_st *gnutls_anti_replay_t;

int gnutls_anti_replay_init(gnutls_anti_replay_t *anti_replay);
void gnutls_anti_replay_deinit(gnutls_anti_replay_t anti_replay);
void gnutls_anti_replay_set_window(gnutls_anti_replay_t anti_replay,
				   unsigned int window);
void gnutls_anti_replay_enable(gnutls_session_t session,
			       gnutls_anti_replay_t anti_replay);

typedef int (*gnutls_db_add_func) (void *, time_t exp_time, const gnutls_datum_t *key,
				   const gnutls_datum_t *data);

void gnutls_anti_replay_set_add_function(gnutls_anti_replay_t,
					 gnutls_db_add_func add_func);

void gnutls_anti_replay_set_ptr(gnutls_anti_replay_t, void *ptr);


/**
 * gnutls_record_encryption_level_t:
 * @GNUTLS_ENCRYPTION_LEVEL_INITIAL: initial level that doesn't involve any
 *    encryption
 * @GNUTLS_ENCRYPTION_LEVEL_EARLY: early traffic secret is installed
 * @GNUTLS_ENCRYPTION_LEVEL_HANDSHAKE: handshake traffic secret is installed
 * @GNUTLS_ENCRYPTION_LEVEL_APPLICATION: application traffic secret is installed
 *
 * Enumeration of different levels of record encryption currently in place.
 * This is used by gnutls_handshake_set_read_function() and
 * gnutls_handshake_write().
 *
 * Since: 3.7.0
 */
typedef enum {
	GNUTLS_ENCRYPTION_LEVEL_INITIAL,
	GNUTLS_ENCRYPTION_LEVEL_EARLY,
	GNUTLS_ENCRYPTION_LEVEL_HANDSHAKE,
	GNUTLS_ENCRYPTION_LEVEL_APPLICATION
} gnutls_record_encryption_level_t;

  /**
   * gnutls_handshake_read_func:
   * @session: the current session
   * @htype: the type of the handshake message (#gnutls_handshake_description_t)
   * @level: #gnutls_record_encryption_level_t
   * @data: the (const) data that was being sent
   * @data_size: the size of data
   *
   * Function prototype for handshake intercepting hooks. It is set using
   * gnutls_handshake_set_read_function().
   *
   * Returns: Non zero on error.
   * Since: 3.7.0
   */
typedef int (*gnutls_handshake_read_func) (gnutls_session_t session,
					   gnutls_record_encryption_level_t level,
					   gnutls_handshake_description_t htype,
					   const void *data, size_t data_size);

void
gnutls_handshake_set_read_function(gnutls_session_t session,
				   gnutls_handshake_read_func func);

int
gnutls_handshake_write(gnutls_session_t session,
		       gnutls_record_encryption_level_t level,
		       const void *data, size_t data_size);

  /**
   * gnutls_handshake_secret_func:
   * @session: the current session
   * @level: the encryption level
   * @secret_read: the secret used for reading, can be %NULL if not set
   * @secret_write: the secret used for writing, can be %NULL if not set
   * @secret_size: the size of the secrets
   *
   * Function prototype for secret hooks. It is set using
   * gnutls_handshake_set_secret_function().
   *
   * Returns: Non zero on error.
   * Since: 3.7.0
   */
typedef int (*gnutls_handshake_secret_func) (gnutls_session_t session,
					     gnutls_record_encryption_level_t level,
					     const void *secret_read,
					     const void *secret_write,
					     size_t secret_size);

void
gnutls_handshake_set_secret_function(gnutls_session_t session,
				     gnutls_handshake_secret_func func);

  /**
   * gnutls_alert_read_func:
   * @session: the current session
   * @level: #gnutls_record_encryption_level_t
   * @alert_level: the level of the alert
   * @alert_desc: the alert description
   *
   * Function prototype for alert intercepting hooks. It is set using
   * gnutls_alert_set_read_function().
   *
   * Returns: Non zero on error.
   * Since: 3.7.0
   */
typedef int (*gnutls_alert_read_func) (gnutls_session_t session,
				       gnutls_record_encryption_level_t level,
				       gnutls_alert_level_t alert_level,
				       gnutls_alert_description_t alert_desc);

void
gnutls_alert_set_read_function(gnutls_session_t session,
			       gnutls_alert_read_func func);

/* FIPS140-2 related functions */
unsigned gnutls_fips140_mode_enabled(void);

/**
 * gnutls_fips_mode_t:
 * @GNUTLS_FIPS140_DISABLED: The FIPS140-2 mode is disabled.
 * @GNUTLS_FIPS140_STRICT: The default mode; all forbidden operations will cause an
 *                         operation failure via error code.
 * @GNUTLS_FIPS140_LAX: The library still uses the FIPS140-2 relevant algorithms but all
 *                      forbidden by FIPS140-2 operations are allowed; this is useful when the
 *                      application is aware of the followed security policy, and needs
 *                      to utilize disallowed operations for other reasons (e.g., compatibility).
 * @GNUTLS_FIPS140_LOG: Similarly to %GNUTLS_FIPS140_LAX, it allows forbidden operations; any use of them results
 *                      to a message to the audit callback functions.
 * @GNUTLS_FIPS140_SELFTESTS: A transient state during library initialization. That state
 *			cannot be set or seen by applications.
 *
 * Enumeration of different operational modes under FIPS140-2.
 */
typedef enum gnutls_fips_mode_t {
  GNUTLS_FIPS140_DISABLED = 0,
  GNUTLS_FIPS140_STRICT = 1,
  GNUTLS_FIPS140_SELFTESTS = 2,
  GNUTLS_FIPS140_LAX = 3,
  GNUTLS_FIPS140_LOG = 4
} gnutls_fips_mode_t;

#define GNUTLS_FIPS140_SET_MODE_THREAD 1

void gnutls_fips140_set_mode(gnutls_fips_mode_t mode, unsigned flags);

#define GNUTLS_FIPS140_SET_LAX_MODE() do { \
	if (gnutls_fips140_mode_enabled()) \
		gnutls_fips140_set_mode(GNUTLS_FIPS140_LAX, GNUTLS_FIPS140_SET_MODE_THREAD); \
	} while(0)

#define GNUTLS_FIPS140_SET_STRICT_MODE() do { \
	if (gnutls_fips140_mode_enabled()) \
		gnutls_fips140_set_mode(GNUTLS_FIPS140_STRICT, GNUTLS_FIPS140_SET_MODE_THREAD); \
	} while(0)

  /* Gnutls error codes. The mapping to a TLS alert is also shown in
   * comments.
   */

#define GNUTLS_E_SUCCESS 0
#define	GNUTLS_E_UNKNOWN_COMPRESSION_ALGORITHM -3
#define	GNUTLS_E_UNKNOWN_CIPHER_TYPE -6
#define	GNUTLS_E_LARGE_PACKET -7
#define GNUTLS_E_UNSUPPORTED_VERSION_PACKET -8	/* GNUTLS_A_PROTOCOL_VERSION */
#define GNUTLS_E_TLS_PACKET_DECODING_ERROR GNUTLS_E_UNEXPECTED_PACKET_LENGTH
#define GNUTLS_E_UNEXPECTED_PACKET_LENGTH -9	/* GNUTLS_A_DECODE_ERROR */
#define GNUTLS_E_INVALID_SESSION -10
#define GNUTLS_E_FATAL_ALERT_RECEIVED -12
#define GNUTLS_E_UNEXPECTED_PACKET -15	/* GNUTLS_A_UNEXPECTED_MESSAGE */
#define GNUTLS_E_WARNING_ALERT_RECEIVED -16
#define GNUTLS_E_ERROR_IN_FINISHED_PACKET -18
#define GNUTLS_E_UNEXPECTED_HANDSHAKE_PACKET -19
#define	GNUTLS_E_UNKNOWN_CIPHER_SUITE -21	/* GNUTLS_A_HANDSHAKE_FAILURE */
#define	GNUTLS_E_UNWANTED_ALGORITHM -22
#define	GNUTLS_E_MPI_SCAN_FAILED -23
#define GNUTLS_E_DECRYPTION_FAILED -24	/* GNUTLS_A_DECRYPTION_FAILED, GNUTLS_A_BAD_RECORD_MAC */
#define GNUTLS_E_MEMORY_ERROR -25
#define GNUTLS_E_DECOMPRESSION_FAILED -26	/* GNUTLS_A_DECOMPRESSION_FAILURE */
#define GNUTLS_E_COMPRESSION_FAILED -27
#define GNUTLS_E_AGAIN -28
#define GNUTLS_E_EXPIRED -29
#define GNUTLS_E_DB_ERROR -30
#define GNUTLS_E_SRP_PWD_ERROR GNUTLS_E_KEYFILE_ERROR
#define GNUTLS_E_KEYFILE_ERROR -31
#define GNUTLS_E_INSUFFICIENT_CREDENTIALS -32
#define GNUTLS_E_INSUFICIENT_CREDENTIALS GNUTLS_E_INSUFFICIENT_CREDENTIALS	/* for backwards compatibility only */
#define GNUTLS_E_INSUFFICIENT_CRED GNUTLS_E_INSUFFICIENT_CREDENTIALS
#define GNUTLS_E_INSUFICIENT_CRED GNUTLS_E_INSUFFICIENT_CREDENTIALS	/* for backwards compatibility only */

#define GNUTLS_E_HASH_FAILED -33
#define GNUTLS_E_BASE64_DECODING_ERROR -34

#define	GNUTLS_E_MPI_PRINT_FAILED -35
#define GNUTLS_E_REHANDSHAKE -37	/* GNUTLS_A_NO_RENEGOTIATION */
#define GNUTLS_E_GOT_APPLICATION_DATA -38
#define GNUTLS_E_RECORD_LIMIT_REACHED -39
#define GNUTLS_E_ENCRYPTION_FAILED -40

#define GNUTLS_E_PK_ENCRYPTION_FAILED -44
#define GNUTLS_E_PK_DECRYPTION_FAILED -45
#define GNUTLS_E_PK_SIGN_FAILED -46
#define GNUTLS_E_X509_UNSUPPORTED_CRITICAL_EXTENSION -47
#define GNUTLS_E_KEY_USAGE_VIOLATION -48
#define GNUTLS_E_NO_CERTIFICATE_FOUND -49	/* GNUTLS_A_BAD_CERTIFICATE */
#define GNUTLS_E_INVALID_REQUEST -50
#define GNUTLS_E_SHORT_MEMORY_BUFFER -51
#define GNUTLS_E_INTERRUPTED -52
#define GNUTLS_E_PUSH_ERROR -53
#define GNUTLS_E_PULL_ERROR -54
#define GNUTLS_E_RECEIVED_ILLEGAL_PARAMETER -55	/* GNUTLS_A_ILLEGAL_PARAMETER */
#define GNUTLS_E_REQUESTED_DATA_NOT_AVAILABLE -56
#define GNUTLS_E_PKCS1_WRONG_PAD -57
#define GNUTLS_E_RECEIVED_ILLEGAL_EXTENSION -58
#define GNUTLS_E_INTERNAL_ERROR -59
#define GNUTLS_E_DH_PRIME_UNACCEPTABLE -63
#define GNUTLS_E_FILE_ERROR -64
#define GNUTLS_E_TOO_MANY_EMPTY_PACKETS -78
#define GNUTLS_E_UNKNOWN_PK_ALGORITHM -80
#define GNUTLS_E_TOO_MANY_HANDSHAKE_PACKETS -81
#define GNUTLS_E_RECEIVED_DISALLOWED_NAME -82 /* GNUTLS_A_ILLEGAL_PARAMETER */
#define GNUTLS_E_CERTIFICATE_REQUIRED -112 /* GNUTLS_A_CERTIFICATE_REQUIRED */

  /* returned if you need to generate temporary RSA
   * parameters. These are needed for export cipher suites.
   */
#define GNUTLS_E_NO_TEMPORARY_RSA_PARAMS -84

#define GNUTLS_E_NO_COMPRESSION_ALGORITHMS -86
#define GNUTLS_E_NO_CIPHER_SUITES -87

#define GNUTLS_E_OPENPGP_GETKEY_FAILED -88
#define GNUTLS_E_PK_SIG_VERIFY_FAILED -89

#define GNUTLS_E_ILLEGAL_SRP_USERNAME -90
#define GNUTLS_E_SRP_PWD_PARSING_ERROR GNUTLS_E_KEYFILE_PARSING_ERROR
#define GNUTLS_E_KEYFILE_PARSING_ERROR -91
#define GNUTLS_E_NO_TEMPORARY_DH_PARAMS -93

  /* For certificate and key stuff
   */
#define GNUTLS_E_ASN1_ELEMENT_NOT_FOUND -67
#define GNUTLS_E_ASN1_IDENTIFIER_NOT_FOUND -68
#define GNUTLS_E_ASN1_DER_ERROR -69
#define GNUTLS_E_ASN1_VALUE_NOT_FOUND -70
#define GNUTLS_E_ASN1_GENERIC_ERROR -71
#define GNUTLS_E_ASN1_VALUE_NOT_VALID -72
#define GNUTLS_E_ASN1_TAG_ERROR -73
#define GNUTLS_E_ASN1_TAG_IMPLICIT -74
#define GNUTLS_E_ASN1_TYPE_ANY_ERROR -75
#define GNUTLS_E_ASN1_SYNTAX_ERROR -76
#define GNUTLS_E_ASN1_DER_OVERFLOW -77
#define GNUTLS_E_OPENPGP_UID_REVOKED -79
#define GNUTLS_E_CERTIFICATE_ERROR -43
#define GNUTLS_E_X509_CERTIFICATE_ERROR GNUTLS_E_CERTIFICATE_ERROR
#define GNUTLS_E_CERTIFICATE_KEY_MISMATCH -60
#define GNUTLS_E_UNSUPPORTED_CERTIFICATE_TYPE -61	/* GNUTLS_A_UNSUPPORTED_CERTIFICATE */
#define GNUTLS_E_X509_UNKNOWN_SAN -62
#define GNUTLS_E_OPENPGP_FINGERPRINT_UNSUPPORTED -94
#define GNUTLS_E_X509_UNSUPPORTED_ATTRIBUTE -95
#define GNUTLS_E_UNKNOWN_HASH_ALGORITHM -96
#define GNUTLS_E_UNKNOWN_PKCS_CONTENT_TYPE -97
#define GNUTLS_E_UNKNOWN_PKCS_BAG_TYPE -98
#define GNUTLS_E_INVALID_PASSWORD -99
#define GNUTLS_E_MAC_VERIFY_FAILED -100	/* for PKCS #12 MAC */
#define GNUTLS_E_CONSTRAINT_ERROR -101

#define GNUTLS_E_WARNING_IA_IPHF_RECEIVED -102
#define GNUTLS_E_WARNING_IA_FPHF_RECEIVED -103

#define GNUTLS_E_IA_VERIFY_FAILED -104
#define GNUTLS_E_UNKNOWN_ALGORITHM -105
#define GNUTLS_E_UNSUPPORTED_SIGNATURE_ALGORITHM -106
#define GNUTLS_E_SAFE_RENEGOTIATION_FAILED -107
#define GNUTLS_E_UNSAFE_RENEGOTIATION_DENIED -108
#define GNUTLS_E_UNKNOWN_SRP_USERNAME -109
#define GNUTLS_E_PREMATURE_TERMINATION -110

#define GNUTLS_E_MALFORMED_CIDR -111

#define GNUTLS_E_BASE64_ENCODING_ERROR -201
#define GNUTLS_E_INCOMPATIBLE_GCRYPT_LIBRARY -202	/* obsolete */
#define GNUTLS_E_INCOMPATIBLE_CRYPTO_LIBRARY -202
#define GNUTLS_E_INCOMPATIBLE_LIBTASN1_LIBRARY -203

#define GNUTLS_E_OPENPGP_KEYRING_ERROR -204
#define GNUTLS_E_X509_UNSUPPORTED_OID -205

#define GNUTLS_E_RANDOM_FAILED -206
#define GNUTLS_E_BASE64_UNEXPECTED_HEADER_ERROR -207

#define GNUTLS_E_OPENPGP_SUBKEY_ERROR -208

#define GNUTLS_E_CRYPTO_ALREADY_REGISTERED GNUTLS_E_ALREADY_REGISTERED
#define GNUTLS_E_ALREADY_REGISTERED -209

#define GNUTLS_E_HANDSHAKE_TOO_LARGE -210

#define GNUTLS_E_CRYPTODEV_IOCTL_ERROR -211
#define GNUTLS_E_CRYPTODEV_DEVICE_ERROR -212

#define GNUTLS_E_CHANNEL_BINDING_NOT_AVAILABLE -213
#define GNUTLS_E_BAD_COOKIE -214
#define GNUTLS_E_OPENPGP_PREFERRED_KEY_ERROR -215
#define GNUTLS_E_INCOMPAT_DSA_KEY_WITH_TLS_PROTOCOL -216
#define GNUTLS_E_INSUFFICIENT_SECURITY -217

#define GNUTLS_E_HEARTBEAT_PONG_RECEIVED -292
#define GNUTLS_E_HEARTBEAT_PING_RECEIVED -293

#define GNUTLS_E_UNRECOGNIZED_NAME -294

/* PKCS11 related */
#define GNUTLS_E_PKCS11_ERROR -300
#define GNUTLS_E_PKCS11_LOAD_ERROR -301
#define GNUTLS_E_PARSING_ERROR -302
#define GNUTLS_E_PKCS11_PIN_ERROR -303

#define GNUTLS_E_PKCS11_SLOT_ERROR -305
#define GNUTLS_E_LOCKING_ERROR -306
#define GNUTLS_E_PKCS11_ATTRIBUTE_ERROR -307
#define GNUTLS_E_PKCS11_DEVICE_ERROR -308
#define GNUTLS_E_PKCS11_DATA_ERROR -309
#define GNUTLS_E_PKCS11_UNSUPPORTED_FEATURE_ERROR -310
#define GNUTLS_E_PKCS11_KEY_ERROR -311
#define GNUTLS_E_PKCS11_PIN_EXPIRED -312
#define GNUTLS_E_PKCS11_PIN_LOCKED -313
#define GNUTLS_E_PKCS11_SESSION_ERROR -314
#define GNUTLS_E_PKCS11_SIGNATURE_ERROR -315
#define GNUTLS_E_PKCS11_TOKEN_ERROR -316
#define GNUTLS_E_PKCS11_USER_ERROR -317

#define GNUTLS_E_CRYPTO_INIT_FAILED -318
#define GNUTLS_E_TIMEDOUT -319
#define GNUTLS_E_USER_ERROR -320
#define GNUTLS_E_ECC_NO_SUPPORTED_CURVES -321
#define GNUTLS_E_ECC_UNSUPPORTED_CURVE -322
#define GNUTLS_E_PKCS11_REQUESTED_OBJECT_NOT_AVAILBLE -323
#define GNUTLS_E_CERTIFICATE_LIST_UNSORTED -324
#define GNUTLS_E_ILLEGAL_PARAMETER -325 /* GNUTLS_A_ILLEGAL_PARAMETER */
#define GNUTLS_E_NO_PRIORITIES_WERE_SET -326
#define GNUTLS_E_X509_UNSUPPORTED_EXTENSION -327
#define GNUTLS_E_SESSION_EOF -328

#define GNUTLS_E_TPM_ERROR -329
#define GNUTLS_E_TPM_KEY_PASSWORD_ERROR -330
#define GNUTLS_E_TPM_SRK_PASSWORD_ERROR -331
#define GNUTLS_E_TPM_SESSION_ERROR -332
#define GNUTLS_E_TPM_KEY_NOT_FOUND -333
#define GNUTLS_E_TPM_UNINITIALIZED -334
#define GNUTLS_E_TPM_NO_LIB -335

#define GNUTLS_E_NO_CERTIFICATE_STATUS -340
#define GNUTLS_E_OCSP_RESPONSE_ERROR -341
#define GNUTLS_E_RANDOM_DEVICE_ERROR -342
#define GNUTLS_E_AUTH_ERROR -343
#define GNUTLS_E_NO_APPLICATION_PROTOCOL -344
#define GNUTLS_E_SOCKETS_INIT_ERROR -345
#define GNUTLS_E_KEY_IMPORT_FAILED -346
#define GNUTLS_E_INAPPROPRIATE_FALLBACK -347 /*GNUTLS_A_INAPPROPRIATE_FALLBACK*/
#define GNUTLS_E_CERTIFICATE_VERIFICATION_ERROR -348
#define GNUTLS_E_PRIVKEY_VERIFICATION_ERROR -349
#define GNUTLS_E_UNEXPECTED_EXTENSIONS_LENGTH -350 /*GNUTLS_A_DECODE_ERROR*/
#define GNUTLS_E_ASN1_EMBEDDED_NULL_IN_STRING -351

#define GNUTLS_E_SELF_TEST_ERROR -400
#define GNUTLS_E_NO_SELF_TEST -401
#define GNUTLS_E_LIB_IN_ERROR_STATE -402
#define GNUTLS_E_PK_GENERATION_ERROR -403
#define GNUTLS_E_IDNA_ERROR -404

#define GNUTLS_E_NEED_FALLBACK -405
#define GNUTLS_E_SESSION_USER_ID_CHANGED -406
#define GNUTLS_E_HANDSHAKE_DURING_FALSE_START -407
#define GNUTLS_E_UNAVAILABLE_DURING_HANDSHAKE -408
#define GNUTLS_E_PK_INVALID_PUBKEY -409
#define GNUTLS_E_PK_INVALID_PRIVKEY -410
#define GNUTLS_E_NOT_YET_ACTIVATED -411
#define GNUTLS_E_INVALID_UTF8_STRING -412
#define GNUTLS_E_NO_EMBEDDED_DATA -413
#define GNUTLS_E_INVALID_UTF8_EMAIL -414
#define GNUTLS_E_INVALID_PASSWORD_STRING -415
#define GNUTLS_E_CERTIFICATE_TIME_ERROR -416
#define GNUTLS_E_RECORD_OVERFLOW -417	/* GNUTLS_A_RECORD_OVERFLOW */
#define GNUTLS_E_ASN1_TIME_ERROR -418
#define GNUTLS_E_INCOMPATIBLE_SIG_WITH_KEY -419
#define GNUTLS_E_PK_INVALID_PUBKEY_PARAMS -420
#define GNUTLS_E_PK_NO_VALIDATION_PARAMS -421
#define GNUTLS_E_OCSP_MISMATCH_WITH_CERTS -422

#define GNUTLS_E_NO_COMMON_KEY_SHARE -423
#define GNUTLS_E_REAUTH_REQUEST -424
#define GNUTLS_E_TOO_MANY_MATCHES -425
#define GNUTLS_E_CRL_VERIFICATION_ERROR -426
#define GNUTLS_E_MISSING_EXTENSION -427
#define GNUTLS_E_DB_ENTRY_EXISTS -428
#define GNUTLS_E_EARLY_DATA_REJECTED -429
#define GNUTLS_E_X509_DUPLICATE_EXTENSION -430

#define GNUTLS_E_UNIMPLEMENTED_FEATURE -1250

/* Internal errors of the library; will never be returned
 * to a calling application */
#define GNUTLS_E_INT_RET_0 -1251
#define GNUTLS_E_INT_CHECK_AGAIN -1252

#define GNUTLS_E_APPLICATION_ERROR_MAX -65000
#define GNUTLS_E_APPLICATION_ERROR_MIN -65500

/* *INDENT-OFF* */
#ifdef __cplusplus
}
#endif
/* *INDENT-ON* */

#include <gnutls/compat.h>

#endif /* GNUTLS_GNUTLS_H */
