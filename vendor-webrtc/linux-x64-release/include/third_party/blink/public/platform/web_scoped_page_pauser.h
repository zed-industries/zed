// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_SCOPED_PAGE_PAUSER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_SCOPED_PAGE_PAUSER_H_

#include <memory>

#include "third_party/blink/public/platform/web_common.h"

namespace blink {

class ScopedBrowsingContextGroupPauser;
class ScopedPagePauser;
class WebLocalFrameImpl;

// WebScopedPagePauser implements the concept of 'pause' in HTML standard.
// https://html.spec.whatwg.org/C/#pause
// All script execution is suspended while any of WebScopedPagePauser instances
// exists.
class WebScopedPagePauser {
 public:
  explicit WebScopedPagePauser(WebLocalFrameImpl&);

  WebScopedPagePauser(const WebScopedPagePauser&) = delete;
  WebScopedPagePauser& operator=(const WebScopedPagePauser&) = delete;
  BLINK_EXPORT ~WebScopedPagePauser();

 private:
  std::unique_ptr<ScopedPagePauser> page_pauser_;
  std::unique_ptr<ScopedBrowsingContextGroupPauser>
      browsing_context_group_pauser_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_SCOPED_PAGE_PAUSER_H_
