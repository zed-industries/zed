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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_LIST_PROPERTY_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_LIST_PROPERTY_HELPER_H_

#include "base/compiler_specific.h"
#include "third_party/blink/renderer/core/svg/properties/svg_list_property.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// Typed wrapper for SVG*List properties that adds type-dependent operations.
template <typename Derived, typename ItemProperty>
class SVGListPropertyHelper : public SVGListPropertyBase {
 public:
  typedef ItemProperty ItemPropertyType;

  SVGListPropertyHelper() = default;
  ~SVGListPropertyHelper() override = default;

  class const_iterator {
   public:
    explicit const_iterator(SVGListPropertyBase::const_iterator wrapped)
        : wrapped_(wrapped) {}

    const_iterator& operator++() {
      UNSAFE_TODO(++wrapped_);
      return *this;
    }
    bool operator==(const const_iterator& other) const {
      return wrapped_ == other.wrapped_;
    }
    bool operator!=(const const_iterator& other) const {
      return !operator==(other);
    }
    const ItemPropertyType* operator->() const {
      return To<ItemPropertyType>(wrapped_->Get());
    }
    const ItemPropertyType* operator*() const {
      return To<ItemPropertyType>(wrapped_->Get());
    }

   private:
    SVGListPropertyBase::const_iterator wrapped_;
  };
  const_iterator begin() const {
    return const_iterator(SVGListPropertyBase::begin());
  }
  const_iterator end() const {
    return const_iterator(SVGListPropertyBase::end());
  }

  using SVGListPropertyBase::IsEmpty;
  using SVGListPropertyBase::length;

  ItemPropertyType* at(uint32_t index) {
    return To<ItemPropertyType>(SVGListPropertyBase::at(index));
  }

  const ItemPropertyType* at(uint32_t index) const {
    return To<ItemPropertyType>(SVGListPropertyBase::at(index));
  }

  using SVGListPropertyBase::Clear;
  void Insert(uint32_t index, ItemPropertyType* new_item) {
    SVGListPropertyBase::Insert(index, new_item);
  }
  using SVGListPropertyBase::Remove;
  void Append(ItemPropertyType* new_item) {
    SVGListPropertyBase::Append(new_item);
  }
  void Replace(uint32_t index, ItemPropertyType* new_item) {
    SVGListPropertyBase::Replace(index, new_item);
  }

  virtual Derived* Clone() const {
    auto* svg_list = MakeGarbageCollected<Derived>();
    svg_list->DeepCopy(To<Derived>(this));
    return svg_list;
  }

  AnimatedPropertyType GetType() const override { return Derived::ClassType(); }

 protected:
  void DeepCopy(const Derived*);

  bool AdjustFromToListValues(const Derived* from_list,
                              const Derived* to_list,
                              float percentage);

  virtual ItemPropertyType* CreatePaddingItem() const {
    return MakeGarbageCollected<ItemPropertyType>();
  }
};

template <typename Derived, typename ItemProperty>
void SVGListPropertyHelper<Derived, ItemProperty>::DeepCopy(
    const Derived* from) {
  Clear();
  for (const auto* from_value : *from)
    Append(from_value->Clone());
}

template <typename Derived, typename ItemProperty>
bool SVGListPropertyHelper<Derived, ItemProperty>::AdjustFromToListValues(
    const Derived* from_list,
    const Derived* to_list,
    float percentage) {
  // If no 'to' value is given, nothing to animate.
  uint32_t to_list_size = to_list->length();
  if (!to_list_size)
    return false;

  // If the 'from' value is given and it's length doesn't match the 'to' value
  // list length, fallback to a discrete animation.
  uint32_t from_list_size = from_list->length();
  if (from_list_size != to_list_size && from_list_size) {
    const Derived* result = percentage < 0.5 ? from_list : to_list;
    // If this is a 'to' animation, the "from" value will be the same
    // list as this list, so avoid the copy in that case since it
    // would clobber the list.
    if (result != this)
      DeepCopy(result);
    return false;
  }

  DCHECK(!from_list_size || from_list_size == to_list_size);
  for (uint32_t i = length(); i < to_list_size; ++i)
    Append(CreatePaddingItem());

  return true;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_LIST_PROPERTY_HELPER_H_
