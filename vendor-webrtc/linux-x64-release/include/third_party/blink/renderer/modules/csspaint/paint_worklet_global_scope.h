// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_WORKLET_GLOBAL_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_WORKLET_GLOBAL_SCOPE_H_

#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/workers/worklet_global_scope.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CSSPaintDefinition;
class ExceptionState;
class ScriptState;
class V8NoArgumentConstructor;
class WorkerReportingProxy;

class MODULES_EXPORT PaintWorkletGlobalScope final : public WorkletGlobalScope {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Creates a main-thread bound PaintWorkletGlobalScope.
  static PaintWorkletGlobalScope* Create(
      LocalFrame*,
      std::unique_ptr<GlobalScopeCreationParams>,
      WorkerReportingProxy&);

  // Creates an worklet-thread bound PaintWorkletGlobalScope.
  static PaintWorkletGlobalScope* Create(
      std::unique_ptr<GlobalScopeCreationParams>,
      WorkerThread*);

  PaintWorkletGlobalScope(LocalFrame*,
                          std::unique_ptr<GlobalScopeCreationParams>,
                          WorkerReportingProxy&);
  PaintWorkletGlobalScope(std::unique_ptr<GlobalScopeCreationParams>,
                          WorkerThread*);
  ~PaintWorkletGlobalScope() override;
  void Dispose() final;

  bool IsPaintWorkletGlobalScope() const final { return true; }
  void registerPaint(const ScriptState* script_state,
                     const String& name,
                     V8NoArgumentConstructor* paint_ctor,
                     ExceptionState&);

  CSSPaintDefinition* FindDefinition(const String& name);
  double devicePixelRatio() const;

  void Trace(Visitor*) const override;

  // Returns the token that uniquely identifies this worklet.
  const PaintWorkletToken& GetPaintWorkletToken() const { return token_; }
  WorkletToken GetWorkletToken() const final { return token_; }
  ExecutionContextToken GetExecutionContextToken() const final {
    return token_;
  }

 private:
  network::mojom::RequestDestination GetDestination() const override {
    return network::mojom::RequestDestination::kPaintWorklet;
  }

  // Registers the global scope with a proxy client, if not already done. Only
  // used for worklet-thread bound PaintWorkletGlobalScopes.
  void RegisterWithProxyClientIfNeeded();

  // The implementation of the "paint definition" concept:
  // https://drafts.css-houdini.org/css-paint-api/#paint-definition
  typedef HeapHashMap<String, Member<CSSPaintDefinition>> DefinitionMap;
  DefinitionMap paint_definitions_;

  // Tracks whether this PaintWorkletGlobalScope has been registered with a
  // PaintWorkletProxyClient. Only used in worklet-thread bound
  // PaintWorkletGlobalScopes.
  bool registered_ = false;

  // Default initialized to generate a distinct token for this worklet.
  const PaintWorkletToken token_;
};

template <>
struct DowncastTraits<PaintWorkletGlobalScope> {
  static bool AllowFrom(const ExecutionContext& context) {
    return context.IsPaintWorkletGlobalScope();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_WORKLET_GLOBAL_SCOPE_H_
