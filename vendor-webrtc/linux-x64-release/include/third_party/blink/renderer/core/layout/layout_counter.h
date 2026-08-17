/*
 * Copyright (C) 2004 Allan Sandfeld Jensen (kde@carewolf.com)
 * Copyright (C) 2006, 2007, 2008, 2009 Apple Inc. All rights reserved.
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
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_COUNTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_COUNTER_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/layout/layout_text.h"
#include "third_party/blink/renderer/core/style/content_data.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"

namespace blink {

// LayoutCounter is used to represent the text of a counter.
// See http://www.w3.org/TR/CSS21/generate.html#counters
//
// Counters are always generated content ("content: counter(a)") thus this
// LayoutObject is always anonymous.
class LayoutCounter : public LayoutText {
 public:
  LayoutCounter(Document&, const CounterContentData&);
  ~LayoutCounter() override;
  void Trace(Visitor*) const override;

  const AtomicString& Identifier() const {
    NOT_DESTROYED();
    return counter_->Identifier();
  }

  void UpdateCounter(Vector<int> counter_values);

  // Returns true if <counter-style> is "disclosure-open" or
  // "disclosure-closed".
  bool IsDirectionalSymbolMarker() const;
  // Returns <string> in counters().
  const AtomicString& Separator() const;

  // Returns LayoutCounter::counter_->ListStyle() if `object` is a
  // LayoutCounter.
  // Returns style.ListStyleType()->GetCounterStyleName() otherwise.
  static const AtomicString& ListStyle(const LayoutObject* object,
                                       const ComputedStyle& style);

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutCounter";
  }

 protected:
  void WillBeDestroyed() override;

 private:
  bool IsCounter() const final {
    NOT_DESTROYED();
    return true;
  }

  const CounterStyle* NullableCounterStyle() const;

  Member<const CounterContentData> counter_;
};

template <>
struct DowncastTraits<LayoutCounter> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsCounter();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_COUNTER_H_
