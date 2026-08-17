/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


/* author: jstewart */

#if defined(_PRVERSION_H)
#else
#define _PRVERSION_H

#include "prtypes.h"

PR_BEGIN_EXTERN_C

/* All components participating in the PR version protocol must expose
 * a structure and a function. The structure is defined below and named
 * according to the naming conventions outlined further below.  The function
 * is called libVersionPoint and returns a pointer to this structure.
 */

/* on NT, always pack the structure the same. */
#ifdef _WIN32
#pragma pack(push, 8)
#endif

typedef struct {
    /*
     * The first field defines which version of this structure is in use.
     * At this time, only version 2 is specified. If this value is not
     * 2, you must read no further into the structure.
     */
    PRInt32    version;

    /* for Version 2, this is the body format. */
    PRInt64         buildTime;      /* 64 bits - usecs since midnight, 1/1/1970 */
    char *          buildTimeString;/* a human readable version of the time */

    PRUint8   vMajor;               /* Major version of this component */
    PRUint8   vMinor;               /* Minor version of this component */
    PRUint8   vPatch;               /* Patch level of this component */

    PRBool          beta;           /* true if this is a beta component */
    PRBool          debug;          /* true if this is a debug component */
    PRBool          special;        /* true if this component is a special build */

    char *          filename;       /* The original filename */
    char *          description;    /* description of this component */
    char *          security;       /* level of security in this component */
    char *          copyright;      /* The copyright for this file */
    char *          comment;        /* free form field for misc usage */
    char *          specialString;  /* the special variant for this build */
} PRVersionDescription;

/* on NT, restore the previous packing */
#ifdef _WIN32
#pragma pack(pop)
#endif

/*
 * All components must define an entrypoint named libVersionPoint which
 * is of type versionEntryPointType.
 *
 * For example, for a library named libfoo, we would have:
 *
 *   PRVersionDescription prVersionDescription_libfoo =
 *   {
 *       ...
 *   };
 *
 *   PR_IMPLEMENT(const PRVersionDescription*) libVersionPoint(void)
 *   {
 *       return &prVersionDescription_libfoo;
 *   }
 */
typedef const PRVersionDescription *(*versionEntryPointType)(void);

/*
 * Where you declare your libVersionPoint, do it like this:
 * PR_IMPLEMENT(const PRVersionDescription *) libVersionPoint(void) {
 *  fill it in...
 * }
 */

/*
 * NAMING CONVENTION FOR struct
 *
 * all components should also expose a static PRVersionDescription
 * The name of the struct should be calculated as follows:
 * Take the value of filename. (If filename is not specified, calculate
 * a short, unique string.)  Convert all non-alphanumeric characters
 * to '_'.  To this, prepend "PRVersionDescription_".  Thus for libfoo.so,
 * the symbol name is "PRVersionDescription_libfoo_so".
 * so the file should have
 * PRVersionDescription PRVersionDescription_libfoo_so { fill it in };
 * on NT, this file should be declspec export.
 */

PR_END_EXTERN_C

#endif  /* defined(_PRVERSION_H) */

/* prvrsion.h */

