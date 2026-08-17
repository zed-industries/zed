// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_SAVE_POINT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_SAVE_POINT_H_

#include <type_traits>

#include "base/memory/stack_allocated.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_token_stream.h"

namespace blink {

// A RAII helper that allows you to rewind the parser if needed
// (e.g., if parsing fails after you've already consumed a few tokens).
class CSSParserSavePoint {
  STACK_ALLOCATED();

 public:
  explicit CSSParserSavePoint(CSSParserTokenStream& stream)
      : stream_(stream), savepoint_(stream.Save()) {}

  ~CSSParserSavePoint() {
    if (!released_) {
      stream_.EnsureLookAhead();
      stream_.Restore(savepoint_);
    }
  }

  void Release() {
    DCHECK(!released_);
    released_ = true;
  }

 private:
  CSSParserTokenStream& stream_;
  CSSParserTokenStream::State savepoint_;
  bool released_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_SAVE_POINT_H_
