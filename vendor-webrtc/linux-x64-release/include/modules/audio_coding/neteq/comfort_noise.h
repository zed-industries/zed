/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_COMFORT_NOISE_H_
#define MODULES_AUDIO_CODING_NETEQ_COMFORT_NOISE_H_

#include <stddef.h>

namespace webrtc {

// Forward declarations.
class AudioMultiVector;
class DecoderDatabase;
class SyncBuffer;
struct Packet;

// This class acts as an interface to the CNG generator.
class ComfortNoise {
 public:
  enum ReturnCodes {
    kOK = 0,
    kUnknownPayloadType,
    kInternalError,
    kMultiChannelNotSupported
  };

  ComfortNoise(int fs_hz,
               DecoderDatabase* decoder_database,
               SyncBuffer* sync_buffer)
      : fs_hz_(fs_hz),
        first_call_(true),
        overlap_length_(5 * fs_hz_ / 8000),
        decoder_database_(decoder_database),
        sync_buffer_(sync_buffer) {}

  ComfortNoise(const ComfortNoise&) = delete;
  ComfortNoise& operator=(const ComfortNoise&) = delete;

  // Resets the state. Should be called before each new comfort noise period.
  void Reset();

  // Update the comfort noise generator with the parameters in `packet`.
  int UpdateParameters(const Packet& packet);

  // Generates `requested_length` samples of comfort noise and writes to
  // `output`. If this is the first in call after Reset (or first after creating
  // the object), it will also mix in comfort noise at the end of the
  // SyncBuffer object provided in the constructor.
  int Generate(size_t requested_length, AudioMultiVector* output);

  // Returns the last error code that was produced by the comfort noise
  // decoder. Returns 0 if no error has been encountered since the last reset.
  int internal_error_code() { return internal_error_code_; }

 private:
  int fs_hz_;
  bool first_call_;
  size_t overlap_length_;
  DecoderDatabase* decoder_database_;
  SyncBuffer* sync_buffer_;
  int internal_error_code_;
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_COMFORT_NOISE_H_
