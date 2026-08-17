// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_DESTINATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_DESTINATION_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/navigation_api/navigate_event_dispatch_params.h"
#include "third_party/blink/renderer/core/navigation_api/navigation_history_entry.h"
#include "third_party/blink/renderer/platform/bindings/script_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class CORE_EXPORT NavigationDestination final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  NavigationDestination(NavigateEventDispatchParams* params,
                        SerializedScriptValue* state,
                        NavigationHistoryEntry* entry)
      : dispatch_params_(params), state_(state), entry_(entry) {}
  ~NavigationDestination() final = default;

  String key() const { return entry_ ? entry_->key() : String(); }
  String id() const { return entry_ ? entry_->id() : String(); }
  const KURL& url() const { return dispatch_params_->url; }
  int64_t index() const { return entry_ ? entry_->index() : -1; }
  bool sameDocument() const {
    return dispatch_params_->event_type != NavigateEventType::kCrossDocument;
  }
  v8::Local<v8::Value> getState(v8::Isolate* isolate) {
    return state_ ? state_->Deserialize(isolate) : v8::Local<v8::Value>();
  }
  void SetSerializedState(scoped_refptr<SerializedScriptValue> state) {
    state_ = state;
  }

  void Trace(Visitor* visitor) const override {
    ScriptWrappable::Trace(visitor);
    visitor->Trace(dispatch_params_);
    visitor->Trace(entry_);
  }

 private:
  Member<NavigateEventDispatchParams> dispatch_params_;
  scoped_refptr<SerializedScriptValue> state_;
  Member<NavigationHistoryEntry> entry_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_DESTINATION_H_
