// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
**********************************************************************
*   Copyright (C) 2001-2009, International Business Machines
*   Corporation and others.  All Rights Reserved.
**********************************************************************
*   Date        Name        Description
*   05/23/00    aliu        Creation.
**********************************************************************
*/
#ifndef TESTUTIL_H
#define TESTUTIL_H

#include "unicode/utypes.h"
#include "unicode/edits.h"
#include "unicode/unistr.h"
#include "intltest.h"

struct EditChange {
    UBool change;
    int32_t oldLength, newLength;
};

/**
 * Utility methods. Everything in this class is static.
 */
class TestUtility {
public:
    static UnicodeString &appendHex(UnicodeString &buf, UChar32 ch);

    static UnicodeString hex(UChar32 ch);

    static UnicodeString hex(const UnicodeString& s);

    static UnicodeString hex(const UnicodeString& s, char16_t sep);

    static UnicodeString hex(const uint8_t* bytes, int32_t len);

    static UBool checkEqualEdits(IntlTest &test, const UnicodeString &name,
                                 const Edits &e1, const Edits &e2, UErrorCode &errorCode);

    static void checkEditsIter(
        IntlTest &test, const UnicodeString &name,
        Edits::Iterator ei1, Edits::Iterator ei2,  // two equal iterators
        const EditChange expected[], int32_t expLength, UBool withUnchanged,
        UErrorCode &errorCode);

private:
    TestUtility() = delete;  // Prevent instantiation
};

#endif
