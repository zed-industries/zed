// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2001, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/********************************************************************************
*
* File CJAPTST.H
*
* Modification History:
*        Name                     Description            
*     Madhu Katragadda            Converted to C
* synwee                          Added TestBase, TestPlainDakutenHandakuten,
*                                 TestSmallLarge, TestKatakanaHiragana,
*                                 TestChooonKigoo
*********************************************************************************/
/**
 * CollationKannaTest(JAPAN) is a third level test class.  This tests the locale
 * specific primary, secondary and tertiary rules.  For example, the ignorable
 * character '-' in string "black-bird".  The en_US locale uses the default
 * collation rules as its sorting sequence.
 */

#ifndef _CJACOLLTST
#define _CJACOLLTST

#include "unicode/utypes.h"

#if !UCONFIG_NO_COLLATION

#include "cintltst.h"


#define MAX_TOKEN_LEN 16


     /*perform test with strength TERTIARY*/
static    void TestTertiary(void);

/* Testing base letters */
static void TestBase(void);

/* Testing plain, Daku-ten, Handaku-ten letters */
static void TestPlainDakutenHandakuten(void);

/* 
* Test Small, Large letters
*/
static void TestSmallLarge(void); 

/*
* Test Katakana, Hiragana letters
*/
static void TestKatakanaHiragana(void);

/*
* Test Choo-on kigoo
*/
static void TestChooonKigoo(void);

#endif /* #if !UCONFIG_NO_COLLATION */

#endif
