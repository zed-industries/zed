/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_AUDIO_FRAME_H_
#define API_AUDIO_AUDIO_FRAME_H_

#include <stddef.h>
#include <stdint.h>

#include <array>
#include <optional>

#include "api/array_view.h"
#include "api/audio/audio_view.h"
#include "api/audio/channel_layout.h"
#include "api/rtp_packet_infos.h"
#include "rtc_base/checks.h"

namespace webrtc {

// Default webrtc buffer size in milliseconds.
constexpr size_t kDefaultAudioBufferLengthMs = 10u;

// Default total number of audio buffers per second based on the default length.
constexpr size_t kDefaultAudioBuffersPerSec =
    1000u / kDefaultAudioBufferLengthMs;

// Returns the number of samples a buffer needs to hold for ~10ms of a single
// audio channel at a given sample rate.
// See also `AudioProcessing::GetFrameSize()`.
inline size_t SampleRateToDefaultChannelSize(size_t sample_rate) {
  // Basic sanity check. 192kHz is the highest supported input sample rate.
  RTC_DCHECK_LE(sample_rate, 192000);
  return sample_rate / kDefaultAudioBuffersPerSec;
}
/////////////////////////////////////////////////////////////////////

/* This class holds up to 120 ms of super-wideband (32 kHz) stereo audio. It
 * allows for adding and subtracting frames while keeping track of the resulting
 * states.
 *
 * Notes
 * - This is a de-facto api, not designed for external use. The AudioFrame class
 *   is in need of overhaul or even replacement, and anyone depending on it
 *   should be prepared for that.
 * - The total number of samples is samples_per_channel_ * num_channels_.
 * - Stereo data is interleaved starting with the left channel.
 */
class AudioFrame {
 public:
  // Using constexpr here causes linker errors unless the variable also has an
  // out-of-class definition, which is impractical in this header-only class.
  // (This makes no sense because it compiles as an enum value, which we most
  // certainly cannot take the address of, just fine.) C++17 introduces inline
  // variables which should allow us to switch to constexpr and keep this a
  // header-only class.
  enum : size_t {
    // Stereo, 32 kHz, 120 ms (2 * 32 * 120)
    // Stereo, 192 kHz, 20 ms (2 * 192 * 20)
    kMaxDataSizeSamples = 7680,
    kMaxDataSizeBytes = kMaxDataSizeSamples * sizeof(int16_t),
  };

  enum VADActivity { kVadActive = 0, kVadPassive = 1, kVadUnknown = 2 };
  enum SpeechType {
    kNormalSpeech = 0,
    kPLC = 1,
    kCNG = 2,
    kPLCCNG = 3,
    kCodecPLC = 5,
    kUndefined = 4
  };

  AudioFrame();

  // Construct an audio frame with frame length properties and channel
  // information. `samples_per_channel()` will be initialized to a 10ms buffer
  // size and if `layout` is not specified (default value of
  // CHANNEL_LAYOUT_UNSUPPORTED is set), then the channel layout is derived
  // (guessed) from `num_channels`.
  AudioFrame(int sample_rate_hz,
             size_t num_channels,
             ChannelLayout layout = CHANNEL_LAYOUT_UNSUPPORTED);

  AudioFrame(const AudioFrame&) = delete;
  AudioFrame& operator=(const AudioFrame&) = delete;

  // Resets all members to their default state.
  void Reset();
  // Same as Reset(), but leaves mute state unchanged. Muting a frame requires
  // the buffer to be zeroed on the next call to mutable_data(). Callers
  // intending to write to the buffer immediately after Reset() can instead use
  // ResetWithoutMuting() to skip this wasteful zeroing.
  void ResetWithoutMuting();

  // TODO: b/335805780 - Accept InterleavedView.
  void UpdateFrame(uint32_t timestamp,
                   const int16_t* data,
                   size_t samples_per_channel,
                   int sample_rate_hz,
                   SpeechType speech_type,
                   VADActivity vad_activity,
                   size_t num_channels = 1);

  void CopyFrom(const AudioFrame& src);

  // Sets a wall-time clock timestamp in milliseconds to be used for profiling
  // of time between two points in the audio chain.
  // Example:
  //   t0: UpdateProfileTimeStamp()
  //   t1: ElapsedProfileTimeMs() => t1 - t0 [msec]
  void UpdateProfileTimeStamp();
  // Returns the time difference between now and when UpdateProfileTimeStamp()
  // was last called. Returns -1 if UpdateProfileTimeStamp() has not yet been
  // called.
  int64_t ElapsedProfileTimeMs() const;

  // data() returns a zeroed static buffer if the frame is muted.
  // TODO: b/335805780 - Return InterleavedView.
  const int16_t* data() const;

  // Returns a read-only view of all the valid samples held by the AudioFrame.
  // For a muted AudioFrame, the samples will all be 0.
  InterleavedView<const int16_t> data_view() const;

  // mutable_frame() always returns a non-static buffer; the first call to
  // mutable_frame() zeros the buffer and marks the frame as unmuted.
  // TODO: b/335805780 - Return an InterleavedView.
  int16_t* mutable_data();

  // Grants write access to the audio buffer. The size of the returned writable
  // view is determined by the `samples_per_channel` and `num_channels`
  // dimensions which the function checks for correctness and stores in the
  // internal member variables; `samples_per_channel()` and `num_channels()`
  // respectively.
  // If the state is currently muted, the returned view will be zeroed out.
  InterleavedView<int16_t> mutable_data(size_t samples_per_channel,
                                        size_t num_channels);

  // Prefer to mute frames using AudioFrameOperations::Mute.
  void Mute();
  // Frame is muted by default.
  bool muted() const;

  size_t max_16bit_samples() const { return data_.size(); }
  size_t samples_per_channel() const { return samples_per_channel_; }
  size_t num_channels() const { return num_channels_; }

  ChannelLayout channel_layout() const { return channel_layout_; }
  // Sets the `channel_layout` property as well as `num_channels`.
  void SetLayoutAndNumChannels(ChannelLayout layout, size_t num_channels);

  int sample_rate_hz() const { return sample_rate_hz_; }

  void set_absolute_capture_timestamp_ms(
      int64_t absolute_capture_time_stamp_ms) {
    absolute_capture_timestamp_ms_ = absolute_capture_time_stamp_ms;
  }

  std::optional<int64_t> absolute_capture_timestamp_ms() const {
    return absolute_capture_timestamp_ms_;
  }

  // Sets the sample_rate_hz and samples_per_channel properties based on a
  // given sample rate and calculates a default 10ms samples_per_channel value.
  void SetSampleRateAndChannelSize(int sample_rate);

  // RTP timestamp of the first sample in the AudioFrame.
  uint32_t timestamp_ = 0;
  // Time since the first frame in milliseconds.
  // -1 represents an uninitialized value.
  int64_t elapsed_time_ms_ = -1;
  // NTP time of the estimated capture time in local timebase in milliseconds.
  // -1 represents an uninitialized value.
  int64_t ntp_time_ms_ = -1;
  size_t samples_per_channel_ = 0;
  int sample_rate_hz_ = 0;
  size_t num_channels_ = 0;
  SpeechType speech_type_ = kUndefined;
  VADActivity vad_activity_ = kVadUnknown;
  // Monotonically increasing timestamp intended for profiling of audio frames.
  // Typically used for measuring elapsed time between two different points in
  // the audio path. No lock is used to save resources and we are thread safe
  // by design.
  // TODO(nisse@webrtc.org): consider using std::optional.
  int64_t profile_timestamp_ms_ = 0;

  // Information about packets used to assemble this audio frame. This is needed
  // by `SourceTracker` when the frame is delivered to the RTCRtpReceiver's
  // MediaStreamTrack, in order to implement getContributingSources(). See:
  // https://w3c.github.io/webrtc-pc/#dom-rtcrtpreceiver-getcontributingsources
  //
  // TODO(bugs.webrtc.org/10757):
  //   Note that this information might not be fully accurate since we currently
  //   don't have a proper way to track it across the audio sync buffer. The
  //   sync buffer is the small sample-holding buffer located after the audio
  //   decoder and before where samples are assembled into output frames.
  //
  // `RtpPacketInfos` may also be empty if the audio samples did not come from
  // RTP packets. E.g. if the audio were locally generated by packet loss
  // concealment, comfort noise generation, etc.
  RtpPacketInfos packet_infos_;

 private:
  // A permanently zeroed out buffer to represent muted frames. This is a
  // header-only class, so the only way to avoid creating a separate zeroed
  // buffer per translation unit is to wrap a static in an inline function.
  static ArrayView<const int16_t> zeroed_data();

  std::array<int16_t, kMaxDataSizeSamples> data_;
  bool muted_ = true;
  ChannelLayout channel_layout_ = CHANNEL_LAYOUT_NONE;

  // Absolute capture timestamp when this audio frame was originally captured.
  // This is only valid for audio frames captured on this machine. The absolute
  // capture timestamp of a received frame is found in `packet_infos_`.
  // This timestamp MUST be based on the same clock as webrtc::TimeMillis().
  std::optional<int64_t> absolute_capture_timestamp_ms_;
};

}  // namespace webrtc

#endif  // API_AUDIO_AUDIO_FRAME_H_
