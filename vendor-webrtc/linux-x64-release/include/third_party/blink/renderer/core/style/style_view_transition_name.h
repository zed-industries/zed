// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_VIEW_TRANSITION_NAME_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_VIEW_TRANSITION_NAME_H_

#include "third_party/blink/renderer/core/css/style_rule_view_transition.h"
#include "third_party/blink/renderer/core/dom/tree_scope.h"
#include "third_party/blink/renderer/core/style/style_image.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "ui/gfx/geometry/point.h"

namespace blink {

class StyleViewTransitionName
    : public GarbageCollected<StyleViewTransitionName> {
 public:
  enum class Type { kAuto, kCustom, kMatchElement };

  bool IsAuto() const { return type_ == Type::kAuto; }
  bool IsMatchElement() const { return type_ == Type::kMatchElement; }
  bool IsCustom() const { return type_ == Type::kCustom; }

  Type GetType() const { return type_; }

  static StyleViewTransitionName* Auto(const TreeScope* tree_scope) {
    return MakeGarbageCollected<StyleViewTransitionName>(Type::kAuto,
                                                         tree_scope);
  }

  static StyleViewTransitionName* MatchElement(const TreeScope* tree_scope) {
    return MakeGarbageCollected<StyleViewTransitionName>(Type::kMatchElement,
                                                         tree_scope);
  }

  static StyleViewTransitionName* Create(const AtomicString& name,
                                         const TreeScope* tree_scope) {
    CHECK(name);
    CHECK_NE(name, "none");
    CHECK_NE(name, "auto");
    return MakeGarbageCollected<StyleViewTransitionName>(name, tree_scope);
  }

  AtomicString CustomName() const {
    CHECK_EQ(type_, Type::kCustom);
    return custom_name_;
  }

  const TreeScope* GetTreeScope() const { return tree_scope_.Get(); }

  bool operator==(const StyleViewTransitionName& other) const {
    return type_ == other.type_ && custom_name_ == other.custom_name_ &&
           tree_scope_ == other.tree_scope_;
  }

  void Trace(Visitor* visitor) const { visitor->Trace(tree_scope_); }

  StyleViewTransitionName(const AtomicString& custom_name,
                          const TreeScope* tree_scope)
      : type_(Type::kCustom),
        custom_name_(custom_name),
        tree_scope_(tree_scope) {}

  explicit StyleViewTransitionName(Type type, const TreeScope* tree_scope)
      : type_(type), tree_scope_(tree_scope) {}

 private:
  Type type_;
  AtomicString custom_name_;
  WeakMember<const TreeScope> tree_scope_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_VIEW_TRANSITION_NAME_H_
