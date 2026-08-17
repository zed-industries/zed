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
 * Options to configure TFLBertNLClassifier.
 */
@interface TFLBertNLClassifierOptions : NSObject

// @deprecated maxSeqLen is now read from the model (i.e. input tensor size)
// automatically.
@property(nonatomic) int maxSeqLen;
@end

/**
 * Classifier API for NLClassification tasks with Bert models, categorizes string into different
 * classes. The API expects a Bert based TFLite model with metadata populated.
 *
 * The metadata should contain the following information:
 *   1 input_process_unit for Wordpiece/Sentencepiece Tokenizer.
 *   3 input tensors with names "ids", "mask" and "segment_ids".
 *   1 output tensor of type float32[1, 2], with a optionally attached label file. If a label
 *     file is attached, the file should be a plain text file with one label per line, the number
 *     of labels should match the number of categories the model outputs.
 */
@interface TFLBertNLClassifier : NSObject

/**
 * Creates TFLBertNLClassifier from a model file.
 *
 * @param modelPath Path to the classification model.
 * @return A TFLBertNLClassifier instance.
 */
+ (instancetype)bertNLClassifierWithModelPath:(NSString *)modelPath
    NS_SWIFT_NAME(bertNLClassifier(modelPath:));

/**
 * Creates TFLBertNLClassifier from a model file.
 *
 * @param modelPath Path to the classification model.
 * @return A TFLBertNLClassifier instance.
 */
+ (instancetype)bertNLClassifierWithModelPath:(NSString *)modelPath
                                      options:(TFLBertNLClassifierOptions *)options
    NS_SWIFT_NAME(bertNLClassifier(modelPath:options:));

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
