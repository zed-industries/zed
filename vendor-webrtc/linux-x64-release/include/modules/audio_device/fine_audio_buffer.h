/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_DEVICE_FINE_AUDIO_BUFFER_H_
#define MODULES_AUDIO_DEVICE_FINE_AUDIO_BUFFER_H_

#include <cstddef>
#include <cstdint>
#include <optional>

#include "api/array_view.h"
#include "rtc_base/buffer.h"

namespace webrtc {

class AudioDeviceBuffer;

// FineAudioBuffer takes an AudioDeviceBuffer (ADB) which deals with 16-bit PCM
// audio samples corresponding to 10ms of data. It then allows for this data
// to be pulled in a finer or coarser granularity. I.e. interacting with this
// class instead of directly with the AudioDeviceBuffer one can ask for any
// number of audio data samples. This class also ensures that audio data can be
// delivered to the ADB in 10ms chunks when the size of the provided audio
// buffers differs from 10ms.
// As an example: calling DeliverRecordedData() with 5ms buffers will deliver
// accumulated 10ms worth of data to the ADB every second call.
class FineAudioBuffer {
 public:
  // `device_buffer` is a buffer that provides 10ms of audio data.
  FineAudioBuffer(AudioDeviceBuffer* audio_device_buffer);
  ~FineAudioBuffer();

  // Clears buffers and counters dealing with playout and/or recording.
  void ResetPlayout();
  void ResetRecord();

  // Utility methods which returns true if valid parameters are acquired at
  // constructions.
  bool IsReadyForPlayout() const;
  bool IsReadyForRecord() const;

  // Copies audio samples into `audio_buffer` where number of requested
  // elements is specified by `audio_buffer.size()`. The producer will always
  // fill up the audio buffer and if no audio exists, the buffer will contain
  // silence instead. The provided delay estimate in `playout_delay_ms` should
  // contain an estimate of the latency between when an audio frame is read from
  // WebRTC and when it is played out on the speaker.
  void GetPlayoutData(ArrayView<int16_t> audio_buffer, int playout_delay_ms);

  // Consumes the audio data in `audio_buffer` and sends it to the WebRTC layer
  // in chunks of 10ms. The sum of the provided delay estimate in
  // `record_delay_ms` and the latest `playout_delay_ms` in GetPlayoutData()
  // are given to the AEC in the audio processing module.
  // They can be fixed values on most platforms and they are ignored if an
  // external (hardware/built-in) AEC is used.
  // Example: buffer size is 5ms => call #1 stores 5ms of data, call #2 stores
  // 5ms of data and sends a total of 10ms to WebRTC and clears the internal
  // cache. Call #3 restarts the scheme above.
  void DeliverRecordedData(ArrayView<const int16_t> audio_buffer,
                           int record_delay_ms) {
    DeliverRecordedData(audio_buffer, record_delay_ms, std::nullopt);
  }
  void DeliverRecordedData(ArrayView<const int16_t> audio_buffer,
                           int record_delay_ms,
                           std::optional<int64_t> capture_time_ns);

 private:
  // Device buffer that works with 10ms chunks of data both for playout and
  // for recording. I.e., the WebRTC side will always be asked for audio to be
  // played out in 10ms chunks and recorded audio will be sent to WebRTC in
  // 10ms chunks as well. This raw pointer is owned by the constructor of this
  // class and the owner must ensure that the pointer is valid during the life-
  // time of this object.
  AudioDeviceBuffer* const audio_device_buffer_;
  // Number of audio samples per channel per 10ms. Set once at construction
  // based on parameters in `audio_device_buffer`.
  const size_t playout_samples_per_channel_10ms_;
  const size_t record_samples_per_channel_10ms_;
  // Number of audio channels. Set once at construction based on parameters in
  // `audio_device_buffer`.
  const size_t playout_channels_;
  const size_t record_channels_;
  // Storage for output samples from which a consumer can read audio buffers
  // in any size using GetPlayoutData().
  BufferT<int16_t> playout_buffer_;
  // Storage for input samples that are about to be delivered to the WebRTC
  // ADB or remains from the last successful delivery of a 10ms audio buffer.
  BufferT<int16_t> record_buffer_;
  // Contains latest delay estimate given to GetPlayoutData().
  int playout_delay_ms_ = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_DEVICE_FINE_AUDIO_BUFFER_H_
