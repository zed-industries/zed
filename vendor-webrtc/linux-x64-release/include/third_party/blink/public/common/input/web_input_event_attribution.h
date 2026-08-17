// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_INPUT_EVENT_ATTRIBUTION_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_INPUT_EVENT_ATTRIBUTION_H_

#include "base/hash/hash.h"
#include "cc/paint/element_id.h"

namespace blink {

// Impl-side guidance regarding the dispatch characteristics of an event.
class WebInputEventAttribution {
 public:
  enum Type {
    // The event will be dispatched to a descendent element of
    // `target_frame_id_`.
    kTargetedFrame,
    // The event will be dispatched to the element/frame that has focus.
    kFocusedFrame,
    // The event's dispatch characteristics are unknown until running the
    // main-thread dispatch algorithm.
    kUnknown,
  };

  explicit WebInputEventAttribution(
      Type type,
      cc::ElementId target_frame_id = cc::ElementId())
      : type_(type), target_frame_id_(target_frame_id) {
    DCHECK(type == kTargetedFrame || !target_frame_id);
  }

  WebInputEventAttribution() : type_(kUnknown) {}

  Type type() const { return type_; }
  cc::ElementId target_frame_id() const { return target_frame_id_; }

  bool operator==(const WebInputEventAttribution& other) const {
    return other.type() == type_ && other.target_frame_id() == target_frame_id_;
  }

  size_t GetHash() const {
    return base::HashInts(type_, target_frame_id_.GetInternalValue());
  }

 private:
  Type type_;
  cc::ElementId target_frame_id_;  // Valid if type is kTargetedFrame.
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_INPUT_EVENT_ATTRIBUTION_H_
