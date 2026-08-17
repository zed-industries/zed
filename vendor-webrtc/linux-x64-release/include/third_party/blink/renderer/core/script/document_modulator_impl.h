// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_DOCUMENT_MODULATOR_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_DOCUMENT_MODULATOR_IMPL_H_

#include "third_party/blink/renderer/core/script/modulator_impl_base.h"

namespace blink {

class ModuleScriptFetcher;
class ScriptState;

// DocumentModulatorImpl is the Modulator implementation used in main documents
// (that means, not worker nor worklets). Module operations depending on the
// Document context should be implemented in this class, not in
// ModulatorImplBase.
class DocumentModulatorImpl final : public ModulatorImplBase {
 public:
  explicit DocumentModulatorImpl(ScriptState*);

  // Implements Modulator.
  ModuleScriptFetcher* CreateModuleScriptFetcher(
      ModuleScriptCustomFetchType,
      base::PassKey<ModuleScriptLoader>) override;

  // https://html.spec.whatwg.org/C/#merge-existing-and-new-import-maps
  void MergeExistingAndNewImportMaps(ImportMap* import_map) override;

  // https://html.spec.whatwg.org/C#add-module-to-resolved-module-set
  void AddModuleToResolvedModuleSet(
      std::optional<AtomicString> referring_script_url,
      AtomicString specifier) override;

 private:
  // Implements ModulatorImplBase.
  bool IsDynamicImportForbidden(String* reason) override;

  // https://html.spec.whatwg.org/C#resolved-module-set
  //
  // We replace the spec's set with two different data structures: a set of all
  // the prefixes resolved at the top-level scope, and a map of scopes to sets
  // of prefixes resolved in them. That permits us to reduce the cost of merging
  // a new map, by performing more work at AddModuleToResolvedModuleSet time,
  // and by keeping more prefixes in memory.
  HashSet<AtomicString> toplevel_resolved_module_set_;
  HashMap<AtomicString, HashSet<AtomicString>> scoped_resolved_module_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_DOCUMENT_MODULATOR_IMPL_H_
