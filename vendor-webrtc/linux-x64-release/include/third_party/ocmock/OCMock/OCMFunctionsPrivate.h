/*
 *  Copyright (c) 2014-2021 Erik Doernenburg and contributors
 *
 *  Licensed under the Apache License, Version 2.0 (the "License"); you may
 *  not use these files except in compliance with the License. You may obtain
 *  a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
 *  WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
 *  License for the specific language governing permissions and limitations
 *  under the License.
 */

#import <Foundation/Foundation.h>

@class OCMLocation;
@class OCClassMockObject;
@class OCPartialMockObject;


BOOL OCMIsClassType(const char *objCType);
BOOL OCMIsBlockType(const char *objCType);
BOOL OCMIsObjectType(const char *objCType);
const char *OCMTypeWithoutQualifiers(const char *objCType);
BOOL OCMEqualTypesAllowingOpaqueStructs(const char *type1, const char *type2);
CFNumberType OCMNumberTypeForObjCType(const char *objcType);
BOOL OCMIsNilValue(const char *objectCType, const void *value, size_t valueSize);

BOOL OCMIsAppleBaseClass(Class cls);
BOOL OCMIsApplePrivateMethod(Class cls, SEL sel);

Class OCMCreateSubclass(Class cls, void *ref);
BOOL OCMIsMockSubclass(Class cls);
void OCMDisposeSubclass(Class cls);

BOOL OCMIsAliasSelector(SEL selector);
SEL OCMAliasForOriginalSelector(SEL selector);
SEL OCMOriginalSelectorForAlias(SEL selector);

void OCMSetAssociatedMockForClass(OCClassMockObject *mock, Class aClass);
OCClassMockObject *OCMGetAssociatedMockForClass(Class aClass, BOOL includeSuperclasses);

void OCMSetAssociatedMockForObject(OCClassMockObject *mock, id anObject);
OCPartialMockObject *OCMGetAssociatedMockForObject(id anObject);

void OCMReportFailure(OCMLocation *loc, NSString *description);

BOOL OCMIsBlock(id potentialBlock);
BOOL OCMIsNonEscapingBlock(id block);


struct OCMBlockDef
{
    void *isa; // initialized to &_NSConcreteStackBlock or &_NSConcreteGlobalBlock
    int flags;
    int reserved;
    void (*invoke)(void *, ...);
    struct block_descriptor {
        unsigned long int reserved;                 // NULL
        unsigned long int size;                     // sizeof(struct Block_literal_1)
        // optional helper functions
        void (*copy_helper)(void *dst, void *src);  // IFF (1<<25)
        void (*dispose_helper)(void *src);          // IFF (1<<25)
        // required ABI.2010.3.16
        const char *signature;                      // IFF (1<<30)
    } *descriptor;
};

enum
{
    OCMBlockIsNoEscape                     = (1 << 23),
    OCMBlockDescriptionFlagsHasCopyDispose = (1 << 25),
    OCMBlockDescriptionFlagsHasSignature   = (1 << 30)
};
