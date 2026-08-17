// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_TRANSITION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_TRANSITION_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_navigation_type.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class NavigationHistoryEntry;

class CORE_EXPORT NavigationTransition final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  NavigationTransition(ExecutionContext*,
                       V8NavigationType::Enum navigation_type,
                       NavigationHistoryEntry* from);
  ~NavigationTransition() final = default;

  V8NavigationType navigationType() const {
    return V8NavigationType(navigation_type_);
  }
  NavigationHistoryEntry* from() { return from_.Get(); }
  ScriptPromise<IDLUndefined> committed(ScriptState* script_state);
  ScriptPromise<IDLUndefined> finished(ScriptState* script_state);

  void ResolveCommittedPromise();
  void ResolveFinishedPromise();
  void RejectFinishedPromise(ScriptValue ex);

  void Trace(Visitor*) const final;

 private:
  using TransitionPromise = ScriptPromiseProperty<IDLUndefined, IDLAny>;

  V8NavigationType::Enum navigation_type_;
  Member<NavigationHistoryEntry> from_;
  Member<TransitionPromise> committed_;
  Member<TransitionPromise> finished_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_TRANSITION_H_
