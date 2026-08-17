// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_PART_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_PART_H_

#include "third_party/blink/renderer/bindings/core/v8/frozen_array.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Node;
class NodeCloningData;
class PartRoot;
class V8UnionChildNodePartOrDocumentPartRoot;

// Implementation of the Part class, which is part of the DOM Parts API.
class CORE_EXPORT Part : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~Part() override = default;

  void Trace(Visitor* visitor) const override;
  virtual bool IsValid() const {
    DCHECK_EQ(is_valid_, root_ && connected_);
    return is_valid_;
  }
  virtual Node* NodeToSortBy() const = 0;
  virtual Part* ClonePart(NodeCloningData&, Node&) const = 0;
  virtual PartRoot* GetAsPartRoot() const { return nullptr; }
  PartRoot* root() const { return root_.Get(); }
  virtual Document& GetDocument() const = 0;

  // Part API
  V8UnionChildNodePartOrDocumentPartRoot* rootForBindings() const;
  const FrozenArray<IDLString>& metadata() const { return *metadata_; }
  virtual void disconnect();

 protected:
  Part(PartRoot& root, Vector<String> metadata)
      : root_(root),
        metadata_(
            MakeGarbageCollected<FrozenArray<IDLString>>(std::move(metadata))) {
  }
  bool IsConnected() { return connected_; }
  static bool IsAcceptableNodeType(Node& node);

 private:
  Member<PartRoot> root_;
  Member<FrozenArray<IDLString>> metadata_;
  bool connected_{true};
  // Checking IsValid() is very hot during cloning, so |is_valid_| is
  // a cached version of (root_ && connected_),
  bool is_valid_{true};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_PART_H_
