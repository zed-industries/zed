/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_DATA_CHANNEL_UTILS_H_
#define PC_DATA_CHANNEL_UTILS_H_

#include <stddef.h>
#include <stdint.h>

#include <deque>
#include <memory>
#include <string>

#include "api/data_channel_interface.h"

namespace webrtc {

// A packet queue which tracks the total queued bytes. Queued packets are
// owned by this class.
class PacketQueue final {
 public:
  size_t byte_count() const { return byte_count_; }

  bool Empty() const;

  std::unique_ptr<DataBuffer> PopFront();

  void PushFront(std::unique_ptr<DataBuffer> packet);
  void PushBack(std::unique_ptr<DataBuffer> packet);

  void Clear();

  void Swap(PacketQueue* other);

 private:
  std::deque<std::unique_ptr<DataBuffer>> packets_;
  size_t byte_count_ = 0;
};

struct DataChannelStats {
  int internal_id;
  int id;
  std::string label;
  std::string protocol;
  DataChannelInterface::DataState state;
  uint32_t messages_sent;
  uint32_t messages_received;
  uint64_t bytes_sent;
  uint64_t bytes_received;
};

}  // namespace webrtc

#endif  // PC_DATA_CHANNEL_UTILS_H_
