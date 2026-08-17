// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
**********************************************************************
* Copyright (C) 2000-2004, International Business Machines Corporation 
* and others.  All Rights Reserved.
**********************************************************************

Get a message out of the default resource bundle, messageformat it,
and print it to stderr
*/

#ifndef _UWMSG
#define _UWMSG

#include <stdio.h>

#include "unicode/ures.h"

/* Set the path to wmsg's bundle.
   Caller owns storage.
*/
U_CFUNC UResourceBundle *u_wmsg_setPath(const char *path, UErrorCode *err);

/* Format a message and print it's output to a given file stream */
U_CFUNC int u_wmsg(FILE *fp, const char *tag, ... );

/* format an error message */
U_CFUNC const UChar* u_wmsg_errorName(UErrorCode err);

#endif
