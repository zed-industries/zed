// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2012, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/********************************************************************************
*
* File CAPITEST.H
*
* Modification History:
*        Name                     Description            
*     Madhu Katragadda               Creation
*********************************************************************************/
/* C API TEST FOR CALENDAR */
#ifndef _CCALTST
#define _CCALTST

#include "unicode/utypes.h"
#include "unicode/ucal.h"
#include "unicode/udat.h"

#if !UCONFIG_NO_FORMATTING

#include "cintltst.h"


    /**
     * The function used to test the Calendar API
     **/
    static void TestCalendar(void);
    /**
     * The function used to test getMillis, setMillis, setDate and setDateTime functions extensively
     **/
    static void TestGetSetDateAPI(void);
    /**
     * This function is used to test and confirm the functioning of 
     * the calendar get and set functions of calendar fields.
     **/
    static void TestFieldGetSet(void);
    /**
     * Execute and test adding and rolling extensively.
     **/
    static void TestAddRollExtensive(void);
    /**
     *Testing the Limits for various Fields of Calendar
     **/
    static void TestGetLimits(void);
    /**
     * Test that the days of the week progress properly when add is called repeatedly
     * for increments of 24 days.
     **/
    static void TestDOWProgression(void);
    /**
     * Confirm that the offset between local time and GMT behaves as expected.
     **/
    static void TestGMTvsLocal(void);
    /**
     * test subroutine used by TestGMTvsLocal()
     */
    static void testZones(int32_t, int32_t, int32_t, int32_t, int32_t, int32_t);
    /**
     * Test getKeywordValuesForLocale API
     */
    static void TestGetKeywordValuesForLocale(void);
    /**
     * Test weekend-related APIs
     */
    static void TestWeekend(void);
    /**
     * Test ambiguous wall time
     */
    static void TestAmbiguousWallTime(void);
    /**
     * Test ucal_getIanaTimeZoneID()
     */
    static void TestGetIanaTimeZoneID(void);

/*Internal functions used*/
    /**
     * test subroutines used by TestAddRollExtensive()
     **/
    static void checkDate(UCalendar* c, int32_t y, int32_t m, int32_t d);

    static void checkDateTime(UCalendar* c, int32_t y, int32_t m, int32_t d, 
                            int32_t hr, int32_t min, int32_t sec, int32_t ms, 
                                                    UCalendarDateFields field);

    /**
     * test subroutines used by TestGetSetDateAPI and TestFieldGetSet
     **/
    static void verify1(const char* msg, UCalendar* c, UDateFormat* dat, int32_t year, int32_t month, int32_t day);

    static void verify2(const char* msg, UCalendar* c, UDateFormat* dat, int32_t year, int32_t month, int32_t day,
                                                                int32_t hour, int32_t min, int32_t sec, int32_t am_pm);

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif
