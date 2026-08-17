// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRIVATE_ATTRIBUTION_PRIVATE_ATTRIBUTION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRIVATE_ATTRIBUTION_PRIVATE_ATTRIBUTION_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class ExceptionState;
class PrivateAttributionEncryptedMatchKey;
class PrivateAttributionNetwork;
class PrivateAttributionOptions;
class ScriptState;

// Interoperable Private Attribution (IPA) is a new web platform API for
// advertising attribution.
//
// It proposes two new user-agent APIs: `get_encrypted_match_key` and
// `get_helper_networks`.
//
// The match keys which leave the user agent are always encrypted towards a
// privacy preserving measurement system, i.e., a distributed multi-party
// computation (MPC) operated by helper parties who are only trusted to not
// collude.
//
// PrivateAttribution object exposes these APIs to the window and is responsible
// for accepting the parameters, calling browser functions and returning the
// results back to the api caller.
class PrivateAttribution final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit PrivateAttribution();
  ~PrivateAttribution() final = default;

  static ScriptPromise<PrivateAttributionEncryptedMatchKey>
  getEncryptedMatchKey(ScriptState*,
                       WTF::String report_collector,
                       PrivateAttributionOptions* options,
                       ExceptionState& exception_state);

  static ScriptPromise<IDLSequence<PrivateAttributionNetwork>>
  getHelperNetworks(ScriptState*, ExceptionState& exception_state);

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRIVATE_ATTRIBUTION_PRIVATE_ATTRIBUTION_H_
