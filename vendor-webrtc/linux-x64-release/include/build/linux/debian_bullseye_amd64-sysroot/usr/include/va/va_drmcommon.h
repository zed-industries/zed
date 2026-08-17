/*
 * va_drmcommon.h - Common utilities for DRM-based drivers
 *
 * Copyright (c) 2012 Intel Corporation. All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT.
 * IN NO EVENT SHALL INTEL AND/OR ITS SUPPLIERS BE LIABLE FOR
 * ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#ifndef VA_DRM_COMMON_H
#define VA_DRM_COMMON_H

#include <stdint.h>


/** \brief DRM authentication type. */
enum {
    /** \brief Disconnected. */
    VA_DRM_AUTH_NONE    = 0,
    /**
     * \brief Connected. Authenticated with DRI1 protocol.
     *
     * @deprecated
     * This is a deprecated authentication type. All DRI-based drivers have
     * been migrated to use the DRI2 protocol. Newly written drivers shall
     * use DRI2 protocol only, or a custom authentication means. e.g. opt
     * for authenticating on the VA driver side, instead of libva side.
     */
    VA_DRM_AUTH_DRI1    = 1,
    /**
     * \brief Connected. Authenticated with DRI2 protocol.
     *
     * This is only useful to VA/X11 drivers. The libva-x11 library provides
     * a helper function VA_DRI2Authenticate() for authenticating the
     * connection. However, DRI2 conformant drivers don't need to call that
     * function since authentication happens on the libva side, implicitly.
     */
    VA_DRM_AUTH_DRI2    = 2,
    /**
     * \brief Connected. Authenticated with some alternate raw protocol.
     *
     * This authentication mode is mainly used in non-VA/X11 drivers.
     * Authentication happens through some alternative method, at the
     * discretion of the VA driver implementation.
     */
    VA_DRM_AUTH_CUSTOM  = 3
};

/** \brief Base DRM state. */
struct drm_state {
    /** \brief DRM connection descriptor. */
    int         fd;
    /** \brief DRM authentication type. */
    int         auth_type;
    /** \brief Reserved bytes for future use, must be zero */
    int         va_reserved[8];
};

/** \brief Kernel DRM buffer memory type.  */
#define VA_SURFACE_ATTRIB_MEM_TYPE_KERNEL_DRM       0x10000000
/** \brief DRM PRIME memory type (old version)
 *
 * This supports only single objects with restricted memory layout.
 * Used with VASurfaceAttribExternalBuffers.
 */
#define VA_SURFACE_ATTRIB_MEM_TYPE_DRM_PRIME        0x20000000
/** \brief DRM PRIME memory type
 *
 * Used with VADRMPRIMESurfaceDescriptor.
 */
#define VA_SURFACE_ATTRIB_MEM_TYPE_DRM_PRIME_2      0x40000000

/**
 * \brief External buffer descriptor for a DRM PRIME surface.
 *
 * For export, call vaExportSurfaceHandle() with mem_type set to
 * VA_SURFACE_ATTRIB_MEM_TYPE_DRM_PRIME_2 and pass a pointer to an
 * instance of this structure to fill.
 * If VA_EXPORT_SURFACE_SEPARATE_LAYERS is specified on export, each
 * layer will contain exactly one plane.  For example, an NV12
 * surface will be exported as two layers, one of DRM_FORMAT_R8 and
 * one of DRM_FORMAT_GR88.
 * If VA_EXPORT_SURFACE_COMPOSED_LAYERS is specified on export,
 * there will be exactly one layer.
 *
 * For import, call vaCreateSurfaces() with the MemoryType attribute
 * set to VA_SURFACE_ATTRIB_MEM_TYPE_DRM_PRIME_2 and the
 * ExternalBufferDescriptor attribute set to point to an array of
 * num_surfaces instances of this structure.
 * The number of planes which need to be provided for a given layer
 * is dependent on both the format and the format modifier used for
 * the objects containing it.  For example, the format DRM_FORMAT_RGBA
 * normally requires one plane, but with the format modifier
 * I915_FORMAT_MOD_Y_TILED_CCS it requires two planes - the first
 * being the main data plane and the second containing the color
 * control surface.
 * Note that a given driver may only support a subset of possible
 * representations of a particular format.  For example, it may only
 * support NV12 surfaces when they are contained within a single DRM
 * object, and therefore fail to create such surfaces if the two
 * planes are in different DRM objects.
 * Note that backend driver will retrieve the resource represent by fd,
 * and a valid surface ID is generated. Backend driver will not close
 * the file descriptor. Application should handle the release of the fd.
 * releasing the fd will not impact the existence of the surface.
 */
typedef struct _VADRMPRIMESurfaceDescriptor {
    /** Pixel format fourcc of the whole surface (VA_FOURCC_*). */
    uint32_t fourcc;
    /** Width of the surface in pixels. */
    uint32_t width;
    /** Height of the surface in pixels. */
    uint32_t height;
    /** Number of distinct DRM objects making up the surface. */
    uint32_t num_objects;
    /** Description of each object. */
    struct {
        /** DRM PRIME file descriptor for this object. */
        int fd;
        /** Total size of this object (may include regions which are
         *  not part of the surface). */
        uint32_t size;
        /** Format modifier applied to this object. */
        uint64_t drm_format_modifier;
    } objects[4];
    /** Number of layers making up the surface. */
    uint32_t num_layers;
    /** Description of each layer in the surface. */
    struct {
        /** DRM format fourcc of this layer (DRM_FOURCC_*). */
        uint32_t drm_format;
        /** Number of planes in this layer. */
        uint32_t num_planes;
        /** Index in the objects array of the object containing each
         *  plane. */
        uint32_t object_index[4];
        /** Offset within the object of each plane. */
        uint32_t offset[4];
        /** Pitch of each plane. */
        uint32_t pitch[4];
    } layers[4];
} VADRMPRIMESurfaceDescriptor;

/**
 * \brief List of DRM format modifiers.
 *
 * To allocate surfaces with one of the modifiers specified in the array, call
 * vaCreateSurfaces() with the VASurfaceAttribDRMFormatModifiers attribute set
 * to point to an array of num_surfaces instances of this structure. The driver
 * will select the optimal modifier in the list.
 *
 * DRM format modifiers are defined in drm_fourcc.h in the Linux kernel.
 */
typedef struct _VADRMFormatModifierList {
    /** Number of modifiers. */
    uint32_t num_modifiers;
    /** Array of modifiers. */
    uint64_t *modifiers;
} VADRMFormatModifierList;


#endif /* VA_DRM_COMMON_H */
