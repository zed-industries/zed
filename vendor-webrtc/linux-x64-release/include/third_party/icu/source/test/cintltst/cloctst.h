// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT:
 * Copyright (c) 1997-2014, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/********************************************************************************
*
* File CLOCTST.H
*
* Modification History:
*        Name                     Description
*     Madhu Katragadda            Converted to C
*********************************************************************************
*/
#ifndef _CLOCTEST
#define _CLOCTEST

#include "cintltst.h"
/*C API TEST FOR LOCALE */

/**
 * Test functions to set and get data fields
 **/
static void TestBasicGetters(void);
static void TestPrefixes(void);
/**
 * Use Locale to access Resource file data and compare against expected values
 **/
static void TestSimpleResourceInfo(void);
/**
 * Use Locale to access Resource file display names and compare against expected values
 **/
static  void TestDisplayNames(void);
static  void TestGetDisplayScriptPreFlighting21160(void);

/**
 * Test getAvailableLocales
 **/
static void TestGetAvailableLocales(void);
static void TestGetAvailableLocalesByType(void);
/**
 * Test functions to set and access a custom data directory
 **/
 static void TestDataDirectory(void);
/**
 * Test functions to test get ISO countries and Languages
 **/
 static void TestISOFunctions(void);
/**
 * Test functions to test get ISO3 countries and Languages Fallback
 **/
 static void TestISO3Fallback(void);
/**
 * Test functions to test get ISO3 countries and Languages for Uninstalled locales
 **/
 static void TestUninstalledISO3Names(void);
 static void TestObsoleteNames(void);
/**
 * Test functions uloc_getDisplaynames()
 **/
 static void TestSimpleDisplayNames(void);
/**
 * Test functions uloc_getDisplaynames()
 **/
 static void TestVariantParsing(void);

 /* Test getting keyword enumeratin */
 static void TestKeywordVariants(void);

 static void TestKeywordSet(void);
 static void TestKeywordSetError(void);

 /* Test getting keyword values */
 static void TestKeywordVariantParsing(void);
 
 /* Test warning for no data in getDisplay* */
 static void TestDisplayNameWarning(void);

 /* Test uloc_getLocaleForLCID */
 static void TestGetLocaleForLCID(void);

/**
 * routine to perform subtests, used by TestDisplayNames
 */
 static void doTestDisplayNames(const char* inLocale, int32_t compareIndex);

 static void TestCanonicalization(void);
 static void TestCanonicalizationBuffer(void);
static  void TestCanonicalization21749StackUseAfterScope(void);

 static void TestDisplayKeywords(void);

 static void TestDisplayKeywordValues(void);

 static void TestGetBaseName(void);

static void TestTrailingNull(void);

static void TestGetLocale(void);

/**
 * additional initialization for datatables storing expected values
 */
static void setUpDataTable(void);
static void cleanUpDataTable(void);
/*static void displayDataTable(void);*/
static void TestAcceptLanguage(void);

/**
 * test locale aliases 
*/
static void TestCalendar(void); 
static void TestDateFormat(void);
static void TestCollation(void);
static void TestULocale(void);
static void TestUResourceBundle(void);
static void TestDisplayName(void);

static void TestAcceptLanguage(void);

static void TestOrientation(void);

static void TestLikelySubtags(void);

/**
 * test terminate correctly.
 */
static void Test21157CorrectTerminating(void);

/**
 * language tag
 */
static void TestForLanguageTag(void);
static void TestToLanguageTag(void);
static void TestBug20132(void);
static void TestLangAndRegionCanonicalize(void);

static void TestToUnicodeLocaleKey(void);
static void TestToLegacyKey(void);
static void TestToUnicodeLocaleType(void);
static void TestToLegacyType(void);
static void TestBug20149(void);
static void TestCDefaultLocale(void);
static void TestBug21449InfiniteLoop(void);


/**
 * U_USING_DEFAULT_WARNING
 */
static void TestUsingDefaultWarning(void);

/**
 * locale data
 */
static void TestEnglishExemplarCharacters(void);

#endif
