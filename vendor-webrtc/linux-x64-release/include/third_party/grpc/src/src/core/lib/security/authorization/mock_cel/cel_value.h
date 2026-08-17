// Copyright 2020 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_MOCK_CEL_CEL_VALUE_H
#define GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_MOCK_CEL_CEL_VALUE_H

// CelValue is a holder, capable of storing all kinds of data
// supported by CEL.
// CelValue defines explicitly typed/named getters/setters.
// When storing pointers to objects, CelValue does not accept ownership
// to them and does not control their lifecycle. Instead objects are expected
// to be either external to expression evaluation, and controlled beyond the
// scope or to be allocated and associated with some allocation/ownership
// controller (Arena).
// Usage examples:
// (a) For primitive types:
//    CelValue value = CelValue::CreateInt64(1);
// (b) For string:
//    std::string* msg("test");
//    CelValue value = CelValue::CreateString(msg);

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <memory>
#include <string>
#include <utility>

#include "absl/strings/string_view.h"
#include "absl/types/span.h"

namespace grpc_core {
namespace mock_cel {

// Break cyclic dependencies for container types.
class CelMap {
 public:
  CelMap() = default;
};

// This is a temporary stub implementation of CEL APIs.
// Once gRPC imports the CEL library, this class will be removed.
class CelValue {
 public:
  // Default constructor.
  // Creates CelValue with null data type.
  CelValue() : CelValue(nullptr) {}

  // We will use factory methods instead of public constructors
  // The reason for this is the high risk of implicit type conversions
  // between bool/int/pointer types.
  // We rely on copy elision to avoid extra copying.
  static CelValue CreateNull() { return CelValue(nullptr); }

  static CelValue CreateInt64(int64_t /*value*/) { return CreateNull(); }

  static CelValue CreateUint64(uint64_t /*value*/) { return CreateNull(); }

  static CelValue CreateStringView(absl::string_view /*value*/) {
    return CreateNull();
  }

  static CelValue CreateString(const std::string* /*str*/) {
    return CreateNull();
  }

  static CelValue CreateMap(const CelMap* /*value*/) { return CreateNull(); }

 private:
  // Constructs CelValue wrapping value supplied as argument.
  // Value type T should be supported by specification of ValueHolder.
  template <class T>
  explicit CelValue(T /*value*/) {}
};

// CelMap implementation that uses STL map container as backing storage.
class ContainerBackedMapImpl : public CelMap {
 public:
  ContainerBackedMapImpl() = default;

  static std::unique_ptr<CelMap> Create(
      absl::Span<std::pair<CelValue, CelValue>> /*key_values*/) {
    return std::make_unique<ContainerBackedMapImpl>();
  }
};

}  // namespace mock_cel
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_MOCK_CEL_CEL_VALUE_H
