/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * ciferfam.h - cipher familie IDs used for configuring ciphers for export
 *              control
 */

#ifndef _CIFERFAM_H_
#define _CIFERFAM_H_

#include "utilrename.h"
/* Cipher Suite "Families" */
#define CIPHER_FAMILY_PKCS12 "PKCS12"
#define CIPHER_FAMILY_SMIME "SMIME"
#define CIPHER_FAMILY_SSL2 "SSLv2" /* deprecated */
#define CIPHER_FAMILY_SSL3 "SSLv3"
#define CIPHER_FAMILY_SSL "SSL"
#define CIPHER_FAMILY_ALL ""
#define CIPHER_FAMILY_UNKNOWN "UNKNOWN"

#define CIPHER_FAMILYID_MASK 0xFFFF0000L
#define CIPHER_FAMILYID_SSL 0x00000000L
#define CIPHER_FAMILYID_SMIME 0x00010000L
#define CIPHER_FAMILYID_PKCS12 0x00020000L

/* SMIME "Cipher Suites" */
/*
 * Note that it is assumed that the cipher number itself can be used
 * as a bit position in a mask, and that mask is currently 32 bits wide.
 * So, if you want to add a cipher that is greater than 0037, secmime.c
 * needs to be made smarter at the same time.
 */
#define SMIME_RC2_CBC_40 (CIPHER_FAMILYID_SMIME | 0001)
#define SMIME_RC2_CBC_64 (CIPHER_FAMILYID_SMIME | 0002)
#define SMIME_RC2_CBC_128 (CIPHER_FAMILYID_SMIME | 0003)
#define SMIME_DES_CBC_56 (CIPHER_FAMILYID_SMIME | 0011)
#define SMIME_DES_EDE3_168 (CIPHER_FAMILYID_SMIME | 0012)
#define SMIME_AES_CBC_128 (CIPHER_FAMILYID_SMIME | 0013)
#define SMIME_AES_CBC_256 (CIPHER_FAMILYID_SMIME | 0014)
#define SMIME_RC5PAD_64_16_40 (CIPHER_FAMILYID_SMIME | 0021)
#define SMIME_RC5PAD_64_16_64 (CIPHER_FAMILYID_SMIME | 0022)
#define SMIME_RC5PAD_64_16_128 (CIPHER_FAMILYID_SMIME | 0023)
#define SMIME_FORTEZZA (CIPHER_FAMILYID_SMIME | 0031)

/* PKCS12 "Cipher Suites" */

#define PKCS12_RC2_CBC_40 (CIPHER_FAMILYID_PKCS12 | 0001)
#define PKCS12_RC2_CBC_128 (CIPHER_FAMILYID_PKCS12 | 0002)
#define PKCS12_RC4_40 (CIPHER_FAMILYID_PKCS12 | 0011)
#define PKCS12_RC4_128 (CIPHER_FAMILYID_PKCS12 | 0012)
#define PKCS12_DES_56 (CIPHER_FAMILYID_PKCS12 | 0021)
#define PKCS12_DES_EDE3_168 (CIPHER_FAMILYID_PKCS12 | 0022)
#define PKCS12_AES_CBC_128 (CIPHER_FAMILYID_PKCS12 | 0031)
#define PKCS12_AES_CBC_192 (CIPHER_FAMILYID_PKCS12 | 0032)
#define PKCS12_AES_CBC_256 (CIPHER_FAMILYID_PKCS12 | 0033)

/* SMIME version numbers are negative, to avoid colliding with SSL versions */
#define SMIME_LIBRARY_VERSION_1_0 -0x0100

#endif /* _CIFERFAM_H_ */
