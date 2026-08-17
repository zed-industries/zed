/*
 * Copyright (C) 2019 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_MEMORY_MANAGED_PAINT_RECORDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_MEMORY_MANAGED_PAINT_RECORDER_H_

#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "third_party/blink/renderer/platform/graphics/memory_managed_paint_canvas.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class PLATFORM_EXPORT MemoryManagedPaintRecorder {
 public:
  class Client {
   public:
    virtual void InitializeForRecording(cc::PaintCanvas* canvas) const = 0;
    virtual void RecordingCleared() = 0;
  };

  // If specified, `client` is notified for events from this object. `client`
  // must outlive this `MemoryManagedPaintRecorder`.
  explicit MemoryManagedPaintRecorder(gfx::Size size, Client* client);
  ~MemoryManagedPaintRecorder();

  void SetClient(Client* client);

  // See comments around `RecordPaintCanvas::maybe_draw_lines_as_paths_` for
  // details.
  void DisableLineDrawingAsPaths();

  // Releases the main recording, leaving the side recording untouched.
  // This effectively flushes the part of the recording that can be flushed,
  // leaving unclosed layer content unflushed, available to be closed and
  // flushed later.
  cc::PaintRecord ReleaseMainRecording();
  cc::PaintRecord CopyMainRecording();

  // Drop all the draw ops recorded in the current layer level. This is used
  // when we are about to draw over the whole canvas or layer, masking away all
  // previously recorded paint ops at that level.
  // Right now however, the `MemoryManagedPaintRecorder` doesn't keep track of
  // each individual layers and instead group them all into a single side
  // recording. We therefore can only drop draw ops when no layers are opened.
  // This could be improved upon by keeping a canvas stack here, if it's ever
  // deemed useful.
  void RestartCurrentLayer();

  // Restarts the whole recording, discarding the side recording and
  // re-initializing the main recording.
  void RestartRecording();

  bool HasRecordedDrawOps() const {
    return main_canvas_.HasRecordedDrawOps() ||
           (side_canvas_ && side_canvas_->HasRecordedDrawOps());
  }
  bool HasReleasableDrawOps() const {
    return main_canvas_.HasRecordedDrawOps();
  }
  bool HasSideRecording() const { return side_canvas_ != nullptr; }
  size_t TotalOpCount() const {
    return main_canvas_.TotalOpCount() +
           (side_canvas_ ? side_canvas_->TotalOpCount() : 0);
  }
  size_t TotalOpBytesUsed() const {
    return main_canvas_.OpBytesUsed() +
           (side_canvas_ ? side_canvas_->OpBytesUsed() : 0);
  }
  size_t ReleasableOpBytesUsed() const { return main_canvas_.OpBytesUsed(); }
  size_t ReleasableImageBytesUsed() const {
    return main_canvas_.ImageBytesUsed();
  }

  // Returns the `PaintCanvas` that currently accumulates draw commands. This is
  // normally the main canvas, but between calls to `BeginSideRecording() and
  // `EndSideRecording()` (when layers are opened), this function returns the
  // side recording.
  const MemoryManagedPaintCanvas& getRecordingCanvas() const {
    return *current_canvas_;
  }
  MemoryManagedPaintCanvas& getRecordingCanvas() { return *current_canvas_; }
  const MemoryManagedPaintCanvas& GetMainCanvas() const { return main_canvas_; }
  const MemoryManagedPaintCanvas* GetSideCanvas() const {
    return side_canvas_.get();
  }

  // Begin a side recording that won't be flushed until `EndSideRecording()` is
  // called. This is used to accumulate draw calls in layers until the layers
  // are closed, after which they can be presented. After calling
  // `BeginSideRecording()`, `getRecordingCanvas()` returns a side `PaintCanvas`
  // that isn't affected by calls to `ReleaseMainRecording()`. Calling
  // `EndSideRecording()` draws the side recording onto the main one.
  void BeginSideRecording();

  // Draw the side recording onto the main recording. Calling
  // `EndSideRecording()` discards the main recording, after which calls to
  // `getRecordingCanvas()` returns the main `PaintCanvas`.
  void EndSideRecording();

 private:
  // Pointer to the client interested in events from this
  // `MemoryManagedPaintRecorder`. If `nullptr`, notifications are disabled.
  raw_ptr<Client> client_ = nullptr;

  const gfx::Size size_;

  // Top-level recording to which we draw when no layers are opened.
  MemoryManagedPaintCanvas main_canvas_;

  // Side recording to which we write when any layers are opened. We don't
  // currently keep track of each individual layers. The goals is to be able to
  // flush any paint ops that is not in a layer if we ever need to present a
  // frame and keep the layer content alive until the layers are later closed
  // and presented.
  std::unique_ptr<MemoryManagedPaintCanvas> side_canvas_;

  // Points to the current canvas we are recording into, either `main_canvas_`
  // or `side_canvas_`.
  // RAW_PTR_EXCLUSION: Performance reasons (based on analysis of MotionMark).
  RAW_PTR_EXCLUSION MemoryManagedPaintCanvas* current_canvas_ = &main_canvas_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_MEMORY_MANAGED_PAINT_RECORDER_H_
