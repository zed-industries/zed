// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TRANSITION_PSEUDO_ELEMENT_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TRANSITION_PSEUDO_ELEMENT_DATA_H_

#include "base/check_op.h"
#include "base/notreached.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class TransitionPseudoElementData final
    : public GarbageCollected<TransitionPseudoElementData> {
 public:
  TransitionPseudoElementData() = default;
  TransitionPseudoElementData(const TransitionPseudoElementData&) = delete;
  TransitionPseudoElementData& operator=(const TransitionPseudoElementData&) =
      delete;

  void SetPseudoElement(PseudoId,
                        PseudoElement*,
                        const AtomicString& view_transition_name = g_null_atom);
  PseudoElement* GetPseudoElement(
      PseudoId,
      const AtomicString& view_transition_name = g_null_atom) const;

  void AddPseudoElements(HeapVector<Member<PseudoElement>, 2>* result) const;

  bool HasPseudoElements() const;
  void ClearPseudoElements();
  void Trace(Visitor* visitor) const {
    visitor->Trace(transition_);
    visitor->Trace(transition_outgoing_image_);
    visitor->Trace(transition_incoming_image_);
    visitor->Trace(transition_image_wrapper_);
    visitor->Trace(transition_containers_);
  }

  bool HasViewTransitionGroupPseudoElement() const {
    return !transition_containers_.empty();
  }

 private:
  Member<PseudoElement> transition_;
  Member<PseudoElement> transition_outgoing_image_;
  Member<PseudoElement> transition_incoming_image_;
  Member<PseudoElement> transition_image_wrapper_;
  HeapHashMap<AtomicString, Member<PseudoElement>> transition_containers_;
};

inline bool TransitionPseudoElementData::HasPseudoElements() const {
  return transition_ || transition_outgoing_image_ ||
         transition_incoming_image_ || transition_image_wrapper_ ||
         !transition_containers_.empty();
}

inline void TransitionPseudoElementData::ClearPseudoElements() {
  SetPseudoElement(kPseudoIdViewTransition, nullptr);
  SetPseudoElement(kPseudoIdViewTransitionImagePair, nullptr,
                   transition_image_wrapper_
                       ? transition_image_wrapper_->view_transition_name()
                       : g_null_atom);
  SetPseudoElement(kPseudoIdViewTransitionOld, nullptr,
                   transition_outgoing_image_
                       ? transition_outgoing_image_->view_transition_name()
                       : g_null_atom);
  SetPseudoElement(kPseudoIdViewTransitionNew, nullptr,
                   transition_incoming_image_
                       ? transition_incoming_image_->view_transition_name()
                       : g_null_atom);

  for (auto& entry : transition_containers_)
    entry.value->Dispose();
  transition_containers_.clear();
}

inline void TransitionPseudoElementData::SetPseudoElement(
    PseudoId pseudo_id,
    PseudoElement* element,
    const AtomicString& view_transition_name) {
  PseudoElement* previous_element =
      GetPseudoElement(pseudo_id, view_transition_name);
  switch (pseudo_id) {
    case kPseudoIdViewTransition:
      transition_ = element;
      break;
    case kPseudoIdViewTransitionImagePair:
      DCHECK(!element ||
             element->view_transition_name() == view_transition_name);
      transition_image_wrapper_ = element;
      break;
    case kPseudoIdViewTransitionOld:
      DCHECK(!element ||
             element->view_transition_name() == view_transition_name);
      transition_outgoing_image_ = element;
      break;
    case kPseudoIdViewTransitionNew:
      DCHECK(!element ||
             element->view_transition_name() == view_transition_name);
      transition_incoming_image_ = element;
      break;
    case kPseudoIdViewTransitionGroup: {
      DCHECK(view_transition_name);
      if (element) {
        DCHECK_EQ(element->view_transition_name(), view_transition_name);
        transition_containers_.Set(view_transition_name, element);
      } else {
        transition_containers_.erase(view_transition_name);
      }
      break;
    }
    default:
      NOTREACHED();
  }

  if (previous_element)
    previous_element->Dispose();
}

inline PseudoElement* TransitionPseudoElementData::GetPseudoElement(
    PseudoId pseudo_id,
    const AtomicString& view_transition_name) const {
  switch (pseudo_id) {
    case kPseudoIdViewTransition:
      return transition_.Get();
    case kPseudoIdViewTransitionImagePair:
      DCHECK(!transition_image_wrapper_ || !view_transition_name ||
             transition_image_wrapper_->view_transition_name() ==
                 view_transition_name);
      return transition_image_wrapper_.Get();
    case kPseudoIdViewTransitionOld:
      DCHECK(!transition_outgoing_image_ || !view_transition_name ||
             transition_outgoing_image_->view_transition_name() ==
                 view_transition_name);
      return transition_outgoing_image_.Get();
    case kPseudoIdViewTransitionNew:
      DCHECK(!transition_incoming_image_ || !view_transition_name ||
             transition_incoming_image_->view_transition_name() ==
                 view_transition_name);
      return transition_incoming_image_.Get();
    case kPseudoIdViewTransitionGroup: {
      DCHECK(view_transition_name);
      auto it = transition_containers_.find(view_transition_name);
      return it == transition_containers_.end() ? nullptr : it->value.Get();
    }
    default:
      NOTREACHED();
  }
  return nullptr;
}

inline void TransitionPseudoElementData::AddPseudoElements(
    HeapVector<Member<PseudoElement>, 2>* result) const {
  if (transition_)
    result->push_back(transition_);
  if (transition_image_wrapper_)
    result->push_back(transition_image_wrapper_);
  if (transition_outgoing_image_)
    result->push_back(transition_outgoing_image_);
  if (transition_incoming_image_)
    result->push_back(transition_incoming_image_);
  for (const auto& entry : transition_containers_)
    result->push_back(entry.value);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TRANSITION_PSEUDO_ELEMENT_DATA_H_
