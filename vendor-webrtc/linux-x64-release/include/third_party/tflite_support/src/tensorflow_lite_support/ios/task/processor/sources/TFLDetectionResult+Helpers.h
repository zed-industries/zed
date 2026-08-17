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
#include "tensorflow_lite_support/c/task/processor/detection_result.h"
#import "tensorflow_lite_support/ios/task/processor/sources/TFLDetectionResult.h"

NS_ASSUME_NONNULL_BEGIN

@interface TFLDetectionResult (Helpers)
/**
 * Creates and retrurns a TFLDetectionResult from a TfLiteDetectionResult returned by
 * TFLite Task C Library Object Detection task.
 *
 * @param cDetectionResult Detection  results returned by TFLite Task C Library
 * Object Detection task.
 *
 * @return Detection Result of type TFLDetectionResult to be returned by inference methods
 * of the iOS TF Lite Task Object Detection task.
 */
+ (TFLDetectionResult *)detectionResultWithCResult:(TfLiteDetectionResult *)cDetectionResult;
@end

NS_ASSUME_NONNULL_END
