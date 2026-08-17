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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_LIST_PROPERTY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_LIST_PROPERTY_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/svg/properties/svg_listable_property.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

// Base class for properties that represent lists of other properties. Used to
// implement SVG*List types that hold objects (SVGLengthList, SVGNumberList,
// SVGPointList and SVGTransformList).
class SVGListPropertyBase : public SVGPropertyBase {
 protected:
  using ListType = HeapVector<Member<SVGListablePropertyBase>>;

  uint32_t length() const { return values_.size(); }
  bool IsEmpty() const { return !length(); }

  using const_iterator = typename ListType::const_iterator;
  const_iterator begin() const { return values_.begin(); }
  const_iterator end() const { return values_.end(); }

  SVGListablePropertyBase* at(uint32_t index) {
    DCHECK_LT(index, values_.size());
    DCHECK_EQ(values_[index]->OwnerList(), this);
    return values_[index].Get();
  }

  const SVGListablePropertyBase* at(uint32_t index) const {
    return const_cast<SVGListPropertyBase*>(this)->at(index);
  }

  void Clear();
  void Insert(uint32_t index, SVGListablePropertyBase* new_item);
  void Remove(uint32_t index);
  void Append(SVGListablePropertyBase* new_item);
  void Replace(uint32_t index, SVGListablePropertyBase* new_item);

 public:
  WTF::String ValueAsString() const final;

  void Trace(Visitor* visitor) const final {
    visitor->Trace(values_);
    SVGPropertyBase::Trace(visitor);
  }

 private:
  ListType values_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_LIST_PROPERTY_H_
