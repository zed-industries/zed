/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_AUDIO_DEVICE_INCLUDE_TEST_AUDIO_DEVICE_H_
#define MODULES_AUDIO_DEVICE_INCLUDE_TEST_AUDIO_DEVICE_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <string>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/audio/audio_device.h"
#include "api/audio/audio_device_defines.h"
#include "api/scoped_refptr.h"
#include "api/task_queue/task_queue_factory.h"
#include "rtc_base/buffer.h"

namespace webrtc {

// This is test API and is in development, so it can be changed/removed without
// notice.

// This class exists for historical reasons. For now it only contains static
// methods to create test AudioDeviceModule. Implementation details of that
// module are considered private. This class isn't intended to be instantiated.
class TestAudioDeviceModule {
 public:
  // Returns the number of samples that Capturers and Renderers with this
  // sampling frequency will work with every time Capture or Render is called.
  static size_t SamplesPerFrame(int sampling_frequency_in_hz);

  class Capturer {
   public:
    virtual ~Capturer() {}
    // Returns the sampling frequency in Hz of the audio data that this
    // capturer produces.
    virtual int SamplingFrequency() const = 0;
    // Returns the number of channels of captured audio data.
    virtual int NumChannels() const = 0;
    // Replaces the contents of `buffer` with 10ms of captured audio data
    // (see TestAudioDeviceModule::SamplesPerFrame). Returns true if the
    // capturer can keep producing data, or false when the capture finishes.
    virtual bool Capture(BufferT<int16_t>* buffer) = 0;
  };

  class Renderer {
   public:
    virtual ~Renderer() {}
    // Returns the sampling frequency in Hz of the audio data that this
    // renderer receives.
    virtual int SamplingFrequency() const = 0;
    // Returns the number of channels of audio data to be required.
    virtual int NumChannels() const = 0;
    // Renders the passed audio data and returns true if the renderer wants
    // to keep receiving data, or false otherwise.
    virtual bool Render(ArrayView<const int16_t> data) = 0;
  };

  // A fake capturer that generates pulses with random samples between
  // -max_amplitude and +max_amplitude.
  class PulsedNoiseCapturer : public Capturer {
   public:
    ~PulsedNoiseCapturer() override {}

    virtual void SetMaxAmplitude(int16_t amplitude) = 0;
  };

  // Creates a new TestAudioDeviceModule. When capturing or playing, 10 ms audio
  // frames will be processed every 10ms / `speed`.
  // `capturer` is an object that produces audio data. Can be nullptr if this
  // device is never used for recording.
  // `renderer` is an object that receives audio data that would have been
  // played out. Can be nullptr if this device is never used for playing.
  // Use one of the Create... functions to get these instances.
  static scoped_refptr<AudioDeviceModule> Create(
      TaskQueueFactory* task_queue_factory,
      std::unique_ptr<Capturer> capturer,
      std::unique_ptr<Renderer> renderer,
      float speed = 1);

  // Returns a Capturer instance that generates a signal of `num_channels`
  // channels where every second frame is zero and every second frame is evenly
  // distributed random noise with max amplitude `max_amplitude`.
  static std::unique_ptr<PulsedNoiseCapturer> CreatePulsedNoiseCapturer(
      int16_t max_amplitude,
      int sampling_frequency_in_hz,
      int num_channels = 1);

  // Returns a Renderer instance that does nothing with the audio data.
  static std::unique_ptr<Renderer> CreateDiscardRenderer(
      int sampling_frequency_in_hz,
      int num_channels = 1);

  // WavReader and WavWriter creation based on file name.

  // Returns a Capturer instance that gets its data from a WAV file. The sample
  // rate and channels will be checked against the Wav file.
  static std::unique_ptr<Capturer> CreateWavFileReader(
      absl::string_view filename,
      int sampling_frequency_in_hz,
      int num_channels = 1);

  // Returns a Capturer instance that gets its data from a file.
  // Automatically detects sample rate and num of channels.
  // `repeat` - if true, the file will be replayed from the start when we reach
  // the end of file.
  static std::unique_ptr<Capturer> CreateWavFileReader(
      absl::string_view filename,
      bool repeat = false);

  // Returns a Renderer instance that writes its data to a file.
  static std::unique_ptr<Renderer> CreateWavFileWriter(
      absl::string_view filename,
      int sampling_frequency_in_hz,
      int num_channels = 1);

  // Returns a Renderer instance that writes its data to a WAV file, cutting
  // off silence at the beginning (not necessarily perfect silence, see
  // kAmplitudeThreshold) and at the end (only actual 0 samples in this case).
  static std::unique_ptr<Renderer> CreateBoundedWavFileWriter(
      absl::string_view filename,
      int sampling_frequency_in_hz,
      int num_channels = 1);

  // Returns a Capturer instance that gets its data from a raw file (*.raw).
  static std::unique_ptr<Capturer> CreateRawFileReader(
      absl::string_view filename,
      int sampling_frequency_in_hz = 48000,
      int num_channels = 2,
      bool repeat = true);

  // Returns a Renderer instance that writes its data to a raw file (*.raw),
  // cutting off silence at the beginning (not necessarily perfect silence, see
  // kAmplitudeThreshold) and at the end (only actual 0 samples in this case).
  static std::unique_ptr<Renderer> CreateRawFileWriter(
      absl::string_view filename,
      int sampling_frequency_in_hz = 48000,
      int num_channels = 2);

 private:
  TestAudioDeviceModule() = default;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_DEVICE_INCLUDE_TEST_AUDIO_DEVICE_H_
