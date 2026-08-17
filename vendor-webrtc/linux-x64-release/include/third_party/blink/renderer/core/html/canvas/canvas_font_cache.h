// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_FONT_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_FONT_CACHE_H_

#include <memory>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_value_set.h"
#include "third_party/blink/renderer/platform/fonts/font.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/linked_hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ComputedStyle;
class Document;
class FontCachePurgePreventer;
class HTMLCanvasElement;

class CORE_EXPORT CanvasFontCache final
    : public GarbageCollected<CanvasFontCache>,
      public Thread::TaskObserver {
  USING_PRE_FINALIZER(CanvasFontCache, Dispose);

 public:
  explicit CanvasFontCache(Document&);

  MutableCSSPropertyValueSet* ParseFont(const String&);
  void PruneAll();
  unsigned size();

  void Trace(Visitor*) const;

  static unsigned MaxFonts();
  unsigned HardMaxFonts();

  void WillUseCurrentFont() { SchedulePruningIfNeeded(); }
  const Font* GetFontUsingDefaultStyle(HTMLCanvasElement& canvas,
                                       const String&);

  // TaskObserver implementation
  void DidProcessTask(const base::PendingTask&) override;
  void WillProcessTask(const base::PendingTask&, bool) override {}

  // For testing
  bool IsInCache(const String&) const;
  unsigned int GetCacheSize() const;

  ~CanvasFontCache() override;

 private:
  void Dispose();
  void SchedulePruningIfNeeded();
  typedef HeapHashMap<String, Member<MutableCSSPropertyValueSet>>
      MutableStylePropertyMap;

  HeapHashMap<String, Member<const Font>> fonts_resolved_using_default_style_;
  MutableStylePropertyMap fetched_fonts_;
  LinkedHashSet<String> font_lru_list_;
  std::unique_ptr<FontCachePurgePreventer> main_cache_purge_preventer_;
  Member<Document> document_;
  Member<const ComputedStyle> default_font_style_;
  bool pruning_scheduled_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_FONT_CACHE_H_
