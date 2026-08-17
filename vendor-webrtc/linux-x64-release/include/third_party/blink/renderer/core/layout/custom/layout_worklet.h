// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_LAYOUT_WORKLET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_LAYOUT_WORKLET_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/custom/document_layout_definition.h"
#include "third_party/blink/renderer/core/layout/custom/pending_layout_registry.h"
#include "third_party/blink/renderer/core/workers/worklet.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Node;
class LayoutWorkletGlobalScopeProxy;

extern DocumentLayoutDefinition* const kInvalidDocumentLayoutDefinition;

// Manages a layout worklet:
// https://drafts.css-houdini.org/css-layout-api/#dom-css-layoutworklet
//
// Provides access to web developer defined layout classes within multiple
// global scopes.
class CORE_EXPORT LayoutWorklet : public Worklet,
                                  public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  // At the moment, layout worklet allows at most two global scopes at any time.
  static const size_t kNumGlobalScopes;
  static LayoutWorklet* From(LocalDOMWindow&);

  explicit LayoutWorklet(LocalDOMWindow&);
  LayoutWorklet(const LayoutWorklet&) = delete;
  LayoutWorklet& operator=(const LayoutWorklet&) = delete;
  ~LayoutWorklet() override;

  typedef HeapHashMap<String, Member<DocumentLayoutDefinition>>
      DocumentDefinitionMap;
  DocumentDefinitionMap* GetDocumentDefinitionMap() {
    return &document_definition_map_;
  }

  void AddPendingLayout(const AtomicString& name, Node*);
  LayoutWorkletGlobalScopeProxy* Proxy();

  void Trace(Visitor*) const override;

 protected:
  // TODO(ikilpatrick): Make selection of the global scope non-deterministic.
  wtf_size_t SelectGlobalScope() final { return 0u; }

 private:
  friend class LayoutWorkletTest;

  // Implements Worklet.
  bool NeedsToCreateGlobalScope() final;
  WorkletGlobalScopeProxy* CreateGlobalScope() final;

  DocumentDefinitionMap document_definition_map_;
  Member<PendingLayoutRegistry> pending_layout_registry_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_LAYOUT_WORKLET_H_
