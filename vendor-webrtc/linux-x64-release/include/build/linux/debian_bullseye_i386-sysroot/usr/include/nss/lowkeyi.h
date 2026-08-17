/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _LOWKEYI_H_
#define _LOWKEYI_H_

#include "prtypes.h"
#include "seccomon.h"
#include "secoidt.h"
#include "lowkeyti.h"

SEC_BEGIN_PROTOS

/*
 * See bugzilla bug 125359
 * Since NSS (via PKCS#11) wants to handle big integers as unsigned ints,
 * all of the templates above that en/decode into integers must be converted
 * from ASN.1's signed integer type.  This is done by marking either the
 * source or destination (encoding or decoding, respectively) type as
 * siUnsignedInteger.
 */
extern void prepare_low_rsa_priv_key_for_asn1(NSSLOWKEYPrivateKey *key);
extern void prepare_low_pqg_params_for_asn1(PQGParams *params);
extern void prepare_low_dsa_priv_key_for_asn1(NSSLOWKEYPrivateKey *key);
extern void prepare_low_dsa_priv_key_export_for_asn1(NSSLOWKEYPrivateKey *key);
extern void prepare_low_dh_priv_key_for_asn1(NSSLOWKEYPrivateKey *key);
extern void prepare_low_ec_priv_key_for_asn1(NSSLOWKEYPrivateKey *key);
extern void prepare_low_ecparams_for_asn1(ECParams *params);
extern void prepare_low_rsa_pub_key_for_asn1(NSSLOWKEYPublicKey *key);

/*
** Destroy a private key object.
**  "key" the object
**  "freeit" if PR_TRUE then free the object as well as its sub-objects
*/
extern void nsslowkey_DestroyPrivateKey(NSSLOWKEYPrivateKey *key);

/*
** Destroy a public key object.
**  "key" the object
**  "freeit" if PR_TRUE then free the object as well as its sub-objects
*/
extern void nsslowkey_DestroyPublicKey(NSSLOWKEYPublicKey *key);

/*
** Return the modulus length of "pubKey".
*/
extern unsigned int nsslowkey_PublicModulusLen(NSSLOWKEYPublicKey *pubKey);

/*
** Return the modulus length of "privKey".
*/
extern unsigned int nsslowkey_PrivateModulusLen(NSSLOWKEYPrivateKey *privKey);

/*
** Convert a low private key "privateKey" into a public low key
*/
extern NSSLOWKEYPublicKey *
nsslowkey_ConvertToPublicKey(NSSLOWKEYPrivateKey *privateKey);

/* Make a copy of a low private key in it's own arena.
 * a return of NULL indicates an error.
 */
extern NSSLOWKEYPrivateKey *
nsslowkey_CopyPrivateKey(NSSLOWKEYPrivateKey *privKey);

SEC_END_PROTOS

#endif /* _LOWKEYI_H_ */
