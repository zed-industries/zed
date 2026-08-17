// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2002, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef CHARITERTEST_H
#define CHARITERTEST_H

#include "intltest.h"
#include "unicode/uiter.h"

/**
 * Some tests for CharacterIterator and StringCharacterIterator
 **/
class CharIterTest: public IntlTest {
public:
    CharIterTest();
    
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;

    /**
     * Test Constructors and operators ==, != and a few other methods
     **/
    void TestConstructionAndEquality();
    /**
     * Test Constructors and operators ==, != and a few other methods for UChariter
     **/
    void TestConstructionAndEqualityUChariter();
    /**
     * test the iteration functionality in different ways
     **/
    void TestIteration();
     /**
     * test the iteration functionality in different ways with  unicodestring of UChar32's
     **/
    void TestIterationUChar32();

    void TestUCharIterator();
    void TestUCharIterator(UCharIterator *iter, CharacterIterator &ci, const char *moves, const char *which);
    void TestCoverage();
    void TestCharIteratorSubClasses();
};

#endif

