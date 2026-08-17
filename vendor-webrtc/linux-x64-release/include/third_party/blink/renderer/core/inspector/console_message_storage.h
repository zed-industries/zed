// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_CONSOLE_MESSAGE_STORAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_CONSOLE_MESSAGE_STORAGE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ConsoleMessage;
class ExecutionContext;

class CORE_EXPORT ConsoleMessageStorage
    : public GarbageCollected<ConsoleMessageStorage> {
 public:
  ConsoleMessageStorage();
  ConsoleMessageStorage(const ConsoleMessageStorage&) = delete;
  ConsoleMessageStorage& operator=(const ConsoleMessageStorage&) = delete;

  // If |discard_duplicates| is set, the message will only be added if no
  // console message with the same text has exists in |messages_|. Returns
  // whether the given message was actually added.
  bool AddConsoleMessage(ExecutionContext*,
                         ConsoleMessage*,
                         bool discard_duplicates = false);
  void Clear();
  wtf_size_t size() const;
  ConsoleMessage* at(wtf_size_t index) const;
  int ExpiredCount() const;

  void Trace(Visitor*) const;

 private:
  int expired_count_;
  HeapDeque<Member<ConsoleMessage>> messages_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_CONSOLE_MESSAGE_STORAGE_H_
