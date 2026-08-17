// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_DATA_CHANNEL_ATTACHMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_DATA_CHANNEL_ATTACHMENT_H_

#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/webrtc/api/data_channel_interface.h"

namespace blink {

// Used to serialize RTCDataChannels.
class MODULES_EXPORT RTCDataChannelAttachment
    : public SerializedScriptValue::Attachment {
 public:
  using NativeDataChannelVector =
      Vector<webrtc::scoped_refptr<webrtc::DataChannelInterface>>;

  static const void* const kAttachmentKey;
  RTCDataChannelAttachment() = default;
  ~RTCDataChannelAttachment() override = default;

  bool IsLockedToAgentCluster() const override {
    return !native_channels_.empty();
  }

  size_t size() const { return native_channels_.size(); }

  NativeDataChannelVector& DataChannels() { return native_channels_; }

  const NativeDataChannelVector& DataChannels() const {
    return native_channels_;
  }

 private:
  NativeDataChannelVector native_channels_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_DATA_CHANNEL_ATTACHMENT_H_
