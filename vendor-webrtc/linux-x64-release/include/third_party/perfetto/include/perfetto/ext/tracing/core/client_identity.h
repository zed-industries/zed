/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_EXT_TRACING_CORE_CLIENT_IDENTITY_H_
#define INCLUDE_PERFETTO_EXT_TRACING_CORE_CLIENT_IDENTITY_H_

#include "perfetto/ext/base/sys_types.h"
#include "perfetto/ext/tracing/core/basic_types.h"

namespace perfetto {

// This class groups data fields of a connected client that can get passed in
// the tracing core to be emitted to trace packets.
class ClientIdentity {
 public:
  ClientIdentity() = default;
  ClientIdentity(uid_t uid, pid_t pid, MachineID machine_id = kDefaultMachineID)
      : uid_(uid), pid_(pid), machine_id_(machine_id) {}

  bool has_uid() const { return uid_ != base::kInvalidUid; }
  uid_t uid() const { return uid_; }

  bool has_pid() const { return pid_ != base::kInvalidPid; }
  pid_t pid() const { return pid_; }

  bool has_non_default_machine_id() const {
    return machine_id_ != kDefaultMachineID;
  }
  base::MachineID machine_id() const { return machine_id_; }

 private:
  uid_t uid_ = base::kInvalidUid;
  pid_t pid_ = base::kInvalidPid;
  MachineID machine_id_ = kDefaultMachineID;
};
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_TRACING_CORE_CLIENT_IDENTITY_H_
