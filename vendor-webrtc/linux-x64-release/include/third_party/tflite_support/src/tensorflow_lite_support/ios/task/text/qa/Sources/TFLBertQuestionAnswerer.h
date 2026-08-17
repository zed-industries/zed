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
 * Struct to represent the logit and offset of the answer related to context.
 */
struct TFLPos {
  int start;
  int end;
  float logit;
};

/**
 * Class for the Answer to BertQuestionAnswerer.
 */
@interface TFLQAAnswer : NSObject
@property(nonatomic) struct TFLPos pos;
@property(nonatomic) NSString* text;
@end

/**
 * BertQA task API, performs tokenization for models (BERT, Albert, etc.) in
 * preprocess and returns most possible answers.
 *
 * In particular, the branch of BERT models use WordPiece tokenizer, and the
 * branch of Albert models use SentencePiece tokenizer, respectively.
 */
@interface TFLBertQuestionAnswerer : NSObject

/**
 * Creates a BertQuestionAnswerer instance with an albert model or mobilebert
 * model. The API expects a Bert based TFLite model with metadata containing
 * the following information:
 *  input_process_units: for Wordpiece/Sentencepiece Tokenizer
 *  3 input tensors with names "ids", "mask" and "segment_ids"
 *  2 output tensors with names "end_logits" and "start_logits"
 * Sample models:
 *   https://tfhub.dev/tensorflow/lite-model/albert_lite_base/squadv1/1
 *   https://tfhub.dev/tensorflow/lite-model/mobilebert/1/default/1
 * @param modelPath The file path to the tflite model.
 * @return A BertQuestionAnswerer instance.
 */
+ (instancetype)questionAnswererWithModelPath:(NSString *)modelPath
    NS_SWIFT_NAME(questionAnswerer(modelPath:));

/**
 * Answers question based on the context. Could be empty if no answer was found
 * from the given context.
 * 
 * @param context Context the question bases on.
 * @param question Question to ask.
 *
 * @return A list of answers to the question, reversely sorted by the
 * probability of each answer.
 */
- (NSArray<TFLQAAnswer*>*)answerWithContext:(NSString*)context
                                   question:(NSString*)question
    NS_SWIFT_NAME(answer(context:question:));
@end
NS_ASSUME_NONNULL_END
