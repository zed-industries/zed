// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_DELETE_METHOD_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_DELETE_METHOD_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/shared_storage/shared_storage_modifier_method.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class SharedStorageModifierMethodOptions;

class MODULES_EXPORT SharedStorageDeleteMethod
    : public SharedStorageModifierMethod {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static SharedStorageDeleteMethod* Create(ScriptState*,
                                           const String& key,
                                           ExceptionState&);
  static SharedStorageDeleteMethod* Create(
      ScriptState*,
      const String& key,
      const SharedStorageModifierMethodOptions*,
      ExceptionState&);

  SharedStorageDeleteMethod(ScriptState*,
                            const String& key,
                            const SharedStorageModifierMethodOptions*,
                            ExceptionState&);

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_DELETE_METHOD_H_
