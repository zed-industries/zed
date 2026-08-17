// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ENCRYPTEDMEDIA_MEDIA_KEYS_GET_STATUS_FOR_POLICY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ENCRYPTEDMEDIA_MEDIA_KEYS_GET_STATUS_FOR_POLICY_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class MediaKeys;
class MediaKeysPolicy;
class ScriptState;
class V8MediaKeyStatus;

class MediaKeysGetStatusForPolicy {
  STATIC_ONLY(MediaKeysGetStatusForPolicy);

 public:
  static ScriptPromise<V8MediaKeyStatus> getStatusForPolicy(
      ScriptState*,
      MediaKeys&,
      const MediaKeysPolicy*,
      ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ENCRYPTEDMEDIA_MEDIA_KEYS_GET_STATUS_FOR_POLICY_H_
