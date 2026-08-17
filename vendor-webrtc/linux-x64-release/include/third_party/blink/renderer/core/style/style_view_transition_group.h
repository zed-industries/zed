// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_VIEW_TRANSITION_GROUP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_VIEW_TRANSITION_GROUP_H_

#include "base/memory/values_equivalent.h"
#include "third_party/blink/renderer/core/css/style_rule_view_transition.h"
#include "third_party/blink/renderer/core/style/style_image.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "ui/gfx/geometry/point.h"

namespace blink {

class StyleViewTransitionGroup {
  DISALLOW_NEW();

 public:
  enum class GroupType { kNormal, kCustom, kNearest, kContain };

  bool IsNearest() const { return type_ == GroupType::kNearest; }
  bool IsCustom() const { return type_ == GroupType::kCustom; }
  bool IsNormal() const { return type_ == GroupType::kNormal; }
  bool IsContain() const { return type_ == GroupType::kContain; }

  static StyleViewTransitionGroup Nearest() {
    return StyleViewTransitionGroup(GroupType::kNearest);
  }
  static StyleViewTransitionGroup Normal() {
    return StyleViewTransitionGroup(GroupType::kNormal);
  }
  static StyleViewTransitionGroup Contain() {
    return StyleViewTransitionGroup(GroupType::kContain);
  }
  static StyleViewTransitionGroup Create(const AtomicString& name) {
    CHECK(name);
    CHECK_NE(name, "nearest");
    CHECK_NE(name, "normal");
    CHECK_NE(name, "contain");
    return StyleViewTransitionGroup(name);
  }

  AtomicString CustomName() const {
    CHECK_EQ(type_, GroupType::kCustom);
    return custom_name_;
  }

  bool operator==(const StyleViewTransitionGroup& other) const {
    return type_ == other.type_ && custom_name_ == other.custom_name_;
  }

 private:
  explicit StyleViewTransitionGroup(const AtomicString& custom_name)
      : type_(GroupType::kCustom), custom_name_(custom_name) {}

  explicit StyleViewTransitionGroup(GroupType type) : type_(type) {}
  GroupType type_;
  AtomicString custom_name_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_VIEW_TRANSITION_GROUP_H_
