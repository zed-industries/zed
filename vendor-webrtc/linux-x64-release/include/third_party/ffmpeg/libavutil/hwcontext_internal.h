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

#ifndef AVUTIL_HWCONTEXT_INTERNAL_H
#define AVUTIL_HWCONTEXT_INTERNAL_H

#include <stddef.h>

#include "buffer.h"
#include "hwcontext.h"
#include "frame.h"
#include "pixfmt.h"

typedef struct HWContextType {
    enum AVHWDeviceType type;
    const char         *name;

    /**
     * An array of pixel formats supported by the AVHWFramesContext instances
     * Terminated by AV_PIX_FMT_NONE.
     */
    const enum AVPixelFormat *pix_fmts;

    /**
     * size of the public hardware-specific context,
     * i.e. AVHWDeviceContext.hwctx
     */
    size_t             device_hwctx_size;

    /**
     * Size of the hardware-specific device configuration.
     * (Used to query hwframe constraints.)
     */
    size_t             device_hwconfig_size;

    /**
     * size of the public frame pool hardware-specific context,
     * i.e. AVHWFramesContext.hwctx
     */
    size_t             frames_hwctx_size;

    int              (*device_create)(AVHWDeviceContext *ctx, const char *device,
                                      AVDictionary *opts, int flags);
    int              (*device_derive)(AVHWDeviceContext *dst_ctx,
                                      AVHWDeviceContext *src_ctx,
                                      AVDictionary *opts, int flags);

    int              (*device_init)(AVHWDeviceContext *ctx);
    void             (*device_uninit)(AVHWDeviceContext *ctx);

    int              (*frames_get_constraints)(AVHWDeviceContext *ctx,
                                               const void *hwconfig,
                                               AVHWFramesConstraints *constraints);

    int              (*frames_init)(AVHWFramesContext *ctx);
    void             (*frames_uninit)(AVHWFramesContext *ctx);

    int              (*frames_get_buffer)(AVHWFramesContext *ctx, AVFrame *frame);
    int              (*transfer_get_formats)(AVHWFramesContext *ctx,
                                             enum AVHWFrameTransferDirection dir,
                                             enum AVPixelFormat **formats);
    int              (*transfer_data_to)(AVHWFramesContext *ctx, AVFrame *dst,
                                         const AVFrame *src);
    int              (*transfer_data_from)(AVHWFramesContext *ctx, AVFrame *dst,
                                           const AVFrame *src);

    int              (*map_to)(AVHWFramesContext *ctx, AVFrame *dst,
                               const AVFrame *src, int flags);
    int              (*map_from)(AVHWFramesContext *ctx, AVFrame *dst,
                                 const AVFrame *src, int flags);

    int              (*frames_derive_to)(AVHWFramesContext *dst_ctx,
                                         AVHWFramesContext *src_ctx, int flags);
    int              (*frames_derive_from)(AVHWFramesContext *dst_ctx,
                                           AVHWFramesContext *src_ctx, int flags);
} HWContextType;

typedef struct FFHWFramesContext {
    /**
     * The public AVHWFramesContext. See hwcontext.h for it.
     */
    AVHWFramesContext p;

    const HWContextType *hw_type;

    AVBufferPool *pool_internal;

    /**
     * For a derived context, a reference to the original frames
     * context it was derived from.
     */
    AVBufferRef *source_frames;
    /**
     * Flags to apply to the mapping from the source to the derived
     * frame context when trying to allocate in the derived context.
     */
    int source_allocation_map_flags;
} FFHWFramesContext;

static inline FFHWFramesContext *ffhwframesctx(AVHWFramesContext *ctx)
{
    return (FFHWFramesContext*)ctx;
}

typedef struct HWMapDescriptor {
    /**
     * A reference to the original source of the mapping.
     */
    AVFrame *source;
    /**
     * A reference to the hardware frames context in which this
     * mapping was made.  May be the same as source->hw_frames_ctx,
     * but need not be.
     */
    AVBufferRef *hw_frames_ctx;
    /**
     * Unmap function.
     */
    void (*unmap)(AVHWFramesContext *ctx,
                  struct HWMapDescriptor *hwmap);
    /**
     * Hardware-specific private data associated with the mapping.
     */
    void          *priv;
} HWMapDescriptor;

int ff_hwframe_map_create(AVBufferRef *hwframe_ref,
                          AVFrame *dst, const AVFrame *src,
                          void (*unmap)(AVHWFramesContext *ctx,
                                        HWMapDescriptor *hwmap),
                          void *priv);

/**
 * Replace the current hwmap of dst with the one from src, used for indirect
 * mappings like VAAPI->(DRM)->OpenCL/Vulkan where a direct interop is missing
 */
int ff_hwframe_map_replace(AVFrame *dst, const AVFrame *src);

extern const HWContextType ff_hwcontext_type_cuda;
extern const HWContextType ff_hwcontext_type_d3d11va;
extern const HWContextType ff_hwcontext_type_d3d12va;
extern const HWContextType ff_hwcontext_type_drm;
extern const HWContextType ff_hwcontext_type_dxva2;
extern const HWContextType ff_hwcontext_type_opencl;
extern const HWContextType ff_hwcontext_type_qsv;
extern const HWContextType ff_hwcontext_type_vaapi;
extern const HWContextType ff_hwcontext_type_vdpau;
extern const HWContextType ff_hwcontext_type_videotoolbox;
extern const HWContextType ff_hwcontext_type_mediacodec;
extern const HWContextType ff_hwcontext_type_vulkan;
extern const HWContextType ff_hwcontext_type_amf;

#endif /* AVUTIL_HWCONTEXT_INTERNAL_H */
