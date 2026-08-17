// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#import <Foundation/Foundation.h>

#import "GPBDescriptor.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * A table of known extensions, searchable by name or field number.  When
 * parsing a protocol message that might have extensions, you must provide a
 * GPBExtensionRegistry in which you have registered any extensions that you
 * want to be able to parse. Otherwise, those extensions will just be treated
 * like unknown fields.
 **/
@protocol GPBExtensionRegistry <NSObject>

/**
 * Looks for the extension registered for the given field number on a given
 * GPBDescriptor.
 *
 * @param descriptor  The descriptor to look for a registered extension on.
 * @param fieldNumber The field number of the extension to look for.
 *
 * @return The registered GPBExtensionDescriptor or nil if none was found.
 **/
- (nullable GPBExtensionDescriptor *)extensionForDescriptor:(GPBDescriptor *)descriptor
                                                fieldNumber:(NSInteger)fieldNumber;
@end

/**
 * A concrete implementation of `GPBExtensionRegistry`.
 *
 * The *Root classes provide `+extensionRegistry` for the extensions defined
 * in a given file *and* all files it imports. You can also create a
 * GPBExtensionRegistry, and merge those registries to handle parsing
 * extensions defined from non overlapping files.
 *
 * ```
 * GPBExtensionRegistry *registry = [[MyProtoFileRoot extensionRegistry] copy];
 * [registry addExtension:[OtherMessage neededExtension]]; // Not in MyProtoFile
 * NSError *parseError;
 * MyMessage *msg = [MyMessage parseData:data extensionRegistry:registry error:&parseError];
 * ```
 **/
__attribute__((objc_subclassing_restricted))
@interface GPBExtensionRegistry : NSObject<NSCopying, GPBExtensionRegistry>

/**
 * Adds the given GPBExtensionDescriptor to this registry.
 *
 * @param extension The extension description to add.
 **/
- (void)addExtension:(GPBExtensionDescriptor *)extension;

/**
 * Adds all the extensions from another registry to this registry.
 *
 * @param registry The registry to merge into this registry.
 **/
- (void)addExtensions:(GPBExtensionRegistry *)registry;

@end

NS_ASSUME_NONNULL_END
