/*
 * Copyright (C) 2014 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_STRING_LIST_TEAR_OFF_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_STRING_LIST_TEAR_OFF_H_

#include "third_party/blink/renderer/core/svg/properties/svg_property_tear_off.h"
#include "third_party/blink/renderer/core/svg/svg_string_list.h"
#include "third_party/blink/renderer/platform/bindings/v8_binding.h"

namespace blink {

class SVGStringListTearOff final : public SVGPropertyTearOffBase {
  DEFINE_WRAPPERTYPEINFO();

 public:
  SVGStringListTearOff(SVGStringListBase*, SVGAnimatedPropertyBase* binding);

  // SVGStringList DOM interface:

  // WebIDL requires "unsigned long" type which is uint32_t.
  uint32_t length() const { return list_->length(); }

  void clear(ExceptionState& exception_state) {
    DCHECK(!IsImmutable());
    list_->Clear();
    CommitChange(SVGPropertyCommitReason::kListCleared);
  }

  String initialize(const String& item, ExceptionState& exception_state) {
    DCHECK(!IsImmutable());
    list_->Clear();
    list_->Append(item);
    CommitChange(SVGPropertyCommitReason::kUpdated);
    return item;
  }

  String getItem(uint32_t index, ExceptionState& exception_state) const {
    if (index >= list_->length()) {
      ThrowIndexSize(exception_state, index, list_->length());
      return String();
    }
    return list_->Values()[index];
  }

  String insertItemBefore(const String& item,
                          uint32_t index,
                          ExceptionState& exception_state) {
    DCHECK(!IsImmutable());
    // Spec: If the index is greater than or equal to numberOfItems, then the
    // new item is appended to the end of the list.
    index = std::min(index, list_->length());
    // Spec: Inserts a new item into the list at the specified position. The
    // index of the item before which the new item is to be inserted. The first
    // item is number 0. If the index is equal to 0, then the new item is
    // inserted at the front of the list.
    list_->Insert(index, item);
    CommitChange(SVGPropertyCommitReason::kUpdated);
    return item;
  }

  String replaceItem(const String& item,
                     uint32_t index,
                     ExceptionState& exception_state) {
    DCHECK(!IsImmutable());
    if (index >= list_->length()) {
      ThrowIndexSize(exception_state, index, list_->length());
      return String();
    }
    list_->Replace(index, item);
    CommitChange(SVGPropertyCommitReason::kUpdated);
    return item;
  }

  IndexedPropertySetterResult AnonymousIndexedSetter(
      uint32_t index,
      const String& item,
      ExceptionState& exception_state) {
    replaceItem(item, index, exception_state);
    return IndexedPropertySetterResult::kIntercepted;
  }

  String removeItem(uint32_t index, ExceptionState& exception_state) {
    DCHECK(!IsImmutable());
    if (index >= list_->length()) {
      ThrowIndexSize(exception_state, index, list_->length());
      return String();
    }
    String removed_item = list_->Values()[index];
    list_->Remove(index);
    CommitChange(list_->length() == 0 ? SVGPropertyCommitReason::kListCleared
                                      : SVGPropertyCommitReason::kUpdated);
    return removed_item;
  }

  String appendItem(const String& item, ExceptionState& exception_state) {
    DCHECK(!IsImmutable());
    list_->Append(item);
    CommitChange(SVGPropertyCommitReason::kUpdated);
    return item;
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(list_);
    SVGPropertyTearOffBase::Trace(visitor);
  }

  const Vector<String>& Values() const { return list_->Values(); }

 private:
  const Member<SVGStringListBase> list_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_STRING_LIST_TEAR_OFF_H_
