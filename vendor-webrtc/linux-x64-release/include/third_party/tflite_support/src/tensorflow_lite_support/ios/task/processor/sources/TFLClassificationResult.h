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
#import "tensorflow_lite_support/ios/task/processor/sources/TFLCategory.h"

NS_ASSUME_NONNULL_BEGIN

/** Encapsulates list of predicted classes (aka labels) for a given image classifier head. */
NS_SWIFT_NAME(Classifications)
@interface TFLClassifications : NSObject

/**
 * The index of the image classifier head these classes refer to. This is useful for multi-head
 * models.
 */
@property(nonatomic, readonly) NSInteger headIndex;

/** The name of the classifier head, which is the corresponding tensor metadata
 * name. See https://github.com/tensorflow/tflite-support/blob/710e323265bfb71fdbdd72b3516e00cff15c0326/tensorflow_lite_support/metadata/metadata_schema.fbs#L545
 * This will always be NULL for the `TFLClassifications` in the `TFLClassificationResult` returned by the follwing methods of `TFLImageClassifier`.
 * 1. -[TFLImageClassifier classifyWithGMLImage:error:]
 * 2. -[TFLImageClassifier classifyWithGMLImage:regionOfInterest:error:]
 */
@property(nonatomic, readonly) NSString *headName;

/** The array of predicted classes, usually sorted by descending scores (e.g.from high to low
 * probability). */
@property(nonatomic, readonly) NSArray<TFLCategory *> *categories;

/**
 * Initializes a new `TFLClassifications` with the given head index and array of categories.
 * head name is initialized to `nil`.
 *
 * @param headIndex The index of the image classifier head these classes refer to.
 * @param categories An array of `TFLCategory` objects encapsulating a list of
 * predictions usually sorted by descending scores (e.g. from high to low probability).
 *
 * @return An instance of `TFLClassifications` initialized with the given head index and
 * array of categories.
 */
- (instancetype)initWithHeadIndex:(NSInteger)headIndex
                       categories:(NSArray<TFLCategory *> *)categories;


/**
 * Initializes a new `TFLClassifications` with the given head index, head name and array of categories.
 *
 * @param headIndex The index of the image classifier head these classes refer to.
 * @param headName The name of the classifier head, which is the corresponding tensor metadata
 * name.
 * @param categories An array of `TFLCategory` objects encapsulating a list of
 * predictions usually sorted by descending scores (e.g. from high to low probability).
 *
 * @return An object of `TFLClassifications` initialized with the given head index, head name and
 * array of categories.
 */
- (instancetype)initWithHeadIndex:(NSInteger)headIndex
                         headName:(nullable NSString *)headName
                       categories:(NSArray<TFLCategory *> *)categories;                     

@end

/** Encapsulates results of any classification task. */
NS_SWIFT_NAME(ClassificationResult)
@interface TFLClassificationResult : NSObject

/** Array of TFLClassifications objects containing image classifier predictions per image classifier
 * head.
 */
@property(nonatomic, readonly) NSArray<TFLClassifications *> *classifications;

/**
 * Initializes a new `TFLClassificationResult` with the given array of classifications.
 *
 * @param classifications An Aaray of `TFLClassifications` objects containing image classifier
 * predictions per image classifier head.
 *
 * @return An instance of 1TFLClassificationResult1 initialized with the given array of classifications.
 */
- (instancetype)initWithClassifications:(NSArray<TFLClassifications *> *)classifications;

@end

NS_ASSUME_NONNULL_END
