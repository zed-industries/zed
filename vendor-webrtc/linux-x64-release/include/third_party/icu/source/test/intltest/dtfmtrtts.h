// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2006, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef _DATEFORMATROUNDTRIPTEST_
#define _DATEFORMATROUNDTRIPTEST_
 
#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/unistr.h"
#include "unicode/datefmt.h"
#include "unicode/smpdtfmt.h"
#include "unicode/calendar.h"
#include "intltest.h"

/** 
 * Performs round-trip tests for DateFormat
 **/
class DateFormatRoundTripTest : public IntlTest {    
    
    // IntlTest override
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par ) override;

public:
    DateFormatRoundTripTest();
    virtual ~DateFormatRoundTripTest();
    
    void TestDateFormatRoundTrip();
    void TestCentury();
    void test(const Locale& loc);
    void test(DateFormat *fmt, const Locale &origLocale, UBool timeOnly = false );
    int32_t getField(UDate d, int32_t f);
    UnicodeString& escape(const UnicodeString& src, UnicodeString& dst);
    UDate generateDate(); 
    UDate generateDate(UDate minDate); 


//============================================================
// statics
//============================================================

/**
 * Return a random uint32_t
 **/
static uint32_t randLong() {
    // The portable IntlTest::random() function has sufficient
    // resolution for a 16-bit value, but not for 32 bits.
    return ((uint32_t) (IntlTest::random() * (1<<16))) |
          (((uint32_t) (IntlTest::random() * (1<<16))) << 16);
}

/**
 * Return a random double 0 <= x <= 1.0
 **/
static double randFraction()
{
    return (double)randLong() / (double)0xFFFFFFFF;
}

/**
 * Return a random value from -range..+range (closed).
 **/
static double randDouble(double range)
{
    double a = randFraction();
    //while(TPlatformUtilities::isInfinite(a) || TPlatformUtilities::isNaN(a))
    //    a = randFraction();
    return (2.0 * range * a) - range;
}

protected:
    UBool failure(UErrorCode status, const char* msg);
    UBool failure(UErrorCode status, const char* msg, const UnicodeString& str);

    const UnicodeString& fullFormat(UDate d);

private:

    static int32_t SPARSENESS;
    static int32_t TRIALS;
    static int32_t DEPTH;

    UBool optionv; // true if @v option is given on command line
    SimpleDateFormat *dateFormat;
    UnicodeString fgStr;
    Calendar *getFieldCal;
};

#endif /* #if !UCONFIG_NO_FORMATTING */
 
#endif // _DATEFORMATROUNDTRIPTEST_
//eof
