// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_BACKGROUND_RESOURCE_FETCH_ASSETS_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_BACKGROUND_RESOURCE_FETCH_ASSETS_H_

#include "base/memory/ref_counted.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/platform/url_loader_throttle_provider.h"
#include "third_party/blink/public/platform/web_common.h"

namespace base {
class SequencedTaskRunner;
}  // namespace base

namespace network {
class SharedURLLoaderFactory;
}  // namespace network

namespace blink {

// WebBackgroundResourceFetchAssets is created on the main thread and used to
// get the SharedURLLoaderFactory from the background thread. Used only when
// BackgroundResourceFetch feature is enabled.
class BLINK_PLATFORM_EXPORT WebBackgroundResourceFetchAssets
    : public base::RefCountedThreadSafe<WebBackgroundResourceFetchAssets> {
 public:
  // Returns the task runner used to fetch resources on the background thread.
  virtual const scoped_refptr<base::SequencedTaskRunner>& GetTaskRunner() = 0;

  // Returns a SharedURLLoaderFactory for resource fetching. Must be called on
  // the background thread.
  virtual scoped_refptr<network::SharedURLLoaderFactory> GetLoaderFactory() = 0;

  virtual URLLoaderThrottleProvider* GetThrottleProvider() = 0;

  virtual const blink::LocalFrameToken& GetLocalFrameToken() = 0;

 protected:
  friend class base::RefCountedThreadSafe<WebBackgroundResourceFetchAssets>;
  virtual ~WebBackgroundResourceFetchAssets() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_BACKGROUND_RESOURCE_FETCH_ASSETS_H_
