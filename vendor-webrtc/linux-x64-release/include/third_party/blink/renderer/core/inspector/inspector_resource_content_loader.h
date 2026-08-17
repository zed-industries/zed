// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_RESOURCE_CONTENT_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_RESOURCE_CONTENT_LOADER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class KURL;
class LocalFrame;
class Resource;

class CORE_EXPORT InspectorResourceContentLoader final
    : public GarbageCollected<InspectorResourceContentLoader> {
 public:
  explicit InspectorResourceContentLoader(LocalFrame*);
  InspectorResourceContentLoader(const InspectorResourceContentLoader&) =
      delete;
  InspectorResourceContentLoader& operator=(
      const InspectorResourceContentLoader&) = delete;
  ~InspectorResourceContentLoader();
  void Dispose();
  void Trace(Visitor*) const;

  int CreateClientId();
  void EnsureResourcesContentLoaded(int client_id, base::OnceClosure callback);
  void Cancel(int client_id);
  void DidCommitLoadForLocalFrame(LocalFrame*);

  Resource* ResourceForURL(const KURL&);

 private:
  class ResourceClient;

  void ResourceFinished(ResourceClient*);
  void CheckDone();
  void Start();
  void Stop();
  bool HasFinished();

  using Callbacks = Vector<base::OnceClosure>;
  HashMap<int, Callbacks> callbacks_;
  bool all_requests_started_;
  bool started_;
  Member<LocalFrame> inspected_frame_;
  HeapHashSet<Member<ResourceClient>> pending_resource_clients_;
  HeapVector<Member<Resource>> resources_;
  int last_client_id_;

  friend class ResourceClient;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_RESOURCE_CONTENT_LOADER_H_
