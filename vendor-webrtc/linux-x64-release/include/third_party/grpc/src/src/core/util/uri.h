//
// Copyright 2015 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_URI_H
#define GRPC_SRC_CORE_UTIL_URI_H

#include <grpc/support/port_platform.h>

#include <map>
#include <string>
#include <vector>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"

namespace grpc_core {

class URI {
 public:
  struct QueryParam {
    std::string key;
    std::string value;
    bool operator==(const QueryParam& other) const {
      return key == other.key && value == other.value;
    }
    bool operator<(const QueryParam& other) const {
      int c = key.compare(other.key);
      if (c != 0) return c < 0;
      return value < other.value;
    }
  };

  // Creates a URI by parsing an rfc3986 URI string. Returns an
  // InvalidArgumentError on failure.
  static absl::StatusOr<URI> Parse(absl::string_view uri_text);
  // Creates a URI from components. Returns an InvalidArgumentError on failure.
  static absl::StatusOr<URI> Create(
      std::string scheme, std::string user_info, std::string host_port,
      std::string path, std::vector<QueryParam> query_parameter_pairs,
      std::string fragment);

  URI() = default;

  // Copy construction and assignment
  URI(const URI& other);
  URI& operator=(const URI& other);
  // Move construction and assignment
  URI(URI&&) = default;
  URI& operator=(URI&&) = default;

  static std::string PercentEncodeAuthority(absl::string_view str);
  static std::string PercentEncodePath(absl::string_view str);

  static std::string PercentDecode(absl::string_view str);

  const std::string& scheme() const { return scheme_; }
  std::string authority() const;
  const std::string& user_info() const { return user_info_; }
  const std::string& host_port() const { return host_port_; }
  const std::string& path() const { return path_; }
  // Stores the *last* value appearing for each repeated key in the query
  // string. If you need to capture repeated query parameters, use
  // `query_parameter_pairs`.
  const std::map<absl::string_view, absl::string_view>& query_parameter_map()
      const {
    return query_parameter_map_;
  }
  // A vector of key:value query parameter pairs, kept in order of appearance
  // within the URI string. Repeated keys are represented as separate
  // key:value elements.
  const std::vector<QueryParam>& query_parameter_pairs() const {
    return query_parameter_pairs_;
  }
  const std::string& fragment() const { return fragment_; }

  std::string ToString() const;

  // Returns the encoded path and query params, such as would be used on
  // the wire in an HTTP request.
  std::string EncodedPathAndQueryParams() const;

  bool operator==(const URI& other) const {
    return scheme_ == other.scheme_ && user_info_ == other.user_info_ &&
           host_port_ == other.host_port_ && path_ == other.path_ &&
           query_parameter_pairs_ == other.query_parameter_pairs_ &&
           fragment_ == other.fragment_;
  }

 private:
  URI(std::string scheme, std::string user_info, std::string host_port,
      std::string path, std::vector<QueryParam> query_parameter_pairs,
      std::string fragment);

  std::string scheme_;
  std::string user_info_;
  std::string host_port_;
  std::string path_;
  std::map<absl::string_view, absl::string_view> query_parameter_map_;
  std::vector<QueryParam> query_parameter_pairs_;
  std::string fragment_;
};
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_URI_H
