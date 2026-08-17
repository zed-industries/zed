// Copyright 2024 The gRPC Authors
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

#ifndef GRPC_EVENT_ENGINE_EXTENSIBLE_H
#define GRPC_EVENT_ENGINE_EXTENSIBLE_H

#include <grpc/support/port_platform.h>

#include "absl/strings/string_view.h"

namespace grpc_event_engine {
namespace experimental {

class Extensible {
 public:
  /// A method which allows users to query whether an implementation supports a
  /// specified extension. The name of the extension is provided as an input.
  ///
  /// An extension could be any type with a unique string id. Each extension may
  /// support additional capabilities and if the implementation supports the
  /// queried extension, it should return a valid pointer to the extension type.
  ///
  /// E.g., use case of an EventEngine::Endpoint supporting a custom extension.
  ///
  /// class CustomEndpointExtension {
  ///  public:
  ///   static std::string EndpointExtensionName() {
  ///     return "my.namespace.extension_name";
  ///   }
  ///   virtual void Process() = 0;
  /// }
  ///
  /// class CustomEndpoint :
  ///        public EventEngine::Endpoint, public CustomEndpointExtension {
  ///   public:
  ///     void* QueryExtension(absl::string_view id) override {
  ///       if (id == CustomEndpointExtension::EndpointExtensionName()) {
  ///         return static_cast<CustomEndpointExtension*>(this);
  ///       }
  ///       return nullptr;
  ///     }
  ///     void Process() override { ... }
  ///     ...
  /// }
  ///
  /// auto endpoint =
  ///     static_cast<CustomEndpointExtension*>(endpoint->QueryExtension(
  ///         CustomEndpointExtension::EndpointExtensionName()));
  /// if (endpoint != nullptr) endpoint->Process();
  ///
  virtual void* QueryExtension(absl::string_view /*id*/) { return nullptr; }

 protected:
  ~Extensible() = default;
};

}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GRPC_EVENT_ENGINE_EXTENSIBLE_H
