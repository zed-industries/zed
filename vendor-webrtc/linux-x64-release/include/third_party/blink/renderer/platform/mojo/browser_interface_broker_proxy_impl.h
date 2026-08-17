// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_BROWSER_INTERFACE_BROKER_PROXY_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_BROWSER_INTERFACE_BROKER_PROXY_IMPL_H_

#include <string_view>

#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/generic_pending_receiver.h"
#include "third_party/blink/public/mojom/browser_interface_broker.mojom-blink.h"
#include "third_party/blink/public/platform/browser_interface_broker_proxy.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ContextLifecycleNotifier;

class PLATFORM_EXPORT TestableBrowserInterfaceBrokerProxy
    : public BrowserInterfaceBrokerProxy {
 public:
  using TestBinder =
      base::RepeatingCallback<void(mojo::ScopedMessagePipeHandle)>;

  TestableBrowserInterfaceBrokerProxy(
      const TestableBrowserInterfaceBrokerProxy&) = delete;
  TestableBrowserInterfaceBrokerProxy& operator=(
      const TestableBrowserInterfaceBrokerProxy&) = delete;

  // Overrides how the named interface is bound, rather than sending its
  // receivers to the browser. If |binder| is null, any registered override
  // for the interface is cancelled.
  //
  // Returns |true| if the new binder was successfully set, or |false| if the
  // binder was non-null and an existing binder was already registered for the
  // named interface.
  bool SetBinderForTesting(const std::string& name,
                           TestBinder binder) const override;

 protected:
  TestableBrowserInterfaceBrokerProxy() = default;
  ~TestableBrowserInterfaceBrokerProxy() override = default;

  // Finds and returns a pointer to a `TestBinder` that was earlier registered
  // via `SetBinderForTesting`.  Returns null if no such binder is found.
  TestBinder* FindTestBinder(std::string_view interface_name) const;

 private:
  using BinderMap = WTF::HashMap<
      String,
      base::RepeatingCallback<void(mojo::ScopedMessagePipeHandle)>>;
  mutable BinderMap binder_map_for_testing_;
};

// BrowserInterfaceBrokerProxy provides access to interfaces exposed by the
// browser to the renderer. It is intended to replace document- and
// worker-scoped InterfaceProvider (see https://crbug.com/40519010).
class PLATFORM_EXPORT BrowserInterfaceBrokerProxyImpl
    : public TestableBrowserInterfaceBrokerProxy {
  DISALLOW_NEW();

 public:
  explicit BrowserInterfaceBrokerProxyImpl(ContextLifecycleNotifier* notifier);

  BrowserInterfaceBrokerProxyImpl(const BrowserInterfaceBrokerProxyImpl&) =
      delete;
  BrowserInterfaceBrokerProxyImpl& operator=(
      const BrowserInterfaceBrokerProxyImpl&) = delete;

  void Bind(
      CrossVariantMojoRemote<mojom::BrowserInterfaceBrokerInterfaceBase> broker,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  CrossVariantMojoReceiver<mojom::BrowserInterfaceBrokerInterfaceBase> Reset(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) override;

  // Asks the browser to bind the given receiver. If a non-null testing override
  // was set by |SetBinderForTesting()|, the request will be intercepted by that
  // binder instead of going to the browser.
  void GetInterface(mojo::GenericPendingReceiver) const override;

  bool is_bound() const;

  void Trace(Visitor*) const;

 private:
  // TODO(https://crbug.com/352165586): Stop using
  // `kForceWithoutContextObserver`.
  HeapMojoRemote<mojom::blink::BrowserInterfaceBroker,
                 HeapMojoWrapperMode::kForceWithoutContextObserver>
      broker_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_BROWSER_INTERFACE_BROKER_PROXY_IMPL_H_
