/* Copyright 2022 The TensorFlow Authors. All Rights Reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/
#import <Foundation/Foundation.h>

#import "tensorflow_lite_support/ios/sources/TFLCommonUtils.h"

#include "absl/status/status.h"  // from @com_google_absl

NS_ASSUME_NONNULL_BEGIN

/** Helper utility for the all tasks which encapsulates common functionality of
 * the iOS task library backed by C++ APIs. */
@interface TFLCommonCppUtils : TFLCommonUtils

/**
 * Converts an absl status to an NSError.
 *
 * @param status absl status.
 * @param error Pointer to the memory location where the created error should be saved. If `nil`,
 * no error will be saved.
 */
+ (BOOL)checkCppError:(const absl::Status &)status toError:(NSError **)error;

@end

NS_ASSUME_NONNULL_END
