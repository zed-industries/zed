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

#import "tensorflow_lite_support/ios/task/core/sources/TFLBaseOptions.h"
#import "tensorflow_lite_support/ios/task/processor/sources/TFLClassificationOptions.h"
#import "tensorflow_lite_support/ios/task/processor/sources/TFLClassificationResult.h"
#import "tensorflow_lite_support/odml/ios/image/apis/GMLImage.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * Options to configure TFLImageClassifier.
 */
NS_SWIFT_NAME(ImageClassifierOptions)
@interface TFLImageClassifierOptions : NSObject

/**
 * Base options that are used for creation of any type of task.
 * @discussion Please see `TFLBaseOptions` for more details.
 */
@property(nonatomic, copy) TFLBaseOptions *baseOptions;

/**
 * Options that configure the display and filtering of results.
 * @discussion Please see `TFLClassificationOptions` for more details.
 */
@property(nonatomic, copy) TFLClassificationOptions *classificationOptions;

/**
 * Initializes a new `TFLImageClassifierOptions` with the absolute path to the model file
 * stored locally on the device, set to the given the model path.
 *
 * @discussion The external model file, must be a single standalone TFLite file. It could be packed
 * with TFLite Model Metadata[1] and associated files if exist. Fail to provide the necessary
 * metadata and associated files might result in errors. Check the [documentation]
 * (https://www.tensorflow.org/lite/convert/metadata) for each task about the specific requirement.
 *
 * @param modelPath An absolute path to a TensorFlow Lite model file stored locally on the device.
 *
 * @return An instance of `TFLImageClassifierOptions` initialized to the given
 * model path.
 */
- (instancetype)initWithModelPath:(NSString *)modelPath;

@end

/**
 * A TensorFlow Lite Task Image Classifiier.
 */
NS_SWIFT_NAME(ImageClassifier)
@interface TFLImageClassifier : NSObject

/**
 * Creates a new instance of `TFLImageClassifier` from the given `TFLImageClassifierOptions`.
 *
 * @param options The options to use for configuring the `TFLImageClassifier`.
 * @param error An optional error parameter populated when there is an error in initializing
 * the image classifier.
 *
 * @return A new instance of `TFLImageClassifier` with the given options. `nil` if there is an error
 * in initializing the image classifier.
 */
+ (nullable instancetype)imageClassifierWithOptions:(TFLImageClassifierOptions *)options
                                              error:(NSError **)error
    NS_SWIFT_NAME(classifier(options:));

+ (instancetype)new NS_UNAVAILABLE;

/**
 * Performs classification on the given GMLImage.
 *
 * @discussion This method currently supports classification of only the following types of images:
 * 1. RGB and RGBA images for `GMLImageSourceTypeImage`.
 * 2. kCVPixelFormatType_32BGRA for `GMLImageSourceTypePixelBuffer` and
 *    `GMLImageSourceTypeSampleBuffer`. If you are using `AVCaptureSession` to setup
 *    camera and get the frames for inference, you must request for this format
 *    from AVCaptureVideoDataOutput. Otherwise your classification
 *    results will be wrong.
 *
 * @param image An image to be classified, represented as a `GMLImage`.
 *
 * @return A TFLClassificationResult with one set of results per image classifier head. `nil` if
 * there is an error encountered during classification. Please see `TFLClassificationResult` for
 * more details.
 */
- (nullable TFLClassificationResult *)classifyWithGMLImage:(GMLImage *)image
                                                     error:(NSError **)error
    NS_SWIFT_NAME(classify(mlImage:));

/**
 * Performs classification on the pixels within the specified region of interest of the given
 * `GMLImage`.
 *
 * @discussion This method currently supports inference on only following type of images:
 * 1. RGB and RGBA images for `GMLImageSourceTypeImage`.
 * 2. kCVPixelFormatType_32BGRA for `GMLImageSourceTypePixelBuffer` and
 *    `GMLImageSourceTypeSampleBuffer`. If you are using `AVCaptureSession` to setup
 *    camera and get the frames for inference, you must request for this format
 *    from AVCaptureVideoDataOutput. Otherwise your classification
 *    results will be wrong.
 *
 * @param image An image to be classified, represented as a `GMLImage`.
 * @param roi A CGRect specifying the region of interest within the given `GMLImage`, on which
 * classification should be performed.
 *
 * @return A TFLClassificationResult with one set of results per image classifier head. `nil` if
 * there is an error encountered during classification.
 */
- (nullable TFLClassificationResult *)classifyWithGMLImage:(GMLImage *)image
                                          regionOfInterest:(CGRect)roi
                                                     error:(NSError **)error
    NS_SWIFT_NAME(classify(mlImage:regionOfInterest:));

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
