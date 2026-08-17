// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_MEDIA_SOURCE_REGISTRY_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_MEDIA_SOURCE_REGISTRY_IMPL_H_

#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "third_party/blink/renderer/core/html/media/media_source_attachment.h"
#include "third_party/blink/renderer/core/html/media/media_source_registry.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"

namespace blink {

class KURL;

// This singleton lives on the main thread. It allows registration and
// deregistration of MediaSource objectUrls from the main thread.
class MediaSourceRegistryImpl final : public MediaSourceRegistry {
 public:
  // Creates the singleton instance.
  static void Init();

  // MediaSourceRegistry : URLRegistry overrides for (un)registering blob URLs
  // referring to the specified media source attachment. RegisterURL creates a
  // scoped_refptr to manage the registrable's ref-counted lifetime and puts it
  // in |media_sources_|.
  void RegisterURL(const KURL&, URLRegistrable*) override;

  // UnregisterURL removes the corresponding scoped_refptr and KURL from
  // |media_sources_| if its KURL was there.
  void UnregisterURL(const KURL&) override;

  // MediaSourceRegistry override that finds |url| in |media_sources_| and
  // returns the corresponding scoped_refptr if found. Otherwise, returns an
  // unset scoped_refptr. |url| must be non-empty.
  scoped_refptr<MediaSourceAttachment> LookupMediaSource(
      const String& url) override;

 private:
  // Construction of this singleton informs MediaSourceAttachment of this
  // singleton, for it to use to service URLRegistry interface activities on
  // this registry like lookup, registration and unregistration.
  MediaSourceRegistryImpl();

  HashMap<String, scoped_refptr<MediaSourceAttachment>> media_sources_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_MEDIA_SOURCE_REGISTRY_IMPL_H_
