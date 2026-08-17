/*
 * Copyright (c) 2012 Red Hat Inc.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 *     * Redistributions of source code must retain the above
 *       copyright notice, this list of conditions and the
 *       following disclaimer.
 *     * Redistributions in binary form must reproduce the
 *       above copyright notice, this list of conditions and
 *       the following disclaimer in the documentation and/or
 *       other materials provided with the distribution.
 *     * The names of contributors to this software may not be
 *       used to endorse or promote products derived from this
 *       software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 * BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS
 * OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED
 * AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
 * THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 *
 * Author: Stef Walter <stefw@redhat.com>
 */

#ifndef PKCS11_X_H_
#define PKCS11_X_H_ 1

#if defined(__cplusplus)
extern "C" {
#endif

/* -------------------------------------------------------------------
 * NSS TRUST OBJECTS
 *
 * And related, non-standard
 */

/* Define this if you want the NSS specific symbols */
#define CRYPTOKI_NSS_VENDOR_DEFINED 1
#ifdef CRYPTOKI_NSS_VENDOR_DEFINED

/* Various NSS objects */
#define CKO_NSS_CRL                     0xce534351UL
#define CKO_NSS_SMIME                   0xce534352UL
#define CKO_NSS_TRUST                   0xce534353UL
#define CKO_NSS_BUILTIN_ROOT_LIST       0xce534354UL
#define CKO_NSS_NEWSLOT                 0xce534355UL
#define CKO_NSS_DELSLOT                 0xce534356UL

/* Various NSS key types */
#define CKK_NSS_PKCS8                   0xce534351UL

/* Various NSS attributes */
#define CKA_NSS_URL                     0xce534351UL
#define CKA_NSS_EMAIL                   0xce534352UL
#define CKA_NSS_SMIME_INFO              0xce534353UL
#define CKA_NSS_SMIME_TIMESTAMP         0xce534354UL
#define CKA_NSS_PKCS8_SALT              0xce534355UL
#define CKA_NSS_PASSWORD_CHECK          0xce534356UL
#define CKA_NSS_EXPIRES                 0xce534357UL
#define CKA_NSS_KRL                     0xce534358UL
#define CKA_NSS_PQG_COUNTER             0xce534364UL
#define CKA_NSS_PQG_SEED                0xce534365UL
#define CKA_NSS_PQG_H                   0xce534366UL
#define CKA_NSS_PQG_SEED_BITS           0xce534367UL
#define CKA_NSS_MODULE_SPEC             0xce534368UL
#define CKA_NSS_MOZILLA_CA_POLICY       0xce534372UL
#define CKA_NSS_SERVER_DISTRUST_AFTER   0xce534373UL
#define CKA_NSS_EMAIL_DISTRUST_AFTER    0xce534374UL

/* NSS trust attributes */
#define CKA_TRUST_DIGITAL_SIGNATURE     0xce536351UL
#define CKA_TRUST_NON_REPUDIATION       0xce536352UL
#define CKA_TRUST_KEY_ENCIPHERMENT      0xce536353UL
#define CKA_TRUST_DATA_ENCIPHERMENT     0xce536354UL
#define CKA_TRUST_KEY_AGREEMENT         0xce536355UL
#define CKA_TRUST_KEY_CERT_SIGN         0xce536356UL
#define CKA_TRUST_CRL_SIGN              0xce536357UL
#define CKA_TRUST_SERVER_AUTH           0xce536358UL
#define CKA_TRUST_CLIENT_AUTH           0xce536359UL
#define CKA_TRUST_CODE_SIGNING          0xce53635aUL
#define CKA_TRUST_EMAIL_PROTECTION      0xce53635bUL
#define CKA_TRUST_IPSEC_END_SYSTEM      0xce53635cUL
#define CKA_TRUST_IPSEC_TUNNEL          0xce53635dUL
#define CKA_TRUST_IPSEC_USER            0xce53635eUL
#define CKA_TRUST_TIME_STAMPING         0xce53635fUL
#define CKA_TRUST_STEP_UP_APPROVED      0xce536360UL
#define CKA_CERT_SHA1_HASH              0xce5363b4UL
#define CKA_CERT_MD5_HASH               0xce5363b5UL

/* NSS trust values */
typedef CK_ULONG                        CK_TRUST;
#define CKT_NSS_TRUSTED                 0xce534351UL
#define CKT_NSS_TRUSTED_DELEGATOR       0xce534352UL
#define CKT_NSS_MUST_VERIFY_TRUST       0xce534353UL
#define CKT_NSS_NOT_TRUSTED             0xce53435AUL
#define CKT_NSS_TRUST_UNKNOWN           0xce534355UL
#define CKT_NSS_VALID_DELEGATOR         0xce53435BUL

/* NSS specific mechanisms */
#define CKM_NSS_AES_KEY_WRAP            0xce534351UL
#define CKM_NSS_AES_KEY_WRAP_PAD        0xce534352UL

/* NSS specific return values */
#define CKR_NSS_CERTDB_FAILED           0xce534351UL
#define CKR_NSS_KEYDB_FAILED            0xce534352UL

#endif /* CRYPTOKI_NSS_VENDOR_DEFINED */

/* Define this if you want the vendor specific symbols */
#define CRYPTOKI_X_VENDOR_DEFINED 1
#ifdef CRYPTOKI_X_VENDOR_DEFINED

#define CKA_X_VENDOR   (CKA_VENDOR_DEFINED | 0x58444700UL)
#define CKO_X_VENDOR   (CKA_VENDOR_DEFINED | 0x58444700UL)

/* -------------------------------------------------------------------
 * BLACKLISTS
 */

#define CKA_X_DISTRUSTED                             (CKA_X_VENDOR + 100)

/* -------------------------------------------------------------------
 * CERTIFICATE EXTENSIONS
 *
 * For attaching certificate extensions to certificates
 */

#define CKO_X_CERTIFICATE_EXTENSION                  (CKO_X_VENDOR + 200)

/* From the 2.40 draft */
#ifndef CKA_PUBLIC_KEY_INFO
#define CKA_PUBLIC_KEY_INFO                          0x00000129UL
#endif

#endif /* CRYPTOKI_X_VENDOR_DEFINED */

/* Define this if you want the vendor specific symbols */
#define CRYPTOKI_RU_TEAM_TC26_VENDOR_DEFINED 1
#ifdef CRYPTOKI_RU_TEAM_TC26_VENDOR_DEFINED

/* See https://tc26.ru/standarts/perevody/guidelines-the-pkcs-11-extensions-for-implementing-the-gost-r-34-10-2012-and-gost-r-34-11-2012-russian-standards-.html */

#define NSSCK_VENDOR_PKCS11_RU_TEAM 0xD4321000 /* 0x80000000 | 0x54321000 */
#define CK_VENDOR_PKCS11_RU_TEAM_TC26 NSSCK_VENDOR_PKCS11_RU_TEAM

/* GOST KEY TYPES */
#define CKK_GOSTR3410_512 (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x003)
#define CKK_KUZNECHIK (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x004)
#define CKK_MAGMA (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x005)

/* PKCS #5 PRF Functions */
#define CKP_PKCS5_PBKD2_HMAC_GOSTR3411_2012_512 (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x003)

/* GOST MECHANISMS */
#define CKM_GOSTR3410_512_KEY_PAIR_GEN (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x005)
#define CKM_GOSTR3410_512 (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x006)
#define CKM_GOSTR3410_2012_DERIVE (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x007)
#define CKM_GOSTR3410_WITH_GOSTR3411_2012_256 (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x008)
#define CKM_GOSTR3410_WITH_GOSTR3411_2012_512 (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x009)
#define CKM_GOSTR3410_PUBLIC_KEY_DERIVE (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x00A)
#define CKM_GOSTR3411_2012_256 (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x012)
#define CKM_GOSTR3411_2012_512 (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x013)
#define CKM_GOSTR3411_2012_256_HMAC (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x014)
#define CKM_GOSTR3411_2012_512_HMAC (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x015)
#define CKM_TLS_GOST_PRF_2012_256 (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x016)
#define CKM_TLS_GOST_PRF_2012_512 (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x017)
#define CKM_TLS_GOST_MASTER_KEY_DERIVE_2012_256 (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x018)
#define CKM_KDF_4357 (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x025)
#define CKM_KDF_GOSTR3411_2012_256 (CK_VENDOR_PKCS11_RU_TEAM_TC26 |0x026)

#endif /* CRYPTOKI_RU_TEAM_TC26_VENDOR_DEFINED */

#if defined(__cplusplus)
}
#endif

#endif	/* PKCS11_X_H_ */
