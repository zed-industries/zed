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

#ifndef AVUTIL_VULKAN_FUNCTIONS_H
#define AVUTIL_VULKAN_FUNCTIONS_H

#define VK_NO_PROTOTYPES
#define VK_ENABLE_BETA_EXTENSIONS

#include "hwcontext.h"
#include "hwcontext_vulkan.h"

/* An enum of bitflags for every optional extension we need */
typedef uint64_t FFVulkanExtensions;

#define FF_VK_EXT_EXTERNAL_DMABUF_MEMORY (1ULL <<  0) /* VK_EXT_external_memory_dma_buf */
#define FF_VK_EXT_DRM_MODIFIER_FLAGS     (1ULL <<  1) /* VK_EXT_image_drm_format_modifier */
#define FF_VK_EXT_EXTERNAL_FD_MEMORY     (1ULL <<  2) /* VK_KHR_external_memory_fd */
#define FF_VK_EXT_EXTERNAL_FD_SEM        (1ULL <<  3) /* VK_KHR_external_semaphore_fd */
#define FF_VK_EXT_EXTERNAL_HOST_MEMORY   (1ULL <<  4) /* VK_EXT_external_memory_host */
#define FF_VK_EXT_DEBUG_UTILS            (1ULL <<  5) /* VK_EXT_debug_utils */

#define FF_VK_EXT_EXTERNAL_WIN32_MEMORY  (1ULL <<  6) /* VK_KHR_external_memory_win32 */
#define FF_VK_EXT_EXTERNAL_WIN32_SEM     (1ULL <<  7) /* VK_KHR_external_semaphore_win32 */

#define FF_VK_EXT_DESCRIPTOR_BUFFER      (1ULL <<  8) /* VK_EXT_descriptor_buffer */
#define FF_VK_EXT_DEVICE_DRM             (1ULL <<  9) /* VK_EXT_physical_device_drm */
#define FF_VK_EXT_ATOMIC_FLOAT           (1ULL << 10) /* VK_EXT_shader_atomic_float */
#define FF_VK_EXT_COOP_MATRIX            (1ULL << 11) /* VK_KHR_cooperative_matrix */
#define FF_VK_EXT_OPTICAL_FLOW           (1ULL << 12) /* VK_NV_optical_flow */
#define FF_VK_EXT_SHADER_OBJECT          (1ULL << 13) /* VK_EXT_shader_object */
#define FF_VK_EXT_PUSH_DESCRIPTOR        (1ULL << 14) /* VK_KHR_push_descriptor */
#define FF_VK_EXT_RELAXED_EXTENDED_INSTR (1ULL << 15) /* VK_KHR_shader_relaxed_extended_instruction */

/* Video extensions */
#define FF_VK_EXT_VIDEO_QUEUE            (1ULL << 36) /* VK_KHR_video_queue */
#define FF_VK_EXT_VIDEO_MAINTENANCE_1    (1ULL << 37) /* VK_KHR_video_maintenance1 */
#define FF_VK_EXT_VIDEO_MAINTENANCE_2    (1ULL << 38) /* VK_KHR_video_maintenance2 */

#define FF_VK_EXT_VIDEO_DECODE_QUEUE     (1ULL << 40) /* VK_KHR_video_decode_queue */
#define FF_VK_EXT_VIDEO_DECODE_H264      (1ULL << 41) /* VK_KHR_video_decode_h264 */
#define FF_VK_EXT_VIDEO_DECODE_H265      (1ULL << 42) /* VK_KHR_video_decode_h265 */
#define FF_VK_EXT_VIDEO_DECODE_AV1       (1ULL << 43) /* VK_KHR_video_decode_av1 */

#define FF_VK_EXT_VIDEO_ENCODE_QUEUE     (1ULL << 50) /* VK_KHR_video_encode_queue */
#define FF_VK_EXT_VIDEO_ENCODE_H264      (1ULL << 51) /* VK_KHR_video_encode_h264 */
#define FF_VK_EXT_VIDEO_ENCODE_H265      (1ULL << 52) /* VK_KHR_video_encode_h265 */

#define FF_VK_EXT_PORTABILITY_SUBSET     (1ULL << 62)
#define FF_VK_EXT_NO_FLAG                (1ULL << 63)

/* Macro containing every function that we utilize in our codebase */
#define FN_LIST(MACRO)                                                                   \
    /* Instance */                                                                       \
    MACRO(0, 0, FF_VK_EXT_NO_FLAG,              EnumerateInstanceExtensionProperties)    \
    MACRO(0, 0, FF_VK_EXT_NO_FLAG,              EnumerateInstanceLayerProperties)        \
    MACRO(0, 0, FF_VK_EXT_NO_FLAG,              CreateInstance)                          \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              DestroyInstance)                         \
                                                                                         \
    /* Debug */                                                                          \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              CreateDebugUtilsMessengerEXT)            \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              DestroyDebugUtilsMessengerEXT)           \
                                                                                         \
    /* Device */                                                                         \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              GetDeviceProcAddr)                       \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              CreateDevice)                            \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              GetPhysicalDeviceFeatures2)              \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              GetPhysicalDeviceProperties)             \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              GetPhysicalDeviceExternalSemaphoreProperties)          \
    MACRO(1, 0, FF_VK_EXT_VIDEO_QUEUE,          GetPhysicalDeviceVideoCapabilitiesKHR)     \
    MACRO(1, 0, FF_VK_EXT_VIDEO_QUEUE,          GetPhysicalDeviceVideoFormatPropertiesKHR) \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              DeviceWaitIdle)                          \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              DestroyDevice)                           \
                                                                                         \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              EnumeratePhysicalDevices)                \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              EnumerateDeviceExtensionProperties)      \
                                                                                         \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              GetPhysicalDeviceProperties2)            \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              GetPhysicalDeviceMemoryProperties)       \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              GetPhysicalDeviceFormatProperties2)      \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              GetPhysicalDeviceImageFormatProperties2) \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              GetPhysicalDeviceQueueFamilyProperties)  \
    MACRO(1, 0, FF_VK_EXT_NO_FLAG,              GetPhysicalDeviceQueueFamilyProperties2) \
    MACRO(1, 0, FF_VK_EXT_COOP_MATRIX,          GetPhysicalDeviceCooperativeMatrixPropertiesKHR) \
                                                                                         \
    /* Command pool */                                                                   \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CreateCommandPool)                       \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              DestroyCommandPool)                      \
                                                                                         \
    /* Command buffer */                                                                 \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              AllocateCommandBuffers)                  \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              BeginCommandBuffer)                      \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              EndCommandBuffer)                        \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              FreeCommandBuffers)                      \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CmdDispatch)                             \
                                                                                         \
    /* Queue */                                                                          \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              GetDeviceQueue)                          \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              QueueSubmit)                             \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              QueueSubmit2)                              \
                                                                                         \
    /* Fences */                                                                         \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CreateFence)                             \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              WaitForFences)                           \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              GetFenceStatus)                                        \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              ResetFences)                             \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              DestroyFence)                            \
                                                                                         \
    /* Semaphores */                                                                     \
    MACRO(1, 1, FF_VK_EXT_EXTERNAL_FD_SEM,      GetSemaphoreFdKHR)                       \
    MACRO(1, 1, FF_VK_EXT_EXTERNAL_FD_SEM,      ImportSemaphoreFdKHR)                                  \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CreateSemaphore)                         \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              WaitSemaphores)                          \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              DestroySemaphore)                        \
                                                                                         \
    /* Memory */                                                                         \
    MACRO(1, 1, FF_VK_EXT_EXTERNAL_FD_MEMORY,   GetMemoryFdKHR)                          \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              GetMemoryFdPropertiesKHR)                \
    MACRO(1, 1, FF_VK_EXT_EXTERNAL_HOST_MEMORY, GetMemoryHostPointerPropertiesEXT)       \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              AllocateMemory)                          \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              MapMemory)                               \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              FlushMappedMemoryRanges)                 \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              InvalidateMappedMemoryRanges)            \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              UnmapMemory)                             \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              FreeMemory)                              \
                                                                                         \
    /* Commands */                                                                       \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CmdBindDescriptorSets)                   \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CmdPushConstants)                        \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CmdBindPipeline)                         \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CmdPipelineBarrier)                      \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CmdCopyBufferToImage)                    \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CmdCopyImageToBuffer)                    \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CmdCopyBuffer)                                         \
                                                                                         \
    /* Buffer */                                                                         \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              GetBufferMemoryRequirements2)            \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CreateBuffer)                            \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              BindBufferMemory)                        \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              GetBufferDeviceAddress)                    \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CmdFillBuffer)                             \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              DestroyBuffer)                           \
                                                                                         \
    /* Image */                                                                          \
    MACRO(1, 1, FF_VK_EXT_DRM_MODIFIER_FLAGS,   GetImageDrmFormatModifierPropertiesEXT)  \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              GetImageMemoryRequirements2)             \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CreateImage)                             \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              BindImageMemory2)                        \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              GetImageSubresourceLayout)               \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              DestroyImage)                            \
                                                                                         \
    /* ImageView */                                                                      \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CreateImageView)                         \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              DestroyImageView)                        \
                                                                                         \
    /* DescriptorSet */                                                                  \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CreateDescriptorSetLayout)               \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              AllocateDescriptorSets)                  \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CreateDescriptorPool)                    \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              DestroyDescriptorPool)                   \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              DestroyDescriptorSetLayout)              \
                                                                                         \
    /* Descriptor buffers */                                                               \
    MACRO(1, 1, FF_VK_EXT_DESCRIPTOR_BUFFER,    GetDescriptorSetLayoutSizeEXT)             \
    MACRO(1, 1, FF_VK_EXT_DESCRIPTOR_BUFFER,    GetDescriptorSetLayoutBindingOffsetEXT)    \
    MACRO(1, 1, FF_VK_EXT_DESCRIPTOR_BUFFER,    GetDescriptorEXT)                          \
    MACRO(1, 1, FF_VK_EXT_DESCRIPTOR_BUFFER,    CmdBindDescriptorBuffersEXT)               \
    MACRO(1, 1, FF_VK_EXT_DESCRIPTOR_BUFFER,    CmdSetDescriptorBufferOffsetsEXT)          \
                                                                                           \
    /* DescriptorUpdateTemplate */                                                       \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              UpdateDescriptorSetWithTemplate)         \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CreateDescriptorUpdateTemplate)          \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              DestroyDescriptorUpdateTemplate)         \
                                                                                                       \
    /* Descriptors */                                                                                  \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              UpdateDescriptorSets)                                  \
    MACRO(1, 1, FF_VK_EXT_PUSH_DESCRIPTOR,      CmdPushDescriptorSetKHR)                               \
                                                                                           \
    /* Queries */                                                                          \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CreateQueryPool)                           \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              GetQueryPoolResults)                       \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              ResetQueryPool)                            \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CmdBeginQuery)                             \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CmdEndQuery)                               \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CmdResetQueryPool)                         \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              DestroyQueryPool)                          \
                                                                                           \
    /* sync2 */                                                                            \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CmdPipelineBarrier2)                       \
                                                                                           \
    /* Video queue */                                                                      \
    MACRO(1, 1, FF_VK_EXT_VIDEO_QUEUE,          CreateVideoSessionKHR)                     \
    MACRO(1, 1, FF_VK_EXT_VIDEO_QUEUE,          CreateVideoSessionParametersKHR)           \
    MACRO(1, 1, FF_VK_EXT_VIDEO_QUEUE,          GetVideoSessionMemoryRequirementsKHR)      \
    MACRO(1, 1, FF_VK_EXT_VIDEO_QUEUE,          BindVideoSessionMemoryKHR)                 \
    MACRO(1, 1, FF_VK_EXT_VIDEO_QUEUE,          CmdBeginVideoCodingKHR)                    \
    MACRO(1, 1, FF_VK_EXT_VIDEO_QUEUE,          CmdControlVideoCodingKHR)                  \
    MACRO(1, 1, FF_VK_EXT_VIDEO_QUEUE,          CmdEndVideoCodingKHR)                      \
    MACRO(1, 1, FF_VK_EXT_VIDEO_QUEUE,          DestroyVideoSessionParametersKHR)          \
    MACRO(1, 1, FF_VK_EXT_VIDEO_QUEUE,          DestroyVideoSessionKHR)                    \
                                                                                           \
    /* Video decoding */                                                                   \
    MACRO(1, 1, FF_VK_EXT_VIDEO_DECODE_QUEUE,   CmdDecodeVideoKHR)                         \
                                                                                           \
    /* Video encoding */                                                                   \
    MACRO(1, 1, FF_VK_EXT_VIDEO_ENCODE_QUEUE,   CmdEncodeVideoKHR)                         \
    MACRO(1, 1, FF_VK_EXT_VIDEO_ENCODE_QUEUE,   GetEncodedVideoSessionParametersKHR)       \
    MACRO(1, 0, FF_VK_EXT_VIDEO_ENCODE_QUEUE,   GetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR) \
                                                                                         \
    /* Pipeline */                                                                       \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CreatePipelineLayout)                    \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              DestroyPipelineLayout)                   \
                                                                                         \
    /* PipelineLayout */                                                                 \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CreateComputePipelines)                  \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              DestroyPipeline)                         \
                                                                                         \
    /* Sampler */                                                                        \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CreateSamplerYcbcrConversion)              \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              DestroySamplerYcbcrConversion)             \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CreateSampler)                           \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              DestroySampler)                          \
                                                                                           \
    /* Optical flow */                                                                     \
    MACRO(1, 1, FF_VK_EXT_OPTICAL_FLOW,         BindOpticalFlowSessionImageNV)             \
    MACRO(1, 1, FF_VK_EXT_OPTICAL_FLOW,         CmdOpticalFlowExecuteNV)                   \
    MACRO(1, 1, FF_VK_EXT_OPTICAL_FLOW,         CreateOpticalFlowSessionNV)                \
    MACRO(1, 1, FF_VK_EXT_OPTICAL_FLOW,         DestroyOpticalFlowSessionNV)               \
    MACRO(1, 0, FF_VK_EXT_OPTICAL_FLOW,         GetPhysicalDeviceOpticalFlowImageFormatsNV)\
                                                                                         \
    /* Shaders */                                                                        \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              CreateShaderModule)                      \
    MACRO(1, 1, FF_VK_EXT_NO_FLAG,              DestroyShaderModule)                       \
    MACRO(1, 1, FF_VK_EXT_SHADER_OBJECT,        CmdBindShadersEXT)                         \
    MACRO(1, 1, FF_VK_EXT_SHADER_OBJECT,        CreateShadersEXT)                          \
    MACRO(1, 1, FF_VK_EXT_SHADER_OBJECT,        DestroyShaderEXT)                          \
    MACRO(1, 1, FF_VK_EXT_SHADER_OBJECT,        GetShaderBinaryDataEXT)

/* Macro containing every win32 specific function that we utilize in our codebase */
#define FN_LIST_WIN32(MACRO)                                                             \
    MACRO(1, 1, FF_VK_EXT_EXTERNAL_WIN32_SEM,    GetSemaphoreWin32HandleKHR)             \
    MACRO(1, 1, FF_VK_EXT_EXTERNAL_WIN32_MEMORY, GetMemoryWin32HandleKHR)

/* Macro to turn a function name into a definition */
#define PFN_DEF(req_inst, req_dev, ext_flag, name) \
    PFN_vk##name name;

/* Structure with the definition of all listed functions */
typedef struct FFVulkanFunctions {
    FN_LIST(PFN_DEF)
#ifdef _WIN32
    FN_LIST_WIN32(PFN_DEF)
#endif
} FFVulkanFunctions;

#endif /* AVUTIL_VULKAN_FUNCTIONS_H */
