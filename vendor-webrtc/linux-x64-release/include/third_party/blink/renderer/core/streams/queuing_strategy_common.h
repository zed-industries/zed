// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_QUEUING_STRATEGY_COMMON_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_QUEUING_STRATEGY_COMMON_H_

#include "third_party/blink/renderer/platform/bindings/v8_private_property.h"
#include "v8/include/v8.h"

namespace blink {

class ScriptState;
class ScriptValue;

using SizeFunctionFactory = v8::Local<v8::Function> (*)(ScriptState*);

// Returns the value cached on the global proxy object under |key|, or, if that
// is not set, caches and returns the result of calling |factory|.
ScriptValue GetCachedSizeFunction(ScriptState*,
                                  const V8PrivateProperty::SymbolKey& key,
                                  SizeFunctionFactory factory);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_QUEUING_STRATEGY_COMMON_H_
