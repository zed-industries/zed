/*
 * Video Decode and Presentation API for UNIX (VDPAU) is used for
 * HW decode acceleration for MPEG-1/2, H.264 and VC-1.
 *
 * Copyright (C) 2008 NVIDIA
 *
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

#ifndef AVCODEC_VDPAU_INTERNAL_H
#define AVCODEC_VDPAU_INTERNAL_H

#include <stdint.h>
#include <vdpau/vdpau.h>

#include "libavutil/frame.h"
#include "libavutil/hwcontext.h"
#include "libavutil/hwcontext_vdpau.h"

#include "avcodec.h"
#include "vdpau.h"

/** Extract VdpVideoSurface from an AVFrame */
static inline uintptr_t ff_vdpau_get_surface_id(AVFrame *pic)
{
    return (uintptr_t)pic->data[3];
}

union VDPAUPictureInfo {
    VdpPictureInfoH264        h264;
    VdpPictureInfoMPEG1Or2    mpeg;
    VdpPictureInfoVC1          vc1;
    VdpPictureInfoMPEG4Part2 mpeg4;
#ifdef VDP_DECODER_PROFILE_H264_HIGH_444_PREDICTIVE
    VdpPictureInfoH264Predictive h264_predictive;
#endif
#ifdef VDP_DECODER_PROFILE_HEVC_MAIN
    VdpPictureInfoHEVC        hevc;
#endif
#ifdef VDP_YCBCR_FORMAT_Y_U_V_444
    VdpPictureInfoHEVC444     hevc_444;
#endif
#ifdef VDP_DECODER_PROFILE_VP9_PROFILE_0
    VdpPictureInfoVP9        vp9;
#endif
#ifdef VDP_DECODER_PROFILE_AV1_MAIN
    VdpPictureInfoAV1        av1;
#endif
};

typedef struct VDPAUHWContext {
    AVVDPAUContext context;
    VdpDevice device;
    VdpGetProcAddress *get_proc_address;
    char reset;
    unsigned char flags;
} VDPAUHWContext;

typedef struct VDPAUContext {
    /**
     * VDPAU device handle
     */
    VdpDevice device;

    /**
     * VDPAU decoder handle
     */
    VdpDecoder decoder;

    /**
     * VDPAU device driver
     */
    VdpGetProcAddress *get_proc_address;

    /**
     * VDPAU decoder render callback
     */
    VdpDecoderRender *render;

    uint32_t width;
    uint32_t height;
} VDPAUContext;

struct vdpau_picture_context {
    /**
     * VDPAU picture information.
     */
    union VDPAUPictureInfo info;

    /**
     * Allocated size of the bitstream_buffers table.
     */
    int bitstream_buffers_allocated;

    /**
     * Useful bitstream buffers in the bitstream buffers table.
     */
    int bitstream_buffers_used;

   /**
     * Table of bitstream buffers.
     */
    VdpBitstreamBuffer *bitstream_buffers;
};

int ff_vdpau_common_init(AVCodecContext *avctx, VdpDecoderProfile profile,
                         int level);
int ff_vdpau_common_uninit(AVCodecContext *avctx);

int ff_vdpau_common_start_frame(struct vdpau_picture_context *pic,
                                const uint8_t *buffer, uint32_t size);
int ff_vdpau_common_end_frame(AVCodecContext *avctx, AVFrame *frame,
                              struct vdpau_picture_context *pic);
int ff_vdpau_mpeg_end_frame(AVCodecContext *avctx);
int ff_vdpau_add_buffer(struct vdpau_picture_context *pic, const uint8_t *buf,
                        uint32_t buf_size);
int ff_vdpau_common_frame_params(AVCodecContext *avctx,
                                 AVBufferRef *hw_frames_ctx);

#endif /* AVCODEC_VDPAU_INTERNAL_H */
