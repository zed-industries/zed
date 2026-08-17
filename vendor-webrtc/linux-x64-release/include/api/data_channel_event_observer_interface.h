/*
 *  Copyright (c) 2025 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_DATA_CHANNEL_EVENT_OBSERVER_INTERFACE_H_
#define API_DATA_CHANNEL_EVENT_OBSERVER_INTERFACE_H_

#include <cstdint>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"

namespace webrtc {

// TODO: issues.chromium.org/407785197 - Maybe update the observer to also
// notify on controll messages as well.
// TODO: issues.chromium.org/407785197 - Remove comment below when DataChannel
// logging has been launched.
// NOTE: This class is still under development and may change without notice.
class DataChannelEventObserverInterface {
 public:
  virtual ~DataChannelEventObserverInterface() = default;

  class Message {
   public:
    enum class Direction { kSend, kReceive };
    enum class DataType { kString, kBinary };

    // When `direction` is `kSend` the timestamp represent when the message was
    // handed over to the transport, if `direction` is `kReceive` then it
    // represent when the message was received from the transport.
    int64_t unix_timestamp_ms() const { return unix_timestamp_; }
    void set_unix_timestamp_ms(int64_t timestamp) {
      unix_timestamp_ = timestamp;
    }

    int datachannel_id() const { return datachannel_id_; }
    void set_datachannel_id(int id) { datachannel_id_ = id; }

    absl::string_view label() const { return label_; }
    void set_label(absl::string_view label) { label_ = std::string(label); }

    Direction direction() const { return direction_; }
    void set_direction(Direction direction) { direction_ = direction; }

    DataType data_type() const { return data_type_; }
    void set_data_type(DataType type) { data_type_ = type; }

    const std::vector<uint8_t>& data() const { return data_; }
    void set_data(ArrayView<const uint8_t> d) {
      data_.assign(d.begin(), d.end());
    }

   private:
    int64_t unix_timestamp_;
    int datachannel_id_;
    std::string label_;
    Direction direction_;
    DataType data_type_;
    std::vector<uint8_t> data_;
  };

  virtual void OnMessage(const Message& message) = 0;
};

}  // namespace webrtc

#endif  // API_DATA_CHANNEL_EVENT_OBSERVER_INTERFACE_H_
