// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * Copyright (c) 1997-2013 International Business Machines 
 * Corporation and others. All Rights Reserved.
 ********************************************************************/
/********************************************************************************
*
* File CAPITEST.H
*
* Modification History:
*        Name                     Description            
*     Madhu Katragadda            Converted to C
*     Brian Rower                 Added TestOpenVsOpenRules
*********************************************************************************
*//* C API TEST For COLLATOR */

#ifndef _CCOLLAPITST
#define _CCOLLAPITST

#include "unicode/utypes.h"

#if !UCONFIG_NO_COLLATION

#include "cintltst.h"
#include "callcoll.h"
#define MAX_TOKEN_LEN 16


    /**
     * error reporting utility method
     **/

    static void doAssert(int condition, const char *message);
    /**
     * Collator Class Properties
     * ctor, dtor, createInstance, compare, getStrength/setStrength
     * getDecomposition/setDecomposition, getDisplayName
     */
    void TestProperty(void);
    /**
     * Test RuleBasedCollator and getRules
     **/
    void TestRuleBasedColl(void);
    
    /**
     * Test compare
     **/
    void TestCompare(void);
    /**
     * Test hashCode functionality
     **/
    void TestHashCode(void);
    /**
     * Tests the constructor and numerous other methods for CollationKey
     **/
   void TestSortKey(void);
    /**
     * test the CollationElementIterator methods
     **/
   void TestElemIter(void);
    /**
     * Test ucol_getAvailable and ucol_countAvailable()
     **/
    void TestGetAll(void);
    /**
     * Test ucol_GetDefaultRules ()
    void TestGetDefaultRules(void);
     **/

    void TestDecomposition(void);
    /**
     * Test ucol_safeClone ()
     **/    
    void TestSafeClone(void);

    /**
     * Test ucol_clone ()
     **/
    void TestClone(void);

    /**
     * Test ucol_cloneBinary(), ucol_openBinary()
     **/
    void TestCloneBinary(void);

    /**
     * Test ucol_open() vs. ucol_openRules()
     **/
    void TestOpenVsOpenRules(void);

    /**
     * Test getting bounds for a sortkey
     */
    void TestBounds(void);

    /**
     * Test ucol_getLocale function
     */
    void TestGetLocale(void);

    /**
     * Test buffer overrun while having smaller buffer for sortkey (j1865)
     */
    void TestSortKeyBufferOverrun(void);
    /**
     * Test getting and setting of attributes
     */
    void TestGetSetAttr(void);
    /**
     * Test getTailoredSet
     */
    void TestGetTailoredSet(void);

    /**
     * Test mergeSortKeys
     */
    void TestMergeSortKeys(void);

    /** 
     * test short string and collator identifier functions
     */
    static void TestShortString(void);

    /** 
     * test getContractions and getUnsafeSet
     */
    static void TestGetContractionsAndUnsafes(void);

    /**
     * Test funny stuff with open binary
     */
    static void TestOpenBinary(void);

    /**
     * Test getKeywordValuesForLocale API
     */
    static void TestGetKeywordValuesForLocale(void);

    /**
     * test strcoll with null arg
     */
    static void TestStrcollNull(void);
 
    /**
     * Simple test for ICU-21460.  The issue affects all components, but was originally reported against collation.
     */
    static void TestLocaleIDWithUnderscoreAndExtension(void);

#endif /* #if !UCONFIG_NO_COLLATION */

#endif
