/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef prenv_h___
#define prenv_h___

#include "prtypes.h"

/*******************************************************************************/
/*******************************************************************************/
/****************** THESE FUNCTIONS MAY NOT BE THREAD SAFE *********************/
/*******************************************************************************/
/*******************************************************************************/

PR_BEGIN_EXTERN_C

/*
** PR_GetEnv() -- Retrieve value of environment variable
**
** Description:
** PR_GetEnv() is modeled on Unix getenv().
**
**
** Inputs:
**   var -- The name of the environment variable
**
** Returns:
**   The value of the environment variable 'var' or NULL if
** the variable is undefined.
**
** Restrictions:
**   You'd think that a POSIX getenv(), putenv() would be
**   consistently implemented everywhere. Surprise! It is not. On
**   some platforms, a putenv() where the argument is of
**   the form "name"  causes the named environment variable to
**   be un-set; that is: a subsequent getenv() returns NULL. On
**   other platforms, the putenv() fails, on others, it is a
**   no-op. Similarly, a putenv() where the argument is of the
**   form "name=" causes the named environment variable to be
**   un-set; a subsequent call to getenv() returns NULL. On
**   other platforms, a subsequent call to getenv() returns a
**   pointer to a null-string (a byte of zero).
**
**   PR_GetEnv(), PR_SetEnv() provide a consistent behavior
**   across all supported platforms. There are, however, some
**   restrictions and some practices you must use to achieve
**   consistent results everywhere.
**
**   When manipulating the environment there is no way to un-set
**   an environment variable across all platforms. We suggest
**   you interpret the return of a pointer to null-string to
**   mean the same as a return of NULL from PR_GetEnv().
**
**   A call to PR_SetEnv() where the parameter is of the form
**   "name" will return PR_FAILURE; the environment remains
**   unchanged. A call to PR_SetEnv() where the parameter is
**   of the form "name=" may un-set the envrionment variable on
**   some platforms; on others it may set the value of the
**   environment variable to the null-string.
**
**   For example, to test for NULL return or return of the
**   null-string from PR_GetEnv(), use the following code
**   fragment:
**
**      char *val = PR_GetEnv("foo");
**      if ((NULL == val) || ('\0' == *val)) {
**          ... interpret this as un-set ...
**      }
**
**   The caller must ensure that the string passed
**   to PR_SetEnv() is persistent. That is: The string should
**   not be on the stack, where it can be overwritten
**   on return from the function calling PR_SetEnv().
**   Similarly, the string passed to PR_SetEnv() must not be
**   overwritten by other actions of the process. ... Some
**   platforms use the string by reference rather than copying
**   it into the environment space. ... You have been warned!
**
**   Use of platform-native functions that manipulate the
**   environment (getenv(), putenv(),
**   SetEnvironmentVariable(), etc.) must not be used with
**   NSPR's similar functions. The platform-native functions
**   may not be thread safe and/or may operate on different
**   conceptual environment space than that operated upon by
**   NSPR's functions or other environment manipulating
**   functions on the same platform. (!)
**
*/
NSPR_API(char*) PR_GetEnv(const char *var);

/*
** PR_GetEnvSecure() -- get a security-sensitive environment variable
**
** Description:
**
** PR_GetEnvSecure() is similar to PR_GetEnv(), but it returns NULL if
** the program was run with elevated privilege (e.g., setuid or setgid
** on Unix).  This can be used for cases like log file paths which
** could otherwise be used for privilege escalation.  Note that some
** platforms may have platform-specific privilege elevation mechanisms
** not recognized by this function; see the implementation for details.
*/
NSPR_API(char*) PR_GetEnvSecure(const char *var);

/*
** PR_SetEnv() -- set, unset or change an environment variable
**
** Description:
** PR_SetEnv() is modeled on the Unix putenv() function.
**
** Inputs:
**   string -- pointer to a caller supplied
**   constant, persistent string of the form name=value. Where
**   name is the name of the environment variable to be set or
**   changed; value is the value assigned to the variable.
**
** Returns:
**   PRStatus.
**
** Restrictions:
**   See the Restrictions documented in the description of
**   PR_GetEnv() in this header file.
**
**
*/
NSPR_API(PRStatus) PR_SetEnv(const char *string);

/*
** PR_DuplicateEnvironment() -- Obtain a copy of the environment.
**
** Description:
** PR_DuplicateEnvironment() copies the environment so that it can be
** modified without changing the current process's environment, and
** then passed to interfaces such as POSIX execve().  In particular,
** this avoids needing to allocate memory or take locks in the child
** after a fork(); neither of these is allowed by POSIX after a
** multithreaded process calls fork(), and PR_SetEnv does both.
**
** Inputs:
**   none
**
** Returns:
**   A pointer to a null-terminated array of null-terminated strings,
**   like the traditional global variable "environ".  The array and
**   the strings are allocated with PR_Malloc(), and it is the
**   caller's responsibility to free them.
**
**   In case of memory allocation failure, or if the operating system
**   doesn't support reading the entire environment through the global
**   variable "environ" or similar, returns NULL instead.
**
** Restrictions:
**   Similarly to PR_GetEnv(), this function may not interoperate as
**   expected with the operating system's native environment accessors.
*/
NSPR_API(char **) PR_DuplicateEnvironment(void);

PR_END_EXTERN_C

#endif /* prenv_h___ */
