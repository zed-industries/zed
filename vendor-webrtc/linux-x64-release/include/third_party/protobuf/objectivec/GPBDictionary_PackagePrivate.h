// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#import <Foundation/Foundation.h>

#import "GPBDictionary.h"

#import "GPBCodedInputStream.h"
#import "GPBCodedOutputStream.h"
#import "GPBDescriptor.h"
#import "GPBExtensionRegistry.h"

@protocol GPBDictionaryInternalsProtocol
- (size_t)computeSerializedSizeAsField:(GPBFieldDescriptor *)field;
- (void)writeToCodedOutputStream:(GPBCodedOutputStream *)outputStream
                         asField:(GPBFieldDescriptor *)field;
- (void)setGPBGenericValue:(GPBGenericValue *)value forGPBGenericValueKey:(GPBGenericValue *)key;
- (void)enumerateForTextFormat:(void (^)(id keyObj, id valueObj))block;
@end

// Disable clang-format for the macros.
// clang-format off

//%PDDM-DEFINE DICTIONARY_PRIV_INTERFACES_FOR_POD_KEY(KEY_NAME)
//%DICTIONARY_POD_PRIV_INTERFACES_FOR_KEY(KEY_NAME)
//%DICTIONARY_PRIVATE_INTERFACES(KEY_NAME, Object, Object)
//%PDDM-DEFINE DICTIONARY_POD_PRIV_INTERFACES_FOR_KEY(KEY_NAME)
//%DICTIONARY_PRIVATE_INTERFACES(KEY_NAME, UInt32, Basic)
//%DICTIONARY_PRIVATE_INTERFACES(KEY_NAME, Int32, Basic)
//%DICTIONARY_PRIVATE_INTERFACES(KEY_NAME, UInt64, Basic)
//%DICTIONARY_PRIVATE_INTERFACES(KEY_NAME, Int64, Basic)
//%DICTIONARY_PRIVATE_INTERFACES(KEY_NAME, Bool, Basic)
//%DICTIONARY_PRIVATE_INTERFACES(KEY_NAME, Float, Basic)
//%DICTIONARY_PRIVATE_INTERFACES(KEY_NAME, Double, Basic)
//%DICTIONARY_PRIVATE_INTERFACES(KEY_NAME, Enum, Enum)

//%PDDM-DEFINE DICTIONARY_PRIVATE_INTERFACES(KEY_NAME, VALUE_NAME, HELPER)
//%@interface GPB##KEY_NAME##VALUE_NAME##Dictionary () <GPBDictionaryInternalsProtocol> {
//% @package
//%  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
//%}
//%EXTRA_DICTIONARY_PRIVATE_INTERFACES_##HELPER()@end
//%

//%PDDM-DEFINE EXTRA_DICTIONARY_PRIVATE_INTERFACES_Basic()
// Empty
//%PDDM-DEFINE EXTRA_DICTIONARY_PRIVATE_INTERFACES_Object()
//%- (BOOL)isInitialized;
//%- (instancetype)deepCopyWithZone:(NSZone *)zone
//%    __attribute__((ns_returns_retained));
//%
//%PDDM-DEFINE EXTRA_DICTIONARY_PRIVATE_INTERFACES_Enum()
//%- (NSData *)serializedDataForUnknownValue:(int32_t)value
//%                                   forKey:(GPBGenericValue *)key
//%                              keyDataType:(GPBDataType)keyDataType;
//%

//%PDDM-EXPAND DICTIONARY_PRIV_INTERFACES_FOR_POD_KEY(UInt32)
// This block of code is generated, do not edit it directly.

@interface GPBUInt32UInt32Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBUInt32Int32Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBUInt32UInt64Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBUInt32Int64Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBUInt32BoolDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBUInt32FloatDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBUInt32DoubleDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBUInt32EnumDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
- (NSData *)serializedDataForUnknownValue:(int32_t)value
                                   forKey:(GPBGenericValue *)key
                              keyDataType:(GPBDataType)keyDataType;
@end

@interface GPBUInt32ObjectDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
- (BOOL)isInitialized;
- (instancetype)deepCopyWithZone:(NSZone *)zone
    __attribute__((ns_returns_retained));
@end

//%PDDM-EXPAND DICTIONARY_PRIV_INTERFACES_FOR_POD_KEY(Int32)
// This block of code is generated, do not edit it directly.

@interface GPBInt32UInt32Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBInt32Int32Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBInt32UInt64Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBInt32Int64Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBInt32BoolDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBInt32FloatDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBInt32DoubleDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBInt32EnumDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
- (NSData *)serializedDataForUnknownValue:(int32_t)value
                                   forKey:(GPBGenericValue *)key
                              keyDataType:(GPBDataType)keyDataType;
@end

@interface GPBInt32ObjectDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
- (BOOL)isInitialized;
- (instancetype)deepCopyWithZone:(NSZone *)zone
    __attribute__((ns_returns_retained));
@end

//%PDDM-EXPAND DICTIONARY_PRIV_INTERFACES_FOR_POD_KEY(UInt64)
// This block of code is generated, do not edit it directly.

@interface GPBUInt64UInt32Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBUInt64Int32Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBUInt64UInt64Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBUInt64Int64Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBUInt64BoolDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBUInt64FloatDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBUInt64DoubleDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBUInt64EnumDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
- (NSData *)serializedDataForUnknownValue:(int32_t)value
                                   forKey:(GPBGenericValue *)key
                              keyDataType:(GPBDataType)keyDataType;
@end

@interface GPBUInt64ObjectDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
- (BOOL)isInitialized;
- (instancetype)deepCopyWithZone:(NSZone *)zone
    __attribute__((ns_returns_retained));
@end

//%PDDM-EXPAND DICTIONARY_PRIV_INTERFACES_FOR_POD_KEY(Int64)
// This block of code is generated, do not edit it directly.

@interface GPBInt64UInt32Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBInt64Int32Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBInt64UInt64Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBInt64Int64Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBInt64BoolDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBInt64FloatDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBInt64DoubleDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBInt64EnumDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
- (NSData *)serializedDataForUnknownValue:(int32_t)value
                                   forKey:(GPBGenericValue *)key
                              keyDataType:(GPBDataType)keyDataType;
@end

@interface GPBInt64ObjectDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
- (BOOL)isInitialized;
- (instancetype)deepCopyWithZone:(NSZone *)zone
    __attribute__((ns_returns_retained));
@end

//%PDDM-EXPAND DICTIONARY_PRIV_INTERFACES_FOR_POD_KEY(Bool)
// This block of code is generated, do not edit it directly.

@interface GPBBoolUInt32Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBBoolInt32Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBBoolUInt64Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBBoolInt64Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBBoolBoolDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBBoolFloatDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBBoolDoubleDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBBoolEnumDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
- (NSData *)serializedDataForUnknownValue:(int32_t)value
                                   forKey:(GPBGenericValue *)key
                              keyDataType:(GPBDataType)keyDataType;
@end

@interface GPBBoolObjectDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
- (BOOL)isInitialized;
- (instancetype)deepCopyWithZone:(NSZone *)zone
    __attribute__((ns_returns_retained));
@end

//%PDDM-EXPAND DICTIONARY_POD_PRIV_INTERFACES_FOR_KEY(String)
// This block of code is generated, do not edit it directly.

@interface GPBStringUInt32Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBStringInt32Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBStringUInt64Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBStringInt64Dictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBStringBoolDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBStringFloatDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBStringDoubleDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

@interface GPBStringEnumDictionary () <GPBDictionaryInternalsProtocol> {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
- (NSData *)serializedDataForUnknownValue:(int32_t)value
                                   forKey:(GPBGenericValue *)key
                              keyDataType:(GPBDataType)keyDataType;
@end

//%PDDM-EXPAND-END (6 expansions)

// clang-format on

#pragma mark - NSDictionary Subclass

@interface GPBAutocreatedDictionary : NSMutableDictionary {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

#pragma mark - Helpers

CF_EXTERN_C_BEGIN

// Helper to compute size when an NSDictionary is used for the map instead
// of a custom type.
size_t GPBDictionaryComputeSizeInternalHelper(NSDictionary *dict, GPBFieldDescriptor *field);

// Helper to write out when an NSDictionary is used for the map instead
// of a custom type.
void GPBDictionaryWriteToStreamInternalHelper(GPBCodedOutputStream *outputStream,
                                              NSDictionary *dict, GPBFieldDescriptor *field);

// Helper to check message initialization when an NSDictionary is used for
// the map instead of a custom type.
BOOL GPBDictionaryIsInitializedInternalHelper(NSDictionary *dict, GPBFieldDescriptor *field);

// Helper to read a map instead.
void GPBDictionaryReadEntry(id mapDictionary, GPBCodedInputStream *stream,
                            id<GPBExtensionRegistry> registry, GPBFieldDescriptor *field,
                            GPBMessage *parentMessage);

CF_EXTERN_C_END
