// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_RESOURCES_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_RESOURCES_LOADER_H_

#include "third_party/blink/public/mojom/notifications/notification.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/loader/threaded_icon_loader.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/skia/include/core/SkBitmap.h"

namespace blink {

class ExecutionContext;

// Fetches the resources specified in a given NotificationData. Uses a
// callback to notify the caller when all fetches have finished.
class MODULES_EXPORT NotificationResourcesLoader final
    : public GarbageCollected<NotificationResourcesLoader> {
  USING_PRE_FINALIZER(NotificationResourcesLoader, Stop);

 public:
  // Called when all fetches have finished. Passes a pointer to the
  // NotificationResourcesLoader so callers that use multiple loaders can use
  // the same function to handle the callbacks.
  using CompletionCallback =
      base::OnceCallback<void(NotificationResourcesLoader*)>;

  explicit NotificationResourcesLoader(CompletionCallback completion_callback);
  ~NotificationResourcesLoader();

  // Starts fetching the resources specified in the given NotificationData.
  // If all the urls for the resources are empty or invalid,
  // |m_completionCallback| will be run synchronously, otherwise it will be
  // run asynchronously when all fetches have finished. Should not be called
  // more than once.
  void Start(ExecutionContext* context,
             const mojom::blink::NotificationData& notification_data);

  // Returns a new NotificationResourcesPtr populated with the resources that
  // have been fetched.
  mojom::blink::NotificationResourcesPtr GetResources() const;

  // Stops every loader in |m_imageLoaders|. This is also used as the
  // pre-finalizer.
  void Stop();

  void Trace(Visitor* visitor) const;

 private:
  void LoadIcon(ExecutionContext* context,
                const KURL& url,
                const gfx::Size& resize_dimensions,
                ThreadedIconLoader::IconCallback icon_callback);

  void DidLoadIcon(SkBitmap* out_icon, SkBitmap icon, double resize_scale);

  // Decrements |pending_request_count_| and runs |completion_callback_| if
  // there are no more pending requests.
  void DidFinishRequest();

  bool started_;
  CompletionCallback completion_callback_;
  int pending_request_count_;
  HeapVector<Member<ThreadedIconLoader>> icon_loaders_;
  SkBitmap image_;
  SkBitmap icon_;
  SkBitmap badge_;
  Vector<SkBitmap> action_icons_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_RESOURCES_LOADER_H_
