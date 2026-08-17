/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef prprf_h___
#define prprf_h___

/*
** API for PR printf like routines. Supports the following formats
**  %d - decimal
**  %u - unsigned decimal
**  %x - unsigned hex
**  %X - unsigned uppercase hex
**  %o - unsigned octal
**  %hd, %hu, %hx, %hX, %ho - 16-bit versions of above
**  %ld, %lu, %lx, %lX, %lo - 32-bit versions of above
**  %lld, %llu, %llx, %llX, %llo - 64 bit versions of above
**  %s - string
**  %c - character
**  %p - pointer (deals with machine dependent pointer size)
**  %f - float
**  %g - float
*/
#include "prtypes.h"
#include "prio.h"
#include <stdio.h>
#include <stdarg.h>

PR_BEGIN_EXTERN_C

/*
** sprintf into a fixed size buffer. Guarantees that a NUL is at the end
** of the buffer. Returns the length of the written output, NOT including
** the NUL, or (PRUint32)-1 if an error occurs.
*/
NSPR_API(PRUint32) PR_snprintf(char *out, PRUint32 outlen, const char *fmt, ...);

/*
** sprintf into a PR_MALLOC'd buffer. Return a pointer to the malloc'd
** buffer on success, NULL on failure. Call "PR_smprintf_free" to release
** the memory returned.
*/
NSPR_API(char*) PR_smprintf(const char *fmt, ...);

/*
** Free the memory allocated, for the caller, by PR_smprintf
*/
NSPR_API(void) PR_smprintf_free(char *mem);

/*
** "append" sprintf into a PR_MALLOC'd buffer. "last" is the last value of
** the PR_MALLOC'd buffer. sprintf will append data to the end of last,
** growing it as necessary using realloc. If last is NULL, PR_sprintf_append
** will allocate the initial string. The return value is the new value of
** last for subsequent calls, or NULL if there is a malloc failure.
*/
NSPR_API(char*) PR_sprintf_append(char *last, const char *fmt, ...);

/*
** sprintf into a function. The function "f" is called with a string to
** place into the output. "arg" is an opaque pointer used by the stuff
** function to hold any state needed to do the storage of the output
** data. The return value is a count of the number of characters fed to
** the stuff function, or (PRUint32)-1 if an error occurs.
*/
typedef PRIntn (*PRStuffFunc)(void *arg, const char *s, PRUint32 slen);

NSPR_API(PRUint32) PR_sxprintf(PRStuffFunc f, void *arg, const char *fmt, ...);

/*
** fprintf to a PRFileDesc
*/
NSPR_API(PRUint32) PR_fprintf(struct PRFileDesc* fd, const char *fmt, ...);

/*
** va_list forms of the above.
*/
NSPR_API(PRUint32) PR_vsnprintf(char *out, PRUint32 outlen, const char *fmt, va_list ap);
NSPR_API(char*) PR_vsmprintf(const char *fmt, va_list ap);
NSPR_API(char*) PR_vsprintf_append(char *last, const char *fmt, va_list ap);
NSPR_API(PRUint32) PR_vsxprintf(PRStuffFunc f, void *arg, const char *fmt, va_list ap);
NSPR_API(PRUint32) PR_vfprintf(struct PRFileDesc* fd, const char *fmt, va_list ap);

/*
***************************************************************************
** FUNCTION: PR_sscanf
** DESCRIPTION:
**     PR_sscanf() scans the input character string, performs data
**     conversions, and stores the converted values in the data objects
**     pointed to by its arguments according to the format control
**     string.
**
**     PR_sscanf() behaves the same way as the sscanf() function in the
**     Standard C Library (stdio.h), with the following exceptions:
**     - PR_sscanf() handles the NSPR integer and floating point types,
**       such as PRInt16, PRInt32, PRInt64, and PRFloat64, whereas
**       sscanf() handles the standard C types like short, int, long,
**       and double.
**     - PR_sscanf() has no multibyte character support, while sscanf()
**       does.
** INPUTS:
**     const char *buf
**         a character string holding the input to scan
**     const char *fmt
**         the format control string for the conversions
**     ...
**         variable number of arguments, each of them is a pointer to
**         a data object in which the converted value will be stored
** OUTPUTS: none
** RETURNS: PRInt32
**     The number of values converted and stored.
** RESTRICTIONS:
**    Multibyte characters in 'buf' or 'fmt' are not allowed.
***************************************************************************
*/

NSPR_API(PRInt32) PR_sscanf(const char *buf, const char *fmt, ...);

PR_END_EXTERN_C

#endif /* prprf_h___ */
