// Protocol Buffers - Google's data interchange format
// Copyright 2024 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#import <Foundation/Foundation.h>

#import "GPBMessage.h"
#import "GPBUnknownField.h"

@class GPBMessage;
@class GPBUnknownField;

NS_ASSUME_NONNULL_BEGIN

/**
 * A collection of unknown field numbers and their values.
 *
 * Note: `NSFastEnumeration` is supported to iterate over the `GPBUnknownFields`
 * in order.
 *
 * Reminder: Any field number can occur multiple times. For example, if a .proto
 * file were updated to a have a new (unpacked) repeated field, then each value
 * would appear independently. Likewise, it is possible that a number appears
 * multiple times with different data types, i.e. - unpacked vs. package repeated
 * fields from concatenating binary blobs of data.
 */
__attribute__((objc_subclassing_restricted))
@interface GPBUnknownFields : NSObject<NSCopying, NSFastEnumeration>

/**
 * Initializes a new instance with the data from the unknown fields from the given
 * message.
 *
 * Note: The instance is not linked to the message, any change will not be
 * reflected on the message the changes have to be pushed back to the message
 * with `-[GPBMessage mergeUnknownFields:extensionRegistry:error:]`.
 **/
- (instancetype)initFromMessage:(nonnull GPBMessage *)message;

/**
 * Initializes a new empty instance.
 **/
- (instancetype)init;

/**
 * The number of fields in this set. A single field number can appear in
 * multiple `GPBUnknownField` values as it might be a repeated field (it is
 * also possible that they have different `type` values (for example package vs
 * unpacked repeated fields).
 *
 * Note: `NSFastEnumeration` is supported to iterate over the fields in order.
 **/
@property(nonatomic, readonly, assign) NSUInteger count;

/** If the set is empty or not. */
@property(nonatomic, readonly, assign) BOOL empty;

/**
 * Removes all the fields current in the set.
 **/
- (void)clear;

/**
 * Fetches the subset of all the unknown fields that are for the given field
 * number.
 *
 * @returns An `NSArray` of `GPBUnknownField`s or `nil` if there were none.
 */
- (nullable NSArray<GPBUnknownField *> *)fields:(int32_t)fieldNumber;

/**
 * Add a new varint unknown field.
 *
 * @param fieldNumber The field number to use.
 * @param value The value to add.
 **/
- (void)addFieldNumber:(int32_t)fieldNumber varint:(uint64_t)value;

/**
 * Add a new fixed32 unknown field.
 *
 * @param fieldNumber The field number to use.
 * @param value The value to add.
 **/
- (void)addFieldNumber:(int32_t)fieldNumber fixed32:(uint32_t)value;

/**
 * Add a new fixed64 unknown field.
 *
 * @param fieldNumber The field number to use.
 * @param value The value to add.
 **/
- (void)addFieldNumber:(int32_t)fieldNumber fixed64:(uint64_t)value;

/**
 * Add a new length delimited (length prefixed) unknown field.
 *
 * @param fieldNumber The field number to use.
 * @param value The value to add.
 **/
- (void)addFieldNumber:(int32_t)fieldNumber lengthDelimited:(nonnull NSData *)value;

/**
 * Add a group (tag delimited) unknown field.
 *
 * @param fieldNumber The field number to use.
 *
 * @return A new `GPBUnknownFields` to set the field of the group too.
 **/
- (nonnull GPBUnknownFields *)addGroupWithFieldNumber:(int32_t)fieldNumber;

/**
 * Add the copy of the given unknown field.
 *
 * This can be useful from processing one `GPBUnknownFields` to create another.
 *
 * NOTE: If the field being copied is an Group, this instance added is new and thus
 * the `.group` of that result is also new, so if you intent is to modify the group
 * it *must* be fetched out of the result.
 *
 * @param field The field to add.
 *
 * @return The autoreleased field that was added.
 **/
- (GPBUnknownField *)addCopyOfField:(nonnull GPBUnknownField *)field;

/**
 * Removes the given field from the set.
 *
 * It is a programming error to attempt to remove a field that is not in this collection.
 *
 * Reminder: it is not save to mutate the collection while also using fast enumeration on it.
 *
 * @param field The field to remove.
 **/
- (void)removeField:(nonnull GPBUnknownField *)field;

/**
 * Removes all of the fields from the collection that have the given field number.
 *
 * If there are no fields with the given field number, this is a no-op.
 *
 * @param fieldNumber The field number to remove.
 **/
- (void)clearFieldNumber:(int32_t)fieldNumber;

@end

@interface GPBUnknownFields (AccessHelpers)

/**
 * Fetches the first varint for the given field number.
 *
 * @param fieldNumber The field number to look for.
 * @param outValue A pointer to receive the value if found
 *
 * @returns YES/NO on if there was a matching unknown field.
 **/
- (BOOL)getFirst:(int32_t)fieldNumber varint:(nonnull uint64_t *)outValue;

/**
 * Fetches the first fixed32 for the given field number.
 *
 * @param fieldNumber The field number to look for.
 * @param outValue A pointer to receive the value if found
 *
 * @returns YES/NO on if there was a matching unknown field.
 **/
- (BOOL)getFirst:(int32_t)fieldNumber fixed32:(nonnull uint32_t *)outValue;

/**
 * Fetches the first fixed64 for the given field number.
 *
 * @param fieldNumber The field number to look for.
 * @param outValue A pointer to receive the value if found
 *
 * @returns YES/NO on if there was a matching unknown field.
 **/
- (BOOL)getFirst:(int32_t)fieldNumber fixed64:(nonnull uint64_t *)outValue;

/**
 * Fetches the first length delimited (length prefixed) for the given field number.
 *
 * @param fieldNumber The field number to look for.
 *
 * @returns The first length delimited value for the given field number.
 **/
- (nullable NSData *)firstLengthDelimited:(int32_t)fieldNumber;

/**
 * Fetches the first group (tag delimited) field for the given field number.
 *
 * @param fieldNumber The field number to look for.
 *
 * @returns The first group for the given field number.
 **/
- (nullable GPBUnknownFields *)firstGroup:(int32_t)fieldNumber;

@end

NS_ASSUME_NONNULL_END
