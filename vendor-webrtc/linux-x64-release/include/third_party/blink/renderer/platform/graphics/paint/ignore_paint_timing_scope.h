// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_IGNORE_PAINT_TIMING_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_IGNORE_PAINT_TIMING_SCOPE_H_

#include "base/auto_reset.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Creates a scope to ignore paint timing, e.g. when we are painting contents
// under opacity:0. Currently we store the largest content when the depth is 1
// in order to surface the LCP when the document's opacity changes from 0 to
// nonzero. Care must be taken if changing the conditions under which
// IgnorePaintTimingScope is used in order to ensure correctness.
class PLATFORM_EXPORT IgnorePaintTimingScope {
  STACK_ALLOCATED();

 public:
  IgnorePaintTimingScope()
      : reset_ignore_depth_(&ignore_depth_, ignore_depth_),
        reset_is_document_element_invisible_(&is_document_element_invisible_,
                                             is_document_element_invisible_) {}
  ~IgnorePaintTimingScope() = default;

  static void SetIsDocumentElementInvisible(bool is_invisible) {
    is_document_element_invisible_ = is_invisible;
  }
  static bool IsDocumentElementInvisible() {
    return is_document_element_invisible_;
  }
  static void IncrementIgnoreDepth() { ++ignore_depth_; }
  static int IgnoreDepth() { return ignore_depth_; }

  // Used to bail out of paint timing algorithms when we know we won't track
  // anything. We want to do this when a) document is visible but there is some
  // opacity b) document is invisible but the depth is beyond 1.
  static bool ShouldIgnore() {
    return (!is_document_element_invisible_ && ignore_depth_) ||
           ignore_depth_ > 1;
  }

 private:
  base::AutoReset<int> reset_ignore_depth_;
  base::AutoReset<bool> reset_is_document_element_invisible_;
  static int ignore_depth_;
  static bool is_document_element_invisible_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_IGNORE_PAINT_TIMING_SCOPE_H_
