/* Copyright 2020 The TensorFlow Authors. All Rights Reserved.

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

/**
 * Options to identify input and output tensors of the model.
 */
@interface TFLNLClassifierOptions : NSObject
@property(nonatomic) int inputTensorIndex;
@property(nonatomic) int outputScoreTensorIndex;
@property(nonatomic) int outputLabelTensorIndex;
@property(nonatomic) NSString *inputTensorName;
@property(nonatomic) NSString *outputScoreTensorName;
@property(nonatomic) NSString *outputLabelTensorName;
@end

/**
 * Classifier API for natural language classification tasks, categorizes string into different
 * classes.
 *
 * The API expects a TFLite model with the following input/output tensor:
 *
 *   Input tensor (kTfLiteString)
 *     input of the model, accepts a string.
 *
 *   Output score tensor
 *     (kTfLiteUInt8/kTfLiteInt8/kTfLiteInt16/kTfLiteFloat32/kTfLiteFloat64/kTfLiteBool)
 *     output scores for each class, if type is one of the Int types, dequantize it, if it
 *       is Bool type, convert the values to 0.0 and 1.0 respectively.
 *
 *     can have an optional associated file in metadata for labels, the file should be a
 *       plain text file with one label per line, the number of labels should match the number
 *       of categories the model outputs. Output label tensor: optional (kTfLiteString) -
 *       output classname for each class, should be of the same length with scores. If this
 *       tensor is not present, the API uses score indices as classnames. - will be ignored if
 *       output score tensor already has an associated label file.
 *
 *   Optional Output label tensor (kTfLiteString/kTfLiteInt32)
 *     output classname for each class, should be of the same length with scores. If this
 *       tensor is not present, the API uses score indices as classnames.
 *
 *     will be ignored if output score tensor already has an associated labe file.
 *
 * By default the API tries to find the input/output tensors with default configurations in
 * TFLNLClassifierOptions, with tensor name prioritized over tensor index. The option is
 * configurable for different TFLite models.
 */
@interface TFLNLClassifier : NSObject

/**
 * Creates a TFLNLClassifier instance from TFLNLClassifierOptions.
 *
 * @param modelPath The file path to the tflite mdoel.
 * @param options The TFLNLClassifierOptions to configure the model.
 *
 * @return A TFLNLClassifier instance.
 */
+ (instancetype)nlClassifierWithModelPath:(NSString *)modelPath
                                  options:(TFLNLClassifierOptions *)options
    NS_SWIFT_NAME(nlClassifier(modelPath:options:));

/**
 * Performs classification on a NSString input, returns <NSString *, NSNumber *>
 * for categories and socres.
 *
 * @param text input text to the model.
 * @return A NSDictionary of categorization results.
 */
- (NSDictionary<NSString *, NSNumber *> *)classifyWithText:(NSString *)text
    NS_SWIFT_NAME(classify(text:));
@end
NS_ASSUME_NONNULL_END
