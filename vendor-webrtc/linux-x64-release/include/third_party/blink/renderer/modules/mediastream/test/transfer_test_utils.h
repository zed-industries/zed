// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_TEST_TRANSFER_TEST_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_TEST_TRANSFER_TEST_UTILS_H_

#include "third_party/blink/renderer/modules/mediastream/media_stream_track.h"
#include "third_party/blink/renderer/modules/mediastream/mock_media_stream_track.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_component.h"

namespace blink {

// Override the implementation of MediaStreamTrack::FromTransferredState.
// Consider using ScopedMockMediaStreamTrackFromTransferredState.
void SetFromTransferredStateImplForTesting(
    MediaStreamTrack::FromTransferredStateImplForTesting impl);

// Sets a mock impl for MediaStreamTrack::FromTransferredState for the lifetime
// of the object. The mock impl will save the argument in |last_argument| and
// return |return_value|.
class ScopedMockMediaStreamTrackFromTransferredState {
 public:
  ScopedMockMediaStreamTrackFromTransferredState();
  ~ScopedMockMediaStreamTrackFromTransferredState();

  MediaStreamTrack::TransferredValues last_argument;
  Persistent<MockMediaStreamTrack> return_value{
      MakeGarbageCollected<MockMediaStreamTrack>()};

 private:
  MediaStreamTrack* Impl(const MediaStreamTrack::TransferredValues& data);
};

// Approximates a tab capture MediaStreamComponent for a video track, but uses a
// MockVideoCapturerSource and empty callbacks. ID is "component_id" and label
// is "test_name".
MediaStreamComponent* MakeTabCaptureVideoComponentForTest(
    LocalFrame* frame,
    base::UnguessableToken session_id);

// Approximates a tab capture MediaStreamComponent for an audio track, but uses
// empty callbacks. ID is "component_id" and label is "test_name".
MediaStreamComponent* MakeTabCaptureAudioComponentForTest(
    base::UnguessableToken session_id);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_TEST_TRANSFER_TEST_UTILS_H_
