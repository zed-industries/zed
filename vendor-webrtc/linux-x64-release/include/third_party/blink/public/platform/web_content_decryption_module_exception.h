// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CONTENT_DECRYPTION_MODULE_EXCEPTION_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CONTENT_DECRYPTION_MODULE_EXCEPTION_H_

namespace blink {

// From https://w3c.github.io/encrypted-media/#exceptions.
enum WebContentDecryptionModuleException {
  kWebContentDecryptionModuleExceptionTypeError,
  kWebContentDecryptionModuleExceptionNotSupportedError,
  kWebContentDecryptionModuleExceptionInvalidStateError,
  kWebContentDecryptionModuleExceptionQuotaExceededError,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_CONTENT_DECRYPTION_MODULE_EXCEPTION_H_
