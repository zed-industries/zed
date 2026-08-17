/* Copyright 2022 The MediaPipe Authors.

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

#ifndef MEDIAPIPE_TASKS_CC_COMPONENTS_UTILS_COSINE_SIMILARITY_H_
#define MEDIAPIPE_TASKS_CC_COMPONENTS_UTILS_COSINE_SIMILARITY_H_

#include "absl/status/statusor.h"
#include "mediapipe/tasks/cc/components/containers/embedding_result.h"

namespace mediapipe {
namespace tasks {
namespace components {
namespace utils {

// Utility function to compute cosine similarity [1] between two embeddings. May
// return an InvalidArgumentError if e.g. the embeddings are of different types
// (quantized vs. float), have different sizes, or have a an L2-norm of 0.
//
// [1]: https://en.wikipedia.org/wiki/Cosine_similarity
absl::StatusOr<double> CosineSimilarity(const containers::Embedding& u,
                                        const containers::Embedding& v);

}  // namespace utils
}  // namespace components
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_COMPONENTS_UTILS_COSINE_SIMILARITY_H_
