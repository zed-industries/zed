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

#ifndef AVCODEC_VULKAN_DECODE_H
#define AVCODEC_VULKAN_DECODE_H

#include "codec_id.h"
#include "decode.h"
#include "hwaccel_internal.h"
#include "internal.h"

#include "vulkan_video.h"

typedef struct FFVulkanDecodeDescriptor {
    enum AVCodecID                   codec_id;
    FFVulkanExtensions               decode_extension;
    VkQueueFlagBits                  queue_flags;
    VkVideoCodecOperationFlagBitsKHR decode_op;

    VkExtensionProperties ext_props;
} FFVulkanDecodeDescriptor;

typedef struct FFVulkanDecodeProfileData {
    VkVideoDecodeH264ProfileInfoKHR h264_profile;
    VkVideoDecodeH265ProfileInfoKHR h265_profile;
    VkVideoDecodeAV1ProfileInfoKHR av1_profile;
    VkVideoDecodeUsageInfoKHR usage;
    VkVideoProfileInfoKHR profile;
    VkVideoProfileListInfoKHR profile_list;
} FFVulkanDecodeProfileData;

typedef struct FFVulkanDecodeShared {
    FFVulkanContext s;
    FFVkVideoCommon common;
    AVVulkanDeviceQueueFamily *qf;
    FFVkExecPool exec_pool;

    AVBufferPool *buf_pool;

    VkVideoCapabilitiesKHR caps;
    VkVideoDecodeCapabilitiesKHR dec_caps;

    VkVideoSessionParametersKHR empty_session_params;

    /* Software-defined decoder context */
    void *sd_ctx;
    void (*sd_ctx_free)(struct FFVulkanDecodeShared *ctx);
} FFVulkanDecodeShared;

typedef struct FFVulkanDecodeContext {
    FFVulkanDecodeShared *shared_ctx;
    AVBufferRef *session_params;

    int dedicated_dpb; /* Oddity  #1 - separate DPB images */
    int external_fg;   /* Oddity  #2 - hardware can't apply film grain */
    uint32_t frame_id_alloc_mask; /* For AV1 only */

    /* Workaround for NVIDIA drivers tested with CTS version 1.3.8 for AV1.
     * The tests were incorrect as the OrderHints were offset by 1. */
    int quirk_av1_offset;

    /* Thread-local state below */
    struct HEVCHeaderSet *hevc_headers;
    size_t hevc_headers_size;

    uint32_t                       *slice_off;
    unsigned int                    slice_off_max;
} FFVulkanDecodeContext;

typedef struct FFVulkanDecodePicture {
    AVFrame                        *dpb_frame;      /* Only used for out-of-place decoding. */

    struct {
        VkImageView                     ref[AV_NUM_DATA_POINTERS];        /* Image representation view (reference) */
        VkImageView                     out[AV_NUM_DATA_POINTERS];        /* Image representation view (output-only) */
        VkImageView                     dst[AV_NUM_DATA_POINTERS];        /* Set to img_view_out if no layered refs are used */
        VkImageAspectFlags              aspect[AV_NUM_DATA_POINTERS];     /* Image plane mask bits */
        VkImageAspectFlags              aspect_ref[AV_NUM_DATA_POINTERS]; /* Only used for out-of-place decoding */
    } view;

    VkSemaphore                     sem;
    uint64_t                        sem_value;

    /* Current picture */
    VkVideoPictureResourceInfoKHR   ref;
    VkVideoReferenceSlotInfoKHR     ref_slot;

    /* Picture refs. H264 has the maximum number of refs (36) of any supported codec. */
    VkVideoPictureResourceInfoKHR   refs     [36];
    VkVideoReferenceSlotInfoKHR     ref_slots[36];

    /* Main decoding struct */
    VkVideoDecodeInfoKHR            decode_info;

    /* Slice data */
    AVBufferRef                    *slices_buf;
    size_t                          slices_size;

    /* Vulkan functions needed for destruction, as no other context is guaranteed to exist */
    PFN_vkWaitSemaphores            wait_semaphores;
    PFN_vkDestroyImageView          destroy_image_view;
} FFVulkanDecodePicture;

/**
 * Initialize decoder.
 */
int ff_vk_decode_init(AVCodecContext *avctx);

/**
 * Synchronize the contexts between 2 threads.
 */
int ff_vk_update_thread_context(AVCodecContext *dst, const AVCodecContext *src);

/**
 * Initialize hw_frames_ctx with the parameters needed to decode the stream
 * using the parameters from avctx.
 *
 * NOTE: if avctx->internal->hwaccel_priv_data exists, will partially initialize
 * the context.
 */
int ff_vk_frame_params(AVCodecContext *avctx, AVBufferRef *hw_frames_ctx);

/**
 * Removes current session parameters to recreate them
 */
int ff_vk_params_invalidate(AVCodecContext *avctx, int t, const uint8_t *b, uint32_t s);

/**
 * Prepare a frame, creates the image view, and sets up the dpb fields.
 */
int ff_vk_decode_prepare_frame(FFVulkanDecodeContext *dec, AVFrame *pic,
                               FFVulkanDecodePicture *vkpic, int is_current,
                               int alloc_dpb);

/**
 * Software-defined decoder version of ff_vk_decode_prepare_frame.
 */
int ff_vk_decode_prepare_frame_sdr(FFVulkanDecodeContext *dec, AVFrame *pic,
                                   FFVulkanDecodePicture *vkpic, int is_current,
                                   enum FFVkShaderRepFormat rep_fmt, int alloc_dpb);

/**
 * Add slice data to frame.
 */
int ff_vk_decode_add_slice(AVCodecContext *avctx, FFVulkanDecodePicture *vp,
                           const uint8_t *data, size_t size, int add_startcode,
                           uint32_t *nb_slices, const uint32_t **offsets);

/**
 * Decode a frame.
 */
int ff_vk_decode_frame(AVCodecContext *avctx,
                       AVFrame *pic,    FFVulkanDecodePicture *vp,
                       AVFrame *rpic[], FFVulkanDecodePicture *rvkp[]);

/**
 * Free a frame and its state.
 */
void ff_vk_decode_free_frame(AVHWDeviceContext *dev_ctx, FFVulkanDecodePicture *vp);

/**
 * Get an FFVkBuffer suitable for decoding from.
 */
int ff_vk_get_decode_buffer(FFVulkanDecodeContext *ctx, AVBufferRef **buf,
                            void *create_pNext, size_t size);

/**
 * Create VkVideoSessionParametersKHR wrapped in an AVBufferRef.
 */
int ff_vk_decode_create_params(AVBufferRef **par_ref, void *logctx, FFVulkanDecodeShared *ctx,
                               const VkVideoSessionParametersCreateInfoKHR *session_params_create);

/**
 * Flush decoder.
 */
void ff_vk_decode_flush(AVCodecContext *avctx);

/**
 * Free decoder.
 */
int ff_vk_decode_uninit(AVCodecContext *avctx);

#endif /* AVCODEC_VULKAN_DECODE_H */
