/*
 * Copyright (C) 2008-2012 Free Software Foundation, Inc.
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

#ifndef GNUTLS_CRYPTO_H
#define GNUTLS_CRYPTO_H

#include <gnutls/gnutls.h>

/* *INDENT-OFF* */
#ifdef __cplusplus
extern "C" {
#endif
/* *INDENT-ON* */

typedef struct api_cipher_hd_st *gnutls_cipher_hd_t;

int gnutls_cipher_init(gnutls_cipher_hd_t * handle,
		       gnutls_cipher_algorithm_t cipher,
		       const gnutls_datum_t * key,
		       const gnutls_datum_t * iv);
int gnutls_cipher_encrypt(const gnutls_cipher_hd_t handle,
			  void *text, size_t textlen);
int gnutls_cipher_decrypt(const gnutls_cipher_hd_t handle,
			  void *ciphertext, size_t ciphertextlen);
int gnutls_cipher_decrypt2(gnutls_cipher_hd_t handle,
			   const void *ciphertext,
			   size_t ciphertextlen, void *text,
			   size_t textlen);
int gnutls_cipher_encrypt2(gnutls_cipher_hd_t handle,
			   const void *text, size_t textlen,
			   void *ciphertext, size_t ciphertextlen);

void gnutls_cipher_set_iv(gnutls_cipher_hd_t handle, void *iv,
			  size_t ivlen);

int gnutls_cipher_tag(gnutls_cipher_hd_t handle, void *tag,
		      size_t tag_size);
int gnutls_cipher_add_auth(gnutls_cipher_hd_t handle,
			   const void *text, size_t text_size);

void gnutls_cipher_deinit(gnutls_cipher_hd_t handle);
unsigned gnutls_cipher_get_block_size(gnutls_cipher_algorithm_t algorithm) __GNUTLS_CONST__;
unsigned gnutls_cipher_get_iv_size(gnutls_cipher_algorithm_t algorithm) __GNUTLS_CONST__;
unsigned gnutls_cipher_get_tag_size(gnutls_cipher_algorithm_t algorithm) __GNUTLS_CONST__;

/* AEAD API
 */
typedef struct api_aead_cipher_hd_st *gnutls_aead_cipher_hd_t;

int gnutls_aead_cipher_init(gnutls_aead_cipher_hd_t * handle,
			    gnutls_cipher_algorithm_t cipher,
			    const gnutls_datum_t * key);
int
gnutls_aead_cipher_decrypt(gnutls_aead_cipher_hd_t handle,
			   const void *nonce, size_t nonce_len,
			   const void *auth, size_t auth_len,
			   size_t tag_size,
			   const void *ctext, size_t ctext_len,
			   void *ptext, size_t *ptext_len);
int
gnutls_aead_cipher_encrypt(gnutls_aead_cipher_hd_t handle,
			   const void *nonce, size_t nonce_len,
			   const void *auth, size_t auth_len,
			   size_t tag_size,
			   const void *ptext, size_t ptext_len,
			   void *ctext, size_t *ctext_len);

int
gnutls_aead_cipher_encryptv(gnutls_aead_cipher_hd_t handle,
			    const void *nonce, size_t nonce_len,
			    const giovec_t *auth_iov, int auth_iovcnt,
			    size_t tag_size,
			    const giovec_t *iov, int iovcnt,
			    void *ctext, size_t *ctext_len);

int
gnutls_aead_cipher_encryptv2(gnutls_aead_cipher_hd_t handle,
			     const void *nonce, size_t nonce_len,
			     const giovec_t *auth_iov, int auth_iovcnt,
			     const giovec_t *iov, int iovcnt,
			     void *tag, size_t *tag_size);

int
gnutls_aead_cipher_decryptv2(gnutls_aead_cipher_hd_t handle,
			     const void *nonce, size_t nonce_len,
			     const giovec_t *auth_iov, int auth_iovcnt,
			     const giovec_t *iov, int iovcnt,
			     void *tag, size_t tag_size);

void gnutls_aead_cipher_deinit(gnutls_aead_cipher_hd_t handle);

/* Hash - MAC API */

typedef struct hash_hd_st *gnutls_hash_hd_t;
typedef struct hmac_hd_st *gnutls_hmac_hd_t;

size_t gnutls_mac_get_nonce_size(gnutls_mac_algorithm_t algorithm) __GNUTLS_CONST__;
int gnutls_hmac_init(gnutls_hmac_hd_t * dig,
		     gnutls_mac_algorithm_t algorithm,
		     const void *key, size_t keylen);
void gnutls_hmac_set_nonce(gnutls_hmac_hd_t handle,
			   const void *nonce, size_t nonce_len);
int gnutls_hmac(gnutls_hmac_hd_t handle, const void *text, size_t textlen);
void gnutls_hmac_output(gnutls_hmac_hd_t handle, void *digest);
void gnutls_hmac_deinit(gnutls_hmac_hd_t handle, void *digest);
unsigned gnutls_hmac_get_len(gnutls_mac_algorithm_t algorithm) __GNUTLS_CONST__;
unsigned gnutls_hmac_get_key_size(gnutls_mac_algorithm_t algorithm) __GNUTLS_CONST__;
int gnutls_hmac_fast(gnutls_mac_algorithm_t algorithm,
		     const void *key, size_t keylen,
		     const void *text, size_t textlen, void *digest);
gnutls_hmac_hd_t gnutls_hmac_copy(gnutls_hmac_hd_t handle);

int gnutls_hash_init(gnutls_hash_hd_t * dig,
		     gnutls_digest_algorithm_t algorithm);
int gnutls_hash(gnutls_hash_hd_t handle, const void *text, size_t textlen);
void gnutls_hash_output(gnutls_hash_hd_t handle, void *digest);
void gnutls_hash_deinit(gnutls_hash_hd_t handle, void *digest);
unsigned gnutls_hash_get_len(gnutls_digest_algorithm_t algorithm) __GNUTLS_CONST__;
int gnutls_hash_fast(gnutls_digest_algorithm_t algorithm,
		     const void *text, size_t textlen, void *digest);
gnutls_hash_hd_t gnutls_hash_copy(gnutls_hash_hd_t handle);

/* KDF API */

int gnutls_hkdf_extract(gnutls_mac_algorithm_t mac,
			const gnutls_datum_t *key,
			const gnutls_datum_t *salt,
			void *output);

int gnutls_hkdf_expand(gnutls_mac_algorithm_t mac,
		       const gnutls_datum_t *key,
		       const gnutls_datum_t *info,
		       void *output, size_t length);

int gnutls_pbkdf2(gnutls_mac_algorithm_t mac,
		  const gnutls_datum_t *key,
		  const gnutls_datum_t *salt,
		  unsigned iter_count,
		  void *output, size_t length);

/* register ciphers */


/**
 * gnutls_rnd_level_t:
 * @GNUTLS_RND_NONCE: Non-predictable random number.  Fatal in parts
 *   of session if broken, i.e., vulnerable to statistical analysis.
 * @GNUTLS_RND_RANDOM: Pseudo-random cryptographic random number.
 *   Fatal in session if broken. Example use: temporal keys.
 * @GNUTLS_RND_KEY: Fatal in many sessions if broken. Example use:
 *   Long-term keys.
 *
 * Enumeration of random quality levels.
 */
typedef enum gnutls_rnd_level {
	GNUTLS_RND_NONCE = 0,
	GNUTLS_RND_RANDOM = 1,
	GNUTLS_RND_KEY = 2
} gnutls_rnd_level_t;

int gnutls_rnd(gnutls_rnd_level_t level, void *data, size_t len);

void gnutls_rnd_refresh(void);


/* API to override ciphers and MAC algorithms 
 */

typedef int (*gnutls_cipher_init_func) (gnutls_cipher_algorithm_t, void **ctx, int enc);
typedef int (*gnutls_cipher_setkey_func) (void *ctx, const void *key, size_t keysize);
/* old style ciphers */
typedef int (*gnutls_cipher_setiv_func) (void *ctx, const void *iv, size_t ivsize);
typedef int (*gnutls_cipher_getiv_func) (void *ctx, void *iv, size_t ivsize);
typedef int (*gnutls_cipher_encrypt_func) (void *ctx, const void *plain, size_t plainsize,
				void *encr, size_t encrsize);
typedef int (*gnutls_cipher_decrypt_func) (void *ctx, const void *encr, size_t encrsize,
			void *plain, size_t plainsize);

/* aead ciphers */
typedef int (*gnutls_cipher_auth_func) (void *ctx, const void *data, size_t datasize);
typedef void (*gnutls_cipher_tag_func) (void *ctx, void *tag, size_t tagsize);

typedef int (*gnutls_cipher_aead_encrypt_func) (void *ctx,
			const void *nonce, size_t noncesize,
			const void *auth, size_t authsize,
			size_t tag_size,
			const void *plain, size_t plainsize,
			void *encr, size_t encrsize);
typedef int (*gnutls_cipher_aead_decrypt_func) (void *ctx,
			const void *nonce, size_t noncesize,
			const void *auth, size_t authsize,
			size_t tag_size,
			const void *encr, size_t encrsize,
			void *plain, size_t plainsize);
typedef void (*gnutls_cipher_deinit_func) (void *ctx);

int
gnutls_crypto_register_cipher(gnutls_cipher_algorithm_t algorithm,
			      int priority,
			      gnutls_cipher_init_func init,
			      gnutls_cipher_setkey_func setkey,
			      gnutls_cipher_setiv_func setiv,
			      gnutls_cipher_encrypt_func encrypt,
			      gnutls_cipher_decrypt_func decrypt,
			      gnutls_cipher_deinit_func deinit)
			      _GNUTLS_GCC_ATTR_DEPRECATED;

int
gnutls_crypto_register_aead_cipher(gnutls_cipher_algorithm_t algorithm,
			      int priority,
			      gnutls_cipher_init_func init,
			      gnutls_cipher_setkey_func setkey,
			      gnutls_cipher_aead_encrypt_func aead_encrypt,
			      gnutls_cipher_aead_decrypt_func aead_decrypt,
			      gnutls_cipher_deinit_func deinit)
			      _GNUTLS_GCC_ATTR_DEPRECATED;

typedef int (*gnutls_mac_init_func) (gnutls_mac_algorithm_t, void **ctx);
typedef int (*gnutls_mac_setkey_func) (void *ctx, const void *key, size_t keysize);
typedef int (*gnutls_mac_setnonce_func) (void *ctx, const void *nonce, size_t noncesize);
typedef int (*gnutls_mac_hash_func) (void *ctx, const void *text, size_t textsize);
typedef int (*gnutls_mac_output_func) (void *src_ctx, void *digest, size_t digestsize);
typedef void (*gnutls_mac_deinit_func) (void *ctx);
typedef int (*gnutls_mac_fast_func) (gnutls_mac_algorithm_t, const void *nonce,
		     size_t nonce_size, const void *key, size_t keysize,
		     const void *text, size_t textsize, void *digest);
typedef void *(*gnutls_mac_copy_func) (const void *ctx);

int
gnutls_crypto_register_mac(gnutls_mac_algorithm_t mac,
			   int priority,
			   gnutls_mac_init_func init,
			   gnutls_mac_setkey_func setkey,
			   gnutls_mac_setnonce_func setnonce,
			   gnutls_mac_hash_func hash,
			   gnutls_mac_output_func output,
			   gnutls_mac_deinit_func deinit,
			   gnutls_mac_fast_func hash_fast)
			   _GNUTLS_GCC_ATTR_DEPRECATED;

typedef int (*gnutls_digest_init_func) (gnutls_digest_algorithm_t, void **ctx);
typedef int (*gnutls_digest_hash_func) (void *ctx, const void *text, size_t textsize);
typedef int (*gnutls_digest_output_func) (void *src_ctx, void *digest, size_t digestsize);
typedef void (*gnutls_digest_deinit_func) (void *ctx);
typedef int (*gnutls_digest_fast_func) (gnutls_digest_algorithm_t,
		     const void *text, size_t textsize, void *digest);
typedef void *(*gnutls_digest_copy_func) (const void *ctx);

int
gnutls_crypto_register_digest(gnutls_digest_algorithm_t digest,
			   int priority,
			   gnutls_digest_init_func init,
			   gnutls_digest_hash_func hash,
			   gnutls_digest_output_func output,
			   gnutls_digest_deinit_func deinit,
			   gnutls_digest_fast_func hash_fast)
			   _GNUTLS_GCC_ATTR_DEPRECATED;

/* RSA-PKCS#1 1.5 helper functions */
int
gnutls_encode_ber_digest_info(gnutls_digest_algorithm_t hash,
			      const gnutls_datum_t * digest,
			      gnutls_datum_t * output);

int
gnutls_decode_ber_digest_info(const gnutls_datum_t * info,
			      gnutls_digest_algorithm_t *hash,
			      unsigned char *digest, unsigned int *digest_size);

int gnutls_decode_rs_value(const gnutls_datum_t * sig_value, gnutls_datum_t *r, gnutls_datum_t *s);
int gnutls_encode_rs_value(gnutls_datum_t * sig_value, const gnutls_datum_t * r, const gnutls_datum_t * s);

int gnutls_encode_gost_rs_value(gnutls_datum_t * sig_value, const gnutls_datum_t * r, const gnutls_datum_t  *s);
int gnutls_decode_gost_rs_value(const gnutls_datum_t * sig_value, gnutls_datum_t * r, gnutls_datum_t * s);

/* *INDENT-OFF* */
#ifdef __cplusplus
}
#endif
/* *INDENT-ON* */

#endif /* GNUTLS_CRYPTO_H */
