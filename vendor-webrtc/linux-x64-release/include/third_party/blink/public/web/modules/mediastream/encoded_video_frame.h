// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_MEDIASTREAM_ENCODED_VIDEO_FRAME_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_MEDIASTREAM_ENCODED_VIDEO_FRAME_H_

#include <optional>

#include "base/containers/span.h"
#include "media/base/video_codecs.h"
#include "media/base/video_color_space.h"
#include "media/base/video_transformation.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

// Interface for accessing an encoded frame
class EncodedVideoFrame : public WTF::ThreadSafeRefCounted<EncodedVideoFrame> {
 public:
  virtual ~EncodedVideoFrame() = default;

  // Returns a span of the data of the encoded frame
  virtual base::span<const uint8_t> Data() const = 0;

  // Returns the encoding of the encoded frame
  virtual media::VideoCodec Codec() const = 0;

  // Returns true if this frame is a key frame
  virtual bool IsKeyFrame() const = 0;

  // Returns color space stored in the encoded frame.
  virtual std::optional<gfx::ColorSpace> ColorSpace() const = 0;

  // Returns resolution of encoded frame, or 0x0 if not set.
  virtual gfx::Size Resolution() const = 0;

  // Returns the VideoTransformation stored in the encoded frame.
  virtual std::optional<media::VideoTransformation> Transformation() const = 0;
};

using EncodedVideoFrameCB =
    base::RepeatingCallback<void(scoped_refptr<EncodedVideoFrame>,
                                 base::TimeTicks estimated_capture_time)>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_MEDIASTREAM_ENCODED_VIDEO_FRAME_H_
