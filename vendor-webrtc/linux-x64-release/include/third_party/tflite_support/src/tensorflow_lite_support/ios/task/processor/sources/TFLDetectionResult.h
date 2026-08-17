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
#import <CoreGraphics/CoreGraphics.h>
#import <Foundation/Foundation.h>

#import "tensorflow_lite_support/ios/task/processor/sources/TFLCategory.h"

NS_ASSUME_NONNULL_BEGIN

/** Encapsulates list of predicted classes (aka labels) and bounding box for a detected object. */
NS_SWIFT_NAME(Detection)
@interface TFLDetection : NSObject

/**
 * The index of the image classifier head these classes refer to. This is useful for multi-head
 * models.
 */
@property(nonatomic, readonly) CGRect boundingBox;

/** The array of predicted classes, usually sorted by descending scores (e.g.from high to low
 * probability). */
@property(nonatomic, readonly) NSArray<TFLCategory *> *categories;

/**
 * Initializes an object of `TFLDetection` with the given bounding box and array of categories.
 *
 * @param boundingBox CGRect specifying the bounds of the object represented by this detection.
 * @param categories Array of predicted classes, usually sorted by descending scores (e.g.from high
 * to low probability).
 *
 * @return An instance of `TFLDetection` initialized with the given bounding box and array of categories.
 */
- (instancetype)initWithBoundingBox:(CGRect)boundingBox
                         categories:(NSArray<TFLCategory *> *)categories;

- (instancetype)init NS_UNAVAILABLE;

+ (instancetype)new NS_UNAVAILABLE;

@end

/** Encapsulates results of any object detection task. */
NS_SWIFT_NAME(DetectionResult)
@interface TFLDetectionResult : NSObject

@property(nonatomic, readonly) NSArray<TFLDetection *> *detections;

/**
 * Initializes a new `TFLDetectionResult` with the given array of detections.
 *
 * @param detections Array of detected objects of type TFLDetection.
 *
 * @return An instance of `TFLDetectionResult` initialized with the given array of detections.
 */
- (instancetype)initWithDetections:(NSArray<TFLDetection *> *)detections;

- (instancetype)init NS_UNAVAILABLE;

+ (instancetype)new NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
