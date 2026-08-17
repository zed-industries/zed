/*
 * DXVA2 HW acceleration
 *
 * copyright (c) 2010 Laurent Aimar
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

#ifndef AVCODEC_DXVA2_INTERNAL_H
#define AVCODEC_DXVA2_INTERNAL_H

#define COBJMACROS

#include "config.h"
#include "config_components.h"

/* define the proper COM entries before forcing desktop APIs */
#include <objbase.h>

#define FF_DXVA2_WORKAROUND_SCALING_LIST_ZIGZAG 1 ///< Work around for DXVA2/Direct3D11 and old UVD/UVD+ ATI video cards
#define FF_DXVA2_WORKAROUND_INTEL_CLEARVIDEO    2 ///< Work around for DXVA2/Direct3D11 and old Intel GPUs with ClearVideo interface

#if CONFIG_DXVA2
#include "dxva2.h"
#include "libavutil/hwcontext_dxva2.h"
#define DXVA2_VAR(ctx, var) ctx->dxva2.var
#else
#define DXVA2_VAR(ctx, var) 0
#endif

#if CONFIG_D3D11VA
#include "d3d11va.h"
#include "libavutil/hwcontext_d3d11va.h"
#define D3D11VA_VAR(ctx, var) ctx->d3d11va.var
#else
#define D3D11VA_VAR(ctx, var) 0
#endif

#if CONFIG_D3D12VA
#include "d3d12va_decode.h"
#endif

#if HAVE_DXVA_H
/* When targeting WINAPI_FAMILY_PHONE_APP or WINAPI_FAMILY_APP, dxva.h
 * defines nothing. Force the struct definitions to be visible. */
#undef WINAPI_FAMILY
#define WINAPI_FAMILY WINAPI_FAMILY_DESKTOP_APP
#undef _CRT_BUILD_DESKTOP_APP
#define _CRT_BUILD_DESKTOP_APP 0
#include <dxva.h>
#endif

#include "libavutil/hwcontext.h"

#include "avcodec.h"
#include "internal.h"

typedef void DECODER_BUFFER_DESC;

typedef union {
#if CONFIG_D3D11VA
    struct AVD3D11VAContext  d3d11va;
#endif
#if CONFIG_DXVA2
    struct dxva_context      dxva2;
#endif
#if CONFIG_D3D12VA
    struct D3D12VADecodeContext d3d12va;
#endif
} AVDXVAContext;

typedef struct FFDXVASharedContext {
    AVBufferRef *decoder_ref;

    // FF_DXVA2_WORKAROUND_* flags
    uint64_t workaround;

    // E.g. AV_PIX_FMT_D3D11 (same as AVCodecContext.pix_fmt, except during init)
    enum AVPixelFormat pix_fmt;

    AVHWDeviceContext *device_ctx;

#if CONFIG_D3D11VA
    ID3D11VideoDecoder             *d3d11_decoder;
    D3D11_VIDEO_DECODER_CONFIG      d3d11_config;
    ID3D11VideoDecoderOutputView  **d3d11_views;
    int                          nb_d3d11_views;
    ID3D11Texture2D                *d3d11_texture;
#endif

#if CONFIG_DXVA2
    IDirectXVideoDecoder           *dxva2_decoder;
    IDirectXVideoDecoderService    *dxva2_service;
    DXVA2_ConfigPictureDecode       dxva2_config;
#endif

    // Legacy (but used by code outside of setup)
    // In generic mode, DXVA_CONTEXT() will return a pointer to this.
    AVDXVAContext ctx;
} FFDXVASharedContext;

#define DXVA_SHARED_CONTEXT(avctx) ((FFDXVASharedContext *)((avctx)->internal->hwaccel_priv_data))

#define DXVA_CONTEXT(avctx) (AVDXVAContext *)((avctx)->hwaccel_context ? (avctx)->hwaccel_context : (&(DXVA_SHARED_CONTEXT(avctx)->ctx)))

#define D3D11VA_CONTEXT(ctx) (&ctx->d3d11va)
#define DXVA2_CONTEXT(ctx)   (&ctx->dxva2)

#define DXVA2_CONTEXT_VAR(avctx, ctx, var) (avctx->pix_fmt == AV_PIX_FMT_D3D12 ? 0 : (ff_dxva2_is_d3d11(avctx) ? D3D11VA_VAR(ctx, var) : DXVA2_VAR(ctx, var)))

#define DXVA_CONTEXT_REPORT_ID(avctx, ctx)      (*ff_dxva2_get_report_id(avctx, ctx))
#define DXVA_CONTEXT_WORKAROUND(avctx, ctx)     DXVA2_CONTEXT_VAR(avctx, ctx, workaround)
#define DXVA_CONTEXT_COUNT(avctx, ctx)          DXVA2_CONTEXT_VAR(avctx, ctx, surface_count)
#define DXVA_CONTEXT_DECODER(avctx, ctx)        (avctx->pix_fmt == AV_PIX_FMT_D3D12 ? 0 : (ff_dxva2_is_d3d11(avctx) ? (void *)D3D11VA_VAR(ctx, decoder) : (void *)DXVA2_VAR(ctx, decoder)))
#define DXVA_CONTEXT_CFG(avctx, ctx)            (avctx->pix_fmt == AV_PIX_FMT_D3D12 ? 0 : (ff_dxva2_is_d3d11(avctx) ? (void *)D3D11VA_VAR(ctx, cfg) : (void *)DXVA2_VAR(ctx, cfg)))
#define DXVA_CONTEXT_CFG_BITSTREAM(avctx, ctx)  DXVA2_CONTEXT_VAR(avctx, ctx, cfg->ConfigBitstreamRaw)
#define DXVA_CONTEXT_CFG_INTRARESID(avctx, ctx) DXVA2_CONTEXT_VAR(avctx, ctx, cfg->ConfigIntraResidUnsigned)
#define DXVA_CONTEXT_CFG_RESIDACCEL(avctx, ctx) DXVA2_CONTEXT_VAR(avctx, ctx, cfg->ConfigResidDiffAccelerator)
#define DXVA_CONTEXT_VALID(avctx, ctx)          (DXVA_CONTEXT_DECODER(avctx, ctx) && \
                                                 DXVA_CONTEXT_CFG(avctx, ctx)     && \
                                                 (ff_dxva2_is_d3d11(avctx) || DXVA2_VAR(ctx, surface_count)))

#if CONFIG_D3D12VA
unsigned ff_d3d12va_get_surface_index(const AVCodecContext *avctx,
                                      D3D12VADecodeContext *ctx, const AVFrame *frame,
                                      int curr);
#endif

unsigned ff_dxva2_get_surface_index(const AVCodecContext *avctx,
                                    AVDXVAContext *, const AVFrame *frame, int curr);

int ff_dxva2_commit_buffer(AVCodecContext *, AVDXVAContext *,
                           DECODER_BUFFER_DESC *,
                           unsigned type, const void *data, unsigned size,
                           unsigned mb_count);


int ff_dxva2_common_end_frame(AVCodecContext *, AVFrame *,
                              const void *pp, unsigned pp_size,
                              const void *qm, unsigned qm_size,
                              int (*commit_bs_si)(AVCodecContext *,
                                                  DECODER_BUFFER_DESC *bs,
                                                  DECODER_BUFFER_DESC *slice));

int ff_dxva2_decode_init(AVCodecContext *avctx);

int ff_dxva2_decode_uninit(AVCodecContext *avctx);

int ff_dxva2_common_frame_params(AVCodecContext *avctx,
                                 AVBufferRef *hw_frames_ctx);

int ff_dxva2_is_d3d11(const AVCodecContext *avctx);

unsigned *ff_dxva2_get_report_id(const AVCodecContext *avctx, AVDXVAContext *ctx);

void ff_dxva2_h264_fill_picture_parameters(const AVCodecContext *avctx, AVDXVAContext *ctx, DXVA_PicParams_H264 *pp);

void ff_dxva2_h264_fill_scaling_lists(const AVCodecContext *avctx, AVDXVAContext *ctx, DXVA_Qmatrix_H264 *qm);

#if CONFIG_HEVC_D3D12VA_HWACCEL || CONFIG_HEVC_D3D11VA_HWACCEL || CONFIG_HEVC_D3D11VA2_HWACCEL || CONFIG_HEVC_DXVA2_HWACCEL
void ff_dxva2_hevc_fill_picture_parameters(const AVCodecContext *avctx, AVDXVAContext *ctx, DXVA_PicParams_HEVC *pp);

void ff_dxva2_hevc_fill_scaling_lists(const AVCodecContext *avctx, AVDXVAContext *ctx, DXVA_Qmatrix_HEVC *qm);
#endif

#if CONFIG_VP9_D3D12VA_HWACCEL || CONFIG_VP9_D3D11VA_HWACCEL || CONFIG_VP9_D3D11VA2_HWACCEL || CONFIG_VP9_DXVA2_HWACCEL
int ff_dxva2_vp9_fill_picture_parameters(const AVCodecContext *avctx, AVDXVAContext *ctx, DXVA_PicParams_VP9 *pp);
#endif

#if CONFIG_AV1_D3D12VA_HWACCEL || CONFIG_AV1_D3D11VA_HWACCEL || CONFIG_AV1_D3D11VA2_HWACCEL || CONFIG_AV1_DXVA2_HWACCEL
int ff_dxva2_av1_fill_picture_parameters(const AVCodecContext *avctx, AVDXVAContext *ctx, DXVA_PicParams_AV1 *pp);
#endif

void ff_dxva2_mpeg2_fill_picture_parameters(AVCodecContext *avctx, AVDXVAContext *ctx, DXVA_PictureParameters *pp);

void ff_dxva2_mpeg2_fill_quantization_matrices(AVCodecContext *avctx, AVDXVAContext *ctx, DXVA_QmatrixData *qm);

void ff_dxva2_mpeg2_fill_slice(AVCodecContext *avctx, DXVA_SliceInfo *slice,  unsigned position, const uint8_t *buffer, unsigned size);

void ff_dxva2_vc1_fill_picture_parameters(AVCodecContext *avctx, AVDXVAContext *ctx, DXVA_PictureParameters *pp);

void ff_dxva2_vc1_fill_slice(AVCodecContext *avctx, DXVA_SliceInfo *slice, unsigned position, unsigned size);

#endif /* AVCODEC_DXVA2_INTERNAL_H */
