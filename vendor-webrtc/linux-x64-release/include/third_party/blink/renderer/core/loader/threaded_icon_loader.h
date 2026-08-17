// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_THREADED_ICON_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_THREADED_ICON_LOADER_H_

#include <optional>

#include "base/memory/scoped_refptr.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/threadable_loader.h"
#include "third_party/blink/renderer/core/loader/threadable_loader_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/skia/include/core/SkBitmap.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

class ResourceRequestHead;

// Utility class for loading, decoding, and potentially rescaling an icon on a
// background thread. Note that icons are only downscaled and never upscaled.
// Warning! If the response image type is "image/svg+xml", the process will
// happen on the main thread.
class CORE_EXPORT ThreadedIconLoader final
    : public GarbageCollected<ThreadedIconLoader>,
      public ThreadableLoaderClient {
 public:
  // On failure, |callback| is called with a null SkBitmap and |resize_scale|
  // set to -1. On success, the icon is provided with a |resize_scale| <= 1.
  using IconCallback =
      base::OnceCallback<void(SkBitmap icon, double resize_scale)>;

  // Starts a background task to download and decode the icon.
  // If |resize_dimensions| is provided, the icon will will be downscaled to
  // those dimensions.
  void Start(ExecutionContext* execution_context,
             const ResourceRequestHead& resource_request,
             const std::optional<gfx::Size>& resize_dimensions,
             IconCallback callback);

  // Stops the background task. The provided callback will not be run if
  // `Stop` is called.
  void Stop();

  // ThreadableLoaderClient interface.
  void DidReceiveResponse(uint64_t, const ResourceResponse& response) override;
  void DidReceiveData(base::span<const char> data) override;
  void DidFinishLoading(uint64_t resource_identifier) override;
  void DidFail(uint64_t, const ResourceError& error) override;
  void DidFailRedirectCheck(uint64_t) override;

  void Trace(Visitor* visitor) const override;

 private:
  void OnBackgroundTaskComplete(SkBitmap icon, double resize_scale);

  Member<ThreadableLoader> threadable_loader_;

  // Data received from |threadable_loader_|. Will be invalidated when decoding
  // of the image data starts.
  SegmentedBuffer data_;

  String response_mime_type_;

  std::optional<gfx::Size> resize_dimensions_;

  IconCallback icon_callback_;

  bool stopped_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_THREADED_ICON_LOADER_H_
