// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_API_METHOD_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_API_METHOD_TRACKER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/web/web_frame_load_type.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class NavigationHistoryEntry;
class NavigationResult;
class NavigationOptions;
class ScriptState;
class SerializedScriptValue;

class NavigationApiMethodTracker final
    : public GarbageCollected<NavigationApiMethodTracker> {
 public:
  NavigationApiMethodTracker(
      ScriptState*,
      NavigationOptions*,
      const String& key,
      scoped_refptr<SerializedScriptValue> state = nullptr);

  void NotifyAboutTheCommittedToEntry(NavigationHistoryEntry*,
                                      WebFrameLoadType);
  void ResolveFinishedPromise();
  void RejectFinishedPromise(const ScriptValue& value);
  void CleanupForWillNeverSettle();

  // Note: even though this returns the same NavigationResult every time, the
  // bindings layer will create a new JS object for each distinct navigation API
  // method call, so we still match the specified semantics.
  NavigationResult* GetNavigationResult() const { return result_.Get(); }

  const ScriptValue& GetInfo() const { return info_; }
  const String& GetKey() const { return key_; }

  SerializedScriptValue* GetSerializedState() const {
    return serialized_state_.get();
  }
  void SetSerializedState(scoped_refptr<SerializedScriptValue> state) {
    serialized_state_ = state;
  }

  void Trace(Visitor* visitor) const;

 private:
  scoped_refptr<SerializedScriptValue> serialized_state_;
  ScriptValue info_;
  String key_;
  Member<NavigationHistoryEntry> committed_to_entry_;
  Member<ScriptPromiseResolver<NavigationHistoryEntry>> committed_resolver_;
  Member<ScriptPromiseResolver<NavigationHistoryEntry>> finished_resolver_;
  Member<NavigationResult> result_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_API_METHOD_TRACKER_H_
