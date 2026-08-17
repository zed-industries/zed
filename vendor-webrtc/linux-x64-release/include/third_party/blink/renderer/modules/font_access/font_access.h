// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FONT_ACCESS_FONT_ACCESS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FONT_ACCESS_FONT_ACCESS_H_

#include "base/memory/read_only_shared_memory_region.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/font_access/font_access.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {
class FontMetadata;
class LocalDOMWindow;
class QueryOptions;
class ScriptState;

class FontAccess final : public GarbageCollected<FontAccess>,
                         public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  explicit FontAccess(LocalDOMWindow* window);

  void Trace(blink::Visitor* visitor) const override;

  // Web-exposed interface:
  static ScriptPromise<IDLSequence<FontMetadata>> queryLocalFonts(
      ScriptState* script_state,
      LocalDOMWindow& window,
      const QueryOptions* options,
      ExceptionState& exception_state);

 private:
  // Returns the supplement, creating one as needed.
  static FontAccess* From(LocalDOMWindow* window);

  ScriptPromise<IDLSequence<FontMetadata>> QueryLocalFontsImpl(
      ScriptState* script_state,
      const QueryOptions* options,
      ExceptionState& exception_state);

  void DidGetEnumerationResponse(
      const QueryOptions* options,
      ScriptPromiseResolver<IDLSequence<FontMetadata>>* resolver,
      mojom::blink::FontEnumerationStatus status,
      base::ReadOnlySharedMemoryRegion region);

  // Returns whether the resolver has rejected.
  bool RejectPromiseIfNecessary(
      const mojom::blink::FontEnumerationStatus& status,
      ScriptPromiseResolverBase* resolver);

  void OnDisconnect();

  HeapMojoRemote<mojom::blink::FontAccessManager> remote_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FONT_ACCESS_FONT_ACCESS_H_
