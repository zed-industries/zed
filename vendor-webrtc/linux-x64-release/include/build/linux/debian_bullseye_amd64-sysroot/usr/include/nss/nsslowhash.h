/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * Provide FIPS validated hashing for applications that only need hashing.
 * NOTE: mac'ing requires keys and will not work in this interface.
 * Also NOTE: this only works with Hashing. Only the FIPS interface is enabled.
 */

#ifndef _NSSLOWHASH_H_
#define _NSSLOWHASH_H_

typedef struct NSSLOWInitContextStr NSSLOWInitContext;
typedef struct NSSLOWHASHContextStr NSSLOWHASHContext;

NSSLOWInitContext *NSSLOW_Init(void);
void NSSLOW_Shutdown(NSSLOWInitContext *context);
void NSSLOW_Reset(NSSLOWInitContext *context);
NSSLOWHASHContext *NSSLOWHASH_NewContext(
    NSSLOWInitContext *initContext,
    HASH_HashType hashType);
void NSSLOWHASH_Begin(NSSLOWHASHContext *context);
void NSSLOWHASH_Update(NSSLOWHASHContext *context,
                       const unsigned char *buf,
                       unsigned int len);
void NSSLOWHASH_End(NSSLOWHASHContext *context,
                    unsigned char *buf,
                    unsigned int *ret, unsigned int len);
void NSSLOWHASH_Destroy(NSSLOWHASHContext *context);
unsigned int NSSLOWHASH_Length(NSSLOWHASHContext *context);

#endif
