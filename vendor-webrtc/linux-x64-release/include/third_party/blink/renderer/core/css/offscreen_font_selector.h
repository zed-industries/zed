// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_OFFSCREEN_FONT_SELECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_OFFSCREEN_FONT_SELECTOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_font_selector_base.h"
#include "third_party/blink/renderer/core/css/font_face_cache.h"
#include "third_party/blink/renderer/core/workers/worker_global_scope.h"
#include "third_party/blink/renderer/platform/fonts/generic_font_family_settings.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ExecutionContext;
class FontDescription;
class FontFamily;

class CORE_EXPORT OffscreenFontSelector : public CSSFontSelectorBase {
 public:
  explicit OffscreenFontSelector(WorkerGlobalScope*);
  ~OffscreenFontSelector() override;

  unsigned Version() const override { return 1; }

  const FontData* GetFontData(const FontDescription&,
                              const FontFamily&) override;

  void RegisterForInvalidationCallbacks(FontSelectorClient*) override;
  void UnregisterForInvalidationCallbacks(FontSelectorClient*) override;

  const GenericFontFamilySettings& GetGenericFontFamilySettings() const {
    return generic_font_family_settings_;
  }

  void FontCacheInvalidated() override;
  void FontFaceInvalidated(FontInvalidationReason) override;

  void UpdateGenericFontFamilySettings(const GenericFontFamilySettings&);

  FontFaceCache* GetFontFaceCache() override { return font_face_cache_.Get(); }

  ExecutionContext* GetExecutionContext() const override {
    return worker_ ? worker_->GetExecutionContext() : nullptr;
  }

  void Trace(Visitor*) const override;

 protected:
  void DispatchInvalidationCallbacks();

  // `CSSFontSelectorBase` overrides
  FontMatchingMetrics* GetFontMatchingMetrics() const override;
  UseCounter* GetUseCounter() const override;

 private:
  Member<WorkerGlobalScope> worker_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_OFFSCREEN_FONT_SELECTOR_H_
