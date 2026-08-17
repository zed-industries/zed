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

NS_ASSUME_NONNULL_BEGIN

/** Encapsulates information about a class in the classification results. */
NS_SWIFT_NAME(ClassificationCategory)
@interface TFLCategory : NSObject

/** Index of the class in the corresponding label map, usually packed in the TFLite Model
 * Metadata. */
@property(nonatomic, readonly) NSInteger index;

/** Confidence score for this class . */
@property(nonatomic, readonly) float score;

/** Class name of the class. */
@property(nonatomic, readonly, nullable) NSString *label;

/** Display name of the class. */
@property(nonatomic, readonly, nullable) NSString *displayName;

/**
 * Initializes a new `TFLCategory` with the given index, score, label and display name.
 *
 * @param index Index of the class in the corresponding label map, usually packed in the TFLite
 * Model Metadata.
 *
 * @param score Confidence score for this class.
 *
 * @param label Class name of the class.
 *
 * @param displayName Display name of the class.
 *
 * @return An instance of `TFLCategory` initialized with the given index, score, label and display name.
 */
- (instancetype)initWithIndex:(NSInteger)index
                        score:(float)score
                        label:(nullable NSString *)label
                  displayName:(nullable NSString *)displayName;

- (instancetype)init NS_UNAVAILABLE;

+ (instancetype)new NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
