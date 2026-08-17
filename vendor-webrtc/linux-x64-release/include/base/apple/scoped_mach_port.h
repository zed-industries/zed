// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_APPLE_SCOPED_MACH_PORT_H_
#define BASE_APPLE_SCOPED_MACH_PORT_H_

#include <mach/mach.h>

#include <optional>

#include "base/base_export.h"
#include "base/scoped_generic.h"

namespace base::apple {

namespace internal {

struct BASE_EXPORT SendRightTraits {
  static mach_port_t InvalidValue() { return MACH_PORT_NULL; }

  BASE_EXPORT static void Free(mach_port_t port);
};

struct BASE_EXPORT ReceiveRightTraits {
  static mach_port_t InvalidValue() { return MACH_PORT_NULL; }

  BASE_EXPORT static void Free(mach_port_t port);
};

struct PortSetTraits {
  static mach_port_t InvalidValue() { return MACH_PORT_NULL; }

  BASE_EXPORT static void Free(mach_port_t port);
};

}  // namespace internal

// A scoper for handling a Mach port that names a send right. Send rights are
// reference counted, and this takes ownership of the right on construction
// and then removes a reference to the right on destruction. If the reference
// is the last one on the right, the right is deallocated.
using ScopedMachSendRight =
    ScopedGeneric<mach_port_t, internal::SendRightTraits>;

// A scoper for handling a Mach port's receive right. There is only one
// receive right per port. This takes ownership of the receive right on
// construction and then destroys the right on destruction, turning all
// outstanding send rights into dead names.
using ScopedMachReceiveRight =
    ScopedGeneric<mach_port_t, internal::ReceiveRightTraits>;

// A scoper for handling a Mach port set. A port set can have only one
// reference. This takes ownership of that single reference on construction and
// destroys the port set on destruction. Destroying a port set does not destroy
// the receive rights that are members of the port set.
using ScopedMachPortSet = ScopedGeneric<mach_port_t, internal::PortSetTraits>;

// Constructs a Mach port receive right and places the result in |receive|.
// If |send| is non-null, a send right will be created as well and stored
// there. If |queue_limit| is specified, the receive right will be constructed
// with the specified mpo_qlmit. Returns true on success and false on failure.
BASE_EXPORT bool CreateMachPort(
    ScopedMachReceiveRight* receive,
    ScopedMachSendRight* send,
    std::optional<mach_port_msgcount_t> queue_limit = std::nullopt);

// Increases the user reference count for MACH_PORT_RIGHT_SEND by 1 and returns
// a new scoper to manage the additional right.
BASE_EXPORT ScopedMachSendRight RetainMachSendRight(mach_port_t port);

}  // namespace base::apple

#endif  // BASE_APPLE_SCOPED_MACH_PORT_H_
