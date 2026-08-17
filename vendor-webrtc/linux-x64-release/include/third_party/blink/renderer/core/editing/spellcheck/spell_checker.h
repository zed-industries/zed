/*
 * Copyright (C) 2006, 2007, 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_SPELL_CHECKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_SPELL_CHECKER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker.h"
#include "third_party/blink/renderer/core/editing/spellcheck/text_checking.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class DocumentMarkerGroup;
class Element;
class IdleSpellCheckController;
class LocalDOMWindow;
class LocalFrame;
class HTMLElement;
class Node;
class SpellCheckMarker;
class SpellCheckRequest;
class SpellCheckRequester;
struct TextCheckingResult;
class WebSpellCheckPanelHostClient;
class WebTextCheckClient;

class CORE_EXPORT SpellChecker final : public GarbageCollected<SpellChecker> {
 public:
  explicit SpellChecker(LocalDOMWindow&);
  SpellChecker(const SpellChecker&) = delete;
  SpellChecker& operator=(const SpellChecker&) = delete;

  void Trace(Visitor*) const;

  WebSpellCheckPanelHostClient& SpellCheckPanelHostClient() const;
  WebTextCheckClient* GetTextCheckerClient() const;

  static bool IsSpellCheckingEnabledAt(const Position&);
  bool IsSpellCheckingEnabled() const;
  void IgnoreSpelling();
  void MarkAndReplaceFor(SpellCheckRequest*, const Vector<TextCheckingResult>&);
  void AdvanceToNextMisspelling(bool start_before_selection);
  void ShowSpellingGuessPanel();
  void RespondToChangedContents();
  void RespondToChangedSelection();
  void RespondToChangedEnablement(const HTMLElement&, bool enabled);
  DocumentMarkerGroup* GetSpellCheckMarkerGroupUnderSelection() const;
  // The first String returned in the pair is the selected text.
  // The second String is the marker's description.
  std::pair<String, String> SelectMisspellingAsync();
  void ReplaceMisspelledRange(const String&);
  void RemoveSpellingMarkers();
  void RemoveSpellingMarkersUnderWords(const Vector<String>& words);
  enum class ElementsType { kAll, kOnlyNonEditable };
  void RemoveSpellingAndGrammarMarkers(const HTMLElement&,
                                       ElementsType = ElementsType::kAll);

  void DidEndEditingOnTextField(Element*);
  bool SelectionStartHasMarkerFor(DocumentMarker::MarkerType,
                                  int from,
                                  int length) const;

  void ElementRemoved(Element*);

  // Exposed for testing and idle time spell checker
  SpellCheckRequester& GetSpellCheckRequester() const {
    return *spell_check_requester_;
  }
  IdleSpellCheckController& GetIdleSpellCheckController() const {
    return *idle_spell_check_controller_;
  }

 private:
  LocalFrame& GetFrame() const;

  // Helper functions for advanceToNextMisspelling()
  Vector<TextCheckingResult> FindMisspellings(const String&);
  std::pair<String, int> FindFirstMisspelling(const Position&, const Position&);

  void RemoveMarkers(const EphemeralRange&, DocumentMarker::MarkerTypes);

  Member<LocalDOMWindow> window_;

  const Member<SpellCheckRequester> spell_check_requester_;
  const Member<IdleSpellCheckController> idle_spell_check_controller_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_SPELL_CHECKER_H_
