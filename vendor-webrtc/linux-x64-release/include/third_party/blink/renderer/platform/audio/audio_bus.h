/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_BUS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_BUS_H_

#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/audio/audio_channel.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// An AudioBus represents a collection of one or more AudioChannels.
// The data layout is "planar" as opposed to "interleaved".  An AudioBus with
// one channel is mono, an AudioBus with two channels is stereo, etc.
class PLATFORM_EXPORT AudioBus final : public ThreadSafeRefCounted<AudioBus> {
 public:
  enum {
    kChannelLeft = 0,
    kChannelRight = 1,
    kChannelCenter = 2,  // center and mono are the same
    kChannelMono = 2,
    kChannelLFE = 3,
    kChannelSurroundLeft = 4,
    kChannelSurroundRight = 5,
  };

  enum {
    kLayoutCanonical = 0
    // Can define non-standard layouts here
  };

  enum ChannelInterpretation {
    kSpeakers,
    kDiscrete,
  };

  // allocate indicates whether or not to initially have the AudioChannels
  // created with managed storage.  Normal usage is to pass true here, in which
  // case the AudioChannels will memory-manage their own storage.  If allocate
  // is false then setChannelMemory() has to be called later on for each
  // channel before the AudioBus is useable...
  static scoped_refptr<AudioBus> Create(unsigned number_of_channels,
                                        uint32_t length,
                                        bool allocate = true);

  // Pass in 0.0 for sampleRate to use the file's sample-rate, otherwise a
  // sample-rate conversion to the requested sampleRate will be made (if it
  // doesn't already match the file's sample-rate).  The created buffer will
  // have its sample-rate set correctly to the result.
  static scoped_refptr<AudioBus> CreateBusFromInMemoryAudioFile(
      base::span<const uint8_t> data,
      bool mix_to_mono,
      float sample_rate);

  AudioBus(const AudioBus&) = delete;
  AudioBus& operator=(const AudioBus&) = delete;

  // Tells the given channel to use an externally allocated buffer.
  void SetChannelMemory(unsigned channel_index,
                        float* storage,
                        uint32_t length);

  // Channels
  unsigned NumberOfChannels() const { return channels_.size(); }

  AudioChannel* Channel(unsigned channel) { return &channels_[channel]; }
  const AudioChannel* Channel(unsigned channel) const {
    return &channels_[channel];
  }
  AudioChannel* ChannelByType(unsigned type);
  const AudioChannel* ChannelByType(unsigned type) const;

  // Number of sample-frames
  uint32_t length() const { return length_; }

  // resizeSmaller() can only be called with a new length <= the current length.
  // The data stored in the bus will remain undisturbed.
  void ResizeSmaller(uint32_t new_length);

  // Sample-rate : 0.0 if unknown or "don't care"
  float SampleRate() const { return sample_rate_; }
  void SetSampleRate(float sample_rate) { sample_rate_ = sample_rate; }

  // Zeroes all channels.
  void Zero();

  // Clears the silent flag on all channels.
  void ClearSilentFlag();

  // Returns true if the silent bit is set on all channels.
  bool IsSilent() const;

  // Returns true if the channel count and frame-size match.
  bool TopologyMatches(const AudioBus& source_bus) const;

  // Creates a new buffer from a range in the source buffer.
  // 0 may be returned if the range does not fit in the sourceBuffer
  static scoped_refptr<AudioBus> CreateBufferFromRange(
      const AudioBus* source_buffer,
      unsigned start_frame,
      unsigned end_frame);

  // Creates a new AudioBus by sample-rate converting sourceBus to the
  // newSampleRate.
  // setSampleRate() must have been previously called on sourceBus.
  // Note: sample-rate conversion is already handled in the file-reading code
  // for the mac port, so we don't need this.
  static scoped_refptr<AudioBus> CreateBySampleRateConverting(
      const AudioBus* source_bus,
      bool mix_to_mono,
      double new_sample_rate);

  // Creates a new AudioBus by mixing all the channels down to mono.
  // If sourceBus is already mono, then the returned AudioBus will simply be a
  // copy.
  static scoped_refptr<AudioBus> CreateByMixingToMono(
      const AudioBus* source_bus);

  // Scales all samples by the same amount.
  void Scale(float scale);

  // Copies the samples from the source bus to this one.
  // This is just a simple per-channel copy if the number of channels match,
  // otherwise an up-mix or down-mix is done.
  void CopyFrom(const AudioBus& source_bus, ChannelInterpretation = kSpeakers);

  // Sums the samples from the source bus to this one.
  // This is just a simple per-channel summing if the number of channels match,
  // otherwise an up-mix or down-mix is done.
  void SumFrom(const AudioBus& source_bus, ChannelInterpretation = kSpeakers);

  // Copy each channel from |source_bus| into our corresponding channel.  We
  // scale |source_bus| by |gain| before copying into the bus.
  void CopyWithGainFrom(const AudioBus& source_bus, float gain);

  // Copies the sourceBus by scaling with sample-accurate gain values.
  void CopyWithSampleAccurateGainValuesFrom(const AudioBus& source_bus,
                                            float* gain_values,
                                            unsigned number_of_gain_values);

  // Returns maximum absolute value across all channels (useful for
  // normalization).
  float MaxAbsValue() const;

  // Makes maximum absolute value == 1.0 (if possible).
  void Normalize();

  static scoped_refptr<AudioBus> GetDataResource(int resource_id,
                                                 float sample_rate);

 private:
  AudioBus(unsigned number_of_channels, uint32_t length, bool allocate);

  void DiscreteSumFrom(const AudioBus&);

  // Up/down-mix by in-place summing upon the existing channel content.
  // http://webaudio.github.io/web-audio-api/#channel-up-mixing-and-down-mixing
  void SumFromByUpMixing(const AudioBus&);
  void SumFromByDownMixing(const AudioBus&);

  uint32_t length_;
  Vector<AudioChannel, 2> channels_;
  int layout_;
  float sample_rate_;  // 0.0 if unknown or N/A
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_BUS_H_
