//
// Copyright 2019 gRPC authors.
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

#ifndef GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_BOOTSTRAP_H
#define GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_BOOTSTRAP_H

#include <grpc/support/port_platform.h>

#include <string>

#include "src/core/util/json/json.h"

namespace grpc_core {

bool XdsFederationEnabled();
bool XdsDataErrorHandlingEnabled();

class XdsBootstrap {
 public:
  class Node {
   public:
    virtual ~Node() = default;

    virtual const std::string& id() const = 0;
    virtual const std::string& cluster() const = 0;
    virtual const std::string& locality_region() const = 0;
    virtual const std::string& locality_zone() const = 0;
    virtual const std::string& locality_sub_zone() const = 0;
    virtual const Json::Object& metadata() const = 0;
  };

  class XdsServerTarget {
   public:
    virtual ~XdsServerTarget() = default;
    virtual const std::string& server_uri() const = 0;
    // Returns a key to be used for uniquely identifying this XdsServerTarget.
    virtual std::string Key() const = 0;
    virtual bool Equals(const XdsServerTarget& other) const = 0;
    friend bool operator==(const XdsServerTarget& a, const XdsServerTarget& b) {
      return a.Equals(b);
    }
    friend bool operator!=(const XdsServerTarget& a, const XdsServerTarget& b) {
      return !a.Equals(b);
    }
  };

  class XdsServer {
   public:
    virtual ~XdsServer() = default;

    virtual std::shared_ptr<const XdsServerTarget> target() const = 0;

    // TODO(roth): Remove this method once the data error handling
    // feature passes interop tests.
    virtual bool IgnoreResourceDeletion() const = 0;

    virtual bool FailOnDataErrors() const = 0;
    virtual bool ResourceTimerIsTransientFailure() const = 0;

    virtual bool Equals(const XdsServer& other) const = 0;

    // Returns a key to be used for uniquely identifying this XdsServer.
    virtual std::string Key() const = 0;

    friend bool operator==(const XdsServer& a, const XdsServer& b) {
      return a.Equals(b);
    }
    friend bool operator!=(const XdsServer& a, const XdsServer& b) {
      return !a.Equals(b);
    }
  };

  class Authority {
   public:
    virtual ~Authority() = default;

    virtual std::vector<const XdsServer*> servers() const = 0;
  };

  virtual ~XdsBootstrap() = default;

  virtual std::string ToString() const = 0;

  virtual std::vector<const XdsServer*> servers() const = 0;

  // Returns the node information, or null if not present in the bootstrap
  // config.
  virtual const Node* node() const = 0;

  // Returns a pointer to the specified authority, or null if it does
  // not exist in this bootstrap config.
  virtual const Authority* LookupAuthority(const std::string& name) const = 0;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_BOOTSTRAP_H
