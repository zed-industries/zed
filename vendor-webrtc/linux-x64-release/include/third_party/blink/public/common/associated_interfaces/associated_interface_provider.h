// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_ASSOCIATED_INTERFACES_ASSOCIATED_INTERFACE_PROVIDER_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_ASSOCIATED_INTERFACES_ASSOCIATED_INTERFACE_PROVIDER_H_

#include <string>

#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/associated_remote.h"
#include "mojo/public/cpp/bindings/pending_associated_receiver.h"
#include "mojo/public/cpp/bindings/pending_associated_remote.h"
#include "mojo/public/cpp/bindings/scoped_interface_endpoint_handle.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/associated_interfaces/associated_interfaces.mojom.h"

namespace blink {

// A helper interface for connecting to remote Channel-associated interfaces.
//
// This is analogous to service_manager::InterfaceProvider in that it provides a
// means of binding proxies to remote interfaces, but this is specifically for
// interfaces which must be associated with an IPC::Channel, i.e. retain FIFO
// message ordering with respect to legacy IPC messages.
//
// The Channel with which the remote interfaces are associated depends on the
// configuration of the specific AssociatedInterfaceProvider instance. For
// example, RenderFrameHost exposes an instance of this class for which all
// interfaces are associated with the IPC::ChannelProxy to the render process
// which hosts its corresponding RenderFrame.
class BLINK_COMMON_EXPORT AssociatedInterfaceProvider {
 public:
  // Binds this to a remote mojom::AssociatedInterfaceProvider.
  //
  // |task_runner| must belong to the same thread. It will be used to dispatch
  // all callbacks and connection error notification.
  explicit AssociatedInterfaceProvider(
      mojo::PendingAssociatedRemote<mojom::AssociatedInterfaceProvider> proxy,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner = nullptr);

  // Constructs a local provider with no remote interfaces. This is useful in
  // conjunction with OverrideBinderForTesting(), in test environments where
  // there may not be a remote |mojom::AssociatedInterfaceProvider| available.
  //
  // |task_runner| must belong to the same thread. It will be used to dispatch
  // all callbacks and connection error notification.
  explicit AssociatedInterfaceProvider(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  AssociatedInterfaceProvider(const AssociatedInterfaceProvider&) = delete;
  AssociatedInterfaceProvider& operator=(const AssociatedInterfaceProvider&) =
      delete;

  ~AssociatedInterfaceProvider();

  // Passes an associated endpoint handle to the remote end to be bound to a
  // Channel-associated interface named |name|.
  void GetInterface(const std::string& name,
                    mojo::ScopedInterfaceEndpointHandle handle);

  template <typename Interface>
  void GetInterface(mojo::PendingAssociatedReceiver<Interface> receiver) {
    GetInterface(Interface::Name_, receiver.PassHandle());
  }

  template <typename Interface>
  void GetInterface(mojo::AssociatedRemote<Interface>* remote) {
    GetInterface(remote->BindNewEndpointAndPassReceiver(GetTaskRunner()));
  }

  // Returns the task runner for this provider so callers can generate pending
  // receivers if needed.
  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner() {
    return task_runner_;
  }

  // If there is an override for `name`, passing in a null `binder` removes the
  // override.
  void OverrideBinderForTesting(
      const std::string& name,
      const base::RepeatingCallback<void(mojo::ScopedInterfaceEndpointHandle)>&
          binder);

  // Returns an instance of AssociatedInterfaceProvider that is safe to use but
  // is not connected to anything.
  static AssociatedInterfaceProvider* GetEmptyAssociatedInterfaceProvider();

 private:
  class LocalProvider;

  mojo::AssociatedRemote<mojom::AssociatedInterfaceProvider> proxy_;

  std::unique_ptr<LocalProvider> local_provider_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_ASSOCIATED_INTERFACES_ASSOCIATED_INTERFACE_PROVIDER_H_
