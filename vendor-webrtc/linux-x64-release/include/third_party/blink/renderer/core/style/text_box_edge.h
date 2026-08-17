// Copyright 2024 The Chromium Authors.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_TEXT_BOX_EDGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_TEXT_BOX_EDGE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

//
// Represents the CSS `text-box-edge` property.
// https://drafts.csswg.org/css-inline-3/#propdef-text-box-edge
//
class CORE_EXPORT TextBoxEdge {
  DISALLOW_NEW();

 public:
  // https://drafts.csswg.org/css-inline-3/#text-edges
  enum class Type : uint8_t {
    kAuto,
    kText,
    kCap,
    kEx,
    kAlphabetic,

    // kIdeographic, not implemented.
    // kIdeographicInk, not implemented.

    // When adding values, ensure the following constants and the `field_size`
    // in `css_properties.json5` are in sync.
  };

  static constexpr unsigned kTypeBits = 3;
  static constexpr unsigned kTypeMask = (1 << kTypeBits) - 1;
  // The number of bits needed when storing `TextBoxEdge` as `unsigned`.
  static constexpr unsigned kBits = kTypeBits * 2;

  constexpr TextBoxEdge() = default;
  explicit constexpr TextBoxEdge(Type over)
      : TextBoxEdge(over, UnderForOverChecked(over)) {}
  constexpr TextBoxEdge(Type over, Type under) : over_(over), under_(under) {
    DCHECK(over == Type::kAuto ? under == Type::kAuto : under != Type::kAuto);
  }

  // Convert from/to `unsigned` to store in a bit field in `ComputedStyle`.
  explicit constexpr TextBoxEdge(unsigned value)
      : TextBoxEdge(static_cast<Type>(value & kTypeMask),
                    static_cast<Type>((value >> kTypeBits) & kTypeMask)) {}
  explicit constexpr operator unsigned() const {
    return static_cast<unsigned>(over_) |
           (static_cast<unsigned>(under_) << kTypeBits);
  }

  bool operator==(const TextBoxEdge& other) const {
    return over_ == other.Over() && under_ == other.Under();
  }
  bool operator!=(const TextBoxEdge& other) const { return !(*this == other); }

  const Type& Over() const { return over_; }
  const Type& Under() const { return under_; }

  bool IsAuto() const { return Over() == Type::kAuto; }

  // The default `under` value for the given `over` value. It can be omitted
  // only for some `over` values. `nullopt` means that the `under` value is
  // required for the `over` value.
  static constexpr std::optional<Type> UnderForOver(Type over);
  bool IsUnderDefault() const { return Under() == UnderForOver(Over()); }

 private:
  static constexpr Type UnderForOverChecked(Type over);

  Type over_ = Type::kAuto;
  Type under_ = Type::kAuto;
};

// The initial value being 0 is preferred for performance reasons.
static_assert(static_cast<unsigned>(TextBoxEdge()) == 0);

constexpr std::optional<TextBoxEdge::Type> TextBoxEdge::UnderForOver(
    Type over) {
  switch (over) {
    case Type::kAuto:
    case Type::kText:
      return over;

      // These values require authors to specify the `under` value.
      // https://github.com/w3c/csswg-drafts/issues/10703
    case Type::kCap:
    case Type::kEx:
      return std::nullopt;

    case Type::kAlphabetic:
      NOTREACHED();
  }
}

constexpr TextBoxEdge::Type TextBoxEdge::UnderForOverChecked(Type over) {
  const std::optional<TextBoxEdge::Type> under = UnderForOver(over);
  CHECK(under);
  return *under;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_TEXT_BOX_EDGE_H_
