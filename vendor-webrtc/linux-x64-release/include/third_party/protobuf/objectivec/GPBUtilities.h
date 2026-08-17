// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#import <Foundation/Foundation.h>

#import "GPBArray.h"
#import "GPBDescriptor.h"
#import "GPBMessage.h"
#import "GPBRuntimeTypes.h"

CF_EXTERN_C_BEGIN

NS_ASSUME_NONNULL_BEGIN

/**
 * Generates a string that should be a valid "TextFormat" for the C++ version
 * of Protocol Buffers.
 *
 * @param message    The message to generate from.
 * @param lineIndent A string to use as the prefix for all lines generated. Can
 *                   be nil if no extra indent is needed.
 *
 * @return An NSString with the TextFormat of the message.
 **/
NSString *GPBTextFormatForMessage(GPBMessage *message, NSString *__nullable lineIndent);

/**
 * Checks if the given field number is set on a message.
 *
 * @param self        The message to check.
 * @param fieldNumber The field number to check.
 *
 * @return YES if the field number is set on the given message.
 **/
BOOL GPBMessageHasFieldNumberSet(GPBMessage *self, uint32_t fieldNumber);

/**
 * Checks if the given field is set on a message.
 *
 * @param self  The message to check.
 * @param field The field to check.
 *
 * @return YES if the field is set on the given message.
 **/
BOOL GPBMessageHasFieldSet(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Clears the given field for the given message.
 *
 * @param self  The message for which to clear the field.
 * @param field The field to clear.
 **/
void GPBClearMessageField(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Clears the given oneof field for the given message.
 *
 * @param self  The message for which to clear the field.
 * @param oneof The oneof to clear.
 **/
void GPBClearOneof(GPBMessage *self, GPBOneofDescriptor *oneof);

// Disable clang-format for the macros.
// clang-format off

//%PDDM-EXPAND GPB_ACCESSORS()
// This block of code is generated, do not edit it directly.


//
// Get/Set a given field from/to a message.
//

// Single Fields

/**
 * Gets the value of a bytes field.
 *
 * @param self  The message from which to get the field.
 * @param field The field to get.
 **/
NSData *GPBGetMessageBytesField(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Sets the value of a bytes field.
 *
 * @param self  The message into which to set the field.
 * @param field The field to set.
 * @param value The to set in the field.
 **/
void GPBSetMessageBytesField(GPBMessage *self, GPBFieldDescriptor *field, NSData *value);

/**
 * Gets the value of a string field.
 *
 * @param self  The message from which to get the field.
 * @param field The field to get.
 **/
NSString *GPBGetMessageStringField(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Sets the value of a string field.
 *
 * @param self  The message into which to set the field.
 * @param field The field to set.
 * @param value The to set in the field.
 **/
void GPBSetMessageStringField(GPBMessage *self, GPBFieldDescriptor *field, NSString *value);

/**
 * Gets the value of a message field.
 *
 * @param self  The message from which to get the field.
 * @param field The field to get.
 **/
GPBMessage *GPBGetMessageMessageField(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Sets the value of a message field.
 *
 * @param self  The message into which to set the field.
 * @param field The field to set.
 * @param value The to set in the field.
 **/
void GPBSetMessageMessageField(GPBMessage *self, GPBFieldDescriptor *field, GPBMessage *value);

/**
 * Gets the value of a group field.
 *
 * @param self  The message from which to get the field.
 * @param field The field to get.
 **/
GPBMessage *GPBGetMessageGroupField(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Sets the value of a group field.
 *
 * @param self  The message into which to set the field.
 * @param field The field to set.
 * @param value The to set in the field.
 **/
void GPBSetMessageGroupField(GPBMessage *self, GPBFieldDescriptor *field, GPBMessage *value);

/**
 * Gets the value of a bool field.
 *
 * @param self  The message from which to get the field.
 * @param field The field to get.
 **/
BOOL GPBGetMessageBoolField(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Sets the value of a bool field.
 *
 * @param self  The message into which to set the field.
 * @param field The field to set.
 * @param value The to set in the field.
 **/
void GPBSetMessageBoolField(GPBMessage *self, GPBFieldDescriptor *field, BOOL value);

/**
 * Gets the value of an int32 field.
 *
 * @param self  The message from which to get the field.
 * @param field The field to get.
 **/
int32_t GPBGetMessageInt32Field(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Sets the value of an int32 field.
 *
 * @param self  The message into which to set the field.
 * @param field The field to set.
 * @param value The to set in the field.
 **/
void GPBSetMessageInt32Field(GPBMessage *self, GPBFieldDescriptor *field, int32_t value);

/**
 * Gets the value of an uint32 field.
 *
 * @param self  The message from which to get the field.
 * @param field The field to get.
 **/
uint32_t GPBGetMessageUInt32Field(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Sets the value of an uint32 field.
 *
 * @param self  The message into which to set the field.
 * @param field The field to set.
 * @param value The to set in the field.
 **/
void GPBSetMessageUInt32Field(GPBMessage *self, GPBFieldDescriptor *field, uint32_t value);

/**
 * Gets the value of an int64 field.
 *
 * @param self  The message from which to get the field.
 * @param field The field to get.
 **/
int64_t GPBGetMessageInt64Field(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Sets the value of an int64 field.
 *
 * @param self  The message into which to set the field.
 * @param field The field to set.
 * @param value The to set in the field.
 **/
void GPBSetMessageInt64Field(GPBMessage *self, GPBFieldDescriptor *field, int64_t value);

/**
 * Gets the value of an uint64 field.
 *
 * @param self  The message from which to get the field.
 * @param field The field to get.
 **/
uint64_t GPBGetMessageUInt64Field(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Sets the value of an uint64 field.
 *
 * @param self  The message into which to set the field.
 * @param field The field to set.
 * @param value The to set in the field.
 **/
void GPBSetMessageUInt64Field(GPBMessage *self, GPBFieldDescriptor *field, uint64_t value);

/**
 * Gets the value of a float field.
 *
 * @param self  The message from which to get the field.
 * @param field The field to get.
 **/
float GPBGetMessageFloatField(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Sets the value of a float field.
 *
 * @param self  The message into which to set the field.
 * @param field The field to set.
 * @param value The to set in the field.
 **/
void GPBSetMessageFloatField(GPBMessage *self, GPBFieldDescriptor *field, float value);

/**
 * Gets the value of a double field.
 *
 * @param self  The message from which to get the field.
 * @param field The field to get.
 **/
double GPBGetMessageDoubleField(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Sets the value of a double field.
 *
 * @param self  The message into which to set the field.
 * @param field The field to set.
 * @param value The to set in the field.
 **/
void GPBSetMessageDoubleField(GPBMessage *self, GPBFieldDescriptor *field, double value);

/**
 * Gets the given enum field of a message. For proto3, if the value isn't a
 * member of the enum, @c kGPBUnrecognizedEnumeratorValue will be returned.
 * GPBGetMessageRawEnumField will bypass the check and return whatever value
 * was set.
 *
 * @param self  The message from which to get the field.
 * @param field The field to get.
 *
 * @return The enum value for the given field.
 **/
int32_t GPBGetMessageEnumField(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Set the given enum field of a message. You can only set values that are
 * members of the enum.
 *
 * @param self  The message into which to set the field.
 * @param field The field to set.
 * @param value The enum value to set in the field.
 **/
void GPBSetMessageEnumField(GPBMessage *self,
                            GPBFieldDescriptor *field,
                            int32_t value);

/**
 * Get the given enum field of a message. No check is done to ensure the value
 * was defined in the enum.
 *
 * @param self  The message from which to get the field.
 * @param field The field to get.
 *
 * @return The raw enum value for the given field.
 **/
int32_t GPBGetMessageRawEnumField(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Set the given enum field of a message. You can set the value to anything,
 * even a value that is not a member of the enum.
 *
 * @param self  The message into which to set the field.
 * @param field The field to set.
 * @param value The raw enum value to set in the field.
 **/
void GPBSetMessageRawEnumField(GPBMessage *self,
                               GPBFieldDescriptor *field,
                               int32_t value);

// Repeated Fields

/**
 * Gets the value of a repeated field.
 *
 * @param self  The message from which to get the field.
 * @param field The repeated field to get.
 *
 * @return A GPB*Array or an NSMutableArray based on the field's type.
 **/
id GPBGetMessageRepeatedField(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Sets the value of a repeated field.
 *
 * @param self  The message into which to set the field.
 * @param field The field to set.
 * @param array A GPB*Array or NSMutableArray based on the field's type.
 **/
void GPBSetMessageRepeatedField(GPBMessage *self,
                                GPBFieldDescriptor *field,
                                id array);

// Map Fields

/**
 * Gets the value of a map<> field.
 *
 * @param self  The message from which to get the field.
 * @param field The repeated field to get.
 *
 * @return A GPB*Dictionary or NSMutableDictionary based on the field's type.
 **/
id GPBGetMessageMapField(GPBMessage *self, GPBFieldDescriptor *field);

/**
 * Sets the value of a map<> field.
 *
 * @param self       The message into which to set the field.
 * @param field      The field to set.
 * @param dictionary A GPB*Dictionary or NSMutableDictionary based on the
 *                   field's type.
 **/
void GPBSetMessageMapField(GPBMessage *self,
                           GPBFieldDescriptor *field,
                           id dictionary);

//%PDDM-EXPAND-END GPB_ACCESSORS()

// clang-format on

/**
 * Returns an empty NSData to assign to byte fields when you wish to assign them
 * to empty. Prevents allocating a lot of little [NSData data] objects.
 **/
NSData *GPBEmptyNSData(void) __attribute__((pure));

/**
 * Drops the `unknownFields` from the given message and from all sub message.
 **/
void GPBMessageDropUnknownFieldsRecursively(GPBMessage *message);

NS_ASSUME_NONNULL_END

CF_EXTERN_C_END

// Disable clang-format for the macros.
// clang-format off

//%PDDM-DEFINE GPB_ACCESSORS()
//%
//%//
//%// Get/Set a given field from/to a message.
//%//
//%
//%// Single Fields
//%
//%GPB_ACCESSOR_SINGLE_FULL(Bytes, NSData, , *)
//%GPB_ACCESSOR_SINGLE_FULL(String, NSString, , *)
//%GPB_ACCESSOR_SINGLE_FULL(Message, GPBMessage, , *)
//%GPB_ACCESSOR_SINGLE_FULL(Group, GPBMessage, , *)
//%GPB_ACCESSOR_SINGLE(Bool, BOOL, )
//%GPB_ACCESSOR_SINGLE(Int32, int32_t, n)
//%GPB_ACCESSOR_SINGLE(UInt32, uint32_t, n)
//%GPB_ACCESSOR_SINGLE(Int64, int64_t, n)
//%GPB_ACCESSOR_SINGLE(UInt64, uint64_t, n)
//%GPB_ACCESSOR_SINGLE(Float, float, )
//%GPB_ACCESSOR_SINGLE(Double, double, )
//%/**
//% * Gets the given enum field of a message. For proto3, if the value isn't a
//% * member of the enum, @c kGPBUnrecognizedEnumeratorValue will be returned.
//% * GPBGetMessageRawEnumField will bypass the check and return whatever value
//% * was set.
//% *
//% * @param self  The message from which to get the field.
//% * @param field The field to get.
//% *
//% * @return The enum value for the given field.
//% **/
//%int32_t GPBGetMessageEnumField(GPBMessage *self, GPBFieldDescriptor *field);
//%
//%/**
//% * Set the given enum field of a message. You can only set values that are
//% * members of the enum.
//% *
//% * @param self  The message into which to set the field.
//% * @param field The field to set.
//% * @param value The enum value to set in the field.
//% **/
//%void GPBSetMessageEnumField(GPBMessage *self,
//%                            GPBFieldDescriptor *field,
//%                            int32_t value);
//%
//%/**
//% * Get the given enum field of a message. No check is done to ensure the value
//% * was defined in the enum.
//% *
//% * @param self  The message from which to get the field.
//% * @param field The field to get.
//% *
//% * @return The raw enum value for the given field.
//% **/
//%int32_t GPBGetMessageRawEnumField(GPBMessage *self, GPBFieldDescriptor *field);
//%
//%/**
//% * Set the given enum field of a message. You can set the value to anything,
//% * even a value that is not a member of the enum.
//% *
//% * @param self  The message into which to set the field.
//% * @param field The field to set.
//% * @param value The raw enum value to set in the field.
//% **/
//%void GPBSetMessageRawEnumField(GPBMessage *self,
//%                               GPBFieldDescriptor *field,
//%                               int32_t value);
//%
//%// Repeated Fields
//%
//%/**
//% * Gets the value of a repeated field.
//% *
//% * @param self  The message from which to get the field.
//% * @param field The repeated field to get.
//% *
//% * @return A GPB*Array or an NSMutableArray based on the field's type.
//% **/
//%id GPBGetMessageRepeatedField(GPBMessage *self, GPBFieldDescriptor *field);
//%
//%/**
//% * Sets the value of a repeated field.
//% *
//% * @param self  The message into which to set the field.
//% * @param field The field to set.
//% * @param array A GPB*Array or NSMutableArray based on the field's type.
//% **/
//%void GPBSetMessageRepeatedField(GPBMessage *self,
//%                                GPBFieldDescriptor *field,
//%                                id array);
//%
//%// Map Fields
//%
//%/**
//% * Gets the value of a map<> field.
//% *
//% * @param self  The message from which to get the field.
//% * @param field The repeated field to get.
//% *
//% * @return A GPB*Dictionary or NSMutableDictionary based on the field's type.
//% **/
//%id GPBGetMessageMapField(GPBMessage *self, GPBFieldDescriptor *field);
//%
//%/**
//% * Sets the value of a map<> field.
//% *
//% * @param self       The message into which to set the field.
//% * @param field      The field to set.
//% * @param dictionary A GPB*Dictionary or NSMutableDictionary based on the
//% *                   field's type.
//% **/
//%void GPBSetMessageMapField(GPBMessage *self,
//%                           GPBFieldDescriptor *field,
//%                           id dictionary);
//%

//%PDDM-DEFINE GPB_ACCESSOR_SINGLE(NAME, TYPE, AN)
//%GPB_ACCESSOR_SINGLE_FULL(NAME, TYPE, AN, )
//%PDDM-DEFINE GPB_ACCESSOR_SINGLE_FULL(NAME, TYPE, AN, TisP)
//%/**
//% * Gets the value of a##AN NAME$L field.
//% *
//% * @param self  The message from which to get the field.
//% * @param field The field to get.
//% **/
//%TYPE TisP##GPBGetMessage##NAME##Field(GPBMessage *self, GPBFieldDescriptor *field);
//%
//%/**
//% * Sets the value of a##AN NAME$L field.
//% *
//% * @param self  The message into which to set the field.
//% * @param field The field to set.
//% * @param value The to set in the field.
//% **/
//%void GPBSetMessage##NAME##Field(GPBMessage *self, GPBFieldDescriptor *field, TYPE TisP##value);
//%

// clang-format on
