// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_STATIC_RANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_STATIC_RANGE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/abstract_range.h"
#include "third_party/blink/renderer/core/dom/range.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Document;
class ExceptionState;
class StaticRangeInit;

class CORE_EXPORT StaticRange final : public AbstractRange {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static StaticRange* Create(const Range* range) {
    return MakeGarbageCollected<StaticRange>(
        range->OwnerDocument(), range->startContainer(), range->startOffset(),
        range->endContainer(), range->endOffset());
  }
  static StaticRange* Create(const EphemeralRange&);
  static StaticRange* Create(Document&,
                             const StaticRangeInit*,
                             ExceptionState&);

  StaticRange(Document&,
              Node* start_container,
              unsigned start_offset,
              Node* end_container,
              unsigned end_offset);

  Node* startContainer() const override { return start_container_.Get(); }
  unsigned startOffset() const override { return start_offset_; }

  Node* endContainer() const override { return end_container_.Get(); }
  unsigned endOffset() const override { return end_offset_; }

  bool collapsed() const override {
    return start_container_ == end_container_ && start_offset_ == end_offset_;
  }

  Range* toRange(ExceptionState&) const;

  bool IsValid() const;
  bool IsStaticRange() const override { return true; }
  Document& OwnerDocument() const override { return *owner_document_.Get(); }

  void Trace(Visitor*) const override;

 private:
  Member<Document> owner_document_;  // Required by |toRange()|.
  Member<Node> start_container_;
  unsigned start_offset_ = 0;
  Member<Node> end_container_;
  unsigned end_offset_ = 0;
  mutable bool is_valid_ = false;
  mutable uint64_t dom_tree_version_for_is_valid_ = 0;
};

using StaticRangeVector = HeapVector<Member<StaticRange>>;
using GCedStaticRangeVector = GCedHeapVector<Member<StaticRange>>;

template <>
struct DowncastTraits<StaticRange> {
  static bool AllowFrom(const AbstractRange& abstract_range) {
    return abstract_range.IsStaticRange();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_STATIC_RANGE_H_
