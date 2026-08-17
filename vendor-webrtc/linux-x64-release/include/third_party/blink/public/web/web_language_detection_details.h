// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LANGUAGE_DETECTION_DETAILS_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LANGUAGE_DETECTION_DETAILS_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/platform/web_url.h"

namespace blink {

class WebDocument;

struct BLINK_EXPORT WebLanguageDetectionDetails {
  WebURL url;
  WebString content_language;
  WebString html_language;
  bool has_no_translate_meta = false;

  static WebLanguageDetectionDetails CollectLanguageDetectionDetails(
      const WebDocument&);

  // Use to record UMA metrics on the matches between the xml:lang value, html
  // lang value and the Accept-Language request header values.
  static void RecordAcceptLanguageAndXmlHtmlLangMetric(const WebDocument&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LANGUAGE_DETECTION_DETAILS_H_
