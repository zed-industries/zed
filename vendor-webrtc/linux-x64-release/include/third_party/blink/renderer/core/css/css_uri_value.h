// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_URI_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_URI_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_url_data.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace WTF {
class TextEncoding;
}  // namespace WTF

namespace blink {

class Document;
class KURL;
class SVGResource;

namespace cssvalue {

class CORE_EXPORT CSSURIValue : public CSSValue {
 public:
  explicit CSSURIValue(CSSUrlData url_data);
  ~CSSURIValue();

  SVGResource* EnsureResourceReference() const;
  void ReResolveUrl(const Document&) const;

  const AtomicString& ValueForSerialization() const {
    return url_data_.ValueForSerialization();
  }

  String CustomCSSText() const;

  const CSSUrlData& UrlData() const { return url_data_; }
  bool IsLocal(const Document&) const;
  AtomicString FragmentIdentifier() const;

  // Fragment identifier with trailing spaces removed and URL
  // escape sequences decoded. This is cached, because it can take
  // a surprisingly long time to normalize the URL into an absolute
  // value if we have lots of SVG elements that need to re-run this
  // over and over again.
  const AtomicString& NormalizedFragmentIdentifier() const;

  bool Equals(const CSSURIValue&) const;

  CSSURIValue* ComputedCSSValue(const KURL& base_url,
                                const WTF::TextEncoding&) const;

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  KURL AbsoluteUrl() const;

  CSSUrlData url_data_;

  mutable AtomicString normalized_fragment_identifier_cache_;
  mutable Member<SVGResource> resource_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSURIValue> {
  static bool AllowFrom(const CSSValue& value) { return value.IsURIValue(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_URI_VALUE_H_
