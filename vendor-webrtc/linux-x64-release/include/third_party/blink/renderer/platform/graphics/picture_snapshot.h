/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PICTURE_SNAPSHOT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PICTURE_SNAPSHOT_H_

#include <memory>

#include "third_party/blink/renderer/platform/graphics/graphics_context.h"
#include "third_party/blink/renderer/platform/json/json_values.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

#include "third_party/skia/include/core/SkPicture.h"
#include "third_party/skia/include/core/SkPictureRecorder.h"
#include "third_party/skia/include/core/SkRefCnt.h"

namespace gfx {
class RectF;
}

namespace blink {

class PLATFORM_EXPORT PictureSnapshot : public RefCounted<PictureSnapshot> {
  USING_FAST_MALLOC(PictureSnapshot);

 public:
  struct TilePictureStream : RefCounted<TilePictureStream> {
    gfx::PointF layer_offset;
    sk_sp<SkPicture> picture;
  };

  static scoped_refptr<PictureSnapshot> Load(
      const Vector<scoped_refptr<TilePictureStream>>&);

  PictureSnapshot(sk_sp<const SkPicture>);
  PictureSnapshot(const PictureSnapshot&) = delete;
  PictureSnapshot& operator=(const PictureSnapshot&) = delete;

  Vector<uint8_t> Replay(unsigned from_step = 0,
                         unsigned to_step = 0,
                         double scale = 1.0) const;
  Vector<Vector<base::TimeDelta>> Profile(unsigned min_iterations,
                                          base::TimeDelta min_duration,
                                          const gfx::RectF* clip_rect) const;
  std::unique_ptr<JSONArray> SnapshotCommandLog() const;
  bool IsEmpty() const;

 private:
  std::unique_ptr<SkBitmap> CreateBitmap() const;

  sk_sp<const SkPicture> picture_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PICTURE_SNAPSHOT_H_
