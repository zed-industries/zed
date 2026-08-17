// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2001, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/********************************************************************************
*
* File CBIAPTS.H
*
* Modification History:
*        Name                     Description            
*     Madhu Katragadda               Creation
*********************************************************************************/
/* C API TEST FOR BREAKITERATOR */

#ifndef _CBRKITRAPITST
#define _CBRKITRAPITST

#include "unicode/utypes.h"

#if !UCONFIG_NO_BREAK_ITERATION

#include "cintltst.h"

/**
* This is an API test.  It doesn't test very many cases, and doesn't
* try to test the full functionality.  It just calls each function in the class and
* verifies that it works on a basic level.
**/
/* The function used to test the BreakIterator API*/
static void TestBreakIteratorCAPI(void);

#endif /* #if !UCONFIG_NO_BREAK_ITERATION */

#endif
