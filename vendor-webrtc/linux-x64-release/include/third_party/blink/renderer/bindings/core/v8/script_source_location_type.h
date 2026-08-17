// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_SOURCE_LOCATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_SOURCE_LOCATION_TYPE_H_

namespace blink {

// An enumeration for reporting the source of javascript source code. Only used
// for logging/stats purposes, and shouldn't be used for behavioural differences
// to avoid layer violations.
// TODO(leszeks): This is actually being used for affecting the behaviour of
// inline script caching currently, we should either update this documentation
// or remove that usage.
enum class ScriptSourceLocationType {
  // An unknown or unspecified source.
  kUnknown,

  // An external source file. Always has a corresponding ScriptResource, and
  // created by ClassicPendingScript.
  kExternalFile,

  // Values for source code that is inline inside a script tag. Never have a
  // corresponding ScriptResource, and created by ClassicPendingScript.

  // Source inline inside a script tag created normally by the parser.
  kInline,
  // Source inline inside a script tag created by document.write.
  kInlineInsideDocumentWrite,
  // Source inline inside a script tag created programmatically, rather than
  // by the parser.
  kInlineInsideGeneratedElement,

  // Other values. Never have a ScriptResource and source string is created
  // outside of ClassicPendingScript.

  // A chrome-internal source.
  kInternal,
  // A javascript url, of the form "javascript:<source>".
  kJavascriptUrl,
  // Source from a string passed to a scheduled action (e.g. setTimeout).
  kEvalForScheduledAction,
  // Source from a string evaluated by the inspector.
  kInspector
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_SOURCE_LOCATION_TYPE_H_
