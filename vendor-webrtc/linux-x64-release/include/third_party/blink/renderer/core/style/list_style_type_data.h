// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_LIST_STYLE_TYPE_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_LIST_STYLE_TYPE_DATA_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class CounterStyle;
class Document;
class TreeScope;

class ListStyleTypeData final : public GarbageCollected<ListStyleTypeData> {
 public:
  ~ListStyleTypeData() = default;
  CORE_EXPORT void Trace(Visitor*) const;

  enum class Type { kCounterStyle, kString };

  ListStyleTypeData(Type type,
                    AtomicString name_or_string_value,
                    const TreeScope* tree_scope)
      : type_(type),
        name_or_string_value_(std::move(name_or_string_value)),
        tree_scope_(tree_scope) {}

  static ListStyleTypeData* CreateString(const AtomicString&);
  static ListStyleTypeData* CreateCounterStyle(const AtomicString&,
                                               const TreeScope*);

  bool operator==(const ListStyleTypeData& other) const {
    return type_ == other.type_ &&
           name_or_string_value_ == other.name_or_string_value_ &&
           tree_scope_ == other.tree_scope_;
  }
  bool operator!=(const ListStyleTypeData& other) const {
    return !operator==(other);
  }

  bool IsCounterStyle() const { return type_ == Type::kCounterStyle; }
  bool IsString() const { return type_ == Type::kString; }

  const AtomicString& GetCounterStyleName() const {
    DCHECK_EQ(Type::kCounterStyle, type_);
    return name_or_string_value_;
  }

  const AtomicString& GetStringValue() const {
    DCHECK_EQ(Type::kString, type_);
    return name_or_string_value_;
  }

  const TreeScope* GetTreeScope() const { return tree_scope_.Get(); }

  // TODO(crbug.com/687225): Try not to pass a Document, which is cumbersome.
  bool IsCounterStyleReferenceValid(Document&) const;
  const CounterStyle& GetCounterStyle(Document&) const;

 private:
  Type type_;
  AtomicString name_or_string_value_;

  // The tree scope for looking up the custom counter style name.
  // Must be weak reference to break the following ref cycle of both GC-ed and
  // ref-counted objects:
  // Document --> ComputedStyle --> ListStyleTypeData --> TreeScope(Document)
  WeakMember<const TreeScope> tree_scope_;

  // The CounterStyle that we are using. The reference is updated on demand.
  // Note: this is NOT part of the computed value of 'list-style-type'.
  mutable Member<const CounterStyle> counter_style_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_LIST_STYLE_TYPE_DATA_H_
