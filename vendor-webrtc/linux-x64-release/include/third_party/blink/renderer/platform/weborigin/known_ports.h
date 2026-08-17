/*
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2011, 2012 Apple Inc.
 * All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_KNOWN_PORTS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_KNOWN_PORTS_H_

#include <stdint.h>

#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class KURL;

// Returns true if |port| is known to be the default for |protocol|. |protocol|
// must be lower case.
PLATFORM_EXPORT bool IsDefaultPortForProtocol(uint16_t port,
                                              const WTF::String& protocol);

// Returns 0 for unknown protocols. |protocol| must be lower case.
// Based on https://url.spec.whatwg.org/#default-port
PLATFORM_EXPORT uint16_t DefaultPortForProtocol(const WTF::String& protocol);

// Returns true if the port of the |url| is allowed for the scheme of the |url|.
PLATFORM_EXPORT bool IsPortAllowedForScheme(const KURL&);

// Sets ports as permitted that would otherwise be disallowed. Takes a
// comma-separated list of ports.
PLATFORM_EXPORT void SetExplicitlyAllowedPorts(
    base::span<const uint16_t> allowed_ports);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_KNOWN_PORTS_H_
