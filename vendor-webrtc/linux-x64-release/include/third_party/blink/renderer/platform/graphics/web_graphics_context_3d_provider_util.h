// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_WEB_GRAPHICS_CONTEXT_3D_PROVIDER_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_WEB_GRAPHICS_CONTEXT_3D_PROVIDER_UTIL_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/platform/platform.h"
#include "third_party/blink/public/platform/web_graphics_context_3d_provider.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

// Synchronously creates a WebGraphicsContext3DProvider on any thread.
// Note if this method is not called on the main thread it will block waiting
// for the main thread to allocated an offscreen context provider.
//
// Returns a newly allocated and initialized offscreen context provider,
// backed by an independent context. Returns null if the context cannot be
// created or initialized.
//
// Upon successful completion, |gl_info| and |using_gpu_compositing| will be
// filled in with their actual values.
//
// A blocking task is posted to the main thread to create the context, so do
// not call this method from code which may block main thread progress.
PLATFORM_EXPORT std::unique_ptr<WebGraphicsContext3DProvider>
CreateOffscreenGraphicsContext3DProvider(
    Platform::ContextAttributes context_attributes,
    Platform::GraphicsInfo* gl_info,
    const KURL& url);

// Synchronously creates a WebGPUGraphicsContext3DProvider on any thread.
// Note if this method is not called on the main thread it will block waiting
// for the main thread to allocated an offscreen context provider.
PLATFORM_EXPORT std::unique_ptr<WebGraphicsContext3DProvider>
CreateWebGPUGraphicsContext3DProvider(const KURL& url);

// Asynchronously creates a WebGPUGraphicsContext3DProvider on any thread.
PLATFORM_EXPORT void CreateWebGPUGraphicsContext3DProviderAsync(
    const KURL& url,
    scoped_refptr<base::SingleThreadTaskRunner> current_thread_task_runner,
    WTF::CrossThreadOnceFunction<
        void(std::unique_ptr<WebGraphicsContext3DProvider>)> callback);

// If the shared GPU context exists, sets whether it aggressively frees
// resources to `value`.
PLATFORM_EXPORT void SetAggressivelyFreeSharedGpuContextResourcesIfPossible(
    bool value);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_WEB_GRAPHICS_CONTEXT_3D_PROVIDER_UTIL_H_
