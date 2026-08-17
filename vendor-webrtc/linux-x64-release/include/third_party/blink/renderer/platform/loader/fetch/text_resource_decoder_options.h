// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_TEXT_RESOURCE_DECODER_OPTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_TEXT_RESOURCE_DECODER_OPTIONS_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/text_encoding.h"

namespace blink {

class PLATFORM_EXPORT TextResourceDecoderOptions final {
  DISALLOW_NEW();

 public:
  enum ContentType {
    kPlainTextContent,
    kHTMLContent,
    kJSONContent,
    kXMLContent,
    kCSSContent,
    kMaxContentType = kCSSContent
  };  // PlainText only checks for BOM.

  // Implements https://encoding.spec.whatwg.org/#decode
  // when ContentType is |kPlainTextContent|.
  // The "fallback encoding" is
  // - If TextResourceDecoder::SetEncoding(|encoding|) is called and
  //   |encoding.IsValid()| is true, then |encoding|.
  // - Else if |default_encoding.IsValid()| is true, then |default_encoding|.
  // - Else, Latin-1.
  explicit TextResourceDecoderOptions(
      ContentType,
      const WTF::TextEncoding& default_encoding = WTF::TextEncoding());

  // Corresponds to utf-8 decode in Encoding spec:
  // https://encoding.spec.whatwg.org/#utf-8-decode.
  static TextResourceDecoderOptions CreateUTF8Decode();

  // Corresponds to utf-8 decode without BOM in Encoding spec:
  // https://encoding.spec.whatwg.org/#utf-8-decode-without-bom.
  static TextResourceDecoderOptions CreateUTF8DecodeWithoutBOM();

  static TextResourceDecoderOptions CreateWithAutoDetection(
      ContentType,
      const WTF::TextEncoding& default_encoding,
      const WTF::TextEncoding& hint_encoding,
      const KURL& hint_url);

  void SetUseLenientXMLDecoding() { use_lenient_xml_decoding_ = true; }
  void OverrideContentType(ContentType content_type) {
    if (encoding_detection_option_ != kAlwaysUseUTF8ForText)
      content_type_ = content_type;
  }

  // TextResourceDecoder does three kind of encoding detection:
  // 1. By BOM,
  // 2. By Content if |content_type_| is not |kPlainTextContext|
  //    (e.g. <meta> tag for HTML), and
  // 3. By DetectTextEncoding().
  enum EncodingDetectionOption {
    // Use 1. + 2. + 3.
    kUseAllAutoDetection,

    // Use 1. + 2.
    kUseContentAndBOMBasedDetection,

    // Use None of them.
    // |content_type_| must be |kPlainTextContent| and
    // |default_encoding_| must be UTF8Encoding.
    // This doesn't change encoding based on BOMs, but still processes
    // utf-8 BOMs so that utf-8 BOMs don't appear in the decoded result.
    kAlwaysUseUTF8ForText
  };

  EncodingDetectionOption GetEncodingDetectionOption() const {
    return encoding_detection_option_;
  }
  ContentType GetContentType() const { return content_type_; }
  const WTF::TextEncoding& DefaultEncoding() const { return default_encoding_; }
  bool GetNoBOMDecoding() const { return no_bom_decoding_; }
  bool GetUseLenientXMLDecoding() const { return use_lenient_xml_decoding_; }

  const AtomicString& HintEncoding() const { return hint_encoding_; }
  const KURL& HintURL() const { return hint_url_; }
  const char* HintLanguage() const { return hint_language_.data(); }

 private:
  TextResourceDecoderOptions(EncodingDetectionOption,
                             ContentType,
                             const WTF::TextEncoding& default_encoding,
                             const AtomicString& hint_encoding,
                             const KURL& hint_url);

  EncodingDetectionOption encoding_detection_option_;
  ContentType content_type_;
  WTF::TextEncoding default_encoding_;
  bool no_bom_decoding_;
  bool use_lenient_xml_decoding_;  // Don't stop on XML decoding errors.

  // Hints for DetectTextEncoding().
  // Only used when |encoding_detection_option_| == |kUseAllAutoDetection|.
  AtomicString hint_encoding_;
  KURL hint_url_;
  std::array<char, 3> hint_language_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_TEXT_RESOURCE_DECODER_OPTIONS_H_
