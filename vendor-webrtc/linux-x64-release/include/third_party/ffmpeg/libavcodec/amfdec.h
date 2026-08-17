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

#ifndef AVCODEC_AMFDEC_H
#define AVCODEC_AMFDEC_H

#include <AMF/core/Version.h>
#include <AMF/core/Buffer.h>
#include <AMF/core/Factory.h>
#include <AMF/core/Context.h>
#include <AMF/core/Surface.h>
#include <AMF/components/Component.h>
#include <AMF/components/VideoDecoderUVD.h>

#include "avcodec.h"
#include "libavformat/avformat.h"
#include "libavutil/fifo.h"
#include "libavutil/frame.h"
#include "libavutil/opt.h"
#include "libavutil/hwcontext_amf.h"
/**
* AMF decoder context
*/
typedef struct AMFDecoderContext {
    AVClass            *avclass;
    AVBufferRef        *device_ctx_ref;

    //decoder
    AMFComponent       *decoder; ///< AMF decoder object
    AMF_SURFACE_FORMAT  format;  ///< AMF surface format

    // common decoder options
    int                 decoder_mode;
    int                 timestamp_mode;
    int                 surface_pool_size;
    int                 dpb_size;
    int                 lowlatency;
    int                 smart_access_video;
    int                 skip_transfer_sav;
    int                 drain;
    int                 resolution_changed;
    int                 copy_output;
    AVPacket*           in_pkt;
    enum AMF_SURFACE_FORMAT output_format;

} AMFDecoderContext;

#endif // AVCODEC_AMFDEC_H
