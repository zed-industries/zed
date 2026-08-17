// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2008, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

/**
 * MajorTestLevel is the top level test class for everything in the directory "IntlWork".
 */

#ifndef _INTLTESTCOLLATOR
#define _INTLTESTCOLLATOR

#include "unicode/utypes.h"

#if !UCONFIG_NO_COLLATION

#include "intltest.h"
#include "unicode/coll.h"
#include "unicode/coleitr.h"


class IntlTestCollator: public IntlTest {
    void runIndexedTest(int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;
protected:
    struct Order
    {
        int32_t order;
        int32_t offset;
    };

    // These two should probably go down in IntlTest
    void doTest(Collator* col, const char16_t *source, const char16_t *target, Collator::EComparisonResult result);

    void doTest(Collator* col, const UnicodeString &source, const UnicodeString &target, Collator::EComparisonResult result);
    void doTestVariant(Collator* col, const UnicodeString &source, const UnicodeString &target, Collator::EComparisonResult result);
    virtual void reportCResult( const UnicodeString &source, const UnicodeString &target,
                                CollationKey &sourceKey, CollationKey &targetKey,
                                Collator::EComparisonResult compareResult,
                                Collator::EComparisonResult keyResult,
                                Collator::EComparisonResult incResult,
                                Collator::EComparisonResult expectedResult );

    static UnicodeString &prettify(const CollationKey &source, UnicodeString &target);
    static UnicodeString &appendCompareResult(Collator::EComparisonResult result, UnicodeString &target);
    void backAndForth(CollationElementIterator &iter);
    /**
     * Return an integer array containing all of the collation orders
     * returned by calls to next on the specified iterator
     */
    Order *getOrders(CollationElementIterator &iter, int32_t &orderLength);
    UCollationResult compareUsingPartials(UCollator *coll, const char16_t source[], int32_t sLen, const char16_t target[], int32_t tLen, int32_t pieceSize, UErrorCode &status);

};

#endif /* #if !UCONFIG_NO_COLLATION */

#endif
