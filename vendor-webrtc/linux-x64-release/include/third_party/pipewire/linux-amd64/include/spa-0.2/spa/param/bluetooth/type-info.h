/* Simple Plugin API
 *
 * Copyright © 2018 Wim Taymans
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

#ifndef SPA_BLUETOOTH_TYPES_H
#define SPA_BLUETOOTH_TYPES_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */

#include <spa/param/bluetooth/audio.h>

#define SPA_TYPE_INFO_BluetoothAudioCodec		SPA_TYPE_INFO_ENUM_BASE "BluetoothAudioCodec"
#define SPA_TYPE_INFO_BLUETOOTH_AUDIO_CODEC_BASE	SPA_TYPE_INFO_BluetoothAudioCodec ":"

static const struct spa_type_info spa_type_bluetooth_audio_codec[] = {
	/* A2DP */
	{ SPA_BLUETOOTH_AUDIO_CODEC_SBC, SPA_TYPE_Int, SPA_TYPE_INFO_BLUETOOTH_AUDIO_CODEC_BASE "sbc", NULL },
	{ SPA_BLUETOOTH_AUDIO_CODEC_SBC_XQ, SPA_TYPE_Int, SPA_TYPE_INFO_BLUETOOTH_AUDIO_CODEC_BASE "sbc_xq", NULL },
	{ SPA_BLUETOOTH_AUDIO_CODEC_MPEG, SPA_TYPE_Int, SPA_TYPE_INFO_BLUETOOTH_AUDIO_CODEC_BASE "mpeg", NULL },
	{ SPA_BLUETOOTH_AUDIO_CODEC_AAC, SPA_TYPE_Int, SPA_TYPE_INFO_BLUETOOTH_AUDIO_CODEC_BASE "aac", NULL },
	{ SPA_BLUETOOTH_AUDIO_CODEC_APTX, SPA_TYPE_Int, SPA_TYPE_INFO_BLUETOOTH_AUDIO_CODEC_BASE "aptx", NULL },
	{ SPA_BLUETOOTH_AUDIO_CODEC_APTX_HD, SPA_TYPE_Int, SPA_TYPE_INFO_BLUETOOTH_AUDIO_CODEC_BASE "aptx_hd", NULL },
	{ SPA_BLUETOOTH_AUDIO_CODEC_LDAC, SPA_TYPE_Int, SPA_TYPE_INFO_BLUETOOTH_AUDIO_CODEC_BASE "ldac", NULL },
	{ SPA_BLUETOOTH_AUDIO_CODEC_APTX_LL, SPA_TYPE_Int, SPA_TYPE_INFO_BLUETOOTH_AUDIO_CODEC_BASE "aptx_ll", NULL },
	{ SPA_BLUETOOTH_AUDIO_CODEC_APTX_LL_DUPLEX, SPA_TYPE_Int, SPA_TYPE_INFO_BLUETOOTH_AUDIO_CODEC_BASE "aptx_ll_duplex", NULL },
	{ SPA_BLUETOOTH_AUDIO_CODEC_FASTSTREAM, SPA_TYPE_Int, SPA_TYPE_INFO_BLUETOOTH_AUDIO_CODEC_BASE "faststream", NULL },
	{ SPA_BLUETOOTH_AUDIO_CODEC_FASTSTREAM_DUPLEX, SPA_TYPE_Int, SPA_TYPE_INFO_BLUETOOTH_AUDIO_CODEC_BASE "faststream_duplex", NULL },
	{ SPA_BLUETOOTH_AUDIO_CODEC_LC3PLUS_HR, SPA_TYPE_Int, SPA_TYPE_INFO_BLUETOOTH_AUDIO_CODEC_BASE "lc3plus_hr", NULL },

	{ SPA_BLUETOOTH_AUDIO_CODEC_CVSD, SPA_TYPE_Int, SPA_TYPE_INFO_BLUETOOTH_AUDIO_CODEC_BASE "cvsd", NULL },
	{ SPA_BLUETOOTH_AUDIO_CODEC_MSBC, SPA_TYPE_Int, SPA_TYPE_INFO_BLUETOOTH_AUDIO_CODEC_BASE "msbc", NULL },

	{ 0, 0, NULL, NULL },
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_BLUETOOTH_TYPES_H */
