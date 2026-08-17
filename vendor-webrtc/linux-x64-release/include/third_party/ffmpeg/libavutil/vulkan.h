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

#ifndef AVUTIL_VULKAN_H
#define AVUTIL_VULKAN_H

#define VK_NO_PROTOTYPES

#include <stdatomic.h>

#include "thread.h"
#include "pixdesc.h"
#include "bprint.h"
#include "hwcontext.h"
#include "vulkan_functions.h"
#include "hwcontext_vulkan.h"

/* GLSL management macros */
#define INDENT(N) INDENT_##N
#define INDENT_0
#define INDENT_1 INDENT_0 "    "
#define INDENT_2 INDENT_1 INDENT_1
#define INDENT_3 INDENT_2 INDENT_1
#define INDENT_4 INDENT_3 INDENT_1
#define INDENT_5 INDENT_4 INDENT_1
#define INDENT_6 INDENT_5 INDENT_1
#define C(N, S)          INDENT(N) #S "\n"

#define GLSLC(N, S)                     \
    do {                                \
        av_bprintf(&shd->src, C(N, S)); \
    } while (0)

#define GLSLA(...)                          \
    do {                                    \
        av_bprintf(&shd->src, __VA_ARGS__); \
    } while (0)

#define GLSLF(N, S, ...)                             \
    do {                                             \
        av_bprintf(&shd->src, C(N, S), __VA_ARGS__); \
    } while (0)

#define GLSLD(D)                                        \
    do {                                                \
        av_bprintf(&shd->src, "\n");                    \
        av_bprint_append_data(&shd->src, D, strlen(D)); \
        av_bprintf(&shd->src, "\n");                    \
    } while (0)

/* Helper, pretty much every Vulkan return value needs to be checked */
#define RET(x)                                                                 \
    do {                                                                       \
        if ((err = (x)) < 0)                                                   \
            goto fail;                                                         \
    } while (0)

#define DUP_SAMPLER(x) { x, x, x, x }

typedef struct FFVulkanDescriptorSetBinding {
    const char         *name;
    VkDescriptorType    type;
    const char         *mem_layout;  /* Storage images (rgba8, etc.) and buffers (std430, etc.) */
    const char         *mem_quali;   /* readonly, writeonly, etc. */
    const char         *buf_content; /* For buffers */
    uint32_t            dimensions;  /* Needed for e.g. sampler%iD */
    uint32_t            elems;       /* 0 - scalar, 1 or more - vector */
    VkShaderStageFlags  stages;
    uint32_t            buf_elems;   /* Appends [buf_elems] to the contents. Avoids manually printing to a string. */
    VkSampler           samplers[4]; /* Sampler to use for all elems */
} FFVulkanDescriptorSetBinding;

typedef struct FFVkBuffer {
    VkBuffer buf;
    VkDeviceMemory mem;
    VkMemoryPropertyFlagBits flags;
    size_t size;
    VkDeviceAddress address;

    /* Local use only */
    VkPipelineStageFlags2 stage;
    VkAccessFlags2 access;

    /* Only valid when allocated via ff_vk_get_pooled_buffer with HOST_VISIBLE or
     * via ff_vk_host_map_buffer */
    uint8_t *mapped_mem;

    /* Set by ff_vk_host_map_buffer. This is the offset at which the buffer data
     * actually begins at.
     * The address and mapped_mem fields will be offset by this amount. */
    size_t virtual_offset;

    /* If host mapping, reference to the backing host memory buffer */
    AVBufferRef *host_ref;
} FFVkBuffer;

typedef struct FFVkExecContext {
    uint32_t idx;
    const struct FFVkExecPool *parent;
    pthread_mutex_t lock;
    int had_submission;

    /* Queue for the execution context */
    VkQueue queue;
    int qf;
    int qi;

    /* Command buffer for the context */
    VkCommandBuffer buf;

    /* Fence for the command buffer */
    VkFence fence;

    /* Opaque data, untouched, free to use by users */
    void *opaque;

    void *query_data;
    int query_idx;

    /* Buffer dependencies */
    AVBufferRef **buf_deps;
    int nb_buf_deps;
    unsigned int buf_deps_alloc_size;

    /* Frame dependencies */
    AVFrame **frame_deps;
    unsigned int frame_deps_alloc_size;
    int nb_frame_deps;

    /* Software frame dependencies */
    AVFrame **sw_frame_deps;
    unsigned int sw_frame_deps_alloc_size;
    int nb_sw_frame_deps;

    VkSemaphoreSubmitInfo *sem_wait;
    unsigned int sem_wait_alloc;
    int sem_wait_cnt;

    VkSemaphoreSubmitInfo *sem_sig;
    unsigned int sem_sig_alloc;
    int sem_sig_cnt;

    uint64_t **sem_sig_val_dst;
    unsigned int sem_sig_val_dst_alloc;
    int sem_sig_val_dst_cnt;

    uint8_t *frame_locked;
    unsigned int frame_locked_alloc_size;

    VkAccessFlagBits *access_dst;
    unsigned int access_dst_alloc;

    VkImageLayout *layout_dst;
    unsigned int layout_dst_alloc;

    uint32_t *queue_family_dst;
    unsigned int queue_family_dst_alloc;

    uint8_t *frame_update;
    unsigned int frame_update_alloc_size;
} FFVkExecContext;

typedef struct FFVulkanDescriptorSet {
    /* Descriptor buffer */
    VkDeviceSize layout_size;
    VkDeviceSize aligned_size; /* descriptorBufferOffsetAlignment */
    VkBufferUsageFlags usage;

    VkDescriptorSetLayoutBinding *binding;
    VkDeviceSize *binding_offset;
    int nb_bindings;

    /* Descriptor set is shared between all submissions */
    int singular;
} FFVulkanDescriptorSet;

typedef struct FFVulkanShader {
    /* Name for id/debugging purposes */
    const char *name;

    /* Shader text */
    AVBPrint src;

    /* Compute shader local group sizes */
    int lg_size[3];

    /* Shader bind point/type */
    VkPipelineStageFlags stage;
    VkPipelineBindPoint bind_point;

    /* Creation info */
    VkPipelineShaderStageRequiredSubgroupSizeCreateInfo subgroup_info;

    /* Base shader object */
    VkShaderEXT object;
    VkPipeline pipeline;

    /* Pipeline layout */
    VkPipelineLayout pipeline_layout;

    /* Push consts */
    VkPushConstantRange *push_consts;
    int push_consts_num;

    /* Descriptor sets */
    FFVulkanDescriptorSet *desc_set;
    int nb_descriptor_sets;

    /* Descriptor buffer */
    VkDescriptorSetLayout *desc_layout;
    uint32_t *bound_buffer_indices;

    /* Descriptor pool */
    int use_push;
    VkDescriptorPoolSize *desc_pool_size;
    int nb_desc_pool_size;
} FFVulkanShader;

typedef struct FFVulkanDescriptorSetData {
    /* Descriptor buffer */
    FFVkBuffer buf;
    uint8_t *desc_mem;
} FFVulkanDescriptorSetData;

typedef struct FFVulkanShaderData {
    /* Shader to which this data belongs to */
    FFVulkanShader *shd;
    int nb_descriptor_sets;

    /* Descriptor buffer */
    FFVulkanDescriptorSetData *desc_set_buf;
    VkDescriptorBufferBindingInfoEXT *desc_bind;

    /* Descriptor pools */
    VkDescriptorSet *desc_sets;
    VkDescriptorPool desc_pool;
} FFVulkanShaderData;

typedef struct FFVkExecPool {
    FFVkExecContext *contexts;
    atomic_uint_least64_t idx;

    VkCommandPool cmd_buf_pool;
    VkCommandBuffer *cmd_bufs;
    int pool_size;

    VkQueryPool query_pool;
    void *query_data;
    int query_results;
    int query_statuses;
    int query_64bit;
    int query_status_stride;
    int nb_queries;
    size_t qd_size;

    /* Registered shaders' data */
    FFVulkanShaderData *reg_shd;
    int nb_reg_shd;
} FFVkExecPool;

typedef struct FFVulkanContext {
    const AVClass *class;
    void *log_parent;

    FFVulkanFunctions     vkfn;
    FFVulkanExtensions    extensions;
    VkPhysicalDeviceProperties2 props;
    VkPhysicalDeviceVulkan11Properties props_11;
    VkPhysicalDeviceDriverProperties driver_props;
    VkPhysicalDeviceMemoryProperties mprops;
    VkPhysicalDeviceExternalMemoryHostPropertiesEXT hprops;
    VkPhysicalDeviceDescriptorBufferPropertiesEXT desc_buf_props;
    VkPhysicalDeviceSubgroupSizeControlProperties subgroup_props;
    VkPhysicalDeviceCooperativeMatrixPropertiesKHR coop_matrix_props;
    VkPhysicalDeviceOpticalFlowPropertiesNV optical_flow_props;
    VkQueueFamilyQueryResultStatusPropertiesKHR *query_props;
    VkQueueFamilyVideoPropertiesKHR *video_props;
    VkQueueFamilyProperties2 *qf_props;
    int tot_nb_qfs;

    VkCooperativeMatrixPropertiesKHR *coop_mat_props;
    uint32_t coop_mat_props_nb;

    VkPhysicalDeviceShaderAtomicFloatFeaturesEXT atomic_float_feats;
    VkPhysicalDeviceVulkan12Features feats_12;
    VkPhysicalDeviceFeatures2 feats;

    AVBufferRef           *device_ref;
    AVHWDeviceContext     *device;
    AVVulkanDeviceContext *hwctx;

    AVBufferRef           *input_frames_ref;
    AVBufferRef           *frames_ref;
    AVHWFramesContext     *frames;
    AVVulkanFramesContext *hwfc;

    uint32_t               qfs[64];
    int                    nb_qfs;

    /* Properties */
    int                 output_width;
    int                output_height;
    enum AVPixelFormat output_format;
    enum AVPixelFormat  input_format;
} FFVulkanContext;

static inline int ff_vk_count_images(AVVkFrame *f)
{
    int cnt = 0;
    while (cnt < FF_ARRAY_ELEMS(f->img) && f->img[cnt])
        cnt++;

    return cnt;
}

static inline const void *ff_vk_find_struct(const void *chain, VkStructureType stype)
{
    const VkBaseInStructure *in = chain;
    while (in) {
        if (in->sType == stype)
            return in;

        in = in->pNext;
    }

    return NULL;
}

static inline void ff_vk_link_struct(void *chain, const void *in)
{
    VkBaseOutStructure *out = chain;
    while (out->pNext)
        out = out->pNext;

    out->pNext = (void *)in;
}

/* Identity mapping - r = r, b = b, g = g, a = a */
extern const VkComponentMapping ff_comp_identity_map;

/**
 * Initializes the AVClass, in case this context is not used
 * as the main user's context.
 * May use either a frames context reference, or a device context reference.
 */
int ff_vk_init(FFVulkanContext *s, void *log_parent,
               AVBufferRef *device_ref, AVBufferRef *frames_ref);

/**
 * Converts Vulkan return values to strings
 */
const char *ff_vk_ret2str(VkResult res);

/**
 * Returns 1 if pixfmt is a usable RGB format.
 */
int ff_vk_mt_is_np_rgb(enum AVPixelFormat pix_fmt);

/**
 * Since storage images may not be swizzled, we have to do this in the
 * shader itself. This fills in a lookup table to do it.
 */
void ff_vk_set_perm(enum AVPixelFormat pix_fmt, int lut[4], int inv);

/**
 * Get the aspect flag for a plane from an image.
 */
VkImageAspectFlags ff_vk_aspect_flag(AVFrame *f, int p);

/**
 * Returns the format to use for images in shaders.
 */
enum FFVkShaderRepFormat {
    /* Native format with no conversion. May require casting. */
    FF_VK_REP_NATIVE = 0,
    /* Float conversion of the native format. */
    FF_VK_REP_FLOAT,
    /* Signed integer version of the native format */
    FF_VK_REP_INT,
    /* Unsigned integer version of the native format */
    FF_VK_REP_UINT,
};
const char *ff_vk_shader_rep_fmt(enum AVPixelFormat pix_fmt,
                                 enum FFVkShaderRepFormat rep_fmt);

/**
 * Loads props/mprops/driver_props
 */
int ff_vk_load_props(FFVulkanContext *s);

/**
 * Chooses an appropriate QF.
 */
AVVulkanDeviceQueueFamily *ff_vk_qf_find(FFVulkanContext *s,
                                         VkQueueFlagBits dev_family,
                                         VkVideoCodecOperationFlagBitsKHR vid_ops);

/**
 * Allocates/frees an execution pool.
 * If used in a multi-threaded context, there must be at least as many contexts
 * as there are threads.
 * ff_vk_exec_pool_init_desc() MUST be called if ff_vk_exec_descriptor_set_add()
 * has been called.
 */
int ff_vk_exec_pool_init(FFVulkanContext *s, AVVulkanDeviceQueueFamily *qf,
                         FFVkExecPool *pool, int nb_contexts,
                         int nb_queries, VkQueryType query_type, int query_64bit,
                         const void *query_create_pnext);
void ff_vk_exec_pool_free(FFVulkanContext *s, FFVkExecPool *pool);

/**
 * Retrieve an execution pool. Threadsafe.
 */
FFVkExecContext *ff_vk_exec_get(FFVulkanContext *s, FFVkExecPool *pool);

/**
 * Performs nb_queries queries and returns their results and statuses.
 * 64_BIT and WITH_STATUS flags are ignored as 64_BIT must be specified via
 * query_64bit in ff_vk_exec_pool_init() and WITH_STATUS is always enabled.
 */
VkResult ff_vk_exec_get_query(FFVulkanContext *s, FFVkExecContext *e,
                              void **data, VkQueryResultFlagBits flags);

/**
 * Start/submit/wait an execution.
 * ff_vk_exec_start() always waits on a submission, so using ff_vk_exec_wait()
 * is not necessary (unless using it is just better).
 */
int ff_vk_exec_start(FFVulkanContext *s, FFVkExecContext *e);
int ff_vk_exec_submit(FFVulkanContext *s, FFVkExecContext *e);
void ff_vk_exec_wait(FFVulkanContext *s, FFVkExecContext *e);

/**
 * Execution dependency management.
 * Can attach buffers to executions that will only be unref'd once the
 * buffer has finished executing.
 * Adding a frame dep will *lock the frame*, until either the dependencies
 * are discarded, the execution is submitted, or a failure happens.
 * update_frame will update the frame's properties before it is unlocked,
 * only if submission was successful.
 */
int ff_vk_exec_add_dep_buf(FFVulkanContext *s, FFVkExecContext *e,
                           AVBufferRef **deps, int nb_deps, int ref);
int ff_vk_exec_add_dep_wait_sem(FFVulkanContext *s, FFVkExecContext *e,
                                VkSemaphore sem, uint64_t val,
                                VkPipelineStageFlagBits2 stage);
int ff_vk_exec_add_dep_bool_sem(FFVulkanContext *s, FFVkExecContext *e,
                                VkSemaphore *sem, int nb,
                                VkPipelineStageFlagBits2 stage,
                                int wait); /* Ownership transferred if !wait */
int ff_vk_exec_add_dep_frame(FFVulkanContext *s, FFVkExecContext *e, AVFrame *f,
                             VkPipelineStageFlagBits2 wait_stage,
                             VkPipelineStageFlagBits2 signal_stage);
int ff_vk_exec_add_dep_sw_frame(FFVulkanContext *s, FFVkExecContext *e,
                                AVFrame *f);
void ff_vk_exec_update_frame(FFVulkanContext *s, FFVkExecContext *e, AVFrame *f,
                             VkImageMemoryBarrier2 *bar, uint32_t *nb_img_bar);
int ff_vk_exec_mirror_sem_value(FFVulkanContext *s, FFVkExecContext *e,
                                VkSemaphore *dst, uint64_t *dst_val,
                                AVFrame *f);
void ff_vk_exec_discard_deps(FFVulkanContext *s, FFVkExecContext *e);

/**
 * Create a single imageview for a given plane.
 */
int ff_vk_create_imageview(FFVulkanContext *s,
                           VkImageView *img_view, VkImageAspectFlags *aspect,
                           AVFrame *f, int plane, enum FFVkShaderRepFormat rep_fmt);

/**
 * Create an imageview and add it as a dependency to an execution.
 */
int ff_vk_create_imageviews(FFVulkanContext *s, FFVkExecContext *e,
                            VkImageView views[AV_NUM_DATA_POINTERS],
                            AVFrame *f, enum FFVkShaderRepFormat rep_fmt);

void ff_vk_frame_barrier(FFVulkanContext *s, FFVkExecContext *e,
                         AVFrame *pic, VkImageMemoryBarrier2 *bar, int *nb_bar,
                         VkPipelineStageFlags src_stage,
                         VkPipelineStageFlags dst_stage,
                         VkAccessFlagBits     new_access,
                         VkImageLayout        new_layout,
                         uint32_t             new_qf);

/**
 * Memory/buffer/image allocation helpers.
 */
int ff_vk_alloc_mem(FFVulkanContext *s, VkMemoryRequirements *req,
                    VkMemoryPropertyFlagBits req_flags, void *alloc_extension,
                    VkMemoryPropertyFlagBits *mem_flags, VkDeviceMemory *mem);
int ff_vk_create_buf(FFVulkanContext *s, FFVkBuffer *buf, size_t size,
                     void *pNext, void *alloc_pNext,
                     VkBufferUsageFlags usage, VkMemoryPropertyFlagBits flags);

/**
 * Buffer management code.
 */
int ff_vk_map_buffers(FFVulkanContext *s, FFVkBuffer **buf, uint8_t *mem[],
                      int nb_buffers, int invalidate);
int ff_vk_unmap_buffers(FFVulkanContext *s, FFVkBuffer **buf, int nb_buffers,
                        int flush);

static inline int ff_vk_map_buffer(FFVulkanContext *s, FFVkBuffer *buf, uint8_t **mem,
                                   int invalidate)
{
    return ff_vk_map_buffers(s, (FFVkBuffer *[]){ buf }, mem,
                             1, invalidate);
}

static inline int ff_vk_unmap_buffer(FFVulkanContext *s, FFVkBuffer *buf, int flush)
{
    return ff_vk_unmap_buffers(s, (FFVkBuffer *[]){ buf }, 1, flush);
}

void ff_vk_free_buf(FFVulkanContext *s, FFVkBuffer *buf);

/** Initialize a pool and create AVBufferRefs containing FFVkBuffer.
 * Threadsafe to use. Buffers are automatically mapped on creation if
 * VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT is set in mem_props. Users should
 * synchronize access themselvesd. Mainly meant for device-local buffers. */
int ff_vk_get_pooled_buffer(FFVulkanContext *ctx, AVBufferPool **buf_pool,
                            AVBufferRef **buf, VkBufferUsageFlags usage,
                            void *create_pNext, size_t size,
                            VkMemoryPropertyFlagBits mem_props);

/** Maps a system RAM buffer into a Vulkan buffer.
 * References the source buffer.
 */
int ff_vk_host_map_buffer(FFVulkanContext *s, AVBufferRef **dst,
                          uint8_t *src_data, const AVBufferRef *src_buf,
                          VkBufferUsageFlags usage);

/**
 * Create a sampler.
 */
int ff_vk_init_sampler(FFVulkanContext *s, VkSampler *sampler,
                       int unnorm_coords, VkFilter filt);

/**
 * Initialize a shader object, with a specific set of extensions, type+bind,
 * local group size, and subgroup requirements.
 */
int ff_vk_shader_init(FFVulkanContext *s, FFVulkanShader *shd, const char *name,
                      VkPipelineStageFlags stage,
                      const char *extensions[], int nb_extensions,
                      int lg_x, int lg_y, int lg_z,
                      uint32_t required_subgroup_size);

/**
 * Output the shader code as logging data, with a specific
 * priority.
 */
void ff_vk_shader_print(void *ctx, FFVulkanShader *shd, int prio);

/**
 * Link a shader into an executable.
 */
int ff_vk_shader_link(FFVulkanContext *s, FFVulkanShader *shd,
                      uint8_t *spirv, size_t spirv_len,
                      const char *entrypoint);

/**
 * Add/update push constants for execution.
 */
int ff_vk_shader_add_push_const(FFVulkanShader *shd, int offset, int size,
                                VkShaderStageFlagBits stage);

/**
 * Add descriptor to a shader. Must be called before shader init.
 */
int ff_vk_shader_add_descriptor_set(FFVulkanContext *s, FFVulkanShader *shd,
                                    FFVulkanDescriptorSetBinding *desc, int nb,
                                    int singular, int print_to_shader_only);

/**
 * Register a shader with an exec pool.
 * Pool may be NULL if all descriptor sets are read-only.
 */
int ff_vk_shader_register_exec(FFVulkanContext *s, FFVkExecPool *pool,
                               FFVulkanShader *shd);

/**
 * Bind a shader.
 */
void ff_vk_exec_bind_shader(FFVulkanContext *s, FFVkExecContext *e,
                            FFVulkanShader *shd);

/**
 * Update push constant in a shader.
 * Must be called before binding the shader.
 */
void ff_vk_shader_update_push_const(FFVulkanContext *s, FFVkExecContext *e,
                                    FFVulkanShader *shd,
                                    VkShaderStageFlagBits stage,
                                    int offset, size_t size, void *src);

/**
 * Update a descriptor in a buffer with a buffer.
 * Must be called before binding the shader.
 */
int ff_vk_shader_update_desc_buffer(FFVulkanContext *s, FFVkExecContext *e,
                                    FFVulkanShader *shd,
                                    int set, int bind, int elem,
                                    FFVkBuffer *buf, VkDeviceSize offset, VkDeviceSize len,
                                    VkFormat fmt);

/**
 * Sets an image descriptor for specified shader and binding.
 */
int ff_vk_shader_update_img(FFVulkanContext *s, FFVkExecContext *e,
                            FFVulkanShader *shd, int set, int bind, int offs,
                            VkImageView view, VkImageLayout layout,
                            VkSampler sampler);

/**
 * Update a descriptor in a buffer with an image array..
 * Must be called before binding the shader.
 */
void ff_vk_shader_update_img_array(FFVulkanContext *s, FFVkExecContext *e,
                                   FFVulkanShader *shd, AVFrame *f,
                                   VkImageView *views, int set, int binding,
                                   VkImageLayout layout, VkSampler sampler);

/**
 * Free a shader.
 */
void ff_vk_shader_free(FFVulkanContext *s, FFVulkanShader *shd);

/**
 * Frees main context.
 */
void ff_vk_uninit(FFVulkanContext *s);

#endif /* AVUTIL_VULKAN_H */
