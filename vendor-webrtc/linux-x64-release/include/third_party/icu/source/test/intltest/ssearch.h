// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
 **********************************************************************
 *   Copyright (C) 2005-2012, International Business Machines
 *   Corporation and others.  All Rights Reserved.
 **********************************************************************
 */

#ifndef __SSEARCH_H
#define __SSEARCH_H

#include "unicode/utypes.h"
#include "unicode/unistr.h"
#include "unicode/ucol.h"

#include "intltest.h"

#if !UCONFIG_NO_COLLATION

//
//  Test of the function usearch_search()
//
//     See srchtest.h for the tests for the rest of the string search functions.
//
class SSearchTest: public IntlTest {
public:
  
    SSearchTest();
    virtual ~SSearchTest();

    virtual void runIndexedTest(int32_t index, UBool exec, const char* &name, char* params = nullptr ) override;
#if !UCONFIG_NO_BREAK_ITERATION

    virtual void searchTest();
    virtual void offsetTest();
    virtual void monkeyTest(char *params);
    virtual void sharpSTest();
    virtual void goodSuffixTest();
    virtual void searchTime();

private:
    virtual const char   *getPath(char buffer[2048], const char *filename);
    virtual       int32_t monkeyTestCase(UCollator *coll, const UnicodeString &testCase, const UnicodeString &pattern, const UnicodeString &altPattern,
                                         const char *name, const char *strength, uint32_t seed);
#endif
};

#endif

#endif
