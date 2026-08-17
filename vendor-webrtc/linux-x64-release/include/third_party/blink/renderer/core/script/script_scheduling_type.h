// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_SCRIPT_SCHEDULING_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_SCRIPT_SCHEDULING_TYPE_H_

namespace blink {

// Type of <script>'s scheduling.
//
// This is determined by Steps 31-32 of
// https://html.spec.whatwg.org/C/#prepare-the-script-element.
//
// The enum values are used in histograms and thus do not change existing
// enum values when modifying.
enum class ScriptSchedulingType {
  // Because the scheduling type is determined slightly after PendingScript
  // creation, it is set to kNotSet before ScriptLoader::TakePendingScript()
  // is called. kNotSet should not be observed.
  kNotSet,

  // Deferred scripts controlled by HTMLParserScriptRunner.
  //
  // Spec: Step 31.4.
  //
  // Examples:
  // - <script defer> (parser inserted)
  // - <script type="module"> (parser inserted)
  kDefer,

  // Parser-blocking external scripts controlled by XML/HTMLParserScriptRunner.
  //
  // Spec: Step 31.5.
  //
  // Examples:
  // - <script> (parser inserted)
  kParserBlocking,

  // Parser-blocking inline scripts controlled by XML/HTMLParserScriptRunner.
  //
  // Spec: Step 32.2.
  kParserBlockingInline,

  // In-order scripts controlled by ScriptRunner.
  //
  // Spec: Step 31.3.
  //
  // Examples (either classic or module):
  // - Dynamically inserted <script>s with s.async = false
  kInOrder,

  // Async scripts controlled by ScriptRunner.
  //
  // Spec: Step 31.2.
  //
  // Examples (either classic or module):
  // - <script async> and <script async defer> (parser inserted)
  // - Dynamically inserted <script>s
  // - Dynamically inserted <script>s with s.async = true
  kAsync,

  // Inline <script> executed immediately within prepare-the-script-element.
  // Spec: Step 32.3.
  kImmediate,

  // Force deferred scripts controlled by HTMLParserScriptRunner.
  // These are otherwise parser-blocking scripts that are being forced to
  // execute after parsing completes (due to ForceDeferScriptIntervention).
  //
  // Spec: not yet spec'ed. https://crbug.com/976061 https://crbug.com/1339112
  // kDeprecatedForceDefer is deprecated, but kept here to ensure metrics are
  // recorded correctly in-order.
  kDeprecatedForceDefer,

  // Force in-order scripts controlled by ScriptRunner.
  //
  // Spec: not yet spec'ed. https://crbug.com/1344772
  kForceInOrder,

  kMaxValue = kForceInOrder,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_SCRIPT_SCHEDULING_TYPE_H_
