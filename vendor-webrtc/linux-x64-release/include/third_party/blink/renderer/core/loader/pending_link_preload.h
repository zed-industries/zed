// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_PENDING_LINK_PRELOAD_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_PENDING_LINK_PRELOAD_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/core/loader/link_load_parameters.h"
#include "third_party/blink/renderer/core/script/modulator.h"

namespace blink {

class Document;
class LinkLoader;
class Resource;

// Represents a pending preload, prefetch or modulepreload link. Receives
// callbacks when the loading finishes or errors.
class PendingLinkPreload final : public SingleModuleClient {
 public:
  PendingLinkPreload(Document& document, LinkLoader* loader);
  ~PendingLinkPreload() override;

  void UnblockRendering();
  void Dispose();

  void AddResource(Resource*);

  bool HasResource() const { return finish_observer_.Get(); }
  bool MatchesMedia() const { return matches_media_; }
  void SetMatchesMedia(bool matches) { matches_media_ = matches; }
  Resource* GetResourceForTesting() const;

  void Trace(Visitor*) const override;

 private:
  class FinishObserver;

  // SingleModuleClient implementation
  void NotifyModuleLoadFinished(ModuleScript*) override;

  void NotifyFinished();

  scoped_refptr<base::SingleThreadTaskRunner> GetLoadingTaskRunner();

  Member<Document> document_;
  Member<LinkLoader> loader_;
  Member<FinishObserver> finish_observer_;
  bool matches_media_{false};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_PENDING_LINK_PRELOAD_H_
