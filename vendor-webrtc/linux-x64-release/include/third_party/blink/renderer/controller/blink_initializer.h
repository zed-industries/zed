// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CONTROLLER_BLINK_INITIALIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_CONTROLLER_BLINK_INITIALIZER_H_

#include "third_party/blink/renderer/modules/modules_initializer.h"

namespace blink {

class Platform;

class BlinkInitializer : public ModulesInitializer {
 public:
  void RegisterInterfaces(mojo::BinderMap&) override;
  void OnClearWindowObjectInMainWorld(Document&,
                                      const Settings&) const override;
  void InitLocalFrame(LocalFrame&) const override;

  void InitServiceWorkerGlobalScope(ServiceWorkerGlobalScope&) const override;
  void RegisterMemoryWatchers(Platform*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CONTROLLER_BLINK_INITIALIZER_H_
