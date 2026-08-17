/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _UTILMODT_H_
#define _UTILMODT_H_ 1

/*
 * these are SECMOD flags that would normally be in secmodt.h, but are needed
 * for the parser in util. Fort this reason we preserve the SECMOD names.
 */
#define SECMOD_RSA_FLAG 0x00000001L
#define SECMOD_DSA_FLAG 0x00000002L
#define SECMOD_RC2_FLAG 0x00000004L
#define SECMOD_RC4_FLAG 0x00000008L
#define SECMOD_DES_FLAG 0x00000010L
#define SECMOD_DH_FLAG 0x00000020L
#define SECMOD_FORTEZZA_FLAG 0x00000040L
#define SECMOD_RC5_FLAG 0x00000080L
#define SECMOD_SHA1_FLAG 0x00000100L
#define SECMOD_MD5_FLAG 0x00000200L
#define SECMOD_MD2_FLAG 0x00000400L
#define SECMOD_SSL_FLAG 0x00000800L
#define SECMOD_TLS_FLAG 0x00001000L
#define SECMOD_AES_FLAG 0x00002000L
#define SECMOD_SHA256_FLAG 0x00004000L   /* also for SHA224 */
#define SECMOD_SHA512_FLAG 0x00008000L   /* also for SHA384 */
#define SECMOD_CAMELLIA_FLAG 0x00010000L /* = PUBLIC_MECH_CAMELLIA_FLAG */
#define SECMOD_SEED_FLAG 0x00020000L
#define SECMOD_ECC_FLAG 0x00040000L
/* reserved bit for future, do not use */
#define SECMOD_RESERVED_FLAG 0X08000000L
#define SECMOD_FRIENDLY_FLAG 0x10000000L
#define SECMOD_RANDOM_FLAG 0x80000000L

#define PK11_OWN_PW_DEFAULTS 0x20000000L
#define PK11_DISABLE_FLAG 0x40000000L

/* need to make SECMOD and PK11 prefixes consistent. */
#define SECMOD_OWN_PW_DEFAULTS PK11_OWN_PW_DEFAULTS
#define SECMOD_DISABLE_FLAG PK11_DISABLE_FLAG

#endif /* _UTILMODT_H_ */
