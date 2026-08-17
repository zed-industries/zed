// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_APPLY_VIEWPORT_CHANGES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_APPLY_VIEWPORT_CHANGES_H_

#include "cc/trees/layer_tree_host_client.h"

namespace blink {

// Allow us to use the Args struct for ApplyViewportChanges method within Blink
// core.
using ApplyViewportChangesArgs = cc::ApplyViewportChangesArgs;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_APPLY_VIEWPORT_CHANGES_H_
