// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_NETWORK_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_NETWORK_UTILS_H_

#include <string>
#include <tuple>

#include "services/metrics/public/cpp/ukm_recorder.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class KURL;
class ResourceResponse;

namespace network_utils {

enum PrivateRegistryFilter {
  kIncludePrivateRegistries,
  kExcludePrivateRegistries,
};

PLATFORM_EXPORT bool IsReservedIPAddress(const WTF::StringView& host);

PLATFORM_EXPORT WTF::String GetDomainAndRegistry(const WTF::StringView& host,
                                                 PrivateRegistryFilter);

// Returns the decoded data url as ResourceResponse and SharedBuffer if parsing
// was successful. The result is returned as net error code. It returns net::OK
// if decoding succeeds, otherwise it failed.
PLATFORM_EXPORT std::tuple<int, ResourceResponse, scoped_refptr<SharedBuffer>>
ParseDataURL(const KURL&,
             const WTF::String& method,
             ukm::SourceId source_id = ukm::kInvalidSourceId,
             ukm::UkmRecorder* recorder = nullptr);

// Returns true if the URL is a data URL and its MIME type is in the list of
// supported/recognized MIME types.
PLATFORM_EXPORT bool IsDataURLMimeTypeSupported(
    const KURL&,
    std::string* data = nullptr,
    std::string* mime_type = nullptr);

PLATFORM_EXPORT bool IsRedirectResponseCode(int);

PLATFORM_EXPORT bool IsCertificateTransparencyRequiredError(int);

PLATFORM_EXPORT WTF::String ExpandLanguageList(const WTF::String&);

PLATFORM_EXPORT WTF::String GenerateAcceptLanguageHeader(const WTF::String&);

PLATFORM_EXPORT Vector<char> ParseMultipartBoundary(
    const AtomicString& content_type_header);

}  // namespace network_utils

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_NETWORK_UTILS_H_
