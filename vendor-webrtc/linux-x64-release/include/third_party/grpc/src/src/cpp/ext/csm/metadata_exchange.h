//
//
// Copyright 2023 gRPC authors.
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

#ifndef GRPC_SRC_CPP_EXT_CSM_METADATA_EXCHANGE_H
#define GRPC_SRC_CPP_EXT_CSM_METADATA_EXCHANGE_H

#include <grpc/support/port_platform.h>

#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "google/protobuf/struct.upb.h"
#include "opentelemetry/sdk/common/attribute_utils.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/lib/slice/slice.h"
#include "src/cpp/ext/otel/otel_plugin.h"
#include "upb/mem/arena.hpp"

namespace grpc {
namespace internal {

class ServiceMeshLabelsInjector : public LabelsInjector {
 public:
  explicit ServiceMeshLabelsInjector(
      const opentelemetry::sdk::common::AttributeMap& map);
  // Read the incoming initial metadata to get the set of labels to be added to
  // metrics.
  std::unique_ptr<LabelsIterable> GetLabels(
      grpc_metadata_batch* incoming_initial_metadata) const override;

  // Modify the outgoing initial metadata with metadata information to be sent
  // to the peer.
  void AddLabels(grpc_metadata_batch* outgoing_initial_metadata,
                 LabelsIterable* labels_from_incoming_metadata) const override;

  // Add optional labels to the traced calls.
  bool AddOptionalLabels(
      bool is_client,
      absl::Span<const grpc_core::RefCountedStringValue> optional_labels,
      opentelemetry::nostd::function_ref<
          bool(opentelemetry::nostd::string_view,
               opentelemetry::common::AttributeValue)>
          callback) const override;

  // Gets the size of the actual optional labels.
  size_t GetOptionalLabelsSize(
      bool is_client,
      absl::Span<const grpc_core::RefCountedStringValue> /*optional_labels*/)
      const override {
    return is_client ? 2 : 0;
  }

  const std::vector<std::pair<absl::string_view, std::string>>&
  TestOnlyLocalLabels() const {
    return local_labels_;
  }

  const grpc_core::Slice& TestOnlySerializedLabels() const {
    return serialized_labels_to_send_;
  }

 private:
  std::vector<std::pair<absl::string_view, std::string>> local_labels_;
  grpc_core::Slice serialized_labels_to_send_;
};

// A LabelsIterable class provided by ServiceMeshLabelsInjector. EXPOSED FOR
// TESTING PURPOSES ONLY.
class MeshLabelsIterable : public LabelsIterable {
 public:
  enum class GcpResourceType : std::uint8_t { kGke, kGce, kUnknown };

  MeshLabelsIterable(
      const std::vector<std::pair<absl::string_view, std::string>>&
          local_labels,
      grpc_core::Slice remote_metadata);

  std::optional<std::pair<absl::string_view, absl::string_view>> Next()
      override;

  size_t Size() const override;

  void ResetIteratorPosition() override { pos_ = 0; }

  // Returns true if the peer sent a non-empty base64 encoded
  // "x-envoy-peer-metadata" metadata.
  bool GotRemoteLabels() const { return struct_pb_ != nullptr; }

 private:
  upb::Arena arena_;
  google_protobuf_Struct* struct_pb_ = nullptr;
  const std::vector<std::pair<absl::string_view, std::string>>& local_labels_;
  GcpResourceType remote_type_ = GcpResourceType::kUnknown;
  uint32_t pos_ = 0;
};

}  // namespace internal
}  // namespace grpc

#endif  // GRPC_SRC_CPP_EXT_CSM_METADATA_EXCHANGE_H
