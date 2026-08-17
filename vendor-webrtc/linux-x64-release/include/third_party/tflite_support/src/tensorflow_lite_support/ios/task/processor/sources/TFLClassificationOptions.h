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
#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

/**
 * Holds settings for any single classification task.
 */
NS_SWIFT_NAME(ClassificationOptions)
@interface TFLClassificationOptions : NSObject <NSCopying>

/** If set, all classes  in this list will be filtered out from the results . */
@property(nonatomic, copy) NSArray *labelDenyList;

/** If set, all classes not in this list will be filtered out from the results . */
@property(nonatomic, copy) NSArray *labelAllowList;

/** Display names local for display names*/
@property(nonatomic, copy) NSString *displayNamesLocale;

/** Results with score threshold greater than this value are returned . */
@property(nonatomic) float scoreThreshold;

/** Limit to the number of classes that can be returned in results. */
@property(nonatomic) NSInteger maxResults;

@end

NS_ASSUME_NONNULL_END
