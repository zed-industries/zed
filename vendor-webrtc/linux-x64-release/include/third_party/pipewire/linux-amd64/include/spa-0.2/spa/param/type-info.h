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

#ifndef SPA_PARAM_TYPES_H
#define SPA_PARAM_TYPES_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */

#include <spa/utils/defs.h>
#include <spa/param/props.h>
#include <spa/param/format.h>
#include <spa/buffer/type-info.h>

/* base for parameter object enumerations */
#define SPA_TYPE_INFO_ParamId		SPA_TYPE_INFO_ENUM_BASE "ParamId"
#define SPA_TYPE_INFO_PARAM_ID_BASE	SPA_TYPE_INFO_ParamId ":"

static const struct spa_type_info spa_type_param[] = {
	{ SPA_PARAM_Invalid, SPA_TYPE_None, SPA_TYPE_INFO_PARAM_ID_BASE "Invalid", NULL },
	{ SPA_PARAM_PropInfo, SPA_TYPE_OBJECT_PropInfo, SPA_TYPE_INFO_PARAM_ID_BASE "PropInfo", NULL },
	{ SPA_PARAM_Props, SPA_TYPE_OBJECT_Props, SPA_TYPE_INFO_PARAM_ID_BASE "Props", NULL },
	{ SPA_PARAM_EnumFormat, SPA_TYPE_OBJECT_Format, SPA_TYPE_INFO_PARAM_ID_BASE "EnumFormat", NULL },
	{ SPA_PARAM_Format, SPA_TYPE_OBJECT_Format, SPA_TYPE_INFO_PARAM_ID_BASE "Format", NULL },
	{ SPA_PARAM_Buffers, SPA_TYPE_OBJECT_ParamBuffers, SPA_TYPE_INFO_PARAM_ID_BASE "Buffers", NULL },
	{ SPA_PARAM_Meta, SPA_TYPE_OBJECT_ParamMeta, SPA_TYPE_INFO_PARAM_ID_BASE "Meta", NULL },
	{ SPA_PARAM_IO, SPA_TYPE_OBJECT_ParamIO, SPA_TYPE_INFO_PARAM_ID_BASE "IO", NULL },
	{ SPA_PARAM_EnumProfile, SPA_TYPE_OBJECT_ParamProfile, SPA_TYPE_INFO_PARAM_ID_BASE "EnumProfile", NULL },
	{ SPA_PARAM_Profile, SPA_TYPE_OBJECT_ParamProfile, SPA_TYPE_INFO_PARAM_ID_BASE "Profile", NULL },
	{ SPA_PARAM_EnumPortConfig, SPA_TYPE_OBJECT_ParamPortConfig, SPA_TYPE_INFO_PARAM_ID_BASE "EnumPortConfig", NULL },
	{ SPA_PARAM_PortConfig, SPA_TYPE_OBJECT_ParamPortConfig, SPA_TYPE_INFO_PARAM_ID_BASE "PortConfig", NULL },
	{ SPA_PARAM_EnumRoute, SPA_TYPE_OBJECT_ParamRoute, SPA_TYPE_INFO_PARAM_ID_BASE "EnumRoute", NULL },
	{ SPA_PARAM_Route, SPA_TYPE_OBJECT_ParamRoute, SPA_TYPE_INFO_PARAM_ID_BASE "Route", NULL },
	{ SPA_PARAM_Control, SPA_TYPE_Sequence, SPA_TYPE_INFO_PARAM_ID_BASE "Control", NULL },
	{ SPA_PARAM_Latency, SPA_TYPE_OBJECT_ParamLatency, SPA_TYPE_INFO_PARAM_ID_BASE "Latency", NULL },
	{ SPA_PARAM_ProcessLatency, SPA_TYPE_OBJECT_ParamProcessLatency, SPA_TYPE_INFO_PARAM_ID_BASE "ProcessLatency", NULL },
	{ 0, 0, NULL, NULL },
};

/* base for parameter objects */
#define SPA_TYPE_INFO_Param			SPA_TYPE_INFO_OBJECT_BASE "Param"
#define SPA_TYPE_INFO_PARAM_BASE		SPA_TYPE_INFO_Param ":"

#define SPA_TYPE_INFO_Props			SPA_TYPE_INFO_PARAM_BASE "Props"
#define SPA_TYPE_INFO_PROPS_BASE		SPA_TYPE_INFO_Props ":"

#include <spa/param/audio/type-info.h>
#include <spa/param/video/type-info.h>
#include <spa/param/bluetooth/type-info.h>

static const struct spa_type_info spa_type_prop_float_array[] = {
	{ SPA_PROP_START, SPA_TYPE_Float, SPA_TYPE_INFO_BASE "floatArray", NULL, },
	{ 0, 0, NULL, NULL },
};

static const struct spa_type_info spa_type_prop_channel_map[] = {
	{ SPA_PROP_START, SPA_TYPE_Id, SPA_TYPE_INFO_BASE "channelMap", spa_type_audio_channel, },
	{ 0, 0, NULL, NULL },
};

static const struct spa_type_info spa_type_prop_iec958_codec[] = {
	{ SPA_PROP_START, SPA_TYPE_Id, SPA_TYPE_INFO_BASE "iec958Codec", spa_type_audio_iec958_codec, },
	{ 0, 0, NULL, NULL },
};

#define SPA_TYPE_INFO_ParamBitorder		SPA_TYPE_INFO_ENUM_BASE "ParamBitorder"
#define SPA_TYPE_INFO_PARAM_BITORDER_BASE	SPA_TYPE_INFO_ParamBitorder ":"

static const struct spa_type_info spa_type_param_bitorder[] = {
	{ SPA_PARAM_BITORDER_unknown, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_BITORDER_BASE "unknown", NULL },
	{ SPA_PARAM_BITORDER_msb, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_BITORDER_BASE "msb", NULL },
	{ SPA_PARAM_BITORDER_lsb, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_BITORDER_BASE "lsb", NULL },
	{ 0, 0, NULL, NULL },
};

static const struct spa_type_info spa_type_props[] = {
	{ SPA_PROP_START, SPA_TYPE_Id, SPA_TYPE_INFO_PROPS_BASE, spa_type_param, },
	{ SPA_PROP_unknown, SPA_TYPE_None, SPA_TYPE_INFO_PROPS_BASE "unknown", NULL },
	{ SPA_PROP_device, SPA_TYPE_String, SPA_TYPE_INFO_PROPS_BASE "device", NULL },
	{ SPA_PROP_deviceName, SPA_TYPE_String, SPA_TYPE_INFO_PROPS_BASE "deviceName", NULL },
	{ SPA_PROP_deviceFd, SPA_TYPE_Fd, SPA_TYPE_INFO_PROPS_BASE "deviceFd", NULL },
	{ SPA_PROP_card, SPA_TYPE_String, SPA_TYPE_INFO_PROPS_BASE "card", NULL },
	{ SPA_PROP_cardName, SPA_TYPE_String, SPA_TYPE_INFO_PROPS_BASE "cardName", NULL },
	{ SPA_PROP_minLatency, SPA_TYPE_Int, SPA_TYPE_INFO_PROPS_BASE "minLatency", NULL },
	{ SPA_PROP_maxLatency, SPA_TYPE_Int, SPA_TYPE_INFO_PROPS_BASE "maxLatency", NULL },
	{ SPA_PROP_periods, SPA_TYPE_Int, SPA_TYPE_INFO_PROPS_BASE "periods", NULL },
	{ SPA_PROP_periodSize, SPA_TYPE_Int, SPA_TYPE_INFO_PROPS_BASE "periodSize", NULL },
	{ SPA_PROP_periodEvent, SPA_TYPE_Bool, SPA_TYPE_INFO_PROPS_BASE "periodEvent", NULL },
	{ SPA_PROP_live, SPA_TYPE_Bool, SPA_TYPE_INFO_PROPS_BASE "live", NULL },
	{ SPA_PROP_rate, SPA_TYPE_Double, SPA_TYPE_INFO_PROPS_BASE "rate", NULL },
	{ SPA_PROP_quality, SPA_TYPE_Int, SPA_TYPE_INFO_PROPS_BASE "quality", NULL },
	{ SPA_PROP_bluetoothAudioCodec, SPA_TYPE_Id, SPA_TYPE_INFO_PROPS_BASE "bluetoothAudioCodec", spa_type_bluetooth_audio_codec },

	{ SPA_PROP_waveType, SPA_TYPE_Id, SPA_TYPE_INFO_PROPS_BASE "waveType", NULL },
	{ SPA_PROP_frequency, SPA_TYPE_Int, SPA_TYPE_INFO_PROPS_BASE "frequency", NULL },
	{ SPA_PROP_volume, SPA_TYPE_Float, SPA_TYPE_INFO_PROPS_BASE "volume", NULL },
	{ SPA_PROP_mute, SPA_TYPE_Bool, SPA_TYPE_INFO_PROPS_BASE "mute", NULL },
	{ SPA_PROP_patternType, SPA_TYPE_Id, SPA_TYPE_INFO_PROPS_BASE "patternType", NULL },
	{ SPA_PROP_ditherType, SPA_TYPE_Id, SPA_TYPE_INFO_PROPS_BASE "ditherType", NULL },
	{ SPA_PROP_truncate, SPA_TYPE_Bool, SPA_TYPE_INFO_PROPS_BASE "truncate", NULL },
	{ SPA_PROP_channelVolumes, SPA_TYPE_Array, SPA_TYPE_INFO_PROPS_BASE "channelVolumes", spa_type_prop_float_array },
	{ SPA_PROP_volumeBase, SPA_TYPE_Float, SPA_TYPE_INFO_PROPS_BASE "volumeBase", NULL },
	{ SPA_PROP_volumeStep, SPA_TYPE_Float, SPA_TYPE_INFO_PROPS_BASE "volumeStep", NULL },
	{ SPA_PROP_channelMap, SPA_TYPE_Array, SPA_TYPE_INFO_PROPS_BASE "channelMap", spa_type_prop_channel_map },
	{ SPA_PROP_monitorMute, SPA_TYPE_Bool, SPA_TYPE_INFO_PROPS_BASE "monitorMute", NULL },
	{ SPA_PROP_monitorVolumes, SPA_TYPE_Array, SPA_TYPE_INFO_PROPS_BASE "monitorVolumes", spa_type_prop_float_array },
	{ SPA_PROP_latencyOffsetNsec, SPA_TYPE_Long, SPA_TYPE_INFO_PROPS_BASE "latencyOffsetNsec", NULL },
	{ SPA_PROP_softMute, SPA_TYPE_Bool, SPA_TYPE_INFO_PROPS_BASE "softMute", NULL },
	{ SPA_PROP_softVolumes, SPA_TYPE_Array, SPA_TYPE_INFO_PROPS_BASE "softVolumes", spa_type_prop_float_array },
	{ SPA_PROP_iec958Codecs, SPA_TYPE_Array, SPA_TYPE_INFO_PROPS_BASE "iec958Codecs", spa_type_prop_iec958_codec },

	{ SPA_PROP_brightness, SPA_TYPE_Int, SPA_TYPE_INFO_PROPS_BASE "brightness", NULL },
	{ SPA_PROP_contrast, SPA_TYPE_Int, SPA_TYPE_INFO_PROPS_BASE "contrast", NULL },
	{ SPA_PROP_saturation, SPA_TYPE_Int, SPA_TYPE_INFO_PROPS_BASE "saturation", NULL },
	{ SPA_PROP_hue, SPA_TYPE_Int, SPA_TYPE_INFO_PROPS_BASE "hue", NULL },
	{ SPA_PROP_gamma, SPA_TYPE_Int, SPA_TYPE_INFO_PROPS_BASE "gamma", NULL },
	{ SPA_PROP_exposure, SPA_TYPE_Int, SPA_TYPE_INFO_PROPS_BASE "exposure", NULL },
	{ SPA_PROP_gain, SPA_TYPE_Int, SPA_TYPE_INFO_PROPS_BASE "gain", NULL },
	{ SPA_PROP_sharpness, SPA_TYPE_Int, SPA_TYPE_INFO_PROPS_BASE "sharpness", NULL },

	{ SPA_PROP_params, SPA_TYPE_Struct, SPA_TYPE_INFO_PROPS_BASE "params", NULL },
	{ 0, 0, NULL, NULL },
};

/** Enum Property info */
#define SPA_TYPE_INFO_PropInfo			SPA_TYPE_INFO_PARAM_BASE "PropInfo"
#define SPA_TYPE_INFO_PROP_INFO_BASE		SPA_TYPE_INFO_PropInfo ":"

static const struct spa_type_info spa_type_prop_info[] = {
	{ SPA_PROP_INFO_START, SPA_TYPE_Id, SPA_TYPE_INFO_PROP_INFO_BASE, spa_type_param, },
	{ SPA_PROP_INFO_id, SPA_TYPE_Id, SPA_TYPE_INFO_PROP_INFO_BASE "id", spa_type_props },
	{ SPA_PROP_INFO_name, SPA_TYPE_String, SPA_TYPE_INFO_PROP_INFO_BASE "name", NULL },
	{ SPA_PROP_INFO_type, SPA_TYPE_Pod, SPA_TYPE_INFO_PROP_INFO_BASE "type", NULL },
	{ SPA_PROP_INFO_labels, SPA_TYPE_Struct, SPA_TYPE_INFO_PROP_INFO_BASE "labels", NULL },
	{ SPA_PROP_INFO_container, SPA_TYPE_Id, SPA_TYPE_INFO_PROP_INFO_BASE "container", NULL },
	{ SPA_PROP_INFO_params, SPA_TYPE_Bool, SPA_TYPE_INFO_PROP_INFO_BASE "params", NULL },
	{ SPA_PROP_INFO_description, SPA_TYPE_String, SPA_TYPE_INFO_PROP_INFO_BASE "description", NULL },
	{ 0, 0, NULL, NULL },
};

#define SPA_TYPE_INFO_PARAM_Meta		SPA_TYPE_INFO_PARAM_BASE "Meta"
#define SPA_TYPE_INFO_PARAM_META_BASE		SPA_TYPE_INFO_PARAM_Meta ":"

static const struct spa_type_info spa_type_param_meta[] = {
	{ SPA_PARAM_META_START, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_META_BASE, spa_type_param },
	{ SPA_PARAM_META_type, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_META_BASE "type", spa_type_meta_type },
	{ SPA_PARAM_META_size, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_META_BASE "size", NULL },
	{ 0, 0, NULL, NULL },
};

/** Base for parameters that describe IO areas to exchange data,
 * control and properties with a node.
 */
#define SPA_TYPE_INFO_PARAM_IO		SPA_TYPE_INFO_PARAM_BASE "IO"
#define SPA_TYPE_INFO_PARAM_IO_BASE		SPA_TYPE_INFO_PARAM_IO ":"

static const struct spa_type_info spa_type_param_io[] = {
	{ SPA_PARAM_IO_START, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_IO_BASE, spa_type_param, },
	{ SPA_PARAM_IO_id, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_IO_BASE "id", spa_type_io },
	{ SPA_PARAM_IO_size, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_IO_BASE "size", NULL },
	{ 0, 0, NULL, NULL },
};

#define SPA_TYPE_INFO_Format			SPA_TYPE_INFO_PARAM_BASE "Format"
#define SPA_TYPE_INFO_FORMAT_BASE		SPA_TYPE_INFO_Format ":"

#define SPA_TYPE_INFO_MediaType		SPA_TYPE_INFO_ENUM_BASE "MediaType"
#define SPA_TYPE_INFO_MEDIA_TYPE_BASE	SPA_TYPE_INFO_MediaType ":"

static const struct spa_type_info spa_type_media_type[] = {
	{ SPA_MEDIA_TYPE_unknown, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_TYPE_BASE "unknown", NULL },
	{ SPA_MEDIA_TYPE_audio,   SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_TYPE_BASE "audio", NULL },
	{ SPA_MEDIA_TYPE_video,   SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_TYPE_BASE "video", NULL },
	{ SPA_MEDIA_TYPE_image,   SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_TYPE_BASE "image", NULL },
	{ SPA_MEDIA_TYPE_binary,  SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_TYPE_BASE "binary", NULL },
	{ SPA_MEDIA_TYPE_stream,  SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_TYPE_BASE "stream", NULL },
	{ SPA_MEDIA_TYPE_application,  SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_TYPE_BASE "application", NULL },
	{ 0, 0, NULL, NULL },
};

#define SPA_TYPE_INFO_MediaSubtype		SPA_TYPE_INFO_ENUM_BASE "MediaSubtype"
#define SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE	SPA_TYPE_INFO_MediaSubtype ":"

static const struct spa_type_info spa_type_media_subtype[] = {
	{ SPA_MEDIA_SUBTYPE_unknown, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "unknown", NULL },
	/* generic subtypes */
	{ SPA_MEDIA_SUBTYPE_raw, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "raw", NULL },
	{ SPA_MEDIA_SUBTYPE_dsp, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "dsp", NULL },
	{ SPA_MEDIA_SUBTYPE_iec958, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "iec958", NULL },
	{ SPA_MEDIA_SUBTYPE_dsd, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "dsd", NULL },
	/* audio subtypes */
	{ SPA_MEDIA_SUBTYPE_mp3, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "mp3", NULL },
	{ SPA_MEDIA_SUBTYPE_aac, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "aac", NULL },
	{ SPA_MEDIA_SUBTYPE_vorbis, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "vorbis", NULL },
	{ SPA_MEDIA_SUBTYPE_wma, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "wma", NULL },
	{ SPA_MEDIA_SUBTYPE_ra, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "ra", NULL },
	{ SPA_MEDIA_SUBTYPE_sbc, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "sbc", NULL },
	{ SPA_MEDIA_SUBTYPE_adpcm, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "adpcm", NULL },
	{ SPA_MEDIA_SUBTYPE_g723, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "g723", NULL },
	{ SPA_MEDIA_SUBTYPE_g726, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "g726", NULL },
	{ SPA_MEDIA_SUBTYPE_g729, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "g729", NULL },
	{ SPA_MEDIA_SUBTYPE_amr, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "amr", NULL },
	{ SPA_MEDIA_SUBTYPE_gsm, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "gsm", NULL },
	/* video subtypes */
	{ SPA_MEDIA_SUBTYPE_h264, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "h264", NULL },
	{ SPA_MEDIA_SUBTYPE_mjpg, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "mjpg", NULL },
	{ SPA_MEDIA_SUBTYPE_dv, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "dv", NULL },
	{ SPA_MEDIA_SUBTYPE_mpegts, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "mpegts", NULL },
	{ SPA_MEDIA_SUBTYPE_h263, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "h263", NULL },
	{ SPA_MEDIA_SUBTYPE_mpeg1, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "mpeg1", NULL },
	{ SPA_MEDIA_SUBTYPE_mpeg2, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "mpeg2", NULL },
	{ SPA_MEDIA_SUBTYPE_mpeg4, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "mpeg4", NULL },
	{ SPA_MEDIA_SUBTYPE_xvid, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "xvid", NULL },
	{ SPA_MEDIA_SUBTYPE_vc1, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "vc1", NULL },
	{ SPA_MEDIA_SUBTYPE_vp8, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "vp8", NULL },
	{ SPA_MEDIA_SUBTYPE_vp9, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "vp9", NULL },
	{ SPA_MEDIA_SUBTYPE_bayer, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "bayer", NULL },
	/* image subtypes */
	{ SPA_MEDIA_SUBTYPE_jpeg, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "jpeg", NULL },
	/* stream subtypes */
	{ SPA_MEDIA_SUBTYPE_midi, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "midi", NULL },
	/* application subtypes */
	{ SPA_MEDIA_SUBTYPE_control, SPA_TYPE_Int, SPA_TYPE_INFO_MEDIA_SUBTYPE_BASE "control", NULL },
	{ 0, 0, NULL, NULL },
};

#define SPA_TYPE_INFO_FormatAudio		SPA_TYPE_INFO_FORMAT_BASE "Audio"
#define SPA_TYPE_INFO_FORMAT_AUDIO_BASE		SPA_TYPE_INFO_FormatAudio ":"

#define SPA_TYPE_INFO_FormatVideo		SPA_TYPE_INFO_FORMAT_BASE "Video"
#define SPA_TYPE_INFO_FORMAT_VIDEO_BASE		SPA_TYPE_INFO_FormatVideo ":"

#define SPA_TYPE_INFO_FORMAT_VIDEO_H264		SPA_TYPE_INFO_FORMAT_VIDEO_BASE "H264"
#define SPA_TYPE_INFO_FORMAT_VIDEO_H264_BASE	SPA_TYPE_INFO_FORMAT_VIDEO_H264 ":"

static const struct spa_type_info spa_type_format[] = {
	{ SPA_FORMAT_START, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_BASE, spa_type_param, },

	{ SPA_FORMAT_mediaType, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_BASE "mediaType",
		spa_type_media_type, },
	{ SPA_FORMAT_mediaSubtype, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_BASE "mediaSubtype",
		spa_type_media_subtype, },

	{ SPA_FORMAT_AUDIO_format, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_AUDIO_BASE "format",
		spa_type_audio_format },
	{ SPA_FORMAT_AUDIO_flags, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_AUDIO_BASE "flags",
		spa_type_audio_flags },
	{ SPA_FORMAT_AUDIO_rate, SPA_TYPE_Int, SPA_TYPE_INFO_FORMAT_AUDIO_BASE "rate", NULL },
	{ SPA_FORMAT_AUDIO_channels, SPA_TYPE_Int, SPA_TYPE_INFO_FORMAT_AUDIO_BASE "channels", NULL },
	{ SPA_FORMAT_AUDIO_position, SPA_TYPE_Array, SPA_TYPE_INFO_FORMAT_AUDIO_BASE "position",
		spa_type_prop_channel_map },

	{ SPA_FORMAT_AUDIO_iec958Codec, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_AUDIO_BASE "iec958Codec",
		spa_type_audio_iec958_codec },

	{ SPA_FORMAT_AUDIO_bitorder, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_AUDIO_BASE "bitorder",
		spa_type_param_bitorder },
	{ SPA_FORMAT_AUDIO_interleave, SPA_TYPE_Int, SPA_TYPE_INFO_FORMAT_AUDIO_BASE "interleave", NULL },

	{ SPA_FORMAT_VIDEO_format, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "format",
		spa_type_video_format, },
	{ SPA_FORMAT_VIDEO_modifier, SPA_TYPE_Long, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "modifier", NULL },
	{ SPA_FORMAT_VIDEO_size,  SPA_TYPE_Rectangle, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "size", NULL },
	{ SPA_FORMAT_VIDEO_framerate, SPA_TYPE_Fraction, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "framerate", NULL },
	{ SPA_FORMAT_VIDEO_maxFramerate, SPA_TYPE_Fraction, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "maxFramerate", NULL },
	{ SPA_FORMAT_VIDEO_views, SPA_TYPE_Int, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "views", NULL },
	{ SPA_FORMAT_VIDEO_interlaceMode, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "interlaceMode", NULL },
	{ SPA_FORMAT_VIDEO_pixelAspectRatio, SPA_TYPE_Fraction, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "pixelAspectRatio", NULL },
	{ SPA_FORMAT_VIDEO_multiviewMode, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "multiviewMode", NULL },
	{ SPA_FORMAT_VIDEO_multiviewFlags, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "multiviewFlags", NULL },
	{ SPA_FORMAT_VIDEO_chromaSite, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "chromaSite", NULL },
	{ SPA_FORMAT_VIDEO_colorRange, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "colorRange", NULL },
	{ SPA_FORMAT_VIDEO_colorMatrix, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "colorMatrix", NULL },
	{ SPA_FORMAT_VIDEO_transferFunction, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "transferFunction", NULL },
	{ SPA_FORMAT_VIDEO_colorPrimaries, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "colorPrimaries", NULL },
	{ SPA_FORMAT_VIDEO_profile, SPA_TYPE_Int, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "profile", NULL },
	{ SPA_FORMAT_VIDEO_level, SPA_TYPE_Int, SPA_TYPE_INFO_FORMAT_VIDEO_BASE "level", NULL },

	{ SPA_FORMAT_VIDEO_H264_streamFormat, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_VIDEO_H264_BASE "streamFormat", NULL },
	{ SPA_FORMAT_VIDEO_H264_alignment, SPA_TYPE_Id, SPA_TYPE_INFO_FORMAT_VIDEO_H264_BASE "alignment", NULL },
	{ 0, 0, NULL, NULL },
};

#define SPA_TYPE_INFO_PARAM_Buffers			SPA_TYPE_INFO_PARAM_BASE "Buffers"
#define SPA_TYPE_INFO_PARAM_BUFFERS_BASE		SPA_TYPE_INFO_PARAM_Buffers ":"

#define SPA_TYPE_INFO_PARAM_BlockInfo			SPA_TYPE_INFO_PARAM_BUFFERS_BASE "BlockInfo"
#define SPA_TYPE_INFO_PARAM_BLOCK_INFO_BASE		SPA_TYPE_INFO_PARAM_BlockInfo ":"

static const struct spa_type_info spa_type_param_buffers[] = {
	{ SPA_PARAM_BUFFERS_START,    SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_BUFFERS_BASE, spa_type_param, },
	{ SPA_PARAM_BUFFERS_buffers,  SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_BUFFERS_BASE "buffers", NULL },
	{ SPA_PARAM_BUFFERS_blocks,   SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_BUFFERS_BASE "blocks", NULL },
	{ SPA_PARAM_BUFFERS_size,     SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_BLOCK_INFO_BASE "size", NULL },
	{ SPA_PARAM_BUFFERS_stride,   SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_BLOCK_INFO_BASE "stride", NULL },
	{ SPA_PARAM_BUFFERS_align,    SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_BLOCK_INFO_BASE "align", NULL },
	{ SPA_PARAM_BUFFERS_dataType, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_BLOCK_INFO_BASE "dataType", NULL },
	{ 0, 0, NULL, NULL },
};

#define SPA_TYPE_INFO_ParamAvailability		SPA_TYPE_INFO_ENUM_BASE "ParamAvailability"
#define SPA_TYPE_INFO_PARAM_AVAILABILITY_BASE	SPA_TYPE_INFO_ParamAvailability ":"

static const struct spa_type_info spa_type_param_availability[] = {
	{ SPA_PARAM_AVAILABILITY_unknown, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_AVAILABILITY_BASE "unknown", NULL },
	{ SPA_PARAM_AVAILABILITY_no, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_AVAILABILITY_BASE "no", NULL },
	{ SPA_PARAM_AVAILABILITY_yes, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_AVAILABILITY_BASE "yes", NULL },
	{ 0, 0, NULL, NULL },
};

#define SPA_TYPE_INFO_PARAM_Profile		SPA_TYPE_INFO_PARAM_BASE "Profile"
#define SPA_TYPE_INFO_PARAM_PROFILE_BASE	SPA_TYPE_INFO_PARAM_Profile ":"

static const struct spa_type_info spa_type_param_profile[] = {
	{ SPA_PARAM_PROFILE_START, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_PROFILE_BASE, spa_type_param, },
	{ SPA_PARAM_PROFILE_index, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_PROFILE_BASE "index", NULL },
	{ SPA_PARAM_PROFILE_name, SPA_TYPE_String, SPA_TYPE_INFO_PARAM_PROFILE_BASE "name", NULL },
	{ SPA_PARAM_PROFILE_description, SPA_TYPE_String, SPA_TYPE_INFO_PARAM_PROFILE_BASE "description", NULL },
	{ SPA_PARAM_PROFILE_priority, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_PROFILE_BASE "priority", NULL },
	{ SPA_PARAM_PROFILE_available, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_PROFILE_BASE "available", spa_type_param_availability, },
	{ SPA_PARAM_PROFILE_info, SPA_TYPE_Struct, SPA_TYPE_INFO_PARAM_PROFILE_BASE "info", NULL, },
	{ SPA_PARAM_PROFILE_classes, SPA_TYPE_Struct, SPA_TYPE_INFO_PARAM_PROFILE_BASE "classes", NULL, },
	{ SPA_PARAM_PROFILE_save, SPA_TYPE_Bool, SPA_TYPE_INFO_PARAM_PROFILE_BASE "save", NULL, },
	{ 0, 0, NULL, NULL },
};

#define SPA_TYPE_INFO_ParamPortConfigMode		SPA_TYPE_INFO_ENUM_BASE "ParamPortConfigMode"
#define SPA_TYPE_INFO_PARAM_PORT_CONFIG_MODE_BASE	SPA_TYPE_INFO_ParamPortConfigMode ":"

static const struct spa_type_info spa_type_param_port_config_mode[] = {
	{ SPA_PARAM_PORT_CONFIG_MODE_none, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_PORT_CONFIG_MODE_BASE "none", NULL },
	{ SPA_PARAM_PORT_CONFIG_MODE_passthrough, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_PORT_CONFIG_MODE_BASE "passthrough", NULL },
	{ SPA_PARAM_PORT_CONFIG_MODE_convert, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_PORT_CONFIG_MODE_BASE "convert", NULL },
	{ SPA_PARAM_PORT_CONFIG_MODE_dsp, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_PORT_CONFIG_MODE_BASE "dsp", NULL },
	{ 0, 0, NULL, NULL },
};

#define SPA_TYPE_INFO_PARAM_PortConfig		SPA_TYPE_INFO_PARAM_BASE "PortConfig"
#define SPA_TYPE_INFO_PARAM_PORT_CONFIG_BASE	SPA_TYPE_INFO_PARAM_PortConfig ":"

static const struct spa_type_info spa_type_param_port_config[] = {
	{ SPA_PARAM_PORT_CONFIG_START, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_PORT_CONFIG_BASE, spa_type_param, },
	{ SPA_PARAM_PORT_CONFIG_direction, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_PORT_CONFIG_BASE "direction", spa_type_direction, },
	{ SPA_PARAM_PORT_CONFIG_mode, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_PORT_CONFIG_BASE "mode", spa_type_param_port_config_mode },
	{ SPA_PARAM_PORT_CONFIG_monitor, SPA_TYPE_Bool, SPA_TYPE_INFO_PARAM_PORT_CONFIG_BASE "monitor", NULL },
	{ SPA_PARAM_PORT_CONFIG_control, SPA_TYPE_Bool, SPA_TYPE_INFO_PARAM_PORT_CONFIG_BASE "control", NULL },
	{ SPA_PARAM_PORT_CONFIG_format, SPA_TYPE_OBJECT_Format, SPA_TYPE_INFO_PARAM_PORT_CONFIG_BASE "format", NULL },
	{ 0, 0, NULL, NULL },
};


#define SPA_TYPE_INFO_PARAM_Route		SPA_TYPE_INFO_PARAM_BASE "Route"
#define SPA_TYPE_INFO_PARAM_ROUTE_BASE		SPA_TYPE_INFO_PARAM_Route ":"

static const struct spa_type_info spa_type_param_route[] = {
	{ SPA_PARAM_ROUTE_START, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_ROUTE_BASE, spa_type_param, },
	{ SPA_PARAM_ROUTE_index, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_ROUTE_BASE "index", NULL, },
	{ SPA_PARAM_ROUTE_direction, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_ROUTE_BASE "direction", spa_type_direction, },
	{ SPA_PARAM_ROUTE_device, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_ROUTE_BASE "device", NULL, },
	{ SPA_PARAM_ROUTE_name, SPA_TYPE_String, SPA_TYPE_INFO_PARAM_ROUTE_BASE "name", NULL, },
	{ SPA_PARAM_ROUTE_description, SPA_TYPE_String, SPA_TYPE_INFO_PARAM_ROUTE_BASE "description", NULL, },
	{ SPA_PARAM_ROUTE_priority, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_ROUTE_BASE "priority", NULL, },
	{ SPA_PARAM_ROUTE_available, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_ROUTE_BASE "available", spa_type_param_availability, },
	{ SPA_PARAM_ROUTE_info, SPA_TYPE_Struct, SPA_TYPE_INFO_PARAM_ROUTE_BASE "info", NULL, },
	{ SPA_PARAM_ROUTE_profiles, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_ROUTE_BASE "profiles", NULL, },
	{ SPA_PARAM_ROUTE_props, SPA_TYPE_OBJECT_Props, SPA_TYPE_INFO_PARAM_ROUTE_BASE "props", NULL, },
	{ SPA_PARAM_ROUTE_devices, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_ROUTE_BASE "devices", NULL, },
	{ SPA_PARAM_ROUTE_profile, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_ROUTE_BASE "profile", NULL, },
	{ SPA_PARAM_ROUTE_save, SPA_TYPE_Bool, SPA_TYPE_INFO_PARAM_ROUTE_BASE "save", NULL, },
	{ 0, 0, NULL, NULL },
};

#include <spa/param/profiler.h>

#define SPA_TYPE_INFO_Profiler		SPA_TYPE_INFO_OBJECT_BASE "Profiler"
#define SPA_TYPE_INFO_PROFILER_BASE	SPA_TYPE_INFO_Profiler ":"

static const struct spa_type_info spa_type_profiler[] = {
	{ SPA_PROFILER_START, SPA_TYPE_Id, SPA_TYPE_INFO_PROFILER_BASE, spa_type_param, },
	{ SPA_PROFILER_info, SPA_TYPE_Struct, SPA_TYPE_INFO_PROFILER_BASE "info", NULL, },
	{ SPA_PROFILER_clock, SPA_TYPE_Struct, SPA_TYPE_INFO_PROFILER_BASE "clock", NULL, },
	{ SPA_PROFILER_driverBlock, SPA_TYPE_Struct, SPA_TYPE_INFO_PROFILER_BASE "driverBlock", NULL, },
	{ SPA_PROFILER_followerBlock, SPA_TYPE_Struct, SPA_TYPE_INFO_PROFILER_BASE "followerBlock", NULL, },
	{ 0, 0, NULL, NULL },
};

#define SPA_TYPE_INFO_PARAM_Latency		SPA_TYPE_INFO_PARAM_BASE "Latency"
#define SPA_TYPE_INFO_PARAM_LATENCY_BASE	SPA_TYPE_INFO_PARAM_Latency ":"

static const struct spa_type_info spa_type_param_latency[] = {
	{ SPA_PARAM_LATENCY_START, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_LATENCY_BASE, spa_type_param, },
	{ SPA_PARAM_LATENCY_direction, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_LATENCY_BASE "direction", spa_type_direction, },
	{ SPA_PARAM_LATENCY_minQuantum, SPA_TYPE_Float, SPA_TYPE_INFO_PARAM_LATENCY_BASE "minQuantum", NULL, },
	{ SPA_PARAM_LATENCY_maxQuantum, SPA_TYPE_Float, SPA_TYPE_INFO_PARAM_LATENCY_BASE "maxQuantum", NULL, },
	{ SPA_PARAM_LATENCY_minRate, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_LATENCY_BASE "minRate", NULL, },
	{ SPA_PARAM_LATENCY_maxRate, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_LATENCY_BASE "maxRate", NULL, },
	{ SPA_PARAM_LATENCY_minNs, SPA_TYPE_Long, SPA_TYPE_INFO_PARAM_LATENCY_BASE "minNs", NULL, },
	{ SPA_PARAM_LATENCY_maxNs, SPA_TYPE_Long, SPA_TYPE_INFO_PARAM_LATENCY_BASE "maxNs", NULL, },
	{ 0, 0, NULL, NULL },
};

#define SPA_TYPE_INFO_PARAM_ProcessLatency		SPA_TYPE_INFO_PARAM_BASE "ProcessLatency"
#define SPA_TYPE_INFO_PARAM_PROCESS_LATENCY_BASE	SPA_TYPE_INFO_PARAM_ProcessLatency ":"

static const struct spa_type_info spa_type_param_process_latency[] = {
	{ SPA_PARAM_PROCESS_LATENCY_START, SPA_TYPE_Id, SPA_TYPE_INFO_PARAM_LATENCY_BASE, spa_type_param, },
	{ SPA_PARAM_PROCESS_LATENCY_quantum, SPA_TYPE_Float, SPA_TYPE_INFO_PARAM_PROCESS_LATENCY_BASE "quantum", NULL, },
	{ SPA_PARAM_PROCESS_LATENCY_rate, SPA_TYPE_Int, SPA_TYPE_INFO_PARAM_PROCESS_LATENCY_BASE "rate", NULL, },
	{ SPA_PARAM_PROCESS_LATENCY_ns, SPA_TYPE_Long, SPA_TYPE_INFO_PARAM_PROCESS_LATENCY_BASE "ns", NULL, },
	{ 0, 0, NULL, NULL },
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_PARAM_TYPES_H */
