// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUCHSIA_FUCHSIA_COMPONENT_CONNECT_H_
#define BASE_FUCHSIA_FUCHSIA_COMPONENT_CONNECT_H_

#include <fidl/fuchsia.io/cpp/markers.h>
#include <lib/component/incoming/cpp/protocol.h>
#include <lib/fidl/cpp/wire/connect_service.h>
#include <lib/zx/result.h>

#include <string>

#include "base/base_export.h"
#include "base/fuchsia/process_context.h"

// This namespace is designed to be consistent with the `component::Connect`
// calls used within Fuchsia. Consistency keeps Fuchsia documentation more
// relevant for developers working on Chromium as well as creating less mental
// overhead when working in both domains. See go/natural-component-context for
// more details (Googlers only).
namespace base::fuchsia_component {

template <typename Protocol,
          typename = std::enable_if_t<fidl::IsProtocolV<Protocol>>>
BASE_EXPORT zx::result<> Connect(
    fidl::ServerEnd<Protocol> server_end,
    std::string name = fidl::DiscoverableProtocolName<Protocol>) {
  return component::ConnectAt<Protocol>(
      base::BorrowIncomingServiceDirectoryForProcess(), std::move(server_end),
      name);
}

template <typename Protocol,
          typename = std::enable_if_t<fidl::IsProtocolV<Protocol>>>
BASE_EXPORT zx::result<fidl::ClientEnd<Protocol>> Connect(
    std::string name = fidl::DiscoverableProtocolName<Protocol>) {
  return component::ConnectAt<Protocol>(
      base::BorrowIncomingServiceDirectoryForProcess(), name);
}

template <typename Protocol,
          typename = std::enable_if_t<fidl::IsProtocolV<Protocol>>>
BASE_EXPORT zx::result<fidl::ClientEnd<Protocol>> ConnectAt(
    fidl::UnownedClientEnd<fuchsia_io::Directory> service_directory,
    std::string name = fidl::DiscoverableProtocolName<Protocol>) {
  return component::ConnectAt<Protocol>(service_directory, name);
}

}  // namespace base::fuchsia_component

#endif  // BASE_FUCHSIA_FUCHSIA_COMPONENT_CONNECT_H_
