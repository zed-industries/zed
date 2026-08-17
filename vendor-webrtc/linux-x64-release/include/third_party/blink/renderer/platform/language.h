/*
 * Copyright (C) 2003, 2006, 2010, 2013 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS''
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
 * THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LANGUAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LANGUAGE_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// Returns a BCP-47 language tag such as "en-US".  This is the UI locale of the
// browser application.
PLATFORM_EXPORT AtomicString DefaultLanguage();
// Returns a list of BCP-47 language tags.  This never returns multiple values
// in production.  This is not a value of Accept-Languages header.  See
// ChromeClient::acceptLanguages.
PLATFORM_EXPORT Vector<AtomicString> UserPreferredLanguages();
PLATFORM_EXPORT void OverrideUserPreferredLanguagesForTesting(
    const Vector<AtomicString>&);
PLATFORM_EXPORT wtf_size_t
IndexOfBestMatchingLanguageInList(const AtomicString& language,
                                  const Vector<AtomicString>& language_list);
PLATFORM_EXPORT void InitializePlatformLanguage();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LANGUAGE_H_
