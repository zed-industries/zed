// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LCP_CRITICAL_PATH_PREDICTOR_LCP_SCRIPT_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LCP_CRITICAL_PATH_PREDICTOR_LCP_SCRIPT_OBSERVER_H_

#include <stdint.h>

#include "base/feature_list.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/core_probes_inl.h"

namespace blink {
class LocalFrame;

namespace probe {
class ExecuteScript;
}  // namespace probe

// A probe that keeps track of script execution to help determine LCP element
// dependencies.
class CORE_EXPORT LCPScriptObserver
    : public GarbageCollected<LCPScriptObserver> {
 public:
  explicit LCPScriptObserver(LocalFrame*);
  LCPScriptObserver(const LCPScriptObserver&) = delete;
  LCPScriptObserver& operator=(const LCPScriptObserver&) = delete;
  virtual ~LCPScriptObserver();
  void Shutdown();

  virtual void Trace(Visitor*) const;

  // Instrumenting methods.
  HashSet<String> GetExecutingScriptUrls();

  // Probes
  void Will(const probe::ExecuteScript&);
  void Did(const probe::ExecuteScript&);
  void Will(const probe::CallFunction&);
  void Did(const probe::CallFunction&);

 private:
  GC_PLUGIN_IGNORE("probe references should be valid within will/did callbacks")
  Vector<const probe::ExecuteScript*> stack_script_probes_;

  GC_PLUGIN_IGNORE("probe references should be valid within will/did callbacks")
  Vector<const probe::CallFunction*> stack_function_probes_;

  Member<LocalFrame> local_root_;
  String GetScriptUrlFromCallFunctionProbe(const probe::CallFunction* probe);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LCP_CRITICAL_PATH_PREDICTOR_LCP_SCRIPT_OBSERVER_H_
