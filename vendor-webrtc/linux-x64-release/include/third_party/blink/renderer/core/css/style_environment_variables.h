// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_ENVIRONMENT_VARIABLES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_ENVIRONMENT_VARIABLES_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_variable_data.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class FeatureContext;

// UADefinedVariable contains all user agent defined environment variables with
// a single dimension.
// When adding a new variable the string equivalent needs to be added to
// |GetVariableName|.
enum class UADefinedVariable {
  // The safe area insets are four environment variables that define a
  // rectangle by its top, right, bottom, and left insets from the edge of
  // the viewport.
  kSafeAreaInsetTop,
  kSafeAreaInsetLeft,
  kSafeAreaInsetBottom,
  kSafeAreaInsetRight,

  // When safe-area-inset-* values can change dynamically during the browsing
  // session, their maximum values are expressed as safe-area-max-inset-*.
  // Currently only the bottom inset is dynamic, for Android edge-to-edge UI.
  kSafeAreaMaxInsetTop,
  kSafeAreaMaxInsetLeft,
  kSafeAreaMaxInsetBottom,
  kSafeAreaMaxInsetRight,

  // The keyboard area insets are six environment variables that define a
  // virtual keyboard rectangle by its top, right, bottom, left, width and
  // height insets
  // from the edge of the viewport.
  // Explainers:
  // https://github.com/MicrosoftEdge/MSEdgeExplainers/blob/main/VirtualKeyboardAPI/explainer.md
  kKeyboardInsetTop,
  kKeyboardInsetLeft,
  kKeyboardInsetBottom,
  kKeyboardInsetRight,
  kKeyboardInsetWidth,
  kKeyboardInsetHeight,

  // The title bar area variables are four environment variables that define a
  // rectangle by its x and y position as well as its width and height. They are
  // intended for desktop PWAs that use the window controls overlay.
  // Explainer:
  // https://github.com/WICG/window-controls-overlay/blob/main/explainer.md
  kTitlebarAreaX,
  kTitlebarAreaY,
  kTitlebarAreaWidth,
  kTitlebarAreaHeight,

  // The context menu insets are four environment variables that define a
  // rectangle by its top, right, bottom, and left insets from the edge of
  // the viewport. These are used for the `interesttarget` attribute on mobile
  // devices that display context menus, to indicate the still-unoccluded area
  // of the screen while a context menu is visible.
  // Explainer:
  // https://open-ui.org/components/interest-invokers.explainer/#touchscreen
  kContextMenuInsetTop,
  kContextMenuInsetLeft,
  kContextMenuInsetBottom,
  kContextMenuInsetRight,

  // The text scale as chosen by the user in the OS accessibility settings.
  kPreferredTextScale
};

enum class UADefinedTwoDimensionalVariable {
  // The viewport segment variables describe logically distinct regions of the
  // viewport, and are indexed in two dimensions (x and y).
  kViewportSegmentTop,
  kViewportSegmentRight,
  kViewportSegmentBottom,
  kViewportSegmentLeft,
  kViewportSegmentWidth,
  kViewportSegmentHeight,
};

// StyleEnvironmentVariables stores user agent and user defined CSS environment
// variables. It has a static root instance that stores global values and
// each document has a child that stores document level values.
// Setting and removing values can only be done for the set of variables in
// UADefinedVariable. Note that UADefinedVariables are not always set/defined,
// as they depend on the environment.
class CORE_EXPORT StyleEnvironmentVariables
    : public GarbageCollected<StyleEnvironmentVariables> {
 public:
  static StyleEnvironmentVariables& GetRootInstance();

  // Gets the name of a |UADefinedVariable| as a string.
  // |feature_context| is required for a RuntimeEnabledFeatures check for a
  // variable in origin trial, otherwise nullptr can be passed.
  static const AtomicString GetVariableName(
      UADefinedVariable variable,
      const FeatureContext* feature_context);
  static const AtomicString GetVariableName(
      UADefinedTwoDimensionalVariable variable,
      const FeatureContext* feature_context);

  // Create a new root instance.
  StyleEnvironmentVariables();

  // Create a new instance bound to |parent|.
  StyleEnvironmentVariables(StyleEnvironmentVariables& parent)
      : parent_(parent) {
    parent.children_.push_back(this);
  }

  virtual ~StyleEnvironmentVariables() = default;

  virtual void Trace(Visitor* visitor) const {
    visitor->Trace(children_);
    visitor->Trace(data_);
    visitor->Trace(two_dimension_data_);
    visitor->Trace(parent_);
  }

  // Tokenize |value| and set it. This will invalidate any dependents.
  void SetVariable(UADefinedVariable variable, const String& value);

  // Tokenize |value| and set it. This will invalidate any dependents.
  void SetVariable(UADefinedTwoDimensionalVariable variable,
                   unsigned first_dimension,
                   unsigned second_dimenison,
                   const String& value,
                   const FeatureContext* feature_context);

  // Remove the variable |name| and invalidate any dependents.
  void RemoveVariable(UADefinedVariable variable);
  // Remove all the indexed variables referenced by the enum, and invalidate any
  // dependents.
  void RemoveVariable(UADefinedTwoDimensionalVariable variable,
                      const FeatureContext* feature_context);

  // Resolve the variable |name| by traversing the tree of
  // |StyleEnvironmentVariables|.
  virtual CSSVariableData* ResolveVariable(const AtomicString& name,
                                           WTF::Vector<unsigned> indices);

  // Detach |this| from |parent|.
  void DetachFromParent();

  // Stringify |value| and append 'px'. Helper for setting variables that are
  // CSS lengths.
  static String FormatFloatPx(float value);
  static String FormatPx(int value);

  virtual const FeatureContext* GetFeatureContext() const;

 protected:
  friend class StyleEnvironmentVariablesTest;
  friend class StyleCascadeTest;

  // Tokenize |value| and set it, invalidating dependents along the way.
  void SetVariable(const AtomicString& name, const String& value);
  void SetVariable(const AtomicString& name,
                   unsigned first_dimension,
                   unsigned second_dimension,
                   const String& value);

  void RemoveVariable(const AtomicString& name);

  void ClearForTesting();

  // Called by the parent to tell the child that variable |name| has changed.
  void ParentInvalidatedVariable(const AtomicString& name);

  // Called when variable |name| is changed. This will notify any children that
  // this variable has changed.
  virtual void InvalidateVariable(const AtomicString& name);

 private:
  using TwoDimensionVariableValues =
      GCedHeapVector<HeapVector<Member<CSSVariableData>>>;

  HeapVector<Member<StyleEnvironmentVariables>> children_;
  HeapHashMap<AtomicString, Member<CSSVariableData>> data_;
  HeapHashMap<AtomicString, Member<TwoDimensionVariableValues>>
      two_dimension_data_;
  Member<StyleEnvironmentVariables> parent_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_ENVIRONMENT_VARIABLES_H_
