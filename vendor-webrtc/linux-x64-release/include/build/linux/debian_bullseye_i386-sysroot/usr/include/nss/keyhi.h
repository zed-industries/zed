/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _KEYHI_H_
#define _KEYHI_H_

#include "plarena.h"

#include "seccomon.h"
#include "secoidt.h"
#include "secdert.h"
#include "keythi.h"
#include "certt.h"
/*#include "secpkcs5.h" */

SEC_BEGIN_PROTOS

/*
** Destroy a subject-public-key-info object.
*/
extern void SECKEY_DestroySubjectPublicKeyInfo(CERTSubjectPublicKeyInfo *spki);

/*
** Copy subject-public-key-info "src" to "dst". "dst" is filled in
** appropriately (memory is allocated for each of the sub objects).
*/
extern SECStatus SECKEY_CopySubjectPublicKeyInfo(PLArenaPool *arena,
                                                 CERTSubjectPublicKeyInfo *dst,
                                                 CERTSubjectPublicKeyInfo *src);

/*
** Update the PQG parameters for a cert's public key.
** Only done for DSA certs
*/
extern SECStatus
SECKEY_UpdateCertPQG(CERTCertificate *subjectCert);

/*
** Return the number of bits in the provided big integer.  This assumes that the
** SECItem contains a big-endian number and counts from the first non-zero bit.
*/
extern unsigned SECKEY_BigIntegerBitLength(const SECItem *number);

/*
** Return the strength of the public key in bytes
*/
extern unsigned SECKEY_PublicKeyStrength(const SECKEYPublicKey *pubk);

/*
** Return the strength of the public key in bits
*/
extern unsigned SECKEY_PublicKeyStrengthInBits(const SECKEYPublicKey *pubk);

/*
** Return the length of the signature in bytes
*/
extern unsigned SECKEY_SignatureLen(const SECKEYPublicKey *pubk);

/*
** Make a copy of the private key "privKey"
*/
extern SECKEYPrivateKey *SECKEY_CopyPrivateKey(const SECKEYPrivateKey *privKey);

/*
** Make a copy of the public key "pubKey"
*/
extern SECKEYPublicKey *SECKEY_CopyPublicKey(const SECKEYPublicKey *pubKey);

/*
** Convert a private key "privateKey" into a public key
*/
extern SECKEYPublicKey *SECKEY_ConvertToPublicKey(SECKEYPrivateKey *privateKey);

/*
 * create a new RSA key pair. The private Key is returned...
 */
SECKEYPrivateKey *SECKEY_CreateRSAPrivateKey(int keySizeInBits,
                                             SECKEYPublicKey **pubk, void *cx);

/*
 * create a new DH key pair. The private Key is returned...
 */
SECKEYPrivateKey *SECKEY_CreateDHPrivateKey(SECKEYDHParams *param,
                                            SECKEYPublicKey **pubk, void *cx);

/*
 * create a new EC key pair. The private Key is returned...
 */
SECKEYPrivateKey *SECKEY_CreateECPrivateKey(SECKEYECParams *param,
                                            SECKEYPublicKey **pubk, void *cx);

/*
** Create a subject-public-key-info based on a public key.
*/
extern CERTSubjectPublicKeyInfo *
SECKEY_CreateSubjectPublicKeyInfo(const SECKEYPublicKey *k);

/*
** Convert a base64 ascii encoded DER public key and challenge to spki,
** and verify the signature and challenge data are correct
*/
extern CERTSubjectPublicKeyInfo *
SECKEY_ConvertAndDecodePublicKeyAndChallenge(char *pkacstr, char *challenge,
                                             void *cx);

/*
** Encode a  CERTSubjectPublicKeyInfo structure. into a
** DER encoded subject public key info.
*/
SECItem *
SECKEY_EncodeDERSubjectPublicKeyInfo(const SECKEYPublicKey *pubk);

/*
** Decode a DER encoded subject public key info into a
** CERTSubjectPublicKeyInfo structure.
*/
extern CERTSubjectPublicKeyInfo *
SECKEY_DecodeDERSubjectPublicKeyInfo(const SECItem *spkider);

/*
** Convert a base64 ascii encoded DER subject public key info to our
** internal format.
*/
extern CERTSubjectPublicKeyInfo *
SECKEY_ConvertAndDecodeSubjectPublicKeyInfo(const char *spkistr);

/*
 * extract the public key from a subject Public Key info structure.
 * (used by JSS).
 */
extern SECKEYPublicKey *
SECKEY_ExtractPublicKey(const CERTSubjectPublicKeyInfo *);

/*
** Destroy a private key object.
**	"key" the object
*/
extern void SECKEY_DestroyPrivateKey(SECKEYPrivateKey *key);

/*
** Destroy a public key object.
**	"key" the object
*/
extern void SECKEY_DestroyPublicKey(SECKEYPublicKey *key);

/* Destroy and zero out a private key info structure.  for now this
 * function zero's out memory allocated in an arena for the key
 * since PORT_FreeArena does not currently do this.
 *
 * NOTE -- If a private key info is allocated in an arena, one should
 * not call this function with freeit = PR_FALSE.  The function should
 * destroy the arena.
 */
extern void
SECKEY_DestroyPrivateKeyInfo(SECKEYPrivateKeyInfo *pvk, PRBool freeit);

/* Destroy and zero out an encrypted private key info.
 *
 * NOTE -- If a encrypted private key info is allocated in an arena, one should
 * not call this function with freeit = PR_FALSE.  The function should
 * destroy the arena.
 */
extern void
SECKEY_DestroyEncryptedPrivateKeyInfo(SECKEYEncryptedPrivateKeyInfo *epki,
                                      PRBool freeit);

/* Copy private key info structure.
 *  poolp is the arena into which the contents of from is to be copied.
 *	NULL is a valid entry.
 *  to is the destination private key info
 *  from is the source private key info
 * if either from or to is NULL or an error occurs, SECFailure is
 * returned.  otherwise, SECSuccess is returned.
 */
extern SECStatus
SECKEY_CopyPrivateKeyInfo(PLArenaPool *poolp,
                          SECKEYPrivateKeyInfo *to,
                          const SECKEYPrivateKeyInfo *from);

extern SECStatus
SECKEY_CacheStaticFlags(SECKEYPrivateKey *key);

/* Copy encrypted private key info structure.
 *  poolp is the arena into which the contents of from is to be copied.
 *	NULL is a valid entry.
 *  to is the destination encrypted private key info
 *  from is the source encrypted private key info
 * if either from or to is NULL or an error occurs, SECFailure is
 * returned.  otherwise, SECSuccess is returned.
 */
extern SECStatus
SECKEY_CopyEncryptedPrivateKeyInfo(PLArenaPool *poolp,
                                   SECKEYEncryptedPrivateKeyInfo *to,
                                   const SECKEYEncryptedPrivateKeyInfo *from);
/*
 * Accessor functions for key type of public and private keys.
 */
KeyType SECKEY_GetPrivateKeyType(const SECKEYPrivateKey *privKey);
KeyType SECKEY_GetPublicKeyType(const SECKEYPublicKey *pubKey);

/*
 * Creates a PublicKey from its DER encoding.
 * Currently only supports RSA, DSA, and DH keys.
 */
SECKEYPublicKey *
SECKEY_ImportDERPublicKey(const SECItem *derKey, CK_KEY_TYPE type);

SECKEYPrivateKeyList *
SECKEY_NewPrivateKeyList(void);

void
SECKEY_DestroyPrivateKeyList(SECKEYPrivateKeyList *keys);

void
SECKEY_RemovePrivateKeyListNode(SECKEYPrivateKeyListNode *node);

SECStatus
SECKEY_AddPrivateKeyToListTail(SECKEYPrivateKeyList *list,
                               SECKEYPrivateKey *key);

#define PRIVKEY_LIST_HEAD(l) ((SECKEYPrivateKeyListNode *)PR_LIST_HEAD(&l->list))
#define PRIVKEY_LIST_NEXT(n) ((SECKEYPrivateKeyListNode *)n->links.next)
#define PRIVKEY_LIST_END(n, l) (((void *)n) == ((void *)&l->list))

SECKEYPublicKeyList *
SECKEY_NewPublicKeyList(void);

void
SECKEY_DestroyPublicKeyList(SECKEYPublicKeyList *keys);

void
SECKEY_RemovePublicKeyListNode(SECKEYPublicKeyListNode *node);

SECStatus
SECKEY_AddPublicKeyToListTail(SECKEYPublicKeyList *list,
                              SECKEYPublicKey *key);

#define PUBKEY_LIST_HEAD(l) ((SECKEYPublicKeyListNode *)PR_LIST_HEAD(&l->list))
#define PUBKEY_LIST_NEXT(n) ((SECKEYPublicKeyListNode *)n->links.next)
#define PUBKEY_LIST_END(n, l) (((void *)n) == ((void *)&l->list))

/*
 * Length in bits of the EC's field size.  This is also the length of
 * the x and y coordinates of EC points, such as EC public keys and
 * base points.
 *
 * Return 0 on failure (unknown EC domain parameters).
 */
extern int SECKEY_ECParamsToKeySize(const SECItem *params);

/*
 * Length in bits of the EC base point order, usually denoted n.  This
 * is also the length of EC private keys and ECDSA signature components
 * r and s.
 *
 * Return 0 on failure (unknown EC domain parameters).
 */
extern int SECKEY_ECParamsToBasePointOrderLen(const SECItem *params);

/*
 * Returns the object identifier of the curve, of the provided
 * elliptic curve parameters structures.
 *
 * Return 0 on failure (unknown EC domain parameters).
 */
SECOidTag SECKEY_GetECCOid(const SECKEYECParams *params);

SEC_END_PROTOS

#endif /* _KEYHI_H_ */
