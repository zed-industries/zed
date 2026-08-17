// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CUSTOM_LAYOUT_EDGES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CUSTOM_LAYOUT_EDGES_H_

#include "third_party/blink/renderer/core/layout/geometry/box_strut.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"

namespace blink {

// Represents the border, scrollbar, and padding edges given to the custom
// layout.
class CustomLayoutEdges : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CustomLayoutEdges(const BoxStrut& border_scrollbar_padding)
      : inline_start_(border_scrollbar_padding.inline_start),
        inline_end_(border_scrollbar_padding.inline_end),
        block_start_(border_scrollbar_padding.block_start),
        block_end_(border_scrollbar_padding.block_end) {}
  CustomLayoutEdges(const CustomLayoutEdges&) = delete;
  CustomLayoutEdges& operator=(const CustomLayoutEdges&) = delete;

  // layout_edges.idl
  double inlineStart() const { return inline_start_; }
  double inlineEnd() const { return inline_end_; }
  double blockStart() const { return block_start_; }
  double blockEnd() const { return block_end_; }
  double inlineSum() const { return inline_start_ + inline_end_; }
  double blockSum() const { return block_start_ + block_end_; }

 private:
  double inline_start_;
  double inline_end_;
  double block_start_;
  double block_end_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CUSTOM_LAYOUT_EDGES_H_
