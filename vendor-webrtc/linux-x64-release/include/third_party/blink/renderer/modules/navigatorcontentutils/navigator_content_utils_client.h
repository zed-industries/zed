// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NAVIGATORCONTENTUTILS_NAVIGATOR_CONTENT_UTILS_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NAVIGATORCONTENTUTILS_NAVIGATOR_CONTENT_UTILS_CLIENT_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class KURL;
class LocalFrame;

class MODULES_EXPORT NavigatorContentUtilsClient
    : public GarbageCollected<NavigatorContentUtilsClient> {
 public:
  explicit NavigatorContentUtilsClient(LocalFrame*);
  virtual ~NavigatorContentUtilsClient() = default;

  virtual void RegisterProtocolHandler(const String& scheme, const KURL&);

  virtual void UnregisterProtocolHandler(const String& scheme, const KURL&);

  virtual void Trace(Visitor*) const;

 private:
  Member<LocalFrame> frame_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NAVIGATORCONTENTUTILS_NAVIGATOR_CONTENT_UTILS_CLIENT_H_
