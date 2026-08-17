// Copyright 2022 The TensorFlow Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#import <Foundation/Foundation.h>
#import "tensorflow_lite_support/ios/task/audio/core/audio_record/sources/TFLAudioRecord.h"
#import "tensorflow_lite_support/ios/task/audio/core/sources/TFLFloatBuffer.h"

NS_ASSUME_NONNULL_BEGIN

/** A wrapper class to store input audio used in on-device machine learning. */
NS_SWIFT_NAME(AudioTensor)
@interface TFLAudioTensor : NSObject

/** Audio format specifying the number of channels and sample rate supported. */
@property(nonatomic, readonly) TFLAudioFormat *audioFormat;

/**
 * A copy of all the internal buffer elements in order with the most recent elements
 * appearing at the end of `buffer.data`.
 */
@property(nonatomic, readonly) TFLFloatBuffer *buffer;

/** Capacity of the `TFLAudioTensor` buffer in number of elements. */
@property(nonatomic, readonly) NSUInteger bufferSize;

/**
 * Initializes a new `TFLAudioTensor` with a given `TFLAudioFormat` and sample count.
 *
 * @discussion The `TFLAudioTensor` stores data in a ring buffer of size sampleCount *
 * TFLAudioFormat.channelCount.
 *
 * @param format An audio format of type `TFLAudioFormat`.
 * @param sampleCount The number of samples `TFLAudioTensor` can store at any given
 * time. The `sampleCount` provided will be used to calculate the buffer size of `TFLAudioTensor`
 * (with `bufferSize = format.channelCount * sampleCount`.
 *
 * @return A new instance of TFLAudioTensor with the given audio format and sample count.
 */
- (instancetype)initWithAudioFormat:(TFLAudioFormat *)format
                        sampleCount:(NSUInteger)sampleCount NS_DESIGNATED_INITIALIZER;

- (instancetype)init NS_UNAVAILABLE;

/**
 * Convenience method to load the elements currently in the internal buffer of `TFLAudioRecord` into
 * `TFLAudioTensor`.
 *
 * @discussion You must ensure that the audio formats of `TFLAudioRecord` and the current
 * `TFLAudioTensor` match.
 * New data from the input buffer is appended to the end of the buffer by shifting out
 * any old data from the beginning of the buffer if needed to make space. If the size of the new
 * data to be copied is more than the capacity of the buffer, only the most recent data
 * of the `TFLAudioTensor`'s buffer size will be copied from the input buffer.
 *
 * @param audioRecord  An object of `TFLAudioRecord`.
 * @param error An optional error parameter populated with the reason for failure, if the internal
 * buffer of `TFLAudioRecord` could not be loaded into the `TFLAudioTensor`.
 *
 * @return A boolean indicating if the load operation succeded.
 */
- (BOOL)loadAudioRecord:(TFLAudioRecord *)audioRecord
              withError:(NSError **)error NS_SWIFT_NAME(load(audioRecord:));

/**
 * This function loads the internal buffer of `TFLAudioTensor` with the provided buffer.
 *
 * @discussion New data from the input buffer is appended to the end of the buffer by shifting out
 * any old data from the beginning of the buffer if needed to make space. If the size of the new
 * data to be copied is more than the capacity of the buffer, only the most recent data
 * of the `TFLAudioTensor`'s buffer size will be copied from the input buffer.
 *
 * @param sourceBuffer  A buffer of type `TFLFloatBuffer`. For multi-channel input, the array must
 * be interleaved.
 * @param offset Starting index in the source buffer from which elements should be copied.
 * @param size The number of elements to be copied.
 * @param error An optional error parameter populated with the reason for failure, if the internal
 * buffer could not be loaded with the source buffer.
 *
 * @return A boolean indicating if the load operation succeded.
 */
- (BOOL)loadBuffer:(TFLFloatBuffer *)buffer
            offset:(NSUInteger)offset
              size:(NSUInteger)size
             error:(NSError **)error NS_SWIFT_NAME(load(buffer:offset:size:));

@end

NS_ASSUME_NONNULL_END
