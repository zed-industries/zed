/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef prerror_h___
#define prerror_h___

#include "prtypes.h"

PR_BEGIN_EXTERN_C

typedef PRInt32 PRErrorCode;

#define PR_NSPR_ERROR_BASE -6000

#include "prerr.h"

/*
** Set error will preserve an error condition within a thread context.
** The values stored are the NSPR (platform independent) translation of
** the error. Also, if available, the platform specific oserror is stored.
** If there is no appropriate OS error number, a zero my be supplied.
*/
NSPR_API(void) PR_SetError(PRErrorCode errorCode, PRInt32 oserr);

/*
** The text value specified may be NULL. If it is not NULL and the text length
** is zero, the string is assumed to be a null terminated C string. Otherwise
** the text is assumed to be the length specified and possibly include NULL
** characters (e.g., a multi-national string).
**
** The text will be copied into to thread structure and remain there
** until the next call to PR_SetError.
*/
NSPR_API(void) PR_SetErrorText(
    PRIntn textLength, const char *text);

/*
** Return the current threads last set error code.
*/
NSPR_API(PRErrorCode) PR_GetError(void);

/*
** Return the current threads last set os error code. This is used for
** machine specific code that desires the underlying os error.
*/
NSPR_API(PRInt32) PR_GetOSError(void);

/*
** Get the length of the error text. If a zero is returned, then there
** is no text. Otherwise, the value returned is sufficient to contain
** the error text currently available.
*/
NSPR_API(PRInt32) PR_GetErrorTextLength(void);

/*
** Copy the current threads current error text. Then actual number of bytes
** copied is returned as the result. If the result is zero, the 'text' area
** is unaffected.
*/
NSPR_API(PRInt32) PR_GetErrorText(char *text);


/*
Copyright (C) 1987, 1988 Student Information Processing Board of the
Massachusetts Institute of Technology.

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose and without fee is hereby granted, provided
that the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation, and that the names of M.I.T. and the M.I.T. S.I.P.B. not be
used in advertising or publicity pertaining to distribution of the software
without specific, written prior permission.  M.I.T. and the M.I.T. S.I.P.B.
make no representations about the suitability of this software for any
purpose.  It is provided "as is" without express or implied warranty.
*/


/*
 * NOTE:
 *      The interfaces for error-code-translation described in the rest of
 *      this file are preliminary in the 3.1 release of nspr and are subject
 *      to change in future releases.
 */

/*
** Description: Localizable error code to string function.
**
**
** NSPR provides a mechanism for converting an error code to a
** descriptive string, in a caller-specified language.
**
** Error codes themselves are 32 bit (signed) integers.  Typically,
** the high order 24 bits are an identifier of which error table the
** error code is from, and the low order 8 bits are a sequential error
** number within the table.  NSPR supports error tables whose first
** error code is not a multiple of 256, such error code assignments
** should be avoided when possible.
**
** Error table 0 is defined to match the UNIX system call error table
** (sys_errlist); this allows errno values to be used directly in the
** library.  Other error table numbers are typically formed by
** compacting together the first four characters of the error table
** name.  The mapping between characters in the name and numeric
** values in the error code are defined in a system-independent
** fashion, so that two systems that can pass integral values between
** them can reliably pass error codes without loss of meaning; this
** should work even if the character sets used are not the
** same. (However, if this is to be done, error table 0 should be
** avoided, since the local system call error tables may differ.)
**
** Libraries defining error codes need only provide a table mapping
** error code numbers to names and default English descriptions,
** calling a routine to install the table, making it ``known'' to NSPR
** library.  Once installed, a table may not be removed.  Any error
** code the library generates can be converted to the corresponding
** error message.  There is also a default format for error codes
** accidentally returned before making the table known, which is of
** the form "unknown code foo 32", where "foo" would be the name of
** the table.
**
** Normally, the error code conversion routine only supports the
** languages "i-default" and "en", returning the error-table-provided
** English description for both languages.  The application may
** provide a localization plugin, allowing support for additional
** languages.
**
**/

/**********************************************************************/
/************************* TYPES AND CONSTANTS ************************/
/**********************************************************************/

/*
 * PRLanguageCode --
 *
 *    NSPR represents a language code as a non-negative integer.
 *    Languages 0 is always "i-default" the language you get without
 *    explicit negotiation.  Language 1 is always "en", English
 *    which has been explicitly negotiated.  Additional language
 *    codes are defined by an application-provided localization plugin.
 */
typedef PRUint32 PRLanguageCode;
#define PR_LANGUAGE_I_DEFAULT 0 /* i-default, the default language */
#define PR_LANGUAGE_EN 1 /* English, explicitly negotiated */

/*
 * struct PRErrorMessage --
 *
 *    An error message in an error table.
 */
struct PRErrorMessage {
    const char * name;    /* Macro name for error */
    const char * en_text; /* Default English text */
};

/*
 * struct PRErrorTable --
 *
 *    An error table, provided by a library.
 */
struct PRErrorTable {
    const struct PRErrorMessage * msgs; /* Array of error information */
    const char *name; /* Name of error table source */
    PRErrorCode base; /* Error code for first error in table */
    int n_msgs; /* Number of codes in table */
};

/*
 * struct PRErrorCallbackPrivate --
 *
 *    A private structure for the localization plugin
 */
struct PRErrorCallbackPrivate;

/*
 * struct PRErrorCallbackTablePrivate --
 *
 *    A data structure under which the localization plugin may store information,
 *    associated with an error table, that is private to itself.
 */
struct PRErrorCallbackTablePrivate;

/*
 * PRErrorCallbackLookupFn --
 *
 *    A function of PRErrorCallbackLookupFn type is a localization
 *    plugin callback which converts an error code into a description
 *    in the requested language.  The callback is provided the
 *    appropriate error table, private data for the plugin and the table.
 *    The callback returns the appropriate UTF-8 encoded description, or NULL
 *    if no description can be found.
 */
typedef const char *
PRErrorCallbackLookupFn(PRErrorCode code, PRLanguageCode language,
                        const struct PRErrorTable *table,
                        struct PRErrorCallbackPrivate *cb_private,
                        struct PRErrorCallbackTablePrivate *table_private);

/*
 * PRErrorCallbackNewTableFn --
 *
 *    A function PRErrorCallbackNewTableFn type is a localization plugin
 *    callback which is called once with each error table registered
 *    with NSPR.  The callback is provided with the error table and
 *    the plugin's private structure.  The callback returns any table private
 *    data it wishes to associate with the error table.  Does not need to be thread
 *    safe.
 */
typedef struct PRErrorCallbackTablePrivate *
PRErrorCallbackNewTableFn(const struct PRErrorTable *table,
                          struct PRErrorCallbackPrivate *cb_private);

/**********************************************************************/
/****************************** FUNCTIONS *****************************/
/**********************************************************************/

/***********************************************************************
** FUNCTION:    PR_ErrorToString
** DESCRIPTION:
**  Returns the UTF-8 message for an error code in
**  the requested language.  May return the message
**  in the default language if a translation in the requested
**  language is not available.  The returned string is
**  valid for the duration of the process.  Never returns NULL.
**
***********************************************************************/
NSPR_API(const char *) PR_ErrorToString(PRErrorCode code,
                                        PRLanguageCode language);


/***********************************************************************
** FUNCTION:    PR_ErrorToName
** DESCRIPTION:
**  Returns the macro name for an error code, or NULL
**  if the error code is not known.  The returned string is
**  valid for the duration of the process.
**
**  Does not work for error table 0, the system error codes.
**
***********************************************************************/
NSPR_API(const char *) PR_ErrorToName(PRErrorCode code);


/***********************************************************************
** FUNCTION:    PR_ErrorLanguages
** DESCRIPTION:
**  Returns the RFC 1766 language tags for the language
**  codes PR_ErrorToString() supports.  The returned array is valid
**  for the duration of the process.  Never returns NULL.  The first
**  item in the returned array is the language tag for PRLanguageCode 0,
**  the second is for PRLanguageCode 1, and so on.  The array is terminated
**  with a null pointer.
**
***********************************************************************/
NSPR_API(const char * const *) PR_ErrorLanguages(void);


/***********************************************************************
** FUNCTION:    PR_ErrorInstallTable
** DESCRIPTION:
**  Registers an error table with NSPR.  Must be done exactly once per
**  table.  Memory pointed to by `table' must remain valid for the life
**  of the process.
**
**  NOT THREAD SAFE!
**
***********************************************************************/
NSPR_API(PRErrorCode) PR_ErrorInstallTable(const struct PRErrorTable *table);


/***********************************************************************
** FUNCTION:    PR_ErrorInstallCallback
** DESCRIPTION:
**  Registers an error localization plugin with NSPR.  May be called
**  at most one time.  `languages' contains the language codes supported
**  by this plugin.  Languages 0 and 1 must be "i-default" and "en"
**  respectively.  `lookup' and `newtable' contain pointers to
**  the plugin callback functions.  `cb_private' contains any information
**  private to the plugin functions.
**
**  NOT THREAD SAFE!
**
***********************************************************************/
NSPR_API(void) PR_ErrorInstallCallback(const char * const * languages,
                                       PRErrorCallbackLookupFn *lookup,
                                       PRErrorCallbackNewTableFn *newtable,
                                       struct PRErrorCallbackPrivate *cb_private);

PR_END_EXTERN_C

#endif /* prerror_h___ */
