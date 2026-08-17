// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_WEBRTC_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_WEBRTC_SOURCE_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace base {
class TimeDelta;
}

namespace media {
class AudioBus;
struct AudioGlitchInfo;
}

namespace blink {

class WebRtcAudioRenderer;

// TODO(xians): Move the following two interfaces to webrtc so that
// libjingle can own references to the renderer and capturer.
class PLATFORM_EXPORT WebRtcAudioRendererSource {
 public:
  // Callback to get the rendered data.
  // |audio_bus| must have buffer size |sample_rate/100| and 1-2 channels.
  virtual void RenderData(media::AudioBus* audio_bus,
                          int sample_rate,
                          base::TimeDelta audio_delay,
                          base::TimeDelta* current_time,
                          const media::AudioGlitchInfo& glitch_info) = 0;

  // Callback to notify the client that the renderer is going away.
  virtual void RemoveAudioRenderer(WebRtcAudioRenderer* renderer) = 0;

  // Callback to notify the client that the audio renderer thread stopped.
  // This function must be called only when that thread is actually stopped.
  // Otherwise a race may occur.
  virtual void AudioRendererThreadStopped() = 0;

  // Callback to notify the client of the output device the renderer is using.
  virtual void SetOutputDeviceForAec(const String& output_device_id) = 0;

 protected:
  virtual ~WebRtcAudioRendererSource() {}
};

// TODO(xians): Merge this interface with WebRtcAudioRendererSource.
// The reason why we could not do it today is that WebRtcAudioRendererSource
// gets the data by pulling, while the data is pushed into
// WebRtcPlayoutDataSource::Sink.
class PLATFORM_EXPORT WebRtcPlayoutDataSource {
 public:
  class Sink {
   public:
    // Callback to get the playout data.
    // Called on the audio render thread.
    // |audio_bus| must have buffer size |sample_rate/100| and 1-2 channels.
    virtual void OnPlayoutData(media::AudioBus* audio_bus,
                               int sample_rate,
                               base::TimeDelta audio_delay) = 0;

    // Callback to notify the sink that the source has changed.
    // Called on the main render thread.
    virtual void OnPlayoutDataSourceChanged() = 0;

    // Called to notify that the audio render thread has changed, and
    // OnPlayoutData() will from now on be called on the new thread.
    // Called on the new audio render thread.
    virtual void OnRenderThreadChanged() = 0;

   protected:
    virtual ~Sink() {}
  };

  // Adds/Removes the sink of WebRtcAudioRendererSource to the ADM.
  // These methods are used by the MediaStreamAudioProcesssor to get the
  // rendered data for AEC.
  virtual void AddPlayoutSink(Sink* sink) = 0;
  virtual void RemovePlayoutSink(Sink* sink) = 0;

 protected:
  virtual ~WebRtcPlayoutDataSource() {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_WEBRTC_SOURCE_H_
