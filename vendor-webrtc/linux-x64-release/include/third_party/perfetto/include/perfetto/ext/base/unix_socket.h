/*
 * Copyright (C) 2017 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_EXT_BASE_UNIX_SOCKET_H_
#define INCLUDE_PERFETTO_EXT_BASE_UNIX_SOCKET_H_

#include <stdint.h>
#include <sys/types.h>

#include <memory>
#include <string>
#include <utility>

#include "perfetto/base/build_config.h"
#include "perfetto/base/compiler.h"
#include "perfetto/base/export.h"
#include "perfetto/base/logging.h"
#include "perfetto/base/platform_handle.h"
#include "perfetto/ext/base/scoped_file.h"
#include "perfetto/ext/base/utils.h"
#include "perfetto/ext/base/weak_ptr.h"

struct msghdr;

namespace perfetto {
namespace base {

// Define the ScopedSocketHandle type.

#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
int CloseSocket(SocketHandle);  // A wrapper around ::closesocket().
using ScopedSocketHandle =
    ScopedResource<SocketHandle, CloseSocket, static_cast<SocketHandle>(-1)>;
#else
using ScopedSocketHandle = ScopedFile;
#endif

class TaskRunner;

// Use arbitrarily high values to avoid that some code accidentally ends up
// assuming that these enum values match the sysroot's SOCK_xxx defines rather
// than using MkSockType() / MkSockFamily().
enum class SockType { kStream = 100, kDgram, kSeqPacket };
enum class SockFamily { kUnspec = 0, kUnix = 200, kInet, kInet6, kVsock };

// Controls the getsockopt(SO_PEERCRED) behavior, which allows to obtain the
// peer credentials.
enum class SockPeerCredMode {
  // Obtain the peer credentials immediately after connection and cache them.
  kReadOnConnect = 0,

  // Don't read peer credentials at all. Calls to peer_uid()/peer_pid() will
  // hit a DCHECK and return kInvalidUid/Pid in release builds.
  kIgnore = 1,

#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN) || \
    PERFETTO_BUILDFLAG(PERFETTO_OS_FUCHSIA)
  kDefault = kIgnore,
#else
  kDefault = kReadOnConnect,
#endif
};

// Returns the socket family from the full address that perfetto uses.
// Addr can be:
// - /path/to/socket : for linked AF_UNIX sockets.
// - @abstract_name  : for abstract AF_UNIX sockets.
// - 1.2.3.4:8080    : for Inet sockets.
// - [::1]:8080      : for Inet6 sockets.
// - vsock://-1:3000 : for VM sockets.
SockFamily GetSockFamily(const char* addr);

// NetAddrInfo is a wrapper for a full address and it's family
// and type.
struct NetAddrInfo {
  NetAddrInfo(std::string _ip_port, SockFamily _family, SockType _type)
      : ip_port(std::move(_ip_port)), family(_family), type(_type) {}
  std::string ip_port;  // full address with ip and port
  SockFamily family;    // socket family
  SockType type;        // socket type
};

// Returns a list of NetAddrInfo from ip and port which
// ip can be an ipv4 or an ipv6 or a domain.
// 127.0.0.1    : ipv4 address
// localhost    : domain
// ::1          : ipv6 address
// port is a normal tcp port as string.
std::vector<NetAddrInfo> GetNetAddrInfo(const std::string& ip,
                                        const std::string& port);

// Returns whether inter-process shared memory is supported for the socket.
inline bool SockShmemSupported(SockFamily sock_family) {
#if !PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
  return sock_family == SockFamily::kUnix;
#else
  base::ignore_result(sock_family);
  // On Windows shm is negotiated by sharing an unguessable token
  // over TCP sockets. In theory works on any socket type, in practice
  // we need to tell the difference between a local and a remote
  // connection. For now we assume everything is local.
  // See comments on r.android.com/2951909 .
  return true;
#endif
}
inline bool SockShmemSupported(const char* addr) {
  return SockShmemSupported(GetSockFamily(addr));
}

// UnixSocketRaw is a basic wrapper around sockets. It exposes wrapper
// methods that take care of most common pitfalls (e.g., marking fd as
// O_CLOEXEC, avoiding SIGPIPE, properly handling partial writes). It is used as
// a building block for the more sophisticated UnixSocket class which depends
// on base::TaskRunner.
class UnixSocketRaw {
 public:
  // Creates a new unconnected unix socket.
  static UnixSocketRaw CreateMayFail(SockFamily family, SockType type);

#if !PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
  // Crates a pair of connected sockets.
  static std::pair<UnixSocketRaw, UnixSocketRaw> CreatePairPosix(SockFamily,
                                                                 SockType);
#endif

  // Creates an uninitialized unix socket.
  UnixSocketRaw();

  // Creates a unix socket adopting an existing file descriptor. This is
  // typically used to inherit fds from init via environment variables.
  UnixSocketRaw(ScopedSocketHandle, SockFamily, SockType);

  ~UnixSocketRaw() = default;
  UnixSocketRaw(UnixSocketRaw&&) noexcept = default;
  UnixSocketRaw& operator=(UnixSocketRaw&&) = default;

  bool Bind(const std::string& socket_name);
  bool Listen();
  bool Connect(const std::string& socket_name);
  bool SetTxTimeout(uint32_t timeout_ms);
  bool SetRxTimeout(uint32_t timeout_ms);
  void Shutdown();
  void SetBlocking(bool);
  void DcheckIsBlocking(bool expected) const;  // No-op on release and Win.
  void SetRetainOnExec(bool retain);
  std::string GetSockAddr() const;
  SockType type() const { return type_; }
  SockFamily family() const { return family_; }
  SocketHandle fd() const { return *fd_; }
  explicit operator bool() const { return !!fd_; }

  // This is the handle that passed to TaskRunner.AddFileDescriptorWatch().
  // On UNIX this is just the socket FD. On Windows, we need to create a
  // dedicated event object.
  PlatformHandle watch_handle() const {
#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
    return *event_handle_;
#else
    return *fd_;
#endif
  }

  ScopedSocketHandle ReleaseFd() { return std::move(fd_); }

  // |send_fds| and |num_fds| are ignored on Windows.
  ssize_t Send(const void* msg,
               size_t len,
               const int* send_fds = nullptr,
               size_t num_fds = 0);

  ssize_t SendStr(const std::string& str) {
    return Send(str.data(), str.size());
  }

  // |fd_vec| and |max_files| are ignored on Windows.
  ssize_t Receive(void* msg,
                  size_t len,
                  ScopedFile* fd_vec = nullptr,
                  size_t max_files = 0);

#if !PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
  // UNIX-specific helpers to deal with SCM_RIGHTS.

  // Re-enter sendmsg until all the data has been sent or an error occurs.
  // TODO(fmayer): Figure out how to do timeouts here for heapprofd.
  ssize_t SendMsgAllPosix(struct msghdr* msg);

  // Exposed for testing only.
  // Update msghdr so subsequent sendmsg will send data that remains after n
  // bytes have already been sent.
  static void ShiftMsgHdrPosix(size_t n, struct msghdr* msg);
#endif

 private:
  UnixSocketRaw(SockFamily, SockType);

  UnixSocketRaw(const UnixSocketRaw&) = delete;
  UnixSocketRaw& operator=(const UnixSocketRaw&) = delete;

  ScopedSocketHandle fd_;
#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
  ScopedPlatformHandle event_handle_;
#endif
  SockFamily family_ = SockFamily::kUnix;
  SockType type_ = SockType::kStream;
  uint32_t tx_timeout_ms_ = 0;
};

// A non-blocking UNIX domain socket. Allows also to transfer file descriptors.
// None of the methods in this class are blocking.
// The main design goal is making strong guarantees on the EventListener
// callbacks, in order to avoid ending in some undefined state.
// In case of any error it will aggressively just shut down the socket and
// notify the failure with OnConnect(false) or OnDisconnect() depending on the
// state of the socket (see below).
// EventListener callbacks stop happening as soon as the instance is destroyed.
//
// Lifecycle of a client socket:
//
//                           Connect()
//                               |
//            +------------------+------------------+
//            | (success)                           | (failure or Shutdown())
//            V                                     V
//     OnConnect(true)                         OnConnect(false)
//            |
//            V
//    OnDataAvailable()
//            |
//            V
//     OnDisconnect()  (failure or shutdown)
//
//
// Lifecycle of a server socket:
//
//                          Listen()  --> returns false in case of errors.
//                             |
//                             V
//              OnNewIncomingConnection(new_socket)
//
//          (|new_socket| inherits the same EventListener)
//                             |
//                             V
//                     OnDataAvailable()
//                             | (failure or Shutdown())
//                             V
//                       OnDisconnect()
class PERFETTO_EXPORT_COMPONENT UnixSocket {
 public:
  class EventListener {
   public:
    EventListener() = default;
    virtual ~EventListener();

    EventListener(const EventListener&) = delete;
    EventListener& operator=(const EventListener&) = delete;

    EventListener(EventListener&&) noexcept = default;
    EventListener& operator=(EventListener&&) noexcept = default;

    // After Listen().
    // |self| may be null if the connection was not accepted via a listen
    // socket.
    virtual void OnNewIncomingConnection(
        UnixSocket* self,
        std::unique_ptr<UnixSocket> new_connection);

    // After Connect(), whether successful or not.
    virtual void OnConnect(UnixSocket* self, bool connected);

    // After a successful Connect() or OnNewIncomingConnection(). Either the
    // other endpoint did disconnect or some other error happened.
    virtual void OnDisconnect(UnixSocket* self);

    // Whenever there is data available to Receive(). Note that spurious FD
    // watch events are possible, so it is possible that Receive() soon after
    // OnDataAvailable() returns 0 (just ignore those).
    virtual void OnDataAvailable(UnixSocket* self);
  };

  enum class State {
    kDisconnected = 0,  // Failed connection, peer disconnection or Shutdown().
    kConnecting,  // Soon after Connect(), before it either succeeds or fails.
    kConnected,   // After a successful Connect().
    kListening    // After Listen(), until Shutdown().
  };

  // Creates a socket and starts listening. If SockFamily::kUnix and
  // |socket_name| starts with a '@', an abstract UNIX dmoain socket will be
  // created instead of a filesystem-linked UNIX socket (Linux/Android only).
  // If SockFamily::kInet, |socket_name| is host:port (e.g., "1.2.3.4:8000").
  // If SockFamily::kInet6, |socket_name| is [host]:port (e.g., "[::1]:8000").
  // Returns nullptr if the socket creation or bind fails. If listening fails,
  // (e.g. if another socket with the same name is already listening) the
  // returned socket will have is_listening() == false.
  static std::unique_ptr<UnixSocket> Listen(const std::string& socket_name,
                                            EventListener*,
                                            TaskRunner*,
                                            SockFamily,
                                            SockType);

  // Attaches to a pre-existing socket. The socket must have been created in
  // SOCK_STREAM mode and the caller must have called bind() on it.
  static std::unique_ptr<UnixSocket> Listen(ScopedSocketHandle,
                                            EventListener*,
                                            TaskRunner*,
                                            SockFamily,
                                            SockType);

  // Creates a Unix domain socket and connects to the listening endpoint.
  // Returns always an instance. EventListener::OnConnect(bool success) will
  // be called always, whether the connection succeeded or not.
  static std::unique_ptr<UnixSocket> Connect(
      const std::string& socket_name,
      EventListener*,
      TaskRunner*,
      SockFamily,
      SockType,
      SockPeerCredMode = SockPeerCredMode::kDefault);

  // Constructs a UnixSocket using the given connected socket.
  static std::unique_ptr<UnixSocket> AdoptConnected(
      ScopedSocketHandle,
      EventListener*,
      TaskRunner*,
      SockFamily,
      SockType,
      SockPeerCredMode = SockPeerCredMode::kDefault);

  UnixSocket(const UnixSocket&) = delete;
  UnixSocket& operator=(const UnixSocket&) = delete;
  // Cannot be easily moved because of tasks from the FileDescriptorWatch.
  UnixSocket(UnixSocket&&) = delete;
  UnixSocket& operator=(UnixSocket&&) = delete;

  // This class gives the hard guarantee that no callback is called on the
  // passed EventListener immediately after the object has been destroyed.
  // Any queued callback will be silently dropped.
  ~UnixSocket();

  // Shuts down the current connection, if any. If the socket was Listen()-ing,
  // stops listening. The socket goes back to kNotInitialized state, so it can
  // be reused with Listen() or Connect().
  void Shutdown(bool notify);

  void SetTxTimeout(uint32_t timeout_ms) {
    PERFETTO_CHECK(sock_raw_.SetTxTimeout(timeout_ms));
  }
  void SetRxTimeout(uint32_t timeout_ms) {
    PERFETTO_CHECK(sock_raw_.SetRxTimeout(timeout_ms));
  }

  std::string GetSockAddr() const { return sock_raw_.GetSockAddr(); }

  // Returns true is the message was queued, false if there was no space in the
  // output buffer, in which case the client should retry or give up.
  // If any other error happens the socket will be shutdown and
  // EventListener::OnDisconnect() will be called.
  // If the socket is not connected, Send() will just return false.
  // Does not append a null string terminator to msg in any case.
  bool Send(const void* msg, size_t len, const int* send_fds, size_t num_fds);

  inline bool Send(const void* msg, size_t len, int send_fd = -1) {
    if (send_fd != -1)
      return Send(msg, len, &send_fd, 1);
    return Send(msg, len, nullptr, 0);
  }

  inline bool SendStr(const std::string& msg) {
    return Send(msg.data(), msg.size(), -1);
  }

  // Returns the number of bytes (<= |len|) written in |msg| or 0 if there
  // is no data in the buffer to read or an error occurs (in which case a
  // EventListener::OnDisconnect() will follow).
  // If the ScopedFile pointer is not null and a FD is received, it moves the
  // received FD into that. If a FD is received but the ScopedFile pointer is
  // null, the FD will be automatically closed.
  size_t Receive(void* msg, size_t len, ScopedFile*, size_t max_files = 1);

  inline size_t Receive(void* msg, size_t len) {
    return Receive(msg, len, nullptr, 0);
  }

  // Only for tests. This is slower than Receive() as it requires a heap
  // allocation and a copy for the std::string. Guarantees that the returned
  // string is null terminated even if the underlying message sent by the peer
  // is not.
  std::string ReceiveString(size_t max_length = 1024);

  bool is_connected() const { return state_ == State::kConnected; }
  bool is_listening() const { return state_ == State::kListening; }
  SocketHandle fd() const { return sock_raw_.fd(); }
  SockFamily family() const { return sock_raw_.family(); }

  // User ID of the peer, as returned by the kernel. If the client disconnects
  // and the socket goes into the kDisconnected state, it retains the uid of
  // the last peer.
#if !PERFETTO_BUILDFLAG(PERFETTO_OS_WIN) && \
    !PERFETTO_BUILDFLAG(PERFETTO_OS_FUCHSIA)
  uid_t peer_uid_posix(bool skip_check_for_testing = false) const {
    PERFETTO_DCHECK((!is_listening() && peer_uid_ != kInvalidUid) ||
                    skip_check_for_testing);

    return peer_uid_;
  }
#endif

#if PERFETTO_BUILDFLAG(PERFETTO_OS_LINUX_BUT_NOT_QNX) || \
    PERFETTO_BUILDFLAG(PERFETTO_OS_ANDROID)
  // Process ID of the peer, as returned by the kernel. If the client
  // disconnects and the socket goes into the kDisconnected state, it
  // retains the pid of the last peer.
  //
  // This is only available on Linux / Android.
  pid_t peer_pid_linux(bool skip_check_for_testing = false) const {
    PERFETTO_DCHECK((!is_listening() && peer_pid_ != kInvalidPid) ||
                    skip_check_for_testing);
    return peer_pid_;
  }
#endif

  // This makes the UnixSocket unusable.
  UnixSocketRaw ReleaseSocket();

 private:
  UnixSocket(EventListener*,
             TaskRunner*,
             SockFamily,
             SockType,
             SockPeerCredMode);
  UnixSocket(EventListener*,
             TaskRunner*,
             ScopedSocketHandle,
             State,
             SockFamily,
             SockType,
             SockPeerCredMode);

  // Called once by the corresponding public static factory methods.
  void DoConnect(const std::string& socket_name);

#if !PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
  void ReadPeerCredentialsPosix();
#endif

  void OnEvent();
  void NotifyConnectionState(bool success);

  UnixSocketRaw sock_raw_;
  State state_ = State::kDisconnected;
  SockPeerCredMode peer_cred_mode_ = SockPeerCredMode::kDefault;

#if !PERFETTO_BUILDFLAG(PERFETTO_OS_WIN) && \
    !PERFETTO_BUILDFLAG(PERFETTO_OS_FUCHSIA)
  uid_t peer_uid_ = kInvalidUid;
#endif
#if PERFETTO_BUILDFLAG(PERFETTO_OS_LINUX) || \
    PERFETTO_BUILDFLAG(PERFETTO_OS_ANDROID)
  pid_t peer_pid_ = kInvalidPid;
#endif
  EventListener* const event_listener_;
  TaskRunner* const task_runner_;
  WeakPtrFactory<UnixSocket> weak_ptr_factory_;  // Keep last.
};

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_UNIX_SOCKET_H_
