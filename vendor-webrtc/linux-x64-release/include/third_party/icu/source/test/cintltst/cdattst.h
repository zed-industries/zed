// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2014, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/********************************************************************************
*
* File CDATTST.H
*
* Modification History:
*        Name                     Description            
*     Madhu Katragadda               Creation
*********************************************************************************
*/
/* C API TEST FOR DATE FORMAT */
#ifndef _CDATFRMTST
#define _CDATFRMTST

#include "unicode/utypes.h"
#include "unicode/udat.h"

#if !UCONFIG_NO_FORMATTING

#include "cintltst.h"

    /**
     * The functions used to test the Date format API
     **/
    static void TestDateFormat(void);
    static void TestRelativeDateFormat(void);

    /**
     * The function used to test API  udat_getSymbols(), udat_setSymbols() and udat_countSymbols()
     **/
    static void TestSymbols(void);

    /**
     * Test DateFormat(Calendar) API
     */
    static void TestDateFormatCalendar(void);

    /**
     * test subroutines used by TestSymbols
     **/
    static void VerifygetSymbols(UDateFormat*, UDateFormatSymbolType, int32_t, const char*);
    static void VerifysetSymbols(UDateFormat*, UDateFormatSymbolType, int32_t, const char*);
    static void VerifygetsetSymbols(UDateFormat*, UDateFormat*, UDateFormatSymbolType, int32_t);
    
    /**
     * test subroutine used by the testing functions
     **/
    static UChar* myNumformat(const UNumberFormat* numfor, double d);
    static int getCurrentYear(void);

    /**
     * Test DateFormat override number format API
     */
     static void TestOverrideNumberFormat(void);


#endif /* #if !UCONFIG_NO_FORMATTING */

#endif
