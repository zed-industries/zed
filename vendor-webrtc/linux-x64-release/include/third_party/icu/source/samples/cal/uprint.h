/*
***********************************************************************
* © 2016 and later: Unicode, Inc. and others.
* License & terms of use: http://www.unicode.org/copyright.html
***********************************************************************
**********************************************************************
* Copyright (C) 1998-2004, International Business Machines Corporation
* and others.  All Rights Reserved.
**********************************************************************
*
* File uprint.h
*
* Modification History:
*
*   Date        Name        Description
*   06/14/99    stephen     Creation.
*******************************************************************************
*/

#ifndef UPRINT_H
#define UPRINT_H 1

#include <stdio.h>

#include "unicode/utypes.h"

/* Print a ustring to the specified FILE* in the default codepage */
U_CFUNC void uprint(const UChar *s, FILE *f, UErrorCode *status);

#endif /* ! UPRINT_H */
