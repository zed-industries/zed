// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_MANIFEST_MANAGER_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_MANIFEST_MANAGER_H_

#include "base/functional/callback.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {

class WebLocalFrame;
class WebURL;

class BLINK_EXPORT WebManifestManager {
 public:
  using Callback = base::OnceCallback<void(const WebURL&)>;

  static void RequestManifestForTesting(WebLocalFrame*, Callback callback);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_MANIFEST_MANAGER_H_
