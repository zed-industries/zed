// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_WEB_GRAPHICS_CONTEXT_3D_PROVIDER_WRAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_WEB_GRAPHICS_CONTEXT_3D_PROVIDER_WRAPPER_H_

#include <memory>
#include <utility>

#include "base/check_deref.h"
#include "base/memory/ptr_util.h"
#include "base/memory/weak_ptr.h"
#include "base/observer_list.h"
#include "third_party/blink/public/platform/web_graphics_context_3d_provider.h"
#include "third_party/blink/renderer/platform/graphics/gpu/graphics_context_3d_utils.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class PLATFORM_EXPORT WebGraphicsContext3DProviderWrapper {
  USING_FAST_MALLOC(WebGraphicsContext3DProviderWrapper);

 public:
  class DestructionObserver {
   public:
    virtual ~DestructionObserver() {}
    virtual void OnContextDestroyed() = 0;
  };

  // NOTE: `provider` must be non-null.
  WebGraphicsContext3DProviderWrapper(
      std::unique_ptr<WebGraphicsContext3DProvider> provider)
      : context_provider_(std::move(provider)) {
    CHECK(context_provider_);
    utils_ = base::WrapUnique(new GraphicsContext3DUtils(GetWeakPtr()));
  }
  ~WebGraphicsContext3DProviderWrapper();

  base::WeakPtr<WebGraphicsContext3DProviderWrapper> GetWeakPtr() {
    return weak_ptr_factory_.GetWeakPtr();
  }
  WebGraphicsContext3DProvider& ContextProvider() {
    return CHECK_DEREF(context_provider_.get());
  }

  GraphicsContext3DUtils* Utils() { return utils_.get(); }

  void AddObserver(DestructionObserver*);
  void RemoveObserver(DestructionObserver*);

 private:
  std::unique_ptr<GraphicsContext3DUtils> utils_;
  std::unique_ptr<WebGraphicsContext3DProvider> context_provider_;
  // RAW_PTR_EXCLUSION: Performance reasons(based on analysis of speedometer3).
  base::ObserverList<DestructionObserver>::UncheckedAndRawPtrExcluded
      observers_;
  base::WeakPtrFactory<WebGraphicsContext3DProviderWrapper> weak_ptr_factory_{
      this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_WEB_GRAPHICS_CONTEXT_3D_PROVIDER_WRAPPER_H_
