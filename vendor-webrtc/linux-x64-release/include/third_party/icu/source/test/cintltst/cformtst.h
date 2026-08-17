// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2001, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/********************************************************************************
*
* File CFORMTST.H
*
* Modification History:
*        Name                     Description            
*     Madhu Katragadda               Creation
*********************************************************************************
*/
/* FormatTest is a medium top level test for everything in the  C FORMAT API */

#ifndef _CFORMATTST
#define _CFORMATTST

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "cintltst.h"
#include "unicode/udat.h"
#include "unicode/uformattedvalue.h"


/* Internal function used by all the test format files */
UChar* myDateFormat(UDateFormat *dat, UDate d); 


typedef struct UFieldPositionWithCategory {
    UFieldCategory category;
    int32_t field;
    int32_t beginIndex;
    int32_t endIndex;
} UFieldPositionWithCategory;

// The following are implemented in uformattedvaluetest.c
void checkFormattedValue(
    const char* message,
    const UFormattedValue* fv,
    const UChar* expectedString,
    UFieldCategory expectedCategory,
    const UFieldPosition* expectedFieldPositions,
    int32_t expectedFieldPositionsLength);

void checkMixedFormattedValue(
    const char* message,
    const UFormattedValue* fv,
    const UChar* expectedString,
    const UFieldPositionWithCategory* expectedFieldPositions,
    int32_t length);


#endif /* #if !UCONFIG_NO_FORMATTING */

#endif
