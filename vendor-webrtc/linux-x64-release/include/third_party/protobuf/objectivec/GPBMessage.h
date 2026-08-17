// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#import <Foundation/Foundation.h>

#import "GPBBootstrap.h"
#import "GPBCodedInputStream.h"
#import "GPBCodedOutputStream.h"
#import "GPBDescriptor.h"
#import "GPBExtensionRegistry.h"
#import "GPBUnknownFields.h"

@class GPBCodedInputStream;
@class GPBCodedOutputStream;
@class GPBUnknownFields;

NS_ASSUME_NONNULL_BEGIN

CF_EXTERN_C_BEGIN

/** NSError domain used for errors. */
extern NSString *const GPBMessageErrorDomain;

/** Error codes for NSErrors originated in GPBMessage. */
typedef NS_ENUM(NSInteger, GPBMessageErrorCode) {
  /** Uncategorized error. */
  GPBMessageErrorCodeOther = -100,
  /** Message couldn't be serialized because it is missing required fields. */
  GPBMessageErrorCodeMissingRequiredField = -101,
};

/**
 * Key under which the GPBMessage error's reason is stored inside the userInfo
 * dictionary.
 **/
extern NSString *const GPBErrorReasonKey;

/**
 * An exception name raised during serialization when the message would be
 * larger than the 2GB limit.
 **/
extern NSString *const GPBMessageExceptionMessageTooLarge;

CF_EXTERN_C_END

/**
 * Base class that each generated message subclasses from.
 *
 * @note @c NSCopying support is a "deep copy", in that all sub objects are
 *       copied.  Just like you wouldn't want a UIView/NSView trying to
 *       exist in two places, you don't want a sub message to be a property
 *       property of two other messages.
 *
 * @note While the class supports NSSecureCoding, if the message has any
 *       extensions, they will end up reloaded in the unknown fields as there is
 *       no way for the @c NSCoding plumbing to pass through a
 *       @c GPBExtensionRegistry. To support extensions, instead of passing the
 *       calls off to the Message, simple store the result of @c data, and then
 *       when loading, fetch the data and use
 *       @c +parseFromData:extensionRegistry:error: to provide an extension
 *       registry.
 **/
@interface GPBMessage : NSObject <NSSecureCoding, NSCopying>

// If you add an instance method/property to this class that may conflict with
// fields declared in protos, you need to update objective_helpers.cc. The main
// cases are methods that take no arguments, or setFoo:/hasFoo: type methods.

/**
 * Whether the message, along with all submessages, have the required fields
 * set.
 **/
@property(nonatomic, readonly, getter=isInitialized) BOOL initialized;

/**
 * @return An autoreleased message with the default values set.
 **/
+ (instancetype)message;

/**
 * Creates a new instance by parsing the provided data. This method should be
 * sent to the generated message class that the data should be interpreted as.
 * If there is an error the method returns nil and the error is returned in
 * errorPtr (when provided).
 *
 * @note In DEBUG builds, the parsed message is checked to be sure all required
 *       fields were provided, and the parse will fail if some are missing.
 *
 * @note The errors returned are likely coming from the domain and codes listed
 *       at the top of this file and GPBCodedInputStream.h.
 *
 * @param data     The data to parse.
 * @param errorPtr An optional error pointer to fill in with a failure reason if
 *                 the data can not be parsed.
 *
 * @return A new instance of the generated class.
 **/
+ (nullable instancetype)parseFromData:(NSData *)data error:(NSError **)errorPtr;

/**
 * Creates a new instance by parsing the data. This method should be sent to
 * the generated message class that the data should be interpreted as. If
 * there is an error the method returns nil and the error is returned in
 * errorPtr (when provided).
 *
 * @note In DEBUG builds, the parsed message is checked to be sure all required
 *       fields were provided, and the parse will fail if some are missing.
 *
 * @note The errors returned are likely coming from the domain and codes listed
 *       at the top of this file and GPBCodedInputStream.h.
 *
 * @param data              The data to parse.
 * @param extensionRegistry The extension registry to use to look up extensions.
 * @param errorPtr          An optional error pointer to fill in with a failure
 *                          reason if the data can not be parsed.
 *
 * @return A new instance of the generated class.
 **/
+ (nullable instancetype)parseFromData:(NSData *)data
                     extensionRegistry:(nullable id<GPBExtensionRegistry>)extensionRegistry
                                 error:(NSError **)errorPtr;

/**
 * Creates a new instance by parsing the data from the given input stream. This
 * method should be sent to the generated message class that the data should
 * be interpreted as. If there is an error the method returns nil and the error
 * is returned in errorPtr (when provided).
 *
 * @note In DEBUG builds, the parsed message is checked to be sure all required
 *       fields were provided, and the parse will fail if some are missing.
 *
 * @note The errors returned are likely coming from the domain and codes listed
 *       at the top of this file and GPBCodedInputStream.h.
 *
 * @param input             The stream to read data from.
 * @param extensionRegistry The extension registry to use to look up extensions.
 * @param errorPtr          An optional error pointer to fill in with a failure
 *                          reason if the data can not be parsed.
 *
 * @return A new instance of the generated class.
 **/
+ (nullable instancetype)parseFromCodedInputStream:(GPBCodedInputStream *)input
                                 extensionRegistry:
                                     (nullable id<GPBExtensionRegistry>)extensionRegistry
                                             error:(NSError **)errorPtr;

/**
 * Creates a new instance by parsing the data from the given input stream. This
 * method should be sent to the generated message class that the data should
 * be interpreted as. If there is an error the method returns nil and the error
 * is returned in errorPtr (when provided).
 *
 * @note Unlike the parseFrom... methods, this never checks to see if all of
 *       the required fields are set. So this method can be used to reload
 *       messages that may not be complete.
 *
 * @note The errors returned are likely coming from the domain and codes listed
 *       at the top of this file and GPBCodedInputStream.h.
 *
 * @param input             The stream to read data from.
 * @param extensionRegistry The extension registry to use to look up extensions.
 * @param errorPtr          An optional error pointer to fill in with a failure
 *                          reason if the data can not be parsed.
 *
 * @return A new instance of the generated class.
 **/
+ (nullable instancetype)parseDelimitedFromCodedInputStream:(GPBCodedInputStream *)input
                                          extensionRegistry:
                                              (nullable id<GPBExtensionRegistry>)extensionRegistry
                                                      error:(NSError **)errorPtr;

/**
 * Initializes an instance by parsing the data. This method should be sent to
 * the generated message class that the data should be interpreted as. If
 * there is an error the method returns nil and the error is returned in
 * errorPtr (when provided).
 *
 * @note In DEBUG builds, the parsed message is checked to be sure all required
 *       fields were provided, and the parse will fail if some are missing.
 *
 * @note The errors returned are likely coming from the domain and codes listed
 *       at the top of this file and GPBCodedInputStream.h.
 *
 * @param data     The data to parse.
 * @param errorPtr An optional error pointer to fill in with a failure reason if
 *                 the data can not be parsed.
 *
 * @return An initialized instance of the generated class.
 **/
- (nullable instancetype)initWithData:(NSData *)data error:(NSError **)errorPtr;

/**
 * Initializes an instance by parsing the data. This method should be sent to
 * the generated message class that the data should be interpreted as. If
 * there is an error the method returns nil and the error is returned in
 * errorPtr (when provided).
 *
 * @note In DEBUG builds, the parsed message is checked to be sure all required
 *       fields were provided, and the parse will fail if some are missing.
 *
 * @note The errors returned are likely coming from the domain and codes listed
 *       at the top of this file and GPBCodedInputStream.h.
 *
 * @param data              The data to parse.
 * @param extensionRegistry The extension registry to use to look up extensions.
 * @param errorPtr          An optional error pointer to fill in with a failure
 *                          reason if the data can not be parsed.
 *
 * @return An initialized instance of the generated class.
 **/
- (nullable instancetype)initWithData:(NSData *)data
                    extensionRegistry:(nullable id<GPBExtensionRegistry>)extensionRegistry
                                error:(NSError **)errorPtr;

/**
 * Initializes an instance by parsing the data from the given input stream. This
 * method should be sent to the generated message class that the data should
 * be interpreted as. If there is an error the method returns nil and the error
 * is returned in errorPtr (when provided).
 *
 * @note Unlike the parseFrom... methods, this never checks to see if all of
 *       the required fields are set. So this method can be used to reload
 *       messages that may not be complete.
 *
 * @note The errors returned are likely coming from the domain and codes listed
 *       at the top of this file and GPBCodedInputStream.h.
 *
 * @param input             The stream to read data from.
 * @param extensionRegistry The extension registry to use to look up extensions.
 * @param errorPtr          An optional error pointer to fill in with a failure
 *                          reason if the data can not be parsed.
 *
 * @return An initialized instance of the generated class.
 **/
- (nullable instancetype)initWithCodedInputStream:(GPBCodedInputStream *)input
                                extensionRegistry:
                                    (nullable id<GPBExtensionRegistry>)extensionRegistry
                                            error:(NSError **)errorPtr;

/**
 * Parses the given data as this message's class, and merges those values into
 * this message.
 *
 * @param data              The binary representation of the message to merge.
 * @param extensionRegistry The extension registry to use to look up extensions.
 * @param errorPtr          An optional error pointer to fill in with a failure
 *                          reason if the data can not be parsed. Will only be
 *                          filled in if the data failed to be parsed.
 *
 * @return Boolean indicating success. errorPtr will only be fill in on failure.
 **/
- (BOOL)mergeFromData:(NSData *)data
    extensionRegistry:(nullable id<GPBExtensionRegistry>)extensionRegistry
                error:(NSError **)errorPtr;

/**
 * Merges the fields from another message (of the same type) into this
 * message.
 *
 * @param other Message to merge into this message.
 **/
- (void)mergeFrom:(GPBMessage *)other;

/**
 * Writes out the message to the given coded output stream.
 *
 * @param output The coded output stream into which to write the message.
 *
 * @note This can raise the GPBCodedOutputStreamException_* exceptions.
 *
 * @note The most common cause of this failing is from one thread calling this
 *       while another thread has a reference to this message or a message used
 *       within a field and that other thread mutating the message while this
 *       serialization is taking place.
 **/
- (void)writeToCodedOutputStream:(GPBCodedOutputStream *)output;

/**
 * Writes out the message to the given output stream.
 *
 * @param output The output stream into which to write the message.
 *
 * @note This can raise the GPBCodedOutputStreamException_* exceptions.
 *
 * @note The most common cause of this failing is from one thread calling this
 *       while another thread has a reference to this message or a message used
 *       within a field and that other thread mutating the message while this
 *       serialization is taking place.
 **/
- (void)writeToOutputStream:(NSOutputStream *)output;

/**
 * Writes out a varint for the message size followed by the message to
 * the given output stream.
 *
 * @param output The coded output stream into which to write the message.
 *
 * @note This can raise the GPBCodedOutputStreamException_* exceptions.
 *
 * @note The most common cause of this failing is from one thread calling this
 *       while another thread has a reference to this message or a message used
 *       within a field and that other thread mutating the message while this
 *       serialization is taking place.
 **/
- (void)writeDelimitedToCodedOutputStream:(GPBCodedOutputStream *)output;

/**
 * Writes out a varint for the message size followed by the message to
 * the given output stream.
 *
 * @param output The output stream into which to write the message.
 *
 * @note This can raise the GPBCodedOutputStreamException_* exceptions.
 *
 * @note The most common cause of this failing is from one thread calling this
 *       while another thread has a reference to this message or a message used
 *       within a field and that other thread mutating the message while this
 *       serialization is taking place.
 **/
- (void)writeDelimitedToOutputStream:(NSOutputStream *)output;

/**
 * Serializes the message to an NSData.
 *
 * If there is an error while generating the data, nil is returned.
 *
 * @note This value is not cached, so if you are using it repeatedly, cache
 *       it yourself.
 *
 * @note In DEBUG ONLY, the message is also checked for all required field,
 *       if one is missing, nil will be returned.
 *
 * @note The most common cause of this failing is from one thread calling this
 *       while another thread has a reference to this message or a message used
 *       within a field and that other thread mutating the message while this
 *       serialization is taking place.
 *
 * @return The binary representation of the message.
 **/
- (nullable NSData *)data;

/**
 * Serializes a varint with the message size followed by the message data,
 * returning that as an NSData.
 *
 * @note This value is not cached, so if you are using it repeatedly, it is
 *       recommended to keep a local copy.
 *
 * @note The most common cause of this failing is from one thread calling this
 *       while another thread has a reference to this message or a message used
 *       within a field and that other thread mutating the message while this
 *       serialization is taking place.
 *
 * @return The binary representation of the size along with the message.
 **/
- (NSData *)delimitedData;

/**
 * Calculates the size of the object if it were serialized.
 *
 * This is not a cached value. If you are following a pattern like this:
 *
 * ```
 * size_t size = [aMsg serializedSize];
 * NSMutableData *foo = [NSMutableData dataWithCapacity:size + sizeof(size)];
 * [foo writeSize:size];
 * [foo appendData:[aMsg data]];
 * ```
 *
 * you would be better doing:
 *
 * ```
 * NSData *data = [aMsg data];
 * NSUInteger size = [aMsg length];
 * NSMutableData *foo = [NSMutableData dataWithCapacity:size + sizeof(size)];
 * [foo writeSize:size];
 * [foo appendData:data];
 * ```
 *
 * @return The size of the message in it's binary representation.
 **/
- (size_t)serializedSize;

/**
 * @return The descriptor for the message class.
 **/
+ (GPBDescriptor *)descriptor;

/**
 * Return the descriptor for the message.
 **/
- (GPBDescriptor *)descriptor;

/**
 * @return An array with the extension descriptors that are currently set on the
 * message.
 **/
- (NSArray *)extensionsCurrentlySet;

/**
 * Checks whether there is an extension set on the message which matches the
 * given extension descriptor.
 *
 * @param extension Extension descriptor to check if it's set on the message.
 *
 * @return Whether the extension is currently set on the message.
 **/
- (BOOL)hasExtension:(GPBExtensionDescriptor *)extension;

/*
 * Fetches the given extension's value for this message.
 *
 * Extensions use boxed values (NSNumbers) for PODs and NSMutableArrays for
 * repeated fields. If the extension is a Message one will be auto created for
 * you and returned similar to fields.
 *
 * NOTE: For Enum extensions, if the enum was _closed_, then unknown values
 * were parsed into the message's unknown fields instead of ending up in the
 * extensions, just like what happens with singular/repeated fields. For open
 * enums, the _raw_ value will be in the NSNumber, meaning if one does a
 * `switch` on the values, a `default` case should also be included.
 *
 * @param extension The extension descriptor of the extension to fetch.
 *
 * @return The extension matching the given descriptor, or nil if none found.
 **/
- (nullable id)getExtension:(GPBExtensionDescriptor *)extension;

/**
 * Sets the given extension's value for this message. This only applies for
 * single field extensions (i.e. - not repeated fields).
 *
 * Extensions use boxed values (NSNumbers).
 *
 * @param extension The extension descriptor under which to set the value.
 * @param value     The value to be set as the extension.
 **/
- (void)setExtension:(GPBExtensionDescriptor *)extension value:(nullable id)value;

/**
 * Adds the given value to the extension for this message. This only applies
 * to repeated field extensions. If the field is a repeated POD type, the value
 * should be an NSNumber.
 *
 * @param extension The extension descriptor under which to add the value.
 * @param value     The value to be added to the repeated extension.
 **/
- (void)addExtension:(GPBExtensionDescriptor *)extension value:(id)value;

/**
 * Replaces the value at the given index with the given value for the extension
 * on this message. This only applies to repeated field extensions. If the field
 * is a repeated POD type, the value is should be an NSNumber.
 *
 * @param extension The extension descriptor under which to replace the value.
 * @param index     The index of the extension to be replaced.
 * @param value     The value to be replaced in the repeated extension.
 **/
- (void)setExtension:(GPBExtensionDescriptor *)extension index:(NSUInteger)index value:(id)value;

/**
 * Clears the given extension for this message.
 *
 * @param extension The extension descriptor to be cleared from this message.
 **/
- (void)clearExtension:(GPBExtensionDescriptor *)extension;

/**
 * Resets all of the fields of this message to their default values.
 **/
- (void)clear;

/**
 * Clears any unknown fields on this message.
 *
 * Note: To clear this message's unknown field and all the unknown fields of the
 * messages within the fields of this message, use
 * `GPBMessageDropUnknownFieldsRecursively()`.
 **/
- (void)clearUnknownFields;

/**
 * Merges in the data from an `GPBUnknownFields`, meaning the data from the unknown fields gets
 * re-parsed so any known fields will be properly set.
 *
 * If the intent is to *replace* the message's unknown fields, call `-clearUnknownFields` first.
 *
 * Since the data from the GPBUnknownFields will always be well formed, this call will almost never
 * fail. What could cause it to fail is if the GPBUnknownFields contains a field value that is
 * an error for the message's schema - i.e.: if it contains a length delimited field where the
 * field number for the message is defined to be a _string_ field, however the length delimited
 * data provide is not a valid UTF8 string, or if the field is a _packed_ number field, but the
 * data provided is not a valid for that field.
 *
 * @param unknownFields     The unknown fields to merge the data from.
 * @param extensionRegistry The extension registry to use to look up extensions, can be `nil`.
 * @param errorPtr          An optional error pointer to fill in with a failure
 *                          reason if the data can not be parsed. Will only be
 *                          filled in if the data failed to be parsed.
 *
 * @return Boolean indicating success. errorPtr will only be fill in on failure.
 **/
- (BOOL)mergeUnknownFields:(GPBUnknownFields *)unknownFields
         extensionRegistry:(nullable id<GPBExtensionRegistry>)extensionRegistry
                     error:(NSError **)errorPtr;

@end

NS_ASSUME_NONNULL_END
