// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_RENDERER_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_RENDERER_FACTORY_H_

#include "base/memory/scoped_refptr.h"
#include "base/task/sequenced_task_runner.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_audio_renderer.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_video_renderer.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace base {
class SingleThreadTaskRunner;
}  // namespace base

namespace blink {

class WebMediaStream;
class WebLocalFrame;

// MediaStreamRendererFactory is used by WebMediaPlayerMS to create audio
// and video feeds from a MediaStream provided an URL. The factory methods are
// virtual in order for Blink web tests to be able to override them.
class MODULES_EXPORT MediaStreamRendererFactory {
 public:
  MediaStreamRendererFactory();

  MediaStreamRendererFactory(const MediaStreamRendererFactory&) = delete;
  MediaStreamRendererFactory& operator=(const MediaStreamRendererFactory&) =
      delete;

  virtual ~MediaStreamRendererFactory();

  virtual scoped_refptr<MediaStreamVideoRenderer> GetVideoRenderer(
      const WebMediaStream& web_stream,
      const MediaStreamVideoRenderer::RepaintCB& repaint_cb,
      scoped_refptr<base::SequencedTaskRunner> video_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> main_render_task_runner);

  virtual scoped_refptr<MediaStreamAudioRenderer> GetAudioRenderer(
      const WebMediaStream& web_stream,
      WebLocalFrame* web_frame,
      const WebString& device_id,
      base::RepeatingCallback<void()> on_render_error_callback);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_RENDERER_FACTORY_H_
