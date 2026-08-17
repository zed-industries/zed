// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_EXCEPTION_METADATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_EXCEPTION_METADATA_H_

#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

void MaybeAssociateExceptionMetaData(v8::Local<v8::Value>,
                                     const String& key,
                                     const String& value);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_EXCEPTION_METADATA_H_
