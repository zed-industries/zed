/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_LIST_PROPERTY_TEAR_OFF_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_LIST_PROPERTY_TEAR_OFF_HELPER_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/svg/properties/svg_property_tear_off.h"
#include "third_party/blink/renderer/platform/bindings/v8_binding.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"

namespace blink {

// This is an implementation of the SVG*List property spec:
// https://svgwg.org/svg2-draft/types.html#ListInterfaces
template <typename Derived, typename ListProperty>
class SVGListPropertyTearOffHelper : public SVGPropertyTearOff<ListProperty> {
 public:
  typedef ListProperty ListPropertyType;
  typedef typename ListPropertyType::ItemPropertyType ItemPropertyType;
  typedef typename ItemPropertyType::TearOffType ItemTearOffType;

  // SVG*List DOM interface:

  // WebIDL requires "unsigned long" which is "uint32_t".
  uint32_t length() { return ToDerived()->Target()->length(); }

  void clear(ExceptionState& exception_state) {
    if (ToDerived()->IsImmutable()) {
      SVGPropertyTearOffBase::ThrowReadOnly(exception_state);
      return;
    }
    ToDerived()->Target()->Clear();
    ToDerived()->CommitChange(SVGPropertyCommitReason::kListCleared);
  }

  ItemTearOffType* initialize(ItemTearOffType* item,
                              ExceptionState& exception_state) {
    if (ToDerived()->IsImmutable()) {
      SVGPropertyTearOffBase::ThrowReadOnly(exception_state);
      return nullptr;
    }
    DCHECK(item);
    ItemPropertyType* value = GetValueForInsertionFromTearOff(item);
    // Spec: Clears all existing current items from the list and re-initializes
    // the list to hold the single item specified by the parameter.
    ListPropertyType* list = ToDerived()->Target();
    list->Clear();
    list->Append(value);
    ToDerived()->CommitChange(SVGPropertyCommitReason::kUpdated);
    return AttachedItemTearOff(value);
  }

  ItemTearOffType* getItem(uint32_t index, ExceptionState& exception_state) {
    ListPropertyType* list = ToDerived()->Target();
    if (index >= list->length()) {
      SVGPropertyTearOffBase::ThrowIndexSize(exception_state, index,
                                             list->length());
      return nullptr;
    }
    ItemPropertyType* value = list->at(index);
    return AttachedItemTearOff(value);
  }

  ItemTearOffType* insertItemBefore(ItemTearOffType* item,
                                    uint32_t index,
                                    ExceptionState& exception_state) {
    if (ToDerived()->IsImmutable()) {
      SVGPropertyTearOffBase::ThrowReadOnly(exception_state);
      return nullptr;
    }
    DCHECK(item);
    ItemPropertyType* value = GetValueForInsertionFromTearOff(item);
    ListPropertyType* list = ToDerived()->Target();
    // Spec: If the index is greater than or equal to length, then the new item
    // is appended to the end of the list.
    index = std::min(index, list->length());
    // Spec: Inserts a new item into the list at the specified position. The
    // index of the item before which the new item is to be inserted. The first
    // item is number 0. If the index is equal to 0, then the new item is
    // inserted at the front of the list.
    list->Insert(index, value);
    ToDerived()->CommitChange(SVGPropertyCommitReason::kUpdated);
    return AttachedItemTearOff(value);
  }

  ItemTearOffType* replaceItem(ItemTearOffType* item,
                               uint32_t index,
                               ExceptionState& exception_state) {
    if (ToDerived()->IsImmutable()) {
      SVGPropertyTearOffBase::ThrowReadOnly(exception_state);
      return nullptr;
    }
    ListPropertyType* list = ToDerived()->Target();
    if (index >= list->length()) {
      SVGPropertyTearOffBase::ThrowIndexSize(exception_state, index,
                                             list->length());
      return nullptr;
    }
    DCHECK(item);
    ItemPropertyType* value = GetValueForInsertionFromTearOff(item);
    list->Replace(index, value);
    ToDerived()->CommitChange(SVGPropertyCommitReason::kUpdated);
    return AttachedItemTearOff(value);
  }

  IndexedPropertySetterResult AnonymousIndexedSetter(
      uint32_t index,
      ItemTearOffType* item,
      ExceptionState& exception_state) {
    replaceItem(item, index, exception_state);
    return IndexedPropertySetterResult::kIntercepted;
  }

  ItemTearOffType* removeItem(uint32_t index, ExceptionState& exception_state) {
    if (ToDerived()->IsImmutable()) {
      SVGPropertyTearOffBase::ThrowReadOnly(exception_state);
      return nullptr;
    }
    ListPropertyType* list = ToDerived()->Target();
    if (index >= list->length()) {
      SVGPropertyTearOffBase::ThrowIndexSize(exception_state, index,
                                             list->length());
      return nullptr;
    }
    ItemPropertyType* value = list->at(index);
    list->Remove(index);
    ToDerived()->CommitChange(list->IsEmpty()
                                  ? SVGPropertyCommitReason::kListCleared
                                  : SVGPropertyCommitReason::kUpdated);
    return DetachedItemTearOff(value);
  }

  ItemTearOffType* appendItem(ItemTearOffType* item,
                              ExceptionState& exception_state) {
    if (ToDerived()->IsImmutable()) {
      SVGPropertyTearOffBase::ThrowReadOnly(exception_state);
      return nullptr;
    }
    DCHECK(item);
    ItemPropertyType* value = GetValueForInsertionFromTearOff(item);
    ToDerived()->Target()->Append(value);
    ToDerived()->CommitChange(SVGPropertyCommitReason::kUpdated);
    return AttachedItemTearOff(value);
  }

 protected:
  SVGListPropertyTearOffHelper(ListPropertyType* target,
                               SVGAnimatedPropertyBase* binding,
                               PropertyIsAnimValType property_is_anim_val)
      : SVGPropertyTearOff<ListPropertyType>(target,
                                             binding,
                                             property_is_anim_val) {}

  ItemPropertyType* GetValueForInsertionFromTearOff(
      ItemTearOffType* item_tear_off) {
    ItemPropertyType* item = item_tear_off->Target();
    // |new_item| is immutable, OR
    // |new_item| belongs to a SVGElement, but it does not belong to an animated
    // list, e.g. "textElement.x.baseVal.appendItem(rectElement.width.baseVal)"
    // Spec: If |new_item| is already in a list, then a new object is created
    // with the same values as |new_item| and this item is inserted into the
    // list. Otherwise, |new_item| itself is inserted into the list.
    if (item_tear_off->IsImmutable() || item->OwnerList() ||
        item_tear_off->ContextElement()) {
      // We have to copy the incoming |new_item|,
      // otherwise we'll end up having two tear-offs that operate on the same
      // SVGProperty. Consider the example below: SVGRectElements
      // SVGAnimatedLength 'width' property baseVal points to the same tear-off
      // object that's inserted into SVGTextElements SVGAnimatedLengthList 'x'.
      // textElement.x.baseVal.getItem(0).value += 150 would mutate the
      // rectElement width _and_ the textElement x list. That's obviously wrong,
      // take care of that.
      return item->Clone();
    }
    item_tear_off->Bind(ToDerived()->GetBinding());
    return item;
  }
  ItemTearOffType* AttachedItemTearOff(ItemPropertyType* value) {
    DCHECK(value);
    DCHECK_EQ(value->OwnerList(), ToDerived()->Target());
    return MakeGarbageCollected<ItemTearOffType>(
        value, ToDerived()->GetBinding(), ToDerived()->PropertyIsAnimVal());
  }
  ItemTearOffType* DetachedItemTearOff(ItemPropertyType* value) {
    DCHECK(value);
    DCHECK_EQ(value->OwnerList(), nullptr);
    return MakeGarbageCollected<ItemTearOffType>(value, nullptr,
                                                 kPropertyIsNotAnimVal);
  }

 private:
  Derived* ToDerived() { return static_cast<Derived*>(this); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_LIST_PROPERTY_TEAR_OFF_HELPER_H_
