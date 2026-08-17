// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
 *******************************************************************************
 *
 *   Copyright (C) 2003-2006, International Business Machines
 *   Corporation and others.  All Rights Reserved.
 *
 *******************************************************************************
 *   file name:  testidna.h
 *   encoding:   UTF-8
 *   tab size:   8 (not used)
 *   indentation:4
 *
 *   created on: 2003feb1
 *   created by: Ram Viswanadha
 */

#ifndef TESTIDNA_H
#define TESTIDNA_H

#include "unicode/utypes.h"

#if !UCONFIG_NO_IDNA && !UCONFIG_NO_TRANSLITERATION

#include "intltest.h"
#include "unicode/parseerr.h"
#include "unicode/uidna.h"

U_CDECL_BEGIN
typedef int32_t  
(U_EXPORT2 *TestFunc) (   const char16_t *src, int32_t srcLength,
                char16_t *dest, int32_t destCapacity,
                int32_t options, UParseError *parseError,
                UErrorCode *status);
typedef int32_t  
(U_EXPORT2 *CompareFunc) (const char16_t *s1, int32_t s1Len,
                const char16_t *s2, int32_t s2Len,
                int32_t options,
                UErrorCode *status);


U_CDECL_END

// test the API

class NamePrepTransform;

/**
 * @test
 * @summary General test of HexadecimalToUnicodeTransliterator
 */
class TestIDNA : public IntlTest {
public:
    void runIndexedTest(int32_t index, UBool exec, const char* &name, char* par=nullptr) override;
    void TestDataFile();
    void TestToASCII();
    void TestToUnicode();
    void TestIDNToUnicode();
    void TestIDNToASCII();
    void TestCompare();
    void TestErrorCases();
    void TestChaining();
    void TestRootLabelSeparator();
    void TestCompareReferenceImpl();
    void TestRefIDNA();
    void TestIDNAMonkeyTest();
    void TestConformance();
    NamePrepTransform* getInstance(UErrorCode& status);
    NamePrepTransform* gPrep;
    TestIDNA() : gPrep(nullptr) {}
    virtual ~TestIDNA();

private:
    void testToASCII(const char* testName, TestFunc func);
    void testToUnicode(const char* testName, TestFunc func);
    void testIDNToUnicode(const char* testName, TestFunc func);
    void testIDNToASCII(const char* testName, TestFunc func);
    void testCompare(const char* testName, CompareFunc func);
    void testChaining(const char* toASCIIName, TestFunc toASCII,
                    const char* toUnicodeName, TestFunc toUnicode);
    void debug(const char16_t* src, int32_t srcLength, int32_t options);
    // main testing functions
    void testAPI(const char16_t *src, const char16_t *expected, const char *testName,
             UBool useSTD3ASCIIRules, UErrorCode expectedStatus,
             UBool doCompare, UBool testUnassigned, TestFunc func, UBool testSTD3ASCIIRules=true);

    void testCompare(const char16_t* s1, int32_t s1Len,
                        const char16_t* s2, int32_t s2Len,
                        const char* testName, CompareFunc func,
                        UBool isEqual);

    void testErrorCases(const char* IDNToASCIIName, TestFunc IDNToASCII,
                    const char* IDNToUnicodeName, TestFunc IDNToUnicode);

    void testChaining(const char16_t* src,int32_t numIterations,const char* testName,
                  UBool useSTD3ASCIIRules, UBool caseInsensitive, TestFunc func);

    void testRootLabelSeparator(const char* testName, CompareFunc func, 
                            const char* IDNToASCIIName, TestFunc IDNToASCII,
                            const char* IDNToUnicodeName, TestFunc IDNToUnicode);

    void testCompareReferenceImpl(const char16_t* src, int32_t srcLen);
    
    UnicodeString testCompareReferenceImpl(UnicodeString& src, 
                                TestFunc refIDNA, const char* refIDNAName,
                                TestFunc uIDNA, const char* uIDNAName,
                                int32_t options);

    void testConformance(const char* toASCIIName, TestFunc toASCII,
                         const char* IDNToASCIIName, TestFunc IDNToASCII,
                         const char* IDNToUnicodeName, TestFunc IDNToUnicode,
                         const char* toUnicodeName, TestFunc toUnicode
                         );

};

// test the TRIE data structure
int testData(TestIDNA& test);

#endif /* #if !UCONFIG_NO_IDNA */

#endif
