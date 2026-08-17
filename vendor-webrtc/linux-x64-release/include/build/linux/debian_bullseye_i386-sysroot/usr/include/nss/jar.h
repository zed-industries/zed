/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef __JAR_h_
#define __JAR_h_

/*
 *  In general, any functions that return pointers
 *  have memory owned by the caller.
 *
 */

/* security includes */
#include "cert.h"
#include "hasht.h"

/* nspr 2.0 includes */
#include "prio.h"

#define ZHUGEP

#include <stdio.h>

/* various types */

typedef enum {
    jarTypeMF = 2,
    jarTypeSF = 3,
    jarTypeMeta = 6,
    jarTypePhy = 7,
    jarTypeSign = 10,
    jarTypeSect = 11,
    jarTypeOwner = 13
} jarType;

/* void data in ZZList's contain JAR_Item type */
typedef struct JAR_Item_ {
    char *pathname; /* relative. inside zip file */
    jarType type;   /* various types */
    size_t size;    /* size of data below */
    void *data;     /* totally opaque */
} JAR_Item;

/* hashes */
typedef enum {
    jarHashNone = 0,
    jarHashBad = 1,
    jarHashPresent = 2
} jarHash;

typedef struct JAR_Digest_ {
    jarHash md5_status;
    unsigned char md5[MD5_LENGTH];
    jarHash sha1_status;
    unsigned char sha1[SHA1_LENGTH];
} JAR_Digest;

/* physical archive formats */
typedef enum {
    jarArchGuess = 0,
    jarArchNone = 1,
    jarArchZip = 2,
    jarArchTar = 3
} jarArch;

#include "jar-ds.h"

struct JAR_;

typedef int jar_settable_callback_fn(int status, struct JAR_ *jar,
                                     const char *metafile, char *pathname,
                                     char *errortext);

/* jar object */
typedef struct JAR_ {
    jarArch format; /* physical archive format */

    char *url;      /* Where it came from */
    char *filename; /* Disk location */
    FILE *fp;       /* For multiple extractions */
    /* JAR_FILE */

    /* various linked lists */
    ZZList *manifest; /* Digests of MF sections */
    ZZList *hashes;   /* Digests of actual signed files */
    ZZList *phy;      /* Physical layout of JAR file */
    ZZList *metainfo; /* Global metainfo */

    JAR_Digest *globalmeta; /* digest of .MF global portion */

    /* Below will change to a linked list to support multiple sigs */
    int pkcs7; /* Enforced opaqueness */
    int valid; /* PKCS7 signature validated */

    ZZList *signers; /* the above, per signer */

    /* Window context, very necessary for PKCS11 now */
    void *mw; /* MWContext window context */

    /* Signal callback function */
    jar_settable_callback_fn *signal;
} JAR;

/*
 *  Iterator
 *
 *  Context for iterative operations. Certain operations
 *  require iterating multiple linked lists because of
 *  multiple signers. "nextsign" is used for this purpose.
 *
 */
typedef struct JAR_Context_ {
    JAR *jar;         /* Jar we are searching */
    char *pattern;    /* Regular expression */
    jarType finding;  /* Type of item to find */
    ZZLink *next;     /* Next item in find */
    ZZLink *nextsign; /* Next signer, sometimes */
} JAR_Context;

typedef struct JAR_Signer_ {
    int pkcs7;          /* Enforced opaqueness */
    int valid;          /* PKCS7 signature validated */
    char *owner;        /* name of .RSA file */
    JAR_Digest *digest; /* of .SF file */
    ZZList *sf;         /* Linked list of .SF file contents */
    ZZList *certs;      /* Signing information */
} JAR_Signer;

/* Meta informaton, or "policy", from the manifest file.
   Right now just one tuple per JAR_Item. */
typedef struct JAR_Metainfo_ {
    char *header;
    char *info;
} JAR_Metainfo;

/* This should not be global */
typedef struct JAR_Physical_ {
    unsigned char compression;
    unsigned long offset;
    unsigned long length;
    unsigned long uncompressed_length;
#if defined(XP_UNIX) || defined(XP_BEOS)
    PRUint16 mode;
#endif
} JAR_Physical;

typedef struct JAR_Cert_ {
    size_t length;
    void *key;
    CERTCertificate *cert;
} JAR_Cert;

/* certificate stuff */
typedef enum {
    jarCertCompany = 1,
    jarCertCA = 2,
    jarCertSerial = 3,
    jarCertExpires = 4,
    jarCertNickname = 5,
    jarCertFinger = 6,
    jarCertJavaHack = 100
} jarCert;

/* callback types */
#define JAR_CB_SIGNAL 1

/*
 *  This is the base for the JAR error codes. It will
 *  change when these are incorporated into allxpstr.c,
 *  but right now they won't let me put them there.
 *
 */
#ifndef SEC_ERR_BASE
#define SEC_ERR_BASE (-0x2000)
#endif

#define JAR_BASE SEC_ERR_BASE + 300

/* Jar specific error definitions */

#define JAR_ERR_GENERAL (JAR_BASE + 1)
#define JAR_ERR_FNF (JAR_BASE + 2)
#define JAR_ERR_CORRUPT (JAR_BASE + 3)
#define JAR_ERR_MEMORY (JAR_BASE + 4)
#define JAR_ERR_DISK (JAR_BASE + 5)
#define JAR_ERR_ORDER (JAR_BASE + 6)
#define JAR_ERR_SIG (JAR_BASE + 7)
#define JAR_ERR_METADATA (JAR_BASE + 8)
#define JAR_ERR_ENTRY (JAR_BASE + 9)
#define JAR_ERR_HASH (JAR_BASE + 10)
#define JAR_ERR_PK7 (JAR_BASE + 11)
#define JAR_ERR_PNF (JAR_BASE + 12)

/* Function declarations */

extern JAR *JAR_new(void);

extern void PR_CALLBACK JAR_destroy(JAR *jar);

extern char *JAR_get_error(int status);

extern int JAR_set_callback(int type, JAR *jar, jar_settable_callback_fn *fn);

extern void
JAR_init_callbacks(char *(*string_cb)(int),
                   void *(*find_cx)(void),
                   void *(*init_cx)(void));

/*
 *  JAR_set_context
 *
 *  PKCS11 may require a password to be entered by the user
 *  before any crypto routines may be called. This will require
 *  a window context if used from inside Mozilla.
 *
 *  Call this routine with your context before calling
 *  verifying or signing. If you have no context, call with NULL
 *  and one will be chosen for you.
 *
 */
int JAR_set_context(JAR *jar, void /*MWContext*/ *mw);

/*
 *  Iterative operations
 *
 *  JAR_find sets up for repeated calls with JAR_find_next.
 *  I never liked findfirst and findnext, this is nicer.
 *
 *  Pattern contains a relative pathname to match inside the
 *  archive. It is currently assumed to be "*".
 *
 *  To use:
 *
 *     JAR_Item *item;
 *     JAR_find (jar, "*.class", jarTypeMF);
 *     while (JAR_find_next (jar, &item) >= 0)
 *   { do stuff }
 *
 */

/* Replacement functions with an external context */

extern JAR_Context *JAR_find(JAR *jar, char *pattern, jarType type);

extern int JAR_find_next(JAR_Context *ctx, JAR_Item **it);

extern void JAR_find_end(JAR_Context *ctx);

/*
 *  Function to parse manifest file:
 *
 *  Many signatures may be attached to a single filename located
 *  inside the zip file. We only support one.
 *
 *  Several manifests may be included in the zip file.
 *
 *  You must pass the MANIFEST.MF file before any .SF files.
 *
 *  Right now this returns a big ole list, privately in the jar structure.
 *  If you need to traverse it, use JAR_find if possible.
 *
 *  The path is needed to determine what type of binary signature is
 *  being passed, though it is technically not needed for manifest files.
 *
 *  When parsing an ASCII file, null terminate the ASCII raw_manifest
 *  prior to sending it, and indicate a length of 0. For binary digital
 *  signatures only, indicate the true length of the signature.
 *  (This is legacy behavior.)
 *
 *  You may free the manifest after parsing it.
 *
 */

extern int
JAR_parse_manifest(JAR *jar, char *raw_manifest, long length, const char *path,
                   const char *url);

/*
 *  Verify data (nonstreaming). The signature is actually
 *  checked by JAR_parse_manifest or JAR_pass_archive.
 *
 */

extern JAR_Digest *PR_CALLBACK
JAR_calculate_digest(void *data, long length);

extern int PR_CALLBACK
JAR_verify_digest(JAR *jar, const char *name, JAR_Digest *dig);

extern int
JAR_digest_file(char *filename, JAR_Digest *dig);

/*
 *  Meta information
 *
 *  Currently, since this call does not support passing of an owner
 *  (certificate, or physical name of the .sf file), it is restricted to
 *  returning information located in the manifest.mf file.
 *
 *  Meta information is a name/value pair inside the archive file. Here,
 *  the name is passed in *header and value returned in **info.
 *
 *  Pass a NULL as the name to retrieve metainfo from the global section.
 *
 *  Data is returned in **info, of size *length. The return value
 *  will indicate if no data was found.
 *
 */

extern int
JAR_get_metainfo(JAR *jar, char *name, char *header, void **info,
                 unsigned long *length);

extern char *JAR_get_filename(JAR *jar);

extern char *JAR_get_url(JAR *jar);

/* save the certificate with this fingerprint in persistent
   storage, somewhere, for retrieval in a future session when there
   is no corresponding JAR structure. */
extern int PR_CALLBACK
JAR_stash_cert(JAR *jar, long keylen, void *key);

/* retrieve a certificate presumably stashed with the above
   function, but may be any certificate. Type is &CERTCertificate */
CERTCertificate *
JAR_fetch_cert(long length, void *key);

/*
 *  New functions to handle archives alone
 *    (call JAR_new beforehand)
 *
 *  JAR_pass_archive acts much like parse_manifest. Certificates
 *  are returned in the JAR structure but as opaque data. When calling
 *  JAR_verified_extract you still need to decide which of these
 *  certificates to honor.
 *
 *  Code to examine a JAR structure is in jarbert.c. You can obtain both
 *  a list of filenames and certificates from traversing the linked list.
 *
 */
extern int
JAR_pass_archive(JAR *jar, jarArch format, char *filename, const char *url);

/*
 * Same thing, but don't check signatures
 */
extern int
JAR_pass_archive_unverified(JAR *jar, jarArch format, char *filename,
                            const char *url);

/*
 *  Extracts a relative pathname from the archive and places it
 *  in the filename specified.
 *
 *  Call JAR_set_nailed if you want to keep the file descriptors
 *  open between multiple calls to JAR_verify_extract.
 *
 */
extern int
JAR_verified_extract(JAR *jar, char *path, char *outpath);

/*
 *  JAR_extract does no crypto checking. This can be used if you
 *  need to extract a manifest file or signature, etc.
 *
 */
extern int
JAR_extract(JAR *jar, char *path, char *outpath);

#endif /* __JAR_h_ */
