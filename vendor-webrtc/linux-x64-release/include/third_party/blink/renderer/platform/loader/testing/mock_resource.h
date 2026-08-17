// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_MOCK_RESOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_MOCK_RESOURCE_H_

#include "third_party/blink/renderer/platform/loader/fetch/resource.h"

namespace blink {

class FetchParameters;
class ResourceFetcher;
struct ResourceLoaderOptions;

// Mocked Resource sub-class for testing. MockResource class can pretend a type
// of Resource sub-class in a simple way. You should not expect anything
// complicated to emulate actual sub-resources, but you may be able to use this
// class to verify classes that consume Resource sub-classes in a simple way.
class MockResource final : public Resource {
 public:
  static MockResource* Fetch(FetchParameters&,
                             ResourceFetcher*,
                             ResourceClient*);
  explicit MockResource(const KURL&);
  explicit MockResource(const ResourceRequest&);
  MockResource(const ResourceRequest&, const ResourceLoaderOptions&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_MOCK_RESOURCE_H_
