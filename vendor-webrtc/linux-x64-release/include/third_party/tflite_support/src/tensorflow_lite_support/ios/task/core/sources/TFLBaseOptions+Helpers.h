/* Copyright 2021 The TensorFlow Authors. All Rights Reserved.

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
#include "tensorflow_lite_support/c/task/core/base_options.h"
#import "tensorflow_lite_support/ios/task/core/sources/TFLBaseOptions.h"

NS_ASSUME_NONNULL_BEGIN

@interface TFLBaseOptions (Helpers)
- (void)copyToCOptions:(TfLiteBaseOptions *)cBaseOptions;
@end

NS_ASSUME_NONNULL_END
