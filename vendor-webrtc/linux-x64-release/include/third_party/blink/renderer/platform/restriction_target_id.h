// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_RESTRICTION_TARGET_ID_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_RESTRICTION_TARGET_ID_H_

#include "base/token.h"
#include "base/types/strong_alias.h"

namespace blink {

// RestrictionTarget is a JS-exposed object defined in:
// https://screen-share.github.io/element-capture/#dom-restrictiontarget
// The RestrictionTargetId is the type that's backing the JS-exposed object
// in Chromium's implementation.
using RestrictionTargetId =
    base::StrongAlias<class RestrictionTargetIdTag, base::Token>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_RESTRICTION_TARGET_ID_H_
