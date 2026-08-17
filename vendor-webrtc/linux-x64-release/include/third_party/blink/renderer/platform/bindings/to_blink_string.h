// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_TO_BLINK_STRING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_TO_BLINK_STRING_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8-forward.h"

namespace blink {

enum ExternalMode { kExternalize, kDoNotExternalize };

template <typename StringType>
PLATFORM_EXPORT StringType ToBlinkString(v8::Isolate* isolate,
                                         v8::Local<v8::String>,
                                         ExternalMode);

PLATFORM_EXPORT String ToBlinkString(int value);

// This method is similar to ToBlinkString() except when the underlying
// v8::String cannot be externalized (often happens with short strings like "id"
// on 64-bit platforms where V8 uses pointer compression) the v8::String is
// copied into the given StringView::StackBackingStore which avoids creating an
// AtomicString unnecessarily.
//
// The returned StringView is guaranteed to be valid as long as `backing_store`
// and `v8_string` are alive.
PLATFORM_EXPORT StringView
ToBlinkStringView(v8::Isolate* isolate,
                  v8::Local<v8::String> v8_string,
                  StringView::StackBackingStore& backing_store,
                  ExternalMode external);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_TO_BLINK_STRING_H_
