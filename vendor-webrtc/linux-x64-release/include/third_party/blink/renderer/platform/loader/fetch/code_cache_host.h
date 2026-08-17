// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_CODE_CACHE_HOST_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_CODE_CACHE_HOST_H_

#include "base/memory/weak_ptr.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/loader/code_cache.mojom-blink.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {

// Wrapper around mojo::Remote which can be shared among multiple
// CodeCacheLoaders that may outlive the frame lifetime, e.g. due to
// teardown ordering.
//
// Important: This class is not allowed to be on the Oilpan heap, since accesses
// to the mojo::Remote and the data it holds reliy on the object being valid
// (and not poisoned) until the destructor is called.
class BLINK_PLATFORM_EXPORT CodeCacheHost {
 public:
  explicit CodeCacheHost(mojo::Remote<mojom::blink::CodeCacheHost> remote)
      : remote_(std::move(remote)) {
    DCHECK(remote_.is_bound());
  }

  // Get a weak pointer to this CodeCacheHost. Only valid when the remote
  // has been bound.
  base::WeakPtr<CodeCacheHost> GetWeakPtr() {
    DCHECK(remote_.is_bound());
    return weak_factory_.GetWeakPtr();
  }

  mojom::blink::CodeCacheHost* get() { return remote_.get(); }
  mojom::blink::CodeCacheHost& operator*() { return *remote_.get(); }
  mojom::blink::CodeCacheHost* operator->() { return remote_.get(); }

 private:
  mojo::Remote<mojom::blink::CodeCacheHost> remote_;
  base::WeakPtrFactory<CodeCacheHost> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_CODE_CACHE_HOST_H_
