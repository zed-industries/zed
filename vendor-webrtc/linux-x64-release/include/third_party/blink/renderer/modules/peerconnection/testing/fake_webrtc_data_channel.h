// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TESTING_FAKE_WEBRTC_DATA_CHANNEL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TESTING_FAKE_WEBRTC_DATA_CHANNEL_H_

#include "third_party/webrtc/api/data_channel_interface.h"

namespace webrtc {
class DataChannelObserver;
}

namespace blink {

class FakeWebRTCDataChannel : public webrtc::DataChannelInterface {
 public:
  static webrtc::scoped_refptr<FakeWebRTCDataChannel> Create();

  FakeWebRTCDataChannel() = default;
  ~FakeWebRTCDataChannel() override = default;

  void RegisterObserver(webrtc::DataChannelObserver* observer) override;
  void UnregisterObserver() override;

  std::string label() const override { return "FakeChannel"; }
  bool reliable() const override { return true; }
  int id() const override { return 1; }
  DataState state() const override { return state_; }
  uint32_t messages_sent() const override { return 0; }
  uint64_t bytes_sent() const override { return 0; }
  uint32_t messages_received() const override { return 0; }
  uint64_t bytes_received() const override { return 0; }
  uint64_t buffered_amount() const override { return 0; }
  void Close() override;

  bool close_was_called() { return close_called_; }
  bool register_call_count() { return register_observer_call_count_; }
  bool unregister_call_count() { return unregister_observer_call_count_; }

 private:
  DataState state_ = DataState::kConnecting;
  bool close_called_ = false;
  int register_observer_call_count_ = 0;
  int unregister_observer_call_count_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TESTING_FAKE_WEBRTC_DATA_CHANNEL_H_
