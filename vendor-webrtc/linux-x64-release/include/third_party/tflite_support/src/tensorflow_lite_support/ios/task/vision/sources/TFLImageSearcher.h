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
#import "tensorflow_lite_support/ios/task/processor/sources/TFLEmbeddingOptions.h"
#import "tensorflow_lite_support/ios/task/processor/sources/TFLSearchOptions.h"
#import "tensorflow_lite_support/ios/task/processor/sources/TFLSearchResult.h"
#import "tensorflow_lite_support/odml/ios/image/apis/GMLImage.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * Options to configure TFLImageSearcher.
 */
NS_SWIFT_NAME(ImageSearcherOptions)
@interface TFLImageSearcherOptions : NSObject

/**
 * Base options for configuring the ImageSearcher. This specifies the TFLite
 * model to use for embedding extraction, as well as hardware acceleration
 * options to use as inference time.
 */
@property(nonatomic, copy) TFLBaseOptions *baseOptions;

/**
 * Options controlling the behavior of the embedding model specified in the
 * base options.
 */
@property(nonatomic, copy) TFLEmbeddingOptions *embeddingOptions;

/**
 * Options specifying the index to search into and controlling the search behavior.
 */
@property(nonatomic, copy) TFLSearchOptions *searchOptions;

/**
 * Initializes a new `TFLImageSearcherOptions` with the absolute path to the model file
 * stored locally on the device, set to the given the model path.
 *
 * @discussion The external model file must be a single standalone TFLite file. It could be packed
 * with TFLite Model Metadata[1] and associated files if they exist. Failure to provide the
 * necessary metadata and associated files might result in errors. Check the [documentation]
 * (https://www.tensorflow.org/lite/convert/metadata) for each task about the specific requirement.
 *
 * @param modelPath An absolute path to a TensorFlow Lite model file stored locally on the device.
 *
 * @return An instance of `TFLImageSearcherOptions` initialized to the given model path.
 */
- (instancetype)initWithModelPath:(NSString *)modelPath;

@end

/**
 * A TensorFlow Lite Task Image Searcher.
 */
NS_SWIFT_NAME(ImageSearcher)
@interface TFLImageSearcher : NSObject

/**
 * Creates a new instance of `TFLImageSearcher` from the given `TFLImageSearcherOptions`.
 *
 * @param options The options to use for configuring the `TFLImageSearcher`.
 * @param error An optional error parameter populated when there is an error in initializing
 * the image searcher.
 *
 * @return A new instance of `ImageSearcher` with the given options. `nil` if there is an error
 * in initializing the image searcher.
 */
+ (nullable instancetype)imageSearcherWithOptions:(TFLImageSearcherOptions *)options
                                            error:(NSError **)error
    NS_SWIFT_NAME(searcher(options:));

+ (instancetype)new NS_UNAVAILABLE;

/**
 * Performs embedding extraction on the given GMLImage, followed by nearest-neighbor search in the
 * index.
 *
 * @discussion This method currently supports searching on only the following types of images:
 * 1. RGB and RGBA images for `GMLImageSourceTypeImage`.
 * 2. kCVPixelFormatType_32BGRA for `GMLImageSourceTypePixelBuffer` and
 *    `GMLImageSourceTypeSampleBuffer`. If you are using `AVCaptureSession` to setup
 *    camera and get the frames for inference, you must request for this format
 *    from AVCaptureVideoDataOutput. Otherwise your inference results will be wrong.
 *
 * @param image An image on which to perform embedding extraction, followed by a nearest-neighbor
 * search in the index, represented as a `GMLImage`.

 * @return A `TFLSearchResult`. `nil` if there is an error encountered during embedding extraction
 * and nearest neighbor search. Please see `TFLSearchResult` for more details.
 */
- (nullable TFLSearchResult *)searchWithGMLImage:(GMLImage *)image
                                           error:(NSError **)error NS_SWIFT_NAME(search(mlImage:));

/**
 * Performs embedding extraction on the given GMLImage, followed by nearest-neighbor search in the
 * index on the pixels within the specified region of interest of the given
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
 * @param image An image on which to perform embedding extraction, followed by a nearest-neighbor
 * search in the index, represented as a `GMLImage`.
 *
 * @param roi A CGRect specifying the region of interest within the given `GMLImage`.
 *
 * @return A TFLClassificationResult with one set of results per image classifier head. `nil` if
 * there is an error encountered during classification.
 */
- (nullable TFLSearchResult *)searchWithGMLImage:(GMLImage *)image
                                regionOfInterest:(CGRect)roi
                                           error:(NSError **)error
    NS_SWIFT_NAME(search(mlImage:regionOfInterest:));

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
