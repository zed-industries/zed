// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_AUDIO_SINK_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_AUDIO_SINK_H_

#include "base/time/time.h"
#include "third_party/blink/public/platform/modules/mediastream/web_media_stream_sink.h"
#include "third_party/blink/public/platform/web_common.h"

namespace media {
class AudioBus;
class AudioParameters;
struct AudioGlitchInfo;
}  // namespace media

namespace blink {

class WebMediaStreamTrack;

class BLINK_PLATFORM_EXPORT WebMediaStreamAudioSink
    : public WebMediaStreamSink {
 public:
  // Adds a MediaStreamAudioSink to the audio track to receive audio data from
  // the track.
  // Called on the main render thread.
  static void AddToAudioTrack(WebMediaStreamAudioSink* sink,
                              const WebMediaStreamTrack& track);

  // Removes a MediaStreamAudioSink from the audio track to stop receiving
  // audio data from the track.
  // Called on the main render thread.
  static void RemoveFromAudioTrack(WebMediaStreamAudioSink* sink,
                                   const WebMediaStreamTrack& track);

  // Returns the format of the audio track.
  // Called on the main render thread.
  static media::AudioParameters GetFormatFromAudioTrack(
      const WebMediaStreamTrack& track);

  // Callback called to deliver audio data. The data in |audio_bus| respects the
  // AudioParameters passed in the last call to OnSetFormat().  Called on
  // real-time audio thread.
  //
  // |estimated_capture_time| is the local time at which the first sample frame
  // in |audio_bus| either: 1) was generated, if it was done so locally; or 2)
  // should be targeted for play-out, if it was generated from a remote
  // source. Either way, an implementation should not play-out the audio before
  // this point-in-time. This value is NOT a high-resolution timestamp, and so
  // it should not be used as a presentation time; but, instead, it should be
  // used for buffering playback and for A/V synchronization purposes.
  virtual void OnData(const media::AudioBus& audio_bus,
                      base::TimeTicks estimated_capture_time) = 0;

  // MediaStreamAudioDeliverer<Consumer> expects the Consumers to have an
  // OnData method with this signature. Since no WebMediaStreamAudioSinks use
  // the glitch info, we discard it and call the virtual OnData method which
  // does not take glitch info as an argument.
  void OnData(const media::AudioBus& audio_bus,
              base::TimeTicks estimated_capture_time,
              const media::AudioGlitchInfo& glitch_info) {
    OnData(audio_bus, estimated_capture_time);
  }

  // Callback called when the format of the audio stream has changed.  This is
  // always called at least once before OnData(), and on the same thread.
  virtual void OnSetFormat(const media::AudioParameters& params) = 0;

  // Returns the number of channels preferred by the sink or -1 if
  // unknown.
  virtual int NumPreferredChannels() { return -1; }

 protected:
  ~WebMediaStreamAudioSink() override {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_AUDIO_SINK_H_
