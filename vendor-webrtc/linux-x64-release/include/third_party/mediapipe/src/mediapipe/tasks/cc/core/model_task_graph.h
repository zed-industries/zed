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

#ifndef MEDIAPIPE_TASKS_CC_CORE_MODEL_TASK_GRAPH_H_
#define MEDIAPIPE_TASKS_CC_CORE_MODEL_TASK_GRAPH_H_

#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/status/statusor.h"
#include "absl/strings/str_format.h"
#include "mediapipe/calculators/tensor/inference_calculator.pb.h"
#include "mediapipe/framework/api2/builder.h"
#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/subgraph.h"
#include "mediapipe/tasks/cc/core/model_asset_bundle_resources.h"
#include "mediapipe/tasks/cc/core/model_resources.h"
#include "mediapipe/tasks/cc/core/proto/acceleration.pb.h"
#include "mediapipe/tasks/cc/core/proto/base_options.pb.h"
#include "mediapipe/tasks/cc/core/proto/external_file.pb.h"

namespace mediapipe {
namespace tasks {
namespace core {

// The base class of mediapipe task graphs.
// Graph authors need to create a derived class per mediapipe task graph,
// and override the GetConfig() method to dynamically compose the task-specific
// graph based on the user settings and the model metadata. The mediapipe task
// subgraphs will be fully expanded during the initialization of a MediaPipe
// CalculatorGraph.
class ModelTaskGraph : public Subgraph {
 public:
  // Returns the graph config to use for one instantiation of the model task
  // graph. Must be overridden by subclasses in which the graph authors define
  // the concrete task graphs based on user settings and model metadata.
  absl::StatusOr<mediapipe::CalculatorGraphConfig> GetConfig(
      SubgraphContext* sc) override;

 protected:
  // If the model resources graph service is available, creates a model
  // resources object from the subgraph context, and caches the created model
  // resources into the model resources graph service on success. Otherwise,
  // creates a local model resources object that can only be used in the graph
  // construction stage. The returned model resources pointer will provide graph
  // authors with the access to the metadata extractor and the tflite model.
  // If more than one model resources are created in a graph, the model
  // resources graph service add the tag_suffix to support multiple resources.
  template <typename Options>
  absl::StatusOr<const ModelResources*> CreateModelResources(
      SubgraphContext* sc, std::string tag_suffix = "") {
    auto external_file = std::make_unique<proto::ExternalFile>();
    external_file->Swap(sc->MutableOptions<Options>()
                            ->mutable_base_options()
                            ->mutable_model_asset());
    return CreateModelResources(sc, std::move(external_file), tag_suffix);
  }

  // If the model resources graph service is available, creates a model
  // resources object from the subgraph context, and caches the created model
  // resources into the model resources graph service on success. Otherwise,
  // creates a local model resources object that can only be used in the graph
  // construction stage. Note that the external file contents will be moved
  // into the model resources object on creation. The returned model resources
  // pointer will provide graph authors with the access to the metadata
  // extractor and the tflite model. When the model resources graph service is
  // available, a tag is generated internally asscoiated with the created model
  // resource. If more than one model resources are created in a graph, the
  // model resources graph service add the tag_suffix to support multiple
  // resources.
  absl::StatusOr<const ModelResources*> CreateModelResources(
      SubgraphContext* sc, std::unique_ptr<proto::ExternalFile> external_file,
      std::string tag_suffix = "");

  template <typename Options>
  absl::StatusOr<const ModelResources*> GetOrCreateModelResources(
      SubgraphContext* sc, std::string tag_suffix = "") {
    auto external_file = std::make_unique<proto::ExternalFile>();
    external_file->Swap(sc->MutableOptions<Options>()
                            ->mutable_base_options()
                            ->mutable_model_asset());
    return GetOrCreateModelResources(sc, std::move(external_file), tag_suffix);
  }

  absl::StatusOr<const ModelResources*> GetOrCreateModelResources(
      SubgraphContext* sc, std::unique_ptr<proto::ExternalFile> external_file,
      std::string tag_suffix = "");

  // If the model resources graph service is available, creates a model asset
  // bundle resources object from the subgraph context, and caches the created
  // model asset bundle resources into the model resources graph service on
  // success. Otherwise, creates a local model asset bundle resources object
  // that can only be used in the graph construction stage. The returned model
  // resources pointer will provide graph authors with the access to extracted
  // model files.
  template <typename Options>
  absl::StatusOr<const ModelAssetBundleResources*>
  CreateModelAssetBundleResources(SubgraphContext* sc) {
    auto external_file = std::make_unique<proto::ExternalFile>();
    external_file->Swap(sc->MutableOptions<Options>()
                            ->mutable_base_options()
                            ->mutable_model_asset());
    return CreateModelAssetBundleResources(sc, std::move(external_file));
  }

  // If the model resources graph service is available, creates a model asset
  // bundle resources object from the subgraph context, and caches the created
  // model asset bundle resources into the model resources graph service on
  // success. Otherwise, creates a local model asset bundle resources object
  // that can only be used in the graph construction stage. Note that the
  // external file contents will be moved into the model asset bundle resources
  // object on creation. The returned model asset bundle resources pointer will
  // provide graph authors with the access to extracted model files. When the
  // model resources graph service is available, a tag is generated internally
  // asscoiated with the created model asset bundle resource. If more than one
  // model asset bundle resources are created in a graph, the model resources
  // graph service add the tag_suffix to support multiple resources.
  absl::StatusOr<const ModelAssetBundleResources*>
  CreateModelAssetBundleResources(
      SubgraphContext* sc, std::unique_ptr<proto::ExternalFile> external_file,
      const std::string tag_suffix = "");

  // Inserts a mediapipe task inference subgraph into the provided
  // GraphBuilder. The returned node provides the following interfaces to the
  // the rest of the graph:
  //   - a tensor vector (std::vector<mediapipe::Tensor>) input stream with tag
  //     "TENSORS", representing the input tensors to be consumed by the
  //     inference engine.
  //   - a tensor vector (std::vector<mediapipe::Tensor>) output stream with tag
  //     "TENSORS", representing the output tensors generated by the inference
  //     engine.
  //   - a MetadataExtractor output side packet with tag "METADATA_EXTRACTOR".
  api2::builder::GenericNode& AddInference(
      const ModelResources& model_resources,
      const proto::Acceleration& acceleration,
      api2::builder::Graph& graph) const;

 private:
  std::vector<std::unique_ptr<ModelResources>> local_model_resources_;

  std::vector<std::unique_ptr<ModelAssetBundleResources>>
      local_model_asset_bundle_resources_;
};

}  // namespace core
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_CORE_MODEL_TASK_GRAPH_H_
