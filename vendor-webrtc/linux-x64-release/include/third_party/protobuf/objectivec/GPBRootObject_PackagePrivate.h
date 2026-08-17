// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#import <Foundation/Foundation.h>

#import "GPBRootObject.h"

#import "GPBDescriptor.h"

@interface GPBRootObject ()

// Globally register.
+ (void)globallyRegisterExtension:(GPBExtensionDescriptor *)field;

@end

// Returns YES if the selector was resolved and added to the class,
// NO otherwise.
BOOL GPBResolveExtensionClassMethod(Class self, SEL sel);
