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

#ifndef MEDIAPIPE_TASKS_CC_CORE_MODEL_ASSET_BUNDLE_RESOURCES_H_
#define MEDIAPIPE_TASKS_CC_CORE_MODEL_ASSET_BUNDLE_RESOURCES_H_

#include "absl/container/flat_hash_map.h"
#include "absl/status/statusor.h"
#include "mediapipe/tasks/cc/core/external_file_handler.h"
#include "mediapipe/tasks/cc/core/proto/external_file.pb.h"

namespace mediapipe {
namespace tasks {
namespace core {

// The mediapipe task model asset bundle resources class.
// A ModelAssetBundleResources object, created from an external file proto,
// contains model asset bundle related resources and the method to extract the
// tflite models, resource files or model asset bundles for the mediapipe
// sub-tasks. As the resources are owned by the ModelAssetBundleResources object
// callers must keep ModelAssetBundleResources alive while using any of the
// resources.
class ModelAssetBundleResources {
 public:
  // Takes the ownership of the provided ExternalFile proto and creates
  // ModelAssetBundleResources from the proto. A non-empty tag
  // must be set if the ModelAssetBundleResources will be used through
  // ModelResourcesCacheService.
  static absl::StatusOr<std::unique_ptr<ModelAssetBundleResources>> Create(
      const std::string& tag,
      std::unique_ptr<proto::ExternalFile> model_asset_bundle_file);

  // ModelResources is neither copyable nor movable.
  ModelAssetBundleResources(const ModelAssetBundleResources&) = delete;
  ModelAssetBundleResources& operator=(const ModelAssetBundleResources&) =
      delete;

  // Returns the model asset bundle resources tag.
  std::string GetTag() const { return tag_; }

  // Gets the contents of the model file (either tflite model file, resource
  // file or model bundle file) with the provided name. An error is returned if
  // there is no such model file.
  absl::StatusOr<absl::string_view> GetFile(const std::string& filename) const;

  // Lists all the file names in the model asset model.
  std::vector<std::string> ListFiles() const;

 private:
  // Constructor.
  ModelAssetBundleResources(
      const std::string& tag,
      std::unique_ptr<proto::ExternalFile> model_asset_bundle_file);

  // Extracts the model files (either tflite model file, resource file or model
  // bundle file) from the external file proto.
  absl::Status ExtractFilesFromExternalFileProto();

  // The model asset bundle resources tag.
  const std::string tag_;

  // The model asset bundle file.
  std::unique_ptr<proto::ExternalFile> model_asset_bundle_file_;

  // The ExternalFileHandler for the model asset bundle.
  std::unique_ptr<ExternalFileHandler> model_asset_bundle_file_handler_;

  // The files bundled in model asset bundle, as a map with the filename
  // (corresponding to a basename, e.g. "hand_detector.tflite") as key and
  // a pointer to the file contents as value. Each file can be either a TFLite
  // model file, resource file or a model bundle file for sub-task.
  absl::flat_hash_map<std::string, absl::string_view> files_;
};

}  // namespace core
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_CORE_MODEL_ASSET_BUNDLE_RESOURCES_H_
