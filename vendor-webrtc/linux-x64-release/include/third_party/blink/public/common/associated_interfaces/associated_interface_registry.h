// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_ASSOCIATED_INTERFACES_ASSOCIATED_INTERFACE_REGISTRY_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_ASSOCIATED_INTERFACES_ASSOCIATED_INTERFACE_REGISTRY_H_

#include <map>
#include <string>
#include <type_traits>
#include <vector>

#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/memory/weak_ptr.h"
#include "mojo/public/cpp/bindings/pending_associated_receiver.h"
#include "mojo/public/cpp/bindings/scoped_interface_endpoint_handle.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {

// An AssociatedInterfaceRegistry is a collection of associated interface-
// binding callbacks mapped by interface name.
//
// This is used to register binding callbacks for interfaces which must be
// associated with some IPC::ChannelProxy, meaning that messages on the
// interface retain FIFO with respect to legacy Chrome IPC messages sent or
// dispatched on the channel.
//
// The channel with which a registered interface is associated depends on the
// configuration of the specific AssociatedInterfaceRegistry instance. For
// example, RenderFrame exposes an instance of this class for which all
// interfaces are associated with the IPC::SyncChannel to the browser.
class BLINK_COMMON_EXPORT AssociatedInterfaceRegistry {
 public:
  using Binder =
      base::RepeatingCallback<void(mojo::ScopedInterfaceEndpointHandle)>;

  AssociatedInterfaceRegistry();
  AssociatedInterfaceRegistry(const AssociatedInterfaceRegistry&) = delete;
  AssociatedInterfaceRegistry& operator=(const AssociatedInterfaceRegistry&) =
      delete;
  ~AssociatedInterfaceRegistry();

  // Adds an interface binder to the registry.
  void AddInterface(const std::string& name, const Binder& binder);

  // Removes an interface binder from the registry.
  void RemoveInterface(const std::string& name);

  // Attempts to bind an interface named |interface_name| using a registered
  // binder. If no matching binder exists, this returns |false| and |*handle| is
  // untouched. Otherwise this returns |true| and ownership of |*handle| is
  // taken.
  bool TryBindInterface(const std::string& name,
                        mojo::ScopedInterfaceEndpointHandle* handle);

  template <typename Interface>
  using ReceiverBinder =
      base::RepeatingCallback<void(mojo::PendingAssociatedReceiver<Interface>)>;

  template <typename Interface>
  void AddInterface(
      const std::common_type_t<ReceiverBinder<Interface>&> binder) {
    AddInterface(
        Interface::Name_,
        base::BindRepeating(&BindInterfaceReceiver<Interface>, binder));
  }

  void GetInterfacesForTesting(std::vector<std::string>& out) {
    for (const auto& [key, _] : interfaces_) {
      out.push_back(key);
    }
  }

  base::WeakPtr<AssociatedInterfaceRegistry> GetWeakPtr();

 private:
  template <typename Interface>
  static void BindInterfaceReceiver(
      const ReceiverBinder<Interface>& binder,
      mojo::ScopedInterfaceEndpointHandle handle) {
    binder.Run(mojo::PendingAssociatedReceiver<Interface>(std::move(handle)));
  }

  std::map<std::string, Binder> interfaces_;
  base::WeakPtrFactory<AssociatedInterfaceRegistry> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_ASSOCIATED_INTERFACES_ASSOCIATED_INTERFACE_REGISTRY_H_
