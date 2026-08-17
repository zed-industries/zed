// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#import <Foundation/Foundation.h>

#import "GPBUtilities.h"

#import "GPBDescriptor.h"
#import "GPBDescriptor_PackagePrivate.h"

// Macros for stringifying library symbols. These are used in the generated
// GPB descriptor classes wherever a library symbol name is represented as a
// string.
#define GPBStringify(S) #S
#define GPBStringifySymbol(S) GPBStringify(S)

#define GPBNSStringify(S) @ #S
#define GPBNSStringifySymbol(S) GPBNSStringify(S)

// A type to represent an Objective C class.
// This is actually an `objc_class` but the runtime headers will not allow us to
// reference `objc_class`, so we have defined our own.
typedef struct GPBObjcClass_t GPBObjcClass_t;

// Macros for generating a Class from a class name. These are used in
// the generated GPB descriptor classes wherever an Objective C class
// reference is needed for a generated class.
#define GPBObjCClassSymbol(name) OBJC_CLASS_$_##name
#define GPBObjCClass(name) ((__bridge Class) & (GPBObjCClassSymbol(name)))
#define GPBObjCClassDeclaration(name) extern const GPBObjcClass_t GPBObjCClassSymbol(name)

// Constant to internally mark when there is no has bit.
#define GPBNoHasBit INT32_MAX

CF_EXTERN_C_BEGIN

// These two are used to inject a runtime check for version mismatch into the
// generated sources to make sure they are linked with a supporting runtime.
void GPBCheckRuntimeVersionSupport(int32_t objcRuntimeVersion);
GPB_INLINE void GPB_DEBUG_CHECK_RUNTIME_VERSIONS(void) {
  // NOTE: By being inline here, this captures the value from the library's
  // headers at the time the generated code was compiled.
#if defined(DEBUG) && DEBUG
  GPBCheckRuntimeVersionSupport(GOOGLE_PROTOBUF_OBJC_VERSION);
#endif
}

// Helper called within the library when the runtime detects something that
// indicates a older runtime is being used with newer generated code. Normally
// GPB_DEBUG_CHECK_RUNTIME_VERSIONS() gates this with a better message; this
// is just a final safety net to prevent otherwise hard to diagnose errors.
void GPBRuntimeMatchFailure(void);

// Conversion functions for de/serializing floating point types.

GPB_INLINE int64_t GPBConvertDoubleToInt64(double v) {
  GPBInternalCompileAssert(sizeof(double) == sizeof(int64_t), double_not_64_bits);
  int64_t result;
  memcpy(&result, &v, sizeof(result));
  return result;
}

GPB_INLINE int32_t GPBConvertFloatToInt32(float v) {
  GPBInternalCompileAssert(sizeof(float) == sizeof(int32_t), float_not_32_bits);
  int32_t result;
  memcpy(&result, &v, sizeof(result));
  return result;
}

GPB_INLINE double GPBConvertInt64ToDouble(int64_t v) {
  GPBInternalCompileAssert(sizeof(double) == sizeof(int64_t), double_not_64_bits);
  double result;
  memcpy(&result, &v, sizeof(result));
  return result;
}

GPB_INLINE float GPBConvertInt32ToFloat(int32_t v) {
  GPBInternalCompileAssert(sizeof(float) == sizeof(int32_t), float_not_32_bits);
  float result;
  memcpy(&result, &v, sizeof(result));
  return result;
}

GPB_INLINE int32_t GPBLogicalRightShift32(int32_t value, int32_t spaces) {
  return (int32_t)((uint32_t)(value) >> spaces);
}

GPB_INLINE int64_t GPBLogicalRightShift64(int64_t value, int32_t spaces) {
  return (int64_t)((uint64_t)(value) >> spaces);
}

// Decode a ZigZag-encoded 32-bit value.  ZigZag encodes signed integers
// into values that can be efficiently encoded with varint.  (Otherwise,
// negative values must be sign-extended to 64 bits to be varint encoded,
// thus always taking 10 bytes on the wire.)
GPB_INLINE int32_t GPBDecodeZigZag32(uint32_t n) {
  return (int32_t)(GPBLogicalRightShift32((int32_t)n, 1) ^ -((int32_t)(n) & 1));
}

// Decode a ZigZag-encoded 64-bit value.  ZigZag encodes signed integers
// into values that can be efficiently encoded with varint.  (Otherwise,
// negative values must be sign-extended to 64 bits to be varint encoded,
// thus always taking 10 bytes on the wire.)
GPB_INLINE int64_t GPBDecodeZigZag64(uint64_t n) {
  return (int64_t)(GPBLogicalRightShift64((int64_t)n, 1) ^ -((int64_t)(n) & 1));
}

// Encode a ZigZag-encoded 32-bit value.  ZigZag encodes signed integers
// into values that can be efficiently encoded with varint.  (Otherwise,
// negative values must be sign-extended to 64 bits to be varint encoded,
// thus always taking 10 bytes on the wire.)
GPB_INLINE uint32_t GPBEncodeZigZag32(int32_t n) {
  // Note:  the right-shift must be arithmetic
  return ((uint32_t)n << 1) ^ (uint32_t)(n >> 31);
}

// Encode a ZigZag-encoded 64-bit value.  ZigZag encodes signed integers
// into values that can be efficiently encoded with varint.  (Otherwise,
// negative values must be sign-extended to 64 bits to be varint encoded,
// thus always taking 10 bytes on the wire.)
GPB_INLINE uint64_t GPBEncodeZigZag64(int64_t n) {
  // Note:  the right-shift must be arithmetic
  return ((uint64_t)n << 1) ^ (uint64_t)(n >> 63);
}

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wswitch-enum"
#pragma clang diagnostic ignored "-Wdirect-ivar-access"

GPB_INLINE BOOL GPBDataTypeIsObject(GPBDataType type) {
  switch (type) {
    case GPBDataTypeBytes:
    case GPBDataTypeString:
    case GPBDataTypeMessage:
    case GPBDataTypeGroup:
      return YES;
    default:
      return NO;
  }
}

GPB_INLINE BOOL GPBDataTypeIsMessage(GPBDataType type) {
  switch (type) {
    case GPBDataTypeMessage:
    case GPBDataTypeGroup:
      return YES;
    default:
      return NO;
  }
}

GPB_INLINE BOOL GPBFieldDataTypeIsMessage(GPBFieldDescriptor *field) {
  return GPBDataTypeIsMessage(field->description_->dataType);
}

GPB_INLINE BOOL GPBFieldDataTypeIsObject(GPBFieldDescriptor *field) {
  return GPBDataTypeIsObject(field->description_->dataType);
}

GPB_INLINE BOOL GPBExtensionIsMessage(GPBExtensionDescriptor *ext) {
  return GPBDataTypeIsMessage(ext->description_->dataType);
}

// The field is an array/map or it has an object value.
GPB_INLINE BOOL GPBFieldStoresObject(GPBFieldDescriptor *field) {
  GPBMessageFieldDescription *desc = field->description_;
  if ((desc->flags & (GPBFieldRepeated | GPBFieldMapKeyMask)) != 0) {
    return YES;
  }
  return GPBDataTypeIsObject(desc->dataType);
}

BOOL GPBGetHasIvar(GPBMessage *self, int32_t idx, uint32_t fieldNumber);
void GPBSetHasIvar(GPBMessage *self, int32_t idx, uint32_t fieldNumber, BOOL value);
uint32_t GPBGetHasOneof(GPBMessage *self, int32_t idx);

GPB_INLINE BOOL GPBGetHasIvarField(GPBMessage *self, GPBFieldDescriptor *field) {
  GPBMessageFieldDescription *fieldDesc = field->description_;
  return GPBGetHasIvar(self, fieldDesc->hasIndex, fieldDesc->number);
}

#pragma clang diagnostic pop

// Disable clang-format for the macros.
// clang-format off

//%PDDM-DEFINE GPB_IVAR_SET_DECL(NAME, TYPE)
//%void GPBSet##NAME##IvarWithFieldPrivate(GPBMessage *self,
//%            NAME$S                    GPBFieldDescriptor *field,
//%            NAME$S                    TYPE value);
//%PDDM-EXPAND GPB_IVAR_SET_DECL(Bool, BOOL)
// This block of code is generated, do not edit it directly.

void GPBSetBoolIvarWithFieldPrivate(GPBMessage *self,
                                    GPBFieldDescriptor *field,
                                    BOOL value);
//%PDDM-EXPAND GPB_IVAR_SET_DECL(Int32, int32_t)
// This block of code is generated, do not edit it directly.

void GPBSetInt32IvarWithFieldPrivate(GPBMessage *self,
                                     GPBFieldDescriptor *field,
                                     int32_t value);
//%PDDM-EXPAND GPB_IVAR_SET_DECL(UInt32, uint32_t)
// This block of code is generated, do not edit it directly.

void GPBSetUInt32IvarWithFieldPrivate(GPBMessage *self,
                                      GPBFieldDescriptor *field,
                                      uint32_t value);
//%PDDM-EXPAND GPB_IVAR_SET_DECL(Int64, int64_t)
// This block of code is generated, do not edit it directly.

void GPBSetInt64IvarWithFieldPrivate(GPBMessage *self,
                                     GPBFieldDescriptor *field,
                                     int64_t value);
//%PDDM-EXPAND GPB_IVAR_SET_DECL(UInt64, uint64_t)
// This block of code is generated, do not edit it directly.

void GPBSetUInt64IvarWithFieldPrivate(GPBMessage *self,
                                      GPBFieldDescriptor *field,
                                      uint64_t value);
//%PDDM-EXPAND GPB_IVAR_SET_DECL(Float, float)
// This block of code is generated, do not edit it directly.

void GPBSetFloatIvarWithFieldPrivate(GPBMessage *self,
                                     GPBFieldDescriptor *field,
                                     float value);
//%PDDM-EXPAND GPB_IVAR_SET_DECL(Double, double)
// This block of code is generated, do not edit it directly.

void GPBSetDoubleIvarWithFieldPrivate(GPBMessage *self,
                                      GPBFieldDescriptor *field,
                                      double value);
//%PDDM-EXPAND-END (7 expansions)

// clang-format on

void GPBSetEnumIvarWithFieldPrivate(GPBMessage *self, GPBFieldDescriptor *field, int32_t value);

id GPBGetObjectIvarWithField(GPBMessage *self, GPBFieldDescriptor *field);

void GPBSetObjectIvarWithFieldPrivate(GPBMessage *self, GPBFieldDescriptor *field, id value);
void GPBSetRetainedObjectIvarWithFieldPrivate(GPBMessage *self, GPBFieldDescriptor *field,
                                              id __attribute__((ns_consumed)) value);

// GPBGetObjectIvarWithField will automatically create the field (message) if
// it doesn't exist. GPBGetObjectIvarWithFieldNoAutocreate will return nil.
id GPBGetObjectIvarWithFieldNoAutocreate(GPBMessage *self, GPBFieldDescriptor *field);

// Clears and releases the autocreated message ivar, if it's autocreated. If
// it's not set as autocreated, this method does nothing.
void GPBClearAutocreatedMessageIvarWithField(GPBMessage *self, GPBFieldDescriptor *field);

// Returns an Objective C encoding for |selector|. |instanceSel| should be
// YES if it's an instance selector (as opposed to a class selector).
// |selector| must be a selector from MessageSignatureProtocol.
const char *GPBMessageEncodingForSelector(SEL selector, BOOL instanceSel);

// Helper for text format name encoding.
// decodeData is the data describing the special decodes.
// key and inputString are the input that needs decoding.
NSString *GPBDecodeTextFormatName(const uint8_t *decodeData, int32_t key, NSString *inputString);

// Shims from the older generated code into the runtime.
void GPBSetInt32IvarWithFieldInternal(GPBMessage *self, GPBFieldDescriptor *field, int32_t value,
                                      GPBFileSyntax syntax);
void GPBMaybeClearOneof(GPBMessage *self, GPBOneofDescriptor *oneof, int32_t oneofHasIndex,
                        uint32_t fieldNumberNotToClear);

// A series of selectors that are used solely to get @encoding values
// for them by the dynamic protobuf runtime code. See
// GPBMessageEncodingForSelector for details. GPBRootObject conforms to
// the protocol so that it is encoded in the Objective C runtime.
@protocol GPBMessageSignatureProtocol
@optional

#define GPB_MESSAGE_SIGNATURE_ENTRY(TYPE, NAME) \
  -(TYPE)get##NAME;                             \
  -(void)set##NAME : (TYPE)value;               \
  -(TYPE)get##NAME##AtIndex : (NSUInteger)index;

GPB_MESSAGE_SIGNATURE_ENTRY(BOOL, Bool)
GPB_MESSAGE_SIGNATURE_ENTRY(uint32_t, Fixed32)
GPB_MESSAGE_SIGNATURE_ENTRY(int32_t, SFixed32)
GPB_MESSAGE_SIGNATURE_ENTRY(float, Float)
GPB_MESSAGE_SIGNATURE_ENTRY(uint64_t, Fixed64)
GPB_MESSAGE_SIGNATURE_ENTRY(int64_t, SFixed64)
GPB_MESSAGE_SIGNATURE_ENTRY(double, Double)
GPB_MESSAGE_SIGNATURE_ENTRY(int32_t, Int32)
GPB_MESSAGE_SIGNATURE_ENTRY(int64_t, Int64)
GPB_MESSAGE_SIGNATURE_ENTRY(int32_t, SInt32)
GPB_MESSAGE_SIGNATURE_ENTRY(int64_t, SInt64)
GPB_MESSAGE_SIGNATURE_ENTRY(uint32_t, UInt32)
GPB_MESSAGE_SIGNATURE_ENTRY(uint64_t, UInt64)
GPB_MESSAGE_SIGNATURE_ENTRY(NSData *, Bytes)
GPB_MESSAGE_SIGNATURE_ENTRY(NSString *, String)
GPB_MESSAGE_SIGNATURE_ENTRY(GPBMessage *, Message)
GPB_MESSAGE_SIGNATURE_ENTRY(GPBMessage *, Group)
GPB_MESSAGE_SIGNATURE_ENTRY(int32_t, Enum)

#undef GPB_MESSAGE_SIGNATURE_ENTRY

- (id)getArray;
- (NSUInteger)getArrayCount;
- (void)setArray:(NSArray *)array;
+ (id)getClassValue;
@end

BOOL GPBClassHasSel(Class aClass, SEL sel);

CF_EXTERN_C_END
