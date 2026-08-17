//
//
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
//
//

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_TLS_UTILS_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_TLS_UTILS_H

#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>

#include <string>
#include <vector>

#include "absl/strings/string_view.h"

namespace grpc_core {

// Matches \a subject_alternative_name with \a matcher. Returns true if there
// is a match, false otherwise.
bool VerifySubjectAlternativeName(absl::string_view subject_alternative_name,
                                  const std::string& matcher);

// Returns value for the specified property_name from auth context. Here the
// property is expected to have a single value. Returns empty if multiple values
// are found.
absl::string_view GetAuthPropertyValue(grpc_auth_context* context,
                                       const char* property_name);

// Returns values for the specified property_name from auth context. Here the
// property can have any number of values.
std::vector<absl::string_view> GetAuthPropertyArray(grpc_auth_context* context,
                                                    const char* property_name);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_TLS_UTILS_H
