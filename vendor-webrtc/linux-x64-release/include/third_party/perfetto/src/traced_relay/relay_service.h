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

#ifndef SRC_TRACED_RELAY_RELAY_SERVICE_H_
#define SRC_TRACED_RELAY_RELAY_SERVICE_H_

#include <memory>
#include <vector>

#include "perfetto/ext/base/unix_socket.h"
#include "perfetto/ext/tracing/core/tracing_service.h"
#include "src/traced_relay/socket_relay_handler.h"
#include "src/tracing/ipc/producer/relay_ipc_client.h"

namespace perfetto {

namespace base {
class TaskRunner;
}  // namespace base.

// RelayClient provides a service that is independent of the relayed producers
// and is global in the machine. This class implements time synchronization with
// the host machine:
// 1. Connects to the host machine using the client socket name (e.g.
// vsock://2:10001) to port 10001 of VMADDR_CID_HOST.
// 2. After the socket is connected, send the SetPeerIdentity message to let the
// tracing service know the identity (machine ID) of this RelayClient.
// 3. Then hand over the socket to RelayIPCClient, which is the client
// implementation of the RelayPort RPC service.
// 4. On any socket error, the RelayClient notifies its user using
// OnErrorCallback so the user (class RelayService) may retry the connection.
class RelayClient : private base::UnixSocket::EventListener,
                    private RelayIPCClient::EventListener {
 public:
  using OnErrorCallback = std::function<void()>;
  RelayClient(const std::string& client_sock_name,
              const std::string& machine_id_hint,
              base::TaskRunner* task_runner,
              OnErrorCallback on_destroy_callback);
  ~RelayClient() override;

  bool ipc_client_connected() const { return phase_ != Phase::CONNECTING; }
  bool clock_synced_with_service_for_testing() const {
    return clock_synced_with_service_for_testing_;
  }

 private:
  // UnixSocket::EventListener implementation for connecting to the client
  // socket.
  void OnNewIncomingConnection(base::UnixSocket*,
                               std::unique_ptr<base::UnixSocket>) override {
    // This class doesn't open a socket in listening mode.
    PERFETTO_DFATAL("Should be unreachable.");
  }
  void OnConnect(base::UnixSocket* self, bool connected) override;
  void OnDisconnect(base::UnixSocket*) override { NotifyError(); }
  void OnDataAvailable(base::UnixSocket*) override {
    // Should be handled in the IPC client.
    PERFETTO_DFATAL("Should be unreachable.");
  }

  void NotifyError();
  void Connect();
  void InitRelayRequest();
  void SendSyncClockRequest();

  // RelayIPCClient::EventListener implementation.
  void OnServiceConnected() override;
  void OnServiceDisconnected() override;
  void OnSyncClockResponse(const protos::gen::SyncClockResponse& resp) override;

  enum class Phase : uint32_t { CONNECTING = 1, PING, UPDATE };
  Phase phase_ = Phase::CONNECTING;
  bool clock_synced_with_service_for_testing_ = false;

  base::TaskRunner* task_runner_;
  OnErrorCallback on_error_callback_;

  std::string client_sock_name_;
  // A hint to the host traced for inferring the identifier of this machine.
  std::string machine_id_hint_;
  std::unique_ptr<base::UnixSocket> client_sock_;
  std::unique_ptr<RelayIPCClient> relay_ipc_client_;

  base::WeakPtrFactory<RelayIPCClient::EventListener>
      weak_factory_for_ipc_client{this};
  base::WeakPtrFactory<RelayClient> weak_factory_{this};
};

// A class for relaying the producer data between the local producers and the
// remote tracing service.
class RelayService : public base::UnixSocket::EventListener {
 public:
  explicit RelayService(base::TaskRunner* task_runner);
  ~RelayService() override = default;

  // Starts the service relay that forwards messages between the
  // |server_socket_name| and |client_socket_name| ports.
  void Start(const char* server_socket_name, std::string client_socket_name);

  // Starts the service relay that forwards messages between the
  // |server_socket_handle| and |client_socket_name| ports. Called when the
  // service is started by Android init.
  void Start(base::ScopedSocketHandle server_socket_handle,
             std::string client_socket_name);

  static std::string GetMachineIdHint(
      bool use_pseudo_boot_id_for_testing = false);

  void SetRelayClientDisabledForTesting(bool disabled) {
    relay_client_disabled_for_testing_ = disabled;
  }
  void SetMachineIdHintForTesting(std::string machine_id_hint) {
    machine_id_hint_ = machine_id_hint;
  }
  RelayClient* relay_client_for_testing() { return relay_client_.get(); }

 private:
  struct PendingConnection {
    // This keeps a connected UnixSocketRaw server socket in its first element.
    std::unique_ptr<SocketPair> socket_pair;
    // This keeps the connecting client connection.
    std::unique_ptr<base::UnixSocket> connecting_client_conn;
  };

  RelayService(const RelayService&) = delete;
  RelayService& operator=(const RelayService&) = delete;

  // UnixSocket::EventListener implementation.
  void OnNewIncomingConnection(base::UnixSocket*,
                               std::unique_ptr<base::UnixSocket>) override;
  void OnConnect(base::UnixSocket* self, bool connected) override;
  void OnDisconnect(base::UnixSocket* self) override;
  void OnDataAvailable(base::UnixSocket* self) override;

  void ReconnectRelayClient();
  void ConnectRelayClient();

  base::TaskRunner* const task_runner_ = nullptr;

  // A hint to the host traced for inferring the identifier of this machine.
  std::string machine_id_hint_;

  std::unique_ptr<base::UnixSocket> listening_socket_;
  std::string client_socket_name_;

  // Keeps the socket pairs while waiting for relay connections to be
  // established.
  std::vector<PendingConnection> pending_connections_;

  SocketRelayHandler socket_relay_handler_;

  std::unique_ptr<RelayClient> relay_client_;
  // On RelayClient connection error, how long should we wait before retrying.
  static constexpr uint32_t kDefaultRelayClientRetryDelayMs = 1000;
  uint32_t relay_client_retry_delay_ms_ = kDefaultRelayClientRetryDelayMs;
  bool relay_client_disabled_for_testing_ = false;
};

}  // namespace perfetto

#endif  // SRC_TRACED_RELAY_RELAY_SERVICE_H_
