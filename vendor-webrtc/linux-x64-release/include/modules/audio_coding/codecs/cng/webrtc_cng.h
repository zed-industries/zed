/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_CNG_WEBRTC_CNG_H_
#define MODULES_AUDIO_CODING_CODECS_CNG_WEBRTC_CNG_H_

#include <stdint.h>

#include <cstddef>

#include "api/array_view.h"
#include "rtc_base/buffer.h"

#define WEBRTC_CNG_MAX_LPC_ORDER 12

namespace webrtc {

class ComfortNoiseDecoder {
 public:
  ComfortNoiseDecoder();
  ~ComfortNoiseDecoder() = default;

  ComfortNoiseDecoder(const ComfortNoiseDecoder&) = delete;
  ComfortNoiseDecoder& operator=(const ComfortNoiseDecoder&) = delete;

  void Reset();

  // Updates the CN state when a new SID packet arrives.
  // `sid` is a view of the SID packet without the headers.
  void UpdateSid(ArrayView<const uint8_t> sid);

  // Generates comfort noise.
  // `out_data` will be filled with samples - its size determines the number of
  // samples generated. When `new_period` is true, CNG history will be reset
  // before any audio is generated.  Returns `false` if outData is too large -
  // currently 640 bytes (equalling 10ms at 64kHz).
  // TODO(ossu): Specify better limits for the size of out_data. Either let it
  //             be unbounded or limit to 10ms in the current sample rate.
  bool Generate(ArrayView<int16_t> out_data, bool new_period);

 private:
  uint32_t dec_seed_;
  int32_t dec_target_energy_;
  int32_t dec_used_energy_;
  int16_t dec_target_reflCoefs_[WEBRTC_CNG_MAX_LPC_ORDER + 1];
  int16_t dec_used_reflCoefs_[WEBRTC_CNG_MAX_LPC_ORDER + 1];
  int16_t dec_filtstate_[WEBRTC_CNG_MAX_LPC_ORDER + 1];
  int16_t dec_filtstateLow_[WEBRTC_CNG_MAX_LPC_ORDER + 1];
  uint16_t dec_order_;
  int16_t dec_target_scale_factor_; /* Q29 */
  int16_t dec_used_scale_factor_;   /* Q29 */
};

class ComfortNoiseEncoder {
 public:
  // Creates a comfort noise encoder.
  // `fs` selects sample rate: 8000 for narrowband or 16000 for wideband.
  // `interval` sets the interval at which to generate SID data (in ms).
  // `quality` selects the number of refl. coeffs. Maximum allowed is 12.
  ComfortNoiseEncoder(int fs, int interval, int quality);
  ~ComfortNoiseEncoder() = default;

  ComfortNoiseEncoder(const ComfortNoiseEncoder&) = delete;
  ComfortNoiseEncoder& operator=(const ComfortNoiseEncoder&) = delete;

  // Resets the comfort noise encoder to its initial state.
  // Parameters are set as during construction.
  void Reset(int fs, int interval, int quality);

  // Analyzes background noise from `speech` and appends coefficients to
  // `output`.  Returns the number of coefficients generated.  If `force_sid` is
  // true, a SID frame is forced and the internal sid interval counter is reset.
  // Will fail if the input size is too large (> 640 samples, see
  // ComfortNoiseDecoder::Generate).
  size_t Encode(ArrayView<const int16_t> speech,
                bool force_sid,
                Buffer* output);

 private:
  size_t enc_nrOfCoefs_;
  int enc_sampfreq_;
  int16_t enc_interval_;
  int16_t enc_msSinceSid_;
  int32_t enc_Energy_;
  int16_t enc_reflCoefs_[WEBRTC_CNG_MAX_LPC_ORDER + 1];
  int32_t enc_corrVector_[WEBRTC_CNG_MAX_LPC_ORDER + 1];
  uint32_t enc_seed_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_CODECS_CNG_WEBRTC_CNG_H_
