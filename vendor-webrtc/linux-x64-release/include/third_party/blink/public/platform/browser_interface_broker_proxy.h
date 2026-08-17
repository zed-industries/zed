// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_BROWSER_INTERFACE_BROKER_PROXY_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_BROWSER_INTERFACE_BROKER_PROXY_H_

#include <utility>

#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/generic_pending_receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/browser_interface_broker.mojom-shared.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"

namespace blink {

// BrowserInterfaceBrokerProxy provides access to interfaces exposed by the
// browser to the renderer. It is intended to replace document- and
// worker-scoped InterfaceProvider (see crbug.com/718652).
class BLINK_PLATFORM_EXPORT BrowserInterfaceBrokerProxy {
 public:
  BrowserInterfaceBrokerProxy();

  BrowserInterfaceBrokerProxy(const BrowserInterfaceBrokerProxy&) = delete;
  BrowserInterfaceBrokerProxy& operator=(const BrowserInterfaceBrokerProxy&) =
      delete;

  virtual ~BrowserInterfaceBrokerProxy();

  virtual CrossVariantMojoReceiver<mojom::BrowserInterfaceBrokerInterfaceBase>
  Reset(scoped_refptr<base::SingleThreadTaskRunner> task_runner) = 0;

  // Asks the browser to bind the given receiver. If a non-null testing override
  // was set by |SetBinderForTesting()|, the request will be intercepted by that
  // binder instead of going to the browser.
  virtual void GetInterface(mojo::GenericPendingReceiver) const = 0;

  // TODO(https://crbug.com/40519010): Add a presubmit check for C++ call sites.
  void GetInterface(const std::string& name,
                    mojo::ScopedMessagePipeHandle pipe) const {
    GetInterface(mojo::GenericPendingReceiver(name, std::move(pipe)));
  }

  // Overrides how the named interface is bound, rather than sending its
  // receivers to the browser. If |binder| is null, any registered override
  // for the interface is cancelled.
  //
  // Returns |true| if the new binder was successfully set, or |false| if the
  // binder was non-null and an existing binder was already registered for the
  // named interface.
  virtual bool SetBinderForTesting(
      const std::string& name,
      base::RepeatingCallback<void(mojo::ScopedMessagePipeHandle)> binder)
      const = 0;
};

// Returns an instance of BrowserInterfaceBrokerProxy that is safe to use but is
// not connected to anything.
BLINK_PLATFORM_EXPORT BrowserInterfaceBrokerProxy&
GetEmptyBrowserInterfaceBroker();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_BROWSER_INTERFACE_BROKER_PROXY_H_
