/*
 * Copyright (C) 2021 The Android Open Source Project
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

#ifndef SRC_TOOLS_PROTO_MERGER_PROTO_MERGER_H_
#define SRC_TOOLS_PROTO_MERGER_PROTO_MERGER_H_

#include "perfetto/base/status.h"
#include "src/tools/proto_merger/allowlist.h"
#include "src/tools/proto_merger/proto_file.h"

namespace perfetto {
namespace proto_merger {

// Merges any updates in the proto |upstream| into the proto |input|
// optionally adding any messages/fields/enums/values specified in the
// the |allowlist|.
//
// Some notes about the merging algorithm:
// * Comments for all values are always taken from |upstream|.
// * If an enum is allowed, then so are all it's values.
// * Options for fields are always taken from |input|; any new options in
//   |upstream| are ignored.
// * Changing the type of an existing field is not supported (even if its
//   just a move); this needs to be handled manually.
base::Status MergeProtoFiles(const ProtoFile& input,
                             const ProtoFile& upstream,
                             const Allowlist& allowlist,
                             ProtoFile& out);

}  // namespace proto_merger
}  // namespace perfetto

#endif  // SRC_TOOLS_PROTO_MERGER_PROTO_MERGER_H_
