// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2015, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/


#ifndef MULTITHREADTEST_H
#define MULTITHREADTEST_H

#include "intltest.h"
#include "mutex.h"



/**
 * Tests actual threading
 **/
class MultithreadTest : public IntlTest
{
public:
    MultithreadTest();
    virtual ~MultithreadTest();
    
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;

    /**
     * test that threads even work
     **/
    void TestThreads();

	/**
     * test that arabic shaping can work in threads
     **/
    void TestArabicShapingThreads();
	
#if !UCONFIG_NO_FORMATTING
    /**
     * test that intl functions work in a multithreaded context
     **/
    void TestThreadedIntl();
#endif
    void TestCollators();
    void TestString();
    void TestAnyTranslit();
    void TestUnifiedCache();
    void TestBreakTranslit();
    void TestIncDec();
    void Test20104();
};

#endif

