/*
 * Copyright (C) 2007, 2008, 2011 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_SELECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_SELECTOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_font_selector_base.h"
#include "third_party/blink/renderer/core/css/font_face_cache.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/platform/fonts/generic_font_family_settings.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class FontDescription;
class FontFamily;

// `CSSFontSelector` is owned by `StyleEngine`. There is derived class
// ` PopupMenuCSSFontSelector`.
class CORE_EXPORT CSSFontSelector : public CSSFontSelectorBase {
 public:
  explicit CSSFontSelector(const TreeScope&);
  ~CSSFontSelector() override;

  unsigned Version() const override { return font_face_cache_->Version(); }

  const FontData* GetFontData(const FontDescription&,
                              const FontFamily&) override;

  void FontFaceInvalidated(FontInvalidationReason) override;

  // FontCacheClient implementation
  void FontCacheInvalidated() override;

  void RegisterForInvalidationCallbacks(FontSelectorClient*) override;
  void UnregisterForInvalidationCallbacks(FontSelectorClient*) override;

  ExecutionContext* GetExecutionContext() const override {
    return tree_scope_ ? GetDocument().GetExecutionContext() : nullptr;
  }
  FontFaceCache* GetFontFaceCache() override { return font_face_cache_.Get(); }

  const GenericFontFamilySettings& GetGenericFontFamilySettings() const {
    return generic_font_family_settings_;
  }
  void UpdateGenericFontFamilySettings(Document&);

  const TreeScope* GetTreeScope() const { return tree_scope_.Get(); }
  Document& GetDocument() const {
    DCHECK(tree_scope_);
    return tree_scope_->GetDocument();
  }

  void Trace(Visitor*) const override;

 protected:
  void DispatchInvalidationCallbacks(FontInvalidationReason);

  // `CSSFontSelectorBase` overrides
  bool IsAlive() const override;
  FontMatchingMetrics* GetFontMatchingMetrics() const override;
  UseCounter* GetUseCounter() const override;

 private:
  // TODO(Oilpan): Ideally this should just be a traced Member but that will
  // currently leak because ComputedStyle and its data are not on the heap.
  // See crbug.com/383860 for details.
  WeakMember<const TreeScope> tree_scope_;
  HeapHashSet<WeakMember<FontSelectorClient>> clients_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_SELECTOR_H_
