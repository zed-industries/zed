// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_GOODS_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_GOODS_UTIL_H_

namespace WTF {
class String;
}

namespace blink {
class ScriptState;

namespace digital_goods_util {

void LogConsoleError(ScriptState*, const WTF::String& message);

}  // namespace digital_goods_util
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_GOODS_UTIL_H_
