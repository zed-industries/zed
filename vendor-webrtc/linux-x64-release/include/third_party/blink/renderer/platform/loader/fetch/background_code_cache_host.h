// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_BACKGROUND_CODE_CACHE_HOST_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_BACKGROUND_CODE_CACHE_HOST_H_

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/mojom/loader/code_cache.mojom-blink.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace base {
class SequencedTaskRunner;
}  // namespace base

namespace blink {

class CodeCacheHost;

// BackgroundCodeCacheHost is used for loading code cache from CodeCacheHost on
// a background thread. Used only when BackgroundResourceFetch feature is
// enabled.
class BLINK_PLATFORM_EXPORT BackgroundCodeCacheHost
    : public ThreadSafeRefCounted<BackgroundCodeCacheHost> {
 public:
  explicit BackgroundCodeCacheHost(
      mojo::PendingRemote<mojom::blink::CodeCacheHost> pending_remote);

  BackgroundCodeCacheHost(const BackgroundCodeCacheHost&) = delete;
  BackgroundCodeCacheHost& operator=(const BackgroundCodeCacheHost&) = delete;

  // This method must be called on the `background_task_runner`'s sequence.
  CodeCacheHost& GetCodeCacheHost(
      scoped_refptr<base::SequencedTaskRunner> background_task_runner);

 private:
  friend class ThreadSafeRefCounted<BackgroundCodeCacheHost>;
  ~BackgroundCodeCacheHost();

  mojo::PendingRemote<mojom::blink::CodeCacheHost> pending_remote_;
  std::unique_ptr<CodeCacheHost> code_cache_host_;
  scoped_refptr<base::SequencedTaskRunner> background_task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_BACKGROUND_CODE_CACHE_HOST_H_
