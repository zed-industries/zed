/*
 * Copyright (C) 2003, 2008, 2009 Apple Inc. All rights reserved.
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SCRIPT_REGEXP_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SCRIPT_REGEXP_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/bindings/script_state.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {

enum class MultilineMode { kMultilineDisabled, kMultilineEnabled };
// Map to, respectively: neither u nor v flags, u flag, v flag.
enum class UnicodeMode { kBmpOnly, kUnicode, kUnicodeSets };

class PLATFORM_EXPORT ScriptRegexp final : public GarbageCollected<ScriptRegexp> {
 public:
  // For TextCaseSensitivity argument, TextCaseASCIIInsensitive just adds "i"
  // flag, which is not ASCII case-insensitive match.
  ScriptRegexp(v8::Isolate* isolate,
               const String&,
               TextCaseSensitivity,
               MultilineMode = MultilineMode::kMultilineDisabled,
               UnicodeMode = UnicodeMode::kBmpOnly);

  ScriptRegexp(const ScriptRegexp&) = delete;
  ScriptRegexp& operator=(const ScriptRegexp&) = delete;

  // Attempt to match the given input string against the regexp.  Returns the
  // index of the match within the input string on success and -1 otherwise.
  // If |match_length| is provided, then its populated with the length of the
  // match on success.  If |group_list| is provided its populated with the
  // matched groups within the regexp.  These are the values normally starting
  // at index 1 within the array returned from Regexp.exec().  |group_list|
  // must be empty if it is provided.
  int Match(StringView,
            int start_from = 0,
            int* match_length = nullptr,
            WTF::Vector<String>* group_list = nullptr) const;

  bool IsValid() const { return !regex_.IsEmpty(); }
  // exceptionMessage is available only if !isValid().
  String ExceptionMessage() const { return exception_message_; }

  void Trace(Visitor* visitor) const;

 private:
  Member<ScriptState> script_state_;
  TraceWrapperV8Reference<v8::RegExp> regex_;
  String exception_message_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SCRIPT_REGEXP_H_
