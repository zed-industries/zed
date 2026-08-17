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

#ifndef SRC_TRACING_IPC_PRODUCER_RELAY_IPC_CLIENT_H_
#define SRC_TRACING_IPC_PRODUCER_RELAY_IPC_CLIENT_H_

#include "perfetto/ext/base/thread_checker.h"
#include "perfetto/ext/base/weak_ptr.h"
#include "perfetto/ext/ipc/client.h"
#include "perfetto/ext/ipc/service_proxy.h"
#include "perfetto/ext/tracing/core/tracing_service.h"

#include "protos/perfetto/ipc/relay_port.ipc.h"

namespace perfetto {

namespace base {
class TaskRunner;
}  // namespace base

// Exposes a Service endpoint to the relay service, proxying all requests
// through an IPC channel to the remote Service. This class is the glue layer
// between the generic Service interface exposed to the clients of the library
// and the actual IPC transport.
class RelayIPCClient : public ipc::ServiceProxy::EventListener {
 public:
  class EventListener {
   public:
    virtual ~EventListener();
    // Called when the client receives the response of the SyncClock() request.
    virtual void OnSyncClockResponse(const SyncClockResponse&) = 0;
    // Called when the IPC service is connected and is ready for the SyncClock()
    // requests.
    virtual void OnServiceConnected() = 0;
    // Called when the IPC service is disconnected.
    virtual void OnServiceDisconnected() = 0;
  };

  using SyncClockCallback = std::function<void(const SyncClockResponse&)>;
  using OnDisconnectCallback = std::function<void()>;

  RelayIPCClient(ipc::Client::ConnArgs,
                 base::WeakPtr<EventListener>,
                 base::TaskRunner*);
  ~RelayIPCClient() override;

  void InitRelay(const InitRelayRequest&);
  void SyncClock(const SyncClockRequest&);

  // ipc::ServiceProxy::EventListener implementation.
  // These methods are invoked by the IPC layer, which knows nothing about
  // tracing, producers and consumers.
  void OnConnect() override;
  void OnDisconnect() override;

 private:
  base::WeakPtr<EventListener> listener_;

  base::TaskRunner* const task_runner_;

  // The object that owns the client socket and takes care of IPC traffic.
  std::unique_ptr<ipc::Client> ipc_channel_;

  // The proxy interface for the producer port of the service. It is bound
  // to |ipc_channel_| and (de)serializes method invocations over the wire.
  std::unique_ptr<protos::gen::RelayPortProxy> relay_proxy_;

  bool connected_ = false;
  PERFETTO_THREAD_CHECKER(thread_checker_)
};

}  // namespace perfetto

#endif  // SRC_TRACING_IPC_PRODUCER_RELAY_IPC_CLIENT_H_
