// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#import <Foundation/Foundation.h>

#import "GPBExtensionRegistry.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * Every generated proto file defines a local "Root" class that exposes a
 * GPBExtensionRegistry for all the extensions defined by that file and
 * the files it depends on.
 **/
@interface GPBRootObject : NSObject

/**
 * @return An extension registry for the given file and all the files it depends
 * on.
 **/
+ (GPBExtensionRegistry *)extensionRegistry;

@end

NS_ASSUME_NONNULL_END
