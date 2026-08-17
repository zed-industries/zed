// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_STYLE_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_STYLE_BUILDER_H_

#include "base/containers/flat_map.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/view_transition/view_transition_style_tracker.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

class ViewTransitionStyleBuilder {
 public:
  using ContainerProperties = ViewTransitionStyleTracker::ContainerProperties;
  using CapturedCssProperties = base::flat_map<CSSPropertyID, String>;

  ViewTransitionStyleBuilder() = default;

  void AddUAStyle(const String& style);

  enum class AnimationType { kOldOnly, kNewOnly, kBoth };

  // Both `source_properties` and `animated_css_properties` come from keyframe
  // set up step in
  // https://drafts.csswg.org/css-view-transitions-1/#setup-transition-pseudo-elements-algorithm
  void AddAnimations(AnimationType type,
                     const String& tag,
                     const ContainerProperties& source_properties,
                     const CapturedCssProperties& animated_css_properties,
                     const gfx::Transform& parent_inverse_transform);

  void AddContainerStyles(const String& tag,
                          const ContainerProperties& properties,
                          const CapturedCssProperties& captured_css_properites,
                          const gfx::Transform& parent_inverse_transform);

  String Build();

 private:
  // Adds the needed keyframes and returns the animation name to use.
  String AddKeyframes(const String& tag,
                      const ContainerProperties& source_properties,
                      const CapturedCssProperties& captured_css_properties,
                      const gfx::Transform& parent_inverse_transform);
  void AddRules(const String& selector, const String& tag, const String& rules);
  void AddSelector(const String& name, const String& tag);

  StringBuilder builder_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_STYLE_BUILDER_H_
