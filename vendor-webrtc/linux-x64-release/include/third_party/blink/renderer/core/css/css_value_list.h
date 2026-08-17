/*
 * (C) 1999-2003 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010 Apple Inc. All rights
 * reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_VALUE_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_VALUE_LIST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class CORE_EXPORT CSSValueList : public CSSValue {
 public:
  using const_iterator = HeapVector<Member<const CSSValue>, 4>::const_iterator;
  using const_reverse_iterator =
      HeapVector<Member<const CSSValue>, 4>::const_reverse_iterator;

  static CSSValueList* CreateCommaSeparated() {
    return MakeGarbageCollected<CSSValueList>(kCommaSeparator);
  }
  static CSSValueList* CreateSpaceSeparated() {
    return MakeGarbageCollected<CSSValueList>(kSpaceSeparator);
  }
  static CSSValueList* CreateSlashSeparated() {
    return MakeGarbageCollected<CSSValueList>(kSlashSeparator);
  }
  static CSSValueList* CreateWithSeparatorFrom(const CSSValueList& list) {
    return MakeGarbageCollected<CSSValueList>(
        static_cast<ValueListSeparator>(list.value_list_separator_));
  }

  CSSValueList(ClassType, ValueListSeparator);
  explicit CSSValueList(ValueListSeparator);
  CSSValueList(ValueListSeparator, HeapVector<Member<const CSSValue>, 4>);
  CSSValueList(const CSSValueList&) = delete;
  CSSValueList& operator=(const CSSValueList&) = delete;

  const_iterator begin() const { return values_.begin(); }
  const_iterator end() const { return values_.end(); }
  const_reverse_iterator rbegin() const { return values_.rbegin(); }
  const_reverse_iterator rend() const { return values_.rend(); }

  wtf_size_t length() const { return values_.size(); }
  const CSSValue& Item(wtf_size_t index) const { return *values_[index]; }
  const CSSValue& First() const { return *values_.front(); }
  const CSSValue& Last() const { return *values_.back(); }

  void Append(const CSSValue& value);
  bool RemoveAll(const CSSValue&);
  bool HasValue(const CSSValue&) const;
  CSSValueList* Copy() const;

  WTF::String CustomCSSText() const;
  bool Equals(const CSSValueList&) const;
  unsigned CustomHash() const;

  const CSSValueList& PopulateWithTreeScope(const TreeScope*) const;

  bool HasFailedOrCanceledSubresources() const;

  bool MayContainUrl() const;
  void ReResolveUrl(const Document&) const;

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  HeapVector<Member<const CSSValue>, 4> values_;
};

template <>
struct DowncastTraits<CSSValueList> {
  static bool AllowFrom(const CSSValue& value) { return value.IsValueList(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_VALUE_LIST_H_
