// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_STYLE_PROPERTY_MAP_READ_ONLY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_STYLE_PROPERTY_MAP_READ_ONLY_H_

#include "third_party/blink/renderer/bindings/core/v8/iterable.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_sync_iterator_style_property_map_read_only.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/cssom/css_style_value.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class CORE_EXPORT StylePropertyMapReadOnly
    : public ScriptWrappable,
      public PairSyncIterable<StylePropertyMapReadOnly> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  virtual CSSStyleValue* get(const ExecutionContext*,
                             const String& property_name,
                             ExceptionState&) const = 0;
  virtual CSSStyleValueVector getAll(const ExecutionContext*,
                                     const String& property_name,
                                     ExceptionState&) const = 0;
  virtual bool has(const ExecutionContext*,
                   const String& property_name,
                   ExceptionState&) const = 0;

  virtual unsigned int size() const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_STYLE_PROPERTY_MAP_READ_ONLY_H_
