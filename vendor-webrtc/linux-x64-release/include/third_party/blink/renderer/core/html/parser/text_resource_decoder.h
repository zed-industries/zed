/*
    Copyright (C) 1999 Lars Knoll (knoll@mpi-hd.mpg.de)
    Copyright (C) 2006 Alexey Proskuryakov (ap@nypop.com)
    Copyright (C) 2006, 2008 Apple Inc. All rights reserved.

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Library General Public
    License as published by the Free Software Foundation; either
    version 2 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Library General Public License for more details.

    You should have received a copy of the GNU Library General Public License
    along with this library; see the file COPYING.LIB.  If not, write to
    the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
    Boston, MA 02110-1301, USA.

*/

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_TEXT_RESOURCE_DECODER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_TEXT_RESOURCE_DECODER_H_

#include <memory>

#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/loader/fetch/body_text_decoder.h"
#include "third_party/blink/renderer/platform/loader/fetch/text_resource_decoder_options.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/text/text_encoding.h"

namespace blink {

class HTMLMetaCharsetParser;

// Implements https://encoding.spec.whatwg.org/#decode or
// https://encoding.spec.whatwg.org/#utf-8-decode when an appropriate
// TextResourceDecoderOptions is given.
// See comments in text_resource_decoder_options.h.
//
// To construct a string from known-UTF-8 data without BOM, please use
// WTF::String::FromUTF8 instead.
// TODO(crbug.com/1373623): Move this to blink/renderer/platform and remove
// BodyTextDecoder.
class CORE_EXPORT TextResourceDecoder : public BodyTextDecoder {
  USING_FAST_MALLOC(TextResourceDecoder);

 public:
  enum EncodingSource {
    kDefaultEncoding,
    kAutoDetectedEncoding,
    kEncodingFromContentSniffing,
    kEncodingFromXMLHeader,
    kEncodingFromMetaTag,
    kEncodingFromCSSCharset,
    kEncodingFromHTTPHeader,
    kEncodingFromParentFrame
  };

  explicit TextResourceDecoder(const TextResourceDecoderOptions&);
  TextResourceDecoder(const TextResourceDecoder&) = delete;
  TextResourceDecoder& operator=(const TextResourceDecoder&) = delete;
  ~TextResourceDecoder() override;

  void SetEncoding(const WTF::TextEncoding&, EncodingSource);
  const WTF::TextEncoding& Encoding() const { return encoding_; }
  bool EncodingWasDetectedHeuristically() const {
    return source_ == kAutoDetectedEncoding ||
           source_ == kEncodingFromContentSniffing;
  }

  String Decode(base::span<const char> data,
                String* auto_detected_charset) override;
  String Decode(base::span<const char> data) { return Decode(data, nullptr); }
  String Decode(base::span<const uint8_t> data,
                String* auto_detected_charset = nullptr) {
    return Decode(base::as_chars(data), auto_detected_charset);
  }
  String Flush() override;
  WebEncodingData GetEncodingData() const override;

  bool SawError() const { return saw_error_; }
  wtf_size_t CheckForBOM(base::span<const char>);

 private:
  static const WTF::TextEncoding& DefaultEncoding(
      TextResourceDecoderOptions::ContentType,
      const WTF::TextEncoding& default_encoding);

  void AddToBuffer(base::span<const char> data);
  void AddToBufferIfEmpty(base::span<const char> data);
  bool CheckForCSSCharset(base::span<const char>);
  bool CheckForXMLCharset(base::span<const char>);
  void CheckForMetaCharset(base::span<const char>);
  void AutoDetectEncodingIfAllowed(base::span<const char> data,
                                   String* auto_detected_charset = nullptr);

  const TextResourceDecoderOptions options_;

  WTF::TextEncoding encoding_;
  std::unique_ptr<TextCodec> codec_;
  EncodingSource source_;
  Vector<char> buffer_;
  bool checked_for_bom_;
  bool checked_for_css_charset_;
  bool checked_for_xml_charset_;
  bool checked_for_meta_charset_;
  bool saw_error_;
  bool detection_completed_;

  std::unique_ptr<HTMLMetaCharsetParser> charset_parser_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_TEXT_RESOURCE_DECODER_H_
