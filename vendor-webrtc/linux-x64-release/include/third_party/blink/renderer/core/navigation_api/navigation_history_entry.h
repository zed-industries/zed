// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_HISTORY_ENTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_HISTORY_ENTRY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class LocalDOMWindow;
class ScriptValue;
class SerializedScriptValue;

class CORE_EXPORT NavigationHistoryEntry final : public EventTarget,
                                                 public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  NavigationHistoryEntry(LocalDOMWindow*,
                         const String& key,
                         const String& id,
                         const KURL& url,
                         int64_t document_sequence_number,
                         scoped_refptr<SerializedScriptValue>);
  ~NavigationHistoryEntry() final = default;

  NavigationHistoryEntry* Clone(LocalDOMWindow*);

  String key() const;
  String id() const;
  KURL url();
  int64_t index();
  bool sameDocument() const;

  ScriptValue getState() const;
  SerializedScriptValue* GetSerializedState() { return state_.get(); }
  void SetAndSaveState(scoped_refptr<SerializedScriptValue> state);

  DEFINE_ATTRIBUTE_EVENT_LISTENER(dispose, kDispose)

  // EventTarget overrides:
  const AtomicString& InterfaceName() const final;
  ExecutionContext* GetExecutionContext() const final {
    return ExecutionContextClient::GetExecutionContext();
  }

  void Trace(Visitor*) const final;

 private:
  String key_;
  String id_;
  KURL url_;
  int64_t document_sequence_number_;
  scoped_refptr<SerializedScriptValue> state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_HISTORY_ENTRY_H_
