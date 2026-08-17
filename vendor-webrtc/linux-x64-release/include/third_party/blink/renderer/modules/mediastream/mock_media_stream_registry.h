// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MOCK_MEDIA_STREAM_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MOCK_MEDIA_STREAM_REGISTRY_H_

#include <optional>

#include "third_party/blink/renderer/modules/mediastream/mock_media_stream_video_source.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_descriptor.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class VideoTrackAdapterSettings;

// This class encapsulates creation of a Blink MediaStream having inside the
// necessary Blink and Chromium, track and source. The Chrome Video source is
// a mock.
class MockMediaStreamRegistry final {
 public:
  MockMediaStreamRegistry();

  void Init();

  // Returns the native mock vidoe source for optional use in tests.
  MockMediaStreamVideoSource* AddVideoTrack(
      const String& track_id,
      const VideoTrackAdapterSettings& adapter_settings,
      const std::optional<bool>& noise_reduction,
      bool is_screen_cast,
      double min_frame_rate);
  MockMediaStreamVideoSource* AddVideoTrack(const String& track_id);
  void AddAudioTrack(const String& track_id);

  MediaStreamDescriptor* test_stream() const { return descriptor_.Get(); }

  void reset() { descriptor_ = nullptr; }

 private:
  Persistent<MediaStreamDescriptor> descriptor_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MOCK_MEDIA_STREAM_REGISTRY_H_
