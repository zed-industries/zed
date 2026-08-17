/*
 * Copyright (C) 2008 Apple Inc. All Rights Reserved.
 * Copyright (C) 2010 Google Inc. All Rights Reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_PRELOAD_SCANNER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_PRELOAD_SCANNER_H_

#include <memory>
#include <optional>
#include <utility>

#include "base/memory/ptr_util.h"
#include "base/memory/weak_ptr.h"
#include "base/task/sequenced_task_runner.h"
#include "services/network/public/cpp/client_hints.h"
#include "third_party/blink/public/common/features.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/media_values_cached.h"
#include "third_party/blink/renderer/core/frame/local_frame.h"
#include "third_party/blink/renderer/core/html/parser/background_html_scanner.h"
#include "third_party/blink/renderer/core/html/parser/css_preload_scanner.h"
#include "third_party/blink/renderer/core/html/parser/html_token.h"
#include "third_party/blink/renderer/core/html/parser/preload_request.h"
#include "third_party/blink/renderer/core/lcp_critical_path_predictor/element_locator.h"
#include "third_party/blink/renderer/core/page/viewport_description.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/text/segmented_string.h"
#include "third_party/blink/renderer/platform/wtf/sequence_bound.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
namespace blink {

class HTMLDocumentParser;
class HTMLParserOptions;
class HTMLTokenizer;
class SegmentedString;

// Encapsulates values from the <meta http-equiv="accept-ch"> or
// <meta http-equiv="delegate-ch"> tags. These are collected by the preload
// scanner to be later handled on the main thread.
struct MetaCHValue {
  AtomicString value;
  network::MetaCHType type = network::MetaCHType::HttpEquivAcceptCH;
  // ClientHintsPreferences::UpdateFromMetaCH needs to know if the document
  // preloader was used as otherwise the value may be discarded.
  bool is_doc_preloader = false;
};
using MetaCHValues = Vector<MetaCHValue>;

// Encapsulates data created by HTMLPreloadScanner that needs to be processed on
// the main thread.
struct PendingPreloadData {
  MetaCHValues meta_ch_values;
  std::optional<ViewportDescription> viewport;
  int csp_meta_tag_count = 0;
  bool has_located_potential_lcp_element = false;
  PreloadRequestStream requests;
};

bool Match(const StringImpl* impl, const QualifiedName& q_name);
const StringImpl* TagImplFor(const HTMLToken::DataVector& data);

struct CORE_EXPORT CachedDocumentParameters {
  USING_FAST_MALLOC(CachedDocumentParameters);

 public:
  explicit CachedDocumentParameters(Document*);
  CachedDocumentParameters() = default;
  static void SetLcppPreloadLazyLoadImageTypeForTesting(
      std::optional<features::LcppPreloadLazyLoadImageType> type);

  bool do_html_preload_scanning;
  Length default_viewport_min_width;
  bool viewport_meta_zero_values_quirk;
  bool viewport_meta_enabled;
  network::mojom::ReferrerPolicy referrer_policy;
  LocalFrame::LazyLoadImageSetting lazy_load_image_setting;
  // Work with the element locators. If the LCP candidate image is found and
  // that has a lazy loading indicator, ignore it and create preload request.
  // This will override |lazy_load_image_setting| behavior.
  features::LcppPreloadLazyLoadImageType preload_lazy_load_image_type;
  static std::optional<features::LcppPreloadLazyLoadImageType>
      preload_lazy_load_image_type_for_testing;
  HashSet<String> disabled_image_types;
};

class TokenPreloadScanner {
  USING_FAST_MALLOC(TokenPreloadScanner);

 public:
  enum class ScannerType { kMainDocument, kInsertion };

  TokenPreloadScanner(const KURL& document_url,
                      std::unique_ptr<CachedDocumentParameters>,
                      std::unique_ptr<MediaValuesCached::MediaValuesCachedData>,
                      const ScannerType,
                      Vector<ElementLocator>);
  TokenPreloadScanner(const TokenPreloadScanner&) = delete;
  TokenPreloadScanner& operator=(const TokenPreloadScanner&) = delete;
  ~TokenPreloadScanner();

  void Scan(const HTMLToken&,
            const SegmentedString&,
            PreloadRequestStream& requests,
            MetaCHValues& meta_ch_values,
            std::optional<ViewportDescription>*,
            int* csp_meta_tag_counter);

  void SetPredictedBaseElementURL(const KURL& url) {
    predicted_base_element_url_ = url;
  }

  bool HasLocatedPotentialLcpElement() { return seen_potential_lcp_element_; }

 private:
  class StartTagScanner;

  void HandleMetaNameAttribute(const HTMLToken& token,
                               MetaCHValues& meta_ch_values,
                               std::optional<ViewportDescription>* viewport);

  inline void ScanCommon(const HTMLToken&,
                         const SegmentedString&,
                         PreloadRequestStream& requests,
                         MetaCHValues& meta_ch_values,
                         std::optional<ViewportDescription>*,
                         bool* is_csp_meta_tag);

  void UpdatePredictedBaseURL(const HTMLToken&);

  MediaValuesCached* EnsureMediaValues() {
    if (!media_values_) {
      media_values_ =
          MakeGarbageCollected<MediaValuesCached>(*media_values_cached_data_);
    }
    return media_values_.Get();
  }

  struct PictureData {
    PictureData() : source_size(0.0), source_size_set(false), picked(false) {}
    String source_url;
    float source_size;
    bool source_size_set;
    bool picked;
  };

  CSSPreloadScanner css_scanner_;
  const KURL document_url_;
  KURL predicted_base_element_url_;
  scoped_refptr<const PreloadRequest::ExclusionInfo> exclusion_info_;
  bool in_style_;
  bool in_picture_;
  bool in_script_;
  bool in_script_web_bundle_;
  bool seen_body_;
  bool seen_img_;
  bool seen_potential_lcp_element_ = false;
  PictureData picture_data_;
  size_t template_count_;
  std::unique_ptr<CachedDocumentParameters> document_parameters_;
  std::unique_ptr<MediaValuesCached::MediaValuesCachedData>
      media_values_cached_data_;
  Persistent<MediaValuesCached> media_values_;
  ScannerType scanner_type_;
  element_locator::TokenStreamMatcher lcp_element_matcher_;
};

class CORE_EXPORT HTMLPreloadScanner final {
  USING_FAST_MALLOC(HTMLPreloadScanner);

 public:
  // Creates a HTMLPreloadScanner which can be used on the main thread.
  static std::unique_ptr<HTMLPreloadScanner> Create(
      Document& document,
      HTMLParserOptions options,
      TokenPreloadScanner::ScannerType scanner_type);

  using TakePreloadFn = WTF::CrossThreadRepeatingFunction<void(
      std::unique_ptr<PendingPreloadData>)>;

  // Creates a HTMLPreloadScanner which will be bound to |task_runner|.
  struct Deleter {
    void operator()(const HTMLPreloadScanner* ptr) {
      if (ptr)
        task_runner_->DeleteSoon(FROM_HERE, ptr);
    }
    scoped_refptr<base::SequencedTaskRunner> task_runner_;
  };
  using BackgroundPtr = std::unique_ptr<HTMLPreloadScanner, Deleter>;
  static BackgroundPtr CreateBackground(
      HTMLDocumentParser* parser,
      HTMLParserOptions options,
      scoped_refptr<base::SequencedTaskRunner> task_runner,
      TakePreloadFn take_preload);

  HTMLPreloadScanner(std::unique_ptr<HTMLTokenizer>,
                     const KURL& document_url,
                     std::unique_ptr<CachedDocumentParameters>,
                     std::unique_ptr<MediaValuesCached::MediaValuesCachedData>,
                     const TokenPreloadScanner::ScannerType,
                     std::unique_ptr<BackgroundHTMLScanner::ScriptTokenScanner>
                         script_token_scanner,
                     TakePreloadFn take_preload = TakePreloadFn(),
                     Vector<ElementLocator> locators = {},
                     bool disable_preload_scanning = false);
  HTMLPreloadScanner(const HTMLPreloadScanner&) = delete;
  HTMLPreloadScanner& operator=(const HTMLPreloadScanner&) = delete;
  ~HTMLPreloadScanner();

  void AppendToEnd(const SegmentedString&);

  std::unique_ptr<PendingPreloadData> Scan(
      const KURL& document_base_element_url);

  // Scans |source| and calls |take_preload| with the generated preload data.
  void ScanInBackground(const String& source,
                        const KURL& document_base_element_url);

  static bool IsSkipPreloadScanEnabled(const Document* document);

  base::WeakPtr<HTMLPreloadScanner> AsWeakPtr() {
    return weak_ptr_factory_.GetWeakPtr();
  }

 private:
  TokenPreloadScanner scanner_;
  SegmentedString source_;
  std::unique_ptr<HTMLTokenizer> tokenizer_;
  std::unique_ptr<BackgroundHTMLScanner::ScriptTokenScanner>
      script_token_scanner_;
  TakePreloadFn take_preload_;
  bool skip_preload_scanning_;
  base::WeakPtrFactory<HTMLPreloadScanner> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_PRELOAD_SCANNER_H_
