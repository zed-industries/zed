// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ACCESSIBILITY_AX_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ACCESSIBILITY_AX_CONTEXT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "ui/accessibility/ax_mode.h"

namespace blink {

// An AXContext enables accessibility support in a Document for as
// long as the AXContext is alive. While the AXContext exists,
// Document::ExistingAXObjectCache will always return a valid
// AXObjectCache if the document is still active.
class CORE_EXPORT AXContext {
  USING_FAST_MALLOC(AXContext);

 public:
  AXContext(Document& document, const ui::AXMode& mode);
  AXContext(const AXContext&) = delete;
  AXContext& operator=(const AXContext&) = delete;
  virtual ~AXContext();

  // Note: it's an error to call this after |document| is no longer active.
  // The caller should check this.
  AXObjectCache& GetAXObjectCache();

  // Returns true if the |document| associated to this |AXContext| is active
  // (i.e. document has been initialized and hasn't been detached yet).
  bool HasActiveDocument();

  Document* GetDocument();

  const ui::AXMode& GetAXMode() const { return ax_mode_; }

  void SetAXMode(const ui::AXMode&);

 protected:
  WeakPersistent<Document> document_;
  ui::AXMode ax_mode_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ACCESSIBILITY_AX_CONTEXT_H_
