// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_INTERFACE_REGISTRY_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_INTERFACE_REGISTRY_H_

#include <utility>

#include "base/functional/callback_forward.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/scoped_interface_endpoint_handle.h"
#include "mojo/public/cpp/system/message_pipe.h"
#include "third_party/blink/public/platform/web_common.h"

#if INSIDE_BLINK
#include "mojo/public/cpp/bindings/pending_associated_receiver.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_functional.h"  // nogncheck
#include "third_party/blink/renderer/platform/wtf/functional.h"  // nogncheck
#endif

namespace blink {

using InterfaceFactory =
    base::RepeatingCallback<void(mojo::ScopedMessagePipeHandle)>;
using AssociatedInterfaceFactory =
    base::RepeatingCallback<void(mojo::ScopedInterfaceEndpointHandle)>;

class BLINK_PLATFORM_EXPORT InterfaceRegistry {
 public:
  virtual void AddInterface(
      const char* name,
      const InterfaceFactory&,
      scoped_refptr<base::SingleThreadTaskRunner> = nullptr) = 0;
  // The usage of associated interfaces should be very limited. Please
  // consult the owners of public/platform before adding one.
  virtual void AddAssociatedInterface(const char* name,
                                      const AssociatedInterfaceFactory&) = 0;

  static InterfaceRegistry* GetEmptyInterfaceRegistry();

#if INSIDE_BLINK
  template <typename Interface>
  void AddInterface(
      base::RepeatingCallback<void(mojo::PendingReceiver<Interface>)> factory) {
    AddInterface(
        Interface::Name_,
        WTF::BindRepeating(&InterfaceRegistry::ForwardToInterfaceFactory<
                               mojo::PendingReceiver<Interface>>,
                           std::move(factory)));
  }

  template <typename Interface>
  void AddInterface(
      base::RepeatingCallback<void(mojo::PendingReceiver<Interface>)> factory,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner) {
    DCHECK(task_runner->RunsTasksInCurrentSequence());
    AddInterface(
        Interface::Name_,
        WTF::BindRepeating(&InterfaceRegistry::ForwardToInterfaceFactory<
                               mojo::PendingReceiver<Interface>>,
                           std::move(factory)),
        std::move(task_runner));
  }

  template <typename Interface>
  void AddAssociatedInterface(
      base::RepeatingCallback<void(mojo::PendingAssociatedReceiver<Interface>)>
          factory) {
    AddAssociatedInterface(
        Interface::Name_,
        WTF::BindRepeating(
            &InterfaceRegistry::ForwardToAssociatedInterfaceFactory<
                mojo::PendingAssociatedReceiver<Interface>>,
            std::move(factory)));
  }

 private:
  template <typename MojoType>
  static void ForwardToInterfaceFactory(
      base::RepeatingCallback<void(MojoType)> factory,
      mojo::ScopedMessagePipeHandle handle) {
    factory.Run(MojoType(std::move(handle)));
  }

  template <typename MojoType>
  static void ForwardToAssociatedInterfaceFactory(
      base::RepeatingCallback<void(MojoType)> factory,
      mojo::ScopedInterfaceEndpointHandle handle) {
    factory.Run(MojoType(std::move(handle)));
  }

#endif  // INSIDE_BLINK
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_INTERFACE_REGISTRY_H_
