/*
 * Copyright 2014 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 */

/**
 * \file amdgpu.h
 *
 * Declare public libdrm_amdgpu API
 *
 * This file define API exposed by libdrm_amdgpu library.
 * User wanted to use libdrm_amdgpu functionality must include
 * this file.
 *
 */
#ifndef _AMDGPU_H_
#define _AMDGPU_H_

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

struct drm_amdgpu_info_hw_ip;
struct drm_amdgpu_bo_list_entry;

/*--------------------------------------------------------------------------*/
/* --------------------------- Defines ------------------------------------ */
/*--------------------------------------------------------------------------*/

/**
 * Define max. number of Command Buffers (IB) which could be sent to the single
 * hardware IP to accommodate CE/DE requirements
 *
 * \sa amdgpu_cs_ib_info
*/
#define AMDGPU_CS_MAX_IBS_PER_SUBMIT		4

/**
 * Special timeout value meaning that the timeout is infinite.
 */
#define AMDGPU_TIMEOUT_INFINITE			0xffffffffffffffffull

/**
 * Used in amdgpu_cs_query_fence_status(), meaning that the given timeout
 * is absolute.
 */
#define AMDGPU_QUERY_FENCE_TIMEOUT_IS_ABSOLUTE     (1 << 0)

/*--------------------------------------------------------------------------*/
/* ----------------------------- Enums ------------------------------------ */
/*--------------------------------------------------------------------------*/

/**
 * Enum describing possible handle types
 *
 * \sa amdgpu_bo_import, amdgpu_bo_export
 *
*/
enum amdgpu_bo_handle_type {
	/** GEM flink name (needs DRM authentication, used by DRI2) */
	amdgpu_bo_handle_type_gem_flink_name = 0,

	/** KMS handle which is used by all driver ioctls */
	amdgpu_bo_handle_type_kms = 1,

	/** DMA-buf fd handle */
	amdgpu_bo_handle_type_dma_buf_fd = 2,

	/** Deprecated in favour of and same behaviour as
	 * amdgpu_bo_handle_type_kms, use that instead of this
	 */
	amdgpu_bo_handle_type_kms_noimport = 3,
};

/** Define known types of GPU VM VA ranges */
enum amdgpu_gpu_va_range
{
	/** Allocate from "normal"/general range */
	amdgpu_gpu_va_range_general = 0
};

enum amdgpu_sw_info {
	amdgpu_sw_info_address32_hi = 0,
};

/*--------------------------------------------------------------------------*/
/* -------------------------- Datatypes ----------------------------------- */
/*--------------------------------------------------------------------------*/

/**
 * Define opaque pointer to context associated with fd.
 * This context will be returned as the result of
 * "initialize" function and should be pass as the first
 * parameter to any API call
 */
typedef struct amdgpu_device *amdgpu_device_handle;

/**
 * Define GPU Context type as pointer to opaque structure
 * Example of GPU Context is the "rendering" context associated
 * with OpenGL context (glCreateContext)
 */
typedef struct amdgpu_context *amdgpu_context_handle;

/**
 * Define handle for amdgpu resources: buffer, GDS, etc.
 */
typedef struct amdgpu_bo *amdgpu_bo_handle;

/**
 * Define handle for list of BOs
 */
typedef struct amdgpu_bo_list *amdgpu_bo_list_handle;

/**
 * Define handle to be used to work with VA allocated ranges
 */
typedef struct amdgpu_va *amdgpu_va_handle;

/**
 * Define handle for semaphore
 */
typedef struct amdgpu_semaphore *amdgpu_semaphore_handle;

/*--------------------------------------------------------------------------*/
/* -------------------------- Structures ---------------------------------- */
/*--------------------------------------------------------------------------*/

/**
 * Structure describing memory allocation request
 *
 * \sa amdgpu_bo_alloc()
 *
*/
struct amdgpu_bo_alloc_request {
	/** Allocation request. It must be aligned correctly. */
	uint64_t alloc_size;

	/**
	 * It may be required to have some specific alignment requirements
	 * for physical back-up storage (e.g. for displayable surface).
	 * If 0 there is no special alignment requirement
	 */
	uint64_t phys_alignment;

	/**
	 * UMD should specify where to allocate memory and how it
	 * will be accessed by the CPU.
	 */
	uint32_t preferred_heap;

	/** Additional flags passed on allocation */
	uint64_t flags;
};

/**
 * Special UMD specific information associated with buffer.
 *
 * It may be need to pass some buffer charactersitic as part
 * of buffer sharing. Such information are defined UMD and
 * opaque for libdrm_amdgpu as well for kernel driver.
 *
 * \sa amdgpu_bo_set_metadata(), amdgpu_bo_query_info,
 *     amdgpu_bo_import(), amdgpu_bo_export
 *
*/
struct amdgpu_bo_metadata {
	/** Special flag associated with surface */
	uint64_t flags;

	/**
	 * ASIC-specific tiling information (also used by DCE).
	 * The encoding is defined by the AMDGPU_TILING_* definitions.
	 */
	uint64_t tiling_info;

	/** Size of metadata associated with the buffer, in bytes. */
	uint32_t size_metadata;

	/** UMD specific metadata. Opaque for kernel */
	uint32_t umd_metadata[64];
};

/**
 * Structure describing allocated buffer. Client may need
 * to query such information as part of 'sharing' buffers mechanism
 *
 * \sa amdgpu_bo_set_metadata(), amdgpu_bo_query_info(),
 *     amdgpu_bo_import(), amdgpu_bo_export()
*/
struct amdgpu_bo_info {
	/** Allocated memory size */
	uint64_t alloc_size;

	/**
	 * It may be required to have some specific alignment requirements
	 * for physical back-up storage.
	 */
	uint64_t phys_alignment;

	/** Heap where to allocate memory. */
	uint32_t preferred_heap;

	/** Additional allocation flags. */
	uint64_t alloc_flags;

	/** Metadata associated with buffer if any. */
	struct amdgpu_bo_metadata metadata;
};

/**
 * Structure with information about "imported" buffer
 *
 * \sa amdgpu_bo_import()
 *
 */
struct amdgpu_bo_import_result {
	/** Handle of memory/buffer to use */
	amdgpu_bo_handle buf_handle;

	 /** Buffer size */
	uint64_t alloc_size;
};

/**
 *
 * Structure to describe GDS partitioning information.
 * \note OA and GWS resources are asscoiated with GDS partition
 *
 * \sa amdgpu_gpu_resource_query_gds_info
 *
*/
struct amdgpu_gds_resource_info {
	uint32_t gds_gfx_partition_size;
	uint32_t compute_partition_size;
	uint32_t gds_total_size;
	uint32_t gws_per_gfx_partition;
	uint32_t gws_per_compute_partition;
	uint32_t oa_per_gfx_partition;
	uint32_t oa_per_compute_partition;
};

/**
 * Structure describing CS fence
 *
 * \sa amdgpu_cs_query_fence_status(), amdgpu_cs_request, amdgpu_cs_submit()
 *
*/
struct amdgpu_cs_fence {

	/** In which context IB was sent to execution */
	amdgpu_context_handle context;

	/** To which HW IP type the fence belongs */
	uint32_t ip_type;

	/** IP instance index if there are several IPs of the same type. */
	uint32_t ip_instance;

	/** Ring index of the HW IP */
	uint32_t ring;

	/** Specify fence for which we need to check submission status.*/
	uint64_t fence;
};

/**
 * Structure describing IB
 *
 * \sa amdgpu_cs_request, amdgpu_cs_submit()
 *
*/
struct amdgpu_cs_ib_info {
	/** Special flags */
	uint64_t flags;

	/** Virtual MC address of the command buffer */
	uint64_t ib_mc_address;

	/**
	 * Size of Command Buffer to be submitted.
	 *   - The size is in units of dwords (4 bytes).
	 *   - Could be 0
	 */
	uint32_t size;
};

/**
 * Structure describing fence information
 *
 * \sa amdgpu_cs_request, amdgpu_cs_query_fence,
 *     amdgpu_cs_submit(), amdgpu_cs_query_fence_status()
*/
struct amdgpu_cs_fence_info {
	/** buffer object for the fence */
	amdgpu_bo_handle handle;

	/** fence offset in the unit of sizeof(uint64_t) */
	uint64_t offset;
};

/**
 * Structure describing submission request
 *
 * \note We could have several IBs as packet. e.g. CE, CE, DE case for gfx
 *
 * \sa amdgpu_cs_submit()
*/
struct amdgpu_cs_request {
	/** Specify flags with additional information */
	uint64_t flags;

	/** Specify HW IP block type to which to send the IB. */
	unsigned ip_type;

	/** IP instance index if there are several IPs of the same type. */
	unsigned ip_instance;

	/**
	 * Specify ring index of the IP. We could have several rings
	 * in the same IP. E.g. 0 for SDMA0 and 1 for SDMA1.
	 */
	uint32_t ring;

	/**
	 * List handle with resources used by this request.
	 */
	amdgpu_bo_list_handle resources;

	/**
	 * Number of dependencies this Command submission needs to
	 * wait for before starting execution.
	 */
	uint32_t number_of_dependencies;

	/**
	 * Array of dependencies which need to be met before
	 * execution can start.
	 */
	struct amdgpu_cs_fence *dependencies;

	/** Number of IBs to submit in the field ibs. */
	uint32_t number_of_ibs;

	/**
	 * IBs to submit. Those IBs will be submit together as single entity
	 */
	struct amdgpu_cs_ib_info *ibs;

	/**
	 * The returned sequence number for the command submission 
	 */
	uint64_t seq_no;

	/**
	 * The fence information
	 */
	struct amdgpu_cs_fence_info fence_info;
};

/**
 * Structure which provide information about GPU VM MC Address space
 * alignments requirements
 *
 * \sa amdgpu_query_buffer_size_alignment
 */
struct amdgpu_buffer_size_alignments {
	/** Size alignment requirement for allocation in
	 * local memory */
	uint64_t size_local;

	/**
	 * Size alignment requirement for allocation in remote memory
	 */
	uint64_t size_remote;
};

/**
 * Structure which provide information about heap
 *
 * \sa amdgpu_query_heap_info()
 *
 */
struct amdgpu_heap_info {
	/** Theoretical max. available memory in the given heap */
	uint64_t heap_size;

	/**
	 * Number of bytes allocated in the heap. This includes all processes
	 * and private allocations in the kernel. It changes when new buffers
	 * are allocated, freed, and moved. It cannot be larger than
	 * heap_size.
	 */
	uint64_t heap_usage;

	/**
	 * Theoretical possible max. size of buffer which
	 * could be allocated in the given heap
	 */
	uint64_t max_allocation;
};

/**
 * Describe GPU h/w info needed for UMD correct initialization
 *
 * \sa amdgpu_query_gpu_info()
*/
struct amdgpu_gpu_info {
	/** Asic id */
	uint32_t asic_id;
	/** Chip revision */
	uint32_t chip_rev;
	/** Chip external revision */
	uint32_t chip_external_rev;
	/** Family ID */
	uint32_t family_id;
	/** Special flags */
	uint64_t ids_flags;
	/** max engine clock*/
	uint64_t max_engine_clk;
	/** max memory clock */
	uint64_t max_memory_clk;
	/** number of shader engines */
	uint32_t num_shader_engines;
	/** number of shader arrays per engine */
	uint32_t num_shader_arrays_per_engine;
	/**  Number of available good shader pipes */
	uint32_t avail_quad_shader_pipes;
	/**  Max. number of shader pipes.(including good and bad pipes  */
	uint32_t max_quad_shader_pipes;
	/** Number of parameter cache entries per shader quad pipe */
	uint32_t cache_entries_per_quad_pipe;
	/**  Number of available graphics context */
	uint32_t num_hw_gfx_contexts;
	/** Number of render backend pipes */
	uint32_t rb_pipes;
	/**  Enabled render backend pipe mask */
	uint32_t enabled_rb_pipes_mask;
	/** Frequency of GPU Counter */
	uint32_t gpu_counter_freq;
	/** CC_RB_BACKEND_DISABLE.BACKEND_DISABLE per SE */
	uint32_t backend_disable[4];
	/** Value of MC_ARB_RAMCFG register*/
	uint32_t mc_arb_ramcfg;
	/** Value of GB_ADDR_CONFIG */
	uint32_t gb_addr_cfg;
	/** Values of the GB_TILE_MODE0..31 registers */
	uint32_t gb_tile_mode[32];
	/** Values of GB_MACROTILE_MODE0..15 registers */
	uint32_t gb_macro_tile_mode[16];
	/** Value of PA_SC_RASTER_CONFIG register per SE */
	uint32_t pa_sc_raster_cfg[4];
	/** Value of PA_SC_RASTER_CONFIG_1 register per SE */
	uint32_t pa_sc_raster_cfg1[4];
	/* CU info */
	uint32_t cu_active_number;
	uint32_t cu_ao_mask;
	uint32_t cu_bitmap[4][4];
	/* video memory type info*/
	uint32_t vram_type;
	/* video memory bit width*/
	uint32_t vram_bit_width;
	/** constant engine ram size*/
	uint32_t ce_ram_size;
	/* vce harvesting instance */
	uint32_t vce_harvest_config;
	/* PCI revision ID */
	uint32_t pci_rev_id;
};


/*--------------------------------------------------------------------------*/
/*------------------------- Functions --------------------------------------*/
/*--------------------------------------------------------------------------*/

/*
 * Initialization / Cleanup
 *
*/

/**
 *
 * \param   fd            - \c [in]  File descriptor for AMD GPU device
 *                                   received previously as the result of
 *                                   e.g. drmOpen() call.
 *                                   For legacy fd type, the DRI2/DRI3
 *                                   authentication should be done before
 *                                   calling this function.
 * \param   major_version - \c [out] Major version of library. It is assumed
 *                                   that adding new functionality will cause
 *                                   increase in major version
 * \param   minor_version - \c [out] Minor version of library
 * \param   device_handle - \c [out] Pointer to opaque context which should
 *                                   be passed as the first parameter on each
 *                                   API call
 *
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 *
 * \sa amdgpu_device_deinitialize()
*/
int amdgpu_device_initialize(int fd,
			     uint32_t *major_version,
			     uint32_t *minor_version,
			     amdgpu_device_handle *device_handle);

/**
 *
 * When access to such library does not needed any more the special
 * function must be call giving opportunity to clean up any
 * resources if needed.
 *
 * \param   device_handle - \c [in]  Context associated with file
 *                                   descriptor for AMD GPU device
 *                                   received previously as the
 *                                   result e.g. of drmOpen() call.
 *
 * \return  0 on success\n
 *         <0 - Negative POSIX Error code
 *
 * \sa amdgpu_device_initialize()
 *
*/
int amdgpu_device_deinitialize(amdgpu_device_handle device_handle);

/*
 * Memory Management
 *
*/

/**
 * Allocate memory to be used by UMD for GPU related operations
 *
 * \param   dev		 - \c [in] Device handle.
 *				   See #amdgpu_device_initialize()
 * \param   alloc_buffer - \c [in] Pointer to the structure describing an
 *				   allocation request
 * \param   buf_handle	- \c [out] Allocated buffer handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \sa amdgpu_bo_free()
*/
int amdgpu_bo_alloc(amdgpu_device_handle dev,
		    struct amdgpu_bo_alloc_request *alloc_buffer,
		    amdgpu_bo_handle *buf_handle);

/**
 * Associate opaque data with buffer to be queried by another UMD
 *
 * \param   dev	       - \c [in] Device handle. See #amdgpu_device_initialize()
 * \param   buf_handle - \c [in] Buffer handle
 * \param   info       - \c [in] Metadata to associated with buffer
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
*/
int amdgpu_bo_set_metadata(amdgpu_bo_handle buf_handle,
			   struct amdgpu_bo_metadata *info);

/**
 * Query buffer information including metadata previusly associated with
 * buffer.
 *
 * \param   dev	       - \c [in] Device handle.
 *				 See #amdgpu_device_initialize()
 * \param   buf_handle - \c [in]   Buffer handle
 * \param   info       - \c [out]  Structure describing buffer
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \sa amdgpu_bo_set_metadata(), amdgpu_bo_alloc()
*/
int amdgpu_bo_query_info(amdgpu_bo_handle buf_handle,
			 struct amdgpu_bo_info *info);

/**
 * Allow others to get access to buffer
 *
 * \param   dev		  - \c [in] Device handle.
 *				    See #amdgpu_device_initialize()
 * \param   buf_handle    - \c [in] Buffer handle
 * \param   type          - \c [in] Type of handle requested
 * \param   shared_handle - \c [out] Special "shared" handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \sa amdgpu_bo_import()
 *
*/
int amdgpu_bo_export(amdgpu_bo_handle buf_handle,
		     enum amdgpu_bo_handle_type type,
		     uint32_t *shared_handle);

/**
 * Request access to "shared" buffer
 *
 * \param   dev		  - \c [in] Device handle.
 *				    See #amdgpu_device_initialize()
 * \param   type	  - \c [in] Type of handle requested
 * \param   shared_handle - \c [in] Shared handle received as result "import"
 *				     operation
 * \param   output        - \c [out] Pointer to structure with information
 *				     about imported buffer
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \note  Buffer must be "imported" only using new "fd" (different from
 *	  one used by "exporter").
 *
 * \sa amdgpu_bo_export()
 *
*/
int amdgpu_bo_import(amdgpu_device_handle dev,
		     enum amdgpu_bo_handle_type type,
		     uint32_t shared_handle,
		     struct amdgpu_bo_import_result *output);

/**
 * Request GPU access to user allocated memory e.g. via "malloc"
 *
 * \param dev - [in] Device handle. See #amdgpu_device_initialize()
 * \param cpu - [in] CPU address of user allocated memory which we
 * want to map to GPU address space (make GPU accessible)
 * (This address must be correctly aligned).
 * \param size - [in] Size of allocation (must be correctly aligned)
 * \param buf_handle - [out] Buffer handle for the userptr memory
 * resource on submission and be used in other operations.
 *
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \note
 * This call doesn't guarantee that such memory will be persistently
 * "locked" / make non-pageable. The purpose of this call is to provide
 * opportunity for GPU get access to this resource during submission.
 *
 * The maximum amount of memory which could be mapped in this call depends
 * if overcommit is disabled or not. If overcommit is disabled than the max.
 * amount of memory to be pinned will be limited by left "free" size in total
 * amount of memory which could be locked simultaneously ("GART" size).
 *
 * Supported (theoretical) max. size of mapping is restricted only by
 * "GART" size.
 *
 * It is responsibility of caller to correctly specify access rights
 * on VA assignment.
*/
int amdgpu_create_bo_from_user_mem(amdgpu_device_handle dev,
				    void *cpu, uint64_t size,
				    amdgpu_bo_handle *buf_handle);

/**
 * Validate if the user memory comes from BO
 *
 * \param dev - [in] Device handle. See #amdgpu_device_initialize()
 * \param cpu - [in] CPU address of user allocated memory which we
 * want to map to GPU address space (make GPU accessible)
 * (This address must be correctly aligned).
 * \param size - [in] Size of allocation (must be correctly aligned)
 * \param buf_handle - [out] Buffer handle for the userptr memory
 * if the user memory is not from BO, the buf_handle will be NULL.
 * \param offset_in_bo - [out] offset in this BO for this user memory
 *
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_find_bo_by_cpu_mapping(amdgpu_device_handle dev,
				  void *cpu,
				  uint64_t size,
				  amdgpu_bo_handle *buf_handle,
				  uint64_t *offset_in_bo);

/**
 * Free previously allocated memory
 *
 * \param   dev	       - \c [in] Device handle. See #amdgpu_device_initialize()
 * \param   buf_handle - \c [in]  Buffer handle to free
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \note In the case of memory shared between different applications all
 *	 resources will be “physically” freed only all such applications
 *	 will be terminated
 * \note If is UMD responsibility to ‘free’ buffer only when there is no
 *	 more GPU access
 *
 * \sa amdgpu_bo_set_metadata(), amdgpu_bo_alloc()
 *
*/
int amdgpu_bo_free(amdgpu_bo_handle buf_handle);

/**
 * Increase the reference count of a buffer object
 *
 * \param   bo - \c [in]  Buffer object handle to increase the reference count
 *
 * \sa amdgpu_bo_alloc(), amdgpu_bo_free()
 *
*/
void amdgpu_bo_inc_ref(amdgpu_bo_handle bo);

/**
 * Request CPU access to GPU accessible memory
 *
 * \param   buf_handle - \c [in] Buffer handle
 * \param   cpu        - \c [out] CPU address to be used for access
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \sa amdgpu_bo_cpu_unmap()
 *
*/
int amdgpu_bo_cpu_map(amdgpu_bo_handle buf_handle, void **cpu);

/**
 * Release CPU access to GPU memory
 *
 * \param   buf_handle  - \c [in] Buffer handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \sa amdgpu_bo_cpu_map()
 *
*/
int amdgpu_bo_cpu_unmap(amdgpu_bo_handle buf_handle);

/**
 * Wait until a buffer is not used by the device.
 *
 * \param   dev           - \c [in] Device handle. See #amdgpu_device_initialize()
 * \param   buf_handle    - \c [in] Buffer handle.
 * \param   timeout_ns    - Timeout in nanoseconds.
 * \param   buffer_busy   - 0 if buffer is idle, all GPU access was completed
 *                            and no GPU access is scheduled.
 *                          1 GPU access is in fly or scheduled
 *
 * \return   0 - on success
 *          <0 - Negative POSIX Error code
 */
int amdgpu_bo_wait_for_idle(amdgpu_bo_handle buf_handle,
			    uint64_t timeout_ns,
			    bool *buffer_busy);

/**
 * Creates a BO list handle for command submission.
 *
 * \param   dev			- \c [in] Device handle.
 *				   See #amdgpu_device_initialize()
 * \param   number_of_buffers	- \c [in] Number of BOs in the list
 * \param   buffers		- \c [in] List of BO handles
 * \param   result		- \c [out] Created BO list handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \sa amdgpu_bo_list_destroy_raw(), amdgpu_cs_submit_raw2()
*/
int amdgpu_bo_list_create_raw(amdgpu_device_handle dev,
			      uint32_t number_of_buffers,
			      struct drm_amdgpu_bo_list_entry *buffers,
			      uint32_t *result);

/**
 * Destroys a BO list handle.
 *
 * \param   bo_list	- \c [in] BO list handle.
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \sa amdgpu_bo_list_create_raw(), amdgpu_cs_submit_raw2()
*/
int amdgpu_bo_list_destroy_raw(amdgpu_device_handle dev, uint32_t bo_list);

/**
 * Creates a BO list handle for command submission.
 *
 * \param   dev			- \c [in] Device handle.
 *				   See #amdgpu_device_initialize()
 * \param   number_of_resources	- \c [in] Number of BOs in the list
 * \param   resources		- \c [in] List of BO handles
 * \param   resource_prios	- \c [in] Optional priority for each handle
 * \param   result		- \c [out] Created BO list handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \sa amdgpu_bo_list_destroy()
*/
int amdgpu_bo_list_create(amdgpu_device_handle dev,
			  uint32_t number_of_resources,
			  amdgpu_bo_handle *resources,
			  uint8_t *resource_prios,
			  amdgpu_bo_list_handle *result);

/**
 * Destroys a BO list handle.
 *
 * \param   handle	- \c [in] BO list handle.
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \sa amdgpu_bo_list_create()
*/
int amdgpu_bo_list_destroy(amdgpu_bo_list_handle handle);

/**
 * Update resources for existing BO list
 *
 * \param   handle              - \c [in] BO list handle
 * \param   number_of_resources - \c [in] Number of BOs in the list
 * \param   resources           - \c [in] List of BO handles
 * \param   resource_prios      - \c [in] Optional priority for each handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \sa amdgpu_bo_list_update()
*/
int amdgpu_bo_list_update(amdgpu_bo_list_handle handle,
			  uint32_t number_of_resources,
			  amdgpu_bo_handle *resources,
			  uint8_t *resource_prios);

/*
 * GPU Execution context
 *
*/

/**
 * Create GPU execution Context
 *
 * For the purpose of GPU Scheduler and GPU Robustness extensions it is
 * necessary to have information/identify rendering/compute contexts.
 * It also may be needed to associate some specific requirements with such
 * contexts.  Kernel driver will guarantee that submission from the same
 * context will always be executed in order (first come, first serve).
 *
 *
 * \param   dev      - \c [in] Device handle. See #amdgpu_device_initialize()
 * \param   priority - \c [in] Context creation flags. See AMDGPU_CTX_PRIORITY_*
 * \param   context  - \c [out] GPU Context handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \sa amdgpu_cs_ctx_free()
 *
*/
int amdgpu_cs_ctx_create2(amdgpu_device_handle dev,
			 uint32_t priority,
			 amdgpu_context_handle *context);
/**
 * Create GPU execution Context
 *
 * Refer to amdgpu_cs_ctx_create2 for full documentation. This call
 * is missing the priority parameter.
 *
 * \sa amdgpu_cs_ctx_create2()
 *
*/
int amdgpu_cs_ctx_create(amdgpu_device_handle dev,
			 amdgpu_context_handle *context);

/**
 *
 * Destroy GPU execution context when not needed any more
 *
 * \param   context - \c [in] GPU Context handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \sa amdgpu_cs_ctx_create()
 *
*/
int amdgpu_cs_ctx_free(amdgpu_context_handle context);

/**
 * Override the submission priority for the given context using a master fd.
 *
 * \param   dev        - \c [in] device handle
 * \param   context    - \c [in] context handle for context id
 * \param   master_fd  - \c [in] The master fd to authorize the override.
 * \param   priority   - \c [in] The priority to assign to the context.
 *
 * \return 0 on success or a a negative Posix error code on failure.
 */
int amdgpu_cs_ctx_override_priority(amdgpu_device_handle dev,
                                    amdgpu_context_handle context,
                                    int master_fd,
                                    unsigned priority);

/**
 * Query reset state for the specific GPU Context
 *
 * \param   context - \c [in]  GPU Context handle
 * \param   state   - \c [out] One of AMDGPU_CTX_*_RESET
 * \param   hangs   - \c [out] Number of hangs caused by the context.
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \sa amdgpu_cs_ctx_create()
 *
*/
int amdgpu_cs_query_reset_state(amdgpu_context_handle context,
				uint32_t *state, uint32_t *hangs);

/**
 * Query reset state for the specific GPU Context.
 *
 * \param   context - \c [in]  GPU Context handle
 * \param   flags   - \c [out] A combination of AMDGPU_CTX_QUERY2_FLAGS_*
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \sa amdgpu_cs_ctx_create()
 *
*/
int amdgpu_cs_query_reset_state2(amdgpu_context_handle context,
				 uint64_t *flags);

/*
 * Command Buffers Management
 *
*/

/**
 * Send request to submit command buffers to hardware.
 *
 * Kernel driver could use GPU Scheduler to make decision when physically
 * sent this request to the hardware. Accordingly this request could be put
 * in queue and sent for execution later. The only guarantee is that request
 * from the same GPU context to the same ip:ip_instance:ring will be executed in
 * order.
 *
 * The caller can specify the user fence buffer/location with the fence_info in the
 * cs_request.The sequence number is returned via the 'seq_no' parameter
 * in ibs_request structure.
 *
 *
 * \param   dev		       - \c [in]  Device handle.
 *					  See #amdgpu_device_initialize()
 * \param   context            - \c [in]  GPU Context
 * \param   flags              - \c [in]  Global submission flags
 * \param   ibs_request        - \c [in/out] Pointer to submission requests.
 *					  We could submit to the several
 *					  engines/rings simulteniously as
 *					  'atomic' operation
 * \param   number_of_requests - \c [in]  Number of submission requests
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \note It is required to pass correct resource list with buffer handles
 *	 which will be accessible by command buffers from submission
 *	 This will allow kernel driver to correctly implement "paging".
 *	 Failure to do so will have unpredictable results.
 *
 * \sa amdgpu_command_buffer_alloc(), amdgpu_command_buffer_free(),
 *     amdgpu_cs_query_fence_status()
 *
*/
int amdgpu_cs_submit(amdgpu_context_handle context,
		     uint64_t flags,
		     struct amdgpu_cs_request *ibs_request,
		     uint32_t number_of_requests);

/**
 *  Query status of Command Buffer Submission
 *
 * \param   fence   - \c [in] Structure describing fence to query
 * \param   timeout_ns - \c [in] Timeout value to wait
 * \param   flags   - \c [in] Flags for the query
 * \param   expired - \c [out] If fence expired or not.\n
 *				0  – if fence is not expired\n
 *				!0 - otherwise
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \note If UMD wants only to check operation status and returned immediately
 *	 then timeout value as 0 must be passed. In this case success will be
 *	 returned in the case if submission was completed or timeout error
 *	 code.
 *
 * \sa amdgpu_cs_submit()
*/
int amdgpu_cs_query_fence_status(struct amdgpu_cs_fence *fence,
				 uint64_t timeout_ns,
				 uint64_t flags,
				 uint32_t *expired);

/**
 *  Wait for multiple fences
 *
 * \param   fences      - \c [in] The fence array to wait
 * \param   fence_count - \c [in] The fence count
 * \param   wait_all    - \c [in] If true, wait all fences to be signaled,
 *                                otherwise, wait at least one fence
 * \param   timeout_ns  - \c [in] The timeout to wait, in nanoseconds
 * \param   status      - \c [out] '1' for signaled, '0' for timeout
 * \param   first       - \c [out] the index of the first signaled fence from @fences
 *
 * \return  0 on success
 *          <0 - Negative POSIX Error code
 *
 * \note    Currently it supports only one amdgpu_device. All fences come from
 *          the same amdgpu_device with the same fd.
*/
int amdgpu_cs_wait_fences(struct amdgpu_cs_fence *fences,
			  uint32_t fence_count,
			  bool wait_all,
			  uint64_t timeout_ns,
			  uint32_t *status, uint32_t *first);

/*
 * Query / Info API
 *
*/

/**
 * Query allocation size alignments
 *
 * UMD should query information about GPU VM MC size alignments requirements
 * to be able correctly choose required allocation size and implement
 * internal optimization if needed.
 *
 * \param   dev  - \c [in] Device handle. See #amdgpu_device_initialize()
 * \param   info - \c [out] Pointer to structure to get size alignment
 *			  requirements
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_query_buffer_size_alignment(amdgpu_device_handle dev,
				       struct amdgpu_buffer_size_alignments
						*info);

/**
 * Query firmware versions
 *
 * \param   dev	        - \c [in] Device handle. See #amdgpu_device_initialize()
 * \param   fw_type     - \c [in] AMDGPU_INFO_FW_*
 * \param   ip_instance - \c [in] Index of the IP block of the same type.
 * \param   index       - \c [in] Index of the engine. (for SDMA and MEC)
 * \param   version     - \c [out] Pointer to to the "version" return value
 * \param   feature     - \c [out] Pointer to to the "feature" return value
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_query_firmware_version(amdgpu_device_handle dev, unsigned fw_type,
				  unsigned ip_instance, unsigned index,
				  uint32_t *version, uint32_t *feature);

/**
 * Query the number of HW IP instances of a certain type.
 *
 * \param   dev      - \c [in] Device handle. See #amdgpu_device_initialize()
 * \param   type     - \c [in] Hardware IP block type = AMDGPU_HW_IP_*
 * \param   count    - \c [out] Pointer to structure to get information
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
*/
int amdgpu_query_hw_ip_count(amdgpu_device_handle dev, unsigned type,
			     uint32_t *count);

/**
 * Query engine information
 *
 * This query allows UMD to query information different engines and their
 * capabilities.
 *
 * \param   dev         - \c [in] Device handle. See #amdgpu_device_initialize()
 * \param   type        - \c [in] Hardware IP block type = AMDGPU_HW_IP_*
 * \param   ip_instance - \c [in] Index of the IP block of the same type.
 * \param   info        - \c [out] Pointer to structure to get information
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
*/
int amdgpu_query_hw_ip_info(amdgpu_device_handle dev, unsigned type,
			    unsigned ip_instance,
			    struct drm_amdgpu_info_hw_ip *info);

/**
 * Query heap information
 *
 * This query allows UMD to query potentially available memory resources and
 * adjust their logic if necessary.
 *
 * \param   dev  - \c [in] Device handle. See #amdgpu_device_initialize()
 * \param   heap - \c [in] Heap type
 * \param   info - \c [in] Pointer to structure to get needed information
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_query_heap_info(amdgpu_device_handle dev, uint32_t heap,
			   uint32_t flags, struct amdgpu_heap_info *info);

/**
 * Get the CRTC ID from the mode object ID
 *
 * \param   dev    - \c [in] Device handle. See #amdgpu_device_initialize()
 * \param   id     - \c [in] Mode object ID
 * \param   result - \c [in] Pointer to the CRTC ID
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_query_crtc_from_id(amdgpu_device_handle dev, unsigned id,
			      int32_t *result);

/**
 * Query GPU H/w Info
 *
 * Query hardware specific information
 *
 * \param   dev  - \c [in] Device handle. See #amdgpu_device_initialize()
 * \param   heap - \c [in] Heap type
 * \param   info - \c [in] Pointer to structure to get needed information
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_query_gpu_info(amdgpu_device_handle dev,
			   struct amdgpu_gpu_info *info);

/**
 * Query hardware or driver information.
 *
 * The return size is query-specific and depends on the "info_id" parameter.
 * No more than "size" bytes is returned.
 *
 * \param   dev     - \c [in] Device handle. See #amdgpu_device_initialize()
 * \param   info_id - \c [in] AMDGPU_INFO_*
 * \param   size    - \c [in] Size of the returned value.
 * \param   value   - \c [out] Pointer to the return value.
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX error code
 *
*/
int amdgpu_query_info(amdgpu_device_handle dev, unsigned info_id,
		      unsigned size, void *value);

/**
 * Query hardware or driver information.
 *
 * The return size is query-specific and depends on the "info_id" parameter.
 * No more than "size" bytes is returned.
 *
 * \param   dev     - \c [in] Device handle. See #amdgpu_device_initialize()
 * \param   info    - \c [in] amdgpu_sw_info_*
 * \param   value   - \c [out] Pointer to the return value.
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX error code
 *
*/
int amdgpu_query_sw_info(amdgpu_device_handle dev, enum amdgpu_sw_info info,
			 void *value);

/**
 * Query information about GDS
 *
 * \param   dev	     - \c [in] Device handle. See #amdgpu_device_initialize()
 * \param   gds_info - \c [out] Pointer to structure to get GDS information
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_query_gds_info(amdgpu_device_handle dev,
			struct amdgpu_gds_resource_info *gds_info);

/**
 * Query information about sensor.
 *
 * The return size is query-specific and depends on the "sensor_type"
 * parameter. No more than "size" bytes is returned.
 *
 * \param   dev         - \c [in] Device handle. See #amdgpu_device_initialize()
 * \param   sensor_type - \c [in] AMDGPU_INFO_SENSOR_*
 * \param   size        - \c [in] Size of the returned value.
 * \param   value       - \c [out] Pointer to the return value.
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_query_sensor_info(amdgpu_device_handle dev, unsigned sensor_type,
			     unsigned size, void *value);

/**
 * Read a set of consecutive memory-mapped registers.
 * Not all registers are allowed to be read by userspace.
 *
 * \param   dev          - \c [in] Device handle. See #amdgpu_device_initialize(
 * \param   dword_offset - \c [in] Register offset in dwords
 * \param   count        - \c [in] The number of registers to read starting
 *                                 from the offset
 * \param   instance     - \c [in] GRBM_GFX_INDEX selector. It may have other
 *                                 uses. Set it to 0xffffffff if unsure.
 * \param   flags        - \c [in] Flags with additional information.
 * \param   values       - \c [out] The pointer to return values.
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX error code
 *
*/
int amdgpu_read_mm_registers(amdgpu_device_handle dev, unsigned dword_offset,
			     unsigned count, uint32_t instance, uint32_t flags,
			     uint32_t *values);

/**
 * Flag to request VA address range in the 32bit address space
*/
#define AMDGPU_VA_RANGE_32_BIT		0x1
#define AMDGPU_VA_RANGE_HIGH		0x2

/**
 * Allocate virtual address range
 *
 * \param dev - [in] Device handle. See #amdgpu_device_initialize()
 * \param va_range_type - \c [in] Type of MC va range from which to allocate
 * \param size - \c [in] Size of range. Size must be correctly* aligned.
 * It is client responsibility to correctly aligned size based on the future
 * usage of allocated range.
 * \param va_base_alignment - \c [in] Overwrite base address alignment
 * requirement for GPU VM MC virtual
 * address assignment. Must be multiple of size alignments received as
 * 'amdgpu_buffer_size_alignments'.
 * If 0 use the default one.
 * \param va_base_required - \c [in] Specified required va base address.
 * If 0 then library choose available one.
 * If !0 value will be passed and those value already "in use" then
 * corresponding error status will be returned.
 * \param va_base_allocated - \c [out] On return: Allocated VA base to be used
 * by client.
 * \param va_range_handle - \c [out] On return: Handle assigned to allocation
 * \param flags - \c [in] flags for special VA range
 *
 * \return 0 on success\n
 * >0 - AMD specific error code\n
 * <0 - Negative POSIX Error code
 *
 * \notes \n
 * It is client responsibility to correctly handle VA assignments and usage.
 * Neither kernel driver nor libdrm_amdpgu are able to prevent and
 * detect wrong va assignment.
 *
 * It is client responsibility to correctly handle multi-GPU cases and to pass
 * the corresponding arrays of all devices handles where corresponding VA will
 * be used.
 *
*/
int amdgpu_va_range_alloc(amdgpu_device_handle dev,
			   enum amdgpu_gpu_va_range va_range_type,
			   uint64_t size,
			   uint64_t va_base_alignment,
			   uint64_t va_base_required,
			   uint64_t *va_base_allocated,
			   amdgpu_va_handle *va_range_handle,
			   uint64_t flags);

/**
 * Free previously allocated virtual address range
 *
 *
 * \param va_range_handle - \c [in] Handle assigned to VA allocation
 *
 * \return 0 on success\n
 * >0 - AMD specific error code\n
 * <0 - Negative POSIX Error code
 *
*/
int amdgpu_va_range_free(amdgpu_va_handle va_range_handle);

/**
* Query virtual address range
*
* UMD can query GPU VM range supported by each device
* to initialize its own VAM accordingly.
*
* \param   dev    - [in] Device handle. See #amdgpu_device_initialize()
* \param   type   - \c [in] Type of virtual address range
* \param   offset - \c [out] Start offset of virtual address range
* \param   size   - \c [out] Size of virtual address range
*
* \return   0 on success\n
*          <0 - Negative POSIX Error code
*
*/

int amdgpu_va_range_query(amdgpu_device_handle dev,
			  enum amdgpu_gpu_va_range type,
			  uint64_t *start,
			  uint64_t *end);

/**
 *  VA mapping/unmapping for the buffer object
 *
 * \param  bo		- \c [in] BO handle
 * \param  offset	- \c [in] Start offset to map
 * \param  size		- \c [in] Size to map
 * \param  addr		- \c [in] Start virtual address.
 * \param  flags	- \c [in] Supported flags for mapping/unmapping
 * \param  ops		- \c [in] AMDGPU_VA_OP_MAP or AMDGPU_VA_OP_UNMAP
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/

int amdgpu_bo_va_op(amdgpu_bo_handle bo,
		    uint64_t offset,
		    uint64_t size,
		    uint64_t addr,
		    uint64_t flags,
		    uint32_t ops);

/**
 *  VA mapping/unmapping for a buffer object or PRT region.
 *
 * This is not a simple drop-in extension for amdgpu_bo_va_op; instead, all
 * parameters are treated "raw", i.e. size is not automatically aligned, and
 * all flags must be specified explicitly.
 *
 * \param  dev		- \c [in] device handle
 * \param  bo		- \c [in] BO handle (may be NULL)
 * \param  offset	- \c [in] Start offset to map
 * \param  size		- \c [in] Size to map
 * \param  addr		- \c [in] Start virtual address.
 * \param  flags	- \c [in] Supported flags for mapping/unmapping
 * \param  ops		- \c [in] AMDGPU_VA_OP_MAP or AMDGPU_VA_OP_UNMAP
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/

int amdgpu_bo_va_op_raw(amdgpu_device_handle dev,
			amdgpu_bo_handle bo,
			uint64_t offset,
			uint64_t size,
			uint64_t addr,
			uint64_t flags,
			uint32_t ops);

/**
 *  create semaphore
 *
 * \param   sem	   - \c [out] semaphore handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_cs_create_semaphore(amdgpu_semaphore_handle *sem);

/**
 *  signal semaphore
 *
 * \param   context        - \c [in] GPU Context
 * \param   ip_type        - \c [in] Hardware IP block type = AMDGPU_HW_IP_*
 * \param   ip_instance    - \c [in] Index of the IP block of the same type
 * \param   ring           - \c [in] Specify ring index of the IP
 * \param   sem	           - \c [in] semaphore handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_cs_signal_semaphore(amdgpu_context_handle ctx,
			       uint32_t ip_type,
			       uint32_t ip_instance,
			       uint32_t ring,
			       amdgpu_semaphore_handle sem);

/**
 *  wait semaphore
 *
 * \param   context        - \c [in] GPU Context
 * \param   ip_type        - \c [in] Hardware IP block type = AMDGPU_HW_IP_*
 * \param   ip_instance    - \c [in] Index of the IP block of the same type
 * \param   ring           - \c [in] Specify ring index of the IP
 * \param   sem	           - \c [in] semaphore handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_cs_wait_semaphore(amdgpu_context_handle ctx,
			     uint32_t ip_type,
			     uint32_t ip_instance,
			     uint32_t ring,
			     amdgpu_semaphore_handle sem);

/**
 *  destroy semaphore
 *
 * \param   sem	    - \c [in] semaphore handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_cs_destroy_semaphore(amdgpu_semaphore_handle sem);

/**
 *  Get the ASIC marketing name
 *
 * \param   dev         - \c [in] Device handle. See #amdgpu_device_initialize()
 *
 * \return  the constant string of the marketing name
 *          "NULL" means the ASIC is not found
*/
const char *amdgpu_get_marketing_name(amdgpu_device_handle dev);

/**
 *  Create kernel sync object
 *
 * \param   dev         - \c [in]  device handle
 * \param   flags       - \c [in]  flags that affect creation
 * \param   syncobj     - \c [out] sync object handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_cs_create_syncobj2(amdgpu_device_handle dev,
			      uint32_t  flags,
			      uint32_t *syncobj);

/**
 *  Create kernel sync object
 *
 * \param   dev	      - \c [in]  device handle
 * \param   syncobj   - \c [out] sync object handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_cs_create_syncobj(amdgpu_device_handle dev,
			     uint32_t *syncobj);
/**
 *  Destroy kernel sync object
 *
 * \param   dev	    - \c [in] device handle
 * \param   syncobj - \c [in] sync object handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_cs_destroy_syncobj(amdgpu_device_handle dev,
			      uint32_t syncobj);

/**
 * Reset kernel sync objects to unsignalled state.
 *
 * \param dev           - \c [in] device handle
 * \param syncobjs      - \c [in] array of sync object handles
 * \param syncobj_count - \c [in] number of handles in syncobjs
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_cs_syncobj_reset(amdgpu_device_handle dev,
			    const uint32_t *syncobjs, uint32_t syncobj_count);

/**
 * Signal kernel sync objects.
 *
 * \param dev           - \c [in] device handle
 * \param syncobjs      - \c [in] array of sync object handles
 * \param syncobj_count - \c [in] number of handles in syncobjs
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_cs_syncobj_signal(amdgpu_device_handle dev,
			     const uint32_t *syncobjs, uint32_t syncobj_count);

/**
 * Signal kernel timeline sync objects.
 *
 * \param dev           - \c [in] device handle
 * \param syncobjs      - \c [in] array of sync object handles
 * \param points	- \c [in] array of timeline points
 * \param syncobj_count - \c [in] number of handles in syncobjs
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_cs_syncobj_timeline_signal(amdgpu_device_handle dev,
				      const uint32_t *syncobjs,
				      uint64_t *points,
				      uint32_t syncobj_count);

/**
 *  Wait for one or all sync objects to signal.
 *
 * \param   dev	    - \c [in] self-explanatory
 * \param   handles - \c [in] array of sync object handles
 * \param   num_handles - \c [in] self-explanatory
 * \param   timeout_nsec - \c [in] self-explanatory
 * \param   flags   - \c [in] a bitmask of DRM_SYNCOBJ_WAIT_FLAGS_*
 * \param   first_signaled - \c [in] self-explanatory
 *
 * \return   0 on success\n
 *          -ETIME - Timeout
 *          <0 - Negative POSIX Error code
 *
 */
int amdgpu_cs_syncobj_wait(amdgpu_device_handle dev,
			   uint32_t *handles, unsigned num_handles,
			   int64_t timeout_nsec, unsigned flags,
			   uint32_t *first_signaled);

/**
 *  Wait for one or all sync objects on their points to signal.
 *
 * \param   dev	    - \c [in] self-explanatory
 * \param   handles - \c [in] array of sync object handles
 * \param   points - \c [in] array of sync points to wait
 * \param   num_handles - \c [in] self-explanatory
 * \param   timeout_nsec - \c [in] self-explanatory
 * \param   flags   - \c [in] a bitmask of DRM_SYNCOBJ_WAIT_FLAGS_*
 * \param   first_signaled - \c [in] self-explanatory
 *
 * \return   0 on success\n
 *          -ETIME - Timeout
 *          <0 - Negative POSIX Error code
 *
 */
int amdgpu_cs_syncobj_timeline_wait(amdgpu_device_handle dev,
				    uint32_t *handles, uint64_t *points,
				    unsigned num_handles,
				    int64_t timeout_nsec, unsigned flags,
				    uint32_t *first_signaled);
/**
 *  Query sync objects payloads.
 *
 * \param   dev	    - \c [in] self-explanatory
 * \param   handles - \c [in] array of sync object handles
 * \param   points - \c [out] array of sync points returned, which presents
 * syncobj payload.
 * \param   num_handles - \c [in] self-explanatory
 *
 * \return   0 on success\n
 *          -ETIME - Timeout
 *          <0 - Negative POSIX Error code
 *
 */
int amdgpu_cs_syncobj_query(amdgpu_device_handle dev,
			    uint32_t *handles, uint64_t *points,
			    unsigned num_handles);
/**
 *  Query sync objects last signaled or submitted point.
 *
 * \param   dev	    - \c [in] self-explanatory
 * \param   handles - \c [in] array of sync object handles
 * \param   points - \c [out] array of sync points returned, which presents
 * syncobj payload.
 * \param   num_handles - \c [in] self-explanatory
 * \param   flags   - \c [in] a bitmask of DRM_SYNCOBJ_QUERY_FLAGS_*
 *
 * \return   0 on success\n
 *          -ETIME - Timeout
 *          <0 - Negative POSIX Error code
 *
 */
int amdgpu_cs_syncobj_query2(amdgpu_device_handle dev,
			     uint32_t *handles, uint64_t *points,
			     unsigned num_handles, uint32_t flags);

/**
 *  Export kernel sync object to shareable fd.
 *
 * \param   dev	       - \c [in] device handle
 * \param   syncobj    - \c [in] sync object handle
 * \param   shared_fd  - \c [out] shared file descriptor.
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_cs_export_syncobj(amdgpu_device_handle dev,
			     uint32_t syncobj,
			     int *shared_fd);
/**
 *  Import kernel sync object from shareable fd.
 *
 * \param   dev	       - \c [in] device handle
 * \param   shared_fd  - \c [in] shared file descriptor.
 * \param   syncobj    - \c [out] sync object handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
*/
int amdgpu_cs_import_syncobj(amdgpu_device_handle dev,
			     int shared_fd,
			     uint32_t *syncobj);

/**
 *  Export kernel sync object to a sync_file.
 *
 * \param   dev	       - \c [in] device handle
 * \param   syncobj    - \c [in] sync object handle
 * \param   sync_file_fd - \c [out] sync_file file descriptor.
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 */
int amdgpu_cs_syncobj_export_sync_file(amdgpu_device_handle dev,
				       uint32_t syncobj,
				       int *sync_file_fd);

/**
 *  Import kernel sync object from a sync_file.
 *
 * \param   dev	       - \c [in] device handle
 * \param   syncobj    - \c [in] sync object handle
 * \param   sync_file_fd - \c [in] sync_file file descriptor.
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 */
int amdgpu_cs_syncobj_import_sync_file(amdgpu_device_handle dev,
				       uint32_t syncobj,
				       int sync_file_fd);
/**
 *  Export kernel timeline sync object to a sync_file.
 *
 * \param   dev		- \c [in] device handle
 * \param   syncobj	- \c [in] sync object handle
 * \param   point	- \c [in] timeline point
 * \param   flags	- \c [in] flags
 * \param   sync_file_fd - \c [out] sync_file file descriptor.
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 */
int amdgpu_cs_syncobj_export_sync_file2(amdgpu_device_handle dev,
					uint32_t syncobj,
					uint64_t point,
					uint32_t flags,
					int *sync_file_fd);

/**
 *  Import kernel timeline sync object from a sync_file.
 *
 * \param   dev		- \c [in] device handle
 * \param   syncobj	- \c [in] sync object handle
 * \param   point	- \c [in] timeline point
 * \param   sync_file_fd - \c [in] sync_file file descriptor.
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 */
int amdgpu_cs_syncobj_import_sync_file2(amdgpu_device_handle dev,
					uint32_t syncobj,
					uint64_t point,
					int sync_file_fd);

/**
 *  transfer between syncbojs.
 *
 * \param   dev		- \c [in] device handle
 * \param   dst_handle	- \c [in] sync object handle
 * \param   dst_point	- \c [in] timeline point, 0 presents dst is binary
 * \param   src_handle	- \c [in] sync object handle
 * \param   src_point	- \c [in] timeline point, 0 presents src is binary
 * \param   flags	- \c [in] flags
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 */
int amdgpu_cs_syncobj_transfer(amdgpu_device_handle dev,
			       uint32_t dst_handle,
			       uint64_t dst_point,
			       uint32_t src_handle,
			       uint64_t src_point,
			       uint32_t flags);

/**
 * Export an amdgpu fence as a handle (syncobj or fd).
 *
 * \param what		AMDGPU_FENCE_TO_HANDLE_GET_{SYNCOBJ, FD}
 * \param out_handle	returned handle
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 */
int amdgpu_cs_fence_to_handle(amdgpu_device_handle dev,
			      struct amdgpu_cs_fence *fence,
			      uint32_t what,
			      uint32_t *out_handle);

/**
 *  Submit raw command submission to kernel
 *
 * \param   dev	       - \c [in] device handle
 * \param   context    - \c [in] context handle for context id
 * \param   bo_list_handle - \c [in] request bo list handle (0 for none)
 * \param   num_chunks - \c [in] number of CS chunks to submit
 * \param   chunks     - \c [in] array of CS chunks
 * \param   seq_no     - \c [out] output sequence number for submission.
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 */
struct drm_amdgpu_cs_chunk;
struct drm_amdgpu_cs_chunk_dep;
struct drm_amdgpu_cs_chunk_data;

int amdgpu_cs_submit_raw(amdgpu_device_handle dev,
			 amdgpu_context_handle context,
			 amdgpu_bo_list_handle bo_list_handle,
			 int num_chunks,
			 struct drm_amdgpu_cs_chunk *chunks,
			 uint64_t *seq_no);

/**
 * Submit raw command submission to the kernel with a raw BO list handle.
 *
 * \param   dev	       - \c [in] device handle
 * \param   context    - \c [in] context handle for context id
 * \param   bo_list_handle - \c [in] raw bo list handle (0 for none)
 * \param   num_chunks - \c [in] number of CS chunks to submit
 * \param   chunks     - \c [in] array of CS chunks
 * \param   seq_no     - \c [out] output sequence number for submission.
 *
 * \return   0 on success\n
 *          <0 - Negative POSIX Error code
 *
 * \sa amdgpu_bo_list_create_raw(), amdgpu_bo_list_destroy_raw()
 */
int amdgpu_cs_submit_raw2(amdgpu_device_handle dev,
			  amdgpu_context_handle context,
			  uint32_t bo_list_handle,
			  int num_chunks,
			  struct drm_amdgpu_cs_chunk *chunks,
			  uint64_t *seq_no);

void amdgpu_cs_chunk_fence_to_dep(struct amdgpu_cs_fence *fence,
				  struct drm_amdgpu_cs_chunk_dep *dep);
void amdgpu_cs_chunk_fence_info_to_data(struct amdgpu_cs_fence_info *fence_info,
					struct drm_amdgpu_cs_chunk_data *data);

/**
 * Reserve VMID
 * \param   context - \c [in]  GPU Context
 * \param   flags - \c [in]  TBD
 *
 * \return  0 on success otherwise POSIX Error code
*/
int amdgpu_vm_reserve_vmid(amdgpu_device_handle dev, uint32_t flags);

/**
 * Free reserved VMID
 * \param   context - \c [in]  GPU Context
 * \param   flags - \c [in]  TBD
 *
 * \return  0 on success otherwise POSIX Error code
*/
int amdgpu_vm_unreserve_vmid(amdgpu_device_handle dev, uint32_t flags);

#ifdef __cplusplus
}
#endif
#endif /* #ifdef _AMDGPU_H_ */
