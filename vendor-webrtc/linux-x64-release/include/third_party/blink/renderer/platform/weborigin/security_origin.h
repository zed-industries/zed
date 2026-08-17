/*
 * Copyright (C) 2007, 2008 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_SECURITY_ORIGIN_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_SECURITY_ORIGIN_H_

#include <stdint.h>

#include <memory>
#include <optional>

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "url/origin.h"

namespace WTF {
class StringBuilder;
}  // namespace WTF

namespace blink {

class KURL;

// An identifier which defines the source of content (e.g. a document) and
// restricts what other objects it is permitted to access (based on their
// security origin). Most commonly, an origin is a (scheme, host, port, domain)
// tuple, such as the tuple origin (https, chromium.org, null, null). However,
// there are also opaque origins which do not have a corresponding tuple.
//
// See also: https://html.spec.whatwg.org/C/#concept-origin
class PLATFORM_EXPORT SecurityOrigin : public RefCounted<SecurityOrigin> {
  USING_FAST_MALLOC(SecurityOrigin);

 public:
  enum class AccessResultDomainDetail {
    kDomainNotRelevant,
    kDomainNotSet,
    kDomainSetByOnlyOneOrigin,
    kDomainMatchNecessary,
    kDomainMatchUnnecessary,
    kDomainMismatch,
    kDomainNotRelevantAgentClusterMismatch,
  };

  // SecurityOrigin::Create() resolves |url| to its SecurityOrigin. When |url|
  // contains a standard (scheme, host, port) tuple, |reference_origin| is
  // ignored. If |reference_origin| is provided and an opaque origin is returned
  // (for example, if |url| has the "data:" scheme), the opaque origin will be
  // derived from |reference_origin|, retaining the precursor information.  If
  // |url| is "about:blank", a copy of |reference_origin| is returned.  If no
  // |reference_origin| has been provided, then "data:" and "about:blank" URLs
  // will resolve into an opaque, unique origin.
  static scoped_refptr<SecurityOrigin> CreateWithReferenceOrigin(
      const KURL& url,
      const SecurityOrigin* reference_origin);

  // Equivalent to CreateWithReferenceOrigin without supplying value for
  // |reference_origin|.
  static scoped_refptr<SecurityOrigin> Create(const KURL& url);

  // Creates a new opaque SecurityOrigin that is guaranteed to be cross-origin
  // to all currently existing SecurityOrigins.
  static scoped_refptr<SecurityOrigin> CreateUniqueOpaque();

  static scoped_refptr<SecurityOrigin> CreateFromString(const String&);

  // Constructs a non-opaque tuple origin, analogously to
  // url::Origin::Origin(url::SchemeHostPort).
  //
  // REQUIRES: The tuple be valid: |protocol| must contain a standard scheme and
  // |host| must be canonicalized and (except for "file" URLs) nonempty.
  static scoped_refptr<SecurityOrigin> CreateFromValidTuple(
      const String& protocol,
      const String& host,
      uint16_t port);

  static scoped_refptr<SecurityOrigin> CreateFromUrlOrigin(const url::Origin&);
  url::Origin ToUrlOrigin() const;

  SecurityOrigin(const SecurityOrigin&) = delete;
  SecurityOrigin& operator=(const SecurityOrigin&) = delete;

  // Some URL schemes use nested URLs for their security context. For example,
  // filesystem URLs look like the following:
  //
  //   filesystem:http://example.com/temporary/path/to/file.png
  //
  // We're supposed to use "http://example.com" as the origin.
  //
  // Generally, we add URL schemes to this list when Blink supports them. For
  // example, we don't include the "jar" scheme, even though Firefox
  // understands that "jar" uses an inner URL for its security origin.
  static bool ShouldUseInnerURL(const KURL&);
  static KURL ExtractInnerURL(const KURL&);

  // Create a deep copy of this SecurityOrigin. This method is useful
  // when marshalling a SecurityOrigin to another thread.
  scoped_refptr<SecurityOrigin> IsolatedCopy() const;

  // Set the domain property of this security origin to newDomain. This
  // function does not check whether newDomain is a suffix of the current
  // domain. The caller is responsible for validating newDomain.
  void SetDomainFromDOM(const String& new_domain);
  bool DomainWasSetInDOM() const { return domain_was_set_in_dom_; }

  const String& Protocol() const { return protocol_; }
  const String& Host() const { return host_; }
  const String& Domain() const { return domain_; }

  // Returns the registrable domain if available.
  // For non-tuple origin, IP address URL, and public suffixes, this returns a
  // null string. https://url.spec.whatwg.org/#host-registrable-domain
  String RegistrableDomain() const;

  // Returns the effective port, even if it is the default port for the
  // scheme (e.g. "http" => 80).
  uint16_t Port() const { return port_; }

  // Returns true if this SecurityOrigin can script objects in the given
  // SecurityOrigin. This check is similar to `IsSameOriginDomainWith()`, but
  // additionally takes "universal access" flag into account, as well as the
  // origin's agent cluster (see https://tc39.es/ecma262/#sec-agent-clusters).
  //
  // Note: This kind of access check should be rare; `IsSameOriginWith()` is
  // almost certainly the right choice for new security checks.
  //
  // TODO(1027191): We're currently calling this method in a number of places
  // where either `IsSameOriginWith()` or `IsSameOriginDomainWith()` might
  // be more appropriate. We should audit its existing usage, and it might
  // make sense to move it out of SecurityOrigin entirely to align it more
  // tightly with `BindingSecurity` where it's clearly necessary.
  bool CanAccess(const SecurityOrigin* other) const {
    AccessResultDomainDetail unused_detail;
    return CanAccess(other, unused_detail);
  }
  bool CanAccess(const scoped_refptr<SecurityOrigin>& other) const {
    return CanAccess(other.get());
  }
  bool CanAccess(const SecurityOrigin* other, AccessResultDomainDetail&) const;
  bool CanAccess(const scoped_refptr<SecurityOrigin>& other,
                 AccessResultDomainDetail& detail) const {
    return CanAccess(other.get(), detail);
  }

  // Returns true if this SecurityOrigin can read content retrieved from
  // the given URL.
  // Note: This function may return false when |url| has data scheme, which
  // is not aligned with CORS. If you want a CORS-aligned check, just use
  // CORS mode (e.g., network::mojom::RequestMode::kSameOrigin), or
  // use CanReadContent.
  // See
  // https://docs.google.com/document/d/1_BD15unoPJVwKyf5yOUDu5kie492TTaBxzhJ58j1rD4/edit.
  bool CanRequest(const KURL& url) const;

  // Returns true if content from this URL can be read without CORS from this
  // security origin. For example, call this function before drawing an image
  // onto an HTML canvas element with the drawImage API.
  bool CanReadContent(const KURL&) const;

  // Returns true if |document| can display content from the given URL (e.g.,
  // in an iframe or as an image). For example, web sites generally cannot
  // display content from the user's files system.
  bool CanDisplay(const KURL&) const;

  // Returns true if the origin loads resources either from the local
  // machine or over the network from a
  // cryptographically-authenticated origin, as described in
  // https://w3c.github.io/webappsec-secure-contexts/#is-origin-trustworthy
  bool IsPotentiallyTrustworthy() const;

  // Returns a human-readable error message describing that a non-secure
  // origin's access to a feature is denied.
  static String IsPotentiallyTrustworthyErrorMessage();

  // Returns true if this SecurityOrigin can load local resources, such
  // as images, iframes, and style sheets, and can link to local URLs.
  // For example, call this function before creating an iframe to a
  // file:// URL.
  //
  // Note: A SecurityOrigin might be allowed to load local resources
  //       without being able to issue an XMLHttpRequest for a local URL.
  //       To determine whether the SecurityOrigin can issue an
  //       XMLHttpRequest for a URL, call canReadContent(url).
  bool CanLoadLocalResources() const { return can_load_local_resources_; }

  // Explicitly grant the ability to load local resources to this
  // SecurityOrigin.
  //
  // Note: This method exists only to support backwards compatibility
  //       with older versions of WebKit.
  void GrantLoadLocalResources();

  // Explicitly grant the ability to access every other SecurityOrigin.
  //
  // WARNING: This is an extremely powerful ability. Use with caution!
  void GrantUniversalAccess();
  bool IsGrantedUniversalAccess() const { return universal_access_; }

  // Whether this origin has ability to access another SecurityOrigin
  // if everything but the agent clusters do not match.
  void GrantCrossAgentClusterAccess();
  bool IsGrantedCrossAgentClusterAccess() const {
    return cross_agent_cluster_access_;
  }

  bool CanAccessDatabase() const { return !IsOpaque(); }
  bool CanAccessLocalStorage() const { return !IsOpaque(); }
  bool CanAccessSharedWorkers() const { return !IsOpaque(); }
  bool CanAccessServiceWorkers() const { return !IsOpaque(); }
  bool CanAccessCookies() const { return !IsOpaque(); }
  bool CanAccessPasswordManager() const { return !IsOpaque(); }
  bool CanAccessFileSystem() const { return !IsOpaque(); }
  bool CanAccessCacheStorage() const { return !IsOpaque(); }
  bool CanAccessLocks() const { return !IsOpaque(); }
  bool CanAccessSessionStorage() const { return !IsOpaque(); }
  bool CanAccessStorageBuckets() const { return !IsOpaque(); }

  // The local SecurityOrigin is the most privileged SecurityOrigin.
  // The local SecurityOrigin can script any document, navigate to local
  // resources, and can set arbitrary headers on XMLHttpRequests.
  bool IsLocal() const;

  // Returns true if the host is one of 127.0.0.1/8, ::1/128, or "localhost".
  bool IsLocalhost() const;

  // Returns true if the origin is not a tuple origin (i.e. an origin consisting
  // of a scheme, host, port, and domain). Opaque origins are created for a
  // variety of situations (see https://whatwg.org/C/origin.html#origin for more
  // details), such as for documents generated from data: URLs or documents
  // with the sandboxed origin browsing context flag set.
  bool IsOpaque() const { return !!nonce_if_opaque_; }

  // By default 'file:' URLs may access other 'file:' URLs. This method
  // denies access. If either SecurityOrigin sets this flag, the access
  // check will fail.
  void BlockLocalAccessFromLocalOrigin();

  // Convert this SecurityOrigin into a string. The string representation of a
  // SecurityOrigin is similar to a URL, except it lacks a path component. The
  // string representation does not encode the value of the SecurityOrigin's
  // domain property.
  //
  // When using the string value, it's important to remember that it might be
  // "null". This typically happens when this SecurityOrigin is opaque (e.g. the
  // origin of a sandboxed iframe).
  //
  // This should be kept in sync with url::Origin::Serialize().
  //
  // TODO(crbug.com/40554285, crbug.com/40467682): Note that there's a subtle
  // difference in how this function handles file: URL origins compared to
  // url::Origin::Serialize(). url::Origin always serializes them to "file://",
  // whereas this function serializes them to "null" or // "file://" depending
  // on the `allow_file_access_from_file_urls` flag in WebPreferences. This
  // difference should be cleaned up, along with the workaround for it in
  // RenderFrameProxyHost::SerializePostMessageSourceOrigin().
  String ToString() const;
  AtomicString ToAtomicString() const;

  // Similar to ToString(), but does not take into account any factors that
  // could make the string return "null".
  String ToRawString() const;

  // Returns a token that helps distinguish origins, or null string. When not
  // null string, the tokens are guaranteed to be different if not the same
  // origin, i.e. if two tokens are the same and not null, the two
  // SecurityOrigins are the same origin. Thus, tokens can be used for fast
  // check of origins.
  //
  // This is pretty similar to ToString(), but this returns null string instead
  // of "null", and includes a host part in case of file: scheme.
  //
  // Note that the same tokens only guarantee that the SecurityOrigins are
  // the same origin and not the same origin-domain. See also:
  // https://html.spec.whatwg.org/C/origin.html#same-origin
  // https://html.spec.whatwg.org/C/origin.html#same-origin-domain
  String ToTokenForFastCheck() const;

  // This method implements HTML's "same origin" check, which verifies equality
  // of opaque origins, or exact (scheme,host,port) matches. Note that
  // `document.domain` does not come into play for this comparison.
  //
  // This method does not take the "universal access" flag into account. It does
  // take the "local access" flag into account, considering `file:` origins that
  // set the flag to be same-origin with all other `file:` origins that set the
  // flag.
  //
  // https://html.spec.whatwg.org/#same-origin
  bool IsSameOriginWith(const SecurityOrigin*) const;
  static bool AreSameOrigin(const KURL& a, const KURL& b);

  // This method implements HTML's "same origin-domain" check, which takes
  // `document.domain` into account when comparing two origins.
  //
  // This method does not take the "universal access" flag into account. It does
  // take the "local access" flag into account, considering `file:` origins that
  // set the flag to be same origin-domain with all other `file:` origins that
  // set the flag (assuming no `document.domain` mismatch).
  //
  // Note: Same origin-domain checks should be rare, and `IsSameOriginWith()`
  // is almost certainly the right choice for new security checks.
  //
  // https://html.spec.whatwg.org/#same-origin-domain
  bool IsSameOriginDomainWith(const SecurityOrigin* other) const {
    AccessResultDomainDetail unused_detail;
    return IsSameOriginDomainWith(other, unused_detail);
  }
  bool IsSameOriginDomainWith(const SecurityOrigin*,
                              AccessResultDomainDetail&) const;

  // This method implements HTML's "same site" check, which is true if the two
  // origins are schemelessly same site, and either are both opaque or are both
  // tuple origins with the same scheme.
  //
  // Note: Use of "same site" should be avoided when possible, in favor of "same
  // origin" checks. A "same origin" check is generally more appropriate for
  // security decisions, as registrable domains cannot be relied upon to provide
  // a hard security boundary.
  //
  // https://html.spec.whatwg.org/#same-site
  bool IsSameSiteWith(const SecurityOrigin* other) const;

  static const KURL& UrlWithUniqueOpaqueOrigin();

  // Transfer origin privileges from another security origin.
  // The following privileges are currently copied over:
  //
  //   - Grant universal access.
  //   - Grant loading of local resources.
  //   - Use path-based file:// origins.
  struct PrivilegeData {
    bool universal_access_;
    bool can_load_local_resources_;
    bool block_local_access_from_local_origin_;
  };
  std::unique_ptr<PrivilegeData> CreatePrivilegeData() const;
  void TransferPrivilegesFrom(std::unique_ptr<PrivilegeData>);

  void SetOpaqueOriginIsPotentiallyTrustworthy(
      bool is_opaque_origin_potentially_trustworthy);

  // Creates a new opaque security origin derived from |this| (|this| becomes
  // its precursor).
  scoped_refptr<SecurityOrigin> DeriveNewOpaqueOrigin() const;

  // If this is an opaque origin that was derived from a tuple origin, return
  // the origin from which this was derived. Otherwise returns |this|. This
  // method may be used for things like CSP 'self' computation which require
  // the origin before sandbox flags are applied. It should NOT be used for
  // any security checks (such as bindings).
  const SecurityOrigin* GetOriginOrPrecursorOriginIfOpaque() const;

  // Only used for document.domain setting. The method should probably be moved
  // if we need it for something more general.
  static String CanonicalizeSpecialHost(const String& host, bool* success);
  static String CanonicalizeHost(const String& host,
                                 const String& scheme,
                                 bool* success);

  // Return a security origin that is assigned to the agent cluster. This will
  // be a copy of this security origin if the current agent doesn't match the
  // provided agent, otherwise it will be a reference to this.
  scoped_refptr<SecurityOrigin> GetOriginForAgentCluster(
      const base::UnguessableToken& cluster_id);

  const base::UnguessableToken& AgentClusterId() const {
    return agent_cluster_id_;
  }

  // Returns true if this security origin is serialized to "null".
  bool SerializesAsNull() const;

  // Whether document.open was called in between two different windows, causing
  // the SecurityOrigin to be shared by both. This is only used to record
  // metrics.
  // To be removed after shipping DocumentOpenSandboxInheritanceRemoval feature.
  void set_aliased_by_document_open() { aliased_by_document_open_ = true; }
  bool aliased_by_document_open() const { return aliased_by_document_open_; }

  bool block_local_access_from_local_origin() const {
    return block_local_access_from_local_origin_;
  }

 private:
  // Various serialisation and test routines that need direct nonce access.
  friend struct mojo::UrlOriginAdapter;
  friend struct WTF::HashTraits<scoped_refptr<const SecurityOrigin>>;
  friend class SecurityOriginTest;

  // For calling GetNonceForSerialization().
  friend class BlobURLOpaqueOriginNonceMap;

  // Creates a new opaque SecurityOrigin using the supplied |precursor| origin
  // and |nonce|.
  static scoped_refptr<SecurityOrigin> CreateOpaque(
      const url::Origin::Nonce& nonce,
      const SecurityOrigin* precursor);

  // Create an opaque SecurityOrigin.
  SecurityOrigin(const url::Origin::Nonce& nonce,
                 const SecurityOrigin* precursor_origin);

  // Creates an opaque SecurityOrigin with a new unique nonce. Similar to the
  // above, but preferred when there is no pre-existing nonce to copy, as
  // copying a nonce requires forcing eager initialisation of that nonce.
  enum class NewUniqueOpaque {
    kWithLazyInitNonce,
  };
  SecurityOrigin(NewUniqueOpaque, const SecurityOrigin* precursor_origin);

  // Create a tuple SecurityOrigin, with parameters via KURL
  static scoped_refptr<SecurityOrigin> CreateInternal(const KURL& url);

  // Constructs a non-opaque tuple origin, analogously to
  // url::Origin::Origin(url::SchemeHostPort).
  SecurityOrigin(const String& protocol, const String& host, uint16_t port);

  enum class ConstructIsolatedCopy { kConstructIsolatedCopyBit };
  // Clone a SecurityOrigin which is safe to use on other threads.
  SecurityOrigin(const SecurityOrigin* other, ConstructIsolatedCopy);

  enum class ConstructSameThreadCopy { kConstructSameThreadCopyBit };
  // Clone a SecurityOrigin which is *NOT* safe to use on other threads.
  SecurityOrigin(const SecurityOrigin* other, ConstructSameThreadCopy);

  // FIXME: Rename this function to something more semantic.
  bool PassesFileCheck(const SecurityOrigin*) const;
  void BuildRawString(WTF::StringBuilder&) const;

  // Get the nonce associated with this origin, if it is opaque. This should be
  // used only when trying to send an Origin across an IPC pipe or comparing
  // blob URL's opaque origins in the thread-safe way.
  const base::UnguessableToken* GetNonceForSerialization() const;

  const String protocol_ = g_empty_string;
  const String host_ = g_empty_string;
  String domain_ = g_empty_string;
  const uint16_t port_ = 0;
  const std::optional<url::Origin::Nonce> nonce_if_opaque_;
  bool universal_access_ = false;
  bool domain_was_set_in_dom_ = false;
  bool can_load_local_resources_ = false;
  bool block_local_access_from_local_origin_ = false;
  bool is_opaque_origin_potentially_trustworthy_ = false;
  bool cross_agent_cluster_access_ = false;
  bool aliased_by_document_open_ = false;

  // A security origin can have an empty |agent_cluster_id_|. It occurs in the
  // cases where a security origin hasn't been assigned to a document yet.
  base::UnguessableToken agent_cluster_id_;

  // For opaque origins, tracks the non-opaque origin from which the opaque
  // origin is derived.
  const scoped_refptr<const SecurityOrigin> precursor_origin_;
};

}  // namespace blink

namespace WTF {

// The default HashTraits of SecurityOrigin implements the "same origin"
// equality relation between two origins. As such it ignores the domain that
// might or might not be set on the origin. If you need "same origin-domain"
// equality you'll need to define a custom hash traits type using a different
// hash function.
template <>
struct HashTraits<scoped_refptr<const blink::SecurityOrigin>>
    : GenericHashTraits<scoped_refptr<const blink::SecurityOrigin>> {
  static unsigned GetHash(const blink::SecurityOrigin* origin) {
    const base::UnguessableToken* nonce = origin->GetNonceForSerialization();
    size_t nonce_hash = nonce ? base::UnguessableTokenHash()(*nonce) : 0;

    unsigned hash_codes[] = {
      origin->Protocol().Impl() ? origin->Protocol().Impl()->GetHash() : 0,
      origin->Host().Impl() ? origin->Host().Impl()->GetHash() : 0,
      origin->Port(),
#if ARCH_CPU_32_BITS
      nonce_hash,
#elif ARCH_CPU_64_BITS
      static_cast<unsigned>(nonce_hash),
      static_cast<unsigned>(nonce_hash >> 32),
#else
#error "Unknown bits"
#endif
    };
    return StringHasher::HashMemory(base::as_byte_span(hash_codes));
  }
  static unsigned GetHash(
      const scoped_refptr<const blink::SecurityOrigin>& origin) {
    return GetHash(origin.get());
  }

  static bool Equal(const blink::SecurityOrigin* a,
                    const blink::SecurityOrigin* b) {
    return a->IsSameOriginWith(b);
  }
  static bool Equal(const blink::SecurityOrigin* a,
                    const scoped_refptr<const blink::SecurityOrigin>& b) {
    return Equal(a, b.get());
  }
  static bool Equal(const scoped_refptr<const blink::SecurityOrigin>& a,
                    const blink::SecurityOrigin* b) {
    return Equal(a.get(), b);
  }
  static bool Equal(const scoped_refptr<const blink::SecurityOrigin>& a,
                    const scoped_refptr<const blink::SecurityOrigin>& b) {
    return Equal(a.get(), b.get());
  }

  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_SECURITY_ORIGIN_H_
