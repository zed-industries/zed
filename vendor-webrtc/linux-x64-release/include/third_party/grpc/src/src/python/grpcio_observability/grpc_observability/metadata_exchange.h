//
//
// Copyright 2024 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_PYTHON_OBSERVABILITY_METADATA_EXCHANGE_H
#define GRPC_PYTHON_OBSERVABILITY_METADATA_EXCHANGE_H

#include <stddef.h>
#include <stdint.h>

#include <bitset>
#include <memory>
#include <string>
#include <utility>

#include "absl/strings/string_view.h"
#include "constants.h"
#include "python_observability_context.h"
#include "src/core/call/metadata_batch.h"

namespace grpc_observability {

class PythonLabelsInjector {
 public:
  explicit PythonLabelsInjector(const std::vector<Label>& exchange_labels);

  // Read the incoming initial metadata to get the set of labels exchanged from
  // peer.
  std::vector<Label> GetExchangeLabels(
      grpc_metadata_batch* incoming_initial_metadata) const;

  // Add metadata_to_exchange_ to the outgoing initial metadata.
  void AddExchangeLabelsToMetadata(
      grpc_metadata_batch* outgoing_initial_metadata) const;

  // Add optional xds labels from optional_labels_span to labels.
  void AddXdsOptionalLabels(
      bool is_client,
      absl::Span<const grpc_core::RefCountedStringValue> optional_labels_span,
      std::vector<Label>& labels);

 private:
  std::vector<std::pair<std::string, std::string>> metadata_to_exchange_;
};

}  // namespace grpc_observability

#endif  // GRPC_PYTHON_OBSERVABILITY_CONSTANTS_H
