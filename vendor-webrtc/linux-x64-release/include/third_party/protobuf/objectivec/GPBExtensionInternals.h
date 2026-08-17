// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#import <Foundation/Foundation.h>

#import "GPBCodedOutputStream.h"
#import "GPBDescriptor.h"

size_t GPBComputeExtensionSerializedSizeIncludingTag(
    GPBExtensionDescriptor *extension, id value);

void GPBWriteExtensionValueToOutputStream(GPBExtensionDescriptor *extension,
                                          id value,
                                          GPBCodedOutputStream *output);
