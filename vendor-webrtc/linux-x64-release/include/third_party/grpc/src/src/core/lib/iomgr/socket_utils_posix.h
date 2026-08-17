//
//
// Copyright 2015 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_SOCKET_UTILS_POSIX_H
#define GRPC_SRC_CORE_LIB_IOMGR_SOCKET_UTILS_POSIX_H

#include <grpc/event_engine/endpoint_config.h>
#include <grpc/event_engine/event_engine.h>
#include <grpc/impl/grpc_types.h>
#include <grpc/support/port_platform.h>

#include <cstddef>
#include <memory>
#include <utility>

#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/resolved_address.h"
#include "src/core/lib/iomgr/socket_mutator.h"
#include "src/core/lib/resource_quota/resource_quota.h"
#include "src/core/util/ref_counted_ptr.h"

#ifdef GRPC_LINUX_ERRQUEUE
#ifndef SO_ZEROCOPY
#define SO_ZEROCOPY 60
#endif
#ifndef SO_EE_ORIGIN_ZEROCOPY
#define SO_EE_ORIGIN_ZEROCOPY 5
#endif
#endif  // ifdef GRPC_LINUX_ERRQUEUE

namespace grpc_core {

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
  int dscp = kDscpNotSet;
  bool expand_wildcard_addrs = false;
  bool allow_reuse_port = false;
  RefCountedPtr<ResourceQuota> resource_quota;
  struct grpc_socket_mutator* socket_mutator = nullptr;
  std::shared_ptr<grpc_event_engine::experimental::EventEngine> event_engine;
  PosixTcpOptions() = default;
  // Move ctor
  PosixTcpOptions(PosixTcpOptions&& other) noexcept {
    socket_mutator = std::exchange(other.socket_mutator, nullptr);
    resource_quota = std::move(other.resource_quota);
    event_engine = std::move(other.event_engine);
    CopyIntegerOptions(other);
  }
  // Move assignment
  PosixTcpOptions& operator=(PosixTcpOptions&& other) noexcept {
    if (socket_mutator != nullptr) {
      grpc_socket_mutator_unref(socket_mutator);
    }
    socket_mutator = std::exchange(other.socket_mutator, nullptr);
    resource_quota = std::move(other.resource_quota);
    event_engine = std::move(other.event_engine);
    CopyIntegerOptions(other);
    return *this;
  }
  // Copy ctor
  PosixTcpOptions(const PosixTcpOptions& other) {
    if (other.socket_mutator != nullptr) {
      socket_mutator = grpc_socket_mutator_ref(other.socket_mutator);
    }
    resource_quota = other.resource_quota;
    event_engine = other.event_engine;
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
    event_engine = other.event_engine;
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

}  // namespace grpc_core

grpc_core::PosixTcpOptions TcpOptionsFromEndpointConfig(
    const grpc_event_engine::experimental::EndpointConfig& config);

// a wrapper for accept or accept4
int grpc_accept4(int sockfd, grpc_resolved_address* resolved_addr, int nonblock,
                 int cloexec);

// set a socket to use zerocopy
grpc_error_handle grpc_set_socket_zerocopy(int fd);

// set a socket to non blocking mode
grpc_error_handle grpc_set_socket_nonblocking(int fd, int non_blocking);

// set a socket to close on exec
grpc_error_handle grpc_set_socket_cloexec(int fd, int close_on_exec);

// set a socket to reuse old addresses
grpc_error_handle grpc_set_socket_reuse_addr(int fd, int reuse);

// return true if SO_REUSEPORT is supported
bool grpc_is_socket_reuse_port_supported();

// disable nagle
grpc_error_handle grpc_set_socket_low_latency(int fd, int low_latency);

// set SO_REUSEPORT
grpc_error_handle grpc_set_socket_reuse_port(int fd, int reuse);

/* Set Differentiated Services Code Point (DSCP) */
grpc_error_handle grpc_set_socket_dscp(int fd, int dscp);

// Configure the default values for TCP_USER_TIMEOUT
void config_default_tcp_user_timeout(bool enable, int timeout, bool is_client);

// Set TCP_USER_TIMEOUT
grpc_error_handle grpc_set_socket_tcp_user_timeout(
    int fd, const grpc_core::PosixTcpOptions& options, bool is_client);

// Returns true if this system can create AF_INET6 sockets bound to ::1.
// The value is probed once, and cached for the life of the process.

// This is more restrictive than checking for socket(AF_INET6) to succeed,
// because Linux with "net.ipv6.conf.all.disable_ipv6 = 1" is able to create
// and bind IPv6 sockets, but cannot connect to a getsockname() of [::]:port
// without a valid loopback interface.  Rather than expose this half-broken
// state to library users, we turn off IPv6 sockets.
int grpc_ipv6_loopback_available(void);

// Tries to set SO_NOSIGPIPE if available on this platform.
// If SO_NO_SIGPIPE is not available, returns 1.
grpc_error_handle grpc_set_socket_no_sigpipe_if_possible(int fd);

// Tries to set IP_PKTINFO if available on this platform.
// If IP_PKTINFO is not available, returns 1.
grpc_error_handle grpc_set_socket_ip_pktinfo_if_possible(int fd);

// Tries to set IPV6_RECVPKTINFO if available on this platform.
// If IPV6_RECVPKTINFO is not available, returns 1.
grpc_error_handle grpc_set_socket_ipv6_recvpktinfo_if_possible(int fd);

// Tries to set the socket's send buffer to given size.
grpc_error_handle grpc_set_socket_sndbuf(int fd, int buffer_size_bytes);

// Tries to set the socket's receive buffer to given size.
grpc_error_handle grpc_set_socket_rcvbuf(int fd, int buffer_size_bytes);

// Tries to set the socket using a grpc_socket_mutator
grpc_error_handle grpc_set_socket_with_mutator(int fd, grpc_fd_usage usage,
                                               grpc_socket_mutator* mutator);

// Extracts the first socket mutator from config if any and applies on the fd.
//
grpc_error_handle grpc_apply_socket_mutator_in_args(
    int fd, grpc_fd_usage usage, const grpc_core::PosixTcpOptions& options);

// An enum to keep track of IPv4/IPv6 socket modes.

// Currently, this information is only used when a socket is first created, but
// in the future we may wish to store it alongside the fd.  This would let calls
// like sendto() know which family to use without asking the kernel first.
typedef enum grpc_dualstack_mode {
  // Uninitialized, or a non-IP socket.
  GRPC_DSMODE_NONE,
  // AF_INET only.
  GRPC_DSMODE_IPV4,
  // AF_INET6 only, because IPV6_V6ONLY could not be cleared.
  GRPC_DSMODE_IPV6,
  // AF_INET6, which also supports ::ffff-mapped IPv4 addresses.
  GRPC_DSMODE_DUALSTACK
} grpc_dualstack_mode;

// Only tests should use this flag.
extern int grpc_forbid_dualstack_sockets_for_testing;

// Tries to set the socket to dualstack. Returns 1 on success.
int grpc_set_socket_dualstack(int fd);

// Creates a new socket for connecting to (or listening on) an address.

// If addr is AF_INET6, this creates an IPv6 socket first.  If that fails,
// and addr is within ::ffff:0.0.0.0/96, then it automatically falls back to
// an IPv4 socket.

// If addr is AF_INET, AF_UNIX, or anything else, then this is similar to
// calling socket() directly.

// Returns an fd on success, otherwise returns -1 with errno set to the result
// of a failed socket() call.

// The *dsmode output indicates which address family was actually created.
// The recommended way to use this is:
// - First convert to IPv6 using grpc_sockaddr_to_v4mapped().
// - Create the socket.
// - If *dsmode is IPV4, use grpc_sockaddr_is_v4mapped() to convert back to
//   IPv4, so that bind() or connect() see the correct family.
// Also, it's important to distinguish between DUALSTACK and IPV6 when
// listening on the [::] wildcard address.
grpc_error_handle grpc_create_dualstack_socket(
    const grpc_resolved_address* addr, int type, int protocol,
    grpc_dualstack_mode* dsmode, int* newfd);

// Same as grpc_create_dualstack_socket(), but use the given socket factory (if
// non-null) to create the socket, rather than calling socket() directly.
grpc_error_handle grpc_create_dualstack_socket_using_factory(
    grpc_socket_factory* factory, const grpc_resolved_address* addr, int type,
    int protocol, grpc_dualstack_mode* dsmode, int* newfd);

#endif  // GRPC_SRC_CORE_LIB_IOMGR_SOCKET_UTILS_POSIX_H
