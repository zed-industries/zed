/*
 * Direct3D 12 HW acceleration.
 *
 * copyright (c) 2022-2023 Wu Jianhua <toqsxw@outlook.com>
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

#ifndef AVUTIL_HWCONTEXT_D3D12VA_H
#define AVUTIL_HWCONTEXT_D3D12VA_H

/**
 * @file
 * An API-specific header for AV_HWDEVICE_TYPE_D3D12VA.
 *
 * AVHWFramesContext.pool must contain AVBufferRefs whose
 * data pointer points to an AVD3D12VAFrame struct.
 */
#include <stdint.h>
#include <initguid.h>
#include <d3d12.h>
#include <d3d12sdklayers.h>
#include <d3d12video.h>

/**
 * @brief This struct is allocated as AVHWDeviceContext.hwctx
 *
 */
typedef struct AVD3D12VADeviceContext {
    /**
     * Device used for objects creation and access. This can also be
     * used to set the libavcodec decoding device.
     *
     * Can be set by the user. This is the only mandatory field - the other
     * device context fields are set from this and are available for convenience.
     *
     * Deallocating the AVHWDeviceContext will always release this interface,
     * and it does not matter whether it was user-allocated.
     */
    ID3D12Device *device;

    /**
     * If unset, this will be set from the device field on init.
     *
     * Deallocating the AVHWDeviceContext will always release this interface,
     * and it does not matter whether it was user-allocated.
     */
    ID3D12VideoDevice *video_device;

    /**
     * Callbacks for locking. They protect access to the internal staging
     * texture (for av_hwframe_transfer_data() calls). They do NOT protect
     * access to hwcontext or decoder state in general.
     *
     * If unset on init, the hwcontext implementation will set them to use an
     * internal mutex.
     *
     * The underlying lock must be recursive. lock_ctx is for free use by the
     * locking implementation.
     */
    void (*lock)(void *lock_ctx);
    void (*unlock)(void *lock_ctx);
    void *lock_ctx;
} AVD3D12VADeviceContext;

/**
 * @brief This struct is used to sync d3d12 execution
 *
 */
typedef struct AVD3D12VASyncContext {
    /**
     * D3D12 fence object
     */
    ID3D12Fence *fence;

    /**
     * A handle to the event object that's raised when the fence
     * reaches a certain value.
     */
    HANDLE event;

    /**
     * The fence value used for sync
     */
    uint64_t fence_value;
} AVD3D12VASyncContext;

/**
 * @brief D3D12VA frame descriptor for pool allocation.
 *
 */
typedef struct AVD3D12VAFrame {
    /**
     * The texture in which the frame is located. The reference count is
     * managed by the AVBufferRef, and destroying the reference will release
     * the interface.
     */
    ID3D12Resource *texture;

    /**
     * The sync context for the texture
     *
     * @see: https://learn.microsoft.com/en-us/windows/win32/medfound/direct3d-12-video-overview#directx-12-fences
     */
    AVD3D12VASyncContext sync_ctx;
} AVD3D12VAFrame;

/**
 * @brief This struct is allocated as AVHWFramesContext.hwctx
 *
 */
typedef struct AVD3D12VAFramesContext {
    /**
     * DXGI_FORMAT format. MUST be compatible with the pixel format.
     * If unset, will be automatically set.
     */
    DXGI_FORMAT format;

    /**
     * Options for working with resources.
     * If unset, this will be D3D12_RESOURCE_FLAG_NONE.
     *
     * @see https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_resource_flags
     */
    D3D12_RESOURCE_FLAGS flags;
} AVD3D12VAFramesContext;

#endif /* AVUTIL_HWCONTEXT_D3D12VA_H */
