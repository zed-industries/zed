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

#ifndef MEDIAPIPE_FRAMEWORK_GRAPH_SERVICE_H_
#define MEDIAPIPE_FRAMEWORK_GRAPH_SERVICE_H_

#include <memory>
#include <type_traits>
#include <utility>

#include "absl/log/absl_check.h"
#include "absl/strings/str_cat.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {

// The GraphService API can be used to define extensions to a graph's execution
// environment. These are, essentially, graph-level singletons, and are
// available to all calculators in the graph (and in any subgraphs) without
// requiring a manual connection.
//
// IMPORTANT: this is an experimental API. Get in touch with the MediaPipe team
// if you want to use it. In most cases, you should use a side packet instead.

class GraphServiceBase {
 public:
  // TODO: fix services for which default init is broken, remove
  // this setting.
  enum DefaultInitSupport {
    kAllowDefaultInitialization,
    kDisallowDefaultInitialization
  };

  explicit constexpr GraphServiceBase(const char* key) : key(key) {}

  inline virtual absl::StatusOr<Packet> CreateDefaultObject() const {
    return DefaultInitializationUnsupported();
  }

  const char* key;

 protected:
  // `GraphService<T>` objects, deriving `GraphServiceBase` are designed to be
  // global constants and not ever deleted through `GraphServiceBase`. Hence,
  // protected and non-virtual destructor which helps to make `GraphService<T>`
  // trivially destructible and properly defined as global constants.
  //
  // A class with any virtual functions should have a destructor that is either
  // public and virtual or else protected and non-virtual.
  // https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#Rc-dtor-virtual
  ~GraphServiceBase() = default;

  absl::Status DefaultInitializationUnsupported() const {
    return absl::UnimplementedError(absl::StrCat(
        "Graph service '", key, "' does not support default initialization"));
  }
};

// A global constant to refer a service:
// - Requesting `CalculatorContract::UseService` from calculator
// - Accessing `Calculator/SubgraphContext::Service`from calculator/subgraph
// - Setting before graph initialization `CalculatorGraph::SetServiceObject`
//
// NOTE: In headers, define your graph service reference safely as following:
// `inline constexpr GraphService<YourService> kYourService("YourService");`
//
template <typename T>
class GraphService final : public GraphServiceBase {
 public:
  using type = T;
  using packet_type = std::shared_ptr<T>;

  constexpr GraphService(const char* my_key, DefaultInitSupport default_init =
                                                 kDisallowDefaultInitialization)
      : GraphServiceBase(my_key), default_init_(default_init) {}

  absl::StatusOr<Packet> CreateDefaultObject() const final {
    if (default_init_ != kAllowDefaultInitialization) {
      return DefaultInitializationUnsupported();
    }
    auto packet_or = CreateDefaultObjectInternal();
    if (packet_or.ok()) {
      return MakePacket<std::shared_ptr<T>>(std::move(packet_or).value());
    } else {
      return packet_or.status();
    }
  }

 private:
  absl::StatusOr<std::shared_ptr<T>> CreateDefaultObjectInternal() const {
    auto call_create = [](auto x) -> decltype(decltype(x)::type::Create()) {
      return decltype(x)::type::Create();
    };
    if constexpr (std::is_invocable_r_v<absl::StatusOr<std::shared_ptr<T>>,
                                        decltype(call_create), type_tag<T>>) {
      return T::Create();
    }
    if constexpr (std::is_default_constructible_v<T>) {
      return std::make_shared<T>();
    }
    return DefaultInitializationUnsupported();
  }

  template <class U>
  struct type_tag {
    using type = U;
  };

  DefaultInitSupport default_init_;
};

template <typename T>
class ServiceBinding {
 public:
  bool IsAvailable() { return service_ != nullptr; }
  T& GetObject() {
    ABSL_CHECK(service_) << "Service is unavailable.";
    return *service_;
  }

  ServiceBinding() {}
  explicit ServiceBinding(std::shared_ptr<T> service) : service_(service) {}

 private:
  std::shared_ptr<T> service_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_GRAPH_SERVICE_H_
