// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_PART_ROOT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_PART_ROOT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/container_node.h"
#include "third_party/blink/renderer/core/dom/part_root.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class Document;

// Implementation of the DocumentPartRoot class, which is part of the DOM Parts
// API. A DocumentPartRoot holds the parts for a Document or DocumentFragment.
// A Document always owns one DocumentPartRoot.
class CORE_EXPORT DocumentPartRoot : public ScriptWrappable, public PartRoot {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit DocumentPartRoot(ContainerNode& root_container)
      : root_container_(root_container) {}
  DocumentPartRoot(const DocumentPartRoot&) = delete;
  ~DocumentPartRoot() override = default;

  Document& GetDocument() const override {
    return root_container_->GetDocument();
  }
  bool IsDocumentPartRoot() const override { return true; }
  Node* FirstIncludedChildNode() const override {
    return root_container_->firstChild();
  }
  Node* LastIncludedChildNode() const override {
    return root_container_->lastChild();
  }

  void Trace(Visitor*) const override;

  // PartRoot API
  PartRootUnion* clone(ExceptionState&);
  ContainerNode* rootContainer() const override {
    return root_container_.Get();
  }

 protected:
  const PartRoot* GetParentPartRoot() const override { return nullptr; }

 private:
  Member<ContainerNode> root_container_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_PART_ROOT_H_
