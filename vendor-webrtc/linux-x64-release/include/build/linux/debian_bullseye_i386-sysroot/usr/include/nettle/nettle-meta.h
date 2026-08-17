/* nettle-meta.h

   Information about algorithms.

   Copyright (C) 2002, 2014, 2020 Niels Möller

   This file is part of GNU Nettle.

   GNU Nettle is free software: you can redistribute it and/or
   modify it under the terms of either:

     * the GNU Lesser General Public License as published by the Free
       Software Foundation; either version 3 of the License, or (at your
       option) any later version.

   or

     * the GNU General Public License as published by the Free
       Software Foundation; either version 2 of the License, or (at your
       option) any later version.

   or both in parallel, as here.

   GNU Nettle is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received copies of the GNU General Public License and
   the GNU Lesser General Public License along with this program.  If
   not, see http://www.gnu.org/licenses/.
*/

#ifndef NETTLE_META_H_INCLUDED
#define NETTLE_META_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif


struct nettle_cipher
{
  const char *name;
  
  unsigned context_size;
  
  /* Zero for stream ciphers */
  unsigned block_size;

  /* Suggested key size; other sizes are sometimes possible. */
  unsigned key_size;

  nettle_set_key_func *set_encrypt_key;
  nettle_set_key_func *set_decrypt_key;

  nettle_cipher_func *encrypt;
  nettle_cipher_func *decrypt;
};

/* null-terminated list of ciphers implemented by this version of nettle */
const struct nettle_cipher * const * _NETTLE_ATTRIBUTE_PURE
nettle_get_ciphers (void);

#define nettle_ciphers (nettle_get_ciphers())

extern const struct nettle_cipher nettle_aes128;
extern const struct nettle_cipher nettle_aes192;
extern const struct nettle_cipher nettle_aes256;

extern const struct nettle_cipher nettle_camellia128;
extern const struct nettle_cipher nettle_camellia192;
extern const struct nettle_cipher nettle_camellia256;

extern const struct nettle_cipher nettle_cast128;

extern const struct nettle_cipher nettle_serpent128;
extern const struct nettle_cipher nettle_serpent192;
extern const struct nettle_cipher nettle_serpent256;

extern const struct nettle_cipher nettle_twofish128;
extern const struct nettle_cipher nettle_twofish192;
extern const struct nettle_cipher nettle_twofish256;

extern const struct nettle_cipher nettle_arctwo40;
extern const struct nettle_cipher nettle_arctwo64;
extern const struct nettle_cipher nettle_arctwo128;
extern const struct nettle_cipher nettle_arctwo_gutmann128;

struct nettle_hash
{
  const char *name;

  /* Size of the context struct */
  unsigned context_size;

  /* Size of digests */
  unsigned digest_size;
  
  /* Internal block size */
  unsigned block_size;

  nettle_hash_init_func *init;
  nettle_hash_update_func *update;
  nettle_hash_digest_func *digest;
};

#define _NETTLE_HASH(name, NAME) {		\
 #name,						\
 sizeof(struct name##_ctx),			\
 NAME##_DIGEST_SIZE,				\
 NAME##_BLOCK_SIZE,				\
 (nettle_hash_init_func *) name##_init,		\
 (nettle_hash_update_func *) name##_update,	\
 (nettle_hash_digest_func *) name##_digest	\
} 

/* null-terminated list of digests implemented by this version of nettle */
const struct nettle_hash * const * _NETTLE_ATTRIBUTE_PURE
nettle_get_hashes (void);

#define nettle_hashes (nettle_get_hashes())

const struct nettle_hash *
nettle_lookup_hash (const char *name);

extern const struct nettle_hash nettle_md2;
extern const struct nettle_hash nettle_md4;
extern const struct nettle_hash nettle_md5;
extern const struct nettle_hash nettle_gosthash94;
extern const struct nettle_hash nettle_gosthash94cp;
extern const struct nettle_hash nettle_ripemd160;
extern const struct nettle_hash nettle_sha1;
extern const struct nettle_hash nettle_sha224;
extern const struct nettle_hash nettle_sha256;
extern const struct nettle_hash nettle_sha384;
extern const struct nettle_hash nettle_sha512;
extern const struct nettle_hash nettle_sha512_224;
extern const struct nettle_hash nettle_sha512_256;
extern const struct nettle_hash nettle_sha3_224;
extern const struct nettle_hash nettle_sha3_256;
extern const struct nettle_hash nettle_sha3_384;
extern const struct nettle_hash nettle_sha3_512;
extern const struct nettle_hash nettle_streebog256;
extern const struct nettle_hash nettle_streebog512;

struct nettle_mac
{
  const char *name;

  /* Size of the context struct */
  unsigned context_size;

  /* Size of digests */
  unsigned digest_size;

  /* Key size */
  unsigned key_size;

  nettle_set_key_func *set_key;
  nettle_hash_update_func *update;
  nettle_hash_digest_func *digest;
};

struct nettle_aead
{
  const char *name;
  
  unsigned context_size;
  /* Block size for encrypt and decrypt. */
  unsigned block_size;
  unsigned key_size;
  unsigned nonce_size;
  unsigned digest_size;

  nettle_set_key_func *set_encrypt_key;
  nettle_set_key_func *set_decrypt_key;
  nettle_set_key_func *set_nonce;
  nettle_hash_update_func *update;
  nettle_crypt_func *encrypt;
  nettle_crypt_func *decrypt;
  /* FIXME: Drop length argument? */
  nettle_hash_digest_func *digest;
};

/* null-terminated list of aead constructions implemented by this
   version of nettle */
const struct nettle_aead * const * _NETTLE_ATTRIBUTE_PURE
nettle_get_aeads (void);

#define nettle_aeads (nettle_get_aeads())

extern const struct nettle_aead nettle_gcm_aes128;
extern const struct nettle_aead nettle_gcm_aes192;
extern const struct nettle_aead nettle_gcm_aes256;
extern const struct nettle_aead nettle_gcm_camellia128;
extern const struct nettle_aead nettle_gcm_camellia256;
extern const struct nettle_aead nettle_eax_aes128;
extern const struct nettle_aead nettle_chacha_poly1305;

struct nettle_armor
{
  const char *name;
  unsigned encode_context_size;
  unsigned decode_context_size;

  unsigned encode_final_length;

  nettle_armor_init_func *encode_init;
  nettle_armor_length_func *encode_length;
  nettle_armor_encode_update_func *encode_update;
  nettle_armor_encode_final_func *encode_final;
  
  nettle_armor_init_func *decode_init;
  nettle_armor_length_func *decode_length;
  nettle_armor_decode_update_func *decode_update;
  nettle_armor_decode_final_func *decode_final;
};

#define _NETTLE_ARMOR(name, NAME) {				\
  #name,							\
  sizeof(struct name##_encode_ctx),				\
  sizeof(struct name##_decode_ctx),				\
  NAME##_ENCODE_FINAL_LENGTH,					\
  (nettle_armor_init_func *) name##_encode_init,		\
  (nettle_armor_length_func *) name##_encode_length,		\
  (nettle_armor_encode_update_func *) name##_encode_update,	\
  (nettle_armor_encode_final_func *) name##_encode_final,	\
  (nettle_armor_init_func *) name##_decode_init,		\
  (nettle_armor_length_func *) name##_decode_length,		\
  (nettle_armor_decode_update_func *) name##_decode_update,	\
  (nettle_armor_decode_final_func *) name##_decode_final,	\
}

#define _NETTLE_ARMOR_0(name, NAME) {				\
  #name,							\
  0,								\
  sizeof(struct name##_decode_ctx),				\
  NAME##_ENCODE_FINAL_LENGTH,					\
  (nettle_armor_init_func *) name##_encode_init,		\
  (nettle_armor_length_func *) name##_encode_length,		\
  (nettle_armor_encode_update_func *) name##_encode_update,	\
  (nettle_armor_encode_final_func *) name##_encode_final,	\
  (nettle_armor_init_func *) name##_decode_init,		\
  (nettle_armor_length_func *) name##_decode_length,		\
  (nettle_armor_decode_update_func *) name##_decode_update,	\
  (nettle_armor_decode_final_func *) name##_decode_final,	\
}

/* null-terminated list of armor schemes implemented by this version of nettle */
const struct nettle_armor * const * _NETTLE_ATTRIBUTE_PURE
nettle_get_armors (void);

#define nettle_armors (nettle_get_armors())

extern const struct nettle_armor nettle_base64;
extern const struct nettle_armor nettle_base64url;
extern const struct nettle_armor nettle_base16;

#define _NETTLE_HMAC(name, HASH) {		\
  #name,					\
  sizeof(struct name##_ctx),			\
  HASH##_DIGEST_SIZE,				\
  HASH##_DIGEST_SIZE,				\
  name##_set_key_wrapper,			\
  (nettle_hash_update_func *) name##_update,	\
  (nettle_hash_digest_func *) name##_digest,	\
}

/* null-terminated list of macs implemented by this
   version of nettle */
const struct nettle_mac * const * _NETTLE_ATTRIBUTE_PURE
nettle_get_macs (void);

#define nettle_macs (nettle_get_macs())

extern const struct nettle_mac nettle_cmac_aes128;
extern const struct nettle_mac nettle_cmac_aes256;
extern const struct nettle_mac nettle_cmac_des3;

/* HMAC variants with key size = digest size */
extern const struct nettle_mac nettle_hmac_md5;
extern const struct nettle_mac nettle_hmac_ripemd160;
extern const struct nettle_mac nettle_hmac_sha1;
extern const struct nettle_mac nettle_hmac_sha224;
extern const struct nettle_mac nettle_hmac_sha256;
extern const struct nettle_mac nettle_hmac_sha384;
extern const struct nettle_mac nettle_hmac_sha512;
extern const struct nettle_mac nettle_hmac_streebog256;
extern const struct nettle_mac nettle_hmac_streebog512;

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_META_H_INCLUDED */
