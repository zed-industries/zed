// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COUNTERS_ATTACHMENT_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COUNTERS_ATTACHMENT_CONTEXT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class Element;
class LayoutObject;

// This class is used to keep track of the current counter values for a
// document.
// It is used to calculate the counter() and counters() CSS function values.
class CORE_EXPORT CountersAttachmentContext {
  STACK_ALLOCATED();

  struct CounterEntry : public GarbageCollected<CounterEntry> {
    CounterEntry(const LayoutObject& layout_object, int value)
        : layout_object(&layout_object), value(value) {}
    Member<const LayoutObject> layout_object;
    int value;

    void Trace(Visitor*) const;
  };

  using CounterStack = GCedHeapVector<Member<CounterEntry>>;
  using CounterInheritanceTable =
      GCedHeapHashMap<AtomicString, Member<CounterStack>>;

 public:
  enum class Type {
    kIncrementType = 1 << 0,
    kResetType = 1 << 1,
    kSetType = 1 << 2
  };

  CountersAttachmentContext();

  CountersAttachmentContext(CountersAttachmentContext&&) = default;
  CountersAttachmentContext& operator=(CountersAttachmentContext&&) = default;

  CountersAttachmentContext& operator=(const CountersAttachmentContext&) =
      delete;

  // Create a shallow copy of this object, meaning that the counter stacks will
  // be shared between this and the copy.
  CountersAttachmentContext ShallowClone() const {
    return CountersAttachmentContext(*this);
  }

  // Create a deep copy of this object, including all counter stacks.
  CountersAttachmentContext DeepClone() const;

  void EnterObject(const LayoutObject&, bool is_page_box = false);
  void LeaveObject(const LayoutObject&, bool is_page_box = false);

  // only_last = true for counter(), = false for counters().
  Vector<int> GetCounterValues(const LayoutObject&,
                               const AtomicString& counter_name,
                               bool only_last);
  void SetAttachmentRootIsDocumentElement() {
    attachment_root_is_document_element_ = true;
  }
  bool AttachmentRootIsDocumentElement() const {
    return attachment_root_is_document_element_;
  }
  static bool ElementGeneratesListItemCounter(const Element& element);

 private:
  // The default copy constructor can be used to create shallow copies.
  CountersAttachmentContext(const CountersAttachmentContext&) = default;

  void ProcessCounter(const LayoutObject& layout_object,
                      const AtomicString& counter_name,
                      unsigned counter_type,
                      int value_argument,
                      bool is_page_box);

  // When a counter is incremented or reset in a page or page margin context,
  // this may obscure all counters of the same name within the document. When
  // this happens, insert a boundary on the stack, create a new counter, and
  // return true.
  bool ObscurePageCounterIfNeeded(const LayoutObject& layout_object,
                                  const AtomicString& counter_name,
                                  unsigned counter_type,
                                  int value_argument,
                                  bool is_page_box);
  void UnobscurePageCounterIfNeeded(const AtomicString& counter_name,
                                    unsigned counter_type,
                                    bool is_page_box);

  void CreateCounter(const LayoutObject&,
                     const AtomicString& counter_name,
                     int value);
  void RemoveStaleCounters(const LayoutObject&,
                           const AtomicString& counter_name);
  void RemoveCounterIfAncestorExists(const LayoutObject&,
                                     const AtomicString& counter_name);
  void UpdateCounterValue(const LayoutObject&,
                          const AtomicString& counter_name,
                          unsigned counter_type,
                          int counter_value);
  void MaybeCreateListItemCounter(const Element& element);
  void EnterStyleContainmentScope();
  void LeaveStyleContainmentScope();

  AtomicString list_item_{"list-item"};
  // True if attachment started from documentElement. If true, counters
  // calculations are done as part of layout tree attachment.
  bool attachment_root_is_document_element_ = false;
  // Hash table of counter name <-> counters stack, for inheritance.
  CounterInheritanceTable* counter_inheritance_table_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COUNTERS_ATTACHMENT_CONTEXT_H_
