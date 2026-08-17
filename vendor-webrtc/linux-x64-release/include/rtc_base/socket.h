/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_SOCKET_H_
#define RTC_BASE_SOCKET_H_

#include <errno.h>

#include <cstddef>
#include <cstdint>
#include <optional>

// IWYU pragma: begin_exports
#if defined(WEBRTC_POSIX)
#include <arpa/inet.h>
#include <sys/types.h>
#define SOCKET_EACCES EACCES
#endif
// IWYU pragma: end_exports

#include "api/units/timestamp.h"
#include "rtc_base/buffer.h"
#include "rtc_base/checks.h"
#include "rtc_base/ip_address.h"
#include "rtc_base/net_helpers.h"
#include "rtc_base/network/ecn_marking.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/third_party/sigslot/sigslot.h"

// Rather than converting errors into a private namespace,
// Reuse the POSIX socket api errors. Note this depends on
// Win32 compatibility.

#if defined(WEBRTC_WIN)
#undef EWOULDBLOCK  // Remove errno.h's definition for each macro below.
#define EWOULDBLOCK WSAEWOULDBLOCK
#undef EINPROGRESS
#define EINPROGRESS WSAEINPROGRESS
#undef EALREADY
#define EALREADY WSAEALREADY
#undef EMSGSIZE
#define EMSGSIZE WSAEMSGSIZE
#undef EADDRINUSE
#define EADDRINUSE WSAEADDRINUSE
#undef EADDRNOTAVAIL
#define EADDRNOTAVAIL WSAEADDRNOTAVAIL
#undef ENETDOWN
#define ENETDOWN WSAENETDOWN
#undef ECONNABORTED
#define ECONNABORTED WSAECONNABORTED
#undef ENOBUFS
#define ENOBUFS WSAENOBUFS
#undef EISCONN
#define EISCONN WSAEISCONN
#undef ENOTCONN
#define ENOTCONN WSAENOTCONN
#undef ECONNREFUSED
#define ECONNREFUSED WSAECONNREFUSED
#undef EHOSTUNREACH
#define EHOSTUNREACH WSAEHOSTUNREACH
#undef ENETUNREACH
#define ENETUNREACH WSAENETUNREACH
#define SOCKET_EACCES WSAEACCES
#endif  // WEBRTC_WIN

#if defined(WEBRTC_POSIX)
#define INVALID_SOCKET (-1)
#define SOCKET_ERROR (-1)
#define closesocket(s) close(s)
#endif  // WEBRTC_POSIX

namespace webrtc {

inline bool IsBlockingError(int e) {
  return (e == EWOULDBLOCK) || (e == EAGAIN) || (e == EINPROGRESS);
}

// General interface for the socket implementations of various networks.  The
// methods match those of normal UNIX sockets very closely.
class RTC_EXPORT Socket {
 public:
  struct ReceiveBuffer {
    ReceiveBuffer(Buffer& payload) : payload(payload) {}

    std::optional<Timestamp> arrival_time;
    SocketAddress source_address;
    EcnMarking ecn = EcnMarking::kNotEct;
    Buffer& payload;
  };
  virtual ~Socket() {}

  Socket(const Socket&) = delete;
  Socket& operator=(const Socket&) = delete;

  // Returns the address to which the socket is bound.  If the socket is not
  // bound, then the any-address is returned.
  virtual SocketAddress GetLocalAddress() const = 0;

  // Returns the address to which the socket is connected.  If the socket is
  // not connected, then the any-address is returned.
  virtual SocketAddress GetRemoteAddress() const = 0;

  virtual int Bind(const SocketAddress& addr) = 0;
  virtual int Connect(const SocketAddress& addr) = 0;
  virtual int Send(const void* pv, size_t cb) = 0;
  virtual int SendTo(const void* pv, size_t cb, const SocketAddress& addr) = 0;
  // `timestamp` is in units of microseconds.
  virtual int Recv(void* pv, size_t cb, int64_t* timestamp) = 0;
  // TODO(webrtc:15368): Deprecate and remove.
  virtual int RecvFrom(void* /* pv */,
                       size_t /* cb */,
                       SocketAddress* /* paddr */,
                       int64_t* /* timestamp */) {
    // Not implemented. Use RecvFrom(ReceiveBuffer& buffer).
    RTC_CHECK_NOTREACHED();
  }
  // Intended to replace RecvFrom(void* ...).
  // Default implementation calls RecvFrom(void* ...) with 64Kbyte buffer.
  // Returns number of bytes received or a negative value on error.
  virtual int RecvFrom(ReceiveBuffer& buffer);
  virtual int Listen(int backlog) = 0;
  virtual Socket* Accept(SocketAddress* paddr) = 0;
  virtual int Close() = 0;
  virtual int GetError() const = 0;
  virtual void SetError(int error) = 0;
  inline bool IsBlocking() const { return IsBlockingError(GetError()); }

  enum ConnState { CS_CLOSED, CS_CONNECTING, CS_CONNECTED };
  virtual ConnState GetState() const = 0;

  enum Option {
    OPT_DONTFRAGMENT,
    OPT_RCVBUF,                // receive buffer size
    OPT_SNDBUF,                // send buffer size
    OPT_NODELAY,               // whether Nagle algorithm is enabled
    OPT_IPV6_V6ONLY,           // Whether the socket is IPv6 only.
    OPT_DSCP,                  // DSCP code
    OPT_RTP_SENDTIME_EXTN_ID,  // This is a non-traditional socket option param.
                               // This is specific to libjingle and will be used
                               // if SendTime option is needed at socket level.
    OPT_SEND_ECN,              // 2-bit ECN
    OPT_RECV_ECN,
    OPT_KEEPALIVE,         // Enable socket keep alive
    OPT_TCP_KEEPCNT,       // Set TCP keep alive count
    OPT_TCP_KEEPIDLE,      // Set TCP keep alive idle time in seconds
    OPT_TCP_KEEPINTVL,     // Set TCP keep alive interval in seconds
    OPT_TCP_USER_TIMEOUT,  // Set TCP user timeout
  };
  virtual int GetOption(Option opt, int* value) = 0;
  virtual int SetOption(Option opt, int value) = 0;

  // SignalReadEvent and SignalWriteEvent use multi_threaded_local to allow
  // access concurrently from different thread.
  // For example SignalReadEvent::connect will be called in AsyncUDPSocket ctor
  // but at the same time the SocketDispatcher may be signaling the read event.
  // ready to read
  sigslot::signal1<Socket*, sigslot::multi_threaded_local> SignalReadEvent;
  // ready to write
  sigslot::signal1<Socket*, sigslot::multi_threaded_local> SignalWriteEvent;
  sigslot::signal1<Socket*> SignalConnectEvent;     // connected
  sigslot::signal2<Socket*, int> SignalCloseEvent;  // closed

 protected:
  Socket() {}
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::IsBlockingError;
using ::webrtc::Socket;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_SOCKET_H_
