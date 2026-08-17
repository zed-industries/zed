// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUCHSIA_TEST_INTERFACE_IMPL_H_
#define BASE_FUCHSIA_TEST_INTERFACE_IMPL_H_

#include <lib/fidl/cpp/binding_set.h>
#include <lib/zx/channel.h>
#include <zircon/types.h>

#include "base/testfidl/cpp/fidl.h"

namespace base {

class TestInterfaceImpl : public testfidl::TestInterface {
 public:
  TestInterfaceImpl();
  ~TestInterfaceImpl() override;

  // TestInterface implementation:
  void Add(int32_t a, int32_t b, AddCallback callback) override;

  fidl::BindingSet<testfidl::TestInterface>& bindings() { return bindings_; }

 private:
  fidl::BindingSet<testfidl::TestInterface> bindings_;
};

// Exercises the `TestInterface` channel identified by `ptr`, returning
// `ZX_OK` on success. Any error-handler for `ptr` will be removed before this
// function returns.
zx_status_t VerifyTestInterface(
    fidl::InterfacePtr<testfidl::TestInterface>& ptr);

}  // namespace base

#endif  // BASE_FUCHSIA_TEST_INTERFACE_IMPL_H_
