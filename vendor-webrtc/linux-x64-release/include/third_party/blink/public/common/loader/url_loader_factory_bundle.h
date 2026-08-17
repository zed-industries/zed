// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_URL_LOADER_FACTORY_BUNDLE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_URL_LOADER_FACTORY_BUNDLE_H_

#include <map>
#include <memory>
#include <string>
#include <utility>

#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "services/network/public/cpp/shared_url_loader_factory.h"
#include "services/network/public/mojom/url_loader_factory.mojom.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/loader/local_resource_loader_config.mojom.h"
#include "url/origin.h"

namespace network {
struct ResourceRequest;
}

namespace blink {

// Holds the internal state of a URLLoaderFactoryBundle in a form that is safe
// to pass across sequences.
class BLINK_COMMON_EXPORT PendingURLLoaderFactoryBundle
    : public network::PendingSharedURLLoaderFactory {
 public:
  // Map from URL scheme to PendingRemote<URLLoaderFactory> for handling URL
  // requests for schemes not handled by the |pending_default_factory|. See also
  // URLLoaderFactoryBundle::SchemeMap.
  using SchemeMap =
      std::map<std::string,
               mojo::PendingRemote<network::mojom::URLLoaderFactory>>;

  // Map from origin of isolated world to PendingRemote<URLLoaderFactory> for
  // handling this isolated world's requests (e.g., for relaxing CORB for
  // requests initiated from content scripts).
  using OriginMap =
      std::map<url::Origin,
               mojo::PendingRemote<network::mojom::URLLoaderFactory>>;

  PendingURLLoaderFactoryBundle();
  PendingURLLoaderFactoryBundle(
      mojo::PendingRemote<network::mojom::URLLoaderFactory>
          pending_default_factory,
      SchemeMap scheme_specific_pending_factories,
      OriginMap isolated_world_pending_factories,
      mojom::LocalResourceLoaderConfigPtr local_resource_loader_config,
      bool bypass_redirect_checks);
  PendingURLLoaderFactoryBundle(const PendingURLLoaderFactoryBundle&) = delete;
  PendingURLLoaderFactoryBundle& operator=(
      const PendingURLLoaderFactoryBundle&) = delete;
  ~PendingURLLoaderFactoryBundle() override;

  mojo::PendingRemote<network::mojom::URLLoaderFactory>&
  pending_default_factory() {
    return pending_default_factory_;
  }

  SchemeMap& pending_scheme_specific_factories() {
    return pending_scheme_specific_factories_;
  }
  OriginMap& pending_isolated_world_factories() {
    return pending_isolated_world_factories_;
  }

  mojom::LocalResourceLoaderConfigPtr& local_resource_loader_config() {
    return local_resource_loader_config_;
  }
  void set_local_resource_loader_config(
      mojom::LocalResourceLoaderConfigPtr local_resource_loader_config) {
    local_resource_loader_config_ = std::move(local_resource_loader_config);
  }

  bool bypass_redirect_checks() const { return bypass_redirect_checks_; }
  void set_bypass_redirect_checks(bool bypass_redirect_checks) {
    bypass_redirect_checks_ = bypass_redirect_checks;
  }

  virtual bool IsTrackedChildPendingURLLoaderFactoryBundle() const;

 protected:
  // PendingSharedURLLoaderFactory implementation.
  scoped_refptr<network::SharedURLLoaderFactory> CreateFactory() override;

  mojo::PendingRemote<network::mojom::URLLoaderFactory>
      pending_default_factory_;
  SchemeMap pending_scheme_specific_factories_;

  // TODO(https://crbug.com/1098410): Remove the
  // `pending_isolated_world_factories_` field once Chrome Platform Apps are
  // gone.
  OriginMap pending_isolated_world_factories_;

  mojom::LocalResourceLoaderConfigPtr local_resource_loader_config_;

  bool bypass_redirect_checks_ = false;
};

// Encapsulates a collection of URLLoaderFactoryPtrs which can be usd to acquire
// loaders for various types of resource requests.
class BLINK_COMMON_EXPORT URLLoaderFactoryBundle
    : public network::SharedURLLoaderFactory {
 public:
  URLLoaderFactoryBundle();

  explicit URLLoaderFactoryBundle(
      std::unique_ptr<PendingURLLoaderFactoryBundle> pending_factories);

  // SharedURLLoaderFactory implementation.
  void CreateLoaderAndStart(
      mojo::PendingReceiver<network::mojom::URLLoader> loader,
      int32_t request_id,
      uint32_t options,
      const network::ResourceRequest& request,
      mojo::PendingRemote<network::mojom::URLLoaderClient> client,
      const net::MutableNetworkTrafficAnnotationTag& traffic_annotation)
      override;
  void Clone(mojo::PendingReceiver<network::mojom::URLLoaderFactory> receiver)
      override;
  std::unique_ptr<network::PendingSharedURLLoaderFactory> Clone() override;
  bool BypassRedirectChecks() const override;

  // The |pending_factories| contains replacement factories for a subset of the
  // existing bundle.
  void Update(std::unique_ptr<PendingURLLoaderFactoryBundle> pending_factories);

  bool HasBoundDefaultFactory() const { return default_factory_.is_bound(); }

 protected:
  ~URLLoaderFactoryBundle() override;

  // Returns a factory which can be used to acquire a loader for |request|.
  network::mojom::URLLoaderFactory* GetFactory(
      const network::ResourceRequest& request);

  template <typename TKey>
  static std::map<TKey, mojo::PendingRemote<network::mojom::URLLoaderFactory>>
  CloneRemoteMapToPendingRemoteMap(
      const std::map<TKey, mojo::Remote<network::mojom::URLLoaderFactory>>&
          input) {
    std::map<TKey, mojo::PendingRemote<network::mojom::URLLoaderFactory>>
        output;
    for (const auto& it : input) {
      const TKey& key = it.first;
      const mojo::Remote<network::mojom::URLLoaderFactory>& factory = it.second;
      mojo::PendingRemote<network::mojom::URLLoaderFactory> pending_factory;
      factory->Clone(pending_factory.InitWithNewPipeAndPassReceiver());
      output.emplace(key, std::move(pending_factory));
    }
    return output;
  }

  // |default_factory_| is the default factory used by the bundle. It usually
  // goes to "network", but it's possible it was overriden in case when the
  // context should not be given access to the network.
  mojo::Remote<network::mojom::URLLoaderFactory> default_factory_;

  // Map from URL scheme to Remote<URLLoaderFactory> for handling URL requests
  // for schemes not handled by the |default_factory_|.  See also
  // PendingURLLoaderFactoryBundle::SchemeMap and
  // ContentBrowserClient::SchemeToURLLoaderFactoryMap.
  using SchemeMap =
      std::map<std::string, mojo::Remote<network::mojom::URLLoaderFactory>>;
  SchemeMap scheme_specific_factories_;

  // Map from origin of isolated world to Remote<URLLoaderFactory> for handling
  // this isolated world's requests. See also
  // PendingURLLoaderFactoryBundle::OriginMap.
  //
  // TODO(https://crbug.com/1098410): Remove the `isolated_world_factories_`
  // field once Chrome Platform Apps are gone.
  using OriginMap =
      std::map<url::Origin, mojo::Remote<network::mojom::URLLoaderFactory>>;
  OriginMap isolated_world_factories_;

  mojom::LocalResourceLoaderConfigPtr local_resource_loader_config_;

  bool bypass_redirect_checks_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_URL_LOADER_FACTORY_BUNDLE_H_
