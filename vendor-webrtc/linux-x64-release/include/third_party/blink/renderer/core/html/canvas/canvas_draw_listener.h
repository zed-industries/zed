// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_DRAW_LISTENER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_DRAW_LISTENER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/static_bitmap_image.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace media {
class VideoFrame;
}  // namespace media

namespace blink {

class CORE_EXPORT CanvasDrawListener : public GarbageCollectedMixin {
 public:
  virtual ~CanvasDrawListener();
  // GetNewFrameCallback returns a once-callback to be made with a VideoFrame
  // when the frame is ready. This function needs to be called for every new
  // frame. The returned callback may be made immediately, with a delay (e.g,
  // if reading data back from the GPU), or never (e.g, because of a GPU context
  // lost).
  using NewFrameCallback =
      base::OnceCallback<void(scoped_refptr<media::VideoFrame>)>;
  virtual NewFrameCallback GetNewFrameCallback() = 0;

  // Returns true if the alpha channel of the frame provided to SendNewFrame's
  // callback will not be used. This allows the caller to discard the channel
  // ahead of time.
  virtual bool CanDiscardAlpha() const = 0;

  virtual bool NeedsNewFrame() const = 0;
  virtual void RequestFrame() = 0;

 protected:
  CanvasDrawListener();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_DRAW_LISTENER_H_
