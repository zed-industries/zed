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

#include "tensorflow_lite_support/c/task/vision/core/frame_buffer.h"

#import "tensorflow_lite_support/odml/ios/image/apis/GMLImage.h"

NS_ASSUME_NONNULL_BEGIN

/** Helper utility for performing operations on GMLImage specific to the
 * TF Lite Task Vision library
 */
@interface GMLImage (Utils)

/** Bitmap size of the image. */
@property(nonatomic, readonly) CGSize bitmapSize;

/**
 * Returns the underlying uint8 pixel buffer of a GMLImage.
 *
 * @param error Pointer to the memory location where errors if any should be
 * saved. If @c NULL, no error will be saved.
 *
 * @return The underlying pixel buffer of gmlImage or nil in case of errors.
 */
- (nullable uint8_t *)bufferWithError:(NSError *_Nullable *)error;

/**
 * Creates and returns a TfLiteFrameBuffer from a GMLImage. TfLiteFrameBuffer is
 * used by the TFLite Task Vision C library to hold the backing buffer of any
 * image. Image inputs to the TFLite Task Vision C library is of type
 * TfLiteFrameBuffer.
 *
 * @param error Pointer to the memory location where errors if any should be
 * saved. If `nil`, no error will be saved.
 *
 * @return The TfLiteFrameBuffer created from the gmlImage which can be used
 * with the TF Lite Task Vision C library.
 */
- (nullable TfLiteFrameBuffer *)cFrameBufferWithError:(NSError *_Nullable *)error;

/**
 * Gets grayscale pixel buffer from GMLImage if source type is
 * GMLImageSourceTypeImage.
 *
 * @warning Currently method only returns gray scale pixel buffer if source type
 * is GMLImageSourceTypeImage since extracting gray scale pixel buffer from
 * other source types is not a necessity for the current testing framework.
 *
 * @return The CVPixelBufferRef for the newly created gray scale pixel buffer.
 */
- (CVPixelBufferRef)grayScalePixelBuffer;

/**
 * Loads an image from a file in an app bundle into a GMLImage object.
 *
 * @param classObject The specified class associated with the bundle containing
 * the file to be loaded.
 * @param name Name of the image file.
 * @param type Extenstion of the image file.
 *
 * @return The GMLImage object contains the loaded image. This method returns
 * nil if it cannot load the image.
 */
+ (nullable GMLImage *)imageFromBundleWithClass:(Class)classObject
                                       fileName:(NSString *)name
                                         ofType:(NSString *)type
    NS_SWIFT_NAME(imageFromBundle(class:filename:type:));

@end

NS_ASSUME_NONNULL_END
