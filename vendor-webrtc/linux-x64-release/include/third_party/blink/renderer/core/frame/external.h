// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_EXTERNAL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_EXTERNAL_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class External : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  External() = default;

  void AddSearchProvider() {}
  void IsSearchProviderInstalled() {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_EXTERNAL_H_
