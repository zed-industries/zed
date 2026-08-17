// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_INDEX_CONTENT_INDEX_ICON_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_INDEX_CONTENT_INDEX_ICON_LOADER_H_

#include <memory>

#include "third_party/blink/public/mojom/content_index/content_index.mojom-blink.h"
#include "third_party/blink/renderer/core/loader/threaded_icon_loader.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExecutionContext;

class MODULES_EXPORT ContentIndexIconLoader final
    : public GarbageCollected<ContentIndexIconLoader> {
 public:
  using IconsCallback =
      base::OnceCallback<void(mojom::blink::ContentDescriptionPtr description,
                              Vector<SkBitmap> icons)>;

  ContentIndexIconLoader();

  void Start(ExecutionContext* execution_context,
             mojom::blink::ContentDescriptionPtr description,
             const Vector<gfx::Size>& icon_sizes,
             IconsCallback callback);

  void Trace(Visitor* visitor) const {}

 private:
  void DidGetIcons(mojom::blink::ContentDescriptionPtr description,
                   std::unique_ptr<Vector<SkBitmap>> icons,
                   IconsCallback callback);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_INDEX_CONTENT_INDEX_ICON_LOADER_H_
