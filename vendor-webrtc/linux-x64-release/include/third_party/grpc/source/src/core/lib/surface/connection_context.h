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

#ifndef GRPC_SRC_CORE_LIB_SURFACE_CONNECTION_CONTEXT_H
#define GRPC_SRC_CORE_LIB_SURFACE_CONNECTION_CONTEXT_H

#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <cstdint>
#include <vector>

#include "src/core/util/no_destruct.h"
#include "src/core/util/orphanable.h"

namespace grpc_core {

class ConnectionContext;

template <typename T>
struct ConnectionContextProperty;

namespace connection_context_detail {

// Tracks all registered/known auth property types (these should only be
// registered via AuthPropertyTraits at static initialization time).
class BaseConnectionContextPropertiesTraits {
 public:
  // Count of number of registered auth context properties.
  static uint16_t NumProperties() {
    return static_cast<uint16_t>(RegisteredTraits().size());
  }

  // Number of bytes required to store pointers of all registered auth context
  // properties.
  static size_t Size() { return NumProperties() * sizeof(void*); }

  // Call the registered destruction function for a context.
  static void Destroy(uint16_t id, void* ptr) {
    if (ptr == nullptr) return;
    RegisteredTraits()[id](ptr);
  }

  // Allocate a new context id and register the destruction function.
  template <typename T>
  static uint16_t AllocateId(void (*destroy)(void* ptr)) {
    auto& traits = RegisteredTraits();
    const uint16_t id = static_cast<uint16_t>(traits.size());
    traits.push_back(destroy);
    return id;
  }

 private:
  // Allocate a new context id and register the destruction function.

  static std::vector<void (*)(void*)>& RegisteredTraits() {
    static NoDestruct<std::vector<void (*)(void*)>> registered_traits;
    return *registered_traits;
  }
};

template <typename T>
class ConnectionContextPropertiesTraits
    : public BaseConnectionContextPropertiesTraits {
 public:
  static uint16_t id() { return id_; }
  template <typename... Args>
  static T* Construct(Args&&... args) {
    return new T(std::forward<Args>(args)...);
  }
  static void Destruct(void* p) { delete reinterpret_cast<T*>(p); }

 protected:
  static const uint16_t id_;
};

template <typename T>
const uint16_t ConnectionContextPropertiesTraits<T>::id_ =
    BaseConnectionContextPropertiesTraits::AllocateId<T>(
        ConnectionContextPropertiesTraits<T>::Destruct);

}  // namespace connection_context_detail

class ConnectionContext final : public Orphanable {
 public:
  static OrphanablePtr<ConnectionContext> Create();

  // Sets the value of a registered property if it is not already set. Return
  // false if the property is already set. If the property is not already
  // set, an object of type Which is created using the passed args
  // and stored in the map.
  template <typename Which, typename... Args>
  bool EmplaceIfUnset(Args&&... args) {
    using Traits =
        connection_context_detail::ConnectionContextPropertiesTraits<Which>;
    Which* value = static_cast<Which*>(registered_properties()[Traits::id()]);
    if (value == nullptr) {
      registered_properties()[Traits::id()] = Traits::Construct(args...);
      return true;
    }
    return false;
  }

  // Force updates the value of a registered property. It deletes any previously
  // set value.
  template <typename Which, typename... Args>
  void Update(Args&&... args) {
    using Traits =
        connection_context_detail::ConnectionContextPropertiesTraits<Which>;
    Which* prev = static_cast<Which*>(registered_properties()[Traits::id()]);
    if (prev != nullptr) {
      Traits::Destroy(Traits::id(), prev);
    }
    registered_properties()[Traits::id()] = Traits::Construct(args...);
  }

  // Returns the value of a registered property. If the property is not set,
  // returns nullptr.
  template <typename Which>
  const Which* Get() {
    return static_cast<Which*>(
        registered_properties()
            [connection_context_detail::ConnectionContextPropertiesTraits<
                Which>::id()]);
  }

  void Orphan() override;

  ~ConnectionContext() override;

 private:
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION void** registered_properties() {
    return reinterpret_cast<void**>(this + 1);
  }

  ConnectionContext();
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SURFACE_CONNECTION_CONTEXT_H
