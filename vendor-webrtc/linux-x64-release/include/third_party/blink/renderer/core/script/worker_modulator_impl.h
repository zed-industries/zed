// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_WORKER_MODULATOR_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_WORKER_MODULATOR_IMPL_H_

#include "third_party/blink/renderer/core/script/modulator_impl_base.h"

namespace blink {

class ModuleScriptFetcher;
class ScriptState;

// WorkerModulatorImpl is the Modulator implementation used in worker contexts
// (that means, not main documents). Module operations depending on the Worker
// context should be implemented in this class, not in ModulatorImplBase.
class WorkerModulatorImpl final : public ModulatorImplBase {
 public:
  explicit WorkerModulatorImpl(ScriptState*);

  // Implements ModulatorImplBase.
  ModuleScriptFetcher* CreateModuleScriptFetcher(
      ModuleScriptCustomFetchType,
      base::PassKey<ModuleScriptLoader> pass_key) override;

 private:
  // Implements ModulatorImplBase.
  bool IsDynamicImportForbidden(String* reason) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_WORKER_MODULATOR_IMPL_H_
