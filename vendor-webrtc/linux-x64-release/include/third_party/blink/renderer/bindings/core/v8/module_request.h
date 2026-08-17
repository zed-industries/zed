// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_MODULE_REQUEST_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_MODULE_REQUEST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// An import attribute key/value pair per spec:
// https://tc39.es/proposal-import-attributes/
struct ImportAttribute {
  String key;
  String value;
  TextPosition position;
  ImportAttribute(const String& key,
                  const String& value,
                  const TextPosition& position)
      : key(key), value(value), position(position) {}
};

// An instance of a ModuleRequest record:
// https://tc39.es/proposal-import-attributes/#sec-modulerequest-record
// Represents a module script's request to import a module given a specifier and
// list of import attributes.
struct CORE_EXPORT ModuleRequest {
  String specifier;
  TextPosition position;
  Vector<ImportAttribute> import_attributes;
  ModuleRequest(const String& specifier,
                const TextPosition& position,
                const Vector<ImportAttribute>& import_attributes)
      : specifier(specifier),
        position(position),
        import_attributes(import_attributes) {}

  String GetModuleTypeString() const;

  bool HasInvalidImportAttributeKey(String* invalid_key) const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_MODULE_REQUEST_H_
