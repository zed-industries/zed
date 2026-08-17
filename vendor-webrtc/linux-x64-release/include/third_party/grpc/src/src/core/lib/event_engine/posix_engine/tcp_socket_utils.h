// Copyright 2022 The gRPC Authors
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

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_TCP_SOCKET_UTILS_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_TCP_SOCKET_UTILS_H

#include <grpc/event_engine/endpoint_config.h>
#include <grpc/event_engine/event_engine.h>
#include <grpc/event_engine/memory_allocator.h>
#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>

#include <functional>
#include <string>
#include <utility>

#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "src/core/lib/iomgr/port.h"
#include "src/core/lib/iomgr/socket_mutator.h"
#include "src/core/lib/resource_quota/resource_quota.h"
#include "src/core/util/ref_counted_ptr.h"

#ifdef GRPC_POSIX_SOCKET_UTILS_COMMON
#include <sys/socket.h>
#endif

#ifdef GRPC_LINUX_ERRQUEUE
#ifndef SO_ZEROCOPY
#define SO_ZEROCOPY 60
#endif
#ifndef SO_EE_ORIGIN_ZEROCOPY
#define SO_EE_ORIGIN_ZEROCOPY 5
#endif
#endif  // ifdef GRPC_LINUX_ERRQUEUE

namespace grpc_event_engine::experimental {

struct PosixTcpOptions {
  static constexpr int kDefaultReadChunkSize = 8192;
  static constexpr int kDefaultMinReadChunksize = 256;
  static constexpr int kDefaultMaxReadChunksize = 4 * 1024 * 1024;
  static constexpr int kZerocpTxEnabledDefault = 0;
  static constexpr int kMaxChunkSize = 32 * 1024 * 1024;
  static constexpr int kDefaultMaxSends = 4;
  static constexpr size_t kDefaultSendBytesThreshold = 16 * 1024;
  // Let the system decide the proper buffer size.
  static constexpr int kReadBufferSizeUnset = -1;
  static constexpr int kDscpNotSet = -1;
  int tcp_read_chunk_size = kDefaultReadChunkSize;
  int tcp_min_read_chunk_size = kDefaultMinReadChunksize;
  int tcp_max_read_chunk_size = kDefaultMaxReadChunksize;
  int tcp_tx_zerocopy_send_bytes_threshold = kDefaultSendBytesThreshold;
  int tcp_tx_zerocopy_max_simultaneous_sends = kDefaultMaxSends;
  int tcp_receive_buffer_size = kReadBufferSizeUnset;
  bool tcp_tx_zero_copy_enabled = kZerocpTxEnabledDefault;
  int keep_alive_time_ms = 0;
  int keep_alive_timeout_ms = 0;
  bool expand_wildcard_addrs = false;
  bool allow_reuse_port = false;
  int dscp = kDscpNotSet;
  grpc_core::RefCountedPtr<grpc_core::ResourceQuota> resource_quota;
  struct grpc_socket_mutator* socket_mutator = nullptr;
  grpc_event_engine::experimental::MemoryAllocatorFactory*
      memory_allocator_factory = nullptr;
  PosixTcpOptions() = default;
  // Move ctor
  PosixTcpOptions(PosixTcpOptions&& other) noexcept {
    socket_mutator = std::exchange(other.socket_mutator, nullptr);
    resource_quota = std::move(other.resource_quota);
    CopyIntegerOptions(other);
  }
  // Move assignment
  PosixTcpOptions& operator=(PosixTcpOptions&& other) noexcept {
    if (socket_mutator != nullptr) {
      grpc_socket_mutator_unref(socket_mutator);
    }
    socket_mutator = std::exchange(other.socket_mutator, nullptr);
    resource_quota = std::move(other.resource_quota);
    memory_allocator_factory =
        std::exchange(other.memory_allocator_factory, nullptr);
    CopyIntegerOptions(other);
    return *this;
  }
  // Copy ctor
  PosixTcpOptions(const PosixTcpOptions& other) {
    if (other.socket_mutator != nullptr) {
      socket_mutator = grpc_socket_mutator_ref(other.socket_mutator);
    }
    resource_quota = other.resource_quota;
    memory_allocator_factory = other.memory_allocator_factory;
    CopyIntegerOptions(other);
  }
  // Copy assignment
  PosixTcpOptions& operator=(const PosixTcpOptions& other) {
    if (&other == this) {
      return *this;
    }
    if (socket_mutator != nullptr) {
      grpc_socket_mutator_unref(socket_mutator);
      socket_mutator = nullptr;
    }
    if (other.socket_mutator != nullptr) {
      socket_mutator = grpc_socket_mutator_ref(other.socket_mutator);
    }
    resource_quota = other.resource_quota;
    memory_allocator_factory = other.memory_allocator_factory;
    CopyIntegerOptions(other);
    return *this;
  }
  // Destructor.
  ~PosixTcpOptions() {
    if (socket_mutator != nullptr) {
      grpc_socket_mutator_unref(socket_mutator);
    }
  }

 private:
  void CopyIntegerOptions(const PosixTcpOptions& other) {
    tcp_read_chunk_size = other.tcp_read_chunk_size;
    tcp_min_read_chunk_size = other.tcp_min_read_chunk_size;
    tcp_max_read_chunk_size = other.tcp_max_read_chunk_size;
    tcp_tx_zerocopy_send_bytes_threshold =
        other.tcp_tx_zerocopy_send_bytes_threshold;
    tcp_tx_zerocopy_max_simultaneous_sends =
        other.tcp_tx_zerocopy_max_simultaneous_sends;
    tcp_tx_zero_copy_enabled = other.tcp_tx_zero_copy_enabled;
    keep_alive_time_ms = other.keep_alive_time_ms;
    keep_alive_timeout_ms = other.keep_alive_timeout_ms;
    expand_wildcard_addrs = other.expand_wildcard_addrs;
    allow_reuse_port = other.allow_reuse_port;
    dscp = other.dscp;
  }
};

PosixTcpOptions TcpOptionsFromEndpointConfig(
    const grpc_event_engine::experimental::EndpointConfig& config);

// a wrapper for accept or accept4
int Accept4(int sockfd,
            grpc_event_engine::experimental::EventEngine::ResolvedAddress& addr,
            int nonblock, int cloexec);

// Unlink the path pointed to by the given address if it refers to UDS path.
void UnlinkIfUnixDomainSocket(
    const EventEngine::ResolvedAddress& resolved_addr);

class PosixSocketWrapper {
 public:
  explicit PosixSocketWrapper(int fd) : fd_(fd) { CHECK_GT(fd_, 0); }

  PosixSocketWrapper() : fd_(-1) {};

  ~PosixSocketWrapper() = default;

  // Instruct the kernel to wait for specified number of bytes to be received on
  // the socket before generating an interrupt for packet receive. If the call
  // succeeds, it returns the number of bytes (wait threshold) that was actually
  // set.
  absl::StatusOr<int> SetSocketRcvLowat(int bytes);

  // Set socket to use zerocopy
  absl::Status SetSocketZeroCopy();

  // Set socket to non blocking mode
  absl::Status SetSocketNonBlocking(int non_blocking);

  // Set socket to close on exec
  absl::Status SetSocketCloexec(int close_on_exec);

  // Set socket to reuse old addresses
  absl::Status SetSocketReuseAddr(int reuse);

  // Disable nagle algorithm
  absl::Status SetSocketLowLatency(int low_latency);

  // Set SO_REUSEPORT
  absl::Status SetSocketReusePort(int reuse);

  // Set Differentiated Services Code Point (DSCP)
  absl::Status SetSocketDscp(int dscp);

  // Override default Tcp user timeout values if necessary.
  void TrySetSocketTcpUserTimeout(const PosixTcpOptions& options,
                                  bool is_client);

  // Tries to set SO_NOSIGPIPE if available on this platform.
  // If SO_NO_SIGPIPE is not available, returns not OK status.
  absl::Status SetSocketNoSigpipeIfPossible();

  // Tries to set IP_PKTINFO if available on this platform. If IP_PKTINFO is not
  // available, returns not OK status.
  absl::Status SetSocketIpPktInfoIfPossible();

  // Tries to set IPV6_RECVPKTINFO if available on this platform. If
  // IPV6_RECVPKTINFO is not available, returns not OK status.
  absl::Status SetSocketIpv6RecvPktInfoIfPossible();

  // Tries to set the socket's send buffer to given size.
  absl::Status SetSocketSndBuf(int buffer_size_bytes);

  // Tries to set the socket's receive buffer to given size.
  absl::Status SetSocketRcvBuf(int buffer_size_bytes);

  // Tries to set the socket using a grpc_socket_mutator
  absl::Status SetSocketMutator(grpc_fd_usage usage,
                                grpc_socket_mutator* mutator);

  // Extracts the first socket mutator from config if any and applies on the fd.
  absl::Status ApplySocketMutatorInOptions(grpc_fd_usage usage,
                                           const PosixTcpOptions& options);

  // Return LocalAddress as EventEngine::ResolvedAddress
  absl::StatusOr<EventEngine::ResolvedAddress> LocalAddress();

  // Return PeerAddress as EventEngine::ResolvedAddress
  absl::StatusOr<EventEngine::ResolvedAddress> PeerAddress();

  // Return LocalAddress as string
  absl::StatusOr<std::string> LocalAddressString();

  // Return PeerAddress as string
  absl::StatusOr<std::string> PeerAddressString();

  // An enum to keep track of IPv4/IPv6 socket modes.

  // Currently, this information is only used when a socket is first created,
  // but in the future we may wish to store it alongside the fd.  This would let
  // calls like sendto() know which family to use without asking the kernel
  // first.
  enum DSMode {
    // Uninitialized, or a non-IP socket.
    DSMODE_NONE,
    // AF_INET only.
    DSMODE_IPV4,
    // AF_INET6 only, because IPV6_V6ONLY could not be cleared.
    DSMODE_IPV6,
    // AF_INET6, which also supports ::ffff-mapped IPv4 addresses.
    DSMODE_DUALSTACK
  };

  // Returns the underlying file-descriptor.
  int Fd() const { return fd_; }

  // Static methods

  // Configure default values for tcp user timeout to be used by client
  // and server side sockets.
  static void ConfigureDefaultTcpUserTimeout(bool enable, int timeout,
                                             bool is_client);

  // Return true if SO_REUSEPORT is supported
  static bool IsSocketReusePortSupported();

  // Returns true if this system can create AF_INET6 sockets bound to ::1.
  // The value is probed once, and cached for the life of the process.

  // This is more restrictive than checking for socket(AF_INET6) to succeed,
  // because Linux with "net.ipv6.conf.all.disable_ipv6 = 1" is able to create
  // and bind IPv6 sockets, but cannot connect to a getsockname() of [::]:port
  // without a valid loopback interface.  Rather than expose this half-broken
  // state to library users, we turn off IPv6 sockets.
  static bool IsIpv6LoopbackAvailable();

  // Creates a new socket for connecting to (or listening on) an address.

  // If addr is AF_INET6, this creates an IPv6 socket first.  If that fails,
  // and addr is within ::ffff:0.0.0.0/96, then it automatically falls back to
  // an IPv4 socket.

  // If addr is AF_INET, AF_UNIX, or anything else, then this is similar to
  // calling socket() directly.

  // Returns an PosixSocketWrapper on success, otherwise returns a not-OK
  // absl::Status

  // The dsmode output indicates which address family was actually created.
  static absl::StatusOr<PosixSocketWrapper> CreateDualStackSocket(
      std::function<int(int /*domain*/, int /*type*/, int /*protocol*/)>
          socket_factory,
      const experimental::EventEngine::ResolvedAddress& addr, int type,
      int protocol, DSMode& dsmode);

  struct PosixSocketCreateResult;
  // Return a PosixSocketCreateResult which manages a configured, unbound,
  // unconnected TCP client fd.
  //  options: may contain custom tcp settings for the fd.
  //  target_addr: the destination address.
  //
  // Returns: Not-OK status on error. Otherwise it returns a
  // PosixSocketWrapper::PosixSocketCreateResult type which includes a sock
  // of type PosixSocketWrapper and a mapped_target_addr which is
  // target_addr mapped to an address appropriate to the type of socket FD
  // created. For example, if target_addr is IPv4 and dual stack sockets are
  // available, mapped_target_addr will be an IPv4-mapped IPv6 address.
  //
  static absl::StatusOr<PosixSocketCreateResult>
  CreateAndPrepareTcpClientSocket(
      const PosixTcpOptions& options,
      const EventEngine::ResolvedAddress& target_addr);

 private:
  int fd_;
};

struct PosixSocketWrapper::PosixSocketCreateResult {
  PosixSocketWrapper sock;
  EventEngine::ResolvedAddress mapped_target_addr;
};

bool SetSocketDualStack(int fd);

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_TCP_SOCKET_UTILS_H
