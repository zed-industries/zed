// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_EYEDROPPER_EYE_DROPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_EYEDROPPER_EYE_DROPPER_H_

#include <memory>

#include "third_party/blink/public/mojom/choosers/color_chooser.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class AbortSignal;
class ColorSelectionOptions;
class ColorSelectionResult;
enum class DOMExceptionCode;
class ExceptionState;
class ScopedAbortState;

// The EyeDropper API enables developers to use a browser-supplied eyedropper
// in their web applications. This feature is still
// under development, and is not part of the standard. It can be enabled
// by passing --enable-blink-features=EyeDropperAPI. See
// https://github.com/WICG/eyedropper-api/blob/main/README.md
// for more details.
class EyeDropper final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit EyeDropper(ExecutionContext*);
  static EyeDropper* Create(ExecutionContext*);
  EyeDropper(const EyeDropper&) = delete;
  EyeDropper& operator=(const EyeDropper&) = delete;
  ~EyeDropper() override = default;

  // Opens the eyedropper and replaces the cursor with a browser-defined
  // preview.
  ScriptPromise<ColorSelectionResult> open(ScriptState*,
                                           const ColorSelectionOptions*,
                                           ExceptionState&);

  void Trace(Visitor*) const override;

 private:
  class OpenAbortAlgorithm;

  void AbortCallback(AbortSignal* signal);
  void EyeDropperResponseHandler(std::unique_ptr<ScopedAbortState>,
                                 ScriptPromiseResolver<ColorSelectionResult>*,
                                 bool,
                                 uint32_t);
  void EndChooser(std::unique_ptr<ScopedAbortState>);
  void RejectPromiseHelper(DOMExceptionCode, const WTF::String&);

  HeapMojoRemote<mojom::blink::EyeDropperChooser> eye_dropper_chooser_;
  Member<ScriptPromiseResolver<ColorSelectionResult>> resolver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_EYEDROPPER_EYE_DROPPER_H_
