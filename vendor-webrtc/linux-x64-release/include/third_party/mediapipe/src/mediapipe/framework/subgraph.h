// Copyright 2019 The MediaPipe Authors.
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
//
// Facility for registering subgraphs that can be included in other graphs.

#ifndef MEDIAPIPE_FRAMEWORK_SUBGRAPH_H_
#define MEDIAPIPE_FRAMEWORK_SUBGRAPH_H_

#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/deps/registration.h"
#include "mediapipe/framework/graph_service.h"
#include "mediapipe/framework/graph_service_manager.h"
#include "mediapipe/framework/resources.h"
#include "mediapipe/framework/resources_service.h"
#include "mediapipe/framework/tool/calculator_graph_template.pb.h"
#include "mediapipe/framework/tool/options_util.h"

namespace mediapipe {

class SubgraphContext {
 public:
  SubgraphContext() : SubgraphContext(nullptr, nullptr) {}
  // @node and/or @service_manager can be nullptr.
  SubgraphContext(CalculatorGraphConfig::Node* node,
                  const GraphServiceManager* service_manager)
      : default_node_(node ? std::nullopt
                           : std::optional<CalculatorGraphConfig::Node>(
                                 CalculatorGraphConfig::Node())),
        original_node_(node ? *node : default_node_.value()),
        default_service_manager_(
            service_manager
                ? std::nullopt
                : std::optional<GraphServiceManager>(GraphServiceManager())),
        service_manager_(service_manager ? *std::move(service_manager)
                                         : default_service_manager_.value()),
        resources_([this]() {
          std::shared_ptr<Resources> resources =
              service_manager_.GetServiceObject(kResourcesService);
          if (!resources) {
            resources = CreateDefaultResources();
          }
          return resources;
        }()),
        options_map_(
            std::move(tool::MutableOptionsMap().Initialize(original_node_))) {}

  template <typename T>
  const T& Options() {
    return options_map_.Get<T>();
  }

  template <typename T>
  T* MutableOptions() {
    return options_map_.GetMutable<T>();
  }

  template <typename T>
  bool HasOptions() {
    return options_map_.Has<T>();
  }

  const CalculatorGraphConfig::Node& OriginalNode() const {
    return original_node_;
  }

  template <typename T>
  ServiceBinding<T> Service(const GraphService<T>& service) const {
    return ServiceBinding<T>(service_manager_.GetServiceObject(service));
  }

  // Gets interface to access resources (file system, assets, etc.) from
  // subgraphs.
  //
  // NOTE: this is the preferred way to access resources from subgraphs and
  // calculators as it allows for fine grained per graph configuration.
  //
  // Resources can be configured by setting a custom `kResourcesService` graph
  // service on `CalculatorGraph`. The default resources service can be created
  // and reused through `CreateDefaultResources`.
  const Resources& GetResources() { return *resources_; }

 private:
  // Populated if node is not provided during construction.
  std::optional<CalculatorGraphConfig::Node> default_node_;

  CalculatorGraphConfig::Node& original_node_;

  // Populated if service manager is not provided during construction.
  const std::optional<GraphServiceManager> default_service_manager_;

  const GraphServiceManager& service_manager_;

  std::shared_ptr<Resources> resources_;

  tool::MutableOptionsMap options_map_;
};

// Instances of this class are responsible for providing a subgraph config.
// They are only used during graph construction. They do not stay alive once
// the graph is running.
class Subgraph {
 public:
  using SubgraphOptions = CalculatorGraphConfig::Node;
  Subgraph();
  virtual ~Subgraph();

  // Returns the config to use for one instantiation of the subgraph. The
  // nodes and generators in this config will replace the subgraph node in
  // the parent graph.
  // Subclasses may use `SubgraphContext*` param to parameterize the config.
  // TODO: make this static?
  virtual absl::StatusOr<CalculatorGraphConfig> GetConfig(SubgraphContext* sc) {
    if (sc == nullptr) {
      return GetConfig(SubgraphOptions{});
    }
    return GetConfig(sc->OriginalNode());
  }

  // Kept for backward compatibility - please override `GetConfig` taking
  // `SubgraphContext*` param.
  virtual absl::StatusOr<CalculatorGraphConfig> GetConfig(
      const SubgraphOptions& options) {
    return absl::UnimplementedError("Not implemented.");
  }

  // Returns options of a specific type.
  template <typename T>
  static T GetOptions(const Subgraph::SubgraphOptions& supgraph_options) {
    return tool::OptionsMap().Initialize(supgraph_options).Get<T>();
  }

  template <typename T>
  static bool HasOptions(const Subgraph::SubgraphOptions& supgraph_options) {
    return tool::OptionsMap().Initialize(supgraph_options).Has<T>();
  }

  // Returns the CalculatorGraphConfig::Node specifying the subgraph.
  // This provides to Subgraphs the same graph information that GetContract
  // provides to Calculators.
  static CalculatorGraphConfig::Node GetNode(
      const Subgraph::SubgraphOptions& supgraph_options) {
    return supgraph_options;
  }
};

using SubgraphRegistry = GlobalFactoryRegistry<std::unique_ptr<Subgraph>>;

#define REGISTER_MEDIAPIPE_GRAPH(name)                             \
  REGISTER_FACTORY_FUNCTION_QUALIFIED(mediapipe::SubgraphRegistry, \
                                      subgraph_registration, name, \
                                      std::make_unique<name>)

// A graph factory holding a literal CalculatorGraphConfig.
class ProtoSubgraph : public Subgraph {
 public:
  ProtoSubgraph(const CalculatorGraphConfig& config);
  virtual ~ProtoSubgraph();
  virtual absl::StatusOr<CalculatorGraphConfig> GetConfig(
      const Subgraph::SubgraphOptions& options);

 private:
  CalculatorGraphConfig config_;
};

// A graph factory holding a literal CalculatorGraphTemplate.
class TemplateSubgraph : public Subgraph {
 public:
  TemplateSubgraph(const CalculatorGraphTemplate& templ);
  virtual ~TemplateSubgraph();
  virtual absl::StatusOr<CalculatorGraphConfig> GetConfig(
      const Subgraph::SubgraphOptions& options);

 private:
  CalculatorGraphTemplate templ_;
};

// A local registry of CalculatorGraphConfig definitions.
class GraphRegistry {
 public:
  // Creates a GraphRegistry derived from the GlobalFactoryRegistry,
  // which stores statically linked Subgraphs.
  GraphRegistry();

  // Creates a GraphRegistry derived from the specified FunctionRegistry,
  // which is used in place of the GlobalFactoryRegistry.
  // Ownership of the specified FunctionRegistry is not transferred.
  GraphRegistry(FunctionRegistry<std::unique_ptr<Subgraph>>* factories);

  // Registers a graph config builder type, using a factory function.
  void Register(const std::string& type_name,
                std::function<std::unique_ptr<Subgraph>()> factory);

  // Registers a graph config by name.
  void Register(const std::string& type_name,
                const CalculatorGraphConfig& config);

  // Registers a template graph config by name.
  void Register(const std::string& type_name,
                const CalculatorGraphTemplate& templ);

  // Returns true if the specified graph config is registered.
  bool IsRegistered(const std::string& ns, const std::string& type_name) const;

  // Returns the specified graph config.
  absl::StatusOr<CalculatorGraphConfig> CreateByName(
      absl::string_view ns, absl::string_view type_name,
      SubgraphContext* context = nullptr) const;

  static GraphRegistry global_graph_registry;

 private:
  // The FunctionRegistry for dynamically loaded Subgraphs.
  mutable FunctionRegistry<std::unique_ptr<Subgraph>> local_factories_;
  // The FunctionRegistry for statically linked Subgraphs.
  // The global_factories_ registry is overridden by local_factories_.
  mutable FunctionRegistry<std::unique_ptr<Subgraph>>* global_factories_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_SUBGRAPH_H_
