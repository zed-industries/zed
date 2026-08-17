// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 2002-2006, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************
 *
 * @author Mark E. Davis
 * @author Vladimir Weinstein
 */

/**
 * Test Canonical Iterator
 */

#ifndef _CANITTST
#define _CANITTST

#include "unicode/utypes.h"

#if !UCONFIG_NO_NORMALIZATION


U_NAMESPACE_BEGIN

class Transliterator;

U_NAMESPACE_END

#include "unicode/translit.h"
#include "unicode/caniter.h"
#include "intltest.h"
#include "hash.h"

class CanonicalIteratorTest : public IntlTest {
public:
    CanonicalIteratorTest();
    virtual ~CanonicalIteratorTest();

    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;

    void TestCanonicalIterator();
    void TestExhaustive();
    void TestBasic();
    void TestAPI();
    UnicodeString collectionToString(Hashtable *col);
    //static UnicodeString collectionToString(Collection col);
private:
    void expectEqual(const UnicodeString &message, const UnicodeString &item, const UnicodeString &a, const UnicodeString &b);
    void characterTest(UnicodeString &s, UChar32 ch, CanonicalIterator &it);

    Transliterator *nameTrans;
    Transliterator *hexTrans;
        
    UnicodeString getReadable(const UnicodeString &obj);
};

#endif /* #if !UCONFIG_NO_NORMALIZATION */

#endif // _CANITTST
