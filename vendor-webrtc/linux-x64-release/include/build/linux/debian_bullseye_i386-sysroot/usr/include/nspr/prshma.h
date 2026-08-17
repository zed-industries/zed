/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*
** prshma.h -- NSPR Anonymous Shared Memory
**
** NSPR provides an anonymous shared memory based on NSPR's PRFileMap
** type. The anonymous file-mapped shared memory provides an inheritable
** shared memory, as in: the child process inherits the shared memory.
** Compare the file-mapped anonymous shared memory to to a named shared
** memory described in prshm.h. The intent is to provide a shared
** memory that is accessable only by parent and child processes. ...
** It's a security thing.
**
** Depending on the underlying platform, the file-mapped shared memory
** may be backed by a file. ... surprise! ... On some platforms, no
** real file backs the shared memory. On platforms where the shared
** memory is backed by a file, the file's name in the filesystem is
** visible to other processes for only the duration of the creation of
** the file, hopefully a very short time. This restricts processess
** that do not inherit the shared memory from opening the file and
** reading or writing its contents. Further, when all processes
** using an anonymous shared memory terminate, the backing file is
** deleted. ... If you are not paranoid, you're not paying attention.
**
** The file-mapped shared memory requires a protocol for the parent
** process and child process to share the memory. NSPR provides two
** protocols. Use one or the other; don't mix and match.
**
** In the first protocol, the job of passing the inheritable shared
** memory is done via helper-functions with PR_CreateProcess(). In the
** second protocol, the parent process is responsible for creating the
** child process; the parent and child are mutually responsible for
** passing a FileMap string. NSPR provides helper functions for
** extracting data from the PRFileMap object. ... See the examples
** below.
**
** Both sides should adhere strictly to the protocol for proper
** operation. The pseudo-code below shows the use of a file-mapped
** shared memory by a parent and child processes. In the examples, the
** server creates the file-mapped shared memory, the client attaches to
** it.
**
** First protocol.
** Server:
**
**   fm = PR_OpenAnonFileMap(dirName, size, FilemapProt);
**   addr = PR_MemMap(fm);
**   attr = PR_NewProcessAttr();
**   PR_ProcessAttrSetInheritableFileMap( attr, fm, shmname );
**   PR_CreateProcess(Client);
**   PR_DestroyProcessAttr(attr);
**   ... yadda ...
**   PR_MemUnmap( addr );
**   PR_CloseFileMap(fm);
**
**
** Client:
**   ... started by server via PR_CreateProcess()
**   fm = PR_GetInheritedFileMap( shmname );
**   addr = PR_MemMap(fm);
**   ... yadda ...
**   PR_MemUnmap(addr);
**   PR_CloseFileMap(fm);
**
**
** Second Protocol:
** Server:
**
**   fm = PR_OpenAnonFileMap(dirName, size, FilemapProt);
**   fmstring = PR_ExportFileMapAsString( fm );
**   addr = PR_MemMap(fm);
**    ... application specific technique to pass fmstring to child
**    ... yadda ... Server uses his own magic to create child
**   PR_MemUnmap( addr );
**   PR_CloseFileMap(fm);
**
**
** Client:
**   ... started by server via his own magic
**   ... application specific technique to find fmstring from parent
**   fm = PR_ImportFileMapFromString( fmstring )
**   addr = PR_MemMap(fm);
**   ... yadda ...
**   PR_MemUnmap(addr);
**   PR_CloseFileMap(fm);
**
**
** lth. 2-Jul-1999.
**
** Note: The second protocol was requested by NelsonB (7/1999); this is
** to accomodate servers which already create their own child processes
** using platform native methods.
**
*/

#ifndef prshma_h___
#define prshma_h___

#include "prtypes.h"
#include "prio.h"
#include "prproces.h"

PR_BEGIN_EXTERN_C

/*
** PR_OpenAnonFileMap() -- Creates an anonymous file-mapped shared memory
**
** Description:
** PR_OpenAnonFileMap() creates an anonymous shared memory. If the
** shared memory already exists, a handle is returned to that shared
** memory object.
**
** On Unix platforms, PR_OpenAnonFileMap() uses 'dirName' as a
** directory name, without the trailing '/', to contain the anonymous
** file. A filename is generated for the name.
**
** On Windows platforms, dirName is ignored.
**
** Inputs:
**   dirName -- A directory name to contain the anonymous file.
**   size -- The size of the shared memory
**   prot -- How the shared memory is mapped. See prio.h
**
** Outputs:
**   PRFileMap *
**
** Returns:
**   Pointer to PRFileMap or NULL on error.
**
*/
NSPR_API( PRFileMap *)
PR_OpenAnonFileMap(
    const char *dirName,
    PRSize      size,
    PRFileMapProtect prot
);

/*
** PR_ProcessAttrSetInheritableFileMap() -- Prepare FileMap for export
**   to my children processes via PR_CreateProcess()
**
** Description:
** PR_ProcessAttrSetInheritableFileMap() connects the PRFileMap to
** PRProcessAttr with shmname. A subsequent call to PR_CreateProcess()
** makes the PRFileMap importable by the child process.
**
** Inputs:
**   attr -- PRProcessAttr, used to pass data to PR_CreateProcess()
**   fm -- PRFileMap structure to be passed to the child process
**   shmname -- The name for the PRFileMap; used by child.
**
** Outputs:
**   PRFileMap *
**
** Returns:
**   PRStatus
**
*/
NSPR_API(PRStatus)
PR_ProcessAttrSetInheritableFileMap(
    PRProcessAttr   *attr,
    PRFileMap       *fm,
    const char      *shmname
);

/*
** PR_GetInheritedFileMap() -- Import a PRFileMap previously exported
**   by my parent process via PR_CreateProcess()
**
** Description:
** PR_GetInheritedFileMap() retrieves a PRFileMap object exported from
** its parent process via PR_CreateProcess().
**
** Inputs:
**    shmname -- The name provided to PR_ProcessAttrSetInheritableFileMap()
**
** Outputs:
**   PRFileMap *
**
** Returns:
**   PRFileMap pointer or NULL.
**
*/
NSPR_API( PRFileMap *)
PR_GetInheritedFileMap(
    const char *shmname
);

/*
** PR_ExportFileMapAsString() -- Creates a string identifying a PRFileMap
**
** Description:
** Creates an identifier, as a string, from a PRFileMap object
** previously created with PR_OpenAnonFileMap().
**
** Inputs:
**   fm -- PRFileMap pointer to be represented as a string.
**   bufsize -- sizeof(buf)
**   buf -- a buffer of length PR_FILEMAP_STRING_BUFSIZE
**
** Outputs:
**   buf contains the stringized PRFileMap identifier
**
** Returns:
**   PRStatus
**
*/
NSPR_API( PRStatus )
PR_ExportFileMapAsString(
    PRFileMap *fm,
    PRSize    bufsize,
    char      *buf
);
#define PR_FILEMAP_STRING_BUFSIZE 128

/*
** PR_ImportFileMapFromString() -- Creates a PRFileMap from the identifying string
**
** Description:
** PR_ImportFileMapFromString() creates a PRFileMap object from a
** string previously created by PR_ExportFileMapAsString().
**
** Inputs:
**   fmstring -- string created by PR_ExportFileMapAsString()
**
** Returns:
**   PRFileMap pointer or NULL.
**
*/
NSPR_API( PRFileMap * )
PR_ImportFileMapFromString(
    const char *fmstring
);

PR_END_EXTERN_C
#endif /* prshma_h___ */
