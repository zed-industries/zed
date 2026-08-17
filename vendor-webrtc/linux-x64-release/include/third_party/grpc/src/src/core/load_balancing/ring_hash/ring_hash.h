//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_RING_HASH_RING_HASH_H
#define GRPC_SRC_CORE_LOAD_BALANCING_RING_HASH_RING_HASH_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include "src/core/service_config/service_config_call_data.h"
#include "src/core/util/json/json.h"
#include "src/core/util/json/json_args.h"
#include "src/core/util/json/json_object_loader.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/util/validation_errors.h"

// Optional endpoint attribute specifying the hash key.
#define GRPC_ARG_RING_HASH_ENDPOINT_HASH_KEY \
  GRPC_ARG_NO_SUBCHANNEL_PREFIX "hash_key"

namespace grpc_core {

class RequestHashAttribute final
    : public ServiceConfigCallData::CallAttributeInterface {
 public:
  static UniqueTypeName TypeName();

  explicit RequestHashAttribute(uint64_t request_hash)
      : request_hash_(request_hash) {}

  uint64_t request_hash() const { return request_hash_; }

 private:
  UniqueTypeName type() const override { return TypeName(); }

  uint64_t request_hash_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_RING_HASH_RING_HASH_H
