// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_THREAD_SAFE_BROWSER_INTERFACE_BROKER_PROXY_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_THREAD_SAFE_BROWSER_INTERFACE_BROKER_PROXY_H_

#include <map>
#include <string>
#include <string_view>

#include "base/memory/ref_counted.h"
#include "base/synchronization/lock.h"
#include "mojo/public/cpp/bindings/generic_pending_receiver.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {

// ThreadSafeBrowserInterfaceBrokerProxy provides access to interfaces exposed
// by the browser to the renderer. It is used to access process-scoped browser
// interfaces from any thread in the render process.
class BLINK_COMMON_EXPORT ThreadSafeBrowserInterfaceBrokerProxy
    : public base::RefCountedThreadSafe<ThreadSafeBrowserInterfaceBrokerProxy> {
 public:
  ThreadSafeBrowserInterfaceBrokerProxy();
  ThreadSafeBrowserInterfaceBrokerProxy(
      const ThreadSafeBrowserInterfaceBrokerProxy&) = delete;
  ThreadSafeBrowserInterfaceBrokerProxy& operator=(
      const ThreadSafeBrowserInterfaceBrokerProxy&) = delete;

  // Asks the browser to bind the given receiver. If a non-null testing override
  // was set by |SetBinderForTesting()|, the request will be intercepted by that
  // Binder instead of going to the browser.
  void GetInterface(mojo::GenericPendingReceiver);

  // Overrides how the named interface is bound, rather than sending its
  // receivers to the browser. If the Binder is null, any registered override
  // for the interface is cancelled.
  //
  // Returns |true| if the new Binder was successfully set, or |false| if the
  // Binder was non-null and an existing Binder was already registered for the
  // named interface.
  using Binder = base::RepeatingCallback<void(mojo::ScopedMessagePipeHandle)>;
  bool SetBinderForTesting(std::string_view interface_name, Binder);

 protected:
  friend class base::RefCountedThreadSafe<
      ThreadSafeBrowserInterfaceBrokerProxy>;

  virtual ~ThreadSafeBrowserInterfaceBrokerProxy();

  virtual void GetInterfaceImpl(mojo::GenericPendingReceiver) = 0;

 private:
  base::Lock binder_map_lock_;
  std::map<std::string, Binder> binder_map_for_testing_
      GUARDED_BY(binder_map_lock_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_THREAD_SAFE_BROWSER_INTERFACE_BROKER_PROXY_H_
