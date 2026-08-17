// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_TRANSFORM_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_TRANSFORM_EVENT_H_

#include "third_party/blink/renderer/core/messaging/blink_transferable_message.h"
#include "third_party/blink/renderer/core/workers/custom_event_message.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_script_transform.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_handle.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class RTCRtpScriptTransformer;
class MODULES_EXPORT RTCTransformEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit RTCTransformEvent(
      ScriptState* script_state,
      CustomEventMessage data,
      scoped_refptr<base::SequencedTaskRunner> transform_task_runner,
      CrossThreadWeakHandle<RTCRtpScriptTransform> transform);
  ~RTCTransformEvent() override = default;

  RTCRtpScriptTransformer* transformer() const { return transformer_; }

  void Trace(Visitor* visitor) const override;

 private:
  const Member<RTCRtpScriptTransformer> transformer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_TRANSFORM_EVENT_H_
