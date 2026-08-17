// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_MEDIA_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_MEDIA_TIMING_H_

#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class KURL;

// This class is an abstract interface to the timing information needed to
// calculate LCP metrics for images and videos. It is intended to be implemented
// by concrete classes for each media type.
class MediaTiming : public GarbageCollectedMixin {
 public:
  // Request URL of the media resource content.
  virtual const KURL& Url() const = 0;

  virtual void SetIsSufficientContentLoadedForPaint() = 0;

  // True if enough of the media resource content has been loaded to decode and
  // display a frame. (The entire image for static media, and the first frame
  // for animated media.)
  virtual bool IsSufficientContentLoadedForPaint() const = 0;

  // True if the Timing-Allow-Origin headers have been received and indicate
  // that the timing information for this resource are allowed to be observed
  // by its cross-origin embedder.
  virtual bool TimingAllowPassed() const = 0;

  // Returns the number of bytes of data used to represent the image (or to
  // represent enough of the content that it can be displayed on screen.) This
  // is used to approximate the entropy of the image so that very-low-entropy
  // elements can be excluded from consideration for LCP.
  virtual uint64_t ContentSizeForEntropy() const = 0;

  // True if this is an animated image or video.
  virtual bool IsAnimatedImage() const = 0;

  // True if this is an animated image or video whose first frame has been
  // decoded and rendered.
  // TODO(iclelland): Change this so that it applies to static images as well.
  virtual bool IsPaintedFirstFrame() const = 0;

  virtual void SetFirstVideoFrameTime(base::TimeTicks) { NOTREACHED(); }
  virtual base::TimeTicks GetFirstVideoFrameTime() const = 0;

  // Returns the loading priority used for the image.
  virtual std::optional<WebURLRequest::Priority> RequestPriority() const = 0;

  virtual bool IsDataUrl() const = 0;

  // Returns the type. For images it would be the specific types like jpg etc.
  // For video, it would be video.
  virtual AtomicString MediaType() const = 0;

  virtual bool IsBroken() const = 0;

  virtual base::TimeTicks DiscoveryTime() const = 0;

  virtual base::TimeTicks LoadStart() const = 0;

  virtual base::TimeTicks LoadEnd() const = 0;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_MEDIA_TIMING_H_
