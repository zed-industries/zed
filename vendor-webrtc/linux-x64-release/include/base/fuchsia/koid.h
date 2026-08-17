// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUCHSIA_KOID_H_
#define BASE_FUCHSIA_KOID_H_

#include <lib/zx/object.h>
#include <zircon/types.h>

#include <optional>

#include "base/base_export.h"

namespace base {

// Returns the Kernel Object IDentifier for the object referred to by |handle|,
// if it is valid.
BASE_EXPORT std::optional<zx_koid_t> GetKoid(const zx::object_base& handle);

// Returns the Kernel Object IDentifier for the peer of the paired object (i.e.
// a channel, socket, eventpair, etc) referred to by |handle|.
BASE_EXPORT std::optional<zx_koid_t> GetRelatedKoid(
    const zx::object_base& handle);

}  // namespace base

#endif  // BASE_FUCHSIA_KOID_H_
