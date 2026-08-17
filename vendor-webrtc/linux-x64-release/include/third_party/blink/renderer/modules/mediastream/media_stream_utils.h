// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_UTILS_H_

#include "base/feature_list.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/mojom/mediastream/media_stream.mojom-blink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

class ExecutionContext;
class LocalFrame;
class MediaStreamSource;
class MediaStreamTrack;

MODULES_EXPORT BASE_DECLARE_FEATURE(kGetDisplayMediaScreenScaleFactor);

class MODULES_EXPORT MediaStreamUtils {
  STATIC_ONLY(MediaStreamUtils);

 public:
  static MediaStreamTrack* CreateLocalAudioTrack(ExecutionContext*,
                                                 MediaStreamSource*);

  static gfx::Size GetScreenSize(LocalFrame* frame);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_UTILS_H_
