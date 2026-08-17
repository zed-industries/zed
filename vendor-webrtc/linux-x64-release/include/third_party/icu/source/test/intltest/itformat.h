// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2001, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

/**
 * MajorTestLevel is the top level test class for everything in the directory "IntlWork".
 */

#ifndef _INTLTESTFORMAT
#define _INTLTESTFORMAT

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/formattedvalue.h"
#include "intltest.h"


class IntlTestFormat: public IntlTest {
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;
};


typedef struct UFieldPositionWithCategory {
    UFieldCategory category;
    int32_t field;
    int32_t beginIndex;
    int32_t endIndex;
} UFieldPositionWithCategory;

class IntlTestWithFieldPosition : public IntlTest {
public:
    // Tests FormattedValue's toString, toTempString, and nextPosition methods.
    //
    // expectedCategory gets combined with expectedFieldPositions to call
    // checkMixedFormattedValue.
    void checkFormattedValue(
        const char16_t* message,
        const FormattedValue& fv,
        UnicodeString expectedString,
        UFieldCategory expectedCategory,
        const UFieldPosition* expectedFieldPositions,
        int32_t length);

    // Tests FormattedValue's toString, toTempString, and nextPosition methods.
    void checkMixedFormattedValue(
        const char16_t* message,
        const FormattedValue& fv,
        UnicodeString expectedString,
        const UFieldPositionWithCategory* expectedFieldPositions,
        int32_t length);
};


#endif /* #if !UCONFIG_NO_FORMATTING */

#endif
