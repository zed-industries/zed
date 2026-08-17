/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_AUDIO_SIGNAL_PROCESSING_INCLUDE_REAL_FFT_H_
#define COMMON_AUDIO_SIGNAL_PROCESSING_INCLUDE_REAL_FFT_H_

#include <stdint.h>

// For ComplexFFT(), the maximum fft order is 10;
// WebRTC APM uses orders of only 7 and 8.
enum { kMaxFFTOrder = 10 };

struct RealFFT;

#ifdef __cplusplus
extern "C" {
#endif

struct RealFFT* WebRtcSpl_CreateRealFFT(int order);
void WebRtcSpl_FreeRealFFT(struct RealFFT* self);

// Compute an FFT for a real-valued signal of length of 2^order,
// where 1 < order <= MAX_FFT_ORDER. Transform length is determined by the
// specification structure, which must be initialized prior to calling the FFT
// function with WebRtcSpl_CreateRealFFT().
// The relationship between the input and output sequences can
// be expressed in terms of the DFT, i.e.:
//     x[n] = (2^(-scalefactor)/N)  . SUM[k=0,...,N-1] X[k].e^(jnk.2.pi/N)
//     n=0,1,2,...N-1
//     N=2^order.
// The conjugate-symmetric output sequence is represented using a CCS vector,
// which is of length N+2, and is organized as follows:
//     Index:      0  1  2  3  4  5   . . .   N-2       N-1       N       N+1
//     Component:  R0 0  R1 I1 R2 I2  . . .   R[N/2-1]  I[N/2-1]  R[N/2]  0
// where R[n] and I[n], respectively, denote the real and imaginary components
// for FFT bin 'n'. Bins  are numbered from 0 to N/2, where N is the FFT length.
// Bin index 0 corresponds to the DC component, and bin index N/2 corresponds to
// the foldover frequency.
//
// Input Arguments:
//   self - pointer to preallocated and initialized FFT specification structure.
//   real_data_in - the input signal. For an ARM Neon platform, it must be
//                  aligned on a 32-byte boundary.
//
// Output Arguments:
//   complex_data_out - the output complex signal with (2^order + 2) 16-bit
//                      elements. For an ARM Neon platform, it must be different
//                      from real_data_in, and aligned on a 32-byte boundary.
//
// Return Value:
//   0  - FFT calculation is successful.
//   -1 - Error with bad arguments (null pointers).
int WebRtcSpl_RealForwardFFT(struct RealFFT* self,
                             const int16_t* real_data_in,
                             int16_t* complex_data_out);

// Compute the inverse FFT for a conjugate-symmetric input sequence of length of
// 2^order, where 1 < order <= MAX_FFT_ORDER. Transform length is determined by
// the specification structure, which must be initialized prior to calling the
// FFT function with WebRtcSpl_CreateRealFFT().
// For a transform of length M, the input sequence is represented using a packed
// CCS vector of length M+2, which is explained in the comments for
// WebRtcSpl_RealForwardFFTC above.
//
// Input Arguments:
//   self - pointer to preallocated and initialized FFT specification structure.
//   complex_data_in - the input complex signal with (2^order + 2) 16-bit
//                     elements. For an ARM Neon platform, it must be aligned on
//                     a 32-byte boundary.
//
// Output Arguments:
//   real_data_out - the output real signal. For an ARM Neon platform, it must
//                   be different to complex_data_in, and aligned on a 32-byte
//                   boundary.
//
// Return Value:
//   0 or a positive number - a value that the elements in the `real_data_out`
//                            should be shifted left with in order to get
//                            correct physical values.
//   -1 - Error with bad arguments (null pointers).
int WebRtcSpl_RealInverseFFT(struct RealFFT* self,
                             const int16_t* complex_data_in,
                             int16_t* real_data_out);

#ifdef __cplusplus
}
#endif

#endif  // COMMON_AUDIO_SIGNAL_PROCESSING_INCLUDE_REAL_FFT_H_
