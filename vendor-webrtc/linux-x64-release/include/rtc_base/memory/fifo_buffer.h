/*
 *  Copyright 2019 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_MEMORY_FIFO_BUFFER_H_
#define RTC_BASE_MEMORY_FIFO_BUFFER_H_

#include <cstddef>
#include <cstdint>
#include <memory>

#include "api/array_view.h"
#include "api/sequence_checker.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "rtc_base/stream.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// FifoBuffer allows for efficient, thread-safe buffering of data between
// writer and reader.
class FifoBuffer final : public StreamInterface {
 public:
  // Creates a FIFO buffer with the specified capacity.
  explicit FifoBuffer(size_t length);
  // Creates a FIFO buffer with the specified capacity and owner
  FifoBuffer(size_t length, Thread* owner);
  ~FifoBuffer() override;

  FifoBuffer(const FifoBuffer&) = delete;
  FifoBuffer& operator=(const FifoBuffer&) = delete;

  // Gets the amount of data currently readable from the buffer.
  bool GetBuffered(size_t* data_len) const;

  // StreamInterface methods
  StreamState GetState() const override;
  StreamResult Read(ArrayView<uint8_t> buffer,
                    size_t& bytes_read,
                    int& error) override;
  StreamResult Write(ArrayView<const uint8_t> buffer,
                     size_t& bytes_written,
                     int& error) override;
  void Close() override;

  // Seek to a byte offset from the beginning of the stream.  Returns false if
  // the stream does not support seeking, or cannot seek to the specified
  // position.
  bool SetPosition(size_t position);

  // Get the byte offset of the current position from the start of the stream.
  // Returns false if the position is not known.
  bool GetPosition(size_t* position) const;

  // Seek to the start of the stream.
  bool Rewind() { return SetPosition(0); }

  // GetReadData returns a pointer to a buffer which is owned by the stream.
  // The buffer contains data_len bytes.  null is returned if no data is
  // available, or if the method fails.  If the caller processes the data, it
  // must call ConsumeReadData with the number of processed bytes.  GetReadData
  // does not require a matching call to ConsumeReadData if the data is not
  // processed.  Read and ConsumeReadData invalidate the buffer returned by
  // GetReadData.
  const void* GetReadData(size_t* data_len);
  void ConsumeReadData(size_t used);
  // GetWriteBuffer returns a pointer to a buffer which is owned by the stream.
  // The buffer has a capacity of buf_len bytes.  null is returned if there is
  // no buffer available, or if the method fails.  The call may write data to
  // the buffer, and then call ConsumeWriteBuffer with the number of bytes
  // written.  GetWriteBuffer does not require a matching call to
  // ConsumeWriteData if no data is written.  Write and
  // ConsumeWriteData invalidate the buffer returned by GetWriteBuffer.
  void* GetWriteBuffer(size_t* buf_len);
  void ConsumeWriteBuffer(size_t used);

 private:
  void PostEvent(int events, int err) {
    RTC_DCHECK_RUN_ON(owner_);
    owner_->PostTask(
        webrtc::SafeTask(task_safety_.flag(), [this, events, err]() {
          RTC_DCHECK_RUN_ON(&callback_sequence_);
          FireEvent(events, err);
        }));
  }

  // Helper method that implements Read. Caller must acquire a lock
  // when calling this method.
  StreamResult ReadLocked(void* buffer, size_t bytes, size_t* bytes_read)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(callback_sequence_);

  // Helper method that implements Write. Caller must acquire a lock
  // when calling this method.
  StreamResult WriteLocked(const void* buffer,
                           size_t bytes,
                           size_t* bytes_written)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(callback_sequence_);

  ScopedTaskSafety task_safety_;

  // keeps the opened/closed state of the stream
  StreamState state_ RTC_GUARDED_BY(callback_sequence_);
  // the allocated buffer
  std::unique_ptr<char[]> buffer_ RTC_GUARDED_BY(callback_sequence_);
  // size of the allocated buffer
  const size_t buffer_length_;
  // amount of readable data in the buffer
  size_t data_length_ RTC_GUARDED_BY(callback_sequence_);
  // offset to the readable data
  size_t read_position_ RTC_GUARDED_BY(callback_sequence_);
  // stream callbacks are dispatched on this thread
  Thread* const owner_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::FifoBuffer;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_MEMORY_FIFO_BUFFER_H_
