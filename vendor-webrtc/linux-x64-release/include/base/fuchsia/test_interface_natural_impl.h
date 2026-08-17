// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUCHSIA_TEST_INTERFACE_NATURAL_IMPL_H_
#define BASE_FUCHSIA_TEST_INTERFACE_NATURAL_IMPL_H_

#include <fidl/base.testfidl/cpp/fidl.h>
#include <fidl/fuchsia.io/cpp/markers.h>
#include <lib/fidl/cpp/traits.h>
#include <lib/fidl/cpp/wire/connect_service.h>
#include <lib/sys/cpp/service_directory.h>

namespace base {

class TestInterfaceNaturalImpl
    : public fidl::Server<base_testfidl::TestInterface> {
 public:
  TestInterfaceNaturalImpl();
  ~TestInterfaceNaturalImpl() override;

  // TestInterface implementation:
  void Add(AddRequest& request, AddCompleter::Sync& completer) override;

  fidl::ServerBindingGroup<base_testfidl::TestInterface>& bindings() {
    return bindings_;
  }

 private:
  fidl::ServerBindingGroup<base_testfidl::TestInterface> bindings_;
};

// Connects and returns a client for `TestInterface` at the specified `name`.
fidl::Client<base_testfidl::TestInterface> CreateTestInterfaceClient(
    fidl::UnownedClientEnd<fuchsia_io::Directory> service_directory,
    const std::string& name =
        fidl::DiscoverableProtocolName<base_testfidl::TestInterface>);

// Exercises the `TestInterface` channel identified by `client`, returning
// `ZX_OK` on success. Any error-handler for `client` will be removed before
// this function returns.
zx_status_t VerifyTestInterface(
    fidl::Client<base_testfidl::TestInterface>& client);

}  // namespace base

#endif  // BASE_FUCHSIA_TEST_INTERFACE_NATURAL_IMPL_H_
