/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_CODECS_AUDIO_DECODER_H_
#define API_AUDIO_CODECS_AUDIO_DECODER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <optional>
#include <vector>

#include "api/array_view.h"
#include "rtc_base/buffer.h"

namespace webrtc {

class AudioDecoder {
 public:
  enum SpeechType {
    kSpeech = 1,
    kComfortNoise = 2,
  };

  // Used by PacketDuration below. Save the value -1 for errors.
  enum { kNotImplemented = -2 };

  AudioDecoder() = default;
  virtual ~AudioDecoder() = default;

  AudioDecoder(const AudioDecoder&) = delete;
  AudioDecoder& operator=(const AudioDecoder&) = delete;

  class EncodedAudioFrame {
   public:
    struct DecodeResult {
      size_t num_decoded_samples;
      SpeechType speech_type;
    };

    virtual ~EncodedAudioFrame() = default;

    // Returns the duration in samples-per-channel of this audio frame.
    // If no duration can be ascertained, returns zero.
    virtual size_t Duration() const = 0;

    // Returns true if this packet contains DTX.
    virtual bool IsDtxPacket() const;

    // Decodes this frame of audio and writes the result in `decoded`.
    // `decoded` must be large enough to store as many samples as indicated by a
    // call to Duration() . On success, returns an std::optional containing the
    // total number of samples across all channels, as well as whether the
    // decoder produced comfort noise or speech. On failure, returns an empty
    // std::optional. Decode may be called at most once per frame object.
    virtual std::optional<DecodeResult> Decode(
        ArrayView<int16_t> decoded) const = 0;
  };

  struct ParseResult {
    ParseResult();
    ParseResult(uint32_t timestamp,
                int priority,
                std::unique_ptr<EncodedAudioFrame> frame);
    ParseResult(ParseResult&& b);
    ~ParseResult();

    ParseResult& operator=(ParseResult&& b);

    // The timestamp of the frame is in samples per channel.
    uint32_t timestamp;
    // The relative priority of the frame compared to other frames of the same
    // payload and the same timeframe. A higher value means a lower priority.
    // The highest priority is zero - negative values are not allowed.
    int priority;
    std::unique_ptr<EncodedAudioFrame> frame;
  };

  // Let the decoder parse this payload and prepare zero or more decodable
  // frames. Each frame must be between 10 ms and 120 ms long. The caller must
  // ensure that the AudioDecoder object outlives any frame objects returned by
  // this call. The decoder is free to swap or move the data from the `payload`
  // buffer. `timestamp` is the input timestamp, in samples, corresponding to
  // the start of the payload.
  virtual std::vector<ParseResult> ParsePayload(Buffer&& payload,
                                                uint32_t timestamp);

  // TODO(bugs.webrtc.org/10098): The Decode and DecodeRedundant methods are
  // obsolete; callers should call ParsePayload instead. For now, subclasses
  // must still implement DecodeInternal.

  // Decodes `encode_len` bytes from `encoded` and writes the result in
  // `decoded`. The maximum bytes allowed to be written into `decoded` is
  // `max_decoded_bytes`. Returns the total number of samples across all
  // channels. If the decoder produced comfort noise, `speech_type`
  // is set to kComfortNoise, otherwise it is kSpeech. The desired output
  // sample rate is provided in `sample_rate_hz`, which must be valid for the
  // codec at hand.
  int Decode(const uint8_t* encoded,
             size_t encoded_len,
             int sample_rate_hz,
             size_t max_decoded_bytes,
             int16_t* decoded,
             SpeechType* speech_type);

  // Same as Decode(), but interfaces to the decoders redundant decode function.
  // The default implementation simply calls the regular Decode() method.
  int DecodeRedundant(const uint8_t* encoded,
                      size_t encoded_len,
                      int sample_rate_hz,
                      size_t max_decoded_bytes,
                      int16_t* decoded,
                      SpeechType* speech_type);

  // Indicates if the decoder implements the DecodePlc method.
  virtual bool HasDecodePlc() const;

  // Calls the packet-loss concealment of the decoder to update the state after
  // one or several lost packets. The caller has to make sure that the
  // memory allocated in `decoded` should accommodate `num_frames` frames.
  virtual size_t DecodePlc(size_t num_frames, int16_t* decoded);

  // Asks the decoder to generate packet-loss concealment and append it to the
  // end of `concealment_audio`. The concealment audio should be in
  // channel-interleaved format, with as many channels as the last decoded
  // packet produced. The implementation must produce at least
  // requested_samples_per_channel, or nothing at all. This is a signal to the
  // caller to conceal the loss with other means. If the implementation provides
  // concealment samples, it is also responsible for "stitching" it together
  // with the decoded audio on either side of the concealment.
  // Note: The default implementation of GeneratePlc will be deleted soon. All
  // implementations must provide their own, which can be a simple as a no-op.
  // TODO(bugs.webrtc.org/9676): Remove default implementation.
  virtual void GeneratePlc(size_t requested_samples_per_channel,
                           BufferT<int16_t>* concealment_audio);

  // Resets the decoder state (empty buffers etc.).
  virtual void Reset() = 0;

  // Returns the last error code from the decoder.
  virtual int ErrorCode();

  // Returns the duration in samples-per-channel of the payload in `encoded`
  // which is `encoded_len` bytes long. Returns kNotImplemented if no duration
  // estimate is available, or -1 in case of an error.
  virtual int PacketDuration(const uint8_t* encoded, size_t encoded_len) const;

  // Returns the duration in samples-per-channel of the redandant payload in
  // `encoded` which is `encoded_len` bytes long. Returns kNotImplemented if no
  // duration estimate is available, or -1 in case of an error.
  virtual int PacketDurationRedundant(const uint8_t* encoded,
                                      size_t encoded_len) const;

  // Detects whether a packet has forward error correction. The packet is
  // comprised of the samples in `encoded` which is `encoded_len` bytes long.
  // Returns true if the packet has FEC and false otherwise.
  virtual bool PacketHasFec(const uint8_t* encoded, size_t encoded_len) const;

  // Returns the actual sample rate of the decoder's output. This value may not
  // change during the lifetime of the decoder.
  virtual int SampleRateHz() const = 0;

  // The number of channels in the decoder's output. This value may not change
  // during the lifetime of the decoder.
  virtual size_t Channels() const = 0;

  // The maximum number of audio channels supported by WebRTC decoders.
  static constexpr int kMaxNumberOfChannels = 24;

 protected:
  static SpeechType ConvertSpeechType(int16_t type);

  virtual int DecodeInternal(const uint8_t* encoded,
                             size_t encoded_len,
                             int sample_rate_hz,
                             int16_t* decoded,
                             SpeechType* speech_type) = 0;

  virtual int DecodeRedundantInternal(const uint8_t* encoded,
                                      size_t encoded_len,
                                      int sample_rate_hz,
                                      int16_t* decoded,
                                      SpeechType* speech_type);
};

}  // namespace webrtc
#endif  // API_AUDIO_CODECS_AUDIO_DECODER_H_
