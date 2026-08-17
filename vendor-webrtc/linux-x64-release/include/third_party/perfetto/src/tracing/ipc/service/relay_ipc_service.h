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

#ifndef SRC_TRACING_IPC_SERVICE_RELAY_IPC_SERVICE_H_
#define SRC_TRACING_IPC_SERVICE_RELAY_IPC_SERVICE_H_

#include <limits>
#include <list>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/sys_types.h"
#include "perfetto/ext/base/weak_ptr.h"
#include "perfetto/ext/ipc/basic_types.h"
#include "perfetto/ext/tracing/core/tracing_service.h"

#include "protos/perfetto/ipc/relay_port.ipc.h"

namespace perfetto {

// Implements the RelayPort IPC service.
class RelayIPCService : public protos::gen::RelayPort {
 public:
  explicit RelayIPCService(TracingService* core_service);
  ~RelayIPCService() override = default;

  void OnClientDisconnected() override;
  void InitRelay(const protos::gen::InitRelayRequest&,
                 DeferredInitRelayResponse) override;
  void SyncClock(const protos::gen::SyncClockRequest&,
                 DeferredSyncClockResponse) override;

 private:
  TracingService* const core_service_;

  using ClockSnapshots =
      base::FlatHashMap<uint32_t, std::pair<uint64_t, uint64_t>>;
  struct ClockSnapshotRecords {
    base::MachineID machine_id = base::kDefaultMachineID;

    // Keep track of most recent clock snapshots, ordered by local timestamps
    // (CLOCK_BOOTTIME).
    std::list<ClockSnapshots> clock_snapshots;

    uint64_t min_rtt = std::numeric_limits<uint64_t>::max();
  };

  TracingService::RelayEndpoint* GetRelayEndpoint(ipc::ClientID);

  base::FlatHashMap<ipc::ClientID,
                    std::unique_ptr<TracingService::RelayEndpoint>>
      relay_endpoints_;

  base::WeakPtrFactory<RelayIPCService> weak_ptr_factory_;  // Keep last.
};

}  // namespace perfetto

#endif  // SRC_TRACING_IPC_SERVICE_RELAY_IPC_SERVICE_H_
