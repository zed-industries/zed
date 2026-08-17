/*
 * Copyright (C) 2024 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_REDACTION_PRUNE_PACKAGE_LIST_H_
#define SRC_TRACE_REDACTION_PRUNE_PACKAGE_LIST_H_

#include <string>

#include "perfetto/base/status.h"
#include "src/trace_redaction/trace_redaction_framework.h"

namespace perfetto::trace_redaction {

// Removes all package list entries that don't match `Context.package_uid`.
// Returns `base::ErrStatus()` if `Context.package_uid` was not set.
class PrunePackageList final : public TransformPrimitive {
 public:
  base::Status Transform(const Context& context,
                         std::string* packet) const override;

 private:
  void OnPackageList(const Context& context,
                     protozero::ConstBytes bytes,
                     protos::pbzero::PackagesList* message) const;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_PRUNE_PACKAGE_LIST_H_
