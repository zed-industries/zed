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

#ifndef SRC_TRACE_REDACTION_FIND_PACKAGE_UID_H_
#define SRC_TRACE_REDACTION_FIND_PACKAGE_UID_H_

#include "src/trace_redaction/trace_redaction_framework.h"

#include "protos/perfetto/trace/trace_packet.pbzero.h"

namespace perfetto::trace_redaction {

// Writes the uid for the package matching `Context.package_name`. Returns
// `kStop` with a package is found, otherwrise `kContinue`. If a package is not
// found, `Context.package_uid` will remain unset and a later primitive will
// need to report the failure.
class FindPackageUid final : public CollectPrimitive {
 public:
  base::Status Begin(Context*) const override;

  base::Status Collect(const protos::pbzero::TracePacket::Decoder& packet,
                       Context* context) const override;

  base::Status End(Context*) const override;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_FIND_PACKAGE_UID_H_
