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
#import "tensorflow_lite_support/ios/task/core/sources/TFLExternalFile.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * Holds options for search task.
 */
NS_SWIFT_NAME(SearchOptions)
@interface TFLSearchOptions : NSObject <NSCopying>

/**
 * The index file to search into. Mandatory only if the index is not attached to the output tensor
 * metadata as an AssociatedFile with type SCANN_INDEX_FILE. Note that in case both are provided,
 * this field takes precedence.
 */
@property(nonatomic, copy) TFLExternalFile *indexFile;

/**
 * Maximum number of nearest neighbor results to return. Defaults to 5.
 */
@property(nonatomic) NSInteger maxResults;

@end

NS_ASSUME_NONNULL_END
