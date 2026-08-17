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
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/cc/task/processor/proto/search_result_proto_inc.h"

#import "tensorflow_lite_support/ios/task/processor/sources/TFLSearchResult.h"

NS_ASSUME_NONNULL_BEGIN

@interface TFLSearchResult (Helpers)
/**
 * Creates and retruns a TFLSearchResult from the result of search task returned by TFLite Task C++
 * Library.
 *
 * @param cppSearchResult search result returned by TFLite Task C++ Library Search task.
 *
 * @return A new TFLSearchResult to be returned by inference methods of the iOS TF Lite Task Search
 * task.
 */
+ (nullable TFLSearchResult *)
    searchResultWithCppResult:
        (const tflite::support::StatusOr<tflite::task::processor::SearchResult> &)cppSearchResult
                        error:(NSError **)error;
@end

NS_ASSUME_NONNULL_END
