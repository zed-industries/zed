// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SANITIZER_SANITIZER_BUILTINS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SANITIZER_SANITIZER_BUILTINS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/core/sanitizer/sanitizer.h"

namespace blink {

class CORE_EXPORT SanitizerBuiltins {
 public:
  // Default safe config, for use with setHTML and parseHTML.
  static const Sanitizer* GetDefaultSafe();
  // Default "unsafe" config, for use with setHTMLUnsafe and parseHTMLUnsafe.
  static const Sanitizer* GetDefaultUnsafe();
  // "Baseline" config, for use with Sanitizer.removeUnsafe.
  static const Sanitizer* GetBaseline();
};

// The builtin configs are generated. The methods below may do non-trivial work.
// Callers should go through the SanitizerBuiltins static methods above.
namespace sanitizer_generated_builtins {
// Default safe + baseline configs.
//
// Manually re-generate with:
// $ ninja -C ... third_party/blink/renderer/core/sanitizer:generated
CORE_EXPORT Sanitizer* BuildDefaultConfig();
CORE_EXPORT Sanitizer* BuildBaselineConfig();

// The "all-known" config is also generated, but is only used for testing.
//
// Manually re-generate with:
// $ ninja -C ... third_party/blink/renderer/core/sanitizer:unit_test_support
Sanitizer* BuildAllKnownConfig_ForTesting();

}  // namespace sanitizer_generated_builtins
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SANITIZER_SANITIZER_BUILTINS_H_
