// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SUGGESTION_TEXT_SUGGESTION_BACKEND_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SUGGESTION_TEXT_SUGGESTION_BACKEND_IMPL_H_

#include "base/types/pass_key.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "third_party/blink/public/mojom/input/input_messages.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class LocalFrame;

// Implementation of mojom::blink::TextSuggestionBackend
class CORE_EXPORT TextSuggestionBackendImpl final
    : public GarbageCollected<TextSuggestionBackendImpl>,
      public mojom::blink::TextSuggestionBackend,
      public Supplement<LocalFrame> {
 public:
  static const char kSupplementName[];
  static TextSuggestionBackendImpl* From(LocalFrame&);
  static void Bind(LocalFrame*,
                   mojo::PendingReceiver<mojom::blink::TextSuggestionBackend>);

  explicit TextSuggestionBackendImpl(
      base::PassKey<TextSuggestionBackendImpl>,
      LocalFrame&,
      mojo::PendingReceiver<mojom::blink::TextSuggestionBackend> receiver);

  // Not copyable or movable
  TextSuggestionBackendImpl(const TextSuggestionBackendImpl&) = delete;
  TextSuggestionBackendImpl& operator=(const TextSuggestionBackendImpl&) =
      delete;

  void Trace(Visitor* visitor) const override;

  void ApplySpellCheckSuggestion(const String& suggestion) final;
  void ApplyTextSuggestion(int32_t marker_tag, int32_t suggestion_index) final;
  void DeleteActiveSuggestionRange() final;
  void OnNewWordAddedToDictionary(const String& word) final;
  void OnSuggestionMenuClosed() final;
  void SuggestionMenuTimeoutCallback(int32_t max_number_of_suggestions) final;

 private:
  HeapMojoReceiver<mojom::blink::TextSuggestionBackend,
                   TextSuggestionBackendImpl,
                   HeapMojoWrapperMode::kForceWithoutContextObserver>
      receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SUGGESTION_TEXT_SUGGESTION_BACKEND_IMPL_H_
