// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MODULE_PENDING_SCRIPT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MODULE_PENDING_SCRIPT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/script/modulator.h"
#include "third_party/blink/renderer/core/script/module_script.h"
#include "third_party/blink/renderer/core/script/pending_script.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ModulePendingScript;

// ModulePendingScriptTreeClient is used to connect from Modulator::FetchTree()
// to ModulePendingScript. Because ModulePendingScript is created after
// Modulator::FetchTree() is called, ModulePendingScriptTreeClient is
// registered as ModuleTreeClient to FetchTree() first, and later
// ModulePendingScript is supplied to ModulePendingScriptTreeClient via
// SetPendingScript() and is notified of module tree load finish.
class ModulePendingScriptTreeClient final : public ModuleTreeClient {
 public:
  ModulePendingScriptTreeClient();
  ~ModulePendingScriptTreeClient() override = default;

  void SetPendingScript(ModulePendingScript* client);

  ModuleScript* GetModuleScript() const { return module_script_.Get(); }

  void Trace(Visitor*) const override;

 private:
  // Implements ModuleTreeClient
  void NotifyModuleTreeLoadFinished(ModuleScript*) override;

  bool finished_ = false;
  Member<ModuleScript> module_script_;
  Member<ModulePendingScript> pending_script_;
};

// PendingScript for a module script
// https://html.spec.whatwg.org/C/#module-script.
class CORE_EXPORT ModulePendingScript : public PendingScript {
 public:
  ModulePendingScript(ScriptElementBase*,
                      ModulePendingScriptTreeClient*,
                      bool is_external,
                      scheduler::TaskAttributionInfo* parent_task);
  ~ModulePendingScript() override;

  void NotifyModuleTreeLoadFinished();

  ModuleScript* GetModuleScript() const {
    return module_tree_client_->GetModuleScript();
  }

  void Trace(Visitor*) const override;

 private:
  // PendingScript
  mojom::blink::ScriptType GetScriptType() const override {
    return mojom::blink::ScriptType::kModule;
  }
  Script* GetSource() const override;
  bool IsReady() const override { return ready_; }
  bool IsExternal() const override { return is_external_; }
  bool WasCanceled() const override { return false; }

  KURL UrlForTracing() const override { return NullURL(); }

  void DisposeInternal() override;

  void CheckState() const override {}

  Member<ModulePendingScriptTreeClient> module_tree_client_;
  bool ready_ = false;
  const bool is_external_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MODULE_PENDING_SCRIPT_H_
