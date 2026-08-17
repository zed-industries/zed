// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SUGGESTION_TEXT_SUGGESTION_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SUGGESTION_TEXT_SUGGESTION_CONTROLLER_H_

#include <utility>
#include "third_party/blink/public/mojom/input/input_host.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace blink {

class Document;
class DocumentMarker;
class LocalDOMWindow;
class LocalFrame;
struct TextSuggestionInfo;

// This class handles functionality related to displaying a menu of text
// suggestions (e.g. from spellcheck), and performing actions relating to those
// suggestions. Android is currently the only platform that has such a menu.
class CORE_EXPORT TextSuggestionController final
    : public GarbageCollected<TextSuggestionController> {
 public:
  explicit TextSuggestionController(LocalDOMWindow&);
  TextSuggestionController(const TextSuggestionController&) = delete;
  TextSuggestionController& operator=(const TextSuggestionController&) = delete;

  bool IsMenuOpen() const;

  void HandlePotentialSuggestionTap(const PositionInFlatTree& caret_position);

  void ApplySpellCheckSuggestion(const String& suggestion);
  void ApplyTextSuggestion(int32_t marker_tag, uint32_t suggestion_index);
  void DeleteActiveSuggestionRange();
  void OnNewWordAddedToDictionary(const String& word);
  void OnSuggestionMenuClosed();
  void SuggestionMenuTimeoutCallback(size_t max_number_of_suggestions);

  void Trace(Visitor*) const;

 private:
  friend class TextSuggestionControllerTest;
  Document& GetDocument() const;
  bool IsAvailable() const;
  LocalFrame& GetFrame() const;

  std::pair<const Node*, const DocumentMarker*> FirstMarkerIntersectingRange(
      const EphemeralRangeInFlatTree&,
      DocumentMarker::MarkerTypes) const;
  std::pair<const Node*, const DocumentMarker*> FirstMarkerTouchingSelection(
      DocumentMarker::MarkerTypes) const;

  void AttemptToDeleteActiveSuggestionRange();
  void CallMojoShowTextSuggestionMenu(
      const Vector<TextSuggestionInfo>& text_suggestion_infos,
      const String& misspelled_word);
  void ShowSpellCheckMenu(
      const std::pair<const Text*, DocumentMarker*>& node_spelling_marker_pair);
  void ShowSuggestionMenu(
      const HeapVector<std::pair<Member<const Text>, Member<DocumentMarker>>>&
          node_suggestion_marker_pairs,
      size_t max_number_of_suggestions);
  void ReplaceActiveSuggestionRange(const String&);
  void ReplaceRangeWithText(const EphemeralRange&, const String& replacement);

  bool is_suggestion_menu_open_;
  const Member<LocalDOMWindow> window_;
  HeapMojoRemote<mojom::blink::TextSuggestionHost> text_suggestion_host_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SUGGESTION_TEXT_SUGGESTION_CONTROLLER_H_
