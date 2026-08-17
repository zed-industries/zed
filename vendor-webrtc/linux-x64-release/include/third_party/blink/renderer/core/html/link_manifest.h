// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_LINK_MANIFEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_LINK_MANIFEST_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/html/link_resource.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class HTMLLinkElement;

class LinkManifest final : public LinkResource {
 public:
  explicit LinkManifest(HTMLLinkElement* owner);
  ~LinkManifest() override;

  // LinkResource
  void Process(LinkLoadParameters::Reason reason) override;
  LinkResourceType GetType() const override { return kManifest; }
  bool HasLoaded() const override;
  void OwnerRemoved() override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_LINK_MANIFEST_H_
