// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LIST_LAYOUT_OUTSIDE_LIST_MARKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LIST_LAYOUT_OUTSIDE_LIST_MARKER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_block_flow.h"
#include "third_party/blink/renderer/core/layout/list/list_marker.h"

namespace blink {

// A LayoutObject subclass for outside-positioned list markers in LayoutNG.
class CORE_EXPORT LayoutOutsideListMarker final : public LayoutBlockFlow {
 public:
  explicit LayoutOutsideListMarker(Element*);

  void WillCollectInlines() override;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutOutsideListMarker";
  }

  bool NeedsOccupyWholeLine() const;

  const ListMarker& Marker() const {
    NOT_DESTROYED();
    return list_marker_;
  }
  ListMarker& Marker() {
    NOT_DESTROYED();
    return list_marker_;
  }

  bool IsMonolithic() const final;

 private:
  bool IsLayoutOutsideListMarker() const final {
    NOT_DESTROYED();
    return true;
  }
  PositionWithAffinity PositionForPoint(const PhysicalOffset&) const override;

  ListMarker list_marker_;
};

template <>
struct DowncastTraits<LayoutOutsideListMarker> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsLayoutOutsideListMarker();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LIST_LAYOUT_OUTSIDE_LIST_MARKER_H_
