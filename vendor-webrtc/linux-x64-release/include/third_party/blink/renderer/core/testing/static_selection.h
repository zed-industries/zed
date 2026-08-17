// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_STATIC_SELECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_STATIC_SELECTION_H_

#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class Node;

class StaticSelection final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static StaticSelection* FromSelectionInDOMTree(const SelectionInDOMTree&);
  static StaticSelection* FromSelectionInFlatTree(const SelectionInFlatTree&);

  explicit StaticSelection(const SelectionInFlatTree&);
  explicit StaticSelection(const SelectionInDOMTree&);
  StaticSelection(const StaticSelection&) = delete;
  StaticSelection& operator=(const StaticSelection&) = delete;

  Node* anchorNode() const { return anchor_node_.Get(); }
  unsigned anchorOffset() const { return anchor_offset_; }
  Node* focusNode() const { return focus_node_.Get(); }
  unsigned focusOffset() const { return focus_offset_; }
  bool isCollapsed() const;

  void Trace(Visitor*) const override;

 private:
  const Member<Node> anchor_node_;
  const unsigned anchor_offset_;
  const Member<Node> focus_node_;
  const unsigned focus_offset_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_STATIC_SELECTION_H_
