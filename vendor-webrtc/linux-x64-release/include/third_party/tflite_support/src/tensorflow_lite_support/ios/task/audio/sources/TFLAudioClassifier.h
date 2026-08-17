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

#import "tensorflow_lite_support/ios/task/audio/core/audio_tensor/sources/TFLAudioTensor.h"
#import "tensorflow_lite_support/ios/task/core/sources/TFLBaseOptions.h"
#import "tensorflow_lite_support/ios/task/processor/sources/TFLClassificationOptions.h"
#import "tensorflow_lite_support/ios/task/processor/sources/TFLClassificationResult.h"

NS_ASSUME_NONNULL_BEGIN

/**
 * Options to configure `TFLAudioClassifier`.
 */
NS_SWIFT_NAME(AudioClassifierOptions)
@interface TFLAudioClassifierOptions : NSObject

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
 * Initializes a new `TFLAudioClassifierOptions` with the absolute path to the model file stored
 * locally on the device.
 *
 * @discussion The external model file must be a single standalone TFLite file. It could be packed
 * with TFLite Model Metadata[1] and associated files if exist. Fail to provide the necessary
 * metadata and associated files might result in errors. Check the
 * [documentation](https://www.tensorflow.org/lite/convert/metadata) for each task about the
 * specific requirement.
 *
 * @param modelPath An absolute path to a TensorFlow Lite model file stored locally on the device.
 *
 * @return An instance of `TFLAudioClassifierOptions` initialized to the given model path.
 */
- (instancetype)initWithModelPath:(NSString *)modelPath;

@end

/**
 * A TensorFlow Lite Task Audio Classifiier.
 */
NS_SWIFT_NAME(AudioClassifier)
@interface TFLAudioClassifier : NSObject

/**
 * Creates a new instance of `TFLAudioClassifier` from the given `TFLAudioClassifierOptions`.
 *
 * @param options The options to use for configuring the `TFLAudioClassifier`.
 * @param error An optional error parameter populated when there is an error in initializing
 * the audio classifier.
 *
 * @return A new instance of `TFLAudioClassifier` with the given options. `nil` if there is an error
 * in initializing the audio classifier.
 */
+ (nullable instancetype)audioClassifierWithOptions:(TFLAudioClassifierOptions *)options
                                              error:(NSError **)error
    NS_SWIFT_NAME(classifier(options:));

+ (instancetype)new NS_UNAVAILABLE;

/**
 * Creates a `TFLAudioTensor` instance to store the input audio samples to be classified. The
 * created `TFLAudioTensor` has the same buffer size as the model input tensor and audio format
 * required by the model.
 *
 * @param error An optional error parameter populated when there is an error in creating the audio
 * tensor.
 *
 * @return A `TFLAudioTensor` with the same buffer size as the model input tensor and audio format
 * required by the model, if creation is successful otherwise nil.
 */
- (TFLAudioTensor *)createInputAudioTensor;

/**
 * Creates a `TFLAudioRecord` instance to start recording audio input from the microphone. The
 * returned `TFLAudioRecord` instance instance is initialized with the audio format and twice the
 * input buffer size required by the model.
 *
 * @discussion After creating the `TFLAudioRecord` using this function, the client needs to call
 * -[TFLAudioRecord startRecordingWithError:] to start recording the audio from the microphone.
 *
 * @param error An optional error parameter populated if there is an error in creating the audio
 * record.
 *
 * @return A `TFLAudioRecord` instance with audio format and twice the input buffer size required by
 * the model, if creation is successful or nil in case of failure.
 */
- (nullable TFLAudioRecord *)createAudioRecordWithError:(NSError **)error;

/**
 * Performs classification on an array of audio samples encapsulated by `TFLAudioTensor`.
 *
 * @param audioTensor A `TFLAudioTensor` to be classified.
 *
 * @return A `TFLClassificationResult` with one set of results per audio classifier head. `nil` if
 * there is an error encountered during classification. Please see `TFLClassificationResult` for
 * more details.
 */
- (nullable TFLClassificationResult *)classifyWithAudioTensor:(TFLAudioTensor *)audioTensor
                                                        error:(NSError **)error
    NS_SWIFT_NAME(classify(audioTensor:));

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
