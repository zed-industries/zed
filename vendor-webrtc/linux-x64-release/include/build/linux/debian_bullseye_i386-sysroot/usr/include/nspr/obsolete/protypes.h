/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
 * This header typedefs the old 'native' types to the new PR<type>s.
 * These definitions are scheduled to be eliminated at the earliest
 * possible time. The NSPR API is implemented and documented using
 * the new definitions.
 */

#if !defined(PROTYPES_H)
#define PROTYPES_H

typedef PRUintn uintn;
#ifndef _XP_Core_
typedef PRIntn intn;
#endif

/*
 * It is trickier to define uint, int8, uint8, int16, uint16,
 * int32, uint32, int64, and uint64 because some of these int
 * types are defined by standard header files on some platforms.
 * Our strategy here is to include all such standard headers
 * first, and then define these int types only if they are not
 * defined by those standard headers.
 */

/*
 * SVR4 typedef of uint is commonly found on UNIX machines.
 *
 * On AIX 4.3, sys/inttypes.h (which is included by sys/types.h)
 * defines the types int8, int16, int32, and int64.
 *
 * On OS/2, sys/types.h defines uint.
 */
#if defined(XP_UNIX) || defined(XP_OS2)
#include <sys/types.h>
#endif

/* model.h on HP-UX defines int8, int16, and int32. */
#ifdef HPUX
#include <model.h>
#endif

/*
 * uint
 */

#if !defined(XP_OS2) && !defined(XP_UNIX) || defined(NTO)
typedef PRUintn uint;
#endif

/*
 * uint64
 */

typedef PRUint64 uint64;

/*
 * uint32
 */

#if !defined(_WIN32) && !defined(XP_OS2) && !defined(NTO)
typedef PRUint32 uint32;
#else
typedef unsigned long uint32;
#endif

/*
 * uint16
 */

typedef PRUint16 uint16;

/*
 * uint8
 */

typedef PRUint8 uint8;

/*
 * int64
 */

#if !defined(_PR_AIX_HAVE_BSD_INT_TYPES)
typedef PRInt64 int64;
#endif

/*
 * int32
 */

#if !defined(_PR_AIX_HAVE_BSD_INT_TYPES) \
    && !defined(HPUX)
#if !defined(_WIN32) && !defined(XP_OS2) && !defined(NTO)
typedef PRInt32 int32;
#else
typedef long int32;
#endif
#endif

/*
 * int16
 */

#if !defined(_PR_AIX_HAVE_BSD_INT_TYPES) \
    && !defined(HPUX)
typedef PRInt16 int16;
#endif

/*
 * int8
 */

#if !defined(_PR_AIX_HAVE_BSD_INT_TYPES) \
    && !defined(HPUX)
typedef PRInt8 int8;
#endif

typedef PRFloat64 float64;
typedef PRUptrdiff uptrdiff_t;
typedef PRUword uprword_t;
typedef PRWord prword_t;


/* Re: prbit.h */
#define TEST_BIT    PR_TEST_BIT
#define SET_BIT     PR_SET_BIT
#define CLEAR_BIT   PR_CLEAR_BIT

/* Re: prarena.h->plarena.h */
#define PRArena PLArena
#define PRArenaPool PLArenaPool
#define PRArenaStats PLArenaStats
#define PR_ARENA_ALIGN PL_ARENA_ALIGN
#define PR_INIT_ARENA_POOL PL_INIT_ARENA_POOL
#define PR_ARENA_ALLOCATE PL_ARENA_ALLOCATE
#define PR_ARENA_GROW PL_ARENA_GROW
#define PR_ARENA_MARK PL_ARENA_MARK
#define PR_CLEAR_UNUSED PL_CLEAR_UNUSED
#define PR_CLEAR_ARENA PL_CLEAR_ARENA
#define PR_ARENA_RELEASE PL_ARENA_RELEASE
#define PR_COUNT_ARENA PL_COUNT_ARENA
#define PR_ARENA_DESTROY PL_ARENA_DESTROY
#define PR_InitArenaPool PL_InitArenaPool
#define PR_FreeArenaPool PL_FreeArenaPool
#define PR_FinishArenaPool PL_FinishArenaPool
#define PR_CompactArenaPool PL_CompactArenaPool
#define PR_ArenaFinish PL_ArenaFinish
#define PR_ArenaAllocate PL_ArenaAllocate
#define PR_ArenaGrow PL_ArenaGrow
#define PR_ArenaRelease PL_ArenaRelease
#define PR_ArenaCountAllocation PL_ArenaCountAllocation
#define PR_ArenaCountInplaceGrowth PL_ArenaCountInplaceGrowth
#define PR_ArenaCountGrowth PL_ArenaCountGrowth
#define PR_ArenaCountRelease PL_ArenaCountRelease
#define PR_ArenaCountRetract PL_ArenaCountRetract

/* Re: prhash.h->plhash.h */
#define PRHashEntry PLHashEntry
#define PRHashTable PLHashTable
#define PRHashNumber PLHashNumber
#define PRHashFunction PLHashFunction
#define PRHashComparator PLHashComparator
#define PRHashEnumerator PLHashEnumerator
#define PRHashAllocOps PLHashAllocOps
#define PR_NewHashTable PL_NewHashTable
#define PR_HashTableDestroy PL_HashTableDestroy
#define PR_HashTableRawLookup PL_HashTableRawLookup
#define PR_HashTableRawAdd PL_HashTableRawAdd
#define PR_HashTableRawRemove PL_HashTableRawRemove
#define PR_HashTableAdd PL_HashTableAdd
#define PR_HashTableRemove PL_HashTableRemove
#define PR_HashTableEnumerateEntries PL_HashTableEnumerateEntries
#define PR_HashTableLookup PL_HashTableLookup
#define PR_HashTableDump PL_HashTableDump
#define PR_HashString PL_HashString
#define PR_CompareStrings PL_CompareStrings
#define PR_CompareValues PL_CompareValues

#endif /* !defined(PROTYPES_H) */
