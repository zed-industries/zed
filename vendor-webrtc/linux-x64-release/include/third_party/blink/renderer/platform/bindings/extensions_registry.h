// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_EXTENSIONS_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_EXTENSIONS_REGISTRY_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ScriptState;

class PLATFORM_EXPORT ExtensionsRegistry {
 public:
  static ExtensionsRegistry& GetInstance();

  ~ExtensionsRegistry() = default;

  using InstallExtensionFuncType = void (*)(ScriptState*);
  void RegisterBlinkExtensionInstallCallback(InstallExtensionFuncType callback);

  void InstallExtensions(ScriptState* script_state);

 private:
  ExtensionsRegistry() = default;

  Vector<InstallExtensionFuncType> install_funcs_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_EXTENSIONS_REGISTRY_H_
