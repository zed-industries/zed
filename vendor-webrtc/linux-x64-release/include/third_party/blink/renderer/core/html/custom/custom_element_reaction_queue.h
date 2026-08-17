// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REACTION_QUEUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REACTION_QUEUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class CustomElementReaction;
class Element;

class CORE_EXPORT CustomElementReactionQueue final
    : public GarbageCollected<CustomElementReactionQueue> {
 public:
  CustomElementReactionQueue();
  CustomElementReactionQueue(const CustomElementReactionQueue&) = delete;
  CustomElementReactionQueue& operator=(const CustomElementReactionQueue&) =
      delete;
  ~CustomElementReactionQueue();

  void Trace(Visitor*) const;

  void Add(CustomElementReaction&);
  void InvokeReactions(Element&);
  bool IsEmpty() { return reactions_.empty(); }
  void Clear();

 private:
  HeapVector<Member<CustomElementReaction>, 1> reactions_;
  wtf_size_t index_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_REACTION_QUEUE_H_
