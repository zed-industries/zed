/*
 * Copyright (C) 2010-2012 Free Software Foundation, Inc.
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

#ifndef GNUTLS_ABSTRACT_H
#define GNUTLS_ABSTRACT_H

#include <gnutls/gnutls.h>
#include <gnutls/x509.h>
#include <gnutls/pkcs11.h>
#include <gnutls/openpgp.h>
#include <gnutls/tpm.h>

/* *INDENT-OFF* */
#ifdef __cplusplus
extern "C" {
#endif
/* *INDENT-ON* */

/* Public key operations */

#define GNUTLS_PUBKEY_VERIFY_FLAG_TLS_RSA GNUTLS_PUBKEY_VERIFY_FLAG_TLS1_RSA
/**
 * gnutls_pubkey_flags:
 * @GNUTLS_PUBKEY_DISABLE_CALLBACKS: The following flag disables call to PIN callbacks. Only
 *   relevant to TPM keys.
 * @GNUTLS_PUBKEY_GET_OPENPGP_FINGERPRINT: request an OPENPGP fingerprint instead of the default.
 *
 * Enumeration of different certificate import flags.
 */
typedef enum gnutls_pubkey_flags {
	GNUTLS_PUBKEY_DISABLE_CALLBACKS = 1 << 2,
	GNUTLS_PUBKEY_GET_OPENPGP_FINGERPRINT = 1 << 3
} gnutls_pubkey_flags_t;

/**
 * gnutls_abstract_export_flags:
 * @GNUTLS_EXPORT_FLAG_NO_LZ: do not prepend a leading zero to exported values
 *
 * Enumeration of different certificate import flags.
 */
typedef enum gnutls_abstract_export_flags {
	GNUTLS_EXPORT_FLAG_NO_LZ = 1
} gnutls_abstract_export_flags_t;

#define GNUTLS_PUBKEY_VERIFY_FLAG_TLS1_RSA GNUTLS_VERIFY_USE_TLS1_RSA

typedef int (*gnutls_privkey_sign_func) (gnutls_privkey_t key,
					 void *userdata,
					 const gnutls_datum_t *raw_data,
					 gnutls_datum_t * signature);


typedef int (*gnutls_privkey_decrypt_func) (gnutls_privkey_t key,
					    void *userdata,
					    const gnutls_datum_t *ciphertext,
					    gnutls_datum_t * plaintext);

typedef int (*gnutls_privkey_decrypt_func2) (gnutls_privkey_t key,
					     void *userdata,
					     const gnutls_datum_t *ciphertext,
					     unsigned char * plaintext,
					     size_t plaintext_size);

/* to be called to sign pre-hashed data. The input will be
 * the output of the hash (such as SHA256) corresponding to
 * the signature algorithm. The algorithm GNUTLS_SIGN_RSA_RAW
 * will be provided when RSA PKCS#1 DigestInfo structure is provided
 * as data (when this is called from a TLS 1.0 or 1.1 session).
 */
typedef int (*gnutls_privkey_sign_hash_func) (gnutls_privkey_t key,
					      gnutls_sign_algorithm_t algo,
					      void *userdata,
					      unsigned int flags,
					      const gnutls_datum_t *hash,
					      gnutls_datum_t * signature);

/* to be called to sign data. The input data will be
 * the data to be signed (and hashed), with the provided
 * signature algorithm. This function is used for algorithms
 * like ed25519 which cannot take pre-hashed data as input.
 */
typedef int (*gnutls_privkey_sign_data_func) (gnutls_privkey_t key,
					      gnutls_sign_algorithm_t algo,
					      void *userdata,
					      unsigned int flags,
					      const gnutls_datum_t *data,
					      gnutls_datum_t * signature);

typedef void (*gnutls_privkey_deinit_func) (gnutls_privkey_t key,
					    void *userdata);


#define GNUTLS_SIGN_ALGO_TO_FLAGS(sig) (unsigned int)((sig)<<20)
#define GNUTLS_FLAGS_TO_SIGN_ALGO(flags) (unsigned int)((flags)>>20)

/* Should return the public key algorithm (gnutls_pk_algorithm_t) */
#define GNUTLS_PRIVKEY_INFO_PK_ALGO 1
/* Should return the preferred signature algorithm (gnutls_sign_algorithm_t) or 0. */
#define GNUTLS_PRIVKEY_INFO_SIGN_ALGO (1<<1)
/* Should return true (1) or false (0) if the provided sign algorithm
 * (obtained with GNUTLS_FLAGS_TO_SIGN_ALGO) is supported.
 */
#define GNUTLS_PRIVKEY_INFO_HAVE_SIGN_ALGO (1<<2)
/* Should return the number of bits of the public key algorithm (required for RSA-PSS)
 * It is the value that should be returned by gnutls_pubkey_get_pk_algorithm() */
#define GNUTLS_PRIVKEY_INFO_PK_ALGO_BITS (1<<3)

/* returns information on the public key associated with userdata */
typedef int (*gnutls_privkey_info_func) (gnutls_privkey_t key, unsigned int flags, void *userdata);

int gnutls_pubkey_init(gnutls_pubkey_t * key);
void gnutls_pubkey_deinit(gnutls_pubkey_t key);

int gnutls_pubkey_verify_params(gnutls_pubkey_t key);

void gnutls_pubkey_set_pin_function(gnutls_pubkey_t key,
				    gnutls_pin_callback_t fn,
				    void *userdata);

int gnutls_pubkey_get_pk_algorithm(gnutls_pubkey_t key,
				   unsigned int *bits);

int
gnutls_pubkey_set_spki(gnutls_pubkey_t key,
			const gnutls_x509_spki_t spki,
			unsigned int flags);

int
gnutls_pubkey_get_spki(gnutls_pubkey_t key,
			const gnutls_x509_spki_t spki,
			unsigned int flags);

int gnutls_pubkey_import_x509(gnutls_pubkey_t key,
			      gnutls_x509_crt_t crt, unsigned int flags);
int gnutls_pubkey_import_x509_crq(gnutls_pubkey_t key,
				  gnutls_x509_crq_t crq,
				  unsigned int flags);
int gnutls_pubkey_import_pkcs11(gnutls_pubkey_t key,
				gnutls_pkcs11_obj_t obj,
				unsigned int flags);
int gnutls_pubkey_import_openpgp(gnutls_pubkey_t key,
				 gnutls_openpgp_crt_t crt,
				 unsigned int flags);

int gnutls_pubkey_import_openpgp_raw(gnutls_pubkey_t pkey,
				     const gnutls_datum_t * data,
				     gnutls_openpgp_crt_fmt_t
				     format,
				     const gnutls_openpgp_keyid_t
				     keyid, unsigned int flags);
int gnutls_pubkey_import_x509_raw(gnutls_pubkey_t pkey,
				  const gnutls_datum_t * data,
				  gnutls_x509_crt_fmt_t format,
				  unsigned int flags);

int
gnutls_pubkey_import_privkey(gnutls_pubkey_t key,
			     gnutls_privkey_t pkey,
			     unsigned int usage, unsigned int flags);

int
gnutls_pubkey_import_tpm_url(gnutls_pubkey_t pkey,
			     const char *url,
			     const char *srk_password, unsigned int flags);

int
gnutls_pubkey_import_url(gnutls_pubkey_t key, const char *url,
			 unsigned int flags);

int
gnutls_pubkey_import_tpm_raw(gnutls_pubkey_t pkey,
			     const gnutls_datum_t * fdata,
			     gnutls_tpmkey_fmt_t format,
			     const char *srk_password, unsigned int flags);

int gnutls_pubkey_get_preferred_hash_algorithm(gnutls_pubkey_t key,
					       gnutls_digest_algorithm_t
					       * hash, unsigned int *mand);

#define gnutls_pubkey_get_pk_rsa_raw gnutls_pubkey_export_rsa_raw
int gnutls_pubkey_export_rsa_raw(gnutls_pubkey_t key,
				 gnutls_datum_t * m, gnutls_datum_t * e);

int gnutls_pubkey_export_rsa_raw2(gnutls_pubkey_t key,
				  gnutls_datum_t * m, gnutls_datum_t * e,
				  unsigned flags);

#define gnutls_pubkey_get_pk_dsa_raw gnutls_pubkey_export_dsa_raw
int gnutls_pubkey_export_dsa_raw(gnutls_pubkey_t key,
				 gnutls_datum_t * p,
				 gnutls_datum_t * q,
				 gnutls_datum_t * g, gnutls_datum_t * y);

int gnutls_pubkey_export_dsa_raw2(gnutls_pubkey_t key,
				 gnutls_datum_t * p,
				 gnutls_datum_t * q,
				 gnutls_datum_t * g, gnutls_datum_t * y,
				 unsigned flags);

int gnutls_pubkey_export_ecc_raw2(gnutls_pubkey_t key,
				 gnutls_ecc_curve_t * curve,
				 gnutls_datum_t * x, gnutls_datum_t * y,
				 unsigned flags);

int gnutls_pubkey_export_gost_raw2(gnutls_pubkey_t key,
				   gnutls_ecc_curve_t * curve,
				   gnutls_digest_algorithm_t * digest,
				   gnutls_gost_paramset_t * paramset,
				   gnutls_datum_t * x, gnutls_datum_t * y,
				   unsigned int flags);

#define gnutls_pubkey_get_pk_ecc_raw gnutls_pubkey_export_ecc_raw
int gnutls_pubkey_export_ecc_raw(gnutls_pubkey_t key,
				 gnutls_ecc_curve_t * curve,
				 gnutls_datum_t * x, gnutls_datum_t * y);

#define gnutls_pubkey_get_pk_ecc_x962 gnutls_pubkey_export_ecc_x962
int gnutls_pubkey_export_ecc_x962(gnutls_pubkey_t key,
				  gnutls_datum_t * parameters,
				  gnutls_datum_t * ecpoint);

int gnutls_pubkey_export(gnutls_pubkey_t key,
			 gnutls_x509_crt_fmt_t format,
			 void *output_data, size_t * output_data_size);

int gnutls_pubkey_export2(gnutls_pubkey_t key,
			  gnutls_x509_crt_fmt_t format,
			  gnutls_datum_t * out);

int gnutls_pubkey_get_key_id(gnutls_pubkey_t key,
			     unsigned int flags,
			     unsigned char *output_data,
			     size_t * output_data_size);

int
gnutls_pubkey_get_openpgp_key_id(gnutls_pubkey_t key,
				 unsigned int flags,
				 unsigned char *output_data,
				 size_t * output_data_size,
				 unsigned int *subkey);

int gnutls_pubkey_get_key_usage(gnutls_pubkey_t key, unsigned int *usage);
int gnutls_pubkey_set_key_usage(gnutls_pubkey_t key, unsigned int usage);

int gnutls_pubkey_import(gnutls_pubkey_t key,
			 const gnutls_datum_t * data,
			 gnutls_x509_crt_fmt_t format);


#define gnutls_pubkey_import_pkcs11_url(key, url, flags) gnutls_pubkey_import_url(key, url, flags)

int gnutls_pubkey_import_dsa_raw(gnutls_pubkey_t key,
				 const gnutls_datum_t * p,
				 const gnutls_datum_t * q,
				 const gnutls_datum_t * g,
				 const gnutls_datum_t * y);
int gnutls_pubkey_import_rsa_raw(gnutls_pubkey_t key,
				 const gnutls_datum_t * m,
				 const gnutls_datum_t * e);

int
gnutls_pubkey_import_ecc_x962(gnutls_pubkey_t key,
			      const gnutls_datum_t * parameters,
			      const gnutls_datum_t * ecpoint);

int
gnutls_pubkey_import_ecc_raw(gnutls_pubkey_t key,
			     gnutls_ecc_curve_t curve,
			     const gnutls_datum_t * x,
			     const gnutls_datum_t * y);

int
gnutls_pubkey_import_gost_raw(gnutls_pubkey_t key,
			     gnutls_ecc_curve_t curve,
			     gnutls_digest_algorithm_t digest,
			     gnutls_gost_paramset_t paramset,
			     const gnutls_datum_t * x,
			     const gnutls_datum_t * y);

int
gnutls_pubkey_encrypt_data(gnutls_pubkey_t key,
			   unsigned int flags,
			   const gnutls_datum_t * plaintext,
			   gnutls_datum_t * ciphertext);

int gnutls_x509_crt_set_pubkey(gnutls_x509_crt_t crt, gnutls_pubkey_t key);

int gnutls_x509_crq_set_pubkey(gnutls_x509_crq_t crq, gnutls_pubkey_t key);

int
gnutls_pubkey_verify_hash2(gnutls_pubkey_t key,
			   gnutls_sign_algorithm_t algo,
			   unsigned int flags,
			   const gnutls_datum_t * hash,
			   const gnutls_datum_t * signature);

int
gnutls_pubkey_verify_data2(gnutls_pubkey_t pubkey,
			   gnutls_sign_algorithm_t algo,
			   unsigned int flags,
			   const gnutls_datum_t * data,
			   const gnutls_datum_t * signature);

/* Private key operations */

int gnutls_privkey_init(gnutls_privkey_t * key);
void gnutls_privkey_deinit(gnutls_privkey_t key);

/* macros to allow specifying a subgroup and group size in gnutls_privkey_generate()
 * and gnutls_x509_privkey_generate() */
#define GNUTLS_SUBGROUP_TO_BITS(group, subgroup) (unsigned int)((subgroup<<16)|(group))
#define GNUTLS_BITS_TO_SUBGROUP(bits) ((bits >> 16) & 0xFFFF)
#define GNUTLS_BITS_TO_GROUP(bits) (bits & 0xFFFF)
#define GNUTLS_BITS_HAVE_SUBGROUP(bits) ((bits) & 0xFFFF0000)

int
gnutls_privkey_generate (gnutls_privkey_t key,
                         gnutls_pk_algorithm_t algo, unsigned int bits,
                         unsigned int flags);
int
gnutls_privkey_generate2(gnutls_privkey_t pkey,
			 gnutls_pk_algorithm_t algo, unsigned int bits,
			 unsigned int flags, const gnutls_keygen_data_st *data, unsigned data_size);

int
gnutls_privkey_set_spki(gnutls_privkey_t key,
			const gnutls_x509_spki_t spki,
			unsigned int flags);

int
gnutls_privkey_get_spki(gnutls_privkey_t key,
			const gnutls_x509_spki_t spki,
			unsigned int flags);

int gnutls_privkey_verify_seed(gnutls_privkey_t key, gnutls_digest_algorithm_t, const void *seed, size_t seed_size);
int gnutls_privkey_get_seed(gnutls_privkey_t key, gnutls_digest_algorithm_t*, void *seed, size_t *seed_size);

int gnutls_privkey_verify_params(gnutls_privkey_t key);

void gnutls_privkey_set_flags(gnutls_privkey_t key, unsigned int flags);

void gnutls_privkey_set_pin_function (gnutls_privkey_t key,
                                      gnutls_pin_callback_t fn, void *userdata);

int gnutls_privkey_get_pk_algorithm(gnutls_privkey_t key,
				    unsigned int *bits);
gnutls_privkey_type_t gnutls_privkey_get_type(gnutls_privkey_t key);
int gnutls_privkey_status(gnutls_privkey_t key);

/**
 * gnutls_privkey_flags:
 * @GNUTLS_PRIVKEY_SIGN_FLAG_TLS1_RSA: Make an RSA signature on the hashed data as in the TLS protocol.
 * @GNUTLS_PRIVKEY_SIGN_FLAG_RSA_PSS: Make an RSA signature on the hashed data with the PSS padding.
 * @GNUTLS_PRIVKEY_FLAG_REPRODUCIBLE: Make a signature on the hashed data with reproducible parameters.
 *   For RSA-PSS, that means to use empty salt instead of random value. To
 *   verify a signature created using this flag, the corresponding SPKI needs
 *   to be set on the public key. Use gnutls_pubkey_set_spki() for that.
 *   For ECDSA/DSA, it uses the deterministic construction of random parameter
 *   according to RFC 6979. Note that this only supports the NIST curves and DSA
 *   subgroup bits up to 512.
 * @GNUTLS_PRIVKEY_IMPORT_AUTO_RELEASE: When importing a private key, automatically
 *   release it when the structure it was imported is released.
 * @GNUTLS_PRIVKEY_IMPORT_COPY: Copy required values during import.
 * @GNUTLS_PRIVKEY_DISABLE_CALLBACKS: The following flag disables call to PIN callbacks etc.
 *   Only relevant to TPM keys.
 * @GNUTLS_PRIVKEY_FLAG_PROVABLE: When generating a key involving prime numbers, use provable primes; a seed may be required.
 * @GNUTLS_PRIVKEY_FLAG_CA: The generated private key is going to be used as a CA (relevant for RSA-PSS keys).
 * @GNUTLS_PRIVKEY_FLAG_EXPORT_COMPAT: Keys generated or imported as provable require an extended format which cannot be read by previous versions
 *   of gnutls or other applications. By setting this flag the key will be exported in a backwards compatible way,
 *   even if the information about the seed used will be lost.
 *
 * Enumeration of different certificate import flags.
 */
typedef enum gnutls_privkey_flags {
	GNUTLS_PRIVKEY_IMPORT_AUTO_RELEASE = 1,
	GNUTLS_PRIVKEY_IMPORT_COPY = 1 << 1,
	GNUTLS_PRIVKEY_DISABLE_CALLBACKS = 1 << 2,
	GNUTLS_PRIVKEY_SIGN_FLAG_TLS1_RSA = 1 << 4,
	GNUTLS_PRIVKEY_FLAG_PROVABLE = 1 << 5,
	GNUTLS_PRIVKEY_FLAG_EXPORT_COMPAT = 1 << 6,
	GNUTLS_PRIVKEY_SIGN_FLAG_RSA_PSS = 1 << 7,
	GNUTLS_PRIVKEY_FLAG_REPRODUCIBLE = 1 << 8,
	GNUTLS_PRIVKEY_FLAG_CA = 1 << 9
} gnutls_privkey_flags_t;

int gnutls_privkey_import_pkcs11(gnutls_privkey_t pkey,
				 gnutls_pkcs11_privkey_t key,
				 unsigned int flags);
int gnutls_privkey_import_x509(gnutls_privkey_t pkey,
			       gnutls_x509_privkey_t key,
			       unsigned int flags);
int gnutls_privkey_import_openpgp(gnutls_privkey_t pkey,
				  gnutls_openpgp_privkey_t key,
				  unsigned int flags);

int gnutls_privkey_export_x509(gnutls_privkey_t pkey,
                               gnutls_x509_privkey_t * key);
int gnutls_privkey_export_openpgp(gnutls_privkey_t pkey,
                                  gnutls_openpgp_privkey_t * key);
int
gnutls_privkey_export_pkcs11(gnutls_privkey_t pkey,
                             gnutls_pkcs11_privkey_t *key);

int gnutls_privkey_import_openpgp_raw(gnutls_privkey_t pkey,
				      const gnutls_datum_t * data,
				      gnutls_openpgp_crt_fmt_t
				      format,
				      const gnutls_openpgp_keyid_t
				      keyid, const char *password);

int gnutls_privkey_import_x509_raw(gnutls_privkey_t pkey,
				   const gnutls_datum_t * data,
				   gnutls_x509_crt_fmt_t format,
				   const char *password,
				   unsigned int flags);

int
gnutls_privkey_import_tpm_raw(gnutls_privkey_t pkey,
			      const gnutls_datum_t * fdata,
			      gnutls_tpmkey_fmt_t format,
			      const char *srk_password,
			      const char *key_password,
			      unsigned int flags);

int
gnutls_privkey_import_tpm_url(gnutls_privkey_t pkey,
			      const char *url,
			      const char *srk_password,
			      const char *key_password,
			      unsigned int flags);

int gnutls_privkey_import_url(gnutls_privkey_t key,
			      const char *url, unsigned int flags);

#if 0
/* for documentation purposes */
int gnutls_privkey_import_pkcs11_url(gnutls_privkey_t key, const char *url);
#endif

#define gnutls_privkey_import_pkcs11_url(key, url) gnutls_privkey_import_url(key, url, 0)

int
gnutls_privkey_import_ext(gnutls_privkey_t pkey,
			  gnutls_pk_algorithm_t pk,
			  void *userdata,
			  gnutls_privkey_sign_func sign_func,
			  gnutls_privkey_decrypt_func
			  decrypt_func, unsigned int flags);

int
gnutls_privkey_import_ext2(gnutls_privkey_t pkey,
			   gnutls_pk_algorithm_t pk,
			   void *userdata,
			   gnutls_privkey_sign_func sign_func,
			   gnutls_privkey_decrypt_func
			   decrypt_func,
			   gnutls_privkey_deinit_func deinit_func,
			   unsigned int flags);

int
gnutls_privkey_import_ext3(gnutls_privkey_t pkey,
                           void *userdata,
                           gnutls_privkey_sign_func sign_func,
                           gnutls_privkey_decrypt_func decrypt_func,
                           gnutls_privkey_deinit_func deinit_func,
                           gnutls_privkey_info_func info_func,
                           unsigned int flags);

int
gnutls_privkey_import_ext4(gnutls_privkey_t pkey,
                           void *userdata,
                           gnutls_privkey_sign_data_func sign_data_func,
                           gnutls_privkey_sign_hash_func sign_hash_func,
                           gnutls_privkey_decrypt_func decrypt_func,
                           gnutls_privkey_deinit_func deinit_func,
                           gnutls_privkey_info_func info_func,
                           unsigned int flags);

int gnutls_privkey_import_dsa_raw(gnutls_privkey_t key,
				       const gnutls_datum_t * p,
				       const gnutls_datum_t * q,
				       const gnutls_datum_t * g,
				       const gnutls_datum_t * y,
				       const gnutls_datum_t * x);

int gnutls_privkey_import_rsa_raw(gnutls_privkey_t key,
					const gnutls_datum_t * m,
					const gnutls_datum_t * e,
					const gnutls_datum_t * d,
					const gnutls_datum_t * p,
					const gnutls_datum_t * q,
					const gnutls_datum_t * u,
					const gnutls_datum_t * e1,
					const gnutls_datum_t * e2);
int gnutls_privkey_import_ecc_raw(gnutls_privkey_t key,
				       gnutls_ecc_curve_t curve,
				       const gnutls_datum_t * x,
				       const gnutls_datum_t * y,
				       const gnutls_datum_t * k);

int gnutls_privkey_import_gost_raw(gnutls_privkey_t key,
				       gnutls_ecc_curve_t curve,
				       gnutls_digest_algorithm_t digest,
				       gnutls_gost_paramset_t paramset,
				       const gnutls_datum_t * x,
				       const gnutls_datum_t * y,
				       const gnutls_datum_t * k);


int gnutls_privkey_sign_data(gnutls_privkey_t signer,
			     gnutls_digest_algorithm_t hash,
			     unsigned int flags,
			     const gnutls_datum_t * data,
			     gnutls_datum_t * signature);

int gnutls_privkey_sign_data2(gnutls_privkey_t signer,
			      gnutls_sign_algorithm_t algo,
			      unsigned int flags,
			      const gnutls_datum_t * data,
			      gnutls_datum_t * signature);

#define gnutls_privkey_sign_raw_data(key, flags, data, sig) \
	gnutls_privkey_sign_hash ( key, 0, GNUTLS_PRIVKEY_SIGN_FLAG_TLS1_RSA, data, sig)

int gnutls_privkey_sign_hash(gnutls_privkey_t signer,
			     gnutls_digest_algorithm_t hash_algo,
			     unsigned int flags,
			     const gnutls_datum_t * hash_data,
			     gnutls_datum_t * signature);

int gnutls_privkey_sign_hash2(gnutls_privkey_t signer,
			      gnutls_sign_algorithm_t algo,
			      unsigned int flags,
			      const gnutls_datum_t * hash_data,
			      gnutls_datum_t * signature);

int gnutls_privkey_decrypt_data(gnutls_privkey_t key,
				unsigned int flags,
				const gnutls_datum_t * ciphertext,
				gnutls_datum_t * plaintext);

int gnutls_privkey_decrypt_data2(gnutls_privkey_t key,
				 unsigned int flags,
				 const gnutls_datum_t * ciphertext,
				 unsigned char * plaintext,
                                 size_t plaintext_size);

int
gnutls_privkey_export_rsa_raw(gnutls_privkey_t key,
				    gnutls_datum_t * m, gnutls_datum_t * e,
				    gnutls_datum_t * d, gnutls_datum_t * p,
				    gnutls_datum_t * q, gnutls_datum_t * u,
				    gnutls_datum_t * e1,
				    gnutls_datum_t * e2);

int
gnutls_privkey_export_rsa_raw2(gnutls_privkey_t key,
				    gnutls_datum_t * m, gnutls_datum_t * e,
				    gnutls_datum_t * d, gnutls_datum_t * p,
				    gnutls_datum_t * q, gnutls_datum_t * u,
				    gnutls_datum_t * e1,
				    gnutls_datum_t * e2, unsigned flags);

int
gnutls_privkey_export_dsa_raw(gnutls_privkey_t key,
			     gnutls_datum_t * p, gnutls_datum_t * q,
			     gnutls_datum_t * g, gnutls_datum_t * y,
			     gnutls_datum_t * x);

int
gnutls_privkey_export_dsa_raw2(gnutls_privkey_t key,
			     gnutls_datum_t * p, gnutls_datum_t * q,
			     gnutls_datum_t * g, gnutls_datum_t * y,
			     gnutls_datum_t * x, unsigned flags);

int
gnutls_privkey_export_ecc_raw(gnutls_privkey_t key,
				       gnutls_ecc_curve_t * curve,
				       gnutls_datum_t * x,
				       gnutls_datum_t * y,
				       gnutls_datum_t * k);

int
gnutls_privkey_export_ecc_raw2(gnutls_privkey_t key,
				       gnutls_ecc_curve_t * curve,
				       gnutls_datum_t * x,
				       gnutls_datum_t * y,
				       gnutls_datum_t * k,
				       unsigned flags);

int
gnutls_privkey_export_gost_raw2(gnutls_privkey_t key,
				       gnutls_ecc_curve_t * curve,
				       gnutls_digest_algorithm_t * digest,
				       gnutls_gost_paramset_t * paramset,
				       gnutls_datum_t * x,
				       gnutls_datum_t * y,
				       gnutls_datum_t * k,
				       unsigned flags);


int gnutls_x509_crt_privkey_sign(gnutls_x509_crt_t crt,
				 gnutls_x509_crt_t issuer,
				 gnutls_privkey_t issuer_key,
				 gnutls_digest_algorithm_t dig,
				 unsigned int flags);

int gnutls_x509_crl_privkey_sign(gnutls_x509_crl_t crl,
				 gnutls_x509_crt_t issuer,
				 gnutls_privkey_t issuer_key,
				 gnutls_digest_algorithm_t dig,
				 unsigned int flags);

int gnutls_x509_crq_privkey_sign(gnutls_x509_crq_t crq,
				 gnutls_privkey_t key,
				 gnutls_digest_algorithm_t dig,
				 unsigned int flags);

/**
 * gnutls_pcert_st:
 * @pubkey: public key of parsed certificate.
 * @cert: certificate itself of parsed certificate
 * @type: type of certificate, a #gnutls_certificate_type_t type.
 *
 * A parsed certificate.
 */
typedef struct gnutls_pcert_st {
	gnutls_pubkey_t pubkey;
	gnutls_datum_t cert;
	gnutls_certificate_type_t type;
} gnutls_pcert_st;

/* This flag is unused/ignored */
#define GNUTLS_PCERT_NO_CERT 1

int gnutls_pcert_import_x509(gnutls_pcert_st * pcert,
			     gnutls_x509_crt_t crt, unsigned int flags);

int gnutls_pcert_import_x509_list(gnutls_pcert_st * pcert,
				  gnutls_x509_crt_t *crt, unsigned *ncrt,
				  unsigned int flags);

int gnutls_pcert_export_x509(gnutls_pcert_st * pcert,
                             gnutls_x509_crt_t * crt);

int
gnutls_pcert_list_import_x509_raw(gnutls_pcert_st * pcerts,
				  unsigned int *pcert_max,
				  const gnutls_datum_t * data,
				  gnutls_x509_crt_fmt_t format,
				  unsigned int flags);

int gnutls_pcert_list_import_x509_file(gnutls_pcert_st *pcert_list,
				       unsigned *pcert_list_size,
				       const char *file,
				       gnutls_x509_crt_fmt_t format,
				       gnutls_pin_callback_t pin_fn,
				       void *pin_fn_userdata,
				       unsigned int flags);

int gnutls_pcert_import_x509_raw(gnutls_pcert_st * pcert,
				 const gnutls_datum_t * cert,
				 gnutls_x509_crt_fmt_t format,
				 unsigned int flags);

int gnutls_pcert_import_openpgp_raw(gnutls_pcert_st * pcert,
				    const gnutls_datum_t * cert,
				    gnutls_openpgp_crt_fmt_t
				    format,
				    gnutls_openpgp_keyid_t keyid,
				    unsigned int flags);

int gnutls_pcert_import_openpgp(gnutls_pcert_st * pcert,
				gnutls_openpgp_crt_t crt,
				unsigned int flags);

int gnutls_pcert_export_openpgp(gnutls_pcert_st * pcert,
                                gnutls_openpgp_crt_t * crt);

void gnutls_pcert_deinit(gnutls_pcert_st * pcert);

int gnutls_pcert_import_rawpk(gnutls_pcert_st* pcert,
			     gnutls_pubkey_t key, unsigned int flags);

int gnutls_pcert_import_rawpk_raw(gnutls_pcert_st* pcert,
				    const gnutls_datum_t* rawpubkey,
				    gnutls_x509_crt_fmt_t format,
				    unsigned int key_usage, unsigned int flags);

/* For certificate credentials */
	/* This is the same as gnutls_certificate_retrieve_function()
	 * but retrieves a gnutls_pcert_st which requires much less processing
	 * within the library.
	 */
typedef int gnutls_certificate_retrieve_function2(gnutls_session_t,
				  const gnutls_datum_t *req_ca_rdn,
				  int nreqs,
				  const gnutls_pk_algorithm_t *pk_algos,
				  int pk_algos_length,
				  gnutls_pcert_st**,
				  unsigned int *pcert_length,
				  gnutls_privkey_t *privkey);


void gnutls_certificate_set_retrieve_function2
    (gnutls_certificate_credentials_t cred,
     gnutls_certificate_retrieve_function2 *func);

struct gnutls_cert_retr_st {
	unsigned version; /* set to 1 */
	gnutls_certificate_credentials_t cred;
	const gnutls_datum_t *req_ca_rdn;
	unsigned nreqs;
	const gnutls_pk_algorithm_t *pk_algos;
	unsigned pk_algos_length;

	/* other fields may be added if version is > 1 */
	unsigned char padding[64];
};

/* When the callback sets this value, gnutls will deinitialize the given
 * values after use */
#define GNUTLS_CERT_RETR_DEINIT_ALL 1

typedef int gnutls_certificate_retrieve_function3(
				gnutls_session_t,
				const struct gnutls_cert_retr_st *info,
				gnutls_pcert_st **certs,
				unsigned int *certs_length,
				gnutls_ocsp_data_st **ocsp,
				unsigned int *ocsp_length,
				gnutls_privkey_t *privkey,
				unsigned int *flags);


void gnutls_certificate_set_retrieve_function3
    (gnutls_certificate_credentials_t cred,
     gnutls_certificate_retrieve_function3 *func);

int
gnutls_certificate_set_key(gnutls_certificate_credentials_t res,
			   const char **names,
			   int names_size,
			   gnutls_pcert_st * pcert_list,
			   int pcert_list_size, gnutls_privkey_t key);

int
gnutls_pubkey_print(gnutls_pubkey_t pubkey,
		    gnutls_certificate_print_formats_t format,
		    gnutls_datum_t * out);

/* *INDENT-OFF* */
#ifdef __cplusplus
}
#endif
/* *INDENT-ON* */

#endif /* GNUTLS_ABSTRACT_H */
