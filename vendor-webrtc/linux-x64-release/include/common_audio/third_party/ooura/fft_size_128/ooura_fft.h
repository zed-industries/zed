/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_UTILITY_OOURA_FFT_H_
#define MODULES_AUDIO_PROCESSING_UTILITY_OOURA_FFT_H_

#include "rtc_base/system/arch.h"

namespace webrtc {

#if defined(WEBRTC_ARCH_X86_FAMILY)
void cft1st_128_SSE2(float* a);
void cftmdl_128_SSE2(float* a);
void rftfsub_128_SSE2(float* a);
void rftbsub_128_SSE2(float* a);
#endif

#if defined(MIPS_FPU_LE)
void cft1st_128_mips(float* a);
void cftmdl_128_mips(float* a);
void rftfsub_128_mips(float* a);
void rftbsub_128_mips(float* a);
#endif

#if defined(WEBRTC_HAS_NEON)
void cft1st_128_neon(float* a);
void cftmdl_128_neon(float* a);
void rftfsub_128_neon(float* a);
void rftbsub_128_neon(float* a);
#endif

class OouraFft {
 public:
  // Ctor allowing the availability of SSE2 support to be specified.
  explicit OouraFft(bool sse2_available);

  // Deprecated: This Ctor will soon be removed.
  OouraFft();
  ~OouraFft();
  void Fft(float* a) const;
  void InverseFft(float* a) const;

 private:
  void cft1st_128(float* a) const;
  void cftmdl_128(float* a) const;
  void rftfsub_128(float* a) const;
  void rftbsub_128(float* a) const;

  void cftfsub_128(float* a) const;
  void cftbsub_128(float* a) const;
  void bitrv2_128(float* a) const;
  bool use_sse2_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_UTILITY_OOURA_FFT_H_
