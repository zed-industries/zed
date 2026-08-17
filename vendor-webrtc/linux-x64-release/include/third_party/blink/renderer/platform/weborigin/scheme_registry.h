/*
 * Copyright (C) 2010 Apple Inc. All Rights Reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_SCHEME_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_SCHEME_REGISTRY_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

using URLSchemesSet = HashSet<String>;

template <typename Mapped, typename MappedTraits>
using URLSchemesMap = HashMap<String, Mapped, HashTraits<String>, MappedTraits>;

class PLATFORM_EXPORT SchemeRegistry {
  STATIC_ONLY(SchemeRegistry);

 public:
  static bool ShouldTreatURLSchemeAsRestrictingMixedContent(const String&);

  // Display-isolated schemes can only be displayed (in the sense of
  // SecurityOrigin::canDisplay) by documents from the same scheme.
  static void RegisterURLSchemeAsDisplayIsolated(const String&);
  static bool ShouldTreatURLSchemeAsDisplayIsolated(const String&);

  static bool ShouldLoadURLSchemeAsEmptyDocument(const String&);

  static void SetDomainRelaxationForbiddenForURLSchemeForTest(bool forbidden,
                                                              const String&);
  static void ResetDomainRelaxationForTest();
  static bool IsDomainRelaxationForbiddenForURLScheme(const String&);

  // Such schemes should delegate to SecurityOrigin::canRequest for any URL
  // passed to SecurityOrigin::canDisplay.
  static bool CanDisplayOnlyIfCanRequest(const String& scheme);

  // Schemes against which javascript: URLs should not be allowed to run (stop
  // bookmarklets from running on sensitive pages).
  static void RegisterURLSchemeAsNotAllowingJavascriptURLs(
      const String& scheme);
  static void RemoveURLSchemeAsNotAllowingJavascriptURLs(const String& scheme);
  static bool ShouldTreatURLSchemeAsNotAllowingJavascriptURLs(
      const String& scheme);

  static bool ShouldTreatURLSchemeAsCorsEnabled(const String& scheme);

  // Serialize the registered schemes in a comma-separated list.
  static String ListOfCorsEnabledURLSchemes();

  // Does the scheme represent a location relevant to web compatibility metrics?
  static bool ShouldTrackUsageMetricsForScheme(const String& scheme);

  // Schemes that can register a service worker.
  static void RegisterURLSchemeAsAllowingServiceWorkers(const String& scheme);
  static bool ShouldTreatURLSchemeAsAllowingServiceWorkers(
      const String& scheme);

  // HTTP-like schemes that are treated as supporting the Fetch API.
  static void RegisterURLSchemeAsSupportingFetchAPI(const String& scheme);
  static bool ShouldTreatURLSchemeAsSupportingFetchAPI(const String& scheme);

  // https://url.spec.whatwg.org/#special-scheme
  static bool IsSpecialScheme(const String& scheme);

  // Schemes which override the first-/third-party checks on a Document.
  static void RegisterURLSchemeAsFirstPartyWhenTopLevel(const String& scheme);
  static void RemoveURLSchemeAsFirstPartyWhenTopLevel(const String& scheme);
  static bool ShouldTreatURLSchemeAsFirstPartyWhenTopLevel(
      const String& scheme);

  // Like RegisterURLSchemeAsFirstPartyWhenTopLevel, but requires the present
  // document to be delivered over a secure scheme.
  static void RegisterURLSchemeAsFirstPartyWhenTopLevelEmbeddingSecure(
      const String& scheme);
  static bool ShouldTreatURLSchemeAsFirstPartyWhenTopLevelEmbeddingSecure(
      const String& top_level_scheme,
      const String& child_scheme);

  // Schemes that can be used in a referrer.
  static void RegisterURLSchemeAsAllowedForReferrer(const String& scheme);
  static void RemoveURLSchemeAsAllowedForReferrer(const String& scheme);
  static bool ShouldTreatURLSchemeAsAllowedForReferrer(const String& scheme);

  // Schemes used for internal error pages, for failed navigations.
  static void RegisterURLSchemeAsError(const String&);
  static bool ShouldTreatURLSchemeAsError(const String& scheme);

  // Schemes which should always allow access to SharedArrayBuffers.
  // TODO(crbug.com/1184892): Remove once fixed.
  static void RegisterURLSchemeAsAllowingSharedArrayBuffers(const String&);
  static bool ShouldTreatURLSchemeAsAllowingSharedArrayBuffers(const String&);

  // Allow resources from some schemes to load on a page, regardless of its
  // Content Security Policy.
  enum PolicyAreas : uint32_t {
    kPolicyAreaNone = 0,
    kPolicyAreaImage = 1 << 0,
    kPolicyAreaStyle = 1 << 1,
    // Add more policy areas as needed by clients.
    kPolicyAreaAll = ~static_cast<uint32_t>(0),
  };
  static void RegisterURLSchemeAsBypassingContentSecurityPolicy(
      const String& scheme,
      PolicyAreas = kPolicyAreaAll);
  static void RemoveURLSchemeRegisteredAsBypassingContentSecurityPolicy(
      const String& scheme);
  static bool SchemeShouldBypassContentSecurityPolicy(
      const String& scheme,
      PolicyAreas = kPolicyAreaAll);

  // Schemes which bypass Secure Context checks defined in
  // https://w3c.github.io/webappsec-secure-contexts/#is-origin-trustworthy
  static void RegisterURLSchemeBypassingSecureContextCheck(
      const String& scheme);
  static bool SchemeShouldBypassSecureContextCheck(const String& scheme);

  // Schemes that can use 'wasm-eval'.
  static void RegisterURLSchemeAsAllowingWasmEvalCSP(const String& scheme);
  static bool SchemeSupportsWasmEvalCSP(const String& scheme);

  // Schemes that represent trusted browser UI.
  // TODO(chromium:1197375) Reconsider usages of this category. Are there
  // meaningful ways to define more abstract permissions or requirements that
  // could be used instead?
  static void RegisterURLSchemeAsWebUI(const String& scheme);
  static void RemoveURLSchemeAsWebUI(const String& scheme);
  static bool IsWebUIScheme(const String& scheme);

  // Like the above, but without threading safety checks.
  static void RegisterURLSchemeAsWebUIForTest(const String& scheme);
  static void RemoveURLSchemeAsWebUIForTest(const String& scheme);

  // Schemes which can use code caching but must check in the renderer whether
  // the script content has changed rather than relying on a response time match
  // from the network cache.
  static void RegisterURLSchemeAsCodeCacheWithHashing(const String& scheme);
  static void RemoveURLSchemeAsCodeCacheWithHashing(const String& scheme);
  static bool SchemeSupportsCodeCacheWithHashing(const String& scheme);

  // WebUI Schemes that can use bundled resource bytecode retrieved from the
  // static resource bundle.
  static void RegisterURLSchemeAsWebUIBundledBytecode(const String& scheme);
  static void RemoveURLSchemeAsWebUIBundledBytecodeForTesting(
      const String& scheme);
  static bool SchemeSupportsWebUIBundledBytecode(const String& scheme);

 private:
  static const URLSchemesSet& LocalSchemes();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_SCHEME_REGISTRY_H_
