// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2001, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/


#ifndef TESTFORMATSMALLCLASSES_H
#define TESTFORMATSMALLCLASSES_H

#include "intltest.h"

/** 
 * tests 3 smaller classes in the format library
 **/
class TestFormatSmallClasses: public IntlTest {
    /**
     * runs tests in 4 local routines,
     * performs test for API and functionality of 3 smaller format classes:
     *    ParsePosition in test_ParsePosition(),
     *    FieldPosition in test_FieldPosition(),
     *    Formattable in test_Formattable().
     **/    
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;
};

#endif
