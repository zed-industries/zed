/*
 * (C) 1999-2003 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2008, 2012 Apple Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_URL_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_URL_DATA_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_origin_clean.h"
#include "third_party/blink/renderer/platform/weborigin/referrer.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace WTF {
class TextEncoding;
}  // namespace WTF

namespace blink {

class Document;
class KURL;

// Stores data for a <url> value (url(), src()).
class CORE_EXPORT CSSUrlData {
 public:
  CSSUrlData(const AtomicString& unresolved_url,
             const KURL& resolved_url,
             const Referrer&,
             OriginClean,
             bool is_ad_related);

  // Create URL data with a resolved (absolute) URL. Generally used for
  // computed values - the above should otherwise be preferred.
  explicit CSSUrlData(const AtomicString& resolved_url);

  // Returns the resolved URL, potentially reresolving against passed Document
  // if there's a potential risk of "dangling markup".
  KURL ResolveUrl(const Document&) const;

  // Re-resolve the URL against the base provided by the passed
  // Document. Returns true if the resolved URL changed, otherwise false.
  bool ReResolveUrl(const Document&) const;

  // Returns an absolutized copy of this URL data (suitable for computed value).
  CSSUrlData MakeAbsolute() const;

  // Returns a copy where the unresolved URL has been resolved against
  // `base_url` (using `charset` encoding if valid).
  CSSUrlData MakeResolved(const KURL& base_url,
                          const WTF::TextEncoding& charset) const;

  // Returns a copy where the referrer has been reset.
  CSSUrlData MakeWithoutReferrer() const;

  const AtomicString& ValueForSerialization() const {
    return is_local_ || absolute_url_.empty() ? relative_url_ : absolute_url_;
  }

  const AtomicString& UnresolvedUrl() const { return relative_url_; }
  const AtomicString& ResolvedUrl() const { return absolute_url_; }

  const Referrer& GetReferrer() const { return referrer_; }

  bool IsFromOriginCleanStyleSheet() const {
    return is_from_origin_clean_style_sheet_;
  }
  OriginClean GetOriginClean() const {
    return is_from_origin_clean_style_sheet_ ? OriginClean::kTrue
                                             : OriginClean::kFalse;
  }
  bool IsAdRelated() const { return is_ad_related_; }

  // Returns true if this URL is "local" to the specified Document (either by
  // being a fragment-only URL or by matching the document URL).
  bool IsLocal(const Document&) const;

  String CssText() const;

  bool operator==(const CSSUrlData& other) const;

 private:
  AtomicString relative_url_;
  mutable AtomicString absolute_url_;
  const Referrer referrer_;

  // Whether the stylesheet that requested this image is origin-clean:
  // https://drafts.csswg.org/cssom-1/#concept-css-style-sheet-origin-clean-flag
  const bool is_from_origin_clean_style_sheet_;

  // Whether this was created by an ad-related CSSParserContext.
  const bool is_ad_related_;

  const bool is_local_;

  // The url passed into the constructor had the PotentiallyDanglingMarkup flag
  // set. That information needs to be passed on to the fetch code to block such
  // resources from loading.
  const bool potentially_dangling_markup_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_URL_DATA_H_
