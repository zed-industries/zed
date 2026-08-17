/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_MEMORY_STREAM_H_
#define RTC_BASE_MEMORY_STREAM_H_

#include <stddef.h>

#include <cstdint>

#include "api/array_view.h"
#include "rtc_base/stream.h"

namespace webrtc {

// MemoryStream dynamically resizes to accomodate written data.

class MemoryStream final : public StreamInterface {
 public:
  MemoryStream();
  ~MemoryStream() override;

  StreamState GetState() const override;
  StreamResult Read(ArrayView<uint8_t> buffer,
                    size_t& bytes_read,
                    int& error) override;
  StreamResult Write(ArrayView<const uint8_t> buffer,
                     size_t& bytes_written,
                     int& error) override;
  void Close() override;
  bool GetSize(size_t* size) const;
  bool ReserveSize(size_t size);

  bool SetPosition(size_t position);
  bool GetPosition(size_t* position) const;
  void Rewind();

  char* GetBuffer() { return buffer_; }
  const char* GetBuffer() const { return buffer_; }

  void SetData(const void* data, size_t length);

 private:
  StreamResult DoReserve(size_t size, int* error);

  // Invariant: 0 <= seek_position <= data_length_ <= buffer_length_
  char* buffer_ = nullptr;
  size_t buffer_length_ = 0;
  size_t data_length_ = 0;
  size_t seek_position_ = 0;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::MemoryStream;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_MEMORY_STREAM_H_
