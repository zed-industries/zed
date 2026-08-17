/* Copyright (c) 2018 Mozilla */
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE FOUNDATION OR
   CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

#ifndef LPCNET_H_
#define LPCNET_H_

#include "opus_types.h"

#define NB_FEATURES 20
#define NB_TOTAL_FEATURES 36

/** Number of audio samples in a feature frame (not for encoding/decoding). */
#define LPCNET_FRAME_SIZE (160)

typedef struct LPCNetState LPCNetState;

typedef struct LPCNetDecState LPCNetDecState;

typedef struct LPCNetEncState LPCNetEncState;

typedef struct LPCNetPLCState LPCNetPLCState;


/** Gets the size of an <code>LPCNetDecState</code> structure.
  * @returns The size in bytes.
  */
int lpcnet_decoder_get_size(void);

/** Initializes a previously allocated decoder state
  * The memory pointed to by st must be at least the size returned by lpcnet_decoder_get_size().
  * This is intended for applications which use their own allocator instead of malloc.
  * @see lpcnet_decoder_create(),lpcnet_decoder_get_size()
  * @param [in] st <tt>LPCNetDecState*</tt>: Decoder state
  * @retval 0 Success
  */
int lpcnet_decoder_init(LPCNetDecState *st);

void lpcnet_reset(LPCNetState *lpcnet);

/** Allocates and initializes a decoder state.
  *  @returns The newly created state
  */
LPCNetDecState *lpcnet_decoder_create(void);

/** Frees an <code>LPCNetDecState</code> allocated by lpcnet_decoder_create().
  * @param[in] st <tt>LPCNetDecState*</tt>: State to be freed.
  */
void lpcnet_decoder_destroy(LPCNetDecState *st);

/** Decodes a packet of LPCNET_COMPRESSED_SIZE bytes (currently 8) into LPCNET_PACKET_SAMPLES samples (currently 640).
  * @param [in] st <tt>LPCNetDecState*</tt>: Decoder state
  * @param [in] buf <tt>const unsigned char *</tt>: Compressed packet
  * @param [out] pcm <tt>opus_int16 *</tt>: Decoded audio
  * @retval 0 Success
  */
int lpcnet_decode(LPCNetDecState *st, const unsigned char *buf, opus_int16 *pcm);



/** Gets the size of an <code>LPCNetEncState</code> structure.
  * @returns The size in bytes.
  */
int lpcnet_encoder_get_size(void);

/** Initializes a previously allocated encoder state
  * The memory pointed to by st must be at least the size returned by lpcnet_encoder_get_size().
  * This is intended for applications which use their own allocator instead of malloc.
  * @see lpcnet_encoder_create(),lpcnet_encoder_get_size()
  * @param [in] st <tt>LPCNetEncState*</tt>: Encoder state
  * @retval 0 Success
  */
int lpcnet_encoder_init(LPCNetEncState *st);

int lpcnet_encoder_load_model(LPCNetEncState *st, const void *data, int len);

/** Allocates and initializes an encoder state.
  *  @returns The newly created state
  */
LPCNetEncState *lpcnet_encoder_create(void);

/** Frees an <code>LPCNetEncState</code> allocated by lpcnet_encoder_create().
  * @param[in] st <tt>LPCNetEncState*</tt>: State to be freed.
  */
void lpcnet_encoder_destroy(LPCNetEncState *st);

/** Encodes LPCNET_PACKET_SAMPLES speech samples (currently 640) into a packet of LPCNET_COMPRESSED_SIZE bytes (currently 8).
  * @param [in] st <tt>LPCNetDecState*</tt>: Encoder state
  * @param [in] pcm <tt>opus_int16 *</tt>: Input speech to be encoded
  * @param [out] buf <tt>const unsigned char *</tt>: Compressed packet
  * @retval 0 Success
  */
int lpcnet_encode(LPCNetEncState *st, const opus_int16 *pcm, unsigned char *buf);

/** Compute features on LPCNET_FRAME_SIZE speech samples (currently 160) and output features for one 10-ms frame.
  * @param [in] st <tt>LPCNetDecState*</tt>: Encoder state
  * @param [in] pcm <tt>opus_int16 *</tt>: Input speech to be analyzed
  * @param [out] features <tt>float[NB_TOTAL_FEATURES]</tt>: Four feature vectors
  * @retval 0 Success
  */
int lpcnet_compute_single_frame_features(LPCNetEncState *st, const opus_int16 *pcm, float features[NB_TOTAL_FEATURES], int arch);


/** Compute features on LPCNET_FRAME_SIZE speech samples (currently 160) and output features for one 10-ms frame.
  * @param [in] st <tt>LPCNetDecState*</tt>: Encoder state
  * @param [in] pcm <tt>float *</tt>: Input speech to be analyzed
  * @param [out] features <tt>float[NB_TOTAL_FEATURES]</tt>: Four feature vectors
  * @retval 0 Success
  */
int lpcnet_compute_single_frame_features_float(LPCNetEncState *st, const float *pcm, float features[NB_TOTAL_FEATURES], int arch);

/** Gets the size of an <code>LPCNetState</code> structure.
  * @returns The size in bytes.
  */
int lpcnet_get_size(void);

/** Initializes a previously allocated synthesis state
  * The memory pointed to by st must be at least the size returned by lpcnet_get_size().
  * This is intended for applications which use their own allocator instead of malloc.
  * @see lpcnet_create(),lpcnet_get_size()
  * @param [in] st <tt>LPCNetState*</tt>: Synthesis state
  * @retval 0 Success
  */
int lpcnet_init(LPCNetState *st);

/** Allocates and initializes a synthesis state.
  *  @returns The newly created state
  */
LPCNetState *lpcnet_create(void);

/** Frees an <code>LPCNetState</code> allocated by lpcnet_create().
  * @param[in] st <tt>LPCNetState*</tt>: State to be freed.
  */
void lpcnet_destroy(LPCNetState *st);

/** Synthesizes speech from an LPCNet feature vector.
  * @param [in] st <tt>LPCNetState*</tt>: Synthesis state
  * @param [in] features <tt>const float *</tt>: Compressed packet
  * @param [out] output <tt>opus_int16 **</tt>: Synthesized speech
  * @param [in] N <tt>int</tt>: Number of samples to generate
  * @retval 0 Success
  */
void lpcnet_synthesize(LPCNetState *st, const float *features, opus_int16 *output, int N);



int lpcnet_plc_init(LPCNetPLCState *st);
void lpcnet_plc_reset(LPCNetPLCState *st);

int lpcnet_plc_update(LPCNetPLCState *st, opus_int16 *pcm);

int lpcnet_plc_conceal(LPCNetPLCState *st, opus_int16 *pcm);

void lpcnet_plc_fec_add(LPCNetPLCState *st, const float *features);

void lpcnet_plc_fec_clear(LPCNetPLCState *st);

int lpcnet_load_model(LPCNetState *st, const void *data, int len);
int lpcnet_plc_load_model(LPCNetPLCState *st, const void *data, int len);

#endif
