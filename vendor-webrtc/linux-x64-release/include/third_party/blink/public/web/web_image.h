/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_IMAGE_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_IMAGE_H_

#include <vector>

#include "base/time/time.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/skia/include/core/SkBitmap.h"

namespace gfx {
class Size;
}  // namespace gfx

namespace blink {

class WebData;

// APIs to decode an image from a string of data.
class BLINK_EXPORT WebImage {
 public:
  // An image with a duration associated. An animation is a sequence of
  // AnimationFrames played in succession.
  struct AnimationFrame {
    SkBitmap bitmap;
    base::TimeDelta duration;
  };

  // Decodes the given image data. If the image has multiple frames,
  // then the frame whose size is desired_size is returned. Otherwise,
  // the first frame is returned. If the image contains EXIF data that changes
  // the orientation, the returned image will be rotated such that the result is
  // top-left oriented.
  static SkBitmap FromData(const WebData&, const gfx::Size& desired_size);

  // Decodes the given data as an SVG image. If the SVG is well-formed, an
  // image of the first frame is returned. If an error is encountered an empty
  // SkBitmap is returned.
  // The |desired_size| will, if non-empty, determine the size of the returned
  // image - matching the sizing behavior of an <img> with 'width' and 'height'
  // specified to |desired_size| pixels. If empty, the intrinsic size (if any)
  // of the image will be used.
  static SkBitmap DecodeSVG(const WebData&, const gfx::Size& desired_size);

  // Returns a list of all frames in the image. Only the first frame at each
  // pixel size will be returned.
  static std::vector<SkBitmap> FramesFromData(const WebData&);

  // Returns a list of all animation frames in the image.
  static std::vector<AnimationFrame> AnimationFromData(const WebData&);

  WebImage() = delete;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_IMAGE_H_
