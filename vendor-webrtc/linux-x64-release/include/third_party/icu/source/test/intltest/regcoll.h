// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2014, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

/**
 * Collation regression tests.
 * (So we'll have no regrets later)
 */

#ifndef _REGCOLL
#define _REGCOLL

#include "unicode/utypes.h"

#if !UCONFIG_NO_COLLATION

#include "unicode/coleitr.h"
#include "tscoll.h"

class CollationRegressionTest: public IntlTestCollator
{
public:

    // If this is too small for the test data, just increase it.
    // Just don't make it too large, otherwise the executable will get too big
    enum EToken_Len { MAX_TOKEN_LEN = 32 };

    CollationRegressionTest();
    virtual ~CollationRegressionTest();

    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;

    // @bug 4048446
    //
    // CollationElementIterator.reset() doesn't work
    //
    void Test4048446(/* char* par */);

    // @bug 4051866
    //
    // Collator -> rules -> Collator round-trip broken for expanding characters
    //
    void Test4051866(/* char* par */);

    // @bug 4053636
    //
    // Collator thinks "black-bird" == "black"
    //
    void Test4053636(/* char* par */);


    // @bug 4054238
    //
    // CollationElementIterator will not work correctly if the associated
    // Collator object's mode is changed
    //
    void Test4054238(/* char* par */);

    // @bug 4054734
    //
    // Collator.IDENTICAL documented but not implemented
    //
    void Test4054734(/* char* par */);

    // @bug 4054736
    //
    // Full Decomposition mode not implemented
    //
    void Test4054736(/* char* par */);

    // @bug 4058613
    //
    // Collator.getInstance() causes an ArrayIndexOutofBoundsException for Korean  
    //
    void Test4058613(/* char* par */);
    
    // @bug 4059820
    //
    // RuleBasedCollator.getRules does not return the exact pattern as input
    // for expanding character sequences
    //
    void Test4059820(/* char* par */);

    // @bug 4060154
    //
    // MergeCollation::fixEntry broken for "& H < \u0131, \u0130, i, I"
    //
    void Test4060154(/* char* par */);

    // @bug 4062418
    //
    // Secondary/Tertiary comparison incorrect in French Secondary
    //
    void Test4062418(/* char* par */);

    // @bug 4065540
    //
    // Collator.compare() method broken if either string contains spaces
    //
    void Test4065540(/* char* par */);

    // @bug 4066189
    //
    // Unicode characters need to be recursively decomposed to get the
    // correct result. For example,
    // u1EB1 -> \u0103 + \u0300 -> a + \u0306 + \u0300.
    //
    void Test4066189(/* char* par */);

    // @bug 4066696
    //
    // French secondary collation checking at the end of compare iteration fails
    //
    void Test4066696(/* char* par */);


    // @bug 4076676
    //
    // Bad canonicalization of same-class combining characters
    //
    void Test4076676(/* char* par */);


    // @bug 4078588
    //
    // RuleBasedCollator breaks on "< a < bb" rule
    //
    void Test4078588(/* char* par */);

    // @bug 4079231
    //
    // RuleBasedCollator.equals(null) throws NullPointerException
    //
    void Test4079231(/* char* par */);

    // @bug 4081866
    //
    // Combining characters in different classes not reordered properly.
    //
    void Test4081866(/* char* par */);

    // @bug 4087241
    //
    // string comparison errors in Scandinavian collators
    //
    void Test4087241(/* char* par */);

    // @bug 4087243
    //
    // CollationKey takes ignorable strings into account when it shouldn't
    //
    void Test4087243(/* char* par */);

    // @bug 4092260
    //
    // Mu/micro conflict
    // Micro symbol and greek lowercase letter Mu should sort identically
    //
    void Test4092260(/* char* par */);

    // @bug 4095316
    //
    void Test4095316(/* char* par */);

    // @bug 4101940
    //
    void Test4101940(/* char* par */);

    // @bug 4103436
    //
    // Collator.compare not handling spaces properly
    //
    void Test4103436(/* char* par */);

    // @bug 4114076
    //
    // Collation not Unicode conformant with Hangul syllables
    //
    void Test4114076(/* char* par */);
    
    
    // @bug 4114077
    //
    // Collation with decomposition off doesn't work for Europe 
    //
    void Test4114077(/* char* par */);

    // @bug 4124632
    //
    // Collator.getCollationKey was hanging on certain character sequences
    //
    void Test4124632(/* char* par */);
    
    // @bug 4132736
    //
    // sort order of french words with multiple accents has errors
    //
    void Test4132736(/* char* par */);
    
    // @bug 4133509
    //
    // The sorting using java.text.CollationKey is not in the exact order
    //
    void Test4133509(/* char* par */);

    // @bug 4139572
    //
    // getCollationKey throws exception for spanish text 
    // Cannot reproduce this bug on 1.2, however it DOES fail on 1.1.6
    //
    void Test4139572(/* char* par */);
    
    // @bug 4141640
    //
    // Support for Swedish gone in 1.1.6 (Can't create Swedish collator) 
    //
    void Test4141640(/* char* par */);

    void Test4179216();

    // Ticket 7189
    //
    // nextSortKeyPart incorrect for EO_S1 collation
    //
    void TestT7189();

    // Ticket 8624
    //
    // Tertiary value compression problem with case first option enabled
    //
    void TestCaseFirstCompression();

    void TestTrailingComment();
    void TestBeforeWithTooStrongAfter();

    // Test use-of-uninitialized-value
    void TestICU22277();

    void TestICU22517();

private:
    //------------------------------------------------------------------------
    // Internal utilities
    //
    void compareArray(Collator &c,
                    const char16_t tests[][CollationRegressionTest::MAX_TOKEN_LEN],
                    int32_t testCount);

    void assertEqual(CollationElementIterator &i1, CollationElementIterator &i2);


    RuleBasedCollator *en_us;

    void caseFirstCompressionSub(Collator *col, UnicodeString opt);
};

#endif /* #if !UCONFIG_NO_COLLATION */

#endif
