// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_SPECULATION_RULE_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_SPECULATION_RULE_LOADER_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_finish_observer.h"

namespace blink {

class Document;
class SpeculationRulesResource;

// This is used for Speculation-Rules header
class CORE_EXPORT SpeculationRuleLoader final : public ResourceFinishObserver,
                                                public NameClient {
 public:
  explicit SpeculationRuleLoader(Document& document);
  ~SpeculationRuleLoader() override;

  void Trace(Visitor*) const override;
  const char* NameInHeapSnapshot() const override {
    return "SpeculationRuleLoader";
  }
  String DebugName() const override { return "SpeculationRuleLoader"; }

  void LoadResource(SpeculationRulesResource*);

 private:
  void NotifyFinished() override;

  Member<Document> document_;
  Member<SpeculationRulesResource> resource_;
  base::TimeTicks start_time_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_SPECULATION_RULE_LOADER_H_
