/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 * Copyright (C) 2014 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ELEMENT_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ELEMENT_DATA_H_

#include "base/containers/span.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/core/dom/attribute.h"
#include "third_party/blink/renderer/core/dom/attribute_collection.h"
#include "third_party/blink/renderer/core/dom/space_split_string.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/bit_field.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class ShareableElementData;
class CSSPropertyValueSet;
class UniqueElementData;

// ElementData represents very common, but not necessarily unique to an element,
// data such as attributes, inline style, and parsed class names and ids.
class ElementData : public GarbageCollected<ElementData> {
 public:
  // Override GarbageCollected's finalizeGarbageCollectedObject to
  // dispatch to the correct subclass destructor.
  void FinalizeGarbageCollectedObject();

  void ClearClass() const { class_names_.Clear(); }
  void SetClass(const AtomicString& class_names) const {
    DCHECK(!class_names.empty());
    class_names_.Set(class_names);
  }
  void SetClassFoldingCase(const AtomicString& class_names) const {
    if (class_names.IsLowerASCII()) {
      return SetClass(class_names);
    }
    return SetClass(class_names.LowerASCII());
  }
  const SpaceSplitString& ClassNames() const { return class_names_; }

  const AtomicString& IdForStyleResolution() const {
    return id_for_style_resolution_;
  }
  AtomicString SetIdForStyleResolution(AtomicString new_id) const {
    return std::exchange(id_for_style_resolution_, std::move(new_id));
  }

  const CSSPropertyValueSet* InlineStyle() const { return inline_style_.Get(); }

  const CSSPropertyValueSet* PresentationAttributeStyle() const {
    return presentation_attribute_style_.Get();
  }

  AttributeCollection Attributes() const;

  bool HasID() const { return !id_for_style_resolution_.IsNull(); }
  bool HasClass() const { return !class_names_.IsNull(); }

  bool IsEquivalent(const ElementData* other) const;

  bool IsUnique() const { return bit_field_.get<IsUniqueFlag>(); }

  void TraceAfterDispatch(blink::Visitor*) const;
  void Trace(Visitor*) const;

 protected:
  using BitField = WTF::ConcurrentlyReadBitField<uint32_t>;
  using IsUniqueFlag =
      BitField::DefineFirstValue<bool, 1, WTF::BitFieldValueConstness::kConst>;
  using ArraySize = IsUniqueFlag::
      DefineNextValue<uint32_t, 28, WTF::BitFieldValueConstness::kConst>;
  using PresentationAttributeStyleIsDirty = ArraySize::DefineNextValue<bool, 1>;
  using StyleAttributeIsDirty =
      PresentationAttributeStyleIsDirty::DefineNextValue<bool, 1>;
  using SvgAttributesAreDirty = StyleAttributeIsDirty::DefineNextValue<bool, 1>;

  ElementData();
  explicit ElementData(unsigned array_size);
  ElementData(const ElementData&, bool is_unique);

  bool presentation_attribute_style_is_dirty() const {
    return bit_field_.get<PresentationAttributeStyleIsDirty>();
  }
  bool style_attribute_is_dirty() const {
    return bit_field_.get<StyleAttributeIsDirty>();
  }
  bool svg_attributes_are_dirty() const {
    return bit_field_.get<SvgAttributesAreDirty>();
  }

  // Following 3 fields are meant to be mutable and can change even when const.
  void SetPresentationAttributeStyleIsDirty(
      bool presentation_attribute_style_is_dirty) const {
    const_cast<BitField*>(&bit_field_)
        ->set<PresentationAttributeStyleIsDirty>(
            presentation_attribute_style_is_dirty);
  }
  void SetStyleAttributeIsDirty(bool style_attribute_is_dirty) const {
    const_cast<BitField*>(&bit_field_)
        ->set<StyleAttributeIsDirty>(style_attribute_is_dirty);
  }
  void SetSvgAttributesAreDirty(bool svg_attributes_are_dirty) const {
    const_cast<BitField*>(&bit_field_)
        ->set<SvgAttributesAreDirty>(svg_attributes_are_dirty);
  }

  BitField bit_field_;

  mutable Member<CSSPropertyValueSet> inline_style_;
  mutable Member<CSSPropertyValueSet> presentation_attribute_style_;
  mutable SpaceSplitString class_names_;
  mutable AtomicString id_for_style_resolution_;

 private:
  friend class Element;
  friend class HTMLImageElement;
  friend class ShareableElementData;
  friend class UniqueElementData;
  friend class SVGElement;
  friend struct DowncastTraits<UniqueElementData>;
  friend struct DowncastTraits<ShareableElementData>;

  UniqueElementData* MakeUniqueCopy() const;
};

template <typename T>
struct ThreadingTrait<T, std::enable_if_t<std::is_base_of_v<ElementData, T>>> {
  static constexpr ThreadAffinity kAffinity = kMainThreadOnly;
};

#if defined(COMPILER_MSVC)
#pragma warning(push)
// Disable "zero-sized array in struct/union" warning
#pragma warning(disable : 4200)
#endif

// SharableElementData is managed by ElementDataCache and is produced by
// the parser during page load for elements that have identical attributes. This
// is a memory optimization since it's very common for many elements to have
// duplicate sets of attributes (ex. the same classes).
class ShareableElementData final : public ElementData {
 public:
  static ShareableElementData* CreateWithAttributes(
      const Vector<Attribute, kAttributePrealloc>&);

  explicit ShareableElementData(const Vector<Attribute, kAttributePrealloc>&);
  explicit ShareableElementData(const UniqueElementData&);
  ~ShareableElementData();

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    ElementData::TraceAfterDispatch(visitor);
  }

  AttributeCollection Attributes() const;

  base::span<Attribute> AttributesSpan() {
    // SAFETY: space for bit_field_.get<ArraySize>() Attributes are allocated
    // after the main object (starting at attribute_array_) by the constructor.
    return UNSAFE_BUFFERS(
        base::span(attribute_array_, bit_field_.get<ArraySize>()));
  }

  base::span<const Attribute> AttributesSpan() const {
    // SAFETY: space for bit_field_.get<ArraySize>() Attributes are allocated
    // after the main object (starting at attribute_array_) by the constructor.
    return UNSAFE_BUFFERS(
        base::span(attribute_array_, bit_field_.get<ArraySize>()));
  }

  Attribute attribute_array_[0];
};

template <>
struct DowncastTraits<ShareableElementData> {
  static bool AllowFrom(const ElementData& data) {
    return !data.bit_field_.get<ElementData::IsUniqueFlag>();
  }
};

#if defined(COMPILER_MSVC)
#pragma warning(pop)
#endif

// UniqueElementData is created when an element needs to mutate its attributes
// or gains presentation attribute style (ex. width="10"). It does not need to
// be created to fill in values in the ElementData that are derived from
// attributes. For example populating the inline_style_ from the style attribute
// doesn't require a UniqueElementData as all elements with the same style
// attribute will have the same inline style.
class UniqueElementData final : public ElementData {
 public:
  ShareableElementData* MakeShareableCopy() const;

  MutableAttributeCollection Attributes();
  AttributeCollection Attributes() const;

  UniqueElementData();
  explicit UniqueElementData(const ShareableElementData&);
  explicit UniqueElementData(const UniqueElementData&);

  void TraceAfterDispatch(blink::Visitor*) const;

  AttributeVector attribute_vector_;
};

template <>
struct DowncastTraits<UniqueElementData> {
  static bool AllowFrom(const ElementData& data) {
    return data.bit_field_.get<ElementData::IsUniqueFlag>();
  }
};

inline AttributeCollection ElementData::Attributes() const {
  if (auto* unique_element_data = DynamicTo<UniqueElementData>(this))
    return unique_element_data->Attributes();
  return To<ShareableElementData>(this)->Attributes();
}

inline AttributeCollection ShareableElementData::Attributes() const {
  return AttributeCollection(attribute_array_, bit_field_.get<ArraySize>());
}

inline AttributeCollection UniqueElementData::Attributes() const {
  return AttributeCollection(attribute_vector_.data(),
                             attribute_vector_.size());
}

inline MutableAttributeCollection UniqueElementData::Attributes() {
  return MutableAttributeCollection(attribute_vector_);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ELEMENT_DATA_H_
