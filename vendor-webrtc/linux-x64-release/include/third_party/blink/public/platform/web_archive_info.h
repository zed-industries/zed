// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ARCHIVE_INFO_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ARCHIVE_INFO_H_

#include "base/time/time.h"
#include "third_party/blink/public/mojom/loader/mhtml_load_result.mojom-shared.h"
#include "third_party/blink/public/platform/web_url.h"

namespace blink {

// Contains basic attributes of an archive resource derived from parsed headers
// and content contained within it. Fields beyond |load_result| are only set
// when |load_result|'s value is MHTMLLoadResult::kSuccess.
struct WebArchiveInfo {
  // MHTML archive load result.
  blink::mojom::MHTMLLoadResult load_result;

  // Main resource URL, the parser chooses the first appropriate resource from
  // within the MTHML file.  This is the Content-Location header from that
  // resource. If |load_result| != kSuccess, |url| is empty.
  WebURL url;

  // Date as reported in the Date: header from the MHTML header section. If
  // |load_result| != kSuccess, |date| is zero.
  base::Time date;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ARCHIVE_INFO_H_
