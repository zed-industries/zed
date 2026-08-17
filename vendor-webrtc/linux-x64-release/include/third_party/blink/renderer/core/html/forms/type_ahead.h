/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_TYPE_AHEAD_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_TYPE_AHEAD_H_

#include <optional>

#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class KeyboardEvent;

class CORE_EXPORT TypeAheadDataSource {
 public:
  virtual ~TypeAheadDataSource() = default;

  virtual int IndexOfSelectedOption() const = 0;
  virtual int OptionCount() const = 0;
  virtual String OptionAtIndex(int index) const = 0;
};

class CORE_EXPORT TypeAhead {
  DISALLOW_NEW();

 public:
  TypeAhead(TypeAheadDataSource*);

  enum ModeFlag {
    kMatchPrefix = 1 << 0,
    kCycleFirstChar = 1 << 1,
    kMatchIndex = 1 << 2,
  };
  using MatchModeFlags = unsigned;

  // Returns the index for the matching option.
  int HandleEvent(const KeyboardEvent&, UChar charCode, MatchModeFlags);
  bool HasActiveSession(const KeyboardEvent&);
  void ResetSession();

 private:
  TypeAheadDataSource* data_source_;
  // platform timestamp of last keyboard event in seconds
  std::optional<base::TimeTicks> last_type_time_;
  UChar repeating_char_;
  StringBuilder buffer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_TYPE_AHEAD_H_
