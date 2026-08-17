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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_DOCUMENT_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_DOCUMENT_PARSER_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/rand_util.h"
#include "base/task/sequenced_task_runner.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/common/features.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/parser_content_policy.h"
#include "third_party/blink/renderer/core/dom/scriptable_document_parser.h"
#include "third_party/blink/renderer/core/html/nesting_level_incrementer.h"
#include "third_party/blink/renderer/core/html/parser/html_document_parser_state.h"
#include "third_party/blink/renderer/core/html/parser/html_input_stream.h"
#include "third_party/blink/renderer/core/html/parser/html_parser_options.h"
#include "third_party/blink/renderer/core/html/parser/html_parser_reentry_permit.h"
#include "third_party/blink/renderer/core/html/parser/html_preload_scanner.h"
#include "third_party/blink/renderer/core/html/parser/html_tokenizer.h"
#include "third_party/blink/renderer/core/html/parser/html_tree_builder.h"
#include "third_party/blink/renderer/core/html/parser/parser_synchronization_policy.h"
#include "third_party/blink/renderer/core/html/parser/preload_request.h"
#include "third_party/blink/renderer/core/html/parser/text_resource_decoder.h"
#include "third_party/blink/renderer/core/page/viewport_description.h"
#include "third_party/blink/renderer/core/script/html_parser_script_runner_host.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_handle.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/sequence_bound.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"

namespace blink {

class AtomicHTMLToken;
class BackgroundHTMLScanner;
class Document;
class DocumentFragment;
class Element;
class HTMLDocument;
class HTMLParserMetrics;
class HTMLParserScriptRunner;
class HTMLPreloadScanner;
class HTMLResourcePreloader;
class HTMLTreeBuilder;
class HTMLDocumentParserState;

enum ParserPrefetchPolicy {
  // Indicates that prefetches/preloads should happen for this document type.
  kAllowPrefetching,
  // Indicates that prefetches are forbidden for this document type.
  kDisallowPrefetching
};

// TODO(https://crbug.com/1049898): These are only exposed to make it possible
// to delete an expired histogram. The test should be rewritten to test at a
// different level, so it won't have to make assertions about internal state.
void CORE_EXPORT ResetDiscardedTokenCountForTesting();
size_t CORE_EXPORT GetDiscardedTokenCountForTesting();

class CORE_EXPORT HTMLDocumentParser : public ScriptableDocumentParser,
                                       private HTMLParserScriptRunnerHost {
 public:
  HTMLDocumentParser(HTMLDocument&,
                     ParserSynchronizationPolicy,
                     ParserPrefetchPolicy prefetch_policy = kAllowPrefetching);
  HTMLDocumentParser(DocumentFragment*,
                     Element* context_element,
                     ParserContentPolicy,
                     ParserPrefetchPolicy prefetch_policy = kAllowPrefetching);
  ~HTMLDocumentParser() override;
  void Trace(Visitor*) const override;

  static void ParseDocumentFragment(
      const String&,
      DocumentFragment*,
      Element* context_element,
      ParserContentPolicy = kAllowScriptingContent);

  // Exposed for testing.
  HTMLParserScriptRunnerHost* AsHTMLParserScriptRunnerHostForTesting() {
    return this;
  }
  // Returns true if any tokenizer pumps / end if delayed work is scheduled.
  // Exposed so that tests can check that the parser's exited in a good state.
  bool HasPendingWorkScheduledForTesting() const;

  HTMLTokenizer& tokenizer() { return tokenizer_; }

  bool DidPumpTokenizerForTesting() const { return did_pump_tokenizer_; }

  unsigned GetChunkCountForTesting() const;

  TextPosition GetTextPosition() const final;
  OrdinalNumber LineNumber() const final;

  HTMLParserReentryPermit* ReentryPermit() { return reentry_permit_.Get(); }

  void AppendBytes(base::span<const uint8_t> bytes) override;
  void Flush() final;
  void SetDecoder(std::unique_ptr<TextResourceDecoder>) final;

  static void ResetCachedFeaturesForTesting();
  static void FlushPreloadScannerThreadForTesting();

  bool HasPendingPreloads();

 protected:
  void insert(const String&) final;
  void Append(const String&) override;
  void Finish() final;

  HTMLTreeBuilder* TreeBuilder() const { return tree_builder_.Get(); }

  void ForcePlaintextForTextDocument();

 private:
  enum NextTokenStatus { kNoTokens, kHaveTokens, kHaveTokensAfterScript };
  class PendingPreloads;

  HTMLDocumentParser(Document&,
                     ParserContentPolicy,
                     ParserSynchronizationPolicy,
                     ParserPrefetchPolicy);

  // DocumentParser
  void Detach() final;
  bool HasInsertionPoint() final;
  void PrepareToStopParsing() final;
  void StopParsing() final;
  ALWAYS_INLINE bool IsPaused() const {
    return IsWaitingForScripts() || task_runner_state_->WaitingForStylesheets();
  }
  bool IsWaitingForScripts() const final;
  bool IsExecutingScript() const final;
  void ExecuteScriptsWaitingForResources() final;
  void DidAddPendingParserBlockingStylesheet() final;
  void DidLoadAllPendingParserBlockingStylesheets() final;
  void CheckIfBlockingStylesheetAdded();
  void DocumentElementAvailable() override;
  void CommitPreloadedData() override;
  void FlushPendingPreloads() override;
  BackgroundScanCallback TakeBackgroundScanCallback() override;

  // HTMLParserScriptRunnerHost
  void NotifyScriptLoaded() final;
  HTMLInputStream& InputStream() final { return input_; }
  bool HasPreloadScanner() const final {
    return preload_scanner_.get() || background_scanner_;
  }
  void AppendCurrentInputStreamToPreloadScannerAndScan() final;

  // This function may end up running script. If it does,
  // `time_executing_script` is incremented by the amount of time it takes to
  // execute script.
  ALWAYS_INLINE NextTokenStatus
  CanTakeNextToken(base::TimeDelta& time_executing_script) {
    if (IsStopped())
      return kNoTokens;

    if (!tree_builder_->HasParserBlockingScript())
      return IsPaused() ? kNoTokens : kHaveTokens;

    // If we're paused waiting for a script, we try to execute scripts before
    // continuing.
    {
      base::ElapsedTimer timer;
      RunScriptsForPausedTreeBuilder();
      time_executing_script += timer.Elapsed();
    }
    return (IsStopped() || IsPaused()) ? kNoTokens : kHaveTokensAfterScript;
  }
  bool PumpTokenizer();
  void PumpTokenizerIfPossible();
  void DeferredPumpTokenizerIfPossible(bool from_finish_append,
                                       base::TimeTicks schedule_time);
  void SchedulePumpTokenizer(bool from_finish_append);
  void ScheduleEndIfDelayed();
  void ConstructTreeFromToken(AtomicHTMLToken& atomic_token);

  void RunScriptsForPausedTreeBuilder();
  void ResumeParsingAfterPause();

  // AttemptToEnd stops document parsing if nothing's currently delaying the end
  // of parsing.
  void AttemptToEnd();
  // EndIfDelayed stops document parsing if AttemptToEnd was previously delayed,
  // or if there are no scripts/resources/nested pumps delaying the end of
  // parsing.
  void EndIfDelayed();
  void AttemptToRunDeferredScriptsAndEnd();
  void end();

  bool IsParsingFragment() const;
  // ShouldDelayEnd assesses whether any resources, scripts or nested pumps are
  // delaying the end of parsing.
  bool ShouldDelayEnd() const;

  std::unique_ptr<HTMLPreloadScanner> CreatePreloadScanner(
      TokenPreloadScanner::ScannerType);

  // Let the given HTMLPreloadScanner scan the input it has, and then preload
  // resources using the resulting PreloadRequests and |preloader_|.
  void ScanAndPreload(HTMLPreloadScanner*);
  void ProcessPreloadData(std::unique_ptr<PendingPreloadData> preload_data);
  void MaybeFetchQueuedPreloads();
  std::string GetPreloadHistogramSuffix();
  void FinishAppend();
  void ScanInBackground(const String& source);

  // Called on the background thread by |background_scanner_|.
  static void AddPreloadDataOnBackgroundThread(
      CrossThreadWeakHandle<HTMLDocumentParser> parser_handle,
      scoped_refptr<PendingPreloads> pending_preloads,
      scoped_refptr<base::SequencedTaskRunner> task_runner,
      std::unique_ptr<PendingPreloadData> preload_data);

  // Returns true if the data should be processed (tokenizer pumped) now. If
  // this returns false, SchedulePumpTokenizer() should be called. This is
  // called when data is available.
  bool ShouldPumpTokenizerNowForFinishAppend() const;

  // Returns true if we should check the clock after parsing a token.
  // We check the clock after parsing a token that's likely slow, or
  // for 1 out of 10 fast tokens.
  bool ShouldCheckTimeBudget(NextTokenStatus next_token_status,
                             html_names::HTMLTag tag,
                             int newly_consumed_characters,
                             int tokens_parsed) const;

  bool ShouldSkipPreloadScan();

  // Check if preloads are allowed considering the presence of a preloader,
  // the presence of queued preloads and the presence of meta CSP tags in the
  // HTML document.
  bool AllowPreloading();

  HTMLInputStream input_;
  const HTMLParserOptions options_;
  Member<HTMLParserReentryPermit> reentry_permit_ =
      MakeGarbageCollected<HTMLParserReentryPermit>();
  HTMLTokenizer tokenizer_;

  Member<HTMLParserScriptRunner> script_runner_;
  Member<HTMLTreeBuilder> tree_builder_;

  std::unique_ptr<HTMLPreloadScanner> preload_scanner_;
  // A scanner used only for input provided to the insert() method.
  std::unique_ptr<HTMLPreloadScanner> insertion_preload_scanner_;
  WTF::SequenceBound<BackgroundHTMLScanner> background_script_scanner_;
  HTMLPreloadScanner::BackgroundPtr background_scanner_;
  using BackgroundScanFn =
      WTF::CrossThreadRepeatingFunction<void(const KURL&, const String&)>;
  BackgroundScanFn background_scan_fn_;

  scoped_refptr<base::SingleThreadTaskRunner> loading_task_runner_;

  Member<HTMLResourcePreloader> preloader_;
  Member<HTMLDocumentParserState> task_runner_state_;
  PreloadRequestStream queued_preloads_;

  // Metrics gathering and reporting
  std::unique_ptr<HTMLParserMetrics> metrics_reporter_;
  // A timer for how long we are inactive after yielding
  std::unique_ptr<base::ElapsedTimer> yield_timer_;

  // If ThreadedPreloadScanner is enabled, preload data will be added to
  // `pending_preloads_` from a background thread. The main thread will
  // take this preload data and send out the requests.
  scoped_refptr<PendingPreloads> pending_preloads_;

  ThreadScheduler* scheduler_;

  // Set to true if PumpTokenizer() was called at least once.
  bool did_pump_tokenizer_ = false;

  // Cached result of ShouldSkipPreloadScan()
  bool should_skip_preload_scan_ = false;

  // Counts how many CSP meta tags have been seen (but not necessarily processed
  // yet). This is used to compare the number of seen tags with the number of
  // processed CSP tags in order to decide if resources can be preloaded.
  int seen_csp_meta_tags_ = 0;

  base::MetricsSubSampler metrics_sub_sampler_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_DOCUMENT_PARSER_H_
