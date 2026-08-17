// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 2002-2010, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/


#ifndef _UOBJECTTEST_
#define _UOBJECTTEST_

#include "intltest.h"

/** 
 * Test uobjtest.h
 **/
class UObjectTest : public IntlTest {
    // IntlTest override
    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par ) override;
private:
    // tests
    void testIDs();
    void testUMemory();
    void TestMFCCompatibility();
    void TestCompilerRTTI();

    //helper

    /**
     * @param obj The UObject to be tested
     * @param className The name of the class being tested 
     * @param factory String version of obj, for exanple   "new UFoo(1,3,4)". nullptr if object is abstract.
     * @param staticID The result of class :: getStaticClassID
     * @return Returns obj, suitable for deletion
     */
    UObject *testClass(UObject *obj,
               const char *className, const char *factory, 
               UClassID staticID);

    UObject *testClassNoClassID(UObject *obj,
               const char *className, const char *factory);
};

#endif
//eof
