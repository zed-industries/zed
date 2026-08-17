/*
 * Copyright 2025 LiveKit, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once

#include <memory>
#include <mutex>

#include "api/data_channel_interface.h"
#include "livekit/webrtc.h"
#include "rtc_base/synchronization/mutex.h"
#include "rust/cxx.h"

namespace livekit_ffi {
class DataChannel;
}  // namespace livekit_ffi
#include "webrtc-sys/src/data_channel.rs.h"

namespace livekit_ffi {

class NativeDataChannelObserver;

webrtc::DataChannelInit to_native_data_channel_init(DataChannelInit init);

class DataChannel {
 public:
  explicit DataChannel(
      std::shared_ptr<RtcRuntime> rtc_runtime,
      webrtc::scoped_refptr<webrtc::DataChannelInterface> data_channel);
  ~DataChannel();

  void register_observer(rust::Box<DataChannelObserverWrapper> observer) const;
  void unregister_observer() const;
  bool send(const DataBuffer& buffer) const;
  int id() const;
  rust::String label() const;
  DataState state() const;
  void close() const;
  uint64_t buffered_amount() const;

 private:
  mutable webrtc::Mutex mutex_;
  std::shared_ptr<RtcRuntime> rtc_runtime_;
  webrtc::scoped_refptr<webrtc::DataChannelInterface> data_channel_;
  mutable std::unique_ptr<NativeDataChannelObserver> observer_;
};

static std::shared_ptr<DataChannel> _shared_data_channel() {
  return nullptr;  // Ignore
}

class NativeDataChannelObserver : public webrtc::DataChannelObserver {
 public:
  NativeDataChannelObserver(rust::Box<DataChannelObserverWrapper> observer,
                            const DataChannel* dc);

  void OnStateChange() override;
  void OnMessage(const webrtc::DataBuffer& buffer) override;
  void OnBufferedAmountChange(uint64_t sent_data_size) override;

 private:
  rust::Box<DataChannelObserverWrapper> observer_;
  const DataChannel* dc_;
};

}  // namespace livekit_ffi
