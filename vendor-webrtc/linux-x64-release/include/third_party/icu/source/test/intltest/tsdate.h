// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2003, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef _INTLTESTDATEFORMAT
#define _INTLTESTDATEFORMAT

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/unistr.h"
#include "unicode/datefmt.h"
#include "intltest.h"

/**
 *  Performs some tests in many variations on DateFormat
 **/
class IntlTestDateFormat: public IntlTest {
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;
    
private:

    /**
     *  test DateFormat::getAvailableLocales
     **/
    void testAvailableLocales(/* char* par */);
    /**
     *  call testLocale for all locales
     **/
    void monsterTest(/* char *par */);

    /**
     *  call tryDate with variations, called by testLocale
     **/
    void testFormat(/* char* par */);
    /**
     *  perform tests using date and fFormat, called in many variations
     **/
    void tryDate(UDate date);
    /**
     *  call testFormat for different DateFormat::EStyle's, etc
     **/
    void testLocale(/*char *par, */const Locale&, const UnicodeString&);
    /**
     *  return a random number
     **/
    double randDouble();
    /**
     * generate description for verbose test output
     **/
    void describeTest();

    DateFormat *fFormat;
    UnicodeString fTestName;
    int32_t fLimit; // How many iterations it should take to reach convergence

    // Values in milliseconds (== Date)
    static constexpr int32_t ONESECOND = 1000;
    static constexpr int32_t ONEMINUTE = 60 * ONESECOND;
    static constexpr int32_t ONEHOUR = 60 * ONEMINUTE;
    static constexpr int32_t ONEDAY = 24 * ONEHOUR;

    static constexpr double ONEYEAR = 365.25 * ONEDAY; // Approximate
    enum EMode
    {
        GENERIC,
        TIME,
        DATE,
        DATE_TIME
    };
public:
    virtual ~IntlTestDateFormat();
};

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif
