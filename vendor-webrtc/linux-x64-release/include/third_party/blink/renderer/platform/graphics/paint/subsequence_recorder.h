// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_SUBSEQUENCE_RECORDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_SUBSEQUENCE_RECORDER_H_

#include "third_party/blink/renderer/platform/graphics/graphics_context.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_controller.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class GraphicsContext;
class PaintController;

// SubsequenceRecorder records BeginSubsequenceDisplayItem and
// EndSubsequenceDisplayItem sentinels at either end of a continguous sequence
// of DisplayItems, and supports caching via a CachedDisplayItem with the
// CachedSubsequence DisplayItem type.
//
// Also note that UseCachedSubsequenceIfPossible is not sufficient to determine
// whether a CachedSubsequence can be used. In particular, the client is
// responsible for checking that none of the DisplayItemClients that contribute
// to the subsequence have been invalidated.
//
class SubsequenceRecorder final {
  STACK_ALLOCATED();

 public:
  static bool UseCachedSubsequenceIfPossible(GraphicsContext& context,
                                             const DisplayItemClient& client) {
    return context.GetPaintController().UseCachedSubsequenceIfPossible(client);
  }

  SubsequenceRecorder(GraphicsContext& context, const DisplayItemClient& client)
      : paint_controller_(context.GetPaintController()) {
    subsequence_index_ = paint_controller_.BeginSubsequence(client);
    paint_controller_.MarkClientForValidation(client);
  }

  SubsequenceRecorder(const SubsequenceRecorder&) = delete;
  SubsequenceRecorder& operator=(const SubsequenceRecorder&) = delete;

  ~SubsequenceRecorder() {
    paint_controller_.EndSubsequence(subsequence_index_);
  }

 private:
  PaintController& paint_controller_;
  wtf_size_t subsequence_index_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_SUBSEQUENCE_RECORDER_H_
