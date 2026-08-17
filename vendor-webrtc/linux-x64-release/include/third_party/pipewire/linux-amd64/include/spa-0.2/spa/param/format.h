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

#ifndef SPA_PARAM_FORMAT_H
#define SPA_PARAM_FORMAT_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */

#include <spa/param/param.h>

/** media type for SPA_TYPE_OBJECT_Format */
enum spa_media_type {
	SPA_MEDIA_TYPE_unknown,
	SPA_MEDIA_TYPE_audio,
	SPA_MEDIA_TYPE_video,
	SPA_MEDIA_TYPE_image,
	SPA_MEDIA_TYPE_binary,
	SPA_MEDIA_TYPE_stream,
	SPA_MEDIA_TYPE_application,
};

/** media subtype for SPA_TYPE_OBJECT_Format */
enum spa_media_subtype {
	SPA_MEDIA_SUBTYPE_unknown,
	SPA_MEDIA_SUBTYPE_raw,
	SPA_MEDIA_SUBTYPE_dsp,
	SPA_MEDIA_SUBTYPE_iec958,	/** S/PDIF */
	SPA_MEDIA_SUBTYPE_dsd,

	SPA_MEDIA_SUBTYPE_START_Audio	= 0x10000,
	SPA_MEDIA_SUBTYPE_mp3,
	SPA_MEDIA_SUBTYPE_aac,
	SPA_MEDIA_SUBTYPE_vorbis,
	SPA_MEDIA_SUBTYPE_wma,
	SPA_MEDIA_SUBTYPE_ra,
	SPA_MEDIA_SUBTYPE_sbc,
	SPA_MEDIA_SUBTYPE_adpcm,
	SPA_MEDIA_SUBTYPE_g723,
	SPA_MEDIA_SUBTYPE_g726,
	SPA_MEDIA_SUBTYPE_g729,
	SPA_MEDIA_SUBTYPE_amr,
	SPA_MEDIA_SUBTYPE_gsm,

	SPA_MEDIA_SUBTYPE_START_Video	= 0x20000,
	SPA_MEDIA_SUBTYPE_h264,
	SPA_MEDIA_SUBTYPE_mjpg,
	SPA_MEDIA_SUBTYPE_dv,
	SPA_MEDIA_SUBTYPE_mpegts,
	SPA_MEDIA_SUBTYPE_h263,
	SPA_MEDIA_SUBTYPE_mpeg1,
	SPA_MEDIA_SUBTYPE_mpeg2,
	SPA_MEDIA_SUBTYPE_mpeg4,
	SPA_MEDIA_SUBTYPE_xvid,
	SPA_MEDIA_SUBTYPE_vc1,
	SPA_MEDIA_SUBTYPE_vp8,
	SPA_MEDIA_SUBTYPE_vp9,
	SPA_MEDIA_SUBTYPE_bayer,

	SPA_MEDIA_SUBTYPE_START_Image	= 0x30000,
	SPA_MEDIA_SUBTYPE_jpeg,

	SPA_MEDIA_SUBTYPE_START_Binary	= 0x40000,

	SPA_MEDIA_SUBTYPE_START_Stream	= 0x50000,
	SPA_MEDIA_SUBTYPE_midi,

	SPA_MEDIA_SUBTYPE_START_Application	= 0x60000,
	SPA_MEDIA_SUBTYPE_control,		/**< control stream, data contains
						  *  spa_pod_sequence with control info. */
};

/** properties for audio SPA_TYPE_OBJECT_Format */
enum spa_format {
	SPA_FORMAT_START,

	SPA_FORMAT_mediaType,		/**< media type (Id enum spa_media_type) */
	SPA_FORMAT_mediaSubtype,	/**< media subtype (Id enum spa_media_subtype) */

	/* Audio format keys */
	SPA_FORMAT_START_Audio = 0x10000,
	SPA_FORMAT_AUDIO_format,	/**< audio format, (Id enum spa_audio_format) */
	SPA_FORMAT_AUDIO_flags,		/**< optional flags (Int) */
	SPA_FORMAT_AUDIO_rate,		/**< sample rate (Int) */
	SPA_FORMAT_AUDIO_channels,	/**< number of audio channels (Int) */
	SPA_FORMAT_AUDIO_position,	/**< channel positions (Id enum spa_audio_position) */

	SPA_FORMAT_AUDIO_iec958Codec,	/**< codec used (IEC958) (Id enum spa_audio_iec958_codec) */

	SPA_FORMAT_AUDIO_bitorder,	/**< bit order (Id enum spa_param_bitorder) */
	SPA_FORMAT_AUDIO_interleave,	/**< Interleave bytes (Int) */

	/* Video Format keys */
	SPA_FORMAT_START_Video = 0x20000,
	SPA_FORMAT_VIDEO_format,		/**< video format (Id enum spa_video_format) */
	SPA_FORMAT_VIDEO_modifier,		/**< format modifier (Long)
						  * use only with DMA-BUF and omit for other buffer types */
	SPA_FORMAT_VIDEO_size,			/**< size (Rectangle) */
	SPA_FORMAT_VIDEO_framerate,		/**< frame rate (Fraction) */
	SPA_FORMAT_VIDEO_maxFramerate,		/**< maximum frame rate (Fraction) */
	SPA_FORMAT_VIDEO_views,			/**< number of views (Int) */
	SPA_FORMAT_VIDEO_interlaceMode,		/**< (Id enum spa_video_interlace_mode) */
	SPA_FORMAT_VIDEO_pixelAspectRatio,	/**< (Rectangle) */
	SPA_FORMAT_VIDEO_multiviewMode,		/**< (Id enum spa_video_multiview_mode) */
	SPA_FORMAT_VIDEO_multiviewFlags,	/**< (Id enum spa_video_multiview_flags) */
	SPA_FORMAT_VIDEO_chromaSite,		/**< /Id enum spa_video_chroma_site) */
	SPA_FORMAT_VIDEO_colorRange,		/**< /Id enum spa_video_color_range) */
	SPA_FORMAT_VIDEO_colorMatrix,		/**< /Id enum spa_video_color_matrix) */
	SPA_FORMAT_VIDEO_transferFunction,	/**< /Id enum spa_video_transfer_function) */
	SPA_FORMAT_VIDEO_colorPrimaries,	/**< /Id enum spa_video_color_primaries) */
	SPA_FORMAT_VIDEO_profile,		/**< (Int) */
	SPA_FORMAT_VIDEO_level,			/**< (Int) */
	SPA_FORMAT_VIDEO_H264_streamFormat,	/**< (Id enum spa_h264_stream_format) */
	SPA_FORMAT_VIDEO_H264_alignment,	/**< (Id enum spa_h264_alignment) */

	/* Image Format keys */
	SPA_FORMAT_START_Image = 0x30000,
	/* Binary Format keys */
	SPA_FORMAT_START_Binary = 0x40000,
	/* Stream Format keys */
	SPA_FORMAT_START_Stream = 0x50000,
	/* Application Format keys */
	SPA_FORMAT_START_Application = 0x60000,
};

#define SPA_KEY_FORMAT_DSP		"format.dsp"		/**< a predefined DSP format,
								  *  Ex. "32 bit float mono audio" */

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_PARAM_FORMAT_H */
