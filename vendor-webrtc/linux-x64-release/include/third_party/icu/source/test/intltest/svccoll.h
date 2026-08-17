// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * Copyright (c) 2004, International Business Machines Corporation
 * and others. All Rights Reserved.
 ********************************************************************/

/**
 * CollationServiceTest tests registration of collators.
 */

#ifndef _SVCCOLL
#define _SVCCOLL

#include "unicode/utypes.h"

#if !UCONFIG_NO_COLLATION

#include "intltest.h"

U_NAMESPACE_BEGIN

class StringEnumeration;

class CollationServiceTest: public IntlTest {
public:
    void runIndexedTest(int32_t index, UBool exec, const char* &name, char* /*par = nullptr */) override;

    void TestRegister();
    void TestRegisterFactory();
    void TestSeparateTree();

 private:
    int32_t checkStringEnumeration(const char* msg,
                                   StringEnumeration& iter,
                                   const char** expected,
                                   int32_t expectedCount);

    int32_t checkAvailable(const char* msg);
};

U_NAMESPACE_END

/* #if !UCONFIG_NO_COLLATION */
#endif

/* #ifndef _SVCCOLL */
#endif
