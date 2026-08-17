// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2007, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef __IntlCalendarTest__
#define __IntlCalendarTest__
 
#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/calendar.h"
#include "unicode/smpdtfmt.h"
#include "caltztst.h"

class IntlCalendarTest: public CalendarTimeZoneTest {
public:
    // IntlTest override
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par ) override;
public:
    void TestTypes();

    void TestGregorian();

    void TestBuddhist();
    void TestBuddhistFormat();
    void TestBug21043Indian();
    void TestBug21044Hebrew();
    void TestBug21045Islamic();
    void TestBug21046IslamicUmalqura();

    void TestTaiwan();

    void TestJapanese();
    void TestJapaneseFormat();
    void TestJapanese3860();
    void TestForceGannenNumbering();
    
    void TestPersian();
    void TestPersianFormat();

    void TestConsistencyGregorian();
    void TestConsistencyCoptic();
    void TestConsistencyEthiopic();
    void TestConsistencyROC();
    void TestConsistencyChinese();
    void TestConsistencyDangi();
    void TestConsistencyBuddhist();
    void TestConsistencyEthiopicAmeteAlem();
    void TestConsistencyHebrew();
    void TestConsistencyIndian();
    void TestConsistencyIslamic();
    void TestConsistencyIslamicCivil();
    void TestConsistencyIslamicRGSA();
    void TestConsistencyIslamicTBLA();
    void TestConsistencyIslamicUmalqura();
    void TestConsistencyPersian();
    void TestConsistencyJapanese();

 protected:
    // Test a Gregorian-Like calendar
    void quasiGregorianTest(Calendar& cal, const Locale& gregoLocale, const int32_t *data);
    void simpleTest(const Locale& loc, const UnicodeString& expect, UDate expectDate, UErrorCode& status);
    void checkConsistency(const char* locale);

    int32_t daysToCheckInConsistency;
 
public: // package
    // internal routine for checking date
    static UnicodeString value(Calendar* calendar);
 
};


#endif /* #if !UCONFIG_NO_FORMATTING */
 
#endif // __IntlCalendarTest__
