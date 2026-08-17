/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef NSSBASET_H
#define NSSBASET_H

/*
 * nssbaset.h
 *
 * This file contains the most low-level, fundamental public types.
 */

#include "nspr.h"
#include "nssilock.h"

/*
 * NSS_EXTERN, NSS_IMPLEMENT, NSS_EXTERN_DATA, NSS_IMPLEMENT_DATA
 *
 * NSS has its own versions of these NSPR macros, in a form which
 * does not confuse ctags and other related utilities.  NSPR
 * defines these macros to take the type as an argument, because
 * of certain OS requirements on platforms not supported by NSS.
 */

#define DUMMY /* dummy */
#define NSS_EXTERN extern
#define NSS_EXTERN_DATA extern
#define NSS_IMPLEMENT
#define NSS_IMPLEMENT_DATA

PR_BEGIN_EXTERN_C

/*
 * NSSError
 *
 * Calls to NSS routines may result in one or more errors being placed
 * on the calling thread's "error stack."  Every possible error that
 * may be returned from a function is declared where the function is
 * prototyped.  All errors are of the following type.
 */

typedef PRInt32 NSSError;

/*
 * NSSArena
 *
 * Arenas are logical sets of heap memory, from which memory may be
 * allocated.  When an arena is destroyed, all memory allocated within
 * that arena is implicitly freed.  These arenas are thread-safe:
 * an arena pointer may be used by multiple threads simultaneously.
 * However, as they are not backed by shared memory, they may only be
 * used within one process.
 */

struct NSSArenaStr;
typedef struct NSSArenaStr NSSArena;

/*
 * NSSItem
 *
 * This is the basic type used to refer to an unconstrained datum of
 * arbitrary size.
 */

struct NSSItemStr {
    void *data;
    PRUint32 size;
};
typedef struct NSSItemStr NSSItem;

/*
 * NSSBER
 *
 * Data packed according to the Basic Encoding Rules of ASN.1.
 */

typedef NSSItem NSSBER;

/*
 * NSSDER
 *
 * Data packed according to the Distinguished Encoding Rules of ASN.1;
 * this form is also known as the Canonical Encoding Rules form (CER).
 */

typedef NSSBER NSSDER;

/*
 * NSSBitString
 *
 * Some ASN.1 types use "bit strings," which are passed around as
 * octet strings but whose length is counted in bits.  We use this
 * typedef of NSSItem to point out the occasions when the length
 * is counted in bits, not octets.
 */

typedef NSSItem NSSBitString;

/*
 * NSSUTF8
 *
 * Character strings encoded in UTF-8, as defined by RFC 2279.
 */

typedef char NSSUTF8;

/*
 * NSSASCII7
 *
 * Character strings guaranteed to be 7-bit ASCII.
 */

typedef char NSSASCII7;

PR_END_EXTERN_C

#endif /* NSSBASET_H */
