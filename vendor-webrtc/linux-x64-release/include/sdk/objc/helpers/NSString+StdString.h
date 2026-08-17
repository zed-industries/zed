/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#include <string>

#include "absl/strings/string_view.h"

NS_ASSUME_NONNULL_BEGIN

@interface NSString (StdString)

@property(nonatomic, readonly) std::string stdString;

+ (std::string)stdStringForString:(NSString *)nsString;
+ (NSString *)stringForStdString:(const std::string &)stdString;

@end

@interface NSString (AbslStringView)

+ (NSString *)stringForAbslStringView:(const absl::string_view)abslStringView;

@end

NS_ASSUME_NONNULL_END
