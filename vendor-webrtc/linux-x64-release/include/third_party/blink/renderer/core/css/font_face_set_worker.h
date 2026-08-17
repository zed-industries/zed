// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FONT_FACE_SET_WORKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FONT_FACE_SET_WORKER_H_

#include "third_party/blink/renderer/bindings/core/v8/iterable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/font_face.h"
#include "third_party/blink/renderer/core/css/font_face_set.h"
#include "third_party/blink/renderer/core/css/offscreen_font_selector.h"
#include "third_party/blink/renderer/core/workers/worker_global_scope.h"
#include "third_party/blink/renderer/core/workers/worker_thread.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Font;

class CORE_EXPORT FontFaceSetWorker final
    : public FontFaceSet,
      public Supplement<WorkerGlobalScope> {
 public:
  static const char kSupplementName[];

  explicit FontFaceSetWorker(WorkerGlobalScope&);
  FontFaceSetWorker(const FontFaceSetWorker&) = delete;
  FontFaceSetWorker& operator=(const FontFaceSetWorker&) = delete;
  ~FontFaceSetWorker() override;

  ScriptPromise<FontFaceSet> ready(ScriptState*) override;

  WorkerGlobalScope* GetWorker() const;

  // FontFace::LoadFontCallback
  void NotifyLoaded(FontFace*) override;
  void NotifyError(FontFace*) override;

  void BeginFontLoading(FontFace*);

  static FontFaceSetWorker* From(WorkerGlobalScope&);

  void Trace(Visitor*) const override;

 protected:
  bool InActiveContext() const override { return true; }
  FontSelector* GetFontSelector() const override {
    // TODO(Fserb): tracking down crbug.com/988125, can be DCHECK later.
    CHECK(GetWorker()->GetThread()->IsCurrentThread());
    return GetWorker()->GetFontSelector();
  }
  // For workers, this is always an empty list.
  const HeapLinkedHashSet<Member<FontFace>>& CSSConnectedFontFaceList()
      const override {
    DCHECK(
        GetFontSelector()->GetFontFaceCache()->CssConnectedFontFaces().size() ==
        0);
    return GetFontSelector()->GetFontFaceCache()->CssConnectedFontFaces();
  }

  const Font* ResolveFontStyle(const String&) override;

 private:
  void FireDoneEventIfPossible() override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FONT_FACE_SET_WORKER_H_
