// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_BACKGROUND_HTML_SCANNER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_BACKGROUND_HTML_SCANNER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_streamer.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/text/segmented_string.h"
#include "third_party/blink/renderer/platform/wtf/sequence_bound.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"

namespace blink {

class HTMLParserOptions;
class HTMLToken;
class HTMLTokenizer;
class ScriptableDocumentParser;

// Scans HTML on a worker thread looking for inline scripts which can be stream
// compiled.
//
// Unlike HTMLPreloadScanner which runs on the main thread, this scanner runs in
// a worker thread. In addition, HTMLPreloadScanner only scans the first chunk
// of body data and won't scan more of the response body unless the parser is
// paused. BackgroundHTMLScanner will scan all body data, since it does not tie
// up the main thread with the scan. If you want to add logic for scanning the
// response body which can be done off the main thread, this is the place to add
// it.
//
// This is currently used to stream compile inline scripts. To do this
// effectively we need to get the script text as early as possible so it can be
// compiled by the time it is needed in the parser. BackgroundHTMLScanner will
// immediately scan the HTML until it finds an inline script, then kick off a
// stream compile task for that script. The parser will store these compile
// tasks in a map which will then be checked and used whenever a matching inline
// script is parsed.
// TODO(crbug.com/1329535): Move HTMLPreloadScanner to a background thread and
// combine these.
class CORE_EXPORT BackgroundHTMLScanner {
  USING_FAST_MALLOC(BackgroundHTMLScanner);

 public:
  // Class for scanning individual tokens and starting script compile jobs.
  class CORE_EXPORT ScriptTokenScanner {
    USING_FAST_MALLOC(BackgroundHTMLScanner);

   public:
    static std::unique_ptr<ScriptTokenScanner> Create(
        ScriptableDocumentParser* parser);

    ScriptTokenScanner(ScriptableDocumentParser* parser,
                       scoped_refptr<base::SequencedTaskRunner> task_runner,
                       wtf_size_t min_script_size);

    void ScanToken(const HTMLToken& token);

    void set_first_script_in_scan(bool value) { first_script_in_scan_ = value; }

   private:
    // Careful this isolate doesn't belong to the sequence that this class
    // executes on.
    v8::Isolate* isolate_;
    CrossThreadWeakPersistent<ScriptableDocumentParser> parser_;
    scoped_refptr<base::SequencedTaskRunner> task_runner_;
    wtf_size_t min_script_size_;
    StringBuilder script_builder_;

    bool in_script_ = false;
    bool first_script_in_scan_ = false;
  };

  // Creates a sequence bound BackgroundHTMLScanner which will live on a
  // background thread. Methods can be called using SequenceBound::AsyncCall().
  static WTF::SequenceBound<BackgroundHTMLScanner> Create(
      const HTMLParserOptions& options,
      ScriptableDocumentParser* parser);
  BackgroundHTMLScanner(std::unique_ptr<HTMLTokenizer> tokenizer,
                        std::unique_ptr<ScriptTokenScanner> token_scanner);
  ~BackgroundHTMLScanner();

  BackgroundHTMLScanner(const BackgroundHTMLScanner&) = delete;
  BackgroundHTMLScanner& operator=(const BackgroundHTMLScanner&) = delete;

  void Scan(const String& source);

 private:
  SegmentedString source_;
  std::unique_ptr<HTMLTokenizer> tokenizer_;
  std::unique_ptr<ScriptTokenScanner> token_scanner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_BACKGROUND_HTML_SCANNER_H_
