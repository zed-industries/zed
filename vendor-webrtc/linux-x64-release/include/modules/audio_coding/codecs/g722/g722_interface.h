/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_G722_G722_INTERFACE_H_
#define MODULES_AUDIO_CODING_CODECS_G722_G722_INTERFACE_H_

#include <stddef.h>
#include <stdint.h>

/*
 * Solution to support multiple instances
 */

typedef struct WebRtcG722EncInst G722EncInst;
typedef struct WebRtcG722DecInst G722DecInst;

/*
 * Comfort noise constants
 */

#define G722_WEBRTC_SPEECH 1
#define G722_WEBRTC_CNG 2

#ifdef __cplusplus
extern "C" {
#endif

/****************************************************************************
 * WebRtcG722_CreateEncoder(...)
 *
 * Create memory used for G722 encoder
 *
 * Input:
 *     - G722enc_inst         : G722 instance for encoder
 *
 * Return value               :  0 - Ok
 *                              -1 - Error
 */
int16_t WebRtcG722_CreateEncoder(G722EncInst** G722enc_inst);

/****************************************************************************
 * WebRtcG722_EncoderInit(...)
 *
 * This function initializes a G722 instance
 *
 * Input:
 *     - G722enc_inst         : G722 instance, i.e. the user that should receive
 *                             be initialized
 *
 * Return value               :  0 - Ok
 *                              -1 - Error
 */

int16_t WebRtcG722_EncoderInit(G722EncInst* G722enc_inst);

/****************************************************************************
 * WebRtcG722_FreeEncoder(...)
 *
 * Free the memory used for G722 encoder
 *
 * Input:
 *     - G722enc_inst         : G722 instance for encoder
 *
 * Return value               :  0 - Ok
 *                              -1 - Error
 */
int WebRtcG722_FreeEncoder(G722EncInst* G722enc_inst);

/****************************************************************************
 * WebRtcG722_Encode(...)
 *
 * This function encodes G722 encoded data.
 *
 * Input:
 *     - G722enc_inst         : G722 instance, i.e. the user that should encode
 *                              a packet
 *     - speechIn             : Input speech vector
 *     - len                  : Samples in speechIn
 *
 * Output:
 *        - encoded           : The encoded data vector
 *
 * Return value               : Length (in bytes) of coded data
 */

size_t WebRtcG722_Encode(G722EncInst* G722enc_inst,
                         const int16_t* speechIn,
                         size_t len,
                         uint8_t* encoded);

/****************************************************************************
 * WebRtcG722_CreateDecoder(...)
 *
 * Create memory used for G722 encoder
 *
 * Input:
 *     - G722dec_inst         : G722 instance for decoder
 *
 * Return value               :  0 - Ok
 *                              -1 - Error
 */
int16_t WebRtcG722_CreateDecoder(G722DecInst** G722dec_inst);

/****************************************************************************
 * WebRtcG722_DecoderInit(...)
 *
 * This function initializes a G722 instance
 *
 * Input:
 *     - inst      : G722 instance
 */

void WebRtcG722_DecoderInit(G722DecInst* inst);

/****************************************************************************
 * WebRtcG722_FreeDecoder(...)
 *
 * Free the memory used for G722 decoder
 *
 * Input:
 *     - G722dec_inst         : G722 instance for decoder
 *
 * Return value               :  0 - Ok
 *                              -1 - Error
 */

int WebRtcG722_FreeDecoder(G722DecInst* G722dec_inst);

/****************************************************************************
 * WebRtcG722_Decode(...)
 *
 * This function decodes a packet with G729 frame(s). Output speech length
 * will be a multiple of 80 samples (80*frames/packet).
 *
 * Input:
 *     - G722dec_inst       : G722 instance, i.e. the user that should decode
 *                            a packet
 *     - encoded            : Encoded G722 frame(s)
 *     - len                : Bytes in encoded vector
 *
 * Output:
 *        - decoded         : The decoded vector
 *      - speechType        : 1 normal, 2 CNG (Since G722 does not have its own
 *                            DTX/CNG scheme it should always return 1)
 *
 * Return value             : Samples in decoded vector
 */

size_t WebRtcG722_Decode(G722DecInst* G722dec_inst,
                         const uint8_t* encoded,
                         size_t len,
                         int16_t* decoded,
                         int16_t* speechType);

/****************************************************************************
 * WebRtcG722_Version(...)
 *
 * Get a string with the current version of the codec
 */

int16_t WebRtcG722_Version(char* versionStr, short len);

#ifdef __cplusplus
}
#endif

#endif /* MODULES_AUDIO_CODING_CODECS_G722_G722_INTERFACE_H_ */
