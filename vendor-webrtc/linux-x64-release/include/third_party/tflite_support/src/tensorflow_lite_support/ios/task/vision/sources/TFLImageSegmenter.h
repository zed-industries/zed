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
#import "tensorflow_lite_support/ios/task/core/sources/TFLBaseOptions.h"
#import "tensorflow_lite_support/ios/task/processor/sources/TFLSegmentationResult.h"
#import "tensorflow_lite_support/odml/ios/image/apis/GMLImage.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * Specifies the type of the output segmentation mask to be returned as the result
 * of the image segmentation operation. This directs the `TFLImageSegmenter` to
 * choose the type of post-processing to be performed on the raw model results.
 */
typedef NS_ENUM(NSUInteger, TFLOutputType) {
  /** Unspecified output type. */
  TFLOutputTypeUnspecified,

  /**
   * Gives a single output mask where each pixel represents the class which
   * the pixel in the original image was predicted to belong to.
   */
  TFLOutputTypeCategoryMask,

  /**
   * Gives a list of output masks where, for each mask, each pixel represents
   * the prediction confidence, usually in the [0, 1] range.
   */
  TFLOutputTypeConfidenceMasks,

} NS_SWIFT_NAME(OutputType);

/**
 * Options to configure `TFLImageSegmenter`.
 */
NS_SWIFT_NAME(ImageSegmenterOptions)
@interface TFLImageSegmenterOptions : NSObject

/**
 * Base options that is used for creation of any type of task.
 * @discussion Please see `TFLBaseOptions` for more details.
 */
@property(nonatomic, copy) TFLBaseOptions *baseOptions;

/**
 * Specifies the type of output segmentation mask to be returned as a result
 * of the image segmentation operation.
 */
@property(nonatomic) TFLOutputType outputType;

/**
 * Display names local for display names
 */
@property(nonatomic, copy) NSString *displayNamesLocale;

/**
 * Initializes a new `TFLImageSegmenterOptions` with the absolute path to the model file
 * stored locally on the device, set to the given the model path.
 * .
 * @discussion The external model file, must be a single standalone TFLite
 * file. It could be packed with TFLite Model Metadata[1] and associated files
 * if exist. Fail to provide the necessary metadata and associated files might
 * result in errors. Check the [documentation](https://www.tensorflow.org/lite/convert/metadata)
 * for each task about the specific requirement.
 *
 * @param modelPath An absolute path to a TensorFlow Lite model file stored locally on the device.
 *
 * @return An instance of `TFLImageSegmenterOptions` initialized to the given
 * model path.
 */
- (instancetype)initWithModelPath:(NSString *)modelPath;

@end

NS_SWIFT_NAME(ImageSegmenter)
@interface TFLImageSegmenter : NSObject

/**
 * Creates a new instance of `TFLImageSegmenter` from the given `TFLImageSegmenterOptions`.
 *
 * @param options The options to use for configuring the `TFLImageSegmenter`.
 * @param error An optional error parameter populated when there is an error in initializing
 * the image segmenter.
 *
 * @return A new instance of `TFLImageSegmenter` with the given options. `nil` if there is an error
 * in initializing the image segmenter.
 */
+ (nullable instancetype)imageSegmenterWithOptions:(nonnull TFLImageSegmenterOptions *)options
                                             error:(NSError **)error
    NS_SWIFT_NAME(segmenter(options:));

+ (instancetype)new NS_UNAVAILABLE;

/**
 * Performs segmentation on the given GMLImage.
 *
 * @discussion This method currently supports segmentation of only the following types of images:
 * 1. RGB and RGBA images for `GMLImageSourceTypeImage`.
 * 2. kCVPixelFormatType_32BGRA for `GMLImageSourceTypePixelBuffer` and
 *    `GMLImageSourceTypeSampleBuffer`. If you are using `AVCaptureSession` to setup
 *    camera and get the frames for inference, you must request for this format
 *    from AVCaptureVideoDataOutput. Otherwise your segmentation
 *    results will be wrong.
 *
 * @param image An image to be segmented, represented as a `GMLImage`.
 *
 * @return A TFLSegmentationResult that holds the segmentation masks returned by the image
 * segmentation task. `nil` if there is an error encountered during segmentation. Please see
 * `TFLSegmentationResult` for more details.
 */
- (nullable TFLSegmentationResult *)segmentWithGMLImage:(GMLImage *)image
                                                  error:(NSError **)error
    NS_SWIFT_NAME(segment(mlImage:));

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
