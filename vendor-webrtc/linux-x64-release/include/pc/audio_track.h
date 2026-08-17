/*
 *  Copyright 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_AUDIO_TRACK_H_
#define PC_AUDIO_TRACK_H_

#include <string>

#include "absl/strings/string_view.h"
#include "api/media_stream_interface.h"
#include "api/media_stream_track.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "rtc_base/system/no_unique_address.h"

namespace webrtc {

// TODO(tommi): Instead of inheriting from `MediaStreamTrack<>`, implement the
// properties directly in this class. `MediaStreamTrack` doesn't guard against
// conflicting access, so we'd need to override those methods anyway in this
// class in order to make sure things are correctly checked.
class AudioTrack : public MediaStreamTrack<AudioTrackInterface>,
                   public ObserverInterface {
 protected:
  // Protected ctor to force use of factory method.
  AudioTrack(absl::string_view label,
             const scoped_refptr<AudioSourceInterface>& source);

  AudioTrack() = delete;
  AudioTrack(const AudioTrack&) = delete;
  AudioTrack& operator=(const AudioTrack&) = delete;

  ~AudioTrack() override;

 public:
  static scoped_refptr<AudioTrack> Create(
      absl::string_view id,
      const scoped_refptr<AudioSourceInterface>& source);

  // MediaStreamTrack implementation.
  std::string kind() const override;

  // AudioTrackInterface implementation.
  AudioSourceInterface* GetSource() const override;

  void AddSink(AudioTrackSinkInterface* sink) override;
  void RemoveSink(AudioTrackSinkInterface* sink) override;

 private:
  // ObserverInterface implementation.
  void OnChanged() override;

 private:
  const scoped_refptr<AudioSourceInterface> audio_source_;
  RTC_NO_UNIQUE_ADDRESS SequenceChecker signaling_thread_checker_;
};

}  // namespace webrtc

#endif  // PC_AUDIO_TRACK_H_
