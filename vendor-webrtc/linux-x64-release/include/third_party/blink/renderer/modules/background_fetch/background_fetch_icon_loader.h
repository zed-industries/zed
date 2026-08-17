// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_BACKGROUND_FETCH_ICON_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_BACKGROUND_FETCH_ICON_LOADER_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_image_resource.h"
#include "third_party/blink/renderer/core/loader/threaded_icon_loader.h"
#include "third_party/blink/renderer/modules/background_fetch/background_fetch_type_converters.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

class BackgroundFetchBridge;

class MODULES_EXPORT BackgroundFetchIconLoader final
    : public GarbageCollected<BackgroundFetchIconLoader> {
 public:
  // The bitmap may be empty if the request failed or the image data could not
  // be decoded. The int64_t returned is the scale of the ideal to chosen icon,
  // before resizing. This is -1 if the ideal icon size is empty, or if no icon
  // provided was suitable.
  using IconCallback = base::OnceCallback<void(const SkBitmap&, int64_t)>;

  BackgroundFetchIconLoader();

  // Asynchronously download an icon from the given url, decodes the loaded
  // data, and passes the bitmap to the given callback.
  void Start(BackgroundFetchBridge* bridge,
             ExecutionContext* execution_context,
             HeapVector<Member<ManifestImageResource>> icons,
             IconCallback icon_callback);

  // Cancels the pending load, if there is one. The |icon_callback_| will not
  // be run.
  void Stop();

  void Trace(Visitor* visitor) const;

 private:
  friend class BackgroundFetchIconLoaderTest;

  // Callback for BackgroundFetchBridge::GetIconDisplaySize()
  void DidGetIconDisplaySizeIfSoLoadIcon(
      ExecutionContext* execution_context,
      IconCallback callback,
      const gfx::Size& icon_display_size_pixels);

  void DidGetIcon(SkBitmap icon, double resize_scale);

  // Picks the best icon from the list of developer provided icons, for current
  // display, given the |ideal_size_pixels|, and returns its KURL.
  // If none of the icons is appropriate, this returns an empty URL.
  KURL PickBestIconForDisplay(ExecutionContext* execution_context,
                              int ideal_size_pixels);

  HeapVector<Member<ManifestImageResource>> icons_;
  Member<ThreadedIconLoader> threaded_icon_loader_;
  IconCallback icon_callback_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BACKGROUND_FETCH_BACKGROUND_FETCH_ICON_LOADER_H_
