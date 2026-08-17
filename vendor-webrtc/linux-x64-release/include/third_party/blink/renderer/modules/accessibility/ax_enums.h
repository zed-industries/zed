// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_ENUMS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_ENUMS_H_

#include <stdint.h>

namespace blink {

enum AccessibilityOrientation {
  kAccessibilityOrientationUndefined = 0,
  kAccessibilityOrientationVertical,
  kAccessibilityOrientationHorizontal,
};

// The input restriction on an object.
enum AXRestriction {
  kRestrictionNone = 0,  // An object that is not disabled.
  kRestrictionReadOnly,
  kRestrictionDisabled,
};

enum AccessibilityExpanded {
  kExpandedUndefined = 0,
  kExpandedCollapsed,
  kExpandedExpanded,
};

enum AccessibilityGrabbedState {
  kGrabbedStateUndefined = 0,
  kGrabbedStateFalse,
  kGrabbedStateTrue,
};

enum AccessibilitySelectedState {
  kSelectedStateUndefined = 0,
  kSelectedStateFalse,
  kSelectedStateTrue,
};

enum class AXBoolAttribute {
  kAriaBusy,
};

enum class AXIntAttribute {
  kAriaColumnCount,
  kAriaRowCount,
};

enum class AXUIntAttribute {
  kAriaColumnIndex,
  kAriaColumnSpan,
  kAriaRowIndex,
  kAriaRowSpan,
};

enum class AXStringAttribute {
  kAriaBrailleLabel,
  kAriaBrailleRoleDescription,
  kAriaKeyShortcuts,
  kAriaRoleDescription,
  // TODO(bebeaudr): kAriaVirtualContent is currently a string attribute to
  // facilitate prototyping. Make it an enum when we're done prototyping.
  kAriaVirtualContent,
};

enum class AXObjectAttribute {
  kAriaActiveDescendant,
  kAriaErrorMessage,
};

enum class AXObjectVectorAttribute {
  kAriaControls,
  kAriaDetails,
  kAriaFlowTo,
};

enum AXObjectInclusion {
  kIncludeObject,
  kIgnoreObject,
  kDefaultBehavior,
};

enum AccessibilityOptionalBool {
  kOptionalBoolUndefined = 0,
  kOptionalBoolTrue,
  kOptionalBoolFalse
};

// The potential native host-language-based text (name, description or
// placeholder) sources for an element.  See
// https://w3c.github.io/html-aam/#accessible-name-and-description-computation
enum AXTextSource {
  kAXTextFromNativeSourceUninitialized = -1,
  kAXTextFromNativeHTMLLabel,
  kAXTextFromNativeHTMLLabelFor,
  kAXTextFromNativeHTMLLabelWrapped,
  kAXTextFromNativeHTMLLegend,
  kAXTextFromNativeHTMLRubyAnnotation,
  kAXTextFromNativeHTMLTableCaption,
  kAXTextFromNativeSVGDescElement,
  kAXTextFromNativeTitleElement,  // HTML and SVG
};

enum AXIgnoredReason {
  kAXActiveFullscreenElement,
  kAXActiveModalDialog,
  kAXAriaModalDialog,
  kAXAriaHiddenElement,
  kAXAriaHiddenSubtree,
  kAXEmptyAlt,
  kAXEmptyText,
  kAXInertElement,
  kAXInertSubtree,
  kAXInertStyle,  // Node is made inert by interactivity:inert
  kAXLabelContainer,
  kAXLabelFor,
  kAXNotRendered,
  kAXNotVisible,
  kAXPresentational,
  kAXProbablyPresentational,
  kAXUninteresting
};

// The following represent functions that could be used as callbacks for
// DeferTreeUpdate. Every enum value represents a function that would be
// called after a tree update is complete.
// Please don't reuse these enums in multiple callers to DeferTreeUpdate().
// Instead, add an enum where the suffix describes where it's being called
// from (this helps when debugging an issue apparent in clean layout, by
// helping clarify the code paths).
enum class TreeUpdateReason : uint8_t {
  // These updates are always associated with a DOM Node:
  kActiveDescendantChanged,
  kAriaExpandedChanged,
  kAriaPressedChanged,
  kAriaSelectedChanged,
  kCSSAnchorChanged,
  kDelayEventFromPostNotification,
  kDidShowMenuListPopup,
  kEditableTextContentChanged,
  kFocusableChanged,
  kIdChanged,
  kMaybeDisallowImplicitSelection,
  kNodeIsAttached,
  kNodeGainedFocus,
  kNodeLostFocus,
  kPostNotificationFromHandleLoadComplete,
  kPostNotificationFromHandleLoadStart,
  kPostNotificationFromHandleScrolledToAnchor,
  kReferenceTargetChanged,
  kRemoveValidationMessageObjectFromFocusedUIElement,
  kRemoveValidationMessageObjectFromValidationMessageObject,
  kRestoreParentOrPrune,
  kRoleChangeFromAriaHasPopup,
  kRoleChangeFromImageMapName,
  kRoleChangeFromRoleOrType,
  kRoleMaybeChangedFromEventListener,
  kRoleMaybeChangedFromHref,
  kRoleMaybeChangedOnSelect,
  kSectionOrRegionRoleMaybeChangedFromLabel,
  kSectionOrRegionRoleMaybeChangedFromLabelledBy,
  kSectionOrRegionRoleMaybeChangedFromTitle,
  kTextChangedOnNode,
  kTextChangedOnClosestNodeForLayoutObject,
  kTextMarkerDataAdded,
  kUpdateActiveMenuOption,
  kUpdateAriaOwns,
  kUpdateTableRole,
  kUseMapAttributeChanged,
  kValidationMessageVisibilityChanged,

  // These updates are associated with an AXID:
  kChildrenChanged,
  kMarkAXObjectDirty,
  kMarkAXSubtreeDirty,
  kTextChangedOnLayoutObject,

  // These updates are used for debugging purposes only.
  kChildInserted,
  kMarkDocumentDirty,
  kNewRelationTargetDirty,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_ENUMS_H_
