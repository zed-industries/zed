// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_VIEW_TRANSITION_ROOT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_VIEW_TRANSITION_ROOT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_block_flow.h"

namespace blink {

class Document;
class ViewTransitionStyleTracker;

// Serves as the root of layout for a ViewTransition hierarchy. In
// spec terms, this object represents the conceptual "Snapshot
// Containing Block":
// https://drafts.csswg.org/css-view-transitions-1/#snapshot-containing-block
// This is similar to the "Initial Containing Block" for regular
// layout.
class CORE_EXPORT LayoutViewTransitionRoot : public LayoutBlockFlow {
 public:
  explicit LayoutViewTransitionRoot(Document&);
  ~LayoutViewTransitionRoot() override;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutViewTransitionRoot";
  }

  bool IsViewTransitionRoot() const override {
    NOT_DESTROYED();
    return true;
  }

  bool AnonymousHasStylePropagationOverride() override {
    NOT_DESTROYED();
    return true;
  }

  void UpdateSnapshotStyle(const ViewTransitionStyleTracker& style_tracker);
};

template <>
struct DowncastTraits<LayoutViewTransitionRoot> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsViewTransitionRoot();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_VIEW_TRANSITION_ROOT_H_
