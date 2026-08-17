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

#ifndef MEDIAPIPE_TASKS_CC_CORE_MODEL_RESOURCES_H_
#define MEDIAPIPE_TASKS_CC_CORE_MODEL_RESOURCES_H_

#include <cstddef>
#include <functional>
#include <memory>
#include <string>

#include "absl/memory/memory.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "mediapipe/framework/api2/packet.h"
#include "mediapipe/tasks/cc/common.h"
#include "mediapipe/tasks/cc/core/external_file_handler.h"
#include "mediapipe/tasks/cc/core/proto/external_file.pb.h"
#include "mediapipe/tasks/cc/metadata/metadata_extractor.h"
#include "mediapipe/util/tflite/error_reporter.h"
#include "tensorflow/lite/core/api/error_reporter.h"
#include "tensorflow/lite/core/api/op_resolver.h"
#include "tensorflow/lite/kernels/register.h"
#include "tensorflow/lite/model.h"
#include "tensorflow/lite/model_builder.h"
#include "tensorflow/lite/tools/verifier.h"

namespace mediapipe {
namespace tasks {
namespace core {

// The mediapipe task model resources class.
// A ModelResources object, created from an external file proto, bundles the
// model-related resources that are needed by a mediapipe task. As the
// resources, including flatbuffer model, op resolver, model metadata extractor,
// and external file handler, are owned by the ModelResources object, callers
// must keep ModelResources alive while using any of the resources.
class ModelResources {
 public:
  // Represents a TfLite model as a FlatBuffer.
  using ModelPtr =
      std::unique_ptr<tflite::FlatBufferModel,
                      std::function<void(tflite::FlatBufferModel*)>>;

  // Takes the ownership of the provided ExternalFile proto and creates
  // ModelResources from the proto and an op resolver object. A non-empty tag
  // must be set if the ModelResources will be used through
  // ModelResourcesCacheService.
  static absl::StatusOr<std::unique_ptr<ModelResources>> Create(
      const std::string& tag, std::unique_ptr<proto::ExternalFile> model_file,
      std::unique_ptr<tflite::OpResolver> op_resolver =
          absl::make_unique<tflite::ops::builtin::BuiltinOpResolver>());

  // Takes the ownership of the provided ExternalFile proto and creates
  // ModelResources from the proto and an op resolver mediapipe packet. A
  // non-empty tag must be set if the ModelResources will be used through
  // ModelResourcesCacheService. The op resolver packet, usually prvoided by a
  // ModelResourcesCacheService object, contains the TFLite op resolvers
  // required by the model.
  static absl::StatusOr<std::unique_ptr<ModelResources>> Create(
      const std::string& tag, std::unique_ptr<proto::ExternalFile> model_file,
      api2::Packet<tflite::OpResolver> op_resolver_packet);

  // ModelResources is neither copyable nor movable.
  ModelResources(const ModelResources&) = delete;
  ModelResources& operator=(const ModelResources&) = delete;

  // Returns the model resources tag.
  const std::string& GetTag() const { return tag_; }

  // Returns the model file proto.
  const proto::ExternalFile& GetModelFile() const { return *model_file_; }

  // Returns a pointer to tflite::model.
  const tflite::Model* GetTfLiteModel() const;

  // Returns a const pointer to the model metadata extractor.
  const metadata::ModelMetadataExtractor* GetMetadataExtractor() const {
    return &metadata_extractor_packet_.Get();
  }

  // Returns a shallow copy of the TFLite model packet.
  api2::Packet<ModelPtr> GetModelPacket() const { return model_packet_; }

  // Returns a shallow copy of the TFLite op reslover packet.
  api2::Packet<tflite::OpResolver> GetOpResolverPacket() const {
    return op_resolver_packet_;
  }

  // Returns a shallow copy of the model metadata extractor packet.
  api2::Packet<metadata::ModelMetadataExtractor> GetMetadataExtractorPacket()
      const {
    return metadata_extractor_packet_;
  }

 private:
  // Direct wrapper around tflite::TfLiteVerifier which checks the integrity of
  // the FlatBuffer data provided as input.
  class Verifier : public tflite::TfLiteVerifier {
   public:
    bool Verify(const char* data, int length,
                tflite::ErrorReporter* reporter) override;
  };

  // Constructor.
  ModelResources(const std::string& tag,
                 std::unique_ptr<proto::ExternalFile> model_file,
                 api2::Packet<tflite::OpResolver> op_resolver_packet);

  // Builds the TFLite model from the ExternalFile proto.
  absl::Status BuildModelFromExternalFileProto();

  // The model resources tag.
  const std::string tag_;
  // The model file.
  std::unique_ptr<proto::ExternalFile> model_file_;
  // The packet stores the TFLite op resolver.
  api2::Packet<tflite::OpResolver> op_resolver_packet_;

  // The ExternalFileHandler for the model.
  std::unique_ptr<ExternalFileHandler> model_file_handler_;
  // The packet stores the TFLite model for actual inference.
  api2::Packet<ModelPtr> model_packet_;
  // The packet stores the TFLite Metadata extractor built from the model.
  api2::Packet<metadata::ModelMetadataExtractor> metadata_extractor_packet_;

  // Extra verifier for FlatBuffer input data.
  Verifier verifier_;
  // Error reporter that captures and prints to stderr low-level TFLite
  // error messages.
  mediapipe::util::tflite::ErrorReporter error_reporter_;
};

}  // namespace core
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_CORE_MODEL_RESOURCES_H_
