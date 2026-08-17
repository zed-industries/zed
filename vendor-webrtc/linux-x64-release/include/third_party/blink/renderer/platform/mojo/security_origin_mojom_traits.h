// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_SECURITY_ORIGIN_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_SECURITY_ORIGIN_MOJOM_TRAITS_H_

#include <optional>
#include <string_view>

#include "mojo/public/cpp/base/unguessable_token_mojom_traits.h"
#include "mojo/public/cpp/bindings/optional_as_pointer.h"
#include "mojo/public/cpp/bindings/string_traits_wtf.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "url/mojom/origin.mojom-shared.h"
#include "url/scheme_host_port.h"

namespace mojo {

struct UrlOriginAdapter {
  static const base::UnguessableToken* nonce_if_opaque(
      const scoped_refptr<const ::blink::SecurityOrigin>& origin) {
    return origin->GetNonceForSerialization();
  }
  static scoped_refptr<blink::SecurityOrigin> CreateSecurityOrigin(
      const url::SchemeHostPort& tuple,
      const std::optional<base::UnguessableToken>& nonce_if_opaque) {
    scoped_refptr<blink::SecurityOrigin> tuple_origin;
    if (tuple.IsValid()) {
      tuple_origin = blink::SecurityOrigin::CreateFromValidTuple(
          String::FromUTF8(tuple.scheme()), String::FromUTF8(tuple.host()),
          tuple.port());
    }

    if (nonce_if_opaque) {
      tuple_origin = blink::SecurityOrigin::CreateOpaque(
          url::Origin::Nonce(*nonce_if_opaque), tuple_origin.get());
    }
    return tuple_origin;
  }
  static const ::blink::SecurityOrigin* GetOriginOrPrecursorOriginIfOpaque(
      const scoped_refptr<const ::blink::SecurityOrigin>& origin) {
    return origin->GetOriginOrPrecursorOriginIfOpaque();
  }
};

template <>
struct StructTraits<url::mojom::OriginDataView,
                    scoped_refptr<const ::blink::SecurityOrigin>> {
  static WTF::String scheme(
      const scoped_refptr<const ::blink::SecurityOrigin>& origin) {
    return UrlOriginAdapter::GetOriginOrPrecursorOriginIfOpaque(origin)
        ->Protocol();
  }
  static WTF::String host(
      const scoped_refptr<const ::blink::SecurityOrigin>& origin) {
    return UrlOriginAdapter::GetOriginOrPrecursorOriginIfOpaque(origin)->Host();
  }
  static uint16_t port(
      const scoped_refptr<const ::blink::SecurityOrigin>& origin) {
    return UrlOriginAdapter::GetOriginOrPrecursorOriginIfOpaque(origin)->Port();
  }
  static mojo::OptionalAsPointer<const base::UnguessableToken> nonce_if_opaque(
      const scoped_refptr<const ::blink::SecurityOrigin>& origin) {
    return mojo::OptionalAsPointer(UrlOriginAdapter::nonce_if_opaque(origin));
  }
  static bool Read(url::mojom::OriginDataView data,
                   scoped_refptr<const ::blink::SecurityOrigin>* out) {
    // This implementation is very close to
    // SecurityOrigin::CreateFromUrlOrigin, so keep in sync if modifications
    // are made in that method.
    std::string_view scheme;
    std::string_view host;
    std::optional<base::UnguessableToken> nonce_if_opaque;
    if (!data.ReadScheme(&scheme) || !data.ReadHost(&host) ||
        !data.ReadNonceIfOpaque(&nonce_if_opaque))
      return false;

    const url::SchemeHostPort& tuple =
        url::SchemeHostPort(scheme, host, data.port());
    if (!tuple.IsValid()) {
      // If the tuple is invalid, it is a valid case if and only if it is an
      // opaque origin and the scheme, host, and port are empty.
      if (!nonce_if_opaque)
        return false;

      if (!scheme.empty() || !host.empty() || data.port() != 0)
        return false;
    }

    *out = UrlOriginAdapter::CreateSecurityOrigin(tuple, nonce_if_opaque);

    // If an opaque origin was created, there must be a valid
    // |nonce_if_opaque| value supplied.
    if ((*out)->IsOpaque() && !nonce_if_opaque.has_value())
      return false;

    return true;
  }

  static bool IsNull(
      const scoped_refptr<const ::blink::SecurityOrigin>& origin) {
    return !origin;
  }

  static void SetToNull(scoped_refptr<const ::blink::SecurityOrigin>* origin) {
    *origin = nullptr;
  }
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_SECURITY_ORIGIN_MOJOM_TRAITS_H_
