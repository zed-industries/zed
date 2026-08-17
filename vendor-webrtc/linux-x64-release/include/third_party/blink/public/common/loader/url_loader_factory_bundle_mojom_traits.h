// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_URL_LOADER_FACTORY_BUNDLE_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_URL_LOADER_FACTORY_BUNDLE_MOJOM_TRAITS_H_

#include <memory>

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/struct_traits.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/loader/url_loader_factory_bundle.h"
#include "third_party/blink/public/mojom/loader/local_resource_loader_config.mojom-forward.h"
#include "third_party/blink/public/mojom/loader/url_loader_factory_bundle.mojom-shared.h"
#include "url/mojom/origin_mojom_traits.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::URLLoaderFactoryBundleDataView,
                 std::unique_ptr<blink::PendingURLLoaderFactoryBundle>> {
  using BundleInfoType = std::unique_ptr<blink::PendingURLLoaderFactoryBundle>;

  static bool IsNull(const BundleInfoType& bundle) { return !bundle; }

  static void SetToNull(BundleInfoType* bundle) { bundle->reset(); }

  static mojo::PendingRemote<network::mojom::URLLoaderFactory> default_factory(
      BundleInfoType& bundle);

  static blink::PendingURLLoaderFactoryBundle::SchemeMap
  scheme_specific_factories(BundleInfoType& bundle);

  static blink::PendingURLLoaderFactoryBundle::OriginMap
  isolated_world_factories(BundleInfoType& bundle);

  static bool bypass_redirect_checks(BundleInfoType& bundle);

  static blink::mojom::LocalResourceLoaderConfigPtr
  local_resource_loader_config(BundleInfoType& bundle);

  static bool Read(blink::mojom::URLLoaderFactoryBundleDataView data,
                   BundleInfoType* out_bundle);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_URL_LOADER_FACTORY_BUNDLE_MOJOM_TRAITS_H_
