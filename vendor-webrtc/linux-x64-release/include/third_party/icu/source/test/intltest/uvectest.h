// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
******************************************************************************
* Copyright (C) 2004, International Business Machines Corporation and        *
* others. All Rights Reserved.                                               *
******************************************************************************
*/

//  file:  uvectest.h


#ifndef UVECTEST_H
#define UVECTEST_H

#include "unicode/utypes.h"

#include "intltest.h"


class UVectorTest: public IntlTest {
public:
  
    UVectorTest();
    virtual ~UVectorTest();

    virtual void runIndexedTest(int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;

    // The following are test functions that are visible from the intltest test framework.
    void UVector_API();
    void UStack_API();
    void Hashtable_API();

};

#endif
