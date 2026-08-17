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

#ifndef SRC_TRACED_RELAY_SOCKET_RELAY_HANDLER_H_
#define SRC_TRACED_RELAY_SOCKET_RELAY_HANDLER_H_

#include <poll.h>

#include <cstring>
#include <deque>
#include <future>
#include <mutex>
#include <optional>
#include <thread>
#include <tuple>

#include "perfetto/base/platform_handle.h"
#include "perfetto/ext/base/event_fd.h"
#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/thread_checker.h"
#include "perfetto/ext/base/unix_socket.h"
#include "perfetto/ext/ipc/basic_types.h"

namespace perfetto {

// FdPoller is a utility for waiting for IO events of a set of watched file
// descriptors. It's used for multiplexing non-blocking IO operations.
class FdPoller {
 public:
  // The interface class for observing IO events from the FdPoller class.
  class Watcher {
   public:
    virtual ~Watcher();
    // Called when |fd| can be read from without blocking. For a socket
    // connection, this indicates the socket read buffer has some data.
    virtual void OnFdReadable(base::PlatformHandle fd) = 0;
    // Called when |fd| can be written to without blocking. For a socket
    // connection, this indicates that the socket write buffer has some capacity
    // for writting data into.
    virtual void OnFdWritable(base::PlatformHandle fd) = 0;
  };

  using WatchEvents = decltype(pollfd::events);

  explicit FdPoller(Watcher* watcher);

  // Watch and unwatch IO event for a given file descriptor.
  inline void WatchForRead(base::PlatformHandle fd) { WatchFd(fd, POLLIN); }
  inline void WatchForWrite(base::PlatformHandle fd) { WatchFd(fd, POLLOUT); }
  inline void UnwatchForRead(base::PlatformHandle fd) { UnwatchFd(fd, POLLIN); }
  inline void UnwatchForWrite(base::PlatformHandle fd) {
    UnwatchFd(fd, POLLOUT);
  }

  // Called when |fd| is no longer of interest (e.g. when |fd| is to be closed).
  void RemoveWatch(base::PlatformHandle fd);

  // Poll for all watched events previously added with WatchForRead() and
  // WatchForWrite().
  //
  // Must be called on poller thread.
  void Poll();

  // Notifies the poller for pending updates. Calling Notify() will unblock the
  // poller and make it return from Poll(). It is caller's responsibility to
  // call Poll() again once the updates are complete.
  //
  // This can be (and typically is) called from any thread.
  void Notify();

 private:
  std::vector<pollfd>::iterator FindPollEvent(base::PlatformHandle fd);
  void WatchFd(base::PlatformHandle fd, WatchEvents events);
  void UnwatchFd(base::PlatformHandle fd, WatchEvents events);

  base::ThreadChecker thread_checker_;
  Watcher* const watcher_;
  base::EventFd notify_fd_;
  std::vector<pollfd> poll_fds_;
};

// This class groups a UnixSocketRaw with an associated ring buffer. The ring
// buffer is used as a temporary storage for data *read* from the socket.
class SocketWithBuffer {
 public:
  constexpr static size_t kBuffSize = ipc::kIPCBufferSize;

  base::UnixSocketRaw sock;

  // Points to the beginning of buffered data.
  inline uint8_t* data() { return &buf_[0]; }
  // Size of the buffered data.
  inline size_t data_size() { return data_size_; }

  // Points to the beginning of the free space for buffering new data.
  inline uint8_t* buffer() { return &buf_[data_size_]; }
  // Size of the free space.
  inline size_t available_bytes() { return buf_.size() - data_size_; }

  // Called when |bytes| of data is enqueued to the buffer.
  void EnqueueData(size_t bytes) {
    PERFETTO_CHECK(bytes <= available_bytes());
    data_size_ += bytes;
  }
  // Called when |bytes| of data is dequeued from the buffer.
  void DequeueData(size_t bytes) {
    PERFETTO_CHECK(bytes <= data_size());
    memmove(data(), data() + bytes, data_size() - bytes);
    data_size_ -= bytes;
  }

  SocketWithBuffer() : buf_(kBuffSize) {}

  // Movable only.
  SocketWithBuffer(SocketWithBuffer&& other) = default;
  SocketWithBuffer& operator=(SocketWithBuffer&& other) = default;
  SocketWithBuffer(const SocketWithBuffer& other) = delete;
  SocketWithBuffer& operator=(const SocketWithBuffer& other) = delete;

 private:
  std::vector<uint8_t> buf_;
  size_t data_size_ = 0;
};

using SocketPair = std::pair<SocketWithBuffer, SocketWithBuffer>;

// SocketRelayHandler bidirectionally forwards data between paired sockets.
// Internally it multiplexes IO operations of the sockets using a FdPoller on a
// dedicated thread.
class SocketRelayHandler : public FdPoller::Watcher {
 public:
  SocketRelayHandler();
  SocketRelayHandler(const SocketRelayHandler&) = delete;
  SocketRelayHandler& operator=(const SocketRelayHandler&) = delete;
  ~SocketRelayHandler() override;

  // Transfer a pair of sockets to be relayed. Can be called from any thread.
  void AddSocketPair(std::unique_ptr<SocketPair> socket_pair);

  // The FdPoller::Watcher callbacks.
  void OnFdReadable(base::PlatformHandle fd) override;
  void OnFdWritable(base::PlatformHandle fd) override;

 private:
  void Run();
  void RemoveSocketPair(SocketWithBuffer&, SocketWithBuffer&);

  // A helper for running a callable object on |io_thread_|.
  template <typename Callable>
  void RunOnIOThread(Callable&& c) {
    std::lock_guard<std::mutex> lock(mutex_);
    pending_tasks_.emplace_back(std::forward<Callable>(c));
    fd_poller_.Notify();
  }

  std::optional<std::tuple<SocketWithBuffer&, SocketWithBuffer&>> GetSocketPair(
      base::PlatformHandle fd);

  base::FlatHashMap<base::PlatformHandle, SocketPair*> socket_pairs_by_fd_;
  std::vector<std::unique_ptr<SocketPair>> socket_pairs_;

  FdPoller fd_poller_;

  // The thread that fd_poller_ polls for IO events. Most methods of this class
  // asserts to be running on this thread.
  std::thread io_thread_;
  base::ThreadChecker io_thread_checker_;

  bool exited_ = false;

  //--------------- Member data with multi-thread access ------------------
  std::mutex mutex_;
  std::deque<std::packaged_task<void()>> pending_tasks_;
};

}  // namespace perfetto
#endif  // SRC_TRACED_RELAY_SOCKET_RELAY_HANDLER_H_
