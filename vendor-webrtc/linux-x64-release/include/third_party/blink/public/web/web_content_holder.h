// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_CONTENT_HOLDER_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_CONTENT_HOLDER_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"
#include "third_party/blink/public/platform/web_string.h"

namespace gfx {
class Rect;
}  // namespace gfx

namespace blink {

class ContentHolder;

// The class to represent the captured content.
class BLINK_EXPORT WebContentHolder {
 public:
  WebContentHolder(const WebContentHolder&);
  WebContentHolder& operator=(const WebContentHolder&);
  virtual ~WebContentHolder();

  WebString GetValue() const;
  gfx::Rect GetBoundingBox() const;
  uint64_t GetId() const;

#if INSIDE_BLINK
  explicit WebContentHolder(ContentHolder& node_info);
#endif

 private:
  WebPrivatePtrForGC<ContentHolder> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_CONTENT_HOLDER_H_
