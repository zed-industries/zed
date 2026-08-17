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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_HTML_PARSER_SCRIPT_RUNNER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_HTML_PARSER_SCRIPT_RUNNER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/html/parser/html_parser_reentry_permit.h"
#include "third_party/blink/renderer/core/script/pending_script.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_client.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"

namespace blink {

class Document;
class Element;
class HTMLParserScriptRunnerHost;

// HTMLParserScriptRunner is responsible for for arranging the execution of
// script elements inserted by the parser, according to the rules for
// 'An end tag whose tag name is "script"':
// https://html.spec.whatwg.org/C/#scriptEndTag
//
// If a script blocks parsing, this class is responsible for holding it, and
// executing it when required.
//
// An HTMLParserScriptRunner is owned by its host, an HTMLDocumentParser.
class HTMLParserScriptRunner final
    : public GarbageCollected<HTMLParserScriptRunner>,
      public PendingScriptClient,
      public NameClient {
 public:
  static HTMLParserScriptRunner* Create(HTMLParserReentryPermit* reentry_permit,
                                        Document* document,
                                        HTMLParserScriptRunnerHost* host) {
    return MakeGarbageCollected<HTMLParserScriptRunner>(reentry_permit,
                                                        document, host);
  }

  HTMLParserScriptRunner(HTMLParserReentryPermit*,
                         Document*,
                         HTMLParserScriptRunnerHost*);
  HTMLParserScriptRunner(const HTMLParserScriptRunner&) = delete;
  HTMLParserScriptRunner& operator=(const HTMLParserScriptRunner&) = delete;
  ~HTMLParserScriptRunner() override;

  // Invoked when the parser is detached.
  //
  // We don't need to call Detach() as a prefinalizer, because PendingScripts
  // are Dispose()d in PendingScripts' prefinalizers.
  void Detach();

  // Processes the passed in script and any pending scripts if possible.
  // This does not necessarily run the script immediately. For instance,
  // execution may not happen until the script loads from the network, or after
  // the document finishes parsing.
  void ProcessScriptElement(Element*,
                            const TextPosition& script_start_position);

  // Invoked when the parsing-blocking script resource has loaded, to execute
  // parsing-blocking scripts.
  void ExecuteScriptsWaitingForLoad();

  // Invoked when all script-blocking resources (e.g., stylesheets) have loaded,
  // to execute parsing-blocking scripts.
  void ExecuteScriptsWaitingForResources();

  // Invoked when parsing is stopping, to execute any developer deferred
  // scripts.
  bool ExecuteScriptsWaitingForParsing();

  bool HasParserBlockingScript() const;
  bool IsExecutingScript() const {
    return !!reentry_permit_->ScriptNestingLevel();
  }

  void Trace(Visitor*) const override;
  const char* NameInHeapSnapshot() const override {
    return "HTMLParserScriptRunner";
  }

 private:
  // PendingScriptClient
  void PendingScriptFinished(PendingScript*) override;

  void ExecutePendingParserBlockingScriptAndDispatchEvent();
  void ExecutePendingDeferredScriptAndDispatchEvent(PendingScript*);
  void ExecuteParsingBlockingScripts();

  // Processes the provided script element, but does not execute any
  // parsing-blocking scripts that may remain after execution.
  void ProcessScriptElementInternal(Element*,
                                    const TextPosition& script_start_position);

  const PendingScript* ParserBlockingScript() const {
    return parser_blocking_script_.Get();
  }

  bool IsParserBlockingScriptReady();

  void PossiblyFetchBlockedDocWriteScript(PendingScript*);

  // Takes and returns the first PendingScript from |waiting_scripts| if it is
  // ready for execution. Otherwise, informs it that |this| is a
  // PendingScriptClient to be informed when it is ready.
  PendingScript* TryTakeReadyScriptWaitingForParsing(
      HeapDeque<Member<PendingScript>>* waiting_scripts);

  Member<HTMLParserReentryPermit> reentry_permit_;
  Member<Document> document_;
  Member<HTMLParserScriptRunnerHost> host_;

  // https://html.spec.whatwg.org/C/#pending-parsing-blocking-script
  Member<PendingScript> parser_blocking_script_;

  // Scripts that were deferred by the web developer. This is an ordered list.
  // https://html.spec.whatwg.org/C/#list-of-scripts-that-will-execute-when-the-document-has-finished-parsing
  HeapDeque<Member<PendingScript>> scripts_to_execute_after_parsing_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_HTML_PARSER_SCRIPT_RUNNER_H_
