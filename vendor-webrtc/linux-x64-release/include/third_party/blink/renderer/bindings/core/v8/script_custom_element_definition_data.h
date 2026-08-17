// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_CUSTOM_ELEMENT_DEFINITION_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_CUSTOM_ELEMENT_DEFINITION_DATA_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class CustomElementRegistry;
class ScriptState;
class V8CustomElementAdoptedCallback;
class V8CustomElementAttributeChangedCallback;
class V8CustomElementConstructor;
class V8CustomElementFormAssociatedCallback;
class V8CustomElementFormDisabledCallback;
class V8CustomElementFormStateRestoreCallback;
class V8VoidFunction;

class ScriptCustomElementDefinitionData {
  STACK_ALLOCATED();

 public:
  ScriptCustomElementDefinitionData() {}

  ScriptCustomElementDefinitionData(const ScriptCustomElementDefinitionData&) =
      delete;
  ScriptCustomElementDefinitionData& operator=(
      const ScriptCustomElementDefinitionData&) = delete;

  ScriptState* script_state_ = nullptr;
  CustomElementRegistry* registry_ = nullptr;
  V8CustomElementConstructor* constructor_ = nullptr;
  V8VoidFunction* connected_callback_ = nullptr;
  V8VoidFunction* disconnected_callback_ = nullptr;
  V8VoidFunction* connected_move_callback_ = nullptr;
  V8CustomElementAdoptedCallback* adopted_callback_ = nullptr;
  V8CustomElementAttributeChangedCallback* attribute_changed_callback_ =
      nullptr;
  V8CustomElementFormAssociatedCallback* form_associated_callback_ = nullptr;
  V8VoidFunction* form_reset_callback_ = nullptr;
  V8CustomElementFormDisabledCallback* form_disabled_callback_ = nullptr;
  V8CustomElementFormStateRestoreCallback* form_state_restore_callback_ =
      nullptr;
  HashSet<AtomicString> observed_attributes_;
  Vector<String> disabled_features_;
  bool is_form_associated_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_CUSTOM_ELEMENT_DEFINITION_DATA_H_
