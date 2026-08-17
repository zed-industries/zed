/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef plhash_h___
#define plhash_h___
/*
 * API to portable hash table code.
 */
#include <stdio.h>
#include "prtypes.h"

PR_BEGIN_EXTERN_C

typedef struct PLHashEntry  PLHashEntry;
typedef struct PLHashTable  PLHashTable;
typedef PRUint32 PLHashNumber;
#define PL_HASH_BITS 32  /* Number of bits in PLHashNumber */
typedef PLHashNumber (PR_CALLBACK *PLHashFunction)(const void *key);
typedef PRIntn (PR_CALLBACK *PLHashComparator)(const void *v1, const void *v2);

typedef PRIntn (PR_CALLBACK *PLHashEnumerator)(PLHashEntry *he, PRIntn i, void *arg);

/* Flag bits in PLHashEnumerator's return value */
#define HT_ENUMERATE_NEXT       0       /* continue enumerating entries */
#define HT_ENUMERATE_STOP       1       /* stop enumerating entries */
#define HT_ENUMERATE_REMOVE     2       /* remove and free the current entry */
#define HT_ENUMERATE_UNHASH     4       /* just unhash the current entry */

typedef struct PLHashAllocOps {
    void *              (PR_CALLBACK *allocTable)(void *pool, PRSize size);
    void                (PR_CALLBACK *freeTable)(void *pool, void *item);
    PLHashEntry *       (PR_CALLBACK *allocEntry)(void *pool, const void *key);
    void                (PR_CALLBACK *freeEntry)(void *pool, PLHashEntry *he, PRUintn flag);
} PLHashAllocOps;

#define HT_FREE_VALUE   0               /* just free the entry's value */
#define HT_FREE_ENTRY   1               /* free value and entire entry */

struct PLHashEntry {
    PLHashEntry         *next;          /* hash chain linkage */
    PLHashNumber        keyHash;        /* key hash function result */
    const void          *key;           /* ptr to opaque key */
    void                *value;         /* ptr to opaque value */
};

struct PLHashTable {
    PLHashEntry         **buckets;      /* vector of hash buckets */
    PRUint32              nentries;       /* number of entries in table */
    PRUint32              shift;          /* multiplicative hash shift */
    PLHashFunction      keyHash;        /* key hash function */
    PLHashComparator    keyCompare;     /* key comparison function */
    PLHashComparator    valueCompare;   /* value comparison function */
    const PLHashAllocOps *allocOps;     /* allocation operations */
    void                *allocPriv;     /* allocation private data */
#ifdef HASHMETER
    PRUint32              nlookups;       /* total number of lookups */
    PRUint32              nsteps;         /* number of hash chains traversed */
    PRUint32              ngrows;         /* number of table expansions */
    PRUint32              nshrinks;       /* number of table contractions */
#endif
};

/*
 * Create a new hash table.
 * If allocOps is null, use default allocator ops built on top of malloc().
 */
PR_EXTERN(PLHashTable *)
PL_NewHashTable(PRUint32 numBuckets, PLHashFunction keyHash,
                PLHashComparator keyCompare, PLHashComparator valueCompare,
                const PLHashAllocOps *allocOps, void *allocPriv);

PR_EXTERN(void)
PL_HashTableDestroy(PLHashTable *ht);

/* Higher level access methods */
PR_EXTERN(PLHashEntry *)
PL_HashTableAdd(PLHashTable *ht, const void *key, void *value);

PR_EXTERN(PRBool)
PL_HashTableRemove(PLHashTable *ht, const void *key);

PR_EXTERN(void *)
PL_HashTableLookup(PLHashTable *ht, const void *key);

PR_EXTERN(void *)
PL_HashTableLookupConst(PLHashTable *ht, const void *key);

PR_EXTERN(PRIntn)
PL_HashTableEnumerateEntries(PLHashTable *ht, PLHashEnumerator f, void *arg);

/* General-purpose C string hash function. */
PR_EXTERN(PLHashNumber)
PL_HashString(const void *key);

/* Compare strings using strcmp(), return true if equal. */
PR_EXTERN(PRIntn)
PL_CompareStrings(const void *v1, const void *v2);

/* Stub function just returns v1 == v2 */
PR_EXTERN(PRIntn)
PL_CompareValues(const void *v1, const void *v2);

/* Low level access methods */
PR_EXTERN(PLHashEntry **)
PL_HashTableRawLookup(PLHashTable *ht, PLHashNumber keyHash, const void *key);

PR_EXTERN(PLHashEntry **)
PL_HashTableRawLookupConst(PLHashTable *ht, PLHashNumber keyHash,
                           const void *key);

PR_EXTERN(PLHashEntry *)
PL_HashTableRawAdd(PLHashTable *ht, PLHashEntry **hep, PLHashNumber keyHash,
                   const void *key, void *value);

PR_EXTERN(void)
PL_HashTableRawRemove(PLHashTable *ht, PLHashEntry **hep, PLHashEntry *he);

/* This can be trivially implemented using PL_HashTableEnumerateEntries. */
PR_EXTERN(PRIntn)
PL_HashTableDump(PLHashTable *ht, PLHashEnumerator dump, FILE *fp);

PR_END_EXTERN_C

#endif /* plhash_h___ */
