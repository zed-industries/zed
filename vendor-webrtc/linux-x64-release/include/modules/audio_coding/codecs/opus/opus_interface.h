/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_OPUS_OPUS_INTERFACE_H_
#define MODULES_AUDIO_CODING_CODECS_OPUS_OPUS_INTERFACE_H_

#include <stddef.h>
#include <stdint.h>

#include "modules/audio_coding/codecs/opus/opus_inst.h"

#ifdef __cplusplus
extern "C" {
#endif

// Opaque wrapper types for the codec state.
typedef struct WebRtcOpusEncInst OpusEncInst;
typedef struct WebRtcOpusDecInst OpusDecInst;

/****************************************************************************
 * WebRtcOpus_EncoderCreate(...)
 *
 * This function creates an Opus encoder that encodes mono or stereo.
 *
 * Input:
 *      - channels           : number of channels; 1 or 2.
 *      - application        : 0 - VOIP applications.
 *                                 Favor speech intelligibility.
 *                             1 - Audio applications.
 *                                 Favor faithfulness to the original input.
 *      - sample_rate_hz     : sample rate of input audio
 *
 * Output:
 *      - inst               : a pointer to Encoder context that is created
 *                             if success.
 *
 * Return value              : 0 - Success
 *                            -1 - Error
 */
int16_t WebRtcOpus_EncoderCreate(OpusEncInst** inst,
                                 size_t channels,
                                 int32_t application,
                                 int sample_rate_hz);

/****************************************************************************
 * WebRtcOpus_MultistreamEncoderCreate(...)
 *
 * This function creates an Opus encoder with any supported channel count.
 *
 * Input:
 *      - channels           : number of channels in the input of the encoder.
 *      - application        : 0 - VOIP applications.
 *                                 Favor speech intelligibility.
 *                             1 - Audio applications.
 *                                 Favor faithfulness to the original input.
 *      - streams            : number of streams, as described in RFC 7845.
 *      - coupled_streams    : number of coupled streams, as described in
 *                             RFC 7845.
 *      - channel_mapping    : the channel mapping; pointer to array of
 *                             `channel` bytes, as described in RFC 7845.
 *
 * Output:
 *      - inst               : a pointer to Encoder context that is created
 *                             if success.
 *
 * Return value              : 0 - Success
 *                            -1 - Error
 */
int16_t WebRtcOpus_MultistreamEncoderCreate(
    OpusEncInst** inst,
    size_t channels,
    int32_t application,
    size_t streams,
    size_t coupled_streams,
    const unsigned char* channel_mapping);

int16_t WebRtcOpus_EncoderFree(OpusEncInst* inst);

/****************************************************************************
 * WebRtcOpus_Encode(...)
 *
 * This function encodes audio as a series of Opus frames and inserts
 * it into a packet. Input buffer can be any length.
 *
 * Input:
 *      - inst                  : Encoder context
 *      - audio_in              : Input speech data buffer
 *      - samples               : Samples per channel in audio_in
 *      - length_encoded_buffer : Output buffer size
 *
 * Output:
 *      - encoded               : Output compressed data buffer
 *
 * Return value                 : >=0 - Length (in bytes) of coded data
 *                                -1 - Error
 */
int WebRtcOpus_Encode(OpusEncInst* inst,
                      const int16_t* audio_in,
                      size_t samples,
                      size_t length_encoded_buffer,
                      uint8_t* encoded);

/****************************************************************************
 * WebRtcOpus_SetBitRate(...)
 *
 * This function adjusts the target bitrate of the encoder.
 *
 * Input:
 *      - inst               : Encoder context
 *      - rate               : New target bitrate
 *
 * Return value              :  0 - Success
 *                             -1 - Error
 */
int16_t WebRtcOpus_SetBitRate(OpusEncInst* inst, int32_t rate);

/****************************************************************************
 * WebRtcOpus_SetPacketLossRate(...)
 *
 * This function configures the encoder's expected packet loss percentage.
 *
 * Input:
 *      - inst               : Encoder context
 *      - loss_rate          : loss percentage in the range 0-100, inclusive.
 * Return value              :  0 - Success
 *                             -1 - Error
 */
int16_t WebRtcOpus_SetPacketLossRate(OpusEncInst* inst, int32_t loss_rate);

/****************************************************************************
 * WebRtcOpus_SetMaxPlaybackRate(...)
 *
 * Configures the maximum playback rate for encoding. Due to hardware
 * limitations, the receiver may render audio up to a playback rate. Opus
 * encoder can use this information to optimize for network usage and encoding
 * complexity. This will affect the audio bandwidth in the coded audio. However,
 * the input/output sample rate is not affected.
 *
 * Input:
 *      - inst               : Encoder context
 *      - frequency_hz       : Maximum playback rate in Hz.
 *                             This parameter can take any value. The relation
 *                             between the value and the Opus internal mode is
 *                             as following:
 *                             frequency_hz <= 8000           narrow band
 *                             8000 < frequency_hz <= 12000   medium band
 *                             12000 < frequency_hz <= 16000  wide band
 *                             16000 < frequency_hz <= 24000  super wide band
 *                             frequency_hz > 24000           full band
 * Return value              :  0 - Success
 *                             -1 - Error
 */
int16_t WebRtcOpus_SetMaxPlaybackRate(OpusEncInst* inst, int32_t frequency_hz);

/****************************************************************************
 * WebRtcOpus_GetMaxPlaybackRate(...)
 *
 * Queries the maximum playback rate for encoding. If different single-stream
 * encoders have different maximum playback rates, this function fails.
 *
 * Input:
 *      - inst               : Encoder context.
 * Output:
 *      - result_hz          : The maximum playback rate in Hz.
 * Return value              :  0 - Success
 *                             -1 - Error
 */
int16_t WebRtcOpus_GetMaxPlaybackRate(OpusEncInst* const inst,
                                      int32_t* result_hz);

/* TODO(minyue): Check whether an API to check the FEC and the packet loss rate
 * is needed. It might not be very useful since there are not many use cases and
 * the caller can always maintain the states. */

/****************************************************************************
 * WebRtcOpus_EnableFec()
 *
 * This function enables FEC for encoding.
 *
 * Input:
 *      - inst               : Encoder context
 *
 * Return value              :  0 - Success
 *                             -1 - Error
 */
int16_t WebRtcOpus_EnableFec(OpusEncInst* inst);

/****************************************************************************
 * WebRtcOpus_DisableFec()
 *
 * This function disables FEC for encoding.
 *
 * Input:
 *      - inst               : Encoder context
 *
 * Return value              :  0 - Success
 *                             -1 - Error
 */
int16_t WebRtcOpus_DisableFec(OpusEncInst* inst);

/****************************************************************************
 * WebRtcOpus_EnableDtx()
 *
 * This function enables Opus internal DTX for encoding.
 *
 * Input:
 *      - inst               : Encoder context
 *
 * Return value              :  0 - Success
 *                             -1 - Error
 */
int16_t WebRtcOpus_EnableDtx(OpusEncInst* inst);

/****************************************************************************
 * WebRtcOpus_DisableDtx()
 *
 * This function disables Opus internal DTX for encoding.
 *
 * Input:
 *      - inst               : Encoder context
 *
 * Return value              :  0 - Success
 *                             -1 - Error
 */
int16_t WebRtcOpus_DisableDtx(OpusEncInst* inst);

/****************************************************************************
 * WebRtcOpus_GetUseDtx()
 *
 * This function gets the DTX configuration used for encoding.
 *
 * Input:
 *      - inst               : Encoder context
 *
 * Return value              :  0 - Encoder does not use DTX.
 *                              1 - Encoder uses DTX.
 *                             -1 - Error.
 */
int16_t WebRtcOpus_GetUseDtx(OpusEncInst* inst);

/****************************************************************************
 * WebRtcOpus_GetUseDtx()
 *
 * This function gets if the encoder is in DTX.
 *
 * Input:
 *      - inst               : Encoder context
 *
 * Return value              :  0 - Encoder is not DTX.
 *                              1 - Encoder is in DTX.
 *                             -1 - Error.
 */
int16_t WebRtcOpus_GetInDtx(OpusEncInst* inst);

/****************************************************************************
 * WebRtcOpus_EnableCbr()
 *
 * This function enables CBR for encoding.
 *
 * Input:
 *      - inst               : Encoder context
 *
 * Return value              :  0 - Success
 *                             -1 - Error
 */
int16_t WebRtcOpus_EnableCbr(OpusEncInst* inst);

/****************************************************************************
 * WebRtcOpus_DisableCbr()
 *
 * This function disables CBR for encoding.
 *
 * Input:
 *      - inst               : Encoder context
 *
 * Return value              :  0 - Success
 *                             -1 - Error
 */
int16_t WebRtcOpus_DisableCbr(OpusEncInst* inst);

/*
 * WebRtcOpus_SetComplexity(...)
 *
 * This function adjusts the computational complexity. The effect is the same as
 * calling the complexity setting of Opus as an Opus encoder related CTL.
 *
 * Input:
 *      - inst               : Encoder context
 *      - complexity         : New target complexity (0-10, inclusive)
 *
 * Return value              :  0 - Success
 *                             -1 - Error
 */
int16_t WebRtcOpus_SetComplexity(OpusEncInst* inst, int32_t complexity);

/*
 * WebRtcOpus_GetBandwidth(...)
 *
 * This function returns the current bandwidth.
 *
 * Input:
 *      - inst               : Encoder context
 *
 * Return value              :  Bandwidth - Success
 *                             -1 - Error
 */
int32_t WebRtcOpus_GetBandwidth(OpusEncInst* inst);

/*
 * WebRtcOpus_SetBandwidth(...)
 *
 * By default Opus decides which bandwidth to encode the signal in depending on
 * the the bitrate. This function overrules the previous setting and forces the
 * encoder to encode in narrowband/wideband/fullband/etc.
 *
 * Input:
 *      - inst               : Encoder context
 *      - bandwidth          : New target bandwidth. Valid values are:
 *                             OPUS_BANDWIDTH_NARROWBAND
 *                             OPUS_BANDWIDTH_MEDIUMBAND
 *                             OPUS_BANDWIDTH_WIDEBAND
 *                             OPUS_BANDWIDTH_SUPERWIDEBAND
 *                             OPUS_BANDWIDTH_FULLBAND
 *
 * Return value              :  0 - Success
 *                             -1 - Error
 */
int16_t WebRtcOpus_SetBandwidth(OpusEncInst* inst, int32_t bandwidth);

/*
 * WebRtcOpus_SetForceChannels(...)
 *
 * If the encoder is initialized as a stereo encoder, Opus will by default
 * decide whether to encode in mono or stereo based on the bitrate. This
 * function overrules the previous setting, and forces the encoder to encode
 * in auto/mono/stereo.
 *
 * If the Encoder is initialized as a mono encoder, and one tries to force
 * stereo, the function will return an error.
 *
 * Input:
 *      - inst               : Encoder context
 *      - num_channels       : 0 - Not forced
 *                             1 - Mono
 *                             2 - Stereo
 *
 * Return value              :  0 - Success
 *                             -1 - Error
 */
int16_t WebRtcOpus_SetForceChannels(OpusEncInst* inst, size_t num_channels);

int16_t WebRtcOpus_DecoderCreate(OpusDecInst** inst,
                                 size_t channels,
                                 int sample_rate_hz);

/****************************************************************************
 * WebRtcOpus_MultistreamDecoderCreate(...)
 *
 * This function creates an Opus decoder with any supported channel count.
 *
 * Input:
 *      - channels           : number of output channels that the decoder
 *                             will produce.
 *      - streams            : number of encoded streams, as described in
 *                             RFC 7845.
 *      - coupled_streams    : number of coupled streams, as described in
 *                             RFC 7845.
 *      - channel_mapping    : the channel mapping; pointer to array of
 *                             `channel` bytes, as described in RFC 7845.
 *
 * Output:
 *      - inst               : a pointer to a Decoder context that is created
 *                             if success.
 *
 * Return value              : 0 - Success
 *                            -1 - Error
 */
int16_t WebRtcOpus_MultistreamDecoderCreate(
    OpusDecInst** inst,
    size_t channels,
    size_t streams,
    size_t coupled_streams,
    const unsigned char* channel_mapping);

int16_t WebRtcOpus_DecoderFree(OpusDecInst* inst);

/****************************************************************************
 * WebRtcOpus_DecoderChannels(...)
 *
 * This function returns the number of channels created for Opus decoder.
 */
size_t WebRtcOpus_DecoderChannels(OpusDecInst* inst);

/****************************************************************************
 * WebRtcOpus_DecoderInit(...)
 *
 * This function resets state of the decoder.
 *
 * Input:
 *      - inst               : Decoder context
 */
void WebRtcOpus_DecoderInit(OpusDecInst* inst);

/****************************************************************************
 * WebRtcOpus_Decode(...)
 *
 * This function decodes an Opus packet into one or more audio frames at the
 * ACM interface's sampling rate (32 kHz).
 *
 * Input:
 *      - inst               : Decoder context
 *      - encoded            : Encoded data
 *      - encoded_bytes      : Bytes in encoded vector
 *
 * Output:
 *      - decoded            : The decoded vector
 *      - audio_type         : 1 normal, 2 CNG
 *
 * Return value              : >0 - Samples per channel in decoded vector
 *                             -1 - Error
 */
int WebRtcOpus_Decode(OpusDecInst* inst,
                      const uint8_t* encoded,
                      size_t encoded_bytes,
                      int16_t* decoded,
                      int16_t* audio_type);

/****************************************************************************
 * WebRtcOpus_DecodeFec(...)
 *
 * This function decodes the FEC data from an Opus packet into one or more audio
 * frames at the ACM interface's sampling rate (32 kHz).
 *
 * Input:
 *      - inst               : Decoder context
 *      - encoded            : Encoded data
 *      - encoded_bytes      : Bytes in encoded vector
 *
 * Output:
 *      - decoded            : The decoded vector (previous frame)
 *
 * Return value              : >0 - Samples per channel in decoded vector
 *                              0 - No FEC data in the packet
 *                             -1 - Error
 */
int WebRtcOpus_DecodeFec(OpusDecInst* inst,
                         const uint8_t* encoded,
                         size_t encoded_bytes,
                         int16_t* decoded,
                         int16_t* audio_type);

/****************************************************************************
 * WebRtcOpus_DurationEst(...)
 *
 * This function calculates the duration of an opus packet.
 * Input:
 *        - inst                 : Decoder context
 *        - payload              : Encoded data pointer
 *        - payload_length_bytes : Bytes of encoded data
 *
 * Return value                  : The duration of the packet, in samples per
 *                                 channel.
 */
int WebRtcOpus_DurationEst(OpusDecInst* inst,
                           const uint8_t* payload,
                           size_t payload_length_bytes);

/****************************************************************************
 * WebRtcOpus_PlcDuration(...)
 *
 * This function calculates the duration of a frame returned by packet loss
 * concealment (PLC).
 *
 * Input:
 *        - inst                 : Decoder context
 *
 * Return value                  : The duration of a frame returned by PLC, in
 *                                 samples per channel.
 */
int WebRtcOpus_PlcDuration(OpusDecInst* inst);

/* TODO(minyue): Check whether it is needed to add a decoder context to the
 * arguments, like WebRtcOpus_DurationEst(...). In fact, the packet itself tells
 * the duration. The decoder context in WebRtcOpus_DurationEst(...) is not used.
 * So it may be advisable to remove it from WebRtcOpus_DurationEst(...). */

/****************************************************************************
 * WebRtcOpus_FecDurationEst(...)
 *
 * This function calculates the duration of the FEC data within an opus packet.
 * Input:
 *        - payload              : Encoded data pointer
 *        - payload_length_bytes : Bytes of encoded data
 *        - sample_rate_hz       : Sample rate of output audio
 *
 * Return value                  : >0 - The duration of the FEC data in the
 *                                 packet in samples per channel.
 *                                  0 - No FEC data in the packet.
 */
int WebRtcOpus_FecDurationEst(const uint8_t* payload,
                              size_t payload_length_bytes,
                              int sample_rate_hz);

/****************************************************************************
 * WebRtcOpus_PacketHasFec(...)
 *
 * This function detects if an opus packet has FEC.
 * Input:
 *        - payload              : Encoded data pointer
 *        - payload_length_bytes : Bytes of encoded data
 *
 * Return value                  : 0 - the packet does NOT contain FEC.
 *                                 1 - the packet contains FEC.
 */
int WebRtcOpus_PacketHasFec(const uint8_t* payload,
                            size_t payload_length_bytes);

/****************************************************************************
 * WebRtcOpus_PacketHasVoiceActivity(...)
 *
 * This function returns the SILK VAD information encoded in the opus packet.
 * For CELT-only packets that do not have VAD information, it returns -1.
 * Input:
 *        - payload              : Encoded data pointer
 *        - payload_length_bytes : Bytes of encoded data
 *
 * Return value                  : 0 - no frame had the VAD flag set.
 *                                 1 - at least one frame had the VAD flag set.
 *                                -1 - VAD status could not be determined.
 */
int WebRtcOpus_PacketHasVoiceActivity(const uint8_t* payload,
                                      size_t payload_length_bytes);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // MODULES_AUDIO_CODING_CODECS_OPUS_OPUS_INTERFACE_H_
