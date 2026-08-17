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

#ifndef AVUTIL_HWCONTEXT_VULKAN_H
#define AVUTIL_HWCONTEXT_VULKAN_H

#include <vulkan/vulkan.h>

#include "pixfmt.h"
#include "frame.h"

/**
 * @file
 * API-specific header for AV_HWDEVICE_TYPE_VULKAN.
 *
 * For user-allocated pools, AVHWFramesContext.pool must return AVBufferRefs
 * with the data pointer set to an AVVkFrame.
 */

/**
 * Main Vulkan context, allocated as AVHWDeviceContext.hwctx.
 * All of these can be set before init to change what the context uses
 */
typedef struct AVVulkanDeviceContext {
    /**
     * Custom memory allocator, else NULL
     */
    const VkAllocationCallbacks *alloc;
    /**
     * Vulkan instance. Must be at least version 1.1.
     */
    VkInstance inst;
    /**
     * Physical device
     */
    VkPhysicalDevice phys_dev;
    /**
     * Active device
     */
    VkDevice act_dev;
    /**
     * Queue family index for graphics
     * @note av_hwdevice_create() will set all 3 queue indices if unset
     * If there is no dedicated queue for compute or transfer operations,
     * they will be set to the graphics queue index which can handle both.
     * nb_graphics_queues indicates how many queues were enabled for the
     * graphics queue (must be at least 1)
     */
    int queue_family_index;
    int nb_graphics_queues;
    /**
     * Queue family index to use for transfer operations, and the amount of queues
     * enabled. In case there is no dedicated transfer queue, nb_tx_queues
     * must be 0 and queue_family_tx_index must be the same as either the graphics
     * queue or the compute queue, if available.
     */
    int queue_family_tx_index;
    int nb_tx_queues;
    /**
     * Queue family index for compute ops, and the amount of queues enabled.
     * In case there are no dedicated compute queues, nb_comp_queues must be
     * 0 and its queue family index must be set to the graphics queue.
     */
    int queue_family_comp_index;
    int nb_comp_queues;
    /**
     * Enabled instance extensions.
     * If supplying your own device context, set this to an array of strings, with
     * each entry containing the specified Vulkan extension string to enable.
     * Duplicates are possible and accepted.
     * If no extensions are enabled, set these fields to NULL, and 0 respectively.
     */
    const char * const *enabled_inst_extensions;
    int nb_enabled_inst_extensions;
    /**
     * Enabled device extensions. By default, VK_KHR_external_memory_fd,
     * VK_EXT_external_memory_dma_buf, VK_EXT_image_drm_format_modifier,
     * VK_KHR_external_semaphore_fd and VK_EXT_external_memory_host are enabled if found.
     * If supplying your own device context, these fields takes the same format as
     * the above fields, with the same conditions that duplicates are possible
     * and accepted, and that NULL and 0 respectively means no extensions are enabled.
     */
    const char * const *enabled_dev_extensions;
    int nb_enabled_dev_extensions;
    /**
     * This structure should be set to the set of features that present and enabled
     * during device creation. When a device is created by FFmpeg, it will default to
     * enabling all that are present of the shaderImageGatherExtended,
     * fragmentStoresAndAtomics, shaderInt64 and vertexPipelineStoresAndAtomics features.
     */
    VkPhysicalDeviceFeatures2 device_features;
} AVVulkanDeviceContext;

/**
 * Allocated as AVHWFramesContext.hwctx, used to set pool-specific options
 */
typedef struct AVVulkanFramesContext {
    /**
     * Controls the tiling of allocated frames.
     */
    VkImageTiling tiling;
    /**
     * Defines extra usage of output frames. If left as 0, the following bits
     * are set: TRANSFER_SRC, TRANSFER_DST. SAMPLED and STORAGE.
     */
    VkImageUsageFlagBits usage;
    /**
     * Extension data for image creation.
     */
    void *create_pnext;
    /**
     * Extension data for memory allocation. Must have as many entries as
     * the number of planes of the sw_format.
     * This will be chained to VkExportMemoryAllocateInfo, which is used
     * to make all pool images exportable to other APIs if the necessary
     * extensions are present in enabled_dev_extensions.
     */
    void *alloc_pnext[AV_NUM_DATA_POINTERS];
} AVVulkanFramesContext;

/*
 * Frame structure, the VkFormat of the image will always match
 * the pool's sw_format.
 * All frames, imported or allocated, will be created with the
 * VK_IMAGE_CREATE_ALIAS_BIT flag set, so the memory may be aliased if needed.
 *
 * If all three queue family indices in the device context are the same,
 * images will be created with the EXCLUSIVE sharing mode. Otherwise, all images
 * will be created using the CONCURRENT sharing mode.
 *
 * @note the size of this structure is not part of the ABI, to allocate
 * you must use @av_vk_frame_alloc().
 */
typedef struct AVVkFrame {
    /**
     * Vulkan images to which the memory is bound to.
     */
    VkImage img[AV_NUM_DATA_POINTERS];

    /**
     * The same tiling must be used for all images in the frame.
     */
    VkImageTiling tiling;

    /**
     * Memory backing the images. Could be less than the amount of images
     * if importing from a DRM or VAAPI frame.
     */
    VkDeviceMemory mem[AV_NUM_DATA_POINTERS];
    size_t size[AV_NUM_DATA_POINTERS];

    /**
     * OR'd flags for all memory allocated
     */
    VkMemoryPropertyFlagBits flags;

    /**
     * Updated after every barrier
     */
    VkAccessFlagBits access[AV_NUM_DATA_POINTERS];
    VkImageLayout layout[AV_NUM_DATA_POINTERS];

    /**
     * Synchronization semaphores. Must not be freed manually. Must be waited on
     * and signalled at every queue submission.
     * Could be less than the amount of images: either one per VkDeviceMemory
     * or one for the entire frame. All others will be set to VK_NULL_HANDLE.
     */
    VkSemaphore sem[AV_NUM_DATA_POINTERS];

    /**
     * Internal data.
     */
    struct AVVkFrameInternal *internal;
} AVVkFrame;

/**
 * Allocates a single AVVkFrame and initializes everything as 0.
 * @note Must be freed via av_free()
 */
AVVkFrame *av_vk_frame_alloc(void);

/**
 * Returns the format of each image up to the number of planes for a given sw_format.
 * Returns NULL on unsupported formats.
 */
const VkFormat *av_vkfmt_from_pixfmt(enum AVPixelFormat p);

#endif /* AVUTIL_HWCONTEXT_VULKAN_H */
