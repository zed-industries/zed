// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_CURRENT_INPUT_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_CURRENT_INPUT_EVENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class WebInputEvent;

class CORE_EXPORT CurrentInputEvent {
  STATIC_ONLY(CurrentInputEvent);

 public:
  // Gets the "current" input event - event that is currently being processed by
  // either blink::WebViewImpl::HandleInputEventInternal or by
  // blink::WebFrameWidgetImpl::HandleInputEventInternal
  static const WebInputEvent* Get() { return current_input_event_; }

 private:
  friend class WebFrameWidgetImpl;
  friend class NavigationPolicyTest;

  static const WebInputEvent* current_input_event_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_CURRENT_INPUT_EVENT_H_
