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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_CLIPBOARD_COMMANDS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_CLIPBOARD_COMMANDS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class DocumentFragment;
class Element;
class Event;
class ExecutionContext;
class LocalFrame;

enum class DataTransferAccessPolicy;
enum class EditorCommandSource;
enum class PasteMode;

// This class provides static functions about commands related to clipboard.
class CORE_EXPORT ClipboardCommands {
  STATIC_ONLY(ClipboardCommands);

 public:
  static bool EnabledCopy(LocalFrame&, Event*, EditorCommandSource);
  static bool EnabledCut(LocalFrame&, Event*, EditorCommandSource);
  static bool EnabledPaste(LocalFrame&, Event*, EditorCommandSource);

  // Returns |bool| value for Document#execCommand().
  static bool ExecuteCopy(LocalFrame&,
                          Event*,
                          EditorCommandSource,
                          const String&);
  static bool ExecuteCut(LocalFrame&,
                         Event*,
                         EditorCommandSource,
                         const String&);
  static bool ExecutePaste(LocalFrame&,
                           Event*,
                           EditorCommandSource,
                           const String&);
  static bool ExecutePasteGlobalSelection(LocalFrame&,
                                          Event*,
                                          EditorCommandSource,
                                          const String&);
  static bool ExecutePasteAndMatchStyle(LocalFrame&,
                                        Event*,
                                        EditorCommandSource,
                                        const String&);
  static bool ExecutePasteFromImageURL(LocalFrame&,
                                       Event*,
                                       EditorCommandSource,
                                       const String&);

  static bool PasteSupported(LocalFrame*);

  static bool CanReadClipboard(LocalFrame&, EditorCommandSource);
  static bool CanWriteClipboard(LocalFrame&, EditorCommandSource);

  // Returns true when handling a "cut" or "copy" command that originated from
  // the user agent.
  static bool IsExecutingCutOrCopy(ExecutionContext&);
  // As above, but for the "paste" event.
  static bool IsExecutingPaste(ExecutionContext&);

 private:
  static bool CanSmartReplaceInClipboard(LocalFrame&);
  static bool CanDeleteRange(const EphemeralRange&);
  static Element* FindEventTargetForClipboardEvent(LocalFrame&,
                                                   EditorCommandSource);

  // Returns true if Editor should continue with default processing.
  static bool DispatchClipboardEvent(LocalFrame&,
                                     const AtomicString&,
                                     DataTransferAccessPolicy,
                                     EditorCommandSource,
                                     PasteMode);
  static bool DispatchCopyOrCutEvent(LocalFrame&,
                                     EditorCommandSource,
                                     const AtomicString&);
  static bool DispatchPasteEvent(LocalFrame&, PasteMode, EditorCommandSource);

  static void WriteSelectionToClipboard(LocalFrame&);
  static void Paste(LocalFrame&, EditorCommandSource);
  static void PasteAsFragment(LocalFrame&,
                              DocumentFragment*,
                              bool smart_replace,
                              bool match_style,
                              EditorCommandSource);
  static void PasteAsPlainTextFromClipboard(LocalFrame&, EditorCommandSource);
  static void PasteFromClipboard(LocalFrame&, EditorCommandSource);
  static void PasteFromImageURL(LocalFrame&, EditorCommandSource, String);

  using FragmentAndPlainText = std::pair<DocumentFragment*, const bool>;
  static FragmentAndPlainText GetFragmentFromClipboard(LocalFrame&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_CLIPBOARD_COMMANDS_H_
