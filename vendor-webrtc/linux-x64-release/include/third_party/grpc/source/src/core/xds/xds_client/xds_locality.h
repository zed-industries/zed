//
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
//

#ifndef GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_LOCALITY_H
#define GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_LOCALITY_H

#include <string>
#include <utility>

#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "src/core/resolver/endpoint_addresses.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/ref_counted_string.h"
#include "src/core/util/useful.h"

namespace grpc_core {

// An xDS locality name.
class XdsLocalityName final : public RefCounted<XdsLocalityName> {
 public:
  struct Less {
    bool operator()(const XdsLocalityName* lhs,
                    const XdsLocalityName* rhs) const {
      if (lhs == nullptr || rhs == nullptr) return QsortCompare(lhs, rhs);
      return lhs->Compare(*rhs) < 0;
    }

    bool operator()(const RefCountedPtr<XdsLocalityName>& lhs,
                    const RefCountedPtr<XdsLocalityName>& rhs) const {
      return (*this)(lhs.get(), rhs.get());
    }
  };

  XdsLocalityName(std::string region, std::string zone, std::string sub_zone)
      : region_(std::move(region)),
        zone_(std::move(zone)),
        sub_zone_(std::move(sub_zone)),
        human_readable_string_(
            absl::StrFormat("{region=\"%s\", zone=\"%s\", sub_zone=\"%s\"}",
                            region_, zone_, sub_zone_)) {}

  bool operator==(const XdsLocalityName& other) const {
    return region_ == other.region_ && zone_ == other.zone_ &&
           sub_zone_ == other.sub_zone_;
  }

  bool operator!=(const XdsLocalityName& other) const {
    return !(*this == other);
  }

  int Compare(const XdsLocalityName& other) const {
    int cmp_result = region_.compare(other.region_);
    if (cmp_result != 0) return cmp_result;
    cmp_result = zone_.compare(other.zone_);
    if (cmp_result != 0) return cmp_result;
    return sub_zone_.compare(other.sub_zone_);
  }

  const std::string& region() const { return region_; }
  const std::string& zone() const { return zone_; }
  const std::string& sub_zone() const { return sub_zone_; }

  const RefCountedStringValue& human_readable_string() const {
    return human_readable_string_;
  }

  // Channel args traits.
  static absl::string_view ChannelArgName() {
    return GRPC_ARG_NO_SUBCHANNEL_PREFIX "xds_locality_name";
  }
  static int ChannelArgsCompare(const XdsLocalityName* a,
                                const XdsLocalityName* b) {
    return a->Compare(*b);
  }

 private:
  std::string region_;
  std::string zone_;
  std::string sub_zone_;
  RefCountedStringValue human_readable_string_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_LOCALITY_H
