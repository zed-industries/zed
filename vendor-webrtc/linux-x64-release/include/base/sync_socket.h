// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SYNC_SOCKET_H_
#define BASE_SYNC_SOCKET_H_

// A socket abstraction used for sending and receiving plain
// data.  Because the receiving is blocking, they can be used to perform
// rudimentary cross-process synchronization with low latency.

#include <stddef.h>

#include "base/base_export.h"
#include "base/containers/span.h"
#include "base/files/platform_file.h"
#include "base/synchronization/waitable_event.h"
#include "base/time/time.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include <windows.h>
#endif
#include <sys/types.h>

namespace base {

class BASE_EXPORT SyncSocket {
 public:
  using Handle = PlatformFile;
  using ScopedHandle = ScopedPlatformFile;
  static const Handle kInvalidHandle;

  SyncSocket();

  // Creates a SyncSocket from a Handle.
  explicit SyncSocket(Handle handle);
  explicit SyncSocket(ScopedHandle handle);
  SyncSocket(const SyncSocket&) = delete;
  SyncSocket& operator=(const SyncSocket&) = delete;
  virtual ~SyncSocket();

  // Initializes and connects a pair of sockets.
  // |socket_a| and |socket_b| must not hold a valid handle.  Upon successful
  // return, the sockets will both be valid and connected.
  static bool CreatePair(SyncSocket* socket_a, SyncSocket* socket_b);

  // Closes the SyncSocket.
  virtual void Close();

  // Sends the message to the remote peer of the SyncSocket.
  // Note it is not safe to send messages from the same socket handle by
  // multiple threads simultaneously.
  // `data` must be non-empty.
  // Returns the number of bytes sent, or 0 upon failure.
  virtual size_t Send(span<const uint8_t> data);

  // Receives a message from an SyncSocket.
  // The data will be received in `buffer`, which must be non-empty.
  // Returns the number of bytes received, or 0 upon failure.
  virtual size_t Receive(span<uint8_t> buffer);

  // Same as Receive() but only blocks for data until `timeout` has elapsed or
  // `buffer` is exhausted. Currently only timeouts less than one second are
  // allowed. Returns the number of bytes read.
  virtual size_t ReceiveWithTimeout(span<uint8_t> buffer, TimeDelta timeout);

  // Returns the number of bytes available. If non-zero, Receive() will not
  // not block when called.
  virtual size_t Peek();

  // Returns true if the Handle is valid, and false if it is not.
  bool IsValid() const;

  // Extracts the contained handle.  Used for transferring between
  // processes.
  Handle handle() const;

  // Extracts and takes ownership of the contained handle.
  Handle Release();
  ScopedHandle Take();

 protected:
  ScopedHandle handle_;
};

// Derives from SyncSocket and adds support for shutting down the socket from
// another thread while a blocking Receive or Send is being done from the
// thread that owns the socket.
class BASE_EXPORT CancelableSyncSocket : public SyncSocket {
 public:
  CancelableSyncSocket();
  explicit CancelableSyncSocket(Handle handle);
  explicit CancelableSyncSocket(ScopedHandle handle);
  CancelableSyncSocket(const CancelableSyncSocket&) = delete;
  CancelableSyncSocket& operator=(const CancelableSyncSocket&) = delete;
  ~CancelableSyncSocket() override = default;

  // Initializes a pair of cancelable sockets.  See documentation for
  // SyncSocket::CreatePair for more details.
  static bool CreatePair(CancelableSyncSocket* socket_a,
                         CancelableSyncSocket* socket_b);

  // A way to shut down a socket even if another thread is currently performing
  // a blocking Receive or Send.
  bool Shutdown();

#if BUILDFLAG(IS_WIN)
  // Since the Linux and Mac implementations actually use a socket, shutting
  // them down from another thread is pretty simple - we can just call
  // shutdown().  However, the Windows implementation relies on named pipes
  // and there isn't a way to cancel a blocking synchronous Read that is
  // supported on <Vista. So, for Windows only, we override these
  // SyncSocket methods in order to support shutting down the 'socket'.
  void Close() override;
  size_t Receive(span<uint8_t> buffer) override;
  size_t ReceiveWithTimeout(span<uint8_t> buffer, TimeDelta timeout) override;
#endif

  // Send() is overridden to catch cases where the remote end is not responding
  // and we fill the local socket buffer. When `data` is full, this
  // implementation of Send() will not block indefinitely as
  // SyncSocket::Send will, but instead return 0, as no bytes could be sent.
  // Note that the socket will not be closed in this case.
  size_t Send(span<const uint8_t> data) override;

 private:
#if BUILDFLAG(IS_WIN)
  WaitableEvent shutdown_event_;
  WaitableEvent file_operation_;
#endif
};

}  // namespace base

#endif  // BASE_SYNC_SOCKET_H_
