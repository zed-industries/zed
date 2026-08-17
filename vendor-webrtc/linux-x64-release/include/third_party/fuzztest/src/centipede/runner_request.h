// Copyright 2022 The Centipede Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Utilities used for Centipede => Runner requests.

#ifndef THIRD_PARTY_CENTIPEDE_EXECUTION_REQUEST_H_
#define THIRD_PARTY_CENTIPEDE_EXECUTION_REQUEST_H_

#include <cstddef>
#include <vector>

#include "./centipede/execution_metadata.h"
#include "./centipede/mutation_input.h"
#include "./centipede/shared_memory_blob_sequence.h"
#include "./common/defs.h"

namespace fuzztest::internal {

// Sends a request (via `blobseq`) to execute `inputs`.
// Returns the number of sent inputs, which would normally be inputs.size().
size_t RequestExecution(const std::vector<ByteArray> &inputs,
                        BlobSequence &blobseq);

// Sends a request (via `blobseq`) to compute `num_mutants` mutants of `inputs`.
// Returns the number of sent inputs, which would normally be inputs.size().
size_t RequestMutation(size_t num_mutants,
                       const std::vector<MutationInputRef> &inputs,
                       BlobSequence &blobseq);

// Returns whether `blob` indicates an execution request.
bool IsExecutionRequest(Blob blob);

// Returns whether `blob` indicates a mutation request.
bool IsMutationRequest(Blob blob);

// Returns true and sets `num_inputs`
// iff the blob indicates the number of inputs.
bool IsNumInputs(Blob blob, size_t &num_inputs);

// Returns true and sets `num_mutants`
// iff the blob indicates the number of mutants.
bool IsNumMutants(Blob blob, size_t &num_mutants);

// Returns true and read blob into `metadata` iff the blob indicates an
// execution metadata.
bool IsExecutionMetadata(Blob blob, ExecutionMetadata &metadata);

// Returns true iff `blob` indicates a data input.
bool IsDataInput(Blob blob);

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_EXECUTION_REQUEST_H_
