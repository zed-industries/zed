// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_BEFORE_CREATE_POLICY_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_BEFORE_CREATE_POLICY_EVENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CORE_EXPORT BeforeCreatePolicyEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static BeforeCreatePolicyEvent* Create(const String&);
  explicit BeforeCreatePolicyEvent(const String&);
  ~BeforeCreatePolicyEvent() override;

  bool IsBeforeCreatePolicyEvent() const override;

  void setPolicyName(const String& policy_name) { policy_name_ = policy_name; }

  String policyName() const { return policy_name_; }

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  String policy_name_;
};

template <>
struct DowncastTraits<BeforeCreatePolicyEvent> {
  static bool AllowFrom(const Event& event) {
    return event.IsBeforeCreatePolicyEvent();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_BEFORE_CREATE_POLICY_EVENT_H_
