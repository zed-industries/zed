// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SETLIKE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SETLIKE_H_

#include "base/containers/contains.h"
#include "third_party/blink/renderer/bindings/core/v8/iterable.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"

namespace blink {

// Utility class to simplify implementation of various `setlike` classes.
// The consumer of the class needs only to implement the |elements()| method -
// everything else should be provided by this class. For examples, see
// `XRPlaneSet` and `XRAnchorSet`.
template <typename InterfaceType, typename ElementType>
class XRSetlike : public ValueSyncIterable<InterfaceType> {
 public:
  unsigned size() const { return elements().size(); }

  // Returns true if passed in |element| is a member of the XRSetlike.
  bool hasForBinding(ScriptState* script_state,
                     ElementType* element,
                     ExceptionState& exception_state) const {
    DCHECK(element);
    auto all_elements = elements();
    return base::Contains(all_elements, element);
  }

 protected:
  virtual const HeapHashSet<Member<ElementType>>& elements() const = 0;

 private:
  class IterationSource final
      : public ValueSyncIterable<InterfaceType>::IterationSource {
   public:
    explicit IterationSource(const HeapHashSet<Member<ElementType>>& elements)
        : index_(0) {
      elements_.ReserveInitialCapacity(elements.size());
      for (auto element : elements) {
        elements_.push_back(element);
      }
    }

    bool FetchNextItem(ScriptState* script_state,
                       ElementType*& value,
                       ExceptionState& exception_state) override {
      if (index_ >= elements_.size()) {
        return false;
      }

      value = elements_[index_];
      ++index_;

      return true;
    }

    void Trace(Visitor* visitor) const override {
      visitor->Trace(elements_);
      ValueSyncIterable<InterfaceType>::IterationSource::Trace(visitor);
    }

   private:
    HeapVector<Member<ElementType>> elements_;

    unsigned index_;
  };

  // Starts iteration over XRSetlike.
  // Needed for ValueSyncIterable to work properly.
  XRSetlike::IterationSource* CreateIterationSource(
      ScriptState* script_state,
      ExceptionState& exception_state) override {
    return MakeGarbageCollected<XRSetlike::IterationSource>(elements());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SETLIKE_H_
