// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUCHSIA_SCOPED_SERVICE_PUBLISHER_H_
#define BASE_FUCHSIA_SCOPED_SERVICE_PUBLISHER_H_

#include <lib/async/dispatcher.h>
#include <lib/fidl/cpp/interface_request.h>
#include <lib/fidl/cpp/wire/connect_service.h>
#include <lib/sys/cpp/outgoing_directory.h>
#include <lib/vfs/cpp/pseudo_dir.h>
#include <lib/vfs/cpp/service.h>
#include <lib/zx/channel.h>

#include <memory>
#include <string>
#include <string_view>
#include <utility>

#include "base/base_export.h"
#include "base/fuchsia/fuchsia_logging.h"
#include "base/memory/raw_ptr.h"

namespace base {

template <typename Interface>
class BASE_EXPORT ScopedServicePublisher {
 public:
  // Publishes a public service in the specified |outgoing_directory|.
  // |outgoing_directory| and |handler| must outlive the binding.
  ScopedServicePublisher(sys::OutgoingDirectory* outgoing_directory,
                         fidl::InterfaceRequestHandler<Interface> handler,
                         std::string_view name = Interface::Name_)
      : ScopedServicePublisher(outgoing_directory->GetOrCreateDirectory("svc"),
                               std::move(handler),
                               name) {}

  // Publishes a service in the specified |pseudo_dir|. |pseudo_dir| and
  // |handler| must outlive the binding.
  ScopedServicePublisher(vfs::PseudoDir* pseudo_dir,
                         fidl::InterfaceRequestHandler<Interface> handler,
                         std::string_view name = Interface::Name_)
      : pseudo_dir_(pseudo_dir), name_(name) {
    zx_status_t status = pseudo_dir_->AddEntry(
        name_, std::make_unique<vfs::Service>(std::move(handler)));
    ZX_DCHECK(status == ZX_OK, status) << "vfs::PseudoDir::AddEntry";
  }

  ScopedServicePublisher(const ScopedServicePublisher&) = delete;
  ScopedServicePublisher& operator=(const ScopedServicePublisher&) = delete;

  ~ScopedServicePublisher() { pseudo_dir_->RemoveEntry(name_); }

 private:
  const raw_ptr<vfs::PseudoDir> pseudo_dir_ = nullptr;
  std::string name_;
};

template <typename Protocol>
class BASE_EXPORT ScopedNaturalServicePublisher {
 public:
  // Publishes a public service in the specified |outgoing_directory|.
  // |outgoing_directory| and |handler| must outlive the binding. The service is
  // unpublished on destruction.
  ScopedNaturalServicePublisher(
      sys::OutgoingDirectory* outgoing_directory,
      fidl::ProtocolHandler<Protocol> handler,
      std::string_view name = fidl::DiscoverableProtocolName<Protocol>)
      : ScopedNaturalServicePublisher(
            outgoing_directory->GetOrCreateDirectory("svc"),
            std::move(handler),
            name) {}

  // Publishes a service in the specified |pseudo_dir|. |pseudo_dir| and
  // |handler| must outlive the binding. The service is unpublished on
  // destruction.
  ScopedNaturalServicePublisher(
      vfs::PseudoDir* pseudo_dir,
      fidl::ProtocolHandler<Protocol> handler,
      std::string_view name = fidl::DiscoverableProtocolName<Protocol>)
      : pseudo_dir_(pseudo_dir), name_(name) {
    zx_status_t status = pseudo_dir_->AddEntry(
        name_, std::make_unique<vfs::Service>(
                   [handler = std::move(handler)](
                       zx::channel channel, async_dispatcher_t* dispatcher) {
                     handler(fidl::ServerEnd<Protocol>(std::move(channel)));
                   }));
    ZX_DCHECK(status == ZX_OK, status) << "vfs::PseudoDir::AddEntry";
  }

  ScopedNaturalServicePublisher(const ScopedNaturalServicePublisher&) = delete;
  ScopedNaturalServicePublisher& operator=(
      const ScopedNaturalServicePublisher&) = delete;

  ~ScopedNaturalServicePublisher() { pseudo_dir_->RemoveEntry(name_); }

 private:
  const raw_ptr<vfs::PseudoDir> pseudo_dir_ = nullptr;
  std::string name_;
};

}  // namespace base

#endif  // BASE_FUCHSIA_SCOPED_SERVICE_PUBLISHER_H_
