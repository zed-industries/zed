// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUCHSIA_SCOPED_SERVICE_BINDING_H_
#define BASE_FUCHSIA_SCOPED_SERVICE_BINDING_H_

// TODO(crbug.com/42050587): Remove this include once the explicit
// async_get_default_dispatcher() is no longer needed.
#include <lib/async/default.h>
#include <lib/fidl/cpp/binding.h>
#include <lib/fidl/cpp/binding_set.h>
#include <lib/fidl/cpp/interface_request.h>
#include <lib/fidl/cpp/wire/connect_service.h>
#include <lib/zx/channel.h>

#include <optional>
#include <string_view>
#include <utility>

#include "base/base_export.h"
#include "base/fuchsia/scoped_service_publisher.h"
#include "base/functional/callback.h"

namespace sys {
class OutgoingDirectory;
}  // namespace sys

namespace vfs {
class PseudoDir;
}  // namespace vfs

namespace base {

template <typename Interface>
class BASE_EXPORT ScopedServiceBinding {
 public:
  // Publishes a public service in the specified |outgoing_directory|.
  // |outgoing_directory| and |impl| must outlive the binding. The service is
  // unpublished on destruction.
  ScopedServiceBinding(sys::OutgoingDirectory* outgoing_directory,
                       Interface* impl,
                       std::string_view name = Interface::Name_)
      : publisher_(outgoing_directory, bindings_.GetHandler(impl), name) {}

  // Publishes a service in the specified |pseudo_dir|. |pseudo_dir| and |impl|
  // must outlive the binding. The service is unpublished on destruction.
  ScopedServiceBinding(vfs::PseudoDir* pseudo_dir,
                       Interface* impl,
                       std::string_view name = Interface::Name_)
      : publisher_(pseudo_dir, bindings_.GetHandler(impl), name) {}

  ScopedServiceBinding(const ScopedServiceBinding&) = delete;
  ScopedServiceBinding& operator=(const ScopedServiceBinding&) = delete;

  ~ScopedServiceBinding() = default;

  // |on_last_client_callback| will be called every time the number of connected
  // clients drops to 0.
  void SetOnLastClientCallback(base::RepeatingClosure on_last_client_callback) {
    bindings_.set_empty_set_handler(
        [callback = std::move(on_last_client_callback)] { callback.Run(); });
  }

  bool has_clients() const { return bindings_.size() != 0; }

 private:
  fidl::BindingSet<Interface> bindings_;
  ScopedServicePublisher<Interface> publisher_;
};

template <typename Protocol>
class BASE_EXPORT ScopedNaturalServiceBinding {
 public:
  // Publishes a public service in the specified |outgoing_directory|.
  // |outgoing_directory| and |impl| must outlive the binding. The service is
  // unpublished on destruction.
  ScopedNaturalServiceBinding(
      sys::OutgoingDirectory* outgoing_directory,
      fidl::Server<Protocol>* impl,
      std::string_view name = fidl::DiscoverableProtocolName<Protocol>)
      : publisher_(
            outgoing_directory,
            bindings_.CreateHandler(
                impl,
                // TODO(crbug.com/42050587): Remove this param once there's an
                // overload of `CreateHandler` that doesn't require it.
                async_get_default_dispatcher(),
                [](fidl::UnbindInfo info) {}),
            name) {}

  // Publishes a service in the specified |pseudo_dir|. |pseudo_dir| and |impl|
  // must outlive the binding. The service is unpublished on destruction.
  ScopedNaturalServiceBinding(
      vfs::PseudoDir* pseudo_dir,
      fidl::Server<Protocol>* impl,
      std::string_view name = fidl::DiscoverableProtocolName<Protocol>)
      : publisher_(
            pseudo_dir,
            bindings_.CreateHandler(
                impl,
                // TODO(crbug.com/42050587): Remove this param once there's an
                // overload of `CreateHandler` that doesn't require it.
                async_get_default_dispatcher(),
                [](fidl::UnbindInfo info) {}),
            name) {}

  ScopedNaturalServiceBinding(const ScopedNaturalServiceBinding&) = delete;
  ScopedNaturalServiceBinding& operator=(const ScopedNaturalServiceBinding&) =
      delete;

  ~ScopedNaturalServiceBinding() = default;

  // Registers `on_last_client_callback` to be called every time the number of
  // connected clients drops to 0.
  void SetOnLastClientCallback(base::RepeatingClosure on_last_client_callback) {
    bindings_.set_empty_set_handler(
        [callback = std::move(on_last_client_callback)] { callback.Run(); });
  }

  bool has_clients() const { return bindings_.size() != 0; }

 private:
  fidl::ServerBindingGroup<Protocol> bindings_;
  ScopedNaturalServicePublisher<Protocol> publisher_;
};

// Scoped service binding which allows only a single client to be connected
// at any time. By default a new connection will disconnect an existing client.
enum class ScopedServiceBindingPolicy {
  kPreferNew,
  kPreferExisting,
  kConnectOnce
};

template <typename Interface,
          ScopedServiceBindingPolicy Policy =
              ScopedServiceBindingPolicy::kPreferNew>
class BASE_EXPORT ScopedSingleClientServiceBinding {
 public:
  // |outgoing_directory| and |impl| must outlive the binding.
  ScopedSingleClientServiceBinding(sys::OutgoingDirectory* outgoing_directory,
                                   Interface* impl,
                                   std::string_view name = Interface::Name_)
      : binding_(impl) {
    publisher_.emplace(
        outgoing_directory,
        fit::bind_member(this, &ScopedSingleClientServiceBinding::BindClient),
        name);
    binding_.set_error_handler(fit::bind_member(
        this, &ScopedSingleClientServiceBinding::OnBindingEmpty));
  }

  ScopedSingleClientServiceBinding(vfs::PseudoDir* publish_to,
                                   Interface* impl,
                                   std::string_view name = Interface::Name_)
      : binding_(impl) {
    publisher_.emplace(
        publish_to,
        fit::bind_member(this, &ScopedSingleClientServiceBinding::BindClient),
        name);
    binding_.set_error_handler(fit::bind_member(
        this, &ScopedSingleClientServiceBinding::OnBindingEmpty));
  }

  ScopedSingleClientServiceBinding(const ScopedSingleClientServiceBinding&) =
      delete;
  ScopedSingleClientServiceBinding& operator=(
      const ScopedSingleClientServiceBinding&) = delete;

  ~ScopedSingleClientServiceBinding() = default;

  typename Interface::EventSender_& events() { return binding_.events(); }

  // |on_last_client_callback| will be called the first time a client
  // disconnects. It is still  possible for a client to connect after that point
  // if Policy is kPreferNew of kPreferExisting.
  void SetOnLastClientCallback(base::OnceClosure on_last_client_callback) {
    on_last_client_callback_ = std::move(on_last_client_callback);
  }

  bool has_clients() const { return binding_.is_bound(); }

 private:
  void BindClient(fidl::InterfaceRequest<Interface> request) {
    if (Policy == ScopedServiceBindingPolicy::kPreferExisting &&
        binding_.is_bound()) {
      return;
    }
    binding_.Bind(std::move(request));
    if (Policy == ScopedServiceBindingPolicy::kConnectOnce) {
      publisher_.reset();
    }
  }

  void OnBindingEmpty(zx_status_t status) {
    if (on_last_client_callback_) {
      std::move(on_last_client_callback_).Run();
    }
  }

  fidl::Binding<Interface> binding_;
  std::optional<ScopedServicePublisher<Interface>> publisher_;
  base::OnceClosure on_last_client_callback_;
};

}  // namespace base

#endif  // BASE_FUCHSIA_SCOPED_SERVICE_BINDING_H_
