/*
 * Copyright 1999 Precision Insight, Inc., Cedar Park, Texas.
 * Copyright 2000 VA Linux Systems, Inc., Sunnyvale, California.
 * Copyright (c) 2003-2008 Tungsten Graphics, Inc., Cedar Park, TX., USA
 * Copyright (c) 2007 Dave Airlie <airlied@linux.ie>
 * Copyright (c) 2007 Jakob Bornecrantz <wallbraker@gmail.com>
 * Copyright (c) 2007-2008 Intel Corporation
 * Copyright (c) 2008 Red Hat Inc.
 * All rights reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * VA LINUX SYSTEMS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

#ifndef __VKI_LINUX_DRM_H
#define __VKI_LINUX_DRM_H

//----------------------------------------------------------------------
// From include/drm/drm.h
//----------------------------------------------------------------------

typedef unsigned int vki_drm_context_t;
typedef unsigned int vki_drm_drawable_t;
typedef unsigned int vki_drm_magic_t;

struct vki_drm_clip_rect {
	unsigned short x1;
	unsigned short y1;
	unsigned short x2;
	unsigned short y2;
};
struct vki_drm_version {
	int version_major;	  /**< Major version */
	int version_minor;	  /**< Minor version */
	int version_patchlevel;	  /**< Patch level */
	vki_size_t name_len;	  /**< Length of name buffer */
	char __user *name;	  /**< Name of driver */
	vki_size_t date_len;	  /**< Length of date buffer */
	char __user *date;	  /**< User-space buffer to hold date */
	vki_size_t desc_len;	  /**< Length of desc buffer */
	char __user *desc;	  /**< User-space buffer to hold desc */
};
struct vki_drm_unique {
	vki_size_t unique_len;	  /**< Length of unique */
	char __user *unique;	  /**< Unique name for driver instantiation */
};
struct vki_drm_block {
	int unused;
};
struct vki_drm_control {
	enum {
		VKI_DRM_ADD_COMMAND,
		VKI_DRM_RM_COMMAND,
		VKI_DRM_INST_HANDLER,
		VKI_DRM_UNINST_HANDLER
	} func;
	int irq;
};

enum vki_drm_map_type {
	_VKI_DRM_FRAME_BUFFER = 0,	  /**< WC (no caching), no core dump */
	_VKI_DRM_REGISTERS = 1,	  /**< no caching, no core dump */
	_VKI_DRM_SHM = 2,		  /**< shared, cached */
	_VKI_DRM_AGP = 3,		  /**< AGP/GART */
	_VKI_DRM_SCATTER_GATHER = 4,  /**< Scatter/gather memory for PCI DMA */
	_VKI_DRM_CONSISTENT = 5,	  /**< Consistent memory for PCI DMA */
	_VKI_DRM_GEM = 6,		  /**< GEM object */
};
enum vki_drm_map_flags {
	_VKI_DRM_RESTRICTED = 0x01,	     /**< Cannot be mapped to user-virtual */
	_VKI_DRM_READ_ONLY = 0x02,
	_VKI_DRM_LOCKED = 0x04,	     /**< shared, cached, locked */
	_VKI_DRM_KERNEL = 0x08,	     /**< kernel requires access */
	_VKI_DRM_WRITE_COMBINING = 0x10, /**< use write-combining if available */
	_VKI_DRM_CONTAINS_LOCK = 0x20,   /**< SHM page that contains lock */
	_VKI_DRM_REMOVABLE = 0x40,	     /**< Removable mapping */
	_VKI_DRM_DRIVER = 0x80	     /**< Managed by driver */
};
struct vki_drm_ctx_priv_map {
	unsigned int ctx_id;	 /**< Context requesting private mapping */
	void *handle;		 /**< Handle of map */
};
struct vki_drm_map {
	unsigned long offset;	 /**< Requested physical address (0 for SAREA)*/
	unsigned long size;	 /**< Requested physical size (bytes) */
	enum vki_drm_map_type type;	 /**< Type of memory to map */
	enum vki_drm_map_flags flags;	 /**< Flags */
	void *handle;		 /**< User-space: "Handle" to pass to mmap() */
				 /**< Kernel-space: kernel-virtual address */
	int mtrr;		 /**< MTRR slot used */
	/*   Private data */
};
struct vki_drm_client {
	int idx;		/**< Which client desired? */
	int auth;		/**< Is client authenticated? */
	unsigned long pid;	/**< Process ID */
	unsigned long uid;	/**< User ID */
	unsigned long magic;	/**< Magic */
	unsigned long iocs;	/**< Ioctl count */
};
enum vki_drm_stat_type {
	_VKI_DRM_STAT_LOCK,
	_VKI_DRM_STAT_OPENS,
	_VKI_DRM_STAT_CLOSES,
	_VKI_DRM_STAT_IOCTLS,
	_VKI_DRM_STAT_LOCKS,
	_VKI_DRM_STAT_UNLOCKS,
	_VKI_DRM_STAT_VALUE,	/**< Generic value */
	_VKI_DRM_STAT_BYTE,		/**< Generic byte counter (1024bytes/K) */
	_VKI_DRM_STAT_COUNT,	/**< Generic non-byte counter (1000/k) */

	_VKI_DRM_STAT_IRQ,		/**< IRQ */
	_VKI_DRM_STAT_PRIMARY,	/**< Primary DMA bytes */
	_VKI_DRM_STAT_SECONDARY,	/**< Secondary DMA bytes */
	_VKI_DRM_STAT_DMA,		/**< DMA */
	_VKI_DRM_STAT_SPECIAL,	/**< Special DMA (e.g., priority or polled) */
	_VKI_DRM_STAT_MISSED	/**< Missed DMA opportunity */
	    /* Add to the *END* of the list */
};
struct vki_drm_stats {
	unsigned long count;
	struct {
		unsigned long value;
		enum vki_drm_stat_type type;
	} data[15];
};
enum vki_drm_lock_flags {
	_VKI_DRM_LOCK_READY = 0x01,	     /**< Wait until hardware is ready for DMA */
	_VKI_DRM_LOCK_QUIESCENT = 0x02,  /**< Wait until hardware quiescent */
	_VKI_DRM_LOCK_FLUSH = 0x04,	     /**< Flush this context's DMA queue first */
	_VKI_DRM_LOCK_FLUSH_ALL = 0x08,  /**< Flush all DMA queues first */
	/* These *HALT* flags aren't supported yet
	   -- they will be used to support the
	   full-screen DGA-like mode. */
	_VKI_DRM_HALT_ALL_QUEUES = 0x10, /**< Halt all current and future queues */
	_VKI_DRM_HALT_CUR_QUEUES = 0x20  /**< Halt all current queues */
};
struct vki_drm_lock {
	int context;
	enum vki_drm_lock_flags flags;
};
enum vki_drm_dma_flags {
	/* Flags for DMA buffer dispatch */
	_VKI_DRM_DMA_BLOCK = 0x01,	      /**<
				       * Block until buffer dispatched.
				       *
				       * \note The buffer may not yet have
				       * been processed by the hardware --
				       * getting a hardware lock with the
				       * hardware quiescent will ensure
				       * that the buffer has been
				       * processed.
				       */
	_VKI_DRM_DMA_WHILE_LOCKED = 0x02, /**< Dispatch while lock held */
	_VKI_DRM_DMA_PRIORITY = 0x04,     /**< High priority dispatch */

	/* Flags for DMA buffer request */
	_VKI_DRM_DMA_WAIT = 0x10,	      /**< Wait for free buffers */
	_VKI_DRM_DMA_SMALLER_OK = 0x20,   /**< Smaller-than-requested buffers OK */
	_VKI_DRM_DMA_LARGER_OK = 0x40     /**< Larger-than-requested buffers OK */
};
struct vki_drm_buf_desc {
	int count;		 /**< Number of buffers of this size */
	int size;		 /**< Size in bytes */
	int low_mark;		 /**< Low water mark */
	int high_mark;		 /**< High water mark */
	enum {
		_VKI_DRM_PAGE_ALIGN = 0x01,	/**< Align on page boundaries for DMA */
		_VKI_DRM_AGP_BUFFER = 0x02,	/**< Buffer is in AGP space */
		_VKI_DRM_SG_BUFFER = 0x04,	/**< Scatter/gather memory buffer */
		_VKI_DRM_FB_BUFFER = 0x08,	/**< Buffer is in frame buffer */
		_VKI_DRM_PCI_BUFFER_RO = 0x10 /**< Map PCI DMA buffer read-only */
	} flags;
	unsigned long agp_start; /**<
				  * Start address of where the AGP buffers are
				  * in the AGP aperture
				  */
};
struct vki_drm_buf_info {
	int count;		/**< Entries in list */
	struct vki_drm_buf_desc __user *list;
};
struct vki_drm_buf_free {
	int count;
	int __user *list;
};

struct vki_drm_buf_pub {
	int idx;		       /**< Index into the master buffer list */
	int total;		       /**< Buffer size */
	int used;		       /**< Amount of buffer in use (for DMA) */
	void __user *address;	       /**< Address of buffer */
};
struct vki_drm_buf_map {
	int count;		/**< Length of the buffer list */
	void __user *virtuaL;		/**< Mmap'd area in user-virtual */
	struct vki_drm_buf_pub __user *list;	/**< Buffer information */
};
struct vki_drm_dma {
	int context;			  /**< Context handle */
	int send_count;			  /**< Number of buffers to send */
	int __user *send_indices;	  /**< List of handles to buffers */
	int __user *send_sizes;		  /**< Lengths of data to send */
	enum vki_drm_dma_flags flags;	  /**< Flags */
	int request_count;		  /**< Number of buffers requested */
	int request_size;		  /**< Desired size for buffers */
	int __user *request_indices;	  /**< Buffer information */
	int __user *request_sizes;
	int granted_count;		  /**< Number of buffers granted */
};

enum vki_drm_ctx_flags {
	_VKI_DRM_CONTEXT_PRESERVED = 0x01,
	_VKI_DRM_CONTEXT_2DONLY = 0x02
};
struct vki_drm_ctx {
	vki_drm_context_t handle;
	enum vki_drm_ctx_flags flags;
};
struct vki_drm_ctx_res {
	int count;
	struct vki_drm_ctx __user *contexts;
};
struct vki_drm_draw {
	vki_drm_drawable_t handle;
};
typedef enum {
	VKI_DRM_DRAWABLE_CLIPRECTS,
} vki_drm_drawable_info_type_t;
struct vki_drm_update_draw {
	vki_drm_drawable_t handle;
	unsigned int type;
	unsigned int num;
	unsigned long long data;
};
struct vki_drm_auth {
	vki_drm_magic_t magic;
};
struct vki_drm_irq_busid {
	int irq;	/**< IRQ number */
	int busnum;	/**< bus number */
	int devnum;	/**< device number */
	int funcnum;	/**< function number */
};
enum vki_drm_vblank_seq_type {
	_VKI_DRM_VBLANK_ABSOLUTE = 0x0,	/**< Wait for specific vblank sequence number */
	_VKI_DRM_VBLANK_RELATIVE = 0x1,	/**< Wait for given number of vblanks */
	_VKI_DRM_VBLANK_HIGH_CRTC_MASK = 0x0000003e,
	_VKI_DRM_VBLANK_EVENT = 0x4000000,   /**< Send event instead of blocking */
	_VKI_DRM_VBLANK_FLIP = 0x8000000,   /**< Scheduled buffer swap should flip */
	_VKI_DRM_VBLANK_NEXTONMISS = 0x10000000,	/**< If missed, wait for next vblank */
	_VKI_DRM_VBLANK_SECONDARY = 0x20000000,	/**< Secondary display controller */
	_VKI_DRM_VBLANK_SIGNAL = 0x40000000	/**< Send signal instead of blocking, unsupported */
};
struct vki_drm_wait_vblank_request {
	enum vki_drm_vblank_seq_type type;
	unsigned int sequence;
	unsigned long signal;
};
struct vki_drm_wait_vblank_reply {
	enum vki_drm_vblank_seq_type type;
	unsigned int sequence;
	long tval_sec;
	long tval_usec;
};
union vki_drm_wait_vblank {
	struct vki_drm_wait_vblank_request request;
	struct vki_drm_wait_vblank_reply reply;
};
struct vki_drm_modeset_ctl {
	__vki_u32 crtc;
	__vki_u32 cmd;
};
struct vki_drm_agp_mode {
	unsigned long mode;	/**< AGP mode */
};
struct vki_drm_agp_buffer {
	unsigned long size;	/**< In bytes -- will round to page boundary */
	unsigned long handle;	/**< Used for binding / unbinding */
	unsigned long type;	/**< Type of memory to allocate */
	unsigned long physical;	/**< Physical used by i810 */
};
struct vki_drm_agp_binding {
	unsigned long handle;	/**< From drm_agp_buffer */
	unsigned long offset;	/**< In bytes -- will round to page boundary */
};
struct vki_drm_agp_info {
	int agp_version_major;
	int agp_version_minor;
	unsigned long mode;
	unsigned long aperture_base;	/* physical address */
	unsigned long aperture_size;	/* bytes */
	unsigned long memory_allowed;	/* bytes */
	unsigned long memory_used;
	unsigned short id_vendor;
	unsigned short id_device;
};
struct vki_drm_scatter_gather {
	unsigned long size;	/**< In bytes -- will round to page boundary */
	unsigned long handle;	/**< Used for mapping / unmapping */
};

struct vki_drm_set_version {
	int drm_di_major;
	int drm_di_minor;
	int drm_dd_major;
	int drm_dd_minor;
};
struct vki_drm_gem_close {
	__vki_u32 handle;
	__vki_u32 pad;
};
struct vki_drm_gem_flink {
	__vki_u32 handle;
	__vki_u32 name;
};
struct vki_drm_gem_open {
	__vki_u32 name;
	__vki_u32 handle;
	__vki_u64 size;
};

//----------------------------------------------------------------------
// From include/drm/drm_mode.h
//----------------------------------------------------------------------

#define VKI_DRM_DISPLAY_MODE_LEN 32
#define VKI_DRM_PROP_NAME_LEN 32
struct vki_drm_mode_modeinfo {
	__vki_u32 clock;
	__vki_u16 hdisplay, hsync_start, hsync_end, htotal, hskew;
	__vki_u16 vdisplay, vsync_start, vsync_end, vtotal, vscan;

	__vki_u32 vrefresh; /* vertical refresh * 1000 */

	__vki_u32 flags;
	__vki_u32 type;
	char name[VKI_DRM_DISPLAY_MODE_LEN];
};
struct vki_drm_mode_card_res {
	__vki_u64 fb_id_ptr;
	__vki_u64 crtc_id_ptr;
	__vki_u64 connector_id_ptr;
	__vki_u64 encoder_id_ptr;
	__vki_u32 count_fbs;
	__vki_u32 count_crtcs;
	__vki_u32 count_connectors;
	__vki_u32 count_encoders;
	__vki_u32 min_width, max_width;
	__vki_u32 min_height, max_height;
};
struct vki_drm_mode_crtc {
	__vki_u64 set_connectors_ptr;
	__vki_u32 count_connectors;

	__vki_u32 crtc_id; /**< Id */
	__vki_u32 fb_id; /**< Id of framebuffer */

	__vki_u32 x, y; /**< Position on the frameuffer */

	__vki_u32 gamma_size;
	__vki_u32 mode_valid;
	struct vki_drm_mode_modeinfo mode;
};
struct vki_drm_mode_get_encoder {
	__vki_u32 encoder_id;
	__vki_u32 encoder_type;

	__vki_u32 crtc_id; /**< Id of crtc */

	__vki_u32 possible_crtcs;
	__vki_u32 possible_clones;
};
struct vki_drm_mode_get_property {
	__vki_u64 values_ptr; /* values and blob lengths */
	__vki_u64 enum_blob_ptr; /* enum and blob id ptrs */

	__vki_u32 prop_id;
	__vki_u32 flags;
	char name[VKI_DRM_PROP_NAME_LEN];

	__vki_u32 count_values;
	__vki_u32 count_enum_blobs;
};
struct vki_drm_mode_connector_set_property {
	__vki_u64 value;
	__vki_u32 prop_id;
	__vki_u32 connector_id;
};
struct vki_drm_mode_get_blob {
	__vki_u32 blob_id;
	__vki_u32 length;
	__vki_u64 data;
};
struct vki_drm_mode_fb_cmd {
	__vki_u32 fb_id;
	__vki_u32 width, height;
	__vki_u32 pitch;
	__vki_u32 bpp;
	__vki_u32 depth;
	/* driver specific handle */
	__vki_u32 handle;
};
struct vki_drm_mode_mode_cmd {
	__vki_u32 connector_id;
	struct vki_drm_mode_modeinfo mode;
};
struct vki_drm_mode_cursor {
	__vki_u32 flags;
	__vki_u32 crtc_id;
	__vki_s32 x;
	__vki_s32 y;
	__vki_u32 width;
	__vki_u32 height;
	/* driver specific handle */
	__vki_u32 handle;
};
struct vki_drm_mode_crtc_lut {
	__vki_u32 crtc_id;
	__vki_u32 gamma_size;

	/* pointers to arrays */
	__vki_u64 red;
	__vki_u64 green;
	__vki_u64 blue;
};

//----------------------------------------------------------------------
// From include/drm/drm.h
//----------------------------------------------------------------------

#define VKI_DRM_IOCTL_BASE		'd'

#define VKI_DRM_IO(nr)			_VKI_IO(VKI_DRM_IOCTL_BASE,nr)
#define VKI_DRM_IOR(nr,type)		_VKI_IOR(VKI_DRM_IOCTL_BASE,nr,type)
#define VKI_DRM_IOW(nr,type)		_VKI_IOW(VKI_DRM_IOCTL_BASE,nr,type)
#define VKI_DRM_IOWR(nr,type)		_VKI_IOWR(VKI_DRM_IOCTL_BASE,nr,type)


#define VKI_DRM_IOCTL_VERSION		VKI_DRM_IOWR(0x00, struct vki_drm_version)
#define VKI_DRM_IOCTL_GET_UNIQUE	VKI_DRM_IOWR(0x01, struct vki_drm_unique)
#define VKI_DRM_IOCTL_GET_MAGIC		VKI_DRM_IOR( 0x02, struct vki_drm_auth)
#define VKI_DRM_IOCTL_IRQ_BUSID		VKI_DRM_IOWR(0x03, struct vki_drm_irq_busid)
#define VKI_DRM_IOCTL_GET_MAP           VKI_DRM_IOWR(0x04, struct vki_drm_map)
#define VKI_DRM_IOCTL_GET_CLIENT        VKI_DRM_IOWR(0x05, struct vki_drm_client)
#define VKI_DRM_IOCTL_GET_STATS         VKI_DRM_IOR( 0x06, struct vki_drm_stats)
#define VKI_DRM_IOCTL_SET_VERSION	VKI_DRM_IOWR(0x07, struct vki_drm_set_version)
#define VKI_DRM_IOCTL_MODESET_CTL       VKI_DRM_IOW(0x08, struct vki_drm_modeset_ctl)
#define VKI_DRM_IOCTL_GEM_CLOSE		VKI_DRM_IOW (0x09, struct vki_drm_gem_close)
#define VKI_DRM_IOCTL_GEM_FLINK		VKI_DRM_IOWR(0x0a, struct vki_drm_gem_flink)
#define VKI_DRM_IOCTL_GEM_OPEN		VKI_DRM_IOWR(0x0b, struct vki_drm_gem_open)

#define VKI_DRM_IOCTL_SET_UNIQUE	VKI_DRM_IOW( 0x10, struct vki_drm_unique)
#define VKI_DRM_IOCTL_AUTH_MAGIC	VKI_DRM_IOW( 0x11, struct vki_drm_auth)
#define VKI_DRM_IOCTL_BLOCK		VKI_DRM_IOWR(0x12, struct vki_drm_block)
#define VKI_DRM_IOCTL_UNBLOCK		VKI_DRM_IOWR(0x13, struct vki_drm_block)
#define VKI_DRM_IOCTL_CONTROL		VKI_DRM_IOW( 0x14, struct vki_drm_control)
#define VKI_DRM_IOCTL_ADD_MAP		VKI_DRM_IOWR(0x15, struct vki_drm_map)
#define VKI_DRM_IOCTL_ADD_BUFS		VKI_DRM_IOWR(0x16, struct vki_drm_buf_desc)
#define VKI_DRM_IOCTL_MARK_BUFS		VKI_DRM_IOW( 0x17, struct vki_drm_buf_desc)
#define VKI_DRM_IOCTL_INFO_BUFS		VKI_DRM_IOWR(0x18, struct vki_drm_buf_info)
#define VKI_DRM_IOCTL_MAP_BUFS		VKI_DRM_IOWR(0x19, struct vki_drm_buf_map)
#define VKI_DRM_IOCTL_FREE_BUFS		VKI_DRM_IOW( 0x1a, struct vki_drm_buf_free)

#define VKI_DRM_IOCTL_RM_MAP		VKI_DRM_IOW( 0x1b, struct vki_drm_map)

#define VKI_DRM_IOCTL_SET_SAREA_CTX	VKI_DRM_IOW( 0x1c, struct vki_drm_ctx_priv_map)
#define VKI_DRM_IOCTL_GET_SAREA_CTX 	VKI_DRM_IOWR(0x1d, struct vki_drm_ctx_priv_map)

#define VKI_DRM_IOCTL_SET_MASTER        VKI_DRM_IO(0x1e)
#define VKI_DRM_IOCTL_DROP_MASTER       VKI_DRM_IO(0x1f)

#define VKI_DRM_IOCTL_ADD_CTX		VKI_DRM_IOWR(0x20, struct vki_drm_ctx)
#define VKI_DRM_IOCTL_RM_CTX		VKI_DRM_IOWR(0x21, struct vki_drm_ctx)
#define VKI_DRM_IOCTL_MOD_CTX		VKI_DRM_IOW( 0x22, struct vki_drm_ctx)
#define VKI_DRM_IOCTL_GET_CTX		VKI_DRM_IOWR(0x23, struct vki_drm_ctx)
#define VKI_DRM_IOCTL_SWITCH_CTX	VKI_DRM_IOW( 0x24, struct vki_drm_ctx)
#define VKI_DRM_IOCTL_NEW_CTX		VKI_DRM_IOW( 0x25, struct vki_drm_ctx)
#define VKI_DRM_IOCTL_RES_CTX		VKI_DRM_IOWR(0x26, struct vki_drm_ctx_res)
#define VKI_DRM_IOCTL_ADD_DRAW		VKI_DRM_IOWR(0x27, struct vki_drm_draw)
#define VKI_DRM_IOCTL_RM_DRAW		VKI_DRM_IOWR(0x28, struct vki_drm_draw)
#define VKI_DRM_IOCTL_DMA		VKI_DRM_IOWR(0x29, struct vki_drm_dma)
#define VKI_DRM_IOCTL_LOCK		VKI_DRM_IOW( 0x2a, struct vki_drm_lock)
#define VKI_DRM_IOCTL_UNLOCK		VKI_DRM_IOW( 0x2b, struct vki_drm_lock)
#define VKI_DRM_IOCTL_FINISH		VKI_DRM_IOW( 0x2c, struct vki_drm_lock)

#define VKI_DRM_IOCTL_AGP_ACQUIRE	VKI_DRM_IO(  0x30)
#define VKI_DRM_IOCTL_AGP_RELEASE	VKI_DRM_IO(  0x31)
#define VKI_DRM_IOCTL_AGP_ENABLE	VKI_DRM_IOW( 0x32, struct vki_drm_agp_mode)
#define VKI_DRM_IOCTL_AGP_INFO		VKI_DRM_IOR( 0x33, struct vki_drm_agp_info)
#define VKI_DRM_IOCTL_AGP_ALLOC		VKI_DRM_IOWR(0x34, struct vki_drm_agp_buffer)
#define VKI_DRM_IOCTL_AGP_FREE		VKI_DRM_IOW( 0x35, struct vki_drm_agp_buffer)
#define VKI_DRM_IOCTL_AGP_BIND		VKI_DRM_IOW( 0x36, struct vki_drm_agp_binding)
#define VKI_DRM_IOCTL_AGP_UNBIND	VKI_DRM_IOW( 0x37, struct vki_drm_agp_binding)

#define VKI_DRM_IOCTL_SG_ALLOC		VKI_DRM_IOWR(0x38, struct vki_drm_scatter_gather)
#define VKI_DRM_IOCTL_SG_FREE		VKI_DRM_IOW( 0x39, struct vki_drm_scatter_gather)

#define VKI_DRM_IOCTL_WAIT_VBLANK	VKI_DRM_IOWR(0x3a, union vki_drm_wait_vblank)

#define VKI_DRM_IOCTL_UPDATE_DRAW	VKI_DRM_IOW(0x3f, struct vki_drm_update_draw)

#define VKI_DRM_IOCTL_MODE_GETRESOURCES	VKI_DRM_IOWR(0xA0, struct vki_drm_mode_card_res)
#define VKI_DRM_IOCTL_MODE_GETCRTC	VKI_DRM_IOWR(0xA1, struct vki_drm_mode_crtc)
#define VKI_DRM_IOCTL_MODE_SETCRTC	VKI_DRM_IOWR(0xA2, struct vki_drm_mode_crtc)
#define VKI_DRM_IOCTL_MODE_CURSOR	VKI_DRM_IOWR(0xA3, struct vki_drm_mode_cursor)
#define VKI_DRM_IOCTL_MODE_GETGAMMA	VKI_DRM_IOWR(0xA4, struct vki_drm_mode_crtc_lut)
#define VKI_DRM_IOCTL_MODE_SETGAMMA	VKI_DRM_IOWR(0xA5, struct vki_drm_mode_crtc_lut)
#define VKI_DRM_IOCTL_MODE_GETENCODER	VKI_DRM_IOWR(0xA6, struct vki_drm_mode_get_encoder)
#define VKI_DRM_IOCTL_MODE_GETCONNECTOR	VKI_DRM_IOWR(0xA7, struct vki_drm_mode_get_connector)
#define VKI_DRM_IOCTL_MODE_ATTACHMODE	VKI_DRM_IOWR(0xA8, struct vki_drm_mode_mode_cmd)
#define VKI_DRM_IOCTL_MODE_DETACHMODE	VKI_DRM_IOWR(0xA9, struct vki_drm_mode_mode_cmd)

#define VKI_DRM_IOCTL_MODE_GETPROPERTY	VKI_DRM_IOWR(0xAA, struct vki_drm_mode_get_property)
#define VKI_DRM_IOCTL_MODE_SETPROPERTY	VKI_DRM_IOWR(0xAB, struct vki_drm_mode_connector_set_property)
#define VKI_DRM_IOCTL_MODE_GETPROPBLOB	VKI_DRM_IOWR(0xAC, struct vki_drm_mode_get_blob)
#define VKI_DRM_IOCTL_MODE_GETFB	VKI_DRM_IOWR(0xAD, struct vki_drm_mode_fb_cmd)
#define VKI_DRM_IOCTL_MODE_ADDFB	VKI_DRM_IOWR(0xAE, struct vki_drm_mode_fb_cmd)
#define VKI_DRM_IOCTL_MODE_RMFB		VKI_DRM_IOWR(0xAF, unsigned int)

#define VKI_DRM_COMMAND_BASE            0x40
#define VKI_DRM_COMMAND_END		0xA0

//----------------------------------------------------------------------
// From include/drm/i915_drm.h
//----------------------------------------------------------------------

typedef struct _vki_drm_i915_init {
	enum {
		VKI_I915_INIT_DMA = 0x01,
		VKI_I915_CLEANUP_DMA = 0x02,
		VKI_I915_RESUME_DMA = 0x03
	} func;
	unsigned int mmio_offset;
	int sarea_priv_offset;
	unsigned int ring_start;
	unsigned int ring_end;
	unsigned int ring_size;
	unsigned int front_offset;
	unsigned int back_offset;
	unsigned int depth_offset;
	unsigned int w;
	unsigned int h;
	unsigned int pitch;
	unsigned int pitch_bits;
	unsigned int back_pitch;
	unsigned int depth_pitch;
	unsigned int cpp;
	unsigned int chipset;
} vki_drm_i915_init_t;

#define VKI_DRM_I915_INIT		    0x00
#define VKI_DRM_I915_FLUSH		    0x01
#define VKI_DRM_I915_FLIP		    0x02
#define VKI_DRM_I915_BATCHBUFFER	    0x03
#define VKI_DRM_I915_IRQ_EMIT		    0x04
#define VKI_DRM_I915_IRQ_WAIT		    0x05
#define VKI_DRM_I915_GETPARAM		    0x06
#define VKI_DRM_I915_SETPARAM		    0x07
#define VKI_DRM_I915_ALLOC		    0x08
#define VKI_DRM_I915_FREE		    0x09
#define VKI_DRM_I915_INIT_HEAP		    0x0a
#define VKI_DRM_I915_CMDBUFFER		    0x0b
#define VKI_DRM_I915_DESTROY_HEAP	    0x0c
#define VKI_DRM_I915_SET_VBLANK_PIPE	    0x0d
#define VKI_DRM_I915_GET_VBLANK_PIPE	    0x0e
#define VKI_DRM_I915_VBLANK_SWAP	    0x0f
#define VKI_DRM_I915_HWS_ADDR		    0x11
#define VKI_DRM_I915_GEM_INIT		    0x13
#define VKI_DRM_I915_GEM_EXECBUFFER	    0x14
#define VKI_DRM_I915_GEM_PIN		    0x15
#define VKI_DRM_I915_GEM_UNPIN		    0x16
#define VKI_DRM_I915_GEM_BUSY		    0x17
#define VKI_DRM_I915_GEM_THROTTLE	    0x18
#define VKI_DRM_I915_GEM_ENTERVT	    0x19
#define VKI_DRM_I915_GEM_LEAVEVT	    0x1a
#define VKI_DRM_I915_GEM_CREATE		    0x1b
#define VKI_DRM_I915_GEM_PREAD		    0x1c
#define VKI_DRM_I915_GEM_PWRITE		    0x1d
#define VKI_DRM_I915_GEM_MMAP		    0x1e
#define VKI_DRM_I915_GEM_SET_DOMAIN	    0x1f
#define VKI_DRM_I915_GEM_SW_FINISH	    0x20
#define VKI_DRM_I915_GEM_SET_TILING	    0x21
#define VKI_DRM_I915_GEM_GET_TILING	    0x22
#define VKI_DRM_I915_GEM_GET_APERTURE	    0x23
#define VKI_DRM_I915_GEM_MMAP_GTT	    0x24
#define VKI_DRM_I915_GET_PIPE_FROM_CRTC_ID  0x25
#define VKI_DRM_I915_GEM_MADVISE	    0x26
#define VKI_DRM_I915_OVERLAY_PUT_IMAGE	    0x27
#define VKI_DRM_I915_OVERLAY_ATTRS	    0x28
#define VKI_DRM_I915_GEM_EXECBUFFER2	    0x29

#define VKI_DRM_IOCTL_I915_INIT			    VKI_DRM_IOW( VKI_DRM_COMMAND_BASE + VKI_DRM_I915_INIT, vki_drm_i915_init_t)
#define VKI_DRM_IOCTL_I915_FLUSH		    VKI_DRM_IO ( VKI_DRM_COMMAND_BASE + VKI_DRM_I915_FLUSH)
#define VKI_DRM_IOCTL_I915_FLIP			    VKI_DRM_IO ( VKI_DRM_COMMAND_BASE + VKI_DRM_I915_FLIP)
#define VKI_DRM_IOCTL_I915_BATCHBUFFER		    VKI_DRM_IOW( VKI_DRM_COMMAND_BASE + VKI_DRM_I915_BATCHBUFFER, vki_drm_i915_batchbuffer_t)
#define VKI_DRM_IOCTL_I915_IRQ_EMIT		    VKI_DRM_IOWR(VKI_DRM_COMMAND_BASE + VKI_DRM_I915_IRQ_EMIT, vki_drm_i915_irq_emit_t)
#define VKI_DRM_IOCTL_I915_IRQ_WAIT		    VKI_DRM_IOW( VKI_DRM_COMMAND_BASE + VKI_DRM_I915_IRQ_WAIT, vki_drm_i915_irq_wait_t)
#define VKI_DRM_IOCTL_I915_GETPARAM		    VKI_DRM_IOWR(VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GETPARAM, vki_drm_i915_getparam_t)
#define VKI_DRM_IOCTL_I915_SETPARAM		    VKI_DRM_IOW( VKI_DRM_COMMAND_BASE + VKI_DRM_I915_SETPARAM, vki_drm_i915_setparam_t)
#define VKI_DRM_IOCTL_I915_ALLOC		    VKI_DRM_IOWR(VKI_DRM_COMMAND_BASE + VKI_DRM_I915_ALLOC, vki_drm_i915_mem_alloc_t)
#define VKI_DRM_IOCTL_I915_FREE			    VKI_DRM_IOW( VKI_DRM_COMMAND_BASE + VKI_DRM_I915_FREE, vki_drm_i915_mem_free_t)
#define VKI_DRM_IOCTL_I915_INIT_HEAP		    VKI_DRM_IOW( VKI_DRM_COMMAND_BASE + VKI_DRM_I915_INIT_HEAP, vki_drm_i915_mem_init_heap_t)
#define VKI_DRM_IOCTL_I915_CMDBUFFER		    VKI_DRM_IOW( VKI_DRM_COMMAND_BASE + VKI_DRM_I915_CMDBUFFER, vki_drm_i915_cmdbuffer_t)
#define VKI_DRM_IOCTL_I915_DESTROY_HEAP		    VKI_DRM_IOW( VKI_DRM_COMMAND_BASE + VKI_DRM_I915_DESTROY_HEAP, vki_drm_i915_mem_destroy_heap_t)
#define VKI_DRM_IOCTL_I915_SET_VBLANK_PIPE	    VKI_DRM_IOW( VKI_DRM_COMMAND_BASE + VKI_DRM_I915_SET_VBLANK_PIPE, vki_drm_i915_vblank_pipe_t)
#define VKI_DRM_IOCTL_I915_GET_VBLANK_PIPE	    VKI_DRM_IOR( VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GET_VBLANK_PIPE, vki_drm_i915_vblank_pipe_t)
#define VKI_DRM_IOCTL_I915_VBLANK_SWAP		    VKI_DRM_IOWR(VKI_DRM_COMMAND_BASE + VKI_DRM_I915_VBLANK_SWAP, vki_drm_i915_vblank_swap_t)
#define VKI_DRM_IOCTL_I915_GEM_INIT		    VKI_DRM_IOW(VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_INIT, struct vki_drm_i915_gem_init)
#define VKI_DRM_IOCTL_I915_GEM_EXECBUFFER	    VKI_DRM_IOW(VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_EXECBUFFER, struct vki_drm_i915_gem_execbuffer)
#define VKI_DRM_IOCTL_I915_GEM_PIN		    VKI_DRM_IOWR(VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_PIN, struct vki_drm_i915_gem_pin)
#define VKI_DRM_IOCTL_I915_GEM_UNPIN		    VKI_DRM_IOW(VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_UNPIN, struct vki_drm_i915_gem_unpin)
#define VKI_DRM_IOCTL_I915_GEM_BUSY		    VKI_DRM_IOWR(VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_BUSY, struct vki_drm_i915_gem_busy)
#define VKI_DRM_IOCTL_I915_GEM_THROTTLE		    VKI_DRM_IO ( VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_THROTTLE)
#define VKI_DRM_IOCTL_I915_GEM_ENTERVT		    VKI_DRM_IO(VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_ENTERVT)
#define VKI_DRM_IOCTL_I915_GEM_LEAVEVT		    VKI_DRM_IO(VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_LEAVEVT)
#define VKI_DRM_IOCTL_I915_GEM_CREATE		    VKI_DRM_IOWR(VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_CREATE, struct vki_drm_i915_gem_create)
#define VKI_DRM_IOCTL_I915_GEM_PREAD		    VKI_DRM_IOW (VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_PREAD, struct vki_drm_i915_gem_pread)
#define VKI_DRM_IOCTL_I915_GEM_PWRITE		    VKI_DRM_IOW (VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_PWRITE, struct vki_drm_i915_gem_pwrite)
#define VKI_DRM_IOCTL_I915_GEM_MMAPv1		    VKI_DRM_IOWR(VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_MMAP, struct vki_drm_i915_gem_mmap_v1)
#define VKI_DRM_IOCTL_I915_GEM_MMAP		    VKI_DRM_IOWR(VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_MMAP, struct vki_drm_i915_gem_mmap)
#define VKI_DRM_IOCTL_I915_GEM_MMAP_GTT		    VKI_DRM_IOWR(VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_MMAP_GTT, struct vki_drm_i915_gem_mmap_gtt)
#define VKI_DRM_IOCTL_I915_GEM_SET_DOMAIN	    VKI_DRM_IOW (VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_SET_DOMAIN, struct vki_drm_i915_gem_set_domain)
#define VKI_DRM_IOCTL_I915_GEM_SW_FINISH	    VKI_DRM_IOW (VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_SW_FINISH, struct vki_drm_i915_gem_sw_finish)
#define VKI_DRM_IOCTL_I915_GEM_SET_TILING	    VKI_DRM_IOWR (VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_SET_TILING, struct vki_drm_i915_gem_set_tiling)
#define VKI_DRM_IOCTL_I915_GEM_GET_TILING	    VKI_DRM_IOWR (VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_GET_TILING, struct vki_drm_i915_gem_get_tiling)
#define VKI_DRM_IOCTL_I915_GEM_GET_APERTURE	    VKI_DRM_IOR  (VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GEM_GET_APERTURE, struct vki_drm_i915_gem_get_aperture)
#define VKI_DRM_IOCTL_I915_GET_PIPE_FROM_CRTC_ID    VKI_DRM_IOWR(VKI_DRM_COMMAND_BASE + VKI_DRM_I915_GET_PIPE_FROM_CRTC_ID, struct vki_drm_intel_get_pipe_from_crtc_id)

typedef struct vki_drm_i915_batchbuffer {
	int start;		/* agp offset */
	int used;		/* nr bytes in use */
	int DR1;		/* hw flags for GFX_OP_DRAWRECT_INFO */
	int DR4;		/* window origin for GFX_OP_DRAWRECT_INFO */
	int num_cliprects;	/* mulitpass with multiple cliprects? */
	struct vki_drm_clip_rect __user *cliprects;	/* pointer to userspace cliprects */
} vki_drm_i915_batchbuffer_t;
typedef struct _vki_drm_i915_cmdbuffer {
	char __user *buf;	/* pointer to userspace command buffer */
	int sz;			/* nr bytes in buf */
	int DR1;		/* hw flags for GFX_OP_DRAWRECT_INFO */
	int DR4;		/* window origin for GFX_OP_DRAWRECT_INFO */
	int num_cliprects;	/* mulitpass with multiple cliprects? */
	struct vki_drm_clip_rect __user *cliprects;	/* pointer to userspace cliprects */
} vki_drm_i915_cmdbuffer_t;
typedef struct vki_drm_i915_irq_emit {
	int __user *irq_seq;
} vki_drm_i915_irq_emit_t;
typedef struct vki_drm_i915_irq_wait {
	int irq_seq;
} vki_drm_i915_irq_wait_t;
typedef struct vki_drm_i915_getparam {
	int param;
	int __user *value;
} vki_drm_i915_getparam_t;
typedef struct vki_drm_i915_setparam {
	int param;
	int value;
} vki_drm_i915_setparam_t;
typedef struct vki_drm_i915_mem_alloc {
	int region;
	int alignment;
	int size;
	int __user *region_offset;	/* offset from start of fb or agp */
} vki_drm_i915_mem_alloc_t;
typedef struct vki_drm_i915_mem_free {
	int region;
	int region_offset;
} vki_drm_i915_mem_free_t;
typedef struct vki_drm_i915_mem_init_heap {
	int region;
	int size;
	int start;
} vki_drm_i915_mem_init_heap_t;
typedef struct vki_drm_i915_mem_destroy_heap {
	int region;
} vki_drm_i915_mem_destroy_heap_t;
typedef struct vki_drm_i915_vblank_pipe {
	int pipe;
} vki_drm_i915_vblank_pipe_t;
typedef struct vki_drm_i915_vblank_swap {
	vki_drm_drawable_t drawable;
	enum vki_drm_vblank_seq_type seqtype;
	unsigned int sequence;
} vki_drm_i915_vblank_swap_t;
typedef struct vki_drm_i915_hws_addr {
	__vki_u64 addr;
} vki_drm_i915_hws_addr_t;
struct vki_drm_i915_gem_init {
	__vki_u64 gtt_start;
	__vki_u64 gtt_end;
};
struct vki_drm_i915_gem_create {
	__vki_u64 size;
	__vki_u32 handle;
	__vki_u32 pad;
};
struct vki_drm_i915_gem_pread {
	__vki_u32 handle;
	__vki_u32 pad;
	__vki_u64 offset;
	__vki_u64 size;
	__vki_u64 data_ptr;
};
struct vki_drm_i915_gem_pwrite {
	__vki_u32 handle;
	__vki_u32 pad;
	__vki_u64 offset;
	__vki_u64 size;
	__vki_u64 data_ptr;
};
struct vki_drm_i915_gem_mmap_v1 {
	__vki_u32 handle;
	__vki_u32 pad;
	__vki_u64 offset;
	__vki_u64 size;
	__vki_u64 addr_ptr;
};
struct vki_drm_i915_gem_mmap {
	__vki_u32 handle;
	__vki_u32 pad;
	__vki_u64 offset;
	__vki_u64 size;
	__vki_u64 addr_ptr;
	__vki_u64 flags;
};
struct vki_drm_i915_gem_mmap_gtt {
	__vki_u32 handle;
	__vki_u32 pad;
	__vki_u64 offset;
};
struct vki_drm_i915_gem_set_domain {
	__vki_u32 handle;
	__vki_u32 read_domains;
	__vki_u32 write_domain;
};
struct vki_drm_i915_gem_sw_finish {
	__vki_u32 handle;
};
struct vki_drm_i915_gem_relocation_entry {
	__vki_u32 target_handle;
	__vki_u32 delta;
	__vki_u64 offset;
	__vki_u64 presumed_offset;
	__vki_u32 read_domains;
	__vki_u32 write_domain;
};
struct vki_drm_i915_gem_exec_object {
	__vki_u32 handle;
	__vki_u32 relocation_count;
	__vki_u64 relocs_ptr;
	__vki_u64 alignment;
	__vki_u64 offset;
};
struct vki_drm_i915_gem_execbuffer {
	__vki_u64 buffers_ptr;
	__vki_u32 buffer_count;
	__vki_u32 batch_start_offset;
	__vki_u32 batch_len;
	__vki_u32 DR1;
	__vki_u32 DR4;
	__vki_u32 num_cliprects;
	__vki_u64 cliprects_ptr;
};
struct vki_drm_i915_gem_pin {
	__vki_u32 handle;
	__vki_u32 pad;
	__vki_u64 alignment;
	__vki_u64 offset;
};
struct vki_drm_i915_gem_unpin {
	__vki_u32 handle;
	__vki_u32 pad;
};
struct vki_drm_i915_gem_busy {
	__vki_u32 handle;
	__vki_u32 busy;
};
struct vki_drm_i915_gem_set_tiling {
	__vki_u32 handle;
	__vki_u32 tiling_mode;
	__vki_u32 stride;
	__vki_u32 swizzle_mode;
};
struct vki_drm_i915_gem_get_tiling {
	__vki_u32 handle;
	__vki_u32 tiling_mode;
	__vki_u32 swizzle_mode;
};
struct vki_drm_i915_gem_get_aperture {
	__vki_u64 aper_size;
	__vki_u64 aper_available_size;
};
struct vki_drm_i915_get_pipe_from_crtc_id {
	__vki_u32 crtc_id;
	__vki_u32 pipe;
};

#endif //__VKI_LINUX_DRM_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
