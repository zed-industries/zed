// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CASCADE_FILTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CASCADE_FILTER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/properties/css_property.h"

namespace blink {

// Reject properties with the given flags set or unset.
//
// For example, the following applies only inherited properties that don't apply
// to ::first-letter:
//
//  CascadeFilter filter;
//  filter = filter.Add(CSSProperty::kInherited, false);
//  filter = filter.Add(CSSProperty::kValidForFirstLetter, true);
//  filter.Reject(GetCSSPropertyColor());     // -> false
//  filter.Reject(GetCSSPropertyDirection()); // -> true
//  filter.Reject(GetCSSPropertyTop());       // -> true
//
class CORE_EXPORT CascadeFilter {
 public:
  // Empty filter. Rejects nothing.
  CascadeFilter() = default;

  // Creates a filter with a single rule.
  //
  // This is equivalent to:
  //
  //  CascadeFilter filter;
  //  filter = filter.Add(flag, v);
  //
  CascadeFilter(CSSProperty::Flag flag, bool v)
      : mask_(flag), flags_(v ? flag : 0) {}

  bool operator==(const CascadeFilter& o) const {
    return mask_ == o.mask_ && flags_ == o.flags_;
  }
  bool operator!=(const CascadeFilter& o) const {
    return mask_ != o.mask_ || flags_ != o.flags_;
  }

  // Add a given rule to the filter.
  //
  // A flag can be rejected when it's either set or unset. For example
  //
  //  CascadeFilter f1(CSSProperty::kInherited, true); // Rejects inherited
  //  CascadeFilter f2(CSSProperty::kInherited, false); // Rejects non-inherited
  //
  // Note that it's not possible to reject both set and unset flags in the same
  // filter. However, if you wish to reject all properties, you can do so by
  // using the CSSProperty::kProperty flag.
  //
  // Add() will have no effect if there already is a rule for the given flag:
  //
  //  CascadeFilter filter;
  //  CascadeFilter f1 = filter.Add(CSSProperty::kInherited, true);
  //  CascadeFilter f2 = f1.Add(CSSProperty::kInherited, false);
  //  bool equal = f1 == f2; // true. Second call to Add had to effect.
  //
  // If you want to overwrite a previous rule, use Set().
  CascadeFilter Add(CSSProperty::Flag flag, bool v) const {
    const CSSProperty::Flags mask = mask_ | flag;
    const CSSProperty::Flags flags =
        v ? (flags_ | (flag & ~mask_)) : (flags_ & ~(flag & ~mask_));
    return CascadeFilter(mask, flags);
  }

  // Like Add, except overwrites a previous rule for the same flag.
  CascadeFilter Set(CSSProperty::Flag flag, bool v) const {
    const CSSProperty::Flags mask = mask_ | flag;
    const CSSProperty::Flags flags = v ? (flags_ | flag) : (flags_ & ~flag);
    return CascadeFilter(mask, flags);
  }

  bool Rejects(const CSSProperty& property) const {
    return ~(property.GetFlags() ^ flags_) & mask_;
  }

  bool Rejects(CSSProperty::Flag flag, bool v) const {
    return ~((v ? flag : 0) ^ flags_) & (mask_ & flag);
  }

  bool IsEmpty() const { return mask_ == 0; }

 private:
  CascadeFilter(CSSProperty::Flags mask, CSSProperty::Flags flags)
      : mask_(mask), flags_(flags) {}
  // Specifies which bits are significant in flags_. In other words, mask_
  // contains a '1' at the corresponding position for each flag seen by
  // Add().
  CSSProperty::Flags mask_ = 0;
  // Contains the flags to exclude. Only bits set in mask_ matter.
  CSSProperty::Flags flags_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CASCADE_FILTER_H_
