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

#ifndef MEDIAPIPE_TASKS_CC_CORE_MODEL_RESOURCES_CACHE_H_
#define MEDIAPIPE_TASKS_CC_CORE_MODEL_RESOURCES_CACHE_H_

#include <memory>
#include <string>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/memory/memory.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "mediapipe/framework/api2/packet.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/tasks/cc/core/model_asset_bundle_resources.h"
#include "mediapipe/tasks/cc/core/model_resources.h"
#include "tensorflow/lite/core/api/op_resolver.h"

namespace mediapipe {
namespace tasks {
namespace core {

// Manages the insertion and lookup of the cached mediapipe task model
// resources. ModelResourcesCache maps a unique resources tag to a cached
// ModelResources object that bundles the model-related resources (e.g.,
// flatbuffer model, op resolver, and model metadata extractor) of a particular
// model.
class ModelResourcesCache {
 public:
  explicit ModelResourcesCache(
      std::unique_ptr<tflite::OpResolver> graph_op_resolver = nullptr);

  // Returns whether the tag exists in the model resources cache.
  bool Exists(const std::string& tag) const;

  // Returns whether the tag of the model asset bundle exists in the model
  // resources cache.
  bool ModelAssetBundleExists(const std::string& tag) const;

  // Adds a ModelResources object into the cache.
  // The tag of the ModelResources must be unique; the ownership of the
  // ModelResources will be transferred into the cache.
  absl::Status AddModelResources(
      std::unique_ptr<ModelResources> model_resources);

  // Adds a collection of the ModelResources objects into the cache.
  // The tag of the each ModelResources must be unique; the ownership of
  // every ModelResource will be transferred into the cache.
  absl::Status AddModelResourcesCollection(
      std::vector<std::unique_ptr<ModelResources>>& model_resources_collection);

  // Retrieves a const ModelResources pointer by the unique tag.
  absl::StatusOr<const ModelResources*> GetModelResources(
      const std::string& tag) const;

  // Adds a ModelAssetBundleResources object into the cache.
  // The tag of the ModelAssetBundleResources must be unique; the ownership of
  // the ModelAssetBundleResources will be transferred into the cache.
  absl::Status AddModelAssetBundleResources(
      std::unique_ptr<ModelAssetBundleResources> model_asset_bundle_resources);

  // Adds a collection of the ModelAssetBundleResources objects into the cache.
  // The tag of the each ModelAssetBundleResources must be unique; the ownership
  // of every ModelAssetBundleResources will be transferred into the cache.
  absl::Status AddModelAssetBundleResourcesCollection(
      std::vector<std::unique_ptr<ModelAssetBundleResources>>&
          model_asset_bundle_resources_collection);

  // Retrieves a const ModelAssetBundleResources pointer by the unique tag.
  absl::StatusOr<const ModelAssetBundleResources*> GetModelAssetBundleResources(
      const std::string& tag) const;

  // Retrieves the graph op resolver packet.
  absl::StatusOr<api2::Packet<tflite::OpResolver>> GetGraphOpResolverPacket()
      const;

 private:
  // The packet stores all TFLite op resolvers for the models in the graph.
  api2::Packet<tflite::OpResolver> graph_op_resolver_packet_;

  // A collection of ModelResources objects for the models in the graph.
  absl::flat_hash_map<std::string, std::unique_ptr<ModelResources>>
      model_resources_collection_;

  // A collection of ModelAssetBundleResources objects for the model bundles in
  // the graph.
  absl::flat_hash_map<std::string, std::unique_ptr<ModelAssetBundleResources>>
      model_asset_bundle_resources_collection_;
};

// Global service for mediapipe task model resources cache.
inline constexpr mediapipe::GraphService<ModelResourcesCache>
    kModelResourcesCacheService("mediapipe::tasks::ModelResourcesCacheService");

}  // namespace core
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_CORE_MODEL_RESOURCES_CACHE_H_
