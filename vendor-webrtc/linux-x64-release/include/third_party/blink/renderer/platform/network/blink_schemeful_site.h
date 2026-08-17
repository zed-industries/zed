// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_BLINK_SCHEMEFUL_SITE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_BLINK_SCHEMEFUL_SITE_H_

#include "base/check.h"
#include "base/gtest_prod_util.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace network {
namespace mojom {
class SchemefulSiteDataView;
}  // namespace mojom
}  // namespace network

namespace url {
class Origin;
}  // namespace url

namespace blink {

// This class is the blink version of net::SchemefulSite and only implements a
// subset of its features. Its data members should be kept in sync.
//
// It represents a scheme and eTLD+1 for an origin, as specified by
// https://html.spec.whatwg.org/multipage/origin.html#obtain-a-site.
//
// See schemeful_site.h for more information.
class PLATFORM_EXPORT BlinkSchemefulSite {
 public:
  // Creates a BlinkSchemefulSite with a new unique origin.
  BlinkSchemefulSite();

  // Creates a BlinkSchemefulSite with the provided `origin`.
  // The passed `origin` may not match the resulting internal representation in
  // certain circumstances. See the comment, below, on the `site_as_origin_`
  // member.
  explicit BlinkSchemefulSite(scoped_refptr<const SecurityOrigin> origin);
  explicit BlinkSchemefulSite(const url::Origin& origin);

  // Creates a BlinkSchemefulSite by converting a net::SchemefulSite.
  explicit BlinkSchemefulSite(const net::SchemefulSite& site);

  // Converts this BlinkSchemefulSite into a net::SchemefulSite.
  explicit operator net::SchemefulSite() const;

  BlinkSchemefulSite(const BlinkSchemefulSite& other) = default;
  BlinkSchemefulSite& operator=(const BlinkSchemefulSite& other) = default;
  BlinkSchemefulSite(BlinkSchemefulSite&& other) = default;
  BlinkSchemefulSite& operator=(BlinkSchemefulSite&& other) = default;

  // Returns a string version of the internal site_as_origin_. If this origin is
  // unique then this will return "null".
  String Serialize() const;

  // Returns the results of `Serialize()` with some additional human friendly
  // text.
  String GetDebugString() const;

  bool operator==(const BlinkSchemefulSite& rhs) const {
    DCHECK(site_as_origin_);
    DCHECK(rhs.site_as_origin_);
    return site_as_origin_->IsSameOriginWith(rhs.site_as_origin_.get());
  }

  bool operator!=(const BlinkSchemefulSite& rhs) const {
    return !operator==(rhs);
  }

  bool IsOpaque() const { return site_as_origin_->IsOpaque(); }

 private:
  friend struct WTF::HashTraits<BlinkSchemefulSite>;

  // IPC serialization code needs to access internal origin.
  friend struct mojo::StructTraits<network::mojom::SchemefulSiteDataView,
                                   blink::BlinkSchemefulSite>;

  FRIEND_TEST_ALL_PREFIXES(BlinkSchemefulSiteMojomTraitsTest,
                           DeserializeFailure);

  FRIEND_TEST_ALL_PREFIXES(BlinkSchemefulSiteTest, FromWire);

  // Tries to construct an instance from a (potentially
  // untrusted) value of the internal `site_as_origin_`
  // that got received over an RPC.
  //
  // Returns whether successful or not. Doesn't touch
  // `*out` if false is returned.  This returning true does
  // not mean that whoever sent the values did not lie,
  // merely that they are well-formed.
  static bool FromWire(const url::Origin& site_as_origin,
                       BlinkSchemefulSite* out);

  // Origin which stores the result of running the steps documented at
  // https://html.spec.whatwg.org/multipage/origin.html#obtain-a-site. This is
  // not an arbitrary origin. It must either be an opaque origin, or a scheme +
  // eTLD+1 + default port.
  //
  // The `origin` passed into the BlinkSchemefulSite constructors might not
  // match the internal representation used by this class to track the scheme
  // and eTLD+1 representing a schemeful site. This may be the case if, e.g.,
  // the passed `origin` has an eTLD+1 that is not equal to its hostname, or if
  // the port number is not the default port for its scheme.
  //
  // In general, this `site_as_origin_` used for the internal representation
  // should NOT be used directly by SchemefulSite consumers.
  scoped_refptr<const SecurityOrigin> site_as_origin_;
};

}  // namespace blink

namespace WTF {

template <>
struct HashTraits<blink::BlinkSchemefulSite>
    : OneFieldHashTraits<blink::BlinkSchemefulSite,
                         &blink::BlinkSchemefulSite::site_as_origin_> {};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_BLINK_SCHEMEFUL_SITE_H_
