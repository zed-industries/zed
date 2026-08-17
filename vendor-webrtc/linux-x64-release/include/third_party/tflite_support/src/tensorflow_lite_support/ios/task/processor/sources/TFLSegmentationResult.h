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

/** Holds a confidence mask belonging to a single class and its meta data. */
NS_SWIFT_NAME(ConfidenceMask)
@interface TFLConfidenceMask : NSObject

/**
 * Confidence masks of size `width` x `height` for any one class.
 */
@property(nonatomic, readonly) float *mask;

/**
 * The width of the mask. This is an intrinsic parameter of the model being
 * used, and does not depend on the input image dimensions.
 */
@property(nonatomic, readonly) NSInteger width;

/**
 * The height of the mask. This is an intrinsic parameter of the model being
 * used, and does not depend on the input image dimensions.
 */
@property(nonatomic, readonly) NSInteger height;

/**
 * Initializes a confidence mask.
 */
- (instancetype)initWithWidth:(NSInteger)width
                       height:(NSInteger)height
                         mask:(float * _Nullable)mask;

- (instancetype)init NS_UNAVAILABLE;

+ (instancetype)new NS_UNAVAILABLE;

@end

/** Holds category mask and its metadata. */
NS_SWIFT_NAME(CategoryMask)
@interface TFLCategoryMask : NSObject

/**
 * Flattened 2D-array of size `width` x `height`, in row major order.
 * The value of each pixel in this mask represents the class to which the
 * pixel belongs.
 */
@property(nonatomic, readonly) UInt8 *mask;

/**
 * The width of the mask. This is an intrinsic parameter of the model being
 * used, and does not depend on the input image dimensions.
 */
@property(nonatomic, readonly) NSInteger width;

/**
 * The height of the mask. This is an intrinsic parameter of the model being
 * used, and does not depend on the input image dimensions.
 */
@property(nonatomic, readonly) NSInteger height;

+ (instancetype)new NS_UNAVAILABLE;

/**
 * Initializes a new `TFLCategoryMask` mask.
 *
 * @param width Width of the mask.
 * @param height Height of the mask.
 * @param mask Flattened 2D-array of size `width` x `height`, in row major order.
 * The value of each pixel in this mask represents the class to which the
 * pixel belongs.
 *
 * @return An instance of TFLCategoryMask initialized to the specified values.
 */
- (instancetype)initWithWidth:(NSInteger)width
                       height:(NSInteger)height
                         mask:(UInt8 * _Nullable)mask;

- (instancetype)init NS_UNAVAILABLE;

@end

/** Holds a label associated with an RGB color, for display purposes. */
NS_SWIFT_NAME(ColoredLabel)
@interface TFLColoredLabel : NSObject

/** The RGB color components for the label, in the [0, 255] range. */
@property(nonatomic, readonly) NSUInteger r;
@property(nonatomic, readonly) NSUInteger g;
@property(nonatomic, readonly) NSUInteger b;

/**
 * The class name, as provided in the label map packed in the TFLite Model
 * Metadata.
 */
@property(nonatomic, readonly) NSString *label;

/**
 * The display name, as provided in the label map (if available) packed in
 * the TFLite Model Metadata. See displayNamesLocale in
 * TFLClassificationOptions.
 */
@property(nonatomic, readonly) NSString *displayName;

/**
 * Initializes a new `TFLColoredLabel` with red, gree, blue color components, label and display name.
 *
 * @param r Red component of the RGB color components.
 * @param g Green component of the RGB color components.
 * @param b Blue component of the RGB color components.
 * @param label Class name.
 * @param displayName Display name.
 *
 * @return An instance of TFLColoredLabel initialized with red, gree, blue color components, label and display name.
 */
- (instancetype)initWithRed:(NSUInteger)r
                      green:(NSUInteger)g
                       blue:(NSUInteger)b
                      label:(NSString *)label
                displayName:(NSString *)displayName;

- (instancetype)init NS_UNAVAILABLE;

+ (instancetype)new NS_UNAVAILABLE;

@end

/** Encapsulates a resulting segmentation mask and associated metadata. */
NS_SWIFT_NAME(Segmentation)
@interface TFLSegmentation : NSObject

/**
 * Array of confidence masks where each element is a confidence mask of size
 * `width` x `height`, one for each of the supported classes.
 * The value of each pixel in these masks represents the confidence score for
 * this particular class.
 * This property is mutually exclusive with `categoryMask`.
 */
@property(nonatomic, nullable, readonly) NSArray<TFLConfidenceMask *> *confidenceMasks;

/**
 * Holds the category mask.
 * The value of each pixel in this mask represents the class to which the
 * pixel belongs.
 * This property is mutually exclusive with `confidenceMasks`.
 */
@property(nonatomic, nullable, readonly) TFLCategoryMask *categoryMask;

/**
 * The list of colored labels for all the supported categories (classes).
 * Depending on which is present, this list is in 1:1 correspondence with:
 * `category_mask` pixel values, i.e. a pixel with value `i` is associated with
 * `colored_labels[i]`, `confidence_masks` indices, i.e. `confidence_masks[i]`
 * is associated with `colored_labels[i]`.
 */
@property(nonatomic, readonly) NSArray<TFLColoredLabel *> *coloredLabels;

+ (instancetype)new NS_UNAVAILABLE;

/**
 * Initializes a new `TFLSegmentation` with an array of confidence masks and an array of colored labels.
 * `categoryMask` is initialized to `nil` as it is mutually exclusive with `confidenceMasks`.
 *
 * @param confidenceMasks An array of `TFLConfidenceMask` objects.
 * @param coloredLabels An array of `TFLColoredLabel` objects.
 *
 * @return An instance of `TFLSegmentation` initialized with an array of confidence masks and an array of colored labels.
 */
- (instancetype)initWithConfidenceMasks:(NSArray<TFLConfidenceMask *> *)confidenceMasks
                          coloredLabels:(NSArray<TFLColoredLabel *> *)coloredLabels;

/**
 * Initializes a new `TFLSegmentation` with a category mask and array of colored labels.
 * `confidenceMasks` is initialized to `nil` as it is mutually exclusive with `categoryMask`.
 *
 * @param categoryMask A `TFLCategoryMask` object.
 * @param coloredLabels An array of `TFLColoredLabel` objects.
 *
 * @return An instance of `TFLSegmentation` initialized with a category mask and array of colored labels.
 */
- (instancetype)initWithCategoryMask:(TFLCategoryMask *)categoryMask
                       coloredLabels:(NSArray<TFLColoredLabel *> *)coloredLabels;

- (instancetype)init NS_UNAVAILABLE;

@end

/** Encapsulates results of any image segmentation task. */
NS_SWIFT_NAME(SegmentationResult)
@interface TFLSegmentationResult : NSObject

/** Array of segmentations returned after inference by model.
 * Note that at the time, this array is expected to have a single
 * `TfLiteSegmentation`; the field is made an array for later extension to
 * e.g. instance segmentation models, which may return one segmentation per
 * object.
 */
@property(nonatomic, readonly) NSArray<TFLSegmentation *> *segmentations;

+ (instancetype)new NS_UNAVAILABLE;

/**
 * Initializes a new `TFLSegmentationResult` with an array of segmentations.
 *
 * @param segmentations An array of `TFLSegmentation` objects.
 *
 * @return An instance of `TFLSegmentationResult` initialized with an array of segmentations.
 */
- (instancetype)initWithSegmentations:(NSArray<TFLSegmentation *> *)segmentations;

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
