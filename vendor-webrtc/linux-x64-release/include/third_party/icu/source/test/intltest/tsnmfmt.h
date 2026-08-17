// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2003, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef _INTLTESTNUMBERFORMAT
#define _INTLTESTNUMBERFORMAT


#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/numfmt.h"
#include "unicode/locid.h"
#include "intltest.h"


/**
 * This test does round-trip testing (format -> parse -> format -> parse -> etc.) of
 * NumberFormat.
 */
class IntlTestNumberFormat: public IntlTest {
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;

private:

    /**
     *  call tryIt with many variations, called by testLocale
     **/
    void testFormat(/* char* par */);
    /**
     *  perform tests using aNumber and fFormat, called in many variations
     **/
    void tryIt(double aNumber);
    /**
     *  perform tests using aNumber and fFormat, called in many variations
     **/
    void tryIt(int32_t aNumber);
    /**
     *  test NumberFormat::getAvailableLocales
     **/
    void testAvailableLocales(/* char* par */);
    /**
     *  call testLocale for all locales
     **/
    void monsterTest(/* char *par */);
    /**
     *  call testFormat for currency, percent and plain number instances
     **/
    void testLocale(/* char *par, */const Locale& locale, const UnicodeString& localeName);

    NumberFormat*   fFormat;
    UErrorCode      fStatus;
    Locale          fLocale;

public:

    virtual ~IntlTestNumberFormat();

    /*
     * Return a random double that isn't too large.
     */
    static double getSafeDouble(double smallerThanMax);

    /*
     * Return a random double
     **/
    static double randDouble();

    /*
     * Return a random uint32_t
     **/
    static uint32_t randLong();

    /**
     * Return a random double 0 <= x < 1.0
     **/
    static double randFraction()
    {
        return (double)randLong() / (double)0xFFFFFFFF;
    }

};

#endif /* #if !UCONFIG_NO_FORMATTING */

#endif
