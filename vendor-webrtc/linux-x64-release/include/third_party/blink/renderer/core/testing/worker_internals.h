// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_WORKER_INTERNALS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_WORKER_INTERNALS_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExceptionState;
class CanvasImageSource;
class CanvasRenderingContext;
class OriginTrialsTest;
class ScriptState;

class WorkerInternals final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit WorkerInternals();
  ~WorkerInternals() override;

  OriginTrialsTest* originTrialsTest() const;
  void countFeature(ScriptState*, uint32_t feature, ExceptionState&);
  void countDeprecation(ScriptState*, uint32_t feature, ExceptionState&);

  void collectGarbage(ScriptState*);

  void forceLoseCanvasContext(CanvasRenderingContext* ctx);
  bool isCanvasImageSourceAccelerated(const CanvasImageSource*) const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_WORKER_INTERNALS_H_
