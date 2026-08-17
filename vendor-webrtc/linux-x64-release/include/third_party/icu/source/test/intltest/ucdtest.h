// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * Copyright (c) 1997-2016, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#include "unicode/uniset.h"
#include "intltest.h"

/** Helper function for TestUnicodeData */
U_CFUNC void U_CALLCONV unicodeDataLineFn(void *context,
                              char *fields[][2], int32_t fieldCount,
                              UErrorCode *pErrorCode);

U_CFUNC void U_CALLCONV
derivedPropsLineFn(void *context,
                   char *fields[][2], int32_t fieldCount,
                   UErrorCode *pErrorCode);

U_NAMESPACE_BEGIN

class Hashtable;

U_NAMESPACE_END

/** 
 * Test API and functionality of class Unicode
 **/
class UnicodeTest: public IntlTest {
public:
    UnicodeTest();
    virtual ~UnicodeTest();

    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;

    void TestAdditionalProperties();
    void TestBinaryValues();
    void TestConsistency();
    void TestPatternProperties();
    void TestScriptMetadata();
    void TestBidiPairedBracketType();
    void TestEmojiProperties();
    void TestEmojiPropertiesOfStrings();
    void TestIndicPositionalCategory();
    void TestIndicSyllabicCategory();
    void TestVerticalOrientation();
    void TestDefaultScriptExtensions();
    void TestInvalidCodePointFolding();
    void TestBinaryCharacterProperties();
    void TestIntCharacterProperties();
    void TestPropertyNames();
    void TestIDSUnaryOperator();
    void TestIDCompatMath();

private:

    friend void U_CALLCONV unicodeDataLineFn(void *context,
                              char *fields[][2], int32_t fieldCount,
                              UErrorCode *pErrorCode);

    friend void U_CALLCONV
    derivedPropsLineFn(void *context,
                           char *fields[][2], int32_t fieldCount,
                           UErrorCode *pErrorCode);

    UnicodeSet derivedProps[30];
    U_NAMESPACE_QUALIFIER Hashtable *unknownPropertyNames;

    UBool compareUSets(const UnicodeSet &a, const UnicodeSet &b,
                       const char *a_name, const char *b_name,
                       UBool diffIsError);
};
