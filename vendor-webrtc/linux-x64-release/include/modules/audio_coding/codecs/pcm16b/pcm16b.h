/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_PCM16B_PCM16B_H_
#define MODULES_AUDIO_CODING_CODECS_PCM16B_PCM16B_H_
/*
 * Define the fixpoint numeric formats
 */

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/****************************************************************************
 * WebRtcPcm16b_Encode(...)
 *
 * "Encode" a sample vector to 16 bit linear (Encoded standard is big endian)
 *
 * Input:
 *              - speech        : Input speech vector
 *              - len           : Number of samples in speech vector
 *
 * Output:
 *              - encoded       : Encoded data vector (big endian 16 bit)
 *
 * Returned value               : Length (in bytes) of coded data.
 *                                Always equal to twice the len input parameter.
 */

size_t WebRtcPcm16b_Encode(const int16_t* speech, size_t len, uint8_t* encoded);

/****************************************************************************
 * WebRtcPcm16b_Decode(...)
 *
 * "Decode" a vector to 16 bit linear (Encoded standard is big endian)
 *
 * Input:
 *              - encoded       : Encoded data vector (big endian 16 bit)
 *              - len           : Number of bytes in encoded
 *
 * Output:
 *              - speech        : Decoded speech vector
 *
 * Returned value               : Samples in speech
 */

size_t WebRtcPcm16b_Decode(const uint8_t* encoded, size_t len, int16_t* speech);

#ifdef __cplusplus
}
#endif

#endif /* MODULES_AUDIO_CODING_CODECS_PCM16B_PCM16B_H_ */
