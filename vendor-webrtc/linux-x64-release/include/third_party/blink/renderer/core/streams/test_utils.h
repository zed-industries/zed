// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Useful utilities for testing streams.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_TEST_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_TEST_UTILS_H_

namespace blink {

class ScriptValue;
class V8TestingScope;

// Evaluate the Javascript in "script" and return the result, failing the test
// if an exception is thrown.
ScriptValue EvalWithPrintingError(V8TestingScope*, const char* script);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_TEST_UTILS_H_
