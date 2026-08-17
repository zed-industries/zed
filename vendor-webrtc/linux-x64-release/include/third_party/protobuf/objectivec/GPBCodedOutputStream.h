// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#import <Foundation/Foundation.h>

#import "GPBRuntimeTypes.h"
#import "GPBWireFormat.h"

#import "GPBArray.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * @c GPBCodedOutputStream exception names.
 **/
extern NSString *const GPBCodedOutputStreamException_OutOfSpace;
extern NSString *const GPBCodedOutputStreamException_WriteFailed;

/**
 * Writes out protocol message fields.
 *
 * The common uses of protocol buffers shouldn't need to use this class.
 * GPBMessage's provide a -data method that will serialize the message for you.
 *
 * @note Any -write* api can raise the GPBCodedOutputStreamException_*
 *       exceptions.
 *
 * @note Subclassing of GPBCodedOutputStream is NOT supported.
 **/
__attribute__((objc_subclassing_restricted))
@interface GPBCodedOutputStream : NSObject

/**
 * Creates a stream to fill in the given data. Data must be sized to fit or
 * an error will be raised when out of space.
 *
 * @param data The data where the stream will be written to.
 *
 * @return A newly instanced GPBCodedOutputStream.
 **/
+ (instancetype)streamWithData:(NSMutableData *)data;

/**
 * Creates a stream to write into the given NSOutputStream.
 *
 * @param output The output stream where the stream will be written to.
 *
 * @return A newly instanced GPBCodedOutputStream.
 **/
+ (instancetype)streamWithOutputStream:(NSOutputStream *)output;

/**
 * Initializes a stream to fill in the given data. Data must be sized to fit
 * or an error will be raised when out of space.
 *
 * @param data The data where the stream will be written to.
 *
 * @return A newly initialized GPBCodedOutputStream.
 **/
- (instancetype)initWithData:(NSMutableData *)data;

/**
 * Initializes a stream to write into the given @c NSOutputStream.
 *
 * @param output The output stream where the stream will be written to.
 *
 * @return A newly initialized GPBCodedOutputStream.
 **/
- (instancetype)initWithOutputStream:(NSOutputStream *)output;

/**
 * Flush any buffered data out.
 **/
- (void)flush;

/**
 * @return The number of bytes written out. Includes bytes not yet flused.
 **/
- (size_t)bytesWritten;

/**
 * Write the raw byte out.
 *
 * @param value The value to write out.
 **/
- (void)writeRawByte:(uint8_t)value;

/**
 * Write the tag for the given field number and wire format.
 *
 * @param fieldNumber The field number.
 * @param format      The wire format the data for the field will be in.
 **/
- (void)writeTag:(uint32_t)fieldNumber format:(GPBWireFormat)format;

/**
 * Write a 32bit value out in little endian format.
 *
 * @param value The value to write out.
 **/
- (void)writeRawLittleEndian32:(int32_t)value;
/**
 * Write a 64bit value out in little endian format.
 *
 * @param value The value to write out.
 **/
- (void)writeRawLittleEndian64:(int64_t)value;

/**
 * Write a 32bit value out in varint format.
 *
 * @param value The value to write out.
 **/
- (void)writeRawVarint32:(int32_t)value;
/**
 * Write a 64bit value out in varint format.
 *
 * @param value The value to write out.
 **/
- (void)writeRawVarint64:(int64_t)value;

/**
 * Write a size_t out as a 32bit varint value.
 *
 * @note This will truncate 64 bit values to 32.
 *
 * @param value The value to write out.
 **/
- (void)writeRawVarintSizeTAs32:(size_t)value;

/**
 * Writes the contents of an NSData out.
 *
 * @param data The data to write out.
 **/
- (void)writeRawData:(NSData *)data;
/**
 * Writes out the given data.
 *
 * @param data   The data blob to write out.
 * @param offset The offset into the blob to start writing out.
 * @param length The number of bytes from the blob to write out.
 **/
- (void)writeRawPtr:(const void *)data offset:(size_t)offset length:(size_t)length;

// Disable clang-format for the macros.
// clang-format off

//%PDDM-EXPAND _WRITE_DECLS()
// This block of code is generated, do not edit it directly.

/**
 * Write a double for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeDouble:(int32_t)fieldNumber value:(double)value;
/**
 * Write a packed array of double for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 * @param tag         The tag assigned to the values.
 **/
- (void)writeDoubleArray:(int32_t)fieldNumber
                  values:(GPBDoubleArray *)values
                     tag:(uint32_t)tag;
/**
 * Write a double without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeDoubleNoTag:(double)value;

/**
 * Write a float for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeFloat:(int32_t)fieldNumber value:(float)value;
/**
 * Write a packed array of float for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 * @param tag         The tag assigned to the values.
 **/
- (void)writeFloatArray:(int32_t)fieldNumber
                 values:(GPBFloatArray *)values
                    tag:(uint32_t)tag;
/**
 * Write a float without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeFloatNoTag:(float)value;

/**
 * Write a uint64_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeUInt64:(int32_t)fieldNumber value:(uint64_t)value;
/**
 * Write a packed array of uint64_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 * @param tag         The tag assigned to the values.
 **/
- (void)writeUInt64Array:(int32_t)fieldNumber
                  values:(GPBUInt64Array *)values
                     tag:(uint32_t)tag;
/**
 * Write a uint64_t without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeUInt64NoTag:(uint64_t)value;

/**
 * Write a int64_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeInt64:(int32_t)fieldNumber value:(int64_t)value;
/**
 * Write a packed array of int64_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 * @param tag         The tag assigned to the values.
 **/
- (void)writeInt64Array:(int32_t)fieldNumber
                 values:(GPBInt64Array *)values
                    tag:(uint32_t)tag;
/**
 * Write a int64_t without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeInt64NoTag:(int64_t)value;

/**
 * Write a int32_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeInt32:(int32_t)fieldNumber value:(int32_t)value;
/**
 * Write a packed array of int32_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 * @param tag         The tag assigned to the values.
 **/
- (void)writeInt32Array:(int32_t)fieldNumber
                 values:(GPBInt32Array *)values
                    tag:(uint32_t)tag;
/**
 * Write a int32_t without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeInt32NoTag:(int32_t)value;

/**
 * Write a uint32_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeUInt32:(int32_t)fieldNumber value:(uint32_t)value;
/**
 * Write a packed array of uint32_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 * @param tag         The tag assigned to the values.
 **/
- (void)writeUInt32Array:(int32_t)fieldNumber
                  values:(GPBUInt32Array *)values
                     tag:(uint32_t)tag;
/**
 * Write a uint32_t without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeUInt32NoTag:(uint32_t)value;

/**
 * Write a uint64_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeFixed64:(int32_t)fieldNumber value:(uint64_t)value;
/**
 * Write a packed array of uint64_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 * @param tag         The tag assigned to the values.
 **/
- (void)writeFixed64Array:(int32_t)fieldNumber
                   values:(GPBUInt64Array *)values
                      tag:(uint32_t)tag;
/**
 * Write a uint64_t without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeFixed64NoTag:(uint64_t)value;

/**
 * Write a uint32_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeFixed32:(int32_t)fieldNumber value:(uint32_t)value;
/**
 * Write a packed array of uint32_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 * @param tag         The tag assigned to the values.
 **/
- (void)writeFixed32Array:(int32_t)fieldNumber
                   values:(GPBUInt32Array *)values
                      tag:(uint32_t)tag;
/**
 * Write a uint32_t without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeFixed32NoTag:(uint32_t)value;

/**
 * Write a int32_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeSInt32:(int32_t)fieldNumber value:(int32_t)value;
/**
 * Write a packed array of int32_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 * @param tag         The tag assigned to the values.
 **/
- (void)writeSInt32Array:(int32_t)fieldNumber
                  values:(GPBInt32Array *)values
                     tag:(uint32_t)tag;
/**
 * Write a int32_t without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeSInt32NoTag:(int32_t)value;

/**
 * Write a int64_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeSInt64:(int32_t)fieldNumber value:(int64_t)value;
/**
 * Write a packed array of int64_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 * @param tag         The tag assigned to the values.
 **/
- (void)writeSInt64Array:(int32_t)fieldNumber
                  values:(GPBInt64Array *)values
                     tag:(uint32_t)tag;
/**
 * Write a int64_t without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeSInt64NoTag:(int64_t)value;

/**
 * Write a int64_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeSFixed64:(int32_t)fieldNumber value:(int64_t)value;
/**
 * Write a packed array of int64_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 * @param tag         The tag assigned to the values.
 **/
- (void)writeSFixed64Array:(int32_t)fieldNumber
                    values:(GPBInt64Array *)values
                       tag:(uint32_t)tag;
/**
 * Write a int64_t without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeSFixed64NoTag:(int64_t)value;

/**
 * Write a int32_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeSFixed32:(int32_t)fieldNumber value:(int32_t)value;
/**
 * Write a packed array of int32_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 * @param tag         The tag assigned to the values.
 **/
- (void)writeSFixed32Array:(int32_t)fieldNumber
                    values:(GPBInt32Array *)values
                       tag:(uint32_t)tag;
/**
 * Write a int32_t without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeSFixed32NoTag:(int32_t)value;

/**
 * Write a BOOL for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeBool:(int32_t)fieldNumber value:(BOOL)value;
/**
 * Write a packed array of BOOL for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 * @param tag         The tag assigned to the values.
 **/
- (void)writeBoolArray:(int32_t)fieldNumber
                values:(GPBBoolArray *)values
                   tag:(uint32_t)tag;
/**
 * Write a BOOL without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeBoolNoTag:(BOOL)value;

/**
 * Write a int32_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeEnum:(int32_t)fieldNumber value:(int32_t)value;
/**
 * Write a packed array of int32_t for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 * @param tag         The tag assigned to the values.
 **/
- (void)writeEnumArray:(int32_t)fieldNumber
                values:(GPBEnumArray *)values
                   tag:(uint32_t)tag;
/**
 * Write a int32_t without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeEnumNoTag:(int32_t)value;

/**
 * Write a NSString for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeString:(int32_t)fieldNumber value:(NSString *)value;
/**
 * Write an array of NSString for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 **/
- (void)writeStringArray:(int32_t)fieldNumber
                  values:(NSArray<NSString*> *)values;
/**
 * Write a NSString without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeStringNoTag:(NSString *)value;

/**
 * Write a GPBMessage for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeMessage:(int32_t)fieldNumber value:(GPBMessage *)value;
/**
 * Write an array of GPBMessage for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 **/
- (void)writeMessageArray:(int32_t)fieldNumber
                   values:(NSArray<GPBMessage*> *)values;
/**
 * Write a GPBMessage without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeMessageNoTag:(GPBMessage *)value;

/**
 * Write a NSData for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeBytes:(int32_t)fieldNumber value:(NSData *)value;
/**
 * Write an array of NSData for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 **/
- (void)writeBytesArray:(int32_t)fieldNumber
                 values:(NSArray<NSData*> *)values;
/**
 * Write a NSData without any tag.
 *
 * @param value The value to write out.
 **/
- (void)writeBytesNoTag:(NSData *)value;

/**
 * Write a GPBMessage for the given field number.
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeGroup:(int32_t)fieldNumber
             value:(GPBMessage *)value;
/**
 * Write an array of GPBMessage for the given field number.
 *
 * @param fieldNumber The field number assigned to the values.
 * @param values      The values to write out.
 **/
- (void)writeGroupArray:(int32_t)fieldNumber
                 values:(NSArray<GPBMessage*> *)values;
/**
 * Write a GPBMessage without any tag (but does write the endGroup tag).
 *
 * @param fieldNumber The field number assigned to the value.
 * @param value       The value to write out.
 **/
- (void)writeGroupNoTag:(int32_t)fieldNumber
                  value:(GPBMessage *)value;

//%PDDM-EXPAND-END _WRITE_DECLS()

// clang-format on

/**
Write a MessageSet extension field to the stream. For historical reasons,
the wire format differs from normal fields.

@param fieldNumber The extension field number to write out.
@param value       The message from where to get the extension.
*/
- (void)writeMessageSetExtension:(int32_t)fieldNumber value:(GPBMessage *)value;

/**
Write an unparsed MessageSet extension field to the stream. For historical
reasons, the wire format differs from normal fields.

@param fieldNumber The extension field number to write out.
@param value       The raw message from where to get the extension.
*/
- (void)writeRawMessageSetExtension:(int32_t)fieldNumber value:(NSData *)value;

@end

NS_ASSUME_NONNULL_END

// Disable clang-format for the macros.
// clang-format off

// Write methods for types that can be in packed arrays.
//%PDDM-DEFINE _WRITE_PACKABLE_DECLS(NAME, ARRAY_TYPE, TYPE)
//%/**
//% * Write a TYPE for the given field number.
//% *
//% * @param fieldNumber The field number assigned to the value.
//% * @param value       The value to write out.
//% **/
//%- (void)write##NAME:(int32_t)fieldNumber value:(TYPE)value;
//%/**
//% * Write a packed array of TYPE for the given field number.
//% *
//% * @param fieldNumber The field number assigned to the values.
//% * @param values      The values to write out.
//% * @param tag         The tag assigned to the values.
//% **/
//%- (void)write##NAME##Array:(int32_t)fieldNumber
//%       NAME$S     values:(GPB##ARRAY_TYPE##Array *)values
//%       NAME$S        tag:(uint32_t)tag;
//%/**
//% * Write a TYPE without any tag.
//% *
//% * @param value The value to write out.
//% **/
//%- (void)write##NAME##NoTag:(TYPE)value;
//%
// Write methods for types that aren't in packed arrays.
//%PDDM-DEFINE _WRITE_UNPACKABLE_DECLS(NAME, TYPE)
//%/**
//% * Write a TYPE for the given field number.
//% *
//% * @param fieldNumber The field number assigned to the value.
//% * @param value       The value to write out.
//% **/
//%- (void)write##NAME:(int32_t)fieldNumber value:(TYPE *)value;
//%/**
//% * Write an array of TYPE for the given field number.
//% *
//% * @param fieldNumber The field number assigned to the values.
//% * @param values      The values to write out.
//% **/
//%- (void)write##NAME##Array:(int32_t)fieldNumber
//%           NAME$S values:(NSArray<##TYPE##*> *)values;
//%/**
//% * Write a TYPE without any tag.
//% *
//% * @param value The value to write out.
//% **/
//%- (void)write##NAME##NoTag:(TYPE *)value;
//%
// Special write methods for Groups.
//%PDDM-DEFINE _WRITE_GROUP_DECLS(NAME, TYPE)
//%/**
//% * Write a TYPE for the given field number.
//% *
//% * @param fieldNumber The field number assigned to the value.
//% * @param value       The value to write out.
//% **/
//%- (void)write##NAME:(int32_t)fieldNumber
//%       NAME$S value:(TYPE *)value;
//%/**
//% * Write an array of TYPE for the given field number.
//% *
//% * @param fieldNumber The field number assigned to the values.
//% * @param values      The values to write out.
//% **/
//%- (void)write##NAME##Array:(int32_t)fieldNumber
//%           NAME$S values:(NSArray<##TYPE##*> *)values;
//%/**
//% * Write a TYPE without any tag (but does write the endGroup tag).
//% *
//% * @param fieldNumber The field number assigned to the value.
//% * @param value       The value to write out.
//% **/
//%- (void)write##NAME##NoTag:(int32_t)fieldNumber
//%            NAME$S value:(TYPE *)value;
//%

// One macro to hide it all up above.
//%PDDM-DEFINE _WRITE_DECLS()
//%_WRITE_PACKABLE_DECLS(Double, Double, double)
//%_WRITE_PACKABLE_DECLS(Float, Float, float)
//%_WRITE_PACKABLE_DECLS(UInt64, UInt64, uint64_t)
//%_WRITE_PACKABLE_DECLS(Int64, Int64, int64_t)
//%_WRITE_PACKABLE_DECLS(Int32, Int32, int32_t)
//%_WRITE_PACKABLE_DECLS(UInt32, UInt32, uint32_t)
//%_WRITE_PACKABLE_DECLS(Fixed64, UInt64, uint64_t)
//%_WRITE_PACKABLE_DECLS(Fixed32, UInt32, uint32_t)
//%_WRITE_PACKABLE_DECLS(SInt32, Int32, int32_t)
//%_WRITE_PACKABLE_DECLS(SInt64, Int64, int64_t)
//%_WRITE_PACKABLE_DECLS(SFixed64, Int64, int64_t)
//%_WRITE_PACKABLE_DECLS(SFixed32, Int32, int32_t)
//%_WRITE_PACKABLE_DECLS(Bool, Bool, BOOL)
//%_WRITE_PACKABLE_DECLS(Enum, Enum, int32_t)
//%_WRITE_UNPACKABLE_DECLS(String, NSString)
//%_WRITE_UNPACKABLE_DECLS(Message, GPBMessage)
//%_WRITE_UNPACKABLE_DECLS(Bytes, NSData)
//%_WRITE_GROUP_DECLS(Group, GPBMessage)

// clang-format on
