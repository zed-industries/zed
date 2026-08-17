// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2009, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/


#ifndef _CALTZTST
#define _CALTZTST

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/unistr.h"
#include "unicode/calendar.h"
#include "unicode/datefmt.h"
#include "intltest.h"

/** 
 * This class doesn't perform any tests, but provides utility methods to its subclasses.
 **/
class CalendarTimeZoneTest : public IntlTest
{
public:
    static void cleanup();
protected:
    // Return true if the given status indicates failure.  Also has the side effect
    // of calling errln().  Msg should be of the form "Class::Method" in general.
    UBool failure(UErrorCode status, const char* msg, UBool possibleDataError=false);

    // Utility method for formatting dates for printing; useful for Java->C++ conversion.
    // Tries to mimic the Java Date.toString() format.
    UnicodeString  dateToString(UDate d);
    UnicodeString& dateToString(UDate d, UnicodeString& str);
    UnicodeString& dateToString(UDate d, UnicodeString& str, const TimeZone& z);

    // Utility methods to create a date.  This is useful for converting Java constructs
    // which create a Date object.  Returns a Date in the current local time.
    UDate date(int32_t y, int32_t m, int32_t d, int32_t hr=0, int32_t min=0, int32_t sec=0);

    // Utility methods to create a date.  Returns a Date in UTC.  This will differ
    // from local dates returned by date() by the current default zone offset.
//  Date utcDate(int y, int m, int d, int hr=0, int min=0, int sec=0);  

    // Utility method to get the fields of a date; similar to Date.getYear() etc.
    void dateToFields(UDate date, int32_t& y, int32_t& m, int32_t& d, int32_t& hr, int32_t& min, int32_t& sec);

protected:
    static DateFormat*         fgDateFormat;
    static Calendar*           fgCalendar;

    // the 'get()' functions are not static because they can call errln().
    // they are effectively static otherwise.
     DateFormat*               getDateFormat();
    static void                releaseDateFormat(DateFormat *f);

     Calendar*                 getCalendar();
    static void                releaseCalendar(Calendar *c);
};

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif //_CALTZTST
//eof
