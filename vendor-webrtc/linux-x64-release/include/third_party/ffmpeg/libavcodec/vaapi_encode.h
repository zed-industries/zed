/*
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVCODEC_VAAPI_ENCODE_H
#define AVCODEC_VAAPI_ENCODE_H

#include <stdint.h>

#include <va/va.h>

#if VA_CHECK_VERSION(1, 0, 0)
#include <va/va_str.h>
#endif

#include "libavutil/hwcontext.h"
#include "libavutil/hwcontext_vaapi.h"

#include "avcodec.h"
#include "hwconfig.h"
#include "hw_base_encode.h"

struct VAAPIEncodeType;
struct VAAPIEncodePicture;

// Codec output packet without timestamp delay, which means the
// output packet has same PTS and DTS.
#define FLAG_TIMESTAMP_NO_DELAY 1 << 6

enum {
    MAX_CONFIG_ATTRIBUTES  = 4,
    MAX_GLOBAL_PARAMS      = 4,
    MAX_PARAM_BUFFER_SIZE  = 1024,
    // A.4.1: table A.6 allows at most 22 tile rows for any level.
    MAX_TILE_ROWS          = 22,
    // A.4.1: table A.6 allows at most 20 tile columns for any level.
    MAX_TILE_COLS          = 20,
};

extern const AVCodecHWConfigInternal *const ff_vaapi_encode_hw_configs[];

typedef struct VAAPIEncodeSlice {
    int             index;
    int             row_start;
    int             row_size;
    int             block_start;
    int             block_size;
    void           *codec_slice_params;
} VAAPIEncodeSlice;

typedef struct VAAPIEncodePicture {
#if VA_CHECK_VERSION(1, 0, 0)
    // ROI regions.
    VAEncROI       *roi;
#else
    void           *roi;
#endif

    VASurfaceID     input_surface;
    VASurfaceID     recon_surface;

    int          nb_param_buffers;
    VABufferID     *param_buffers;

    /* Refcounted via the refstruct-API */
    VABufferID     *output_buffer_ref;
    VABufferID      output_buffer;

    void           *codec_picture_params;

    int          nb_slices;
    VAAPIEncodeSlice *slices;

    /**
     * indicate if current frame is an independent frame that the coded data
     * can be pushed to downstream directly. Coded of non-independent frame
     * data will be concatenated into next independent frame.
     */
    int non_independent_frame;
    /** Tail data of current pic, used only for repeat header of AV1. */
    char tail_data[MAX_PARAM_BUFFER_SIZE];
    /** Byte length of tail_data. */
    size_t tail_size;
} VAAPIEncodePicture;

typedef struct VAAPIEncodeProfile {
    // lavc profile value (AV_PROFILE_*).
    int       av_profile;
    // Supported bit depth.
    int       depth;
    // Number of components.
    int       nb_components;
    // Chroma subsampling in width dimension.
    int       log2_chroma_w;
    // Chroma subsampling in height dimension.
    int       log2_chroma_h;
    // VAAPI profile value.
    VAProfile va_profile;
} VAAPIEncodeProfile;

enum {
    RC_MODE_AUTO,
    RC_MODE_CQP,
    RC_MODE_CBR,
    RC_MODE_VBR,
    RC_MODE_ICQ,
    RC_MODE_QVBR,
    RC_MODE_AVBR,
    RC_MODE_MAX = RC_MODE_AVBR,
};

typedef struct VAAPIEncodeRCMode {
    // Mode from above enum (RC_MODE_*).
    int mode;
    // Name.
    const char *name;
    // Supported in the compile-time VAAPI version.
    int supported;
    // VA mode value (VA_RC_*).
    uint32_t va_mode;
    // Uses bitrate parameters.
    int bitrate;
    // Supports maxrate distinct from bitrate.
    int maxrate;
    // Uses quality value.
    int quality;
    // Supports HRD/VBV parameters.
    int hrd;
} VAAPIEncodeRCMode;

typedef struct VAAPIEncodeContext {
    // Base context.
    FFHWBaseEncodeContext base;

    // Codec-specific hooks.
    const struct VAAPIEncodeType *codec;

    // Use low power encoding mode.
    int             low_power;

    // Max Frame Size
    int             max_frame_size;

    // Explicitly set RC mode (otherwise attempt to pick from
    // available modes).
    int             explicit_rc_mode;

    // Block Level based bitrate control.
    int             blbrc;

    // Explicitly-set QP, for use with the "qp" options.
    // (Forces CQP mode when set, overriding everything else.)
    int             explicit_qp;

    // Desired packed headers.
    unsigned int    desired_packed_headers;

    // Everything above this point must be set before calling
    // ff_vaapi_encode_init().

    // Chosen encoding profile details.
    const VAAPIEncodeProfile *profile;

    // Chosen rate control mode details.
    const VAAPIEncodeRCMode *rc_mode;
    // RC quality level - meaning depends on codec and RC mode.
    // In CQP mode this sets the fixed quantiser value.
    int             rc_quality;

    // Encoding profile (VAProfile*).
    VAProfile       va_profile;
    // Encoding entrypoint (VAEntryoint*).
    VAEntrypoint    va_entrypoint;
    // Rate control mode.
    unsigned int    va_rc_mode;
    // Bitrate for codec-specific encoder parameters.
    unsigned int    va_bit_rate;
    // Packed headers which will actually be sent.
    unsigned int    va_packed_headers;

    // Configuration attributes to use when creating va_config.
    VAConfigAttrib  config_attributes[MAX_CONFIG_ATTRIBUTES];
    int          nb_config_attributes;

    VAConfigID      va_config;
    VAContextID     va_context;

    AVVAAPIDeviceContext *hwctx;

    // Pool of (reusable) bitstream output buffers.
    struct AVRefStructPool *output_buffer_pool;

    // Global parameters which will be applied at the start of the
    // sequence (includes rate control parameters below).
    int             global_params_type[MAX_GLOBAL_PARAMS];
    const void     *global_params     [MAX_GLOBAL_PARAMS];
    size_t          global_params_size[MAX_GLOBAL_PARAMS];
    int          nb_global_params;

    // Rate control parameters.
    VAEncMiscParameterRateControl rc_params;
    VAEncMiscParameterHRD        hrd_params;
    VAEncMiscParameterFrameRate   fr_params;
    VAEncMiscParameterBufferMaxFrameSize mfs_params;
#if VA_CHECK_VERSION(0, 36, 0)
    VAEncMiscParameterBufferQualityLevel quality_params;
#endif

    // Per-sequence parameter structure (VAEncSequenceParameterBuffer*).
    void           *codec_sequence_params;

    // Per-sequence parameters found in the per-picture parameter
    // structure (VAEncPictureParameterBuffer*).
    void           *codec_picture_params;

    // Slice structure.
    int slice_block_rows;
    int slice_block_cols;
    int nb_slices;
    int slice_size;

    // Tile encoding.
    int tile_cols;
    int tile_rows;
    // Tile width of the i-th column.
    int col_width[MAX_TILE_COLS];
    // Tile height of i-th row.
    int row_height[MAX_TILE_ROWS];
    // Location of the i-th tile column boundary.
    int col_bd[MAX_TILE_COLS + 1];
    // Location of the i-th tile row boundary.
    int row_bd[MAX_TILE_ROWS + 1];

    // Maximum number of regions supported by the driver.
    int             roi_max_regions;
    // Quantisation range for offset calculations.  Set by codec-specific
    // code, as it may change based on parameters.
    int             roi_quant_range;

    /** Head data for current output pkt, used only for AV1. */
    //void  *header_data;
    //size_t header_data_size;

    /**
     * Buffered coded data of a pic if it is an non-independent frame.
     * This is a RefStruct reference.
     */
    VABufferID     *coded_buffer_ref;

    // Surface alignment required by driver.
    int             surface_alignment_width;
    int             surface_alignment_height;
} VAAPIEncodeContext;

typedef struct VAAPIEncodeType {
    // List of supported profiles and corresponding VAAPI profiles.
    // (Must end with AV_PROFILE_UNKNOWN.)
    const VAAPIEncodeProfile *profiles;

    // Codec feature flags.
    int flags;

    // Default quality for this codec - used as quantiser or RC quality
    // factor depending on RC mode.
    int default_quality;

    // Determine encode parameters like block sizes for surface alignment
    // and slices. This may need to query the profile and entrypoint,
    // which will be available when this function is called. If not set,
    // assume that all blocks are 16x16 and that surfaces should be
    // aligned to match this.
    int (*get_encoder_caps)(AVCodecContext *avctx);

    // Perform any extra codec-specific configuration after the
    // codec context is initialised (set up the private data and
    // add any necessary global parameters).
    int (*configure)(AVCodecContext *avctx);

    // The size of any private data structure associated with each
    // picture (can be zero if not required).
    size_t picture_priv_data_size;

    // The size of the parameter structures:
    // sizeof(VAEnc{type}ParameterBuffer{codec}).
    size_t sequence_params_size;
    size_t picture_params_size;
    size_t slice_params_size;

    // Fill the parameter structures.
    int  (*init_sequence_params)(AVCodecContext *avctx);
    int   (*init_picture_params)(AVCodecContext *avctx,
                                 FFHWBaseEncodePicture *pic);
    int     (*init_slice_params)(AVCodecContext *avctx,
                                 FFHWBaseEncodePicture *pic,
                                 VAAPIEncodeSlice *slice);

    // The type used by the packed header: this should look like
    // VAEncPackedHeader{something}.
    int sequence_header_type;
    int picture_header_type;
    int slice_header_type;

    // Write the packed header data to the provided buffer.
    // The sequence header is also used to fill the codec extradata
    // when the encoder is starting.
    int (*write_sequence_header)(AVCodecContext *avctx,
                                 char *data, size_t *data_len);
    int  (*write_picture_header)(AVCodecContext *avctx,
                                 FFHWBaseEncodePicture *pic,
                                 char *data, size_t *data_len);
    int    (*write_slice_header)(AVCodecContext *avctx,
                                 VAAPIEncodePicture *pic,
                                 VAAPIEncodeSlice *slice,
                                 char *data, size_t *data_len);

    // Fill an extra parameter structure, which will then be
    // passed to vaRenderPicture().  Will be called repeatedly
    // with increasing index argument until AVERROR_EOF is
    // returned.
    int    (*write_extra_buffer)(AVCodecContext *avctx,
                                 FFHWBaseEncodePicture *pic,
                                 int index, int *type,
                                 char *data, size_t *data_len);

    // Write an extra packed header.  Will be called repeatedly
    // with increasing index argument until AVERROR_EOF is
    // returned.
    int    (*write_extra_header)(AVCodecContext *avctx,
                                 FFHWBaseEncodePicture *pic,
                                 int index, int *type,
                                 char *data, size_t *data_len);
} VAAPIEncodeType;

int ff_vaapi_encode_receive_packet(AVCodecContext *avctx, AVPacket *pkt);

int ff_vaapi_encode_init(AVCodecContext *avctx);
int ff_vaapi_encode_close(AVCodecContext *avctx);


#define VAAPI_ENCODE_COMMON_OPTIONS \
    { "low_power", \
      "Use low-power encoding mode (only available on some platforms; " \
      "may not support all encoding features)", \
      OFFSET(common.low_power), AV_OPT_TYPE_BOOL, \
      { .i64 = 0 }, 0, 1, FLAGS }, \
    { "max_frame_size", \
      "Maximum frame size (in bytes)",\
      OFFSET(common.max_frame_size), AV_OPT_TYPE_INT, \
      { .i64 = 0 }, 0, INT_MAX, FLAGS }

#define VAAPI_ENCODE_RC_MODE(name, desc) \
    { #name, desc, 0, AV_OPT_TYPE_CONST, { .i64 = RC_MODE_ ## name }, \
      0, 0, FLAGS, .unit = "rc_mode" }
#define VAAPI_ENCODE_RC_OPTIONS \
    { "rc_mode",\
      "Set rate control mode", \
      OFFSET(common.explicit_rc_mode), AV_OPT_TYPE_INT, \
      { .i64 = RC_MODE_AUTO }, RC_MODE_AUTO, RC_MODE_MAX, FLAGS, .unit = "rc_mode" }, \
    { "auto", "Choose mode automatically based on other parameters", \
      0, AV_OPT_TYPE_CONST, { .i64 = RC_MODE_AUTO }, 0, 0, FLAGS, .unit = "rc_mode" }, \
    VAAPI_ENCODE_RC_MODE(CQP,  "Constant-quality"), \
    VAAPI_ENCODE_RC_MODE(CBR,  "Constant-bitrate"), \
    VAAPI_ENCODE_RC_MODE(VBR,  "Variable-bitrate"), \
    VAAPI_ENCODE_RC_MODE(ICQ,  "Intelligent constant-quality"), \
    VAAPI_ENCODE_RC_MODE(QVBR, "Quality-defined variable-bitrate"), \
    VAAPI_ENCODE_RC_MODE(AVBR, "Average variable-bitrate"), \
    { "blbrc", \
      "Block level based bitrate control",\
      OFFSET(common.blbrc), AV_OPT_TYPE_BOOL, \
      { .i64 = 0 }, 0, 1, FLAGS }


#endif /* AVCODEC_VAAPI_ENCODE_H */
