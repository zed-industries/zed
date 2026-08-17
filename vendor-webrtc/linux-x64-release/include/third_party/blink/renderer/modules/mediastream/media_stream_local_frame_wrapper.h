// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_LOCAL_FRAME_WRAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_LOCAL_FRAME_WRAPPER_H_

#include "third_party/blink/public/web/web_local_frame.h"
#include "third_party/blink/renderer/core/frame/local_frame.h"

namespace blink {

// This class wraps WebLocalFrame instances, so they can be work as
// weak pointers. Internally, it uses the backing LocalFrame for such.
//
// TODO(crbug.com/704136): Remove this class when its users get fully
// Onion souped, and WeakPersistent<LocalFrame> can be used directly.
class MediaStreamInternalFrameWrapper {
 public:
  MediaStreamInternalFrameWrapper(WebLocalFrame* web_frame)
      : frame_(web_frame ? static_cast<LocalFrame*>(
                               WebLocalFrame::ToCoreFrame(*web_frame))
                         : nullptr) {}

  MediaStreamInternalFrameWrapper(const MediaStreamInternalFrameWrapper&) =
      delete;
  MediaStreamInternalFrameWrapper& operator=(
      const MediaStreamInternalFrameWrapper&) = delete;

  LocalFrame* frame() { return frame_.Get(); }
  WebLocalFrame* web_frame() {
    if (!frame_)
      return nullptr;

    return static_cast<WebLocalFrame*>(WebFrame::FromCoreFrame(frame()));
  }

 private:
  WeakPersistent<LocalFrame> frame_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_LOCAL_FRAME_WRAPPER_H_
