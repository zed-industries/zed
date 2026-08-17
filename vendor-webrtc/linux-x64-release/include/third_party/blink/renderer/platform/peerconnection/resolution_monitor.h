// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RESOLUTION_MONITOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RESOLUTION_MONITOR_H_

#include <memory>
#include <optional>

#include "base/sequence_checker.h"
#include "media/base/video_codecs.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "ui/gfx/geometry/size.h"

namespace media {
class DecoderBuffer;
}  // namespace media

namespace blink {

// Resolution monitor acquires a resolution of DecoderBuffer by parsing it. We
// can know the stream resolution before the first keyframe is decoded. This
// avoids requesting a sender to produce a keyframe again when a software
// decoder fallback happens due to stream resolution change.
class PLATFORM_EXPORT ResolutionMonitor {
 public:
  virtual ~ResolutionMonitor();

  static std::unique_ptr<ResolutionMonitor> Create(media::VideoCodec codec);

  virtual std::optional<gfx::Size> GetResolution(
      const media::DecoderBuffer& buffer) = 0;
  virtual media::VideoCodec codec() const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RESOLUTION_MONITOR_H_
