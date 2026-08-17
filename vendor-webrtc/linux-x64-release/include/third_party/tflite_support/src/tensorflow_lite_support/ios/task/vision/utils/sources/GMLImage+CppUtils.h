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

#include "tensorflow_lite_support/cc/task/vision/core/frame_buffer.h"

#import "tensorflow_lite_support/odml/ios/image/apis/GMLImage.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * Helper utility for converting GMLImage to C++ FrameBuffer accepted by the
 * TF Lite Task Vision C++ library.
 */
@interface GMLImage (CppUtils)

/**
 * Creates and returns a C++ FrameBuffer from a GMLImage.
 * tflite::task::vision::FrameBuffer is used by the TFLite Task Vision C++
 * library to hold the backing buffer of any image.
 *
 * @param buffer Pointer to the memory location where underlying pixel buffer
 * of the image should be saved.
 *
 * @param error Pointer to the memory location where errors if any should be
 * saved. If @c NULL, no error will be saved.
 *
 * @return The FrameBuffer created from the gmlImage which can be used with the
 * TF Lite Task Vision C++ library. @c NULL in case of an error.
 */
- (std::unique_ptr<tflite::task::vision::FrameBuffer>)
    cppFrameBufferWithUnderlyingBuffer:(uint8_t **)buffer
                                 error:(NSError *_Nullable *)error;

@end

NS_ASSUME_NONNULL_END
