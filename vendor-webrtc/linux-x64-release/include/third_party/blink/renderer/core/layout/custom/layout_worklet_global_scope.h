// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_LAYOUT_WORKLET_GLOBAL_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_LAYOUT_WORKLET_GLOBAL_SCOPE_H_

#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/layout/custom/pending_layout_registry.h"
#include "third_party/blink/renderer/core/workers/worklet_global_scope.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CSSLayoutDefinition;
class V8NoArgumentConstructor;
class WorkerReportingProxy;

class CORE_EXPORT LayoutWorkletGlobalScope final : public WorkletGlobalScope {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static LayoutWorkletGlobalScope* Create(
      LocalFrame*,
      std::unique_ptr<GlobalScopeCreationParams>,
      WorkerReportingProxy&,
      PendingLayoutRegistry*);

  LayoutWorkletGlobalScope(LocalFrame*,
                           std::unique_ptr<GlobalScopeCreationParams>,
                           WorkerReportingProxy&,
                           PendingLayoutRegistry*);
  ~LayoutWorkletGlobalScope() override;
  void Dispose() final;

  bool IsLayoutWorkletGlobalScope() const final { return true; }

  // Implements LayoutWorkletGlobalScope.idl
  void registerLayout(const AtomicString& name,
                      V8NoArgumentConstructor* layout_ctor,
                      ExceptionState&);

  CSSLayoutDefinition* FindDefinition(const AtomicString& name);

  void Trace(Visitor*) const override;

  // Returns the token that uniquely identifies this worklet.
  const LayoutWorkletToken& GetLayoutWorkletToken() const { return token_; }
  WorkletToken GetWorkletToken() const final { return token_; }
  ExecutionContextToken GetExecutionContextToken() const final {
    return token_;
  }

 private:
  // https://drafts.css-houdini.org/css-layout-api/#layout-definitions
  typedef HeapHashMap<String, Member<CSSLayoutDefinition>> DefinitionMap;

  // TODO(crbug.com/1286244): Return a proper destination for LayoutWorklet.
  network::mojom::RequestDestination GetDestination() const override {
    return network::mojom::RequestDestination::kScript;
  }

  DefinitionMap layout_definitions_;
  Member<PendingLayoutRegistry> pending_layout_registry_;

  // Default initialized to generate a distinct token for this worklet.
  const LayoutWorkletToken token_;
};

template <>
struct DowncastTraits<LayoutWorkletGlobalScope> {
  static bool AllowFrom(const ExecutionContext& context) {
    return context.IsLayoutWorkletGlobalScope();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_LAYOUT_WORKLET_GLOBAL_SCOPE_H_
