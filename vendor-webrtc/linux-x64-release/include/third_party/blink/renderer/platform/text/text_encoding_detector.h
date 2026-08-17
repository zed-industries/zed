/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_TEXT_ENCODING_DETECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_TEXT_ENCODING_DETECTOR_H_

#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace WTF {
class TextEncoding;
}

namespace blink {
class KURL;

// Given a sequence of bytes in `bytes` and an optional
// `hint_encoding_name`, detect the most likely character encoding.
// The way `hint_encoding_name` is used is up to an implementation.
// Currently, the only caller sets it to the parent frame encoding.
// `hint_url` is optional. You can pass nullptr.
// `hint_user_language` is an optional language code like "fr", and can be
// nullptr.
PLATFORM_EXPORT bool DetectTextEncoding(base::span<const uint8_t> bytes,
                                        const char* hint_encoding_name,
                                        const KURL& hint_url,
                                        const char* hint_user_language,
                                        WTF::TextEncoding* detected_encoding);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_TEXT_ENCODING_DETECTOR_H_
