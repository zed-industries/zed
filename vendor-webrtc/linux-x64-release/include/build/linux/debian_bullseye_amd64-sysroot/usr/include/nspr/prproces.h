/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef prproces_h___
#define prproces_h___

#include "prtypes.h"
#include "prio.h"

PR_BEGIN_EXTERN_C

/************************************************************************/
/*****************************PROCESS OPERATIONS*************************/
/************************************************************************/

typedef struct PRProcess PRProcess;
typedef struct PRProcessAttr PRProcessAttr;

NSPR_API(PRProcessAttr *) PR_NewProcessAttr(void);

NSPR_API(void) PR_ResetProcessAttr(PRProcessAttr *attr);

NSPR_API(void) PR_DestroyProcessAttr(PRProcessAttr *attr);

NSPR_API(void) PR_ProcessAttrSetStdioRedirect(
    PRProcessAttr *attr,
    PRSpecialFD stdioFd,
    PRFileDesc *redirectFd
);

/*
 * OBSOLETE -- use PR_ProcessAttrSetStdioRedirect instead.
 */
NSPR_API(void) PR_SetStdioRedirect(
    PRProcessAttr *attr,
    PRSpecialFD stdioFd,
    PRFileDesc *redirectFd
);

NSPR_API(PRStatus) PR_ProcessAttrSetCurrentDirectory(
    PRProcessAttr *attr,
    const char *dir
);

NSPR_API(PRStatus) PR_ProcessAttrSetInheritableFD(
    PRProcessAttr *attr,
    PRFileDesc *fd,
    const char *name
);

/*
** Create a new process
**
** Create a new process executing the file specified as 'path' and with
** the supplied arguments and environment.
**
** This function may fail because of illegal access (permissions),
** invalid arguments or insufficient resources.
**
** A process may be created such that the creator can later synchronize its
** termination using PR_WaitProcess().
*/

NSPR_API(PRProcess*) PR_CreateProcess(
    const char *path,
    char *const *argv,
    char *const *envp,
    const PRProcessAttr *attr);

NSPR_API(PRStatus) PR_CreateProcessDetached(
    const char *path,
    char *const *argv,
    char *const *envp,
    const PRProcessAttr *attr);

NSPR_API(PRStatus) PR_DetachProcess(PRProcess *process);

NSPR_API(PRStatus) PR_WaitProcess(PRProcess *process, PRInt32 *exitCode);

NSPR_API(PRStatus) PR_KillProcess(PRProcess *process);

PR_END_EXTERN_C

#endif /* prproces_h___ */
