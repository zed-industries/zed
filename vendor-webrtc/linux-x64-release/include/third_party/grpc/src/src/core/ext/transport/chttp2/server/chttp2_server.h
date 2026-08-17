//
//
// Copyright 2016 gRPC authors.
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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_SERVER_CHTTP2_SERVER_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_SERVER_CHTTP2_SERVER_H

#include <grpc/byte_buffer.h>
#include <grpc/event_engine/event_engine.h>
#include <grpc/passive_listener.h>
#include <grpc/support/port_platform.h>

#include <functional>

#include "src/core/ext/transport/chttp2/transport/internal.h"
#include "src/core/handshaker/handshaker.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/server/server.h"

namespace grpc_core {

struct AcceptorDeleter {
  void operator()(grpc_tcp_server_acceptor* acceptor) const {
    if (acceptor != nullptr) {
      grpc_byte_buffer_destroy(acceptor->pending_data);
      gpr_free(acceptor);
    }
  }
};

class Chttp2ServerListener;

namespace testing {
class Chttp2ServerListenerTestPeer;
class ActiveConnectionTestPeer;
class HandshakingStateTestPeer;
}  // namespace testing

// New ChttpServerListener
class NewChttp2ServerListener : public Server::ListenerInterface {
 public:
  using AcceptorPtr =
      std::unique_ptr<grpc_tcp_server_acceptor, AcceptorDeleter>;

  // Keeps state for an individual connection. Lifetime: Internally refcounted
  // and owned by Server::ListenerState.
  class ActiveConnection : public LogicalConnection {
   public:
    // State for handshake. Lifetime: Owned by ActiveConnection and lasts while
    // the handshake is ongoing.
    class HandshakingState : public InternallyRefCounted<HandshakingState> {
     public:
      HandshakingState(RefCountedPtr<ActiveConnection> connection_ref,
                       grpc_tcp_server* tcp_server,
                       grpc_pollset* accepting_pollset, AcceptorPtr acceptor,
                       const ChannelArgs& args,
                       OrphanablePtr<grpc_endpoint> endpoint);

      ~HandshakingState() override;

      void Orphan() override;

      void StartLocked(const ChannelArgs& args);

      void ShutdownLocked(absl::Status status);

     private:
      friend class grpc_core::testing::HandshakingStateTestPeer;

      void OnTimeoutLocked();
      static void OnReceiveSettings(void* arg, grpc_error_handle /* error */);
      void OnHandshakeDoneLocked(absl::StatusOr<HandshakerArgs*> result);

      RefCountedPtr<ActiveConnection> const connection_;
      grpc_tcp_server* const tcp_server_;
      grpc_pollset* const accepting_pollset_;
      const AcceptorPtr acceptor_;
      grpc_pollset_set* const interested_parties_;
      Timestamp const deadline_;
      OrphanablePtr<grpc_endpoint> endpoint_;
      // Following fields are protected by WorkSerializer.
      RefCountedPtr<HandshakeManager> handshake_mgr_;
      // State for enforcing handshake timeout on receiving HTTP/2 settings.
      std::optional<grpc_event_engine::experimental::EventEngine::TaskHandle>
          timer_handle_;
      grpc_closure on_receive_settings_;
    };

    ActiveConnection(RefCountedPtr<Server::ListenerState> listener_state,
                     grpc_tcp_server* tcp_server,
                     grpc_pollset* accepting_pollset, AcceptorPtr acceptor,
                     const ChannelArgs& args, MemoryOwner memory_owner,
                     OrphanablePtr<grpc_endpoint> endpoint);
    void Start(const ChannelArgs& args);

    void SendGoAway() override;
    void DisconnectImmediately() override;

    void Orphan() override;

    // Needed to be able to grab an external weak ref in
    // NewChttp2ServerListener::OnAccept()
    using InternallyRefCounted<LogicalConnection>::RefAsSubclass;

   private:
    friend class grpc_core::testing::ActiveConnectionTestPeer;
    friend class grpc_core::testing::HandshakingStateTestPeer;

    static void OnClose(void* arg, grpc_error_handle error);

    void SendGoAwayImplLocked();
    void DisconnectImmediatelyImplLocked();

    RefCountedPtr<Server::ListenerState> const listener_state_;
    WorkSerializer work_serializer_;
    // Following fields are protected by WorkSerializer.
    // Set by HandshakingState before the handshaking begins and set to a valid
    // transport when handshaking is done successfully.
    std::variant<OrphanablePtr<HandshakingState>,
                 RefCountedPtr<grpc_chttp2_transport>>
        state_;
    grpc_closure on_close_;
    bool shutdown_ = false;
  };

  static grpc_error_handle Create(
      Server* server,
      const grpc_event_engine::experimental::EventEngine::ResolvedAddress& addr,
      const ChannelArgs& args, int* port_num);

  static grpc_error_handle CreateWithAcceptor(Server* server, const char* name,
                                              const ChannelArgs& args);

  static NewChttp2ServerListener* CreateForPassiveListener(
      Server* server, const ChannelArgs& args,
      std::shared_ptr<experimental::PassiveListenerImpl> passive_listener);

  // Do not instantiate directly.  Use one of the factory methods above.
  explicit NewChttp2ServerListener(
      const ChannelArgs& args,
      std::shared_ptr<experimental::PassiveListenerImpl> passive_listener =
          nullptr);
  ~NewChttp2ServerListener() override;

  void AcceptConnectedEndpoint(
      std::unique_ptr<grpc_event_engine::experimental::EventEngine::Endpoint>
          endpoint);

  channelz::ListenSocketNode* channelz_listen_socket_node() const override {
    return channelz_listen_socket_.get();
  }

  void SetServerListenerState(
      RefCountedPtr<Server::ListenerState> listener_state) override {
    listener_state_ = std::move(listener_state);
  }

  const grpc_resolved_address* resolved_address() const override {
    return &resolved_address_;
  }

  void SetOnDestroyDone(grpc_closure* on_destroy_done) override;

  void Orphan() override;

 private:
  friend class experimental::PassiveListenerImpl;

  // To allow access to RefCounted<> like interface.
  friend class RefCountedPtr<NewChttp2ServerListener>;

  friend class grpc_core::testing::Chttp2ServerListenerTestPeer;

  // Should only be called once so as to start the TCP server. This should
  // only be called by the config fetcher.
  void Start() override;

  static void OnAccept(void* arg, grpc_endpoint* tcp,
                       grpc_pollset* accepting_pollset,
                       grpc_tcp_server_acceptor* acceptor);

  static void TcpServerShutdownComplete(void* arg, grpc_error_handle error);

  static void DestroyListener(Server* /*server*/, void* arg,
                              grpc_closure* destroy_done);

  grpc_event_engine::experimental::EventEngine* event_engine() const {
    return listener_state_->server()
        ->channel_args()
        .GetObject<grpc_event_engine::experimental::EventEngine>();
  }

  grpc_tcp_server* tcp_server_ = nullptr;
  grpc_resolved_address resolved_address_;
  RefCountedPtr<Server::ListenerState> listener_state_;
  ChannelArgs args_;
  Mutex mu_;
  bool add_port_on_start_ ABSL_GUARDED_BY(mu_) = false;
  // Signals whether the application has triggered shutdown.
  bool shutdown_ ABSL_GUARDED_BY(mu_) = false;
  grpc_closure tcp_server_shutdown_complete_ ABSL_GUARDED_BY(mu_);
  grpc_closure* on_destroy_done_ ABSL_GUARDED_BY(mu_) = nullptr;
  RefCountedPtr<channelz::ListenSocketNode> channelz_listen_socket_;
  // TODO(yashykt): consider using std::variant<> to minimize memory usage for
  // disjoint cases where different fields are used.
  std::shared_ptr<experimental::PassiveListenerImpl> passive_listener_;
};

namespace experimental {

// An implementation of the public C++ passive listener interface.
// The server builder holds a weak_ptr to one of these objects, and the
// application owns the instance.
// TODO(yashykt): Move this to C-Core since this should be transport agnostic.
// Refer to https://github.com/grpc/grpc/pull/37601/files#r1803547924 for
// details.
class PassiveListenerImpl final : public PassiveListener {
 public:
  absl::Status AcceptConnectedEndpoint(
      std::unique_ptr<grpc_event_engine::experimental::EventEngine::Endpoint>
          endpoint) override ABSL_LOCKS_EXCLUDED(mu_);

  absl::Status AcceptConnectedFd(GRPC_UNUSED int fd) override
      ABSL_LOCKS_EXCLUDED(mu_);

  void ListenerDestroyed() ABSL_LOCKS_EXCLUDED(mu_);

 private:
  // note: the grpc_core::Server redundant namespace qualification is
  // required for older gcc versions.
  friend absl::Status(::grpc_server_add_passive_listener)(
      grpc_core::Server* server, grpc_server_credentials* credentials,
      std::shared_ptr<grpc_core::experimental::PassiveListenerImpl>
          passive_listener);

  Mutex mu_;
  // Data members will be populated when initialized.
  RefCountedPtr<Server> server_;
  std::variant<Chttp2ServerListener*, NewChttp2ServerListener*> listener_;
};

}  // namespace experimental

absl::StatusOr<int> Chttp2ServerAddPort(Server* server, const char* addr,
                                        const ChannelArgs& args);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_SERVER_CHTTP2_SERVER_H
