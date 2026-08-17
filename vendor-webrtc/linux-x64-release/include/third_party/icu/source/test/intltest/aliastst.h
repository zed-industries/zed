// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 2005-2006, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
#ifndef _ALIASTST
#define _ALIASTST

#include "intltest.h"
#include "unicode/locid.h"
#include "unicode/ures.h"

class LocaleAliasTest: public IntlTest {
public:
    void TestCalendar();
    void TestDateFormat();
    void TestCollation();
    void TestULocale();
    void TestUResourceBundle();
    void TestDisplayName(); 
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;
    LocaleAliasTest();
    virtual ~LocaleAliasTest();
private:
    UResourceBundle* resIndex;
    UBool isLocaleAvailable(const char*);
    Locale defLocale; 
};

#endif
