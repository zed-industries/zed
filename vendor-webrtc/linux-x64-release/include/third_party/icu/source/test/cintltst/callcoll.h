// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2008, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/********************************************************************************
*
* File CALLCOLL.H
*
* Modification History:
*        Name                     Description            
*     Madhu Katragadda              Ported to C
*********************************************************************************
*/
/**
 * CollationDummyTest is a third level test class.  This tests creation of 
 * a customized collator object.  For example, number 1 to be sorted 
 * equlivalent to word 'one'.
 */
#ifndef _CALLCOLLTST
#define _CALLCOLLTST

#include "unicode/utypes.h"
#include "unicode/ucoleitr.h"

#if !UCONFIG_NO_COLLATION

#include "cintltst.h"

#define RULE_BUFFER_LEN 8192

struct OrderAndOffset
{
    int32_t order;
    int32_t offset;
};

typedef struct OrderAndOffset OrderAndOffset;

    /* tests comparison of custom collation with different strengths */
void doTest(UCollator*, const UChar* source, const UChar* target, UCollationResult result);
/* verify that iterating forward and backwards over the string yields same CEs */
void backAndForth(UCollationElements *iter);
/* gets an array of CEs for a string in UCollationElements iterator. */
OrderAndOffset* getOrders(UCollationElements *iter, int32_t *orderLength);

void genericOrderingTestWithResult(UCollator *coll, const char * const s[], uint32_t size, UCollationResult result);
void genericOrderingTest(UCollator *coll, const char * const s[], uint32_t size);
void genericLocaleStarter(const char *locale, const char * const s[], uint32_t size);
void genericLocaleStarterWithResult(const char *locale, const char * const s[], uint32_t size, UCollationResult result);
void genericLocaleStarterWithOptions(const char *locale, const char * const s[], uint32_t size, const UColAttribute *attrs, const UColAttributeValue *values, uint32_t attsize);
void genericLocaleStarterWithOptionsAndResult(const char *locale, const char * const s[], uint32_t size, const UColAttribute *attrs, const UColAttributeValue *values, uint32_t attsize, UCollationResult result);
void genericRulesStarterWithResult(const char *rules, const char * const s[], uint32_t size, UCollationResult result);
void genericRulesStarter(const char *rules, const char * const s[], uint32_t size);
void genericRulesStarterWithOptionsAndResult(const char *rules, const char * const s[], uint32_t size, const UColAttribute *attrs, const UColAttributeValue *values, uint32_t attsize, UCollationResult result);
UBool hasCollationElements(const char *locName);


#endif /* #if !UCONFIG_NO_COLLATION */

#endif
