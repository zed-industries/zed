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
#include "tensorflow_lite_support/cc/text/tokenizers/tokenizer.h"

using ::tflite::support::text::tokenizer::Tokenizer;

/**
 * Invokes the cpp tokenizer's tokenize function and converts input/output to objc.
 *
 * @param tokenizer The cpp tokenizer pointer.
 * @param input The input string to be tokenized.
 *
 * @return A list of tokens.
 */
NSArray<NSString *> *Tokenize(Tokenizer *tokenizer, NSString *input);

/**
 * Invokes the cpp tokenizer's convertTokensToIds function and converts input/output to objc.
 *
 * @param tokenizer The cpp tokenizer pointer.
 * @param input The tokens to be converted.
 *
 * @return A list of ids.
 */
NSArray<NSNumber *> *ConvertTokensToIds(Tokenizer *tokenizer, NSArray<NSString *> *tokens);
