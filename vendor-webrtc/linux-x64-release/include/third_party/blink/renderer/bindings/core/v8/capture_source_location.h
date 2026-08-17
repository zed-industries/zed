// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_CAPTURE_SOURCE_LOCATION_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_CAPTURE_SOURCE_LOCATION_H_

#include <memory>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/source_location.h"

namespace blink {

class ExecutionContext;

// Gets the url of the currently executing script. Returns empty string, if no
// script is executing (e.g. during parsing of a meta tag in markup), or the
// script context is otherwise unavailable.
CORE_EXPORT String CaptureCurrentScriptUrl(v8::Isolate* isolate);

// Gets the urls of the scripts at the top of the currently executing stack.
// If available, returns up to |unique_url_count| urls, filtering out duplicate
// urls (e.g. if the stack includes multiple frames from the same script).
// Returns an empty vector, if no script is executing (e.g. during parsing of a
// meta tag in markup), or the script context is otherwise unavailable.
// To minimize the cost of walking the stack, only the top frames (currently 10)
// are examined, regardless of the value of |unique_url_count|.
CORE_EXPORT Vector<String> CaptureScriptUrlsFromCurrentStack(
    v8::Isolate* isolate,
    wtf_size_t unique_url_count);

// Shortcut when location is unknown. Tries to capture call stack or parsing
// location if available.
CORE_EXPORT std::unique_ptr<SourceLocation> CaptureSourceLocation(
    ExecutionContext*);

CORE_EXPORT std::unique_ptr<SourceLocation>
CapturePartialSourceLocationFromStack(v8::Isolate* isolate);

// Shortcut when location is unknown. Tries to capture call stack or parsing
// location using message if available.
CORE_EXPORT std::unique_ptr<SourceLocation> CaptureSourceLocation(
    v8::Isolate* isolate,
    v8::Local<v8::Message> message,
    ExecutionContext* execution_context);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_CAPTURE_SOURCE_LOCATION_H_
