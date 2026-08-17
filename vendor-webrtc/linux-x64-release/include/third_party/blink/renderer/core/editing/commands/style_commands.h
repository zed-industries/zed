/*
 * Copyright (C) 2006, 2007, 2008 Apple Inc. All rights reserved.
 * Copyright (C) 2008 Nokia Corporation and/or its subsidiary(-ies)
 * Copyright (C) 2009 Igalia S.L.
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

// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_STYLE_COMMANDS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_STYLE_COMMANDS_H_

#include "mojo/public/mojom/base/text_direction.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/events/input_event.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class CSSPropertyValueSet;
class EditingStyle;
class EditorInternalCommand;
class Event;
class LocalFrame;

enum class EditingTriState;
enum class EditorCommandSource;

// This class provides static functions about commands related to style.
class CORE_EXPORT StyleCommands {
  STATIC_ONLY(StyleCommands);

 public:
  // Returns |bool| value for Document#execCommand().
  static bool ExecuteBackColor(LocalFrame&,
                               Event*,
                               EditorCommandSource,
                               const String&);
  static bool ExecuteForeColor(LocalFrame&,
                               Event*,
                               EditorCommandSource,
                               const String&);
  static bool ExecuteFontName(LocalFrame&,
                              Event*,
                              EditorCommandSource,
                              const String&);
  static bool ExecuteFontSize(LocalFrame&,
                              Event*,
                              EditorCommandSource,
                              const String&);
  static bool ExecuteFontSizeDelta(LocalFrame&,
                                   Event*,
                                   EditorCommandSource,
                                   const String&);
  static bool ExecuteToggleBold(LocalFrame&,
                                Event*,
                                EditorCommandSource,
                                const String&);
  static bool ExecuteToggleItalic(LocalFrame&,
                                  Event*,
                                  EditorCommandSource,
                                  const String&);
  static bool ExecuteSubscript(LocalFrame&,
                               Event*,
                               EditorCommandSource,
                               const String&);
  static bool ExecuteSuperscript(LocalFrame&,
                                 Event*,
                                 EditorCommandSource,
                                 const String&);
  static bool ExecuteUnscript(LocalFrame&,
                              Event*,
                              EditorCommandSource,
                              const String&);
  static bool ExecuteStrikethrough(LocalFrame&,
                                   Event*,
                                   EditorCommandSource,
                                   const String&);
  static bool ExecuteUnderline(LocalFrame&,
                               Event*,
                               EditorCommandSource,
                               const String&);
  static bool ExecuteMakeTextWritingDirectionLeftToRight(LocalFrame&,
                                                         Event*,
                                                         EditorCommandSource,
                                                         const String&);
  static bool ExecuteMakeTextWritingDirectionNatural(LocalFrame&,
                                                     Event*,
                                                     EditorCommandSource,
                                                     const String&);
  static bool ExecuteMakeTextWritingDirectionRightToLeft(LocalFrame&,
                                                         Event*,
                                                         EditorCommandSource,
                                                         const String&);
  static bool ExecuteStyleWithCSS(LocalFrame&,
                                  Event*,
                                  EditorCommandSource,
                                  const String&);
  static bool ExecuteUseCSS(LocalFrame&,
                            Event*,
                            EditorCommandSource,
                            const String&);

  // State functions
  static EditingTriState StateStyle(LocalFrame&, CSSPropertyID, const char*);
  static EditingTriState StateBold(LocalFrame&, Event*);
  static EditingTriState StateItalic(LocalFrame&, Event*);
  static EditingTriState StateStrikethrough(LocalFrame&, Event*);
  static EditingTriState StateStyleWithCSS(LocalFrame&, Event*);
  static EditingTriState StateSubscript(LocalFrame&, Event*);
  static EditingTriState StateSuperscript(LocalFrame&, Event*);
  static EditingTriState StateTextWritingDirectionLeftToRight(LocalFrame&,
                                                              Event*);
  static EditingTriState StateTextWritingDirectionNatural(LocalFrame&, Event*);
  static EditingTriState StateTextWritingDirectionRightToLeft(LocalFrame&,
                                                              Event*);
  static EditingTriState StateUnderline(LocalFrame&, Event*);

  // Value functions
  static String ValueBackColor(const EditorInternalCommand&,
                               LocalFrame&,
                               Event*);
  static String ValueForeColor(const EditorInternalCommand&,
                               LocalFrame&,
                               Event*);
  static String ValueFontName(const EditorInternalCommand&,
                              LocalFrame&,
                              Event*);
  static String ValueFontSize(const EditorInternalCommand&,
                              LocalFrame&,
                              Event*);
  static String ValueFontSizeDelta(const EditorInternalCommand&,
                                   LocalFrame&,
                                   Event*);

 private:
  static void ApplyStyle(LocalFrame&,
                         CSSPropertyValueSet*,
                         InputEvent::InputType);
  static void ApplyStyleToSelection(LocalFrame&,
                                    CSSPropertyValueSet*,
                                    InputEvent::InputType);
  static bool ApplyCommandToFrame(LocalFrame&,
                                  EditorCommandSource,
                                  InputEvent::InputType,
                                  CSSPropertyValueSet*);
  static bool ExecuteApplyStyle(LocalFrame&,
                                EditorCommandSource,
                                InputEvent::InputType,
                                CSSPropertyID,
                                const String&);
  static bool ExecuteApplyStyle(LocalFrame&,
                                EditorCommandSource,
                                InputEvent::InputType,
                                CSSPropertyID,
                                CSSValueID);
  static bool ExecuteToggleStyle(LocalFrame&,
                                 EditorCommandSource,
                                 InputEvent::InputType,
                                 CSSPropertyID,
                                 const char* off_value,
                                 const char* on_value);

  // FIXME: executeToggleStyleInList does not handle complicated cases such as
  // <b><u>hello</u>world</b> properly. This function must use
  // EditingStyle::SelectionHasStyle to determine the current style but we
  // cannot fix this until https://bugs.webkit.org/show_bug.cgi?id=27818 is
  // resolved.
  static bool ExecuteToggleStyleInList(LocalFrame&,
                                       EditorCommandSource,
                                       InputEvent::InputType,
                                       CSSPropertyID,
                                       const CSSValue&);
  static String ComputeToggleStyleInList(EditingStyle&,
                                         CSSPropertyID,
                                         const CSSValue&);
  static bool SelectionStartHasStyle(LocalFrame&, CSSPropertyID, const String&);
  static String SelectionStartCSSPropertyValue(LocalFrame&, CSSPropertyID);
  static String ValueStyle(LocalFrame&, CSSPropertyID);
  static bool IsUnicodeBidiNestedOrMultipleEmbeddings(CSSValueID);

  // TODO(editing-dev): We should make |textDirectionForSelection()| to take
  // |selectionInDOMTree|.
  static mojo_base::mojom::blink::TextDirection
  TextDirectionForSelection(const VisibleSelection&, EditingStyle*, bool&);
  static EditingTriState StateTextWritingDirection(
      LocalFrame&,
      mojo_base::mojom::blink::TextDirection);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_STYLE_COMMANDS_H_
