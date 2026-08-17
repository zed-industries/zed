/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
** A collection of things thought to be obsolete
*/

#if defined(PROBSLET_H)
#else
#define PROBSLET_H

#include "prio.h"
#include "private/pprio.h"  /* for PROsfd */

PR_BEGIN_EXTERN_C

/*
** Yield the current thread.  The proper function to use in place of
** PR_Yield() is PR_Sleep() with an argument of PR_INTERVAL_NO_WAIT.
*/
NSPR_API(PRStatus) PR_Yield(void);

/************************************************************************/
/************* The following definitions are for select *****************/
/************************************************************************/

/*
** The following is obsolete and will be deleted in the next release!
** These are provided for compatibility, but are GUARANTEED to be slow.
**
** Override PR_MAX_SELECT_DESC if you need more space in the select set.
*/
#ifndef PR_MAX_SELECT_DESC
#define PR_MAX_SELECT_DESC 1024
#endif
typedef struct PR_fd_set {
    PRUint32      hsize;
    PRFileDesc   *harray[PR_MAX_SELECT_DESC];
    PRUint32      nsize;
    PROsfd        narray[PR_MAX_SELECT_DESC];
} PR_fd_set;

/*
*************************************************************************
** FUNCTION:    PR_Select
** DESCRIPTION:
**
** The call returns as soon as I/O is ready on one or more of the underlying
** file/socket descriptors or an exceptional condition is pending. A count of the
** number of ready descriptors is returned unless a timeout occurs in which case
** zero is returned.  On return, PR_Select replaces the given descriptor sets with
** subsets consisting of those descriptors that are ready for the requested condition.
** The total number of ready descriptors in all the sets is the return value.
**
** INPUTS:
**   PRInt32 num
**       This argument is unused but is provided for select(unix) interface
**       compatability.  All input PR_fd_set arguments are self-describing
**       with its own maximum number of elements in the set.
**
**   PR_fd_set *readfds
**       A set describing the io descriptors for which ready for reading
**       condition is of interest.
**
**   PR_fd_set *writefds
**       A set describing the io descriptors for which ready for writing
**       condition is of interest.
**
**   PR_fd_set *exceptfds
**       A set describing the io descriptors for which exception pending
**       condition is of interest.
**
**   Any of the above readfds, writefds or exceptfds may be given as NULL
**   pointers if no descriptors are of interest for that particular condition.
**
**   PRIntervalTime timeout
**       Amount of time the call will block waiting for I/O to become ready.
**       If this time expires without any I/O becoming ready, the result will
**       be zero.
**
** OUTPUTS:
**   PR_fd_set *readfds
**       A set describing the io descriptors which are ready for reading.
**
**   PR_fd_set *writefds
**       A set describing the io descriptors which are ready for writing.
**
**   PR_fd_set *exceptfds
**       A set describing the io descriptors which have pending exception.
**
** RETURN:PRInt32
**   Number of io descriptors with asked for conditions or zero if the function
**   timed out or -1 on failure.  The reason for the failure is obtained by
**   calling PR_GetError().
** XXX can we implement this on windoze and mac?
**************************************************************************
*/
NSPR_API(PRInt32) PR_Select(
    PRInt32 num, PR_fd_set *readfds, PR_fd_set *writefds,
    PR_fd_set *exceptfds, PRIntervalTime timeout);

/*
** The following are not thread safe for two threads operating on them at the
** same time.
**
** The following routines are provided for manipulating io descriptor sets.
** PR_FD_ZERO(&fdset) initializes a descriptor set fdset to the null set.
** PR_FD_SET(fd, &fdset) includes a particular file descriptor fd in fdset.
** PR_FD_CLR(fd, &fdset) removes a file descriptor fd from fdset.
** PR_FD_ISSET(fd, &fdset) is nonzero if file descriptor fd is a member of
** fdset, zero otherwise.
**
** PR_FD_NSET(osfd, &fdset) includes a particular native file descriptor osfd
** in fdset.
** PR_FD_NCLR(osfd, &fdset) removes a native file descriptor osfd from fdset.
** PR_FD_NISSET(osfd, &fdset) is nonzero if native file descriptor osfd is a member of
** fdset, zero otherwise.
*/

NSPR_API(void)        PR_FD_ZERO(PR_fd_set *set);
NSPR_API(void)        PR_FD_SET(PRFileDesc *fd, PR_fd_set *set);
NSPR_API(void)        PR_FD_CLR(PRFileDesc *fd, PR_fd_set *set);
NSPR_API(PRInt32)     PR_FD_ISSET(PRFileDesc *fd, PR_fd_set *set);
NSPR_API(void)        PR_FD_NSET(PROsfd osfd, PR_fd_set *set);
NSPR_API(void)        PR_FD_NCLR(PROsfd osfd, PR_fd_set *set);
NSPR_API(PRInt32)     PR_FD_NISSET(PROsfd osfd, PR_fd_set *set);

/*
** The next two entry points should not be in the API, but they are
** declared here for historical reasons.
*/

NSPR_API(PRInt32) PR_GetSysfdTableMax(void);

NSPR_API(PRInt32) PR_SetSysfdTableSize(PRIntn table_size);

#ifndef NO_NSPR_10_SUPPORT
#include <sys/stat.h>

NSPR_API(PRInt32) PR_Stat(const char *path, struct stat *buf);
#endif /* NO_NSPR_10_SUPPORT */

PR_END_EXTERN_C

#endif /* defined(PROBSLET_H) */

/* probslet.h */
