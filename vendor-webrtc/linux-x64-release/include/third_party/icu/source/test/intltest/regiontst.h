// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/************************************************************************
 * COPYRIGHT:
 * Copyright (c) 2013, International Business Machines Corporation
 * and others. All Rights Reserved.
 ************************************************************************/

#ifndef _REGIONTEST_
#define _REGIONTEST_

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/region.h"
#include "intltest.h"

/**
 * Performs various tests on Region APIs
 **/
class RegionTest: public IntlTest {
    // IntlTest override
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par ) override;

public:
    RegionTest();
    virtual ~RegionTest();
    
    void TestKnownRegions();
    void TestGetInstanceString();
    void TestGetInstanceInt();
    void TestGetContainedRegions();
    void TestGetContainedRegionsWithType();
    void TestGetContainingRegion();
    void TestGetContainingRegionWithType();
    void TestGetPreferredValues();
    void TestContains();
    void TestAvailableTerritories();
    void TestNoContainedRegions();
    void TestGroupingChildren();

private:

    UBool optionv; // true if @v option is given on command line
};

#endif /* #if !UCONFIG_NO_FORMATTING */
 
#endif // _REGIONTEST_
//eof
