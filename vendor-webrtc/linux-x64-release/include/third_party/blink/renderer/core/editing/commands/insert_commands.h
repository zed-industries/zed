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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_INSERT_COMMANDS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_INSERT_COMMANDS_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class DocumentFragment;
class Event;
class HTMLElement;
class LocalFrame;

enum class EditorCommandSource;

// This class provides static functions about commands related to insert.
class InsertCommands {
  STATIC_ONLY(InsertCommands);

 public:
  // Returns |bool| value for Document#execCommand().
  static bool ExecuteInsertBacktab(LocalFrame&,
                                   Event*,
                                   EditorCommandSource,
                                   const WTF::String&);
  static bool ExecuteInsertHorizontalRule(LocalFrame&,
                                          Event*,
                                          EditorCommandSource,
                                          const WTF::String&);
  static bool ExecuteInsertHTML(LocalFrame&,
                                Event*,
                                EditorCommandSource,
                                const WTF::String&);
  static bool ExecuteInsertImage(LocalFrame&,
                                 Event*,
                                 EditorCommandSource,
                                 const WTF::String&);
  static bool ExecuteInsertLineBreak(LocalFrame&,
                                     Event*,
                                     EditorCommandSource,
                                     const WTF::String&);
  static bool ExecuteInsertNewline(LocalFrame&,
                                   Event*,
                                   EditorCommandSource,
                                   const WTF::String&);
  static bool ExecuteInsertNewlineInQuotedContent(LocalFrame&,
                                                  Event*,
                                                  EditorCommandSource,
                                                  const WTF::String&);
  static bool ExecuteInsertOrderedList(LocalFrame&,
                                       Event*,
                                       EditorCommandSource,
                                       const WTF::String&);
  static bool ExecuteInsertParagraph(LocalFrame&,
                                     Event*,
                                     EditorCommandSource,
                                     const WTF::String&);
  static bool ExecuteInsertTab(LocalFrame&,
                               Event*,
                               EditorCommandSource,
                               const WTF::String&);
  static bool ExecuteInsertText(LocalFrame&,
                                Event*,
                                EditorCommandSource,
                                const WTF::String&);
  static bool ExecuteInsertUnorderedList(LocalFrame&,
                                         Event*,
                                         EditorCommandSource,
                                         const WTF::String&);

 private:
  static bool ExecuteInsertFragment(LocalFrame&, DocumentFragment*);
  static bool ExecuteInsertElement(LocalFrame&, HTMLElement*);

  // Related to Editor::selectionForCommand.
  // Certain operations continue to use the target control's selection even if
  // the event handler already moved the selection outside of the text control.
  static LocalFrame& TargetFrame(LocalFrame&, Event*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_INSERT_COMMANDS_H_
