// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 2005-2016, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/************************************************************************
*   Tests for the UText and UTextIterator text abstraction classes
*
************************************************************************/


#ifndef UTXTTEST_H
#define UTXTTEST_H

#include "unicode/utypes.h"
#include "unicode/unistr.h"
#include "unicode/utext.h"

#include "intltest.h"

/**
 * @test
 * @summary Testing the Replaceable class
 */
class UTextTest : public IntlTest {
public:
    UTextTest();
    virtual ~UTextTest();

    void runIndexedTest(int32_t index, UBool exec, const char* &name, char* par=nullptr) override;
    void TextTest();
    void ErrorTest();
    void FreezeTest();
    void Ticket5560();
    void Ticket6847();
    void Ticket10562();
    void Ticket10983();
    void Ticket12130();
    void Ticket13344();
    void AccessChangesChunkSize();

private:
    struct m {                              // Map between native indices & code points.
        int         nativeIdx;
        UChar32     cp;
    };

    void TestString(const UnicodeString &s);
    void TestAccess(const UnicodeString &us, UText *ut, int cpCount, m *cpMap);
    void TestAccessNoClone(const UnicodeString &us, UText *ut, int cpCount, m *cpMap);
    void TestCMR   (const UnicodeString &us, UText *ut, int cpCount, m *nativeMap, m *utf16Map);
    void TestCopyMove(const UnicodeString &us, UText *ut, UBool move,
                      int32_t nativeStart, int32_t nativeLimit, int32_t nativeDest,
                      int32_t u16Start, int32_t u16Limit, int32_t u16Dest);
    void TestReplace(const UnicodeString &us,  // reference UnicodeString in which to do the replace 
            UText         *ut,                 // UnicodeText object under test.
            int32_t       nativeStart,         // Range to be replaced, in UText native units. 
            int32_t       nativeLimit,
            int32_t       u16Start,            // Range to be replaced, in UTF-16 units
            int32_t       u16Limit,            //    for use in the reference UnicodeString.
            const UnicodeString &repStr);      // The replacement string


};


#endif
