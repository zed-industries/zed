// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MOCK_MEDIA_STREAM_AUDIO_SINK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MOCK_MEDIA_STREAM_AUDIO_SINK_H_

#include "media/base/audio_bus.h"
#include "media/base/audio_parameters.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/public/platform/modules/mediastream/web_media_stream_audio_sink.h"

namespace blink {

class MockMediaStreamAudioSink : public WebMediaStreamAudioSink {
 public:
  MockMediaStreamAudioSink();
  ~MockMediaStreamAudioSink() override;

  MOCK_METHOD2(OnData, void(const media::AudioBus&, base::TimeTicks));
  MOCK_METHOD1(OnSetFormat, void(const media::AudioParameters&));
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MOCK_MEDIA_STREAM_AUDIO_SINK_H_
