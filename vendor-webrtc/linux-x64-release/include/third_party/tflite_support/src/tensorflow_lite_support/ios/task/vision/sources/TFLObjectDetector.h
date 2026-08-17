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
#import "tensorflow_lite_support/ios/task/processor/sources/TFLClassificationOptions.h"
#import "tensorflow_lite_support/ios/task/processor/sources/TFLDetectionResult.h"
#import "tensorflow_lite_support/odml/ios/image/apis/GMLImage.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * Options to configure `TFLObjectDetector`.
 */
NS_SWIFT_NAME(ObjectDetectorOptions)
@interface TFLObjectDetectorOptions : NSObject

/**
 * Base options that is used for creation of any type of task.
 * @discussion Please see `TFLBaseOptions` for more details.
 */
@property(nonatomic, copy) TFLBaseOptions *baseOptions;

/**
 * Options that configure the display and filtering of results.
 * @discussion Please see `TFLClassificationOptions` for more details.
 */
@property(nonatomic, copy) TFLClassificationOptions *classificationOptions;

/**
 * Initializes a new `TFLObjectDetectorOptions` with the absolute path to the model file
 * stored locally on the device, set to the given the model path.
 *
 * @discussion The external model file, must be a single standalone TFLite file. It could be packed
 * with TFLite Model Metadata[1] and associated files if exist. Fail to provide the necessary
 * metadata and associated files might result in errors. Check the [documentation]
 * (https://www.tensorflow.org/lite/convert/metadata) for each task about the specific requirement.
 *
 * @param modelPath An absolute path to a TensorFlow Lite model file stored locally on the device.
 * @return An instance of `TFLObjectDetectorOptions` initialized to the given
 * model path.
 */
- (instancetype)initWithModelPath:(NSString *)modelPath;

@end

NS_SWIFT_NAME(ObjectDetector)
@interface TFLObjectDetector : NSObject

/**
 * Creates a new instance of `TFLObjectDetector` from the given `TFLObjectDetectorOptions`.
 *
 * @param options The options to use for configuring the `TFLObjectDetector`.
 * @param error An optional error parameter populated when there is an error in initializing
 * the object detector.
 *
 * @return A new instance of `TFLObjectDetector` with the given options. `nil` if there is an error
 * in initializing the object detector.
 */
+ (nullable instancetype)objectDetectorWithOptions:(TFLObjectDetectorOptions *)options
                                             error:(NSError **)error
    NS_SWIFT_NAME(detector(options:));

+ (instancetype)new NS_UNAVAILABLE;

/**
 * Performs object detection on the given GMLImage.
 * @discussion This method currently supports object detection on only the following types of
 * images:
 * 1. RGB and RGBA images for `GMLImageSourceTypeImage`.
 * 2. `kCVPixelFormatType_32BGRA` for `GMLImageSourceTypePixelBuffer` and
 *    `GMLImageSourceTypeSampleBuffer`. If you are using `AVCaptureSession` to setup
 *    camera and get the frames for inference, you must request for this format
 *    from AVCaptureVideoDataOutput. Otherwise your object detection
 *    results will be wrong.
 *
 * @param image An image on which object detection is to be performed, represented as a `GMLImage`.
 *
 * @return A `TFLDetectionResult` holding an array of TFLDetection objects, each having a bounding
 * box specifying the region the were detected in and an array of predicted classes. Please see
 * `TFLDetectionResult` for more details.
 */
- (nullable TFLDetectionResult *)detectWithGMLImage:(GMLImage *)image
                                              error:(NSError **)error
    NS_SWIFT_NAME(detect(mlImage:));

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
