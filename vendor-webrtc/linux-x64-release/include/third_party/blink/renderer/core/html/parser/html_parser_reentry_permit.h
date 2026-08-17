// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_PARSER_REENTRY_PERMIT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_PARSER_REENTRY_PERMIT_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// The HTML spec for parsing controls reentering the parser from
// script with the "parser pause flag" and "script nesting level."
//
// The parser pause flag puts a brake on whether the tokenizer will
// produce more tokens. When the parser is paused, nested invocations
// of the tokenizer unwind.
//
// The script nesting level is incremented and decremented any time
// the parser causes script to run. The script nesting level:
//
// - May prevent document.open from blowing away the document.
//
// - Governs whether a script element becomes the "pending
//   parsing-blocking script." The pending parsing-blocking script in
//   turn affects whether document.write reenters the parser.
//
// Clearing the parser pause flag is simple: Whenever the script
// nesting level hits zero, the parser pause flag is cleared. However
// setting the parser pause flag is subtle.
//
// Processing a typical script end tag, or running a chain of pending
// parser-blocking scripts after that, does not set the parser pause
// flag. However recursively parsing end script tags, or running
// custom element constructors, does set the parser pause flag.
class HTMLParserReentryPermit final
    : public GarbageCollected<HTMLParserReentryPermit> {
 public:
  HTMLParserReentryPermit();
  HTMLParserReentryPermit(const HTMLParserReentryPermit&) = delete;
  HTMLParserReentryPermit& operator=(const HTMLParserReentryPermit&) = delete;
  ~HTMLParserReentryPermit() = default;

  unsigned ScriptNestingLevel() const { return script_nesting_level_; }
  bool ParserPauseFlag() const { return parser_pause_flag_; }
  void Pause() {
    CHECK(script_nesting_level_);
    parser_pause_flag_ = true;
  }

  class ScriptNestingLevelIncrementer final {
    STACK_ALLOCATED();

   public:
    explicit ScriptNestingLevelIncrementer(HTMLParserReentryPermit* permit)
        : permit_(permit) {
      permit_->script_nesting_level_++;
    }

    ScriptNestingLevelIncrementer(const ScriptNestingLevelIncrementer&) =
        delete;
    ScriptNestingLevelIncrementer& operator=(
        const ScriptNestingLevelIncrementer&) = delete;

    ScriptNestingLevelIncrementer(ScriptNestingLevelIncrementer&&) = default;

    ~ScriptNestingLevelIncrementer() {
      permit_->script_nesting_level_--;
      if (!permit_->script_nesting_level_)
        permit_->parser_pause_flag_ = false;
    }

   private:
    HTMLParserReentryPermit* permit_;
  };

  ScriptNestingLevelIncrementer IncrementScriptNestingLevel() {
    return ScriptNestingLevelIncrementer(this);
  }

  void Trace(Visitor*) const {}

 private:
  // https://html.spec.whatwg.org/C/#script-nesting-level
  unsigned script_nesting_level_ = 0;

  // https://html.spec.whatwg.org/C/#parser-pause-flag
  bool parser_pause_flag_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_PARSER_REENTRY_PERMIT_H_
