/*
 *  Copyright 2012 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_TEST_TURN_SERVER_H_
#define P2P_TEST_TURN_SERVER_H_

#include <cstddef>
#include <cstdint>
#include <list>
#include <map>
#include <memory>
#include <string>
#include <utility>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/packet_socket_factory.h"
#include "api/sequence_checker.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/task_queue/task_queue_base.h"
#include "api/transport/stun.h"
#include "api/units/time_delta.h"
#include "p2p/base/port_interface.h"
#include "rtc_base/async_packet_socket.h"
#include "rtc_base/byte_buffer.h"
#include "rtc_base/ip_address.h"
#include "rtc_base/network/received_packet.h"
#include "rtc_base/socket.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/ssl_adapter.h"
#include "rtc_base/third_party/sigslot/sigslot.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

constexpr int kMinTurnChannelNumber = 0x4000;
constexpr int kMaxTurnChannelNumber = 0x7FFF;

class TurnServer;

// The default server port for TURN, as specified in RFC5766.
const int TURN_SERVER_PORT = 3478;

// Encapsulates the client's connection to the server.
class TurnServerConnection {
 public:
  TurnServerConnection() : proto_(webrtc::PROTO_UDP), socket_(NULL) {}
  TurnServerConnection(const SocketAddress& src,
                       ProtocolType proto,
                       AsyncPacketSocket* socket);
  const SocketAddress& src() const { return src_; }
  AsyncPacketSocket* socket() { return socket_; }
  bool operator==(const TurnServerConnection& t) const;
  bool operator<(const TurnServerConnection& t) const;
  std::string ToString() const;

 private:
  SocketAddress src_;
  SocketAddress dst_;
  ProtocolType proto_;
  AsyncPacketSocket* socket_;
};

// Encapsulates a TURN allocation.
// The object is created when an allocation request is received, and then
// handles TURN messages (via HandleTurnMessage) and channel data messages
// (via HandleChannelData) for this allocation when received by the server.
// The object informs the server when its lifetime timer expires.
class TurnServerAllocation final {
 public:
  TurnServerAllocation(TurnServer* server_,
                       TaskQueueBase* thread,
                       const TurnServerConnection& conn,
                       AsyncPacketSocket* server_socket,
                       absl::string_view key);
  ~TurnServerAllocation();

  TurnServerConnection* conn() { return &conn_; }
  const std::string& key() const { return key_; }
  const std::string& transaction_id() const { return transaction_id_; }
  const std::string& username() const { return username_; }
  const std::string& last_nonce() const { return last_nonce_; }
  void set_last_nonce(absl::string_view nonce) {
    last_nonce_ = std::string(nonce);
  }

  std::string ToString() const;

  void HandleTurnMessage(const TurnMessage* msg);
  void HandleChannelData(ArrayView<const uint8_t> payload);

 private:
  struct Channel {
    ScopedTaskSafety pending_delete;
    const uint16_t id;
    const SocketAddress peer;
  };
  struct Permission {
    ScopedTaskSafety pending_delete;
    IPAddress peer;
  };
  using PermissionList = std::list<Permission>;
  using ChannelList = std::list<Channel>;

  void PostDeleteSelf(TimeDelta delay);

  void HandleAllocateRequest(const TurnMessage* msg);
  void HandleRefreshRequest(const TurnMessage* msg);
  void HandleSendIndication(const TurnMessage* msg);
  void HandleCreatePermissionRequest(const TurnMessage* msg);
  void HandleChannelBindRequest(const TurnMessage* msg);

  void OnExternalPacket(AsyncPacketSocket* socket,
                        const ReceivedIpPacket& packet);

  static TimeDelta ComputeLifetime(const TurnMessage& msg);
  bool HasPermission(const IPAddress& addr);
  void AddPermission(const IPAddress& addr);
  PermissionList::iterator FindPermission(const IPAddress& addr);
  ChannelList::iterator FindChannel(int channel_id);
  ChannelList::iterator FindChannel(const SocketAddress& addr);

  void SendResponse(TurnMessage* msg);
  void SendBadRequestResponse(const TurnMessage* req);
  void SendErrorResponse(const TurnMessage* req,
                         int code,
                         absl::string_view reason);
  void SendExternal(const void* data, size_t size, const SocketAddress& peer);

  TurnServer* const server_;
  TaskQueueBase* const thread_;
  TurnServerConnection conn_;
  std::unique_ptr<AsyncPacketSocket> external_socket_;
  std::string key_;
  std::string transaction_id_;
  std::string username_;
  std::string last_nonce_;
  PermissionList perms_;
  ChannelList channels_;
  ScopedTaskSafety safety_;
};

// An interface through which the MD5 credential hash can be retrieved.
class TurnAuthInterface {
 public:
  // Gets HA1 for the specified user and realm.
  // HA1 = MD5(A1) = MD5(username:realm:password).
  // Return true if the given username and realm are valid, or false if not.
  virtual bool GetKey(absl::string_view username,
                      absl::string_view realm,
                      std::string* key) = 0;
  virtual ~TurnAuthInterface() = default;
};

// An interface enables Turn Server to control redirection behavior.
class TurnRedirectInterface {
 public:
  virtual bool ShouldRedirect(const SocketAddress& address,
                              SocketAddress* out) = 0;
  virtual ~TurnRedirectInterface() {}
};

class StunMessageObserver {
 public:
  virtual void ReceivedMessage(const TurnMessage* msg) = 0;
  virtual void ReceivedChannelData(ArrayView<const uint8_t> payload) = 0;
  virtual ~StunMessageObserver() {}
};

// The core TURN server class. Give it a socket to listen on via
// AddInternalServerSocket, and a factory to create external sockets via
// SetExternalSocketFactory, and it's ready to go.
// Not yet wired up: TCP support.
class TurnServer : public sigslot::has_slots<> {
 public:
  typedef std::map<TurnServerConnection, std::unique_ptr<TurnServerAllocation>>
      AllocationMap;

  explicit TurnServer(TaskQueueBase* thread);
  ~TurnServer() override;

  // Gets/sets the realm value to use for the server.
  const std::string& realm() const {
    RTC_DCHECK_RUN_ON(thread_);
    return realm_;
  }
  void set_realm(absl::string_view realm) {
    RTC_DCHECK_RUN_ON(thread_);
    realm_ = std::string(realm);
  }

  // Gets/sets the value for the SOFTWARE attribute for TURN messages.
  const std::string& software() const {
    RTC_DCHECK_RUN_ON(thread_);
    return software_;
  }
  void set_software(absl::string_view software) {
    RTC_DCHECK_RUN_ON(thread_);
    software_ = std::string(software);
  }

  const AllocationMap& allocations() const {
    RTC_DCHECK_RUN_ON(thread_);
    return allocations_;
  }

  // Sets the authentication callback; does not take ownership.
  void set_auth_hook(TurnAuthInterface* auth_hook) {
    RTC_DCHECK_RUN_ON(thread_);
    auth_hook_ = auth_hook;
  }

  void set_redirect_hook(TurnRedirectInterface* redirect_hook) {
    RTC_DCHECK_RUN_ON(thread_);
    redirect_hook_ = redirect_hook;
  }

  void set_enable_otu_nonce(bool enable) {
    RTC_DCHECK_RUN_ON(thread_);
    enable_otu_nonce_ = enable;
  }

  // If set to true, reject CreatePermission requests to RFC1918 addresses.
  void set_reject_private_addresses(bool filter) {
    RTC_DCHECK_RUN_ON(thread_);
    reject_private_addresses_ = filter;
  }

  void set_reject_bind_requests(bool filter) {
    RTC_DCHECK_RUN_ON(thread_);
    reject_bind_requests_ = filter;
  }

  void set_enable_permission_checks(bool enable) {
    RTC_DCHECK_RUN_ON(thread_);
    enable_permission_checks_ = enable;
  }

  // Starts listening for packets from internal clients.
  void AddInternalSocket(AsyncPacketSocket* socket, ProtocolType proto);
  // Starts listening for the connections on this socket. When someone tries
  // to connect, the connection will be accepted and a new internal socket
  // will be added.
  void AddInternalServerSocket(
      Socket* socket,
      ProtocolType proto,
      std::unique_ptr<SSLAdapterFactory> ssl_adapter_factory = nullptr);
  // Specifies the factory to use for creating external sockets.
  void SetExternalSocketFactory(PacketSocketFactory* factory,
                                const SocketAddress& address);
  // For testing only.
  std::string SetTimestampForNextNonce(int64_t timestamp) {
    RTC_DCHECK_RUN_ON(thread_);
    ts_for_next_nonce_ = timestamp;
    return GenerateNonce(timestamp);
  }

  void SetStunMessageObserver(std::unique_ptr<StunMessageObserver> observer) {
    RTC_DCHECK_RUN_ON(thread_);
    stun_message_observer_ = std::move(observer);
  }

 private:
  // All private member functions and variables should have access restricted to
  // thread_. But compile-time annotations are missing for members access from
  // TurnServerAllocation (via friend declaration).

  std::string GenerateNonce(int64_t now) const RTC_RUN_ON(thread_);
  void OnInternalPacket(AsyncPacketSocket* socket,
                        const ReceivedIpPacket& packet) RTC_RUN_ON(thread_);

  void OnNewInternalConnection(Socket* socket);

  // Accept connections on this server socket.
  void AcceptConnection(Socket* server_socket) RTC_RUN_ON(thread_);
  void OnInternalSocketClose(AsyncPacketSocket* socket, int err);

  void HandleStunMessage(TurnServerConnection* conn,
                         ArrayView<const uint8_t> payload) RTC_RUN_ON(thread_);
  void HandleBindingRequest(TurnServerConnection* conn, const StunMessage* msg)
      RTC_RUN_ON(thread_);
  void HandleAllocateRequest(TurnServerConnection* conn,
                             const TurnMessage* msg,
                             absl::string_view key) RTC_RUN_ON(thread_);

  bool GetKey(const StunMessage* msg, std::string* key) RTC_RUN_ON(thread_);
  bool CheckAuthorization(TurnServerConnection* conn,
                          StunMessage* msg,
                          absl::string_view key) RTC_RUN_ON(thread_);
  bool ValidateNonce(absl::string_view nonce) const RTC_RUN_ON(thread_);

  TurnServerAllocation* FindAllocation(TurnServerConnection* conn)
      RTC_RUN_ON(thread_);
  TurnServerAllocation* CreateAllocation(TurnServerConnection* conn,
                                         int proto,
                                         absl::string_view key)
      RTC_RUN_ON(thread_);

  void SendErrorResponse(TurnServerConnection* conn,
                         const StunMessage* req,
                         int code,
                         absl::string_view reason);

  void SendErrorResponseWithRealmAndNonce(TurnServerConnection* conn,
                                          const StunMessage* req,
                                          int code,
                                          absl::string_view reason)
      RTC_RUN_ON(thread_);

  void SendErrorResponseWithAlternateServer(TurnServerConnection* conn,
                                            const StunMessage* req,
                                            const SocketAddress& addr)
      RTC_RUN_ON(thread_);

  void SendStun(TurnServerConnection* conn, StunMessage* msg);
  void Send(TurnServerConnection* conn, const ByteBufferWriter& buf);

  void DestroyAllocation(TurnServerAllocation* allocation) RTC_RUN_ON(thread_);
  void DestroyInternalSocket(AsyncPacketSocket* socket) RTC_RUN_ON(thread_);

  typedef std::map<AsyncPacketSocket*, ProtocolType> InternalSocketMap;
  struct ServerSocketInfo {
    ProtocolType proto;
    // If non-null, used to wrap accepted sockets.
    std::unique_ptr<SSLAdapterFactory> ssl_adapter_factory;
  };
  typedef std::map<Socket*, ServerSocketInfo> ServerSocketMap;

  TaskQueueBase* const thread_;
  const std::string nonce_key_;
  std::string realm_ RTC_GUARDED_BY(thread_);
  std::string software_ RTC_GUARDED_BY(thread_);
  TurnAuthInterface* auth_hook_ RTC_GUARDED_BY(thread_);
  TurnRedirectInterface* redirect_hook_ RTC_GUARDED_BY(thread_);
  // otu - one-time-use. Server will respond with 438 if it's
  // sees the same nonce in next transaction.
  bool enable_otu_nonce_ RTC_GUARDED_BY(thread_);
  bool reject_private_addresses_ RTC_GUARDED_BY(thread_) = false;
  bool reject_bind_requests_ RTC_GUARDED_BY(thread_) = false;
  // Check for permission when receiving an external packet.
  bool enable_permission_checks_ = true;

  InternalSocketMap server_sockets_ RTC_GUARDED_BY(thread_);
  ServerSocketMap server_listen_sockets_ RTC_GUARDED_BY(thread_);
  std::unique_ptr<PacketSocketFactory> external_socket_factory_
      RTC_GUARDED_BY(thread_);
  SocketAddress external_addr_ RTC_GUARDED_BY(thread_);

  AllocationMap allocations_ RTC_GUARDED_BY(thread_);

  // For testing only. If this is non-zero, the next NONCE will be generated
  // from this value, and it will be reset to 0 after generating the NONCE.
  int64_t ts_for_next_nonce_ RTC_GUARDED_BY(thread_) = 0;

  // For testing only. Used to observe STUN messages received.
  std::unique_ptr<StunMessageObserver> stun_message_observer_
      RTC_GUARDED_BY(thread_);

  friend class TurnServerAllocation;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::kMaxTurnChannelNumber;
using ::webrtc::kMinTurnChannelNumber;
using ::webrtc::StunMessageObserver;
using ::webrtc::TURN_SERVER_PORT;
using ::webrtc::TurnAuthInterface;
using ::webrtc::TurnRedirectInterface;
using ::webrtc::TurnServer;
using ::webrtc::TurnServerAllocation;
using ::webrtc::TurnServerConnection;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_TEST_TURN_SERVER_H_
