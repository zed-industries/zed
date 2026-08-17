// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_ACTIVATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_ACTIVATION_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_navigation_type.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/navigation_api/navigation_history_entry.h"
#include "third_party/blink/renderer/platform/bindings/script_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class CORE_EXPORT NavigationActivation final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  NavigationActivation() = default;
  ~NavigationActivation() final = default;

  void Update(NavigationHistoryEntry* entry,
              NavigationHistoryEntry* from,
              V8NavigationType::Enum navigation_type) {
    entry_ = entry;
    from_ = from;
    navigation_type_ = navigation_type;
  }

  NavigationHistoryEntry* entry() const { return entry_; }
  NavigationHistoryEntry* from() const { return from_; }
  V8NavigationType navigationType() {
    return V8NavigationType(navigation_type_);
  }

  void Trace(Visitor* visitor) const override {
    ScriptWrappable::Trace(visitor);
    visitor->Trace(entry_);
    visitor->Trace(from_);
  }

 private:
  Member<NavigationHistoryEntry> entry_;
  Member<NavigationHistoryEntry> from_;
  V8NavigationType::Enum navigation_type_ = V8NavigationType::Enum::kPush;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_ACTIVATION_H_
