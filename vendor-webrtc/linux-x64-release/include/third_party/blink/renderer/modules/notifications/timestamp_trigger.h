// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_TIMESTAMP_TRIGGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_TIMESTAMP_TRIGGER_H_

#include "third_party/blink/renderer/core/dom/dom_time_stamp.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class MODULES_EXPORT TimestampTrigger : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static TimestampTrigger* Create(const DOMTimeStamp& timestamp);

  explicit TimestampTrigger(const DOMTimeStamp& timestamp);

  DOMTimeStamp timestamp() const { return timestamp_; }

 private:
  DOMTimeStamp timestamp_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_TIMESTAMP_TRIGGER_H_
