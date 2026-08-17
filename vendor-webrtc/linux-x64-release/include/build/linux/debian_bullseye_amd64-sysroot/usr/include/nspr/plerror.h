/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
** File:        plerror.h
** Description: Simple routine to print translate the calling thread's
**              error numbers and print them.
*/

#if defined(PLERROR_H)
#else
#define PLERROR_H

#include "prio.h"
#include "prtypes.h"

PR_BEGIN_EXTERN_C
/*
** Print the messages to "syserr" prepending 'msg' if not NULL.
*/
PR_EXTERN(void) PL_PrintError(const char *msg);

/*
** Print the messages to specified output file prepending 'msg' if not NULL.
*/
PR_EXTERN(void) PL_FPrintError(PRFileDesc *output, const char *msg);

PR_END_EXTERN_C

#endif /* defined(PLERROR_H) */

/* plerror.h */
