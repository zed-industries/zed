/* Simple Plugin API
 *
 * Copyright © 2020 Wim Taymans
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

#ifndef SPA_BLUETOOTH_AUDIO_H
#define SPA_BLUETOOTH_AUDIO_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */
enum spa_bluetooth_audio_codec {
	SPA_BLUETOOTH_AUDIO_CODEC_START,

	/* A2DP */
	SPA_BLUETOOTH_AUDIO_CODEC_SBC,
	SPA_BLUETOOTH_AUDIO_CODEC_SBC_XQ,
	SPA_BLUETOOTH_AUDIO_CODEC_MPEG,
	SPA_BLUETOOTH_AUDIO_CODEC_AAC,
	SPA_BLUETOOTH_AUDIO_CODEC_APTX,
	SPA_BLUETOOTH_AUDIO_CODEC_APTX_HD,
	SPA_BLUETOOTH_AUDIO_CODEC_LDAC,
	SPA_BLUETOOTH_AUDIO_CODEC_APTX_LL,
	SPA_BLUETOOTH_AUDIO_CODEC_APTX_LL_DUPLEX,
	SPA_BLUETOOTH_AUDIO_CODEC_FASTSTREAM,
	SPA_BLUETOOTH_AUDIO_CODEC_FASTSTREAM_DUPLEX,
	SPA_BLUETOOTH_AUDIO_CODEC_LC3PLUS_HR,

	/* HFP */
	SPA_BLUETOOTH_AUDIO_CODEC_CVSD = 0x100,
	SPA_BLUETOOTH_AUDIO_CODEC_MSBC,
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_BLUETOOTH_AUDIO_H */
