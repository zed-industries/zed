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

#import "tensorflow_lite_support/ios/task/audio/core/sources/TFLAudioFormat.h"
#import "tensorflow_lite_support/ios/task/audio/core/sources/TFLFloatBuffer.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * @enum TFLAudioRecordErrorCode
 * This enum specifies error codes for TFLAudioRecord of the TensorFlow Lite Task Library.
 */
typedef NS_ENUM(NSUInteger, TFLAudioRecordErrorCode) {

  /** Unspecified error. */
  TFLAudioRecordErrorCodeUnspecifiedError = 1,

  /** Invalid argument specified. */
  TFLAudioRecordErrorCodeInvalidArgumentError,

  /**
   * Audio processing operation failed.
   * E.g. Format conversion operations by TFLAudioRecord.
   */
  TFLAudioRecordErrorCodeProcessingError,

  /**
   * Audio record permissions were denied by the user.
   */
  TFLAudioRecordErrorCodeRecordPermissionDeniedError,

  /**
   * Audio record permissions cannot be determined. If this error is returned by
   * TFLAudioRecord, the caller has to acquire permissions using AVFoundation.
   */
  TFLAudioRecordErrorCodeRecordPermissionUndeterminedError,

  /**
   * TFLAudioRecord is waiting for new mic input.
   */
  TFLAudioRecordErrorCodeWaitingForNewMicInputError

} NS_SWIFT_NAME(AudioRecordErrorCode);

/** A wrapper class to record the device's microphone continuously. Currently
 * this class only supports recording upto 2 channels. If the number of channels
 * is 2, then the mono microphone input is duplicated to provide dual channel
 * data.
 */
NS_SWIFT_NAME(AudioRecord)
@interface TFLAudioRecord : NSObject

/** Audio format specifying the number of channels and sample rate supported. */
@property(nonatomic, readonly) TFLAudioFormat *audioFormat;

/** Size of the buffer held by `TFLAudioRecord`. It ensures delivery of audio
 * data of length `bufferSize` arrays when you start recording the microphone
 * input.
 */
@property(nonatomic, readonly) NSUInteger bufferSize;

/**
 * Initializes a new `TFLAudioRecord` with the given audio format and buffer size.
 *
 * @param format An audio format of type `TFLAudioFormat`.
 * @param bufferSize Maximum number of elements the internal buffer of
 * `TFLAudioRecord` can hold at any given point of time. The buffer length
 * should be a multiple of `format.channelCount`.
 * @param error An optional error parameter populated if the initialization of `TFLAudioRecord` was
 * not successful.
 *
 * @return An new instance of `TFLAudioRecord` with the given audio format and buffer size. `nil` if
 * there is an error in initializing `TFLAudioRecord`.
 */
- (nullable instancetype)initWithAudioFormat:(TFLAudioFormat *)format
                                  bufferSize:(NSUInteger)bufferSize
                                       error:(NSError **)error;

/**
 * This function starts recording the audio from the microphone if audio record permissions
 * have been granted by the user.
 *
 * @discussion Before calling this function, you must call
 * `-[AVAudioSession requestRecordPermission:]` or `+[AVAudioSession sharedInstance]` to acquire
 * record permissions. If the user has denied permission or the permissions are
 * undetermined, the return value will be false and an appropriate error is
 * populated in the error pointer. The internal buffer of `TFLAudioRecord` of
 * length bufferSize will always have the most recent audio samples acquired
 * from the microphhone if this function returns successfully.  Use:
 * `-[TFLAudioRecord readAtOffset:withSize:error:]` to get the data from the
 * buffer at any instance, if audio recording has started successfully.
 *
 * Use `-[TFLAudioRecord stop]` to stop the audio recording.
 *
 * @param error An optional error parameter populated when the microphone input
 * could not be recorded successfully.
 *
 * @return Boolean value indicating if audio recording started successfully.
 */
- (BOOL)startRecordingWithError:(NSError **)error NS_SWIFT_NAME(startRecording());

/**
 * Stops recording audio from the microphone. All elements in the internal
 * buffer of `TFLAudioRecord` will also be set to zero.
 */
- (void)stop;

/**
 * Returns the `size` number of elements in the internal buffer of
 * `TFLAudioRecord` starting at `offset`, i.e, `buffer[offset:offset+size]`.
 *
 * @param offset Index in the buffer from which elements are to be read.
 * @param size Number of elements to be returned.
 * @param error An optional error parameter populated if the internal buffer could not be read
 * successfully.
 *
 * @return A `TFLFloatBuffer` containing the elements of the internal buffer of `TFLAudioRecord` in
 * the range, `buffer[offset:offset+size]`. Returns `nil` if there is an error in reading the
 * internal buffer.
 */
- (nullable TFLFloatBuffer *)readAtOffset:(NSUInteger)offset
                                 withSize:(NSUInteger)size
                                    error:(NSError **)error;

@end

NS_ASSUME_NONNULL_END
