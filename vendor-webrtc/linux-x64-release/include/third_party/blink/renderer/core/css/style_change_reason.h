// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_CHANGE_REASON_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_CHANGE_REASON_H_

#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class QualifiedName;

namespace style_change_reason {
extern const char kAccessibility[];
extern const char kActiveStylesheetsUpdate[];
extern const char kAffectedByHas[];
extern const char kAnimation[];
extern const char kAttribute[];
extern const char kConditionalBackdrop[];
extern const char kControl[];
extern const char kControlValue[];
extern const char kDeclarativeContent[];
extern const char kDesignMode[];
extern const char kDialog[];
extern const char kDisplayLock[];
extern const char kEditContext[];
extern const char kEnvironmentVariableChanged[];
extern const char kViewTransition[];
extern const char kFrame[];
extern const char kFlatTreeChange[];
extern const char kFonts[];
extern const char kFullscreen[];
extern const char kFunctionRuleChange[];
extern const char kInheritedStyleChangeFromParentFrame[];
extern const char kInlineCSSStyleMutated[];
extern const char kInspector[];
extern const char kKeyframesRuleChange[];
extern const char kLanguage[];
extern const char kLinkColorChange[];
extern const char kMediaQuery[];
extern const char kNodeInserted[];
extern const char kPictureSourceChanged[];
extern const char kPlatformColorChange[];
extern const char kPlaceElement[];
extern const char kPluginChanged[];
extern const char kPopoverVisibilityChange[];
extern const char kPositionTryChange[];
extern const char kPrinting[];
extern const char kPropertyRegistration[];
extern const char kPseudoClass[];
extern const char kRelatedStyleRule[];
extern const char kScrollTimeline[];
extern const char kSVGContainerSizeChange[];
extern const char kSettings[];
extern const char kShadow[];
extern const char kStyleAttributeChange[];
extern const char kStyleRuleChange[];
extern const char kTopLayer[];
extern const char kUseFallback[];
extern const char kViewportDefiningElement[];
extern const char kViewportUnits[];
extern const char kVisuallyOrdered[];
extern const char kWritingModeChange[];
extern const char kZoom[];
}  // namespace style_change_reason
typedef const char StyleChangeReasonString[];

namespace style_change_extra_data {
extern const AtomicString& g_active;
extern const AtomicString& g_active_view_transition;
extern const AtomicString& g_active_view_transition_type;
extern const AtomicString& g_disabled;
extern const AtomicString& g_drag;
extern const AtomicString& g_focus;
extern const AtomicString& g_focus_visible;
extern const AtomicString& g_focus_within;
extern const AtomicString& g_hover;
extern const AtomicString& g_past;
extern const AtomicString& g_unresolved;

void Init();
}  // namespace style_change_extra_data

// |StyleChangeReasonForTracing| is used to trace the reason a
// |Node::setNeedsStyleRecalc| call was made to show it in DevTools or in
// about:tracing.
// |StyleChangeReasonForTracing| is strictly only for the tracing purpose as
// described above. Blink logic must not depend on this value.
class StyleChangeReasonForTracing {
  DISALLOW_NEW();

 public:
  static StyleChangeReasonForTracing Create(
      StyleChangeReasonString reason_string) {
    return StyleChangeReasonForTracing(reason_string, g_null_atom);
  }

  static StyleChangeReasonForTracing CreateWithExtraData(
      StyleChangeReasonString reason_string,
      const AtomicString& extra_data) {
    return StyleChangeReasonForTracing(reason_string, extra_data);
  }

  static StyleChangeReasonForTracing FromAttribute(
      const QualifiedName& attribute_name) {
    return StyleChangeReasonForTracing(style_change_reason::kAttribute,
                                       attribute_name.LocalName());
  }

  String ReasonString() const { return String(reason_); }
  const AtomicString& GetExtraData() const { return extra_data_; }

 private:
  StyleChangeReasonForTracing(StyleChangeReasonString reason_string,
                              const AtomicString& extra_data)
      : reason_(reason_string), extra_data_(extra_data) {}

  // disable comparisons
  void operator==(const StyleChangeReasonForTracing&) const {}
  void operator!=(const StyleChangeReasonForTracing&) const {}

  const char* reason_;
  AtomicString extra_data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_CHANGE_REASON_H_
