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

// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_EDITOR_COMMAND_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_EDITOR_COMMAND_H_

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/static_range.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class EditorInternalCommand;
class Event;
class LocalFrame;

enum class EditingTriState;
enum class EditorCommandSource;

class CORE_EXPORT EditorCommand {
  STACK_ALLOCATED();

 public:
  EditorCommand();
  EditorCommand(const EditorInternalCommand*, EditorCommandSource, LocalFrame*);

  bool Execute(const String& parameter = String(),
               Event* triggering_event = nullptr) const;
  bool Execute(Event* triggering_event) const;

  bool CanExecute(Event* triggering_event = nullptr) const;
  bool IsSupported() const;
  bool IsEnabled(Event* triggering_event = nullptr) const;

  EditingTriState GetState(Event* triggering_event = nullptr) const;
  String Value(Event* triggering_event = nullptr) const;

  bool IsTextInsertion() const;
  bool IsValueInterpretedAsHTML() const;

  // Returns 0 if this EditorCommand is not supported.
  int IdForHistogram() const;

 private:
  LocalFrame& GetFrame() const;

  FRIEND_TEST_ALL_PREFIXES(EditingCommandTest,
                           DeleteSoftLineBackwardTargetRanges);
  // Returns target ranges for the command, currently only supports delete
  // related commands. Used by InputEvent.
  const GCedStaticRangeVector* GetTargetRanges() const;

  const EditorInternalCommand* command_;
  const EditorCommandSource source_;
  LocalFrame* const frame_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_EDITOR_COMMAND_H_
