//
// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_RESOURCE_TYPE_H
#define GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_RESOURCE_TYPE_H
#include <grpc/support/port_platform.h>

#include <memory>
#include <optional>
#include <string>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/xds/xds_client/xds_bootstrap.h"
#include "upb/mem/arena.h"
#include "upb/reflection/def.h"

namespace grpc_core {

class XdsClient;

// Interface for an xDS resource type.
// Used to inject type-specific logic into XdsClient.
class XdsResourceType {
 public:
  // Context passed into Decode().
  struct DecodeContext {
    XdsClient* client;
    const XdsBootstrap::XdsServer& server;
    upb_DefPool* symtab;
    upb_Arena* arena;
  };

  // A base type for resource data.
  // Subclasses will extend this, and their DecodeResults will be
  // downcastable to their extended type.
  struct ResourceData {
    virtual ~ResourceData() = default;
  };

  // Result returned by Decode().
  struct DecodeResult {
    // The resource's name, if it can be determined.
    // If the name is not returned, the resource field should contain a
    // non-OK status.
    std::optional<std::string> name;
    // The parsed and validated resource, or an error status.
    absl::StatusOr<std::shared_ptr<const ResourceData>> resource;
  };

  virtual ~XdsResourceType() = default;

  // Returns v3 resource type.
  virtual absl::string_view type_url() const = 0;

  // Decodes and validates a serialized resource proto.
  virtual DecodeResult Decode(const DecodeContext& context,
                              absl::string_view serialized_resource) const = 0;

  // Returns true if r1 and r2 are equal.
  // Must be invoked only on resources returned by this object's Decode()
  // method.
  virtual bool ResourcesEqual(const ResourceData* r1,
                              const ResourceData* r2) const = 0;

  // Indicates whether the resource type requires that all resources must
  // be present in every SotW response from the server.  If true, a
  // response that does not include a previously seen resource will be
  // interpreted as a deletion of that resource.
  virtual bool AllResourcesRequiredInSotW() const { return false; }

  // Populate upb symtab with xDS proto messages that we want to print
  // properly in logs.
  // Note: This won't actually work properly until upb adds support for
  // Any fields in textproto printing (internal b/178821188).
  virtual void InitUpbSymtab(XdsClient*, upb_DefPool*) const {}
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_RESOURCE_TYPE_H
