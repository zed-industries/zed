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

#ifndef AVCODEC_HW_BASE_ENCODE_H
#define AVCODEC_HW_BASE_ENCODE_H

#include "avcodec.h"
#include "libavutil/hwcontext.h"
#include "libavutil/fifo.h"

#define MAX_DPB_SIZE 16
#define MAX_PICTURE_REFERENCES 2
#define MAX_REORDER_DELAY 16
#define MAX_ASYNC_DEPTH 64
#define MAX_REFERENCE_LIST_NUM 2

static inline const char *ff_hw_base_encode_get_pictype_name(const int type)
{
    const char * const picture_type_name[] = { "IDR", "I", "P", "B" };
    return picture_type_name[type];
}

enum {
    FF_HW_PICTURE_TYPE_IDR = 0,
    FF_HW_PICTURE_TYPE_I   = 1,
    FF_HW_PICTURE_TYPE_P   = 2,
    FF_HW_PICTURE_TYPE_B   = 3,
};

enum {
    // Codec supports controlling the subdivision of pictures into slices.
    FF_HW_FLAG_SLICE_CONTROL         = 1 << 0,
    // Codec only supports constant quality (no rate control).
    FF_HW_FLAG_CONSTANT_QUALITY_ONLY = 1 << 1,
    // Codec is intra-only.
    FF_HW_FLAG_INTRA_ONLY            = 1 << 2,
    // Codec supports B-pictures.
    FF_HW_FLAG_B_PICTURES            = 1 << 3,
    // Codec supports referencing B-pictures.
    FF_HW_FLAG_B_PICTURE_REFERENCES  = 1 << 4,
    // Codec supports non-IDR key pictures (that is, key pictures do
    // not necessarily empty the DPB).
    FF_HW_FLAG_NON_IDR_KEY_PICTURES  = 1 << 5,
};

typedef struct FFHWBaseEncodePicture {
    // API-specific private data
    void *priv;
    // Codec-specific private data
    void *codec_priv;

    struct FFHWBaseEncodePicture *next;

    int64_t         display_order;
    int64_t         encode_order;
    int64_t         pts;
    int64_t         duration;
    int             force_idr;

    void           *opaque;
    AVBufferRef    *opaque_ref;

    int             type;
    int             b_depth;
    int             encode_issued;
    int             encode_complete;

    AVFrame        *input_image;
    AVFrame        *recon_image;

    // Whether this picture is a reference picture.
    int             is_reference;

    // The contents of the DPB after this picture has been decoded.
    // This will contain the picture itself if it is a reference picture,
    // but not if it isn't.
    int                     nb_dpb_pics;
    struct FFHWBaseEncodePicture *dpb[MAX_DPB_SIZE];
    // The reference pictures used in decoding this picture. If they are
    // used by later pictures they will also appear in the DPB. ref[0][] for
    // previous reference frames. ref[1][] for future reference frames.
    int                     nb_refs[MAX_REFERENCE_LIST_NUM];
    struct FFHWBaseEncodePicture *refs[MAX_REFERENCE_LIST_NUM][MAX_PICTURE_REFERENCES];
    // The previous reference picture in encode order.  Must be in at least
    // one of the reference list and DPB list.
    struct FFHWBaseEncodePicture *prev;
    // Reference count for other pictures referring to this one through
    // the above pointers, directly from incomplete pictures and indirectly
    // through completed pictures.
    int             ref_count[2];
    int             ref_removed[2];
} FFHWBaseEncodePicture;

typedef struct FFHWEncodePictureOperation {
    // Size of API-specific internal picture data
    size_t priv_size;
    // Initialize API-specific internals
    int (*init)(AVCodecContext *avctx, FFHWBaseEncodePicture *pic);
    // Issue the picture structure, which will send the frame surface to HW Encode API.
    int (*issue)(AVCodecContext *avctx, FFHWBaseEncodePicture *pic);
    // Get the output AVPacket.
    int (*output)(AVCodecContext *avctx, FFHWBaseEncodePicture *pic, AVPacket *pkt);
    // Free the picture structure.
    int (*free)(AVCodecContext *avctx, FFHWBaseEncodePicture *pic);
}  FFHWEncodePictureOperation;

typedef struct FFHWBaseEncodeContext {
    const AVClass *class;
    void  *log_ctx;

    // Hardware-specific hooks.
    const struct FFHWEncodePictureOperation *op;

    // Global options.

    // Number of I frames between IDR frames.
    int             idr_interval;

    // Desired B frame reference depth.
    int             desired_b_depth;

    // The required size of surfaces.  This is probably the input
    // size (AVCodecContext.width|height) aligned up to whatever
    // block size is required by the codec.
    int             surface_width;
    int             surface_height;

    // The block size for slice calculations.
    int             slice_block_width;
    int             slice_block_height;

    // The hardware device context.
    AVBufferRef    *device_ref;
    AVHWDeviceContext *device;

    // The hardware frame context containing the input frames.
    AVBufferRef    *input_frames_ref;
    AVHWFramesContext *input_frames;

    // The hardware frame context containing the reconstructed frames.
    AVBufferRef    *recon_frames_ref;
    AVHWFramesContext *recon_frames;

    // Current encoding window, in display (input) order.
    FFHWBaseEncodePicture *pic_start, *pic_end;
    // The next picture to use as the previous reference picture in
    // encoding order. Order from small to large in encoding order.
    FFHWBaseEncodePicture *next_prev[MAX_PICTURE_REFERENCES];
    int                  nb_next_prev;

    // Next input order index (display order).
    int64_t         input_order;
    // Number of frames that output is behind input.
    int64_t         output_delay;
    // Next encode order index.
    int64_t         encode_order;
    // Number of frames decode output will need to be delayed.
    int64_t         decode_delay;
    // Next output order index (in encode order).
    int64_t         output_order;

    // Timestamp handling.
    int64_t         first_pts;
    int64_t         dts_pts_diff;
    int64_t         ts_ring[MAX_REORDER_DELAY * 3 +
                            MAX_ASYNC_DEPTH];

    // Frame type decision.
    int gop_size;
    int closed_gop;
    int gop_per_idr;
    int p_per_i;
    int max_b_depth;
    int b_per_p;
    int force_idr;
    int idr_counter;
    int gop_counter;
    int end_of_stream;
    int p_to_gpb;

    // The number of L0/L1 references supported by the driver.
    int             ref_l0;
    int             ref_l1;

    // Whether the driver supports ROI at all.
    int             roi_allowed;

    // The encoder does not support cropping information, so warn about
    // it the first time we encounter any nonzero crop fields.
    int             crop_warned;
    // If the driver does not support ROI then warn the first time we
    // encounter a frame with ROI side data.
    int             roi_warned;

    // The frame to be filled with data.
    AVFrame         *frame;

    // Whether the HW supports sync buffer function.
    // If supported, encode_fifo/async_depth will be used together.
    // Used for output buffer synchronization.
    int             async_encode;

    // Store buffered pic.
    AVFifo          *encode_fifo;
    // Max number of frame buffered in encoder.
    int             async_depth;

    /** Tail data of a pic, now only used for av1 repeat frame header. */
    AVPacket        *tail_pkt;
} FFHWBaseEncodeContext;

int ff_hw_base_encode_set_output_property(FFHWBaseEncodeContext *ctx, AVCodecContext *avctx,
                                          FFHWBaseEncodePicture *pic, AVPacket *pkt, int flag_no_delay);

int ff_hw_base_encode_receive_packet(FFHWBaseEncodeContext *ctx, AVCodecContext *avctx, AVPacket *pkt);

int ff_hw_base_init_gop_structure(FFHWBaseEncodeContext *ctx, AVCodecContext *avctx,
                                  uint32_t ref_l0, uint32_t ref_l1,
                                  int flags, int prediction_pre_only);

int ff_hw_base_get_recon_format(FFHWBaseEncodeContext *ctx, const void *hwconfig,
                                enum AVPixelFormat *fmt);

int ff_hw_base_encode_init(AVCodecContext *avctx, FFHWBaseEncodeContext *ctx);

int ff_hw_base_encode_close(FFHWBaseEncodeContext *ctx);

#define HW_BASE_ENCODE_COMMON_OPTIONS \
    { "idr_interval", \
      "Distance (in I-frames) between key frames", \
      OFFSET(common.base.idr_interval), AV_OPT_TYPE_INT, \
      { .i64 = 0 }, 0, INT_MAX, FLAGS }, \
    { "b_depth", \
      "Maximum B-frame reference depth", \
      OFFSET(common.base.desired_b_depth), AV_OPT_TYPE_INT, \
      { .i64 = 1 }, 1, INT_MAX, FLAGS }, \
    { "async_depth", "Maximum processing parallelism. " \
      "Increase this to improve single channel performance.", \
      OFFSET(common.base.async_depth), AV_OPT_TYPE_INT, \
      { .i64 = 2 }, 1, MAX_ASYNC_DEPTH, FLAGS }

#endif /* AVCODEC_HW_BASE_ENCODE_H */
