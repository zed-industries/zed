/*
 * Copyright (C) 2010 Google, Inc. All Rights Reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCRIPTABLE_DOCUMENT_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCRIPTABLE_DOCUMENT_PARSER_H_

#include "base/synchronization/lock.h"
#include "third_party/blink/renderer/bindings/core/v8/script_streamer.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/parser/css_tokenizer.h"
#include "third_party/blink/renderer/core/dom/decoded_data_document_parser.h"
#include "third_party/blink/renderer/core/dom/parser_content_policy.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"

namespace blink {

class CORE_EXPORT ScriptableDocumentParser : public DecodedDataDocumentParser {
 public:
  // Only used by Document::open for deciding if its safe to act on a
  // JavaScript document.open() call right now, or it should be ignored.
  virtual bool IsExecutingScript() const { return false; }

  virtual void ExecuteScriptsWaitingForResources() = 0;

  virtual bool IsWaitingForScripts() const = 0;
  virtual void DidAddPendingParserBlockingStylesheet() = 0;
  virtual void DidLoadAllPendingParserBlockingStylesheets() = 0;

  // These are used to expose the current line/column to the scripting system.
  virtual bool IsParsingAtLineNumber() const;
  virtual OrdinalNumber LineNumber() const = 0;
  virtual TextPosition GetTextPosition() const = 0;

  void SetWasCreatedByScript(bool was_created_by_script) {
    was_created_by_script_ = was_created_by_script;
  }
  bool WasCreatedByScript() const { return was_created_by_script_; }

  ParserContentPolicy GetParserContentPolicy() {
    return parser_content_policy_;
  }

  // Adds a script streamer for |source| which can be later retrieved with
  // TakeInlineScriptStreamer(). This may be called on any thread.
  void AddInlineScriptStreamer(
      const String& source,
      scoped_refptr<BackgroundInlineScriptStreamer> streamer);

  // Takes a script streamer previously added with AddInlineScriptStreamer().
  // The returned streamer is guaranteed to be correct for script text that
  // matches the passed in |source|.
  InlineScriptStreamer* TakeInlineScriptStreamer(const String& source);
  bool HasInlineScriptStreamerForTesting(const String& source);

 protected:
  explicit ScriptableDocumentParser(
      Document&,
      ParserContentPolicy = kAllowScriptingContent);

 private:
  ScriptableDocumentParser* AsScriptableDocumentParser() final { return this; }

  // http://www.whatwg.org/specs/web-apps/current-work/#script-created-parser
  bool was_created_by_script_;
  ParserContentPolicy parser_content_policy_;

  base::Lock streamers_lock_;
  HashMap<String, scoped_refptr<BackgroundInlineScriptStreamer>>
      inline_script_streamers_ GUARDED_BY(streamers_lock_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCRIPTABLE_DOCUMENT_PARSER_H_
