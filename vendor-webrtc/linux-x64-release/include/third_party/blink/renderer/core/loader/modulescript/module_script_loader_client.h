// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_SCRIPT_LOADER_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_SCRIPT_LOADER_CLIENT_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ModuleScript;

// A ModuleScriptLoaderClient is notified when a single module script load is
// complete.
// Note: Its corresponding module map entry is typically not yet created at the
// time of callback.
class ModuleScriptLoaderClient : public GarbageCollectedMixin {
 public:
  virtual ~ModuleScriptLoaderClient() = default;

 private:
  friend class ModuleScriptLoader;
  friend class ModuleMapTestModulator;

  virtual void NotifyNewSingleModuleFinished(ModuleScript*) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_SCRIPT_LOADER_CLIENT_H_
