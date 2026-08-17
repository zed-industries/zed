// Copyright 2024 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_C_LLM_INFERENCE_ENGINE_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_C_LLM_INFERENCE_ENGINE_H_

#ifdef __cplusplus
#include <cstddef>
#else
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#endif

#ifdef __EMSCRIPTEN__
#include <functional>
#endif  // __EMSCRIPTEN__

#ifndef ODML_EXPORT
#define ODML_EXPORT __attribute__((visibility("default")))
#endif  // ODML_EXPORT

#ifdef __cplusplus
extern "C" {
#endif

typedef void LlmInferenceEngine_Engine;

typedef void LlmInferenceEngine_Session;

// LlmActivationDataType defines the activation data type for the model.
typedef enum {
  // Use Default activation data type mentioned in the model metadata file.
  kLlmActivationDataTypeDefault = 0,

  // Use Float32 activation data type.
  kLlmActivationDataTypeFloat32 = 1,

  // Use Float16 activation data type.
  kLlmActivationDataTypeFloat16 = 2,

  // Use Int16 activation data type.
  kLlmActivationDataTypeInt16 = 3,

  // Use Int8 activation data type.
  kLlmActivationDataTypeInt8 = 4,
} LlmActivationDataType;

// Specify the LiteRT backend to use for the LLM model. If not specified, the
// default backend will be used.
typedef enum {
  // Use default backend extracted from the model.
  kLlmPreferredBackendDefault = 0,

  // Use GPU backend.
  kLlmPreferredBackendGpu = 1,
} LlmPreferredBackend;

// LlmSessionConfig configures how to execute the model.
typedef struct {
  // Path to the model artifact.
  const char* model_path;

#ifdef __EMSCRIPTEN__
  // Function to read model file.
  // The function returns a pointer to heap memory that contains the model file
  // contents started from `offset` with `size`.
  // Since the model file is hosted on JavaScript layer and this function copies
  // the data to the heap memory, the `mode` instructs how the source model file
  // data should be mainuplated:
  //   0: Data will be kept in memory after read.
  //   1: Data will not be accessed again and can be discarded.
  //   2: All data has been used and can be discarded.
  using ReadDataFn =
      std::function<void*(uint64_t offset, uint64_t size, int mode)>;
  ReadDataFn* read_model_fn;
#endif  // __EMSCRIPTEN__

  // Path to the vision encoder to use for vision modality. Optional.
  const char* vision_encoder_path;

  // Path to the vision adapter  to use for vision modality. Optional.
  const char* vision_adapter_path;

  // Directory path for storing model related tokenizer and cache weights. the
  // user is responsible for providing the directory that can be writable by the
  // program.
  const char* cache_dir;

  // Maximum number of tokens for input and output.
  size_t max_num_tokens;

  // Number of decode steps per sync. Used by GPU only. The default value is 3.
  size_t num_decode_steps_per_sync;

  // Sequence batch size for encoding. Used by GPU only. Number of input tokens
  // to process at a time for batch processing. Setting this value to 1 means
  // both the encoding and decoding share the same graph of sequence length
  // of 1. Setting this value to 0 means the batch size will be optimized
  // programmatically.
  size_t sequence_batch_size;

  // Number of supported lora ranks for the base model. Used by GPU only.
  size_t number_of_supported_lora_ranks;

  // The supported lora ranks for the base model. Used by GPU only.
  size_t* supported_lora_ranks;

  // Maximum top k, which is the max Top-K value supported for all
  // sessions created with the engine, used by GPU only. If a session with Top-K
  // value larger than this is being asked to be created, it will be
  // rejected(throw error). If not provided, the max top k will be 1, which
  // means only greedy decoding is supported for any sessions created with this
  // engine.
  size_t max_top_k;

  // Optional setting for specific activation data type.
  LlmActivationDataType llm_activation_data_type;

  // Optional setting for the number of draft tokens to generate when using
  // speculative decoding. Setting to 0 will disable speculative decoding.
  size_t num_draft_tokens;

  // If true, waits for weights to finish uploading when initializing. Otherwise
  // initialization may finish before weights have finished uploading which
  // might push some of the weight upload time into input processing.
  bool wait_for_weight_uploads;

  // Whether the submodel should be used if available.
  bool use_submodel;

  // Optional setting to prefer specific backend instead.
  LlmPreferredBackend preferred_backend;
} LlmModelSettings;

// LlmSessionConfig configures how to execute the model.
typedef struct {
  // Top K number of tokens to be sampled from for each decoding step.
  size_t topk;

  // Maximum cumulative probability over the tokens to sample from in each
  // decoding step for top-p / nucleus sampling.
  float topp;

  // Randomness when decoding the next token, 0.0f means greedy decoding.
  float temperature;

  // random seed, for reproducible sampling.
  size_t random_seed;

  // Path to the LoRA tflite flatbuffer file. Optional.
  // This is only compatible with GPU handwritten models and converter based
  // models.
  const char* lora_path;

  // Whether to configure the graph to include the token cost calculator,
  // which allows users to only compute the cost of a prompt.
  bool include_token_cost_calculator;

  // Whether to configure the graph to include the vision modality.
  bool enable_vision_modality;
} LlmSessionConfig;

// LlmResponseContext is the return type for
// LlmInferenceEngine_Session_PredictSync.
typedef struct {
  // An array of string. The size of the array depends on the number of
  // responses.
  char** response_array;

  // Number of responses.
  int response_count;

  // Done all outputs for this session.
  bool done;
} LlmResponseContext;

// Frees all context within the LlmResponseContext.
ODML_EXPORT void LlmInferenceEngine_CloseResponseContext(
    LlmResponseContext* response_context);

// Create a LlmInferenceEngine session for executing a query.
ODML_EXPORT int LlmInferenceEngine_CreateEngine(
    const LlmModelSettings* model_settings,
    LlmInferenceEngine_Engine** engine_out, char** error_msg);

// Free the engine, will release ownership of resource held by the engine.
// Resource might be freed if no sessions are referencing to it.
ODML_EXPORT void LlmInferenceEngine_Engine_Delete(
    LlmInferenceEngine_Engine* engine);

// Create a LlmInferenceEngine session for executing a query.
ODML_EXPORT int LlmInferenceEngine_CreateSession(
    LlmInferenceEngine_Engine* engine, const LlmSessionConfig* session_config,
    LlmInferenceEngine_Session** session_out, char** error_msg);

// Free the session, will wait until graph is done executing.
ODML_EXPORT void LlmInferenceEngine_Session_Delete(
    LlmInferenceEngine_Session* session);

// Add query chunk to the session. This can be called multiple times to add
// multiple query chunks before calling `PredictSync` or `PredictAsync`. The
// query chunks will be processed in the order they are added, similar to a
// concatenated prompt, but able to be processed in chunks.
ODML_EXPORT int LlmInferenceEngine_Session_AddQueryChunk(
    LlmInferenceEngine_Session* session, const char* input, char** error_msg);

// Adds an SKBitmap to the session.
ODML_EXPORT int LlmInferenceEngine_Session_AddImage(
    LlmInferenceEngine_Session* session, const void* sk_bitmap,
    char** error_msg);

// Return the generated output based on the previously added query chunks in
// sync mode.
ODML_EXPORT int LlmInferenceEngine_Session_PredictSync(
    LlmInferenceEngine_Session* session, LlmResponseContext* response_context,
    char** error_msg);

// Run callback function in async mode.
// The callback will be invoked multiple times until `response_context.done`
// is `true`. You need to invoke `LlmInferenceEngine_CloseResponseContext` after
// each invocation to free memory.
// The callback context can be a pointer to any user defined data structure as
// it is passed to the callback unmodified.
ODML_EXPORT int LlmInferenceEngine_Session_PredictAsync(
    LlmInferenceEngine_Session* session, void* callback_context,
    char** error_msg,
    void (*callback)(void* callback_context,
                     LlmResponseContext* response_context));

// Clone the provided session.
ODML_EXPORT int LlmInferenceEngine_Session_Clone(
    LlmInferenceEngine_Session* session,
    LlmInferenceEngine_Session** cloned_session, char** error_msg);

// Tokenizes an input prompt using a pre-existing processor and returns its
// length in tokens. Returns -1 if tokenization fails.
ODML_EXPORT int LlmInferenceEngine_Session_SizeInTokens(
    LlmInferenceEngine_Session* session, const char* input, char** error_msg);

#ifdef __cplusplus
}  // extern C
#endif

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_C_LLM_INFERENCE_ENGINE_H_
