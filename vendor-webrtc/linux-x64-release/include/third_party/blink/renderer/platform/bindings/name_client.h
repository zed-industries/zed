// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_NAME_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_NAME_CLIENT_H_

#include "third_party/blink/renderer/platform/bindings/buildflags.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "v8/include/cppgc/name-provider.h"

namespace blink {

// NameClient provides classes with a human-readable name that can be used for
// inspecting the object graph.
//
// NameClient is aimed to provide web developers better idea about what object
// instances (or the object reference subgraph held by the instances) is
// consuming heap. It should provide actionable guidance for reducing memory for
// a given website.
//
// NameClient should be inherited for classes which:
// - is likely to be in a reference chain that is likely to hold a consierable
//   amount of memory,
// - Web developers would have a rough idea what it would mean, and
//   (The name is exposed to DevTools)
// - not ScriptWrappable (ScriptWrappable implements NameClient).
//
// Caveat:
//   NameClient should be inherited near the root of the inheritance graph
//   for Member<BaseClass> edges to be attributed correctly.
//
//   Do:
//     class Foo : public GarbageCollected<Foo>, public NameClient {...};
//
//   Don't:
//     class Bar : public GarbageCollected<Bar> {...};
//     class Baz : public Bar, public NameClient {...};
class PLATFORM_EXPORT NameClient : public cppgc::NameProvider {
 public:
  NameClient() = default;
  NameClient(const NameClient&) = delete;
  NameClient& operator=(const NameClient&) = delete;
  ~NameClient() override = default;

  // Human-readable name of this object. The DevTools heap snapshot uses
  // this method to show the object.
  virtual const char* NameInHeapSnapshot() const = 0;

  const char* GetHumanReadableName() const final {
    return NameInHeapSnapshot();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_NAME_CLIENT_H_
