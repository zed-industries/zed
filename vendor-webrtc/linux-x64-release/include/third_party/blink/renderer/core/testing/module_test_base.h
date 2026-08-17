// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_MODULE_TEST_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_MODULE_TEST_BASE_H_

#include <gtest/gtest.h>
#include "base/test/scoped_feature_list.h"
#include "third_party/blink/public/common/features.h"
#include "third_party/blink/renderer/bindings/core/v8/script_evaluation_result.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "v8/include/v8.h"

namespace blink {

class ModuleTestBase {
 public:
  void SetUp() {}
  void TearDown() {}

  // Get the results of a ScriptEvaluationResult from a module.
  // If top-level await is enabled, the method will wait for the result
  // Promise to be resolved.
  v8::Local<v8::Value> GetResult(ScriptState* script_state,
                                 ScriptEvaluationResult result);
  // Get the exception of a ScriptEvaluationResult from a module.
  // If top-level await is enabled, the method will wait for the result
  // Promise to be rejected.
  v8::Local<v8::Value> GetException(ScriptState* script_state,
                                    ScriptEvaluationResult result);

  static v8::Local<v8::Module> CompileModule(ScriptState*,
                                             const char*,
                                             const KURL&);
  static v8::Local<v8::Module> CompileModule(ScriptState*, String, const KURL&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_MODULE_TEST_BASE_H_
