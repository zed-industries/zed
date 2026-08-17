// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_REFERENCE_DRIVERS_SOCKET_TRANSPORT_H_
#define IPCZ_SRC_REFERENCE_DRIVERS_SOCKET_TRANSPORT_H_

#include <cstdint>
#include <functional>
#include <memory>
#include <optional>
#include <thread>
#include <utility>
#include <vector>

#include "reference_drivers/file_descriptor.h"
#include "third_party/abseil-cpp/absl/synchronization/mutex.h"
#include "third_party/abseil-cpp/absl/types/span.h"
#include "util/ref_counted.h"

namespace ipcz::reference_drivers {

// A driver transport implementation backed by a Unix domain socket, suitable
// for use in a multiprocess POSIX testing environment.
class SocketTransport : public RefCounted<SocketTransport> {
 public:
  using Pair = std::pair<Ref<SocketTransport>, Ref<SocketTransport>>;

  struct Message {
    absl::Span<const uint8_t> data;
    absl::Span<FileDescriptor> descriptors;
  };

  // A header injected to prefix every message sent through this
  // SocketTransport, used to frame each message.
  struct Header {
    // The number of bytes in the message, including this Header.
    uint32_t num_bytes;

    // The number of file descriptors in the message.
    uint32_t num_descriptors;
  };

  SocketTransport();
  explicit SocketTransport(FileDescriptor fd);
  SocketTransport(const SocketTransport&) = delete;
  SocketTransport& operator=(const SocketTransport&) = delete;

  // Creates a new pair of entangled SocketTransport objects. For two transports
  // X and Y, a Send(foo) on X will result in an equivalent message arriving on
  // Y once Y is activated. The reverse is also true, since transports are
  // bidirectional.
  static Pair CreatePair();

  // Indicates whether this SocketTransport has been activated yet.
  bool has_been_activated() const { return has_been_activated_; }

  // Spawns an internal I/O thread for this SocketTransport and uses it to
  // monitor the underlying socket for incoming messages, errors, and other
  // relevant events.
  //
  // The transport may invoke `message_handler` or `error_handler` at any time
  // from the I/O thread to notify the client about messages or errors. This
  // continutes until either an error is encountered or Deactivate() is called.
  using MessageHandler = std::function<bool(Message)>;
  using ErrorHandler = std::function<void()>;
  void Activate(
      MessageHandler message_handler = [](Message) { return true; },
      ErrorHandler error_handler = [] {});

  // Stops monitoring the underlying socket. Deactivation may complete
  // asynchronously, and `shutdown_callback` is invoked when complete.
  // Invocation may happen before this call returns if the I/O thread has
  // already been terminated; otherwise the callback is invoked from the I/O
  // thread just before terminating.
  //
  // NOTE: If Activate() has been called, this MUST be called before destroying
  // the SocketTransport.
  void Deactivate(std::function<void()> shutdown_callback);

  // Sends the contents of `message` to the SocketTransport's peer,
  // asynchronously. May be called from any thread.
  //
  // Returns true on success (including cases where the message is queued but
  // not yet transmitted), or false on unrecoverable error.
  bool Send(Message message);

  // Takes ownership of the underlying socket descriptor. This is invalid to
  // call on a SocketTransport which has already been activated, and doing so
  // results in undefined behavior.
  FileDescriptor TakeDescriptor();

 private:
  friend class RefCounted<SocketTransport>;

  ~SocketTransport();

  // Attempts to send `message` without queueing.
  //
  // If `header` is non-empty, its contents are sent just before the contents of
  // `message`.
  //
  // Returns the total number of bytes successfully sent, including any header
  // bytes. If the sum of the size of `header` and `message.data` is returned,
  // then the full message was sent. If any smaller value is returned, including
  // zero, then the message transmission was partially or fully blocked and the
  // remainder will be queued internally by SocketTransport for later
  // transmission. If null is returned, an unrecoverable error was encountered.
  //
  // This method is invoked by only one thread at a time.
  std::optional<size_t> TrySend(absl::Span<uint8_t> header, Message message);

  // Static entry point for the I/O thread.
  static void RunIOThreadForTransport(Ref<SocketTransport> transport);

  // Runs the I/O loop for this SocketTransport. Called from a dedicated,
  // internally managed thread. This method does not return until the underlying
  // socket becomes unusable, some other unrecoverable error is encountered, or
  // BeginShutdown() is invoked from any other thread.
  void RunIOThread();

  // Indicates whether there are any outgoing messages queued.
  bool IsOutgoingQueueEmpty();

  // Ensures that at least `num_bytes` bytes of storage capacity are available
  // at the tail end of `data_buffer_`, and returns the span of all available
  // storage there. If any data is written into this span by the caller, it must
  // be committed with CommitRead() in order to persist it for eventual
  // dispatch.
  //
  // NOTE: The returned value may be invalidated by any subsequent calls to
  // EnsureReadCapacity() or TryDispatchMessages().
  absl::Span<uint8_t> EnsureReadCapacity(size_t num_bytes);

  // Commits data and file descriptors for subsequent dispatch. `num_bytes` is
  // the number of bytes of data to commit starting from the front of the span
  // most recently returned by EnsureReadCapacity().
  void CommitRead(size_t num_bytes, std::vector<FileDescriptor> descriptors);

  // Notifies the transport's client of an unrecoverable error condition. Must
  // be called on the I/O thread.
  void NotifyError();

  // Must be called on the I/O thread any time the socket has
  // received new data or file descriptors and committed them via CommitRead().
  // This gives the SocketTransport an opportunity to parse any complete
  // messages received and dispatch them to its client.
  //
  // Returns true if there were no complete messages to dispatch, or if all
  // complete messages were dispatched successfully. Returns false if a
  // malformed message was encountered or if any message dispatch was rejected
  // by the client.
  //
  // NOTE: This call invalidates any value previously returned by
  // EnsureReadCapacity().
  bool TryDispatchMessages();

  // Called when the underlying socket may be able to send queued outgoing
  // messages again. This may call back into TrySend() to transmit any such
  // queued mesages.
  void TryFlushingOutgoingQueue();

  // Ensures that the I/O loop wakes up for processing.
  void WakeIOThread();

  // Clears any signal from `signal_receiver_` so future polling on that FD will
  // wait for a new signal.
  void ClearIOThreadSignal();

  // Indicates whether Activate() has been called on this transport yet.
  bool has_been_activated_ = false;

  // Background I/O thread used to monitor the underlying socket and dispatch
  // incoming messages or errors.
  absl::Mutex io_thread_mutex_;
  std::unique_ptr<std::thread> io_thread_ ABSL_GUARDED_BY(io_thread_mutex_);

  // Buffer to accumulate incoming data from the underlying socket. Note that a
  // value of 64 kB for this constant was chosen arbitrarily.
  static constexpr size_t kDefaultDataBufferSize = 64 * 1024;
  std::vector<uint8_t> data_buffer_ =
      std::vector<uint8_t>(kDefaultDataBufferSize);

  // A subspan of `data_buffer_` covering all bytes occupied by received data
  // which has not yet been dispatched to the client.
  absl::Span<uint8_t> occupied_data_;

  // Buffer to accumulate incoming file descriptors from the underlying socket.
  static constexpr size_t kDefaultDescriptorBufferSize = 4;
  std::vector<FileDescriptor> descriptor_buffer_ =
      std::vector<FileDescriptor>(kDefaultDescriptorBufferSize);

  // A subspan of `descriptor_buffer_` covering all elements occupied by
  // received file descriptors which have not yet been dispatched to the client.
  absl::Span<FileDescriptor> occupied_descriptors_;

  // Client handlers for incoming messages or errors, as provided to Activate().
  MessageHandler message_handler_;
  ErrorHandler error_handler_;

  // If a Send() ever fails or only partially completes, SocketTransport copies
  // and queues any unsent contents into a DeferredMessage to be transmitted
  // ASAP once the underlying socket might no longer reject it.
  struct DeferredMessage {
    DeferredMessage();

    // Constructs a new DeferredMessage from optional header bytes and message
    // contents. Data is copied into `data`, where `header` and `message` data
    // are concatenated. Descriptors carried by `message` are moved into
    // `descriptors`.
    DeferredMessage(absl::Span<uint8_t> header, Message message);

    DeferredMessage(DeferredMessage&&);
    DeferredMessage& operator=(DeferredMessage&&);
    ~DeferredMessage();
    Message AsMessage();
    bool sent_header = false;
    std::vector<uint8_t> data;
    std::vector<FileDescriptor> descriptors;
  };

  // The queue of outgoing messages; used only if a Send() is rejected by the
  // underlying socket due to e.g. a full buffer.
  absl::Mutex queue_mutex_;
  std::vector<DeferredMessage> outgoing_queue_ ABSL_GUARDED_BY(queue_mutex_);

  // The underlying socket this object uses for I/O.
  FileDescriptor socket_;

  // State used to wake the I/O thread for various reasons other than incoming
  // messages.
  absl::Mutex notify_mutex_;
  bool is_io_thread_done_ ABSL_GUARDED_BY(notify_mutex_) = false;
  std::function<void()> shutdown_callback_ ABSL_GUARDED_BY(notify_mutex_);
  FileDescriptor signal_sender_;
  FileDescriptor signal_receiver_;
};

}  // namespace ipcz::reference_drivers

#endif  // IPCZ_SRC_REFERENCE_DRIVERS_SOCKET_TRANSPORT_H_
