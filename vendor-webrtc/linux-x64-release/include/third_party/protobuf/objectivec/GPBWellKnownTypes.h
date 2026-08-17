// Protocol Buffers - Google's data interchange format
// Copyright 2015 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#import <Foundation/Foundation.h>

#import "GPBAny.pbobjc.h"
#import "GPBDuration.pbobjc.h"
#import "GPBTimestamp.pbobjc.h"

NS_ASSUME_NONNULL_BEGIN

#pragma mark - Errors

/** NSError domain used for errors. */
extern NSString *const GPBWellKnownTypesErrorDomain;

/** Error code for NSError with GPBWellKnownTypesErrorDomain. */
typedef NS_ENUM(NSInteger, GPBWellKnownTypesErrorCode) {
  /** The type_url could not be computed for the requested GPBMessage class. */
  GPBWellKnownTypesErrorCodeFailedToComputeTypeURL = -100,
  /** type_url in a Any doesn’t match that of the requested GPBMessage class. */
  GPBWellKnownTypesErrorCodeTypeURLMismatch = -101,
};

#pragma mark - GPBTimestamp

/**
 * Category for GPBTimestamp to work with standard Foundation time/date types.
 **/
@interface GPBTimestamp (GBPWellKnownTypes)

/** The NSDate representation of this GPBTimestamp. */
@property(nonatomic, readwrite, strong) NSDate *date;

/**
 * The NSTimeInterval representation of this GPBTimestamp.
 *
 * @note: Not all second/nanos combinations can be represented in a
 * NSTimeInterval, so getting this could be a lossy transform.
 **/
@property(nonatomic, readwrite) NSTimeInterval timeIntervalSince1970;

/**
 * Initializes a GPBTimestamp with the given NSDate.
 *
 * @param date The date to configure the GPBTimestamp with.
 *
 * @return A newly initialized GPBTimestamp.
 **/
- (instancetype)initWithDate:(NSDate *)date;

/**
 * Initializes a GPBTimestamp with the given NSTimeInterval.
 *
 * @param timeIntervalSince1970 Time interval to configure the GPBTimestamp with.
 *
 * @return A newly initialized GPBTimestamp.
 **/
- (instancetype)initWithTimeIntervalSince1970:(NSTimeInterval)timeIntervalSince1970;

@end

#pragma mark - GPBDuration

/**
 * Category for GPBDuration to work with standard Foundation time type.
 **/
@interface GPBDuration (GBPWellKnownTypes)

/**
 * The NSTimeInterval representation of this GPBDuration.
 *
 * @note: Not all second/nanos combinations can be represented in a
 * NSTimeInterval, so getting this could be a lossy transform.
 **/
@property(nonatomic, readwrite) NSTimeInterval timeInterval;

/**
 * Initializes a GPBDuration with the given NSTimeInterval.
 *
 * @param timeInterval Time interval to configure the GPBDuration with.
 *
 * @return A newly initialized GPBDuration.
 **/
- (instancetype)initWithTimeInterval:(NSTimeInterval)timeInterval;

@end

#pragma mark - GPBAny

/**
 * Category for GPBAny to help work with the message within the object.
 **/
@interface GPBAny (GBPWellKnownTypes)

/**
 * Convenience method to create a GPBAny containing the serialized message.
 * This uses type.googleapis.com/ as the type_url's prefix.
 *
 * @param message  The message to be packed into the GPBAny.
 * @param errorPtr Pointer to an error that will be populated if something goes
 *                 wrong.
 *
 * @return A newly configured GPBAny with the given message, or nil on failure.
 */
+ (nullable instancetype)anyWithMessage:(nonnull GPBMessage *)message error:(NSError **)errorPtr;

/**
 * Convenience method to create a GPBAny containing the serialized message.
 *
 * @param message       The message to be packed into the GPBAny.
 * @param typeURLPrefix The URL prefix to apply for type_url.
 * @param errorPtr      Pointer to an error that will be populated if something
 *                      goes wrong.
 *
 * @return A newly configured GPBAny with the given message, or nil on failure.
 */
+ (nullable instancetype)anyWithMessage:(nonnull GPBMessage *)message
                          typeURLPrefix:(nonnull NSString *)typeURLPrefix
                                  error:(NSError **)errorPtr;

/**
 * Initializes a GPBAny to contain the serialized message. This uses
 * type.googleapis.com/ as the type_url's prefix.
 *
 * @param message  The message to be packed into the GPBAny.
 * @param errorPtr Pointer to an error that will be populated if something goes
 *                 wrong.
 *
 * @return A newly configured GPBAny with the given message, or nil on failure.
 */
- (nullable instancetype)initWithMessage:(nonnull GPBMessage *)message error:(NSError **)errorPtr;

/**
 * Initializes a GPBAny to contain the serialized message.
 *
 * @param message       The message to be packed into the GPBAny.
 * @param typeURLPrefix The URL prefix to apply for type_url.
 * @param errorPtr      Pointer to an error that will be populated if something
 *                      goes wrong.
 *
 * @return A newly configured GPBAny with the given message, or nil on failure.
 */
- (nullable instancetype)initWithMessage:(nonnull GPBMessage *)message
                           typeURLPrefix:(nonnull NSString *)typeURLPrefix
                                   error:(NSError **)errorPtr;

/**
 * Packs the serialized message into this GPBAny. This uses
 * type.googleapis.com/ as the type_url's prefix.
 *
 * @param message  The message to be packed into the GPBAny.
 * @param errorPtr Pointer to an error that will be populated if something goes
 *                 wrong.
 *
 * @return Whether the packing was successful or not.
 */
- (BOOL)packWithMessage:(nonnull GPBMessage *)message error:(NSError **)errorPtr;

/**
 * Packs the serialized message into this GPBAny.
 *
 * @param message       The message to be packed into the GPBAny.
 * @param typeURLPrefix The URL prefix to apply for type_url.
 * @param errorPtr      Pointer to an error that will be populated if something
 *                      goes wrong.
 *
 * @return Whether the packing was successful or not.
 */
- (BOOL)packWithMessage:(nonnull GPBMessage *)message
          typeURLPrefix:(nonnull NSString *)typeURLPrefix
                  error:(NSError **)errorPtr;

/**
 * Unpacks the serialized message as if it was an instance of the given class.
 *
 * @note When checking type_url, the base URL is not checked, only the fully
 *       qualified name.
 *
 * @param messageClass The class to use to deserialize the contained message.
 * @param errorPtr     Pointer to an error that will be populated if something
 *                     goes wrong.
 *
 * @return An instance of the given class populated with the contained data, or
 *         nil on failure.
 */
- (nullable GPBMessage *)unpackMessageClass:(Class)messageClass error:(NSError **)errorPtr;

/**
 * Unpacks the serialized message as if it was an instance of the given class.
 *
 * @note When checking type_url, the base URL is not checked, only the fully
 *       qualified name.
 *
 * @param messageClass The class to use to deserialize the contained message.
 * @param extensionRegistry The extension registry to use to look up extensions.
 * @param errorPtr     Pointer to an error that will be populated if something
 *                     goes wrong.
 *
 * @return An instance of the given class populated with the contained data, or
 *         nil on failure.
 */
- (nullable GPBMessage *)unpackMessageClass:(Class)messageClass
                          extensionRegistry:(nullable id<GPBExtensionRegistry>)extensionRegistry
                                      error:(NSError **)errorPtr;

@end

NS_ASSUME_NONNULL_END
