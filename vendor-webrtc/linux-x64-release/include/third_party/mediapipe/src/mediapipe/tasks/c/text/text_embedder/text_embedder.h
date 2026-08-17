/* Copyright 2023 The MediaPipe Authors.

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

#ifndef MEDIAPIPE_TASKS_C_TEXT_TEXT_EMBEDDER_TEXT_EMBEDDER_H_
#define MEDIAPIPE_TASKS_C_TEXT_TEXT_EMBEDDER_TEXT_EMBEDDER_H_

#include "mediapipe/tasks/c/components/containers/embedding_result.h"
#include "mediapipe/tasks/c/components/processors/embedder_options.h"
#include "mediapipe/tasks/c/core/base_options.h"

#ifndef MP_EXPORT
#define MP_EXPORT __attribute__((visibility("default")))
#endif  // MP_EXPORT

#ifdef __cplusplus
extern "C" {
#endif

typedef struct EmbeddingResult TextEmbedderResult;

// The options for configuring a MediaPipe text embedder task.
struct TextEmbedderOptions {
  // Base options for configuring MediaPipe Tasks, such as specifying the model
  // file with metadata, accelerator options, op resolver, etc.
  struct BaseOptions base_options;

  // Options for configuring the embedder behavior, such as l2_normalize
  // and quantize.
  struct EmbedderOptions embedder_options;
};

// Creates a TextEmbedder from the provided `options`.
// Returns a pointer to the text embedder on success.
// If an error occurs, returns `nullptr` and sets the error parameter to an
// an error message (if `error_msg` is not `nullptr`). You must free the memory
// allocated for the error message.
MP_EXPORT void* text_embedder_create(struct TextEmbedderOptions* options,
                                     char** error_msg);

// Performs embedding extraction on the input `text`. Returns `0` on success.
// If an error occurs, returns an error code and sets the error parameter to an
// an error message (if `error_msg` is not `nullptr`). You must free the memory
// allocated for the error message.
MP_EXPORT int text_embedder_embed(void* embedder, const char* utf8_str,
                                  TextEmbedderResult* result, char** error_msg);

// Frees the memory allocated inside a TextEmbedderResult result. Does not
// free the result pointer itself.
MP_EXPORT void text_embedder_close_result(TextEmbedderResult* result);

// Shuts down the TextEmbedder when all the work is done. Frees all memory.
// If an error occurs, returns an error code and sets the error parameter to an
// an error message (if `error_msg` is not `nullptr`). You must free the memory
// allocated for the error message.
MP_EXPORT int text_embedder_close(void* embedder, char** error_msg);

// Utility function to compute cosine similarity [1] between two embeddings.
// May return an InvalidArgumentError if e.g. the embeddings are of different
// types (quantized vs. float), have different sizes, or have a an L2-norm of
// 0.
//
// [1]: https://en.wikipedia.org/wiki/Cosine_similarity
MP_EXPORT int text_embedder_cosine_similarity(const struct Embedding* u,
                                              const struct Embedding* v,
                                              double* similarity,
                                              char** error_msg);

#ifdef __cplusplus
}  // extern C
#endif

#endif  // MEDIAPIPE_TASKS_C_TEXT_TEXT_EMBEDDER_TEXT_EMBEDDER_H_
