// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_INCREMENT_LOAD_EVENT_DELAY_COUNT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_INCREMENT_LOAD_EVENT_DELAY_COUNT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Document;

// A helper class that will increment a document's loadEventDelayCount on
// construction and decrement it on destruction (semantics similar to RefPtr).
class CORE_EXPORT IncrementLoadEventDelayCount {
  USING_FAST_MALLOC(IncrementLoadEventDelayCount);

 public:
  explicit IncrementLoadEventDelayCount(Document&);
  IncrementLoadEventDelayCount(const IncrementLoadEventDelayCount&) = delete;
  IncrementLoadEventDelayCount& operator=(const IncrementLoadEventDelayCount&) =
      delete;
  ~IncrementLoadEventDelayCount();

  // Decrements the loadEventDelayCount and checks load event synchronously,
  // and thus can cause synchronous Document load event/JavaScript execution.
  // Call this only when it is safe, e.g. at the top of an async task.
  // After calling this, |this| no longer blocks document's load event and
  // will not decrement loadEventDelayCount at destruction.
  void ClearAndCheckLoadEvent();

  // Increments the new document's count and decrements the old count.
  void DocumentChanged(Document& new_document);

 private:
  WeakPersistent<Document> document_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_INCREMENT_LOAD_EVENT_DELAY_COUNT_H_
