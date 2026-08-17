// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_NAVIGATION_PARAMS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_NAVIGATION_PARAMS_H_

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/navigation/navigation_params.mojom-forward.h"
#include "third_party/blink/public/mojom/navigation/renderer_content_settings.mojom.h"

namespace blink {

BLINK_COMMON_EXPORT mojom::CommonNavigationParamsPtr
CreateCommonNavigationParams();
BLINK_COMMON_EXPORT mojom::CommitNavigationParamsPtr
CreateCommitNavigationParams();

// The embedder is responsible for evaluating content settings for each
// document. Default values are still useful for two reasons:
//   (1) Many tests are effectively embedders and want reasonable defaults.
//   (2) There are a few cases where a renderer will synchronously navigate e.g.
//   SynchronouslyCommitAboutBlankForBug778318. These cases should also use
//   reasonable defaults.
BLINK_COMMON_EXPORT mojom::RendererContentSettingsPtr
CreateDefaultRendererContentSettings();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_NAVIGATION_PARAMS_H_
