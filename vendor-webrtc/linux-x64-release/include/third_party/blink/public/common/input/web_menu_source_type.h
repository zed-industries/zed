// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_MENU_SOURCE_TYPE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_MENU_SOURCE_TYPE_H_

namespace blink {

enum WebMenuSourceType {
  kMenuSourceNone,
  kMenuSourceMouse,
  kMenuSourceKeyboard,
  kMenuSourceTouch,
  kMenuSourceTouchEditMenu,
  kMenuSourceLongPress,
  kMenuSourceLongTap,
  kMenuSourceTouchHandle,
  kMenuSourceStylus,
  kMenuSourceAdjustSelection,
  kMenuSourceAdjustSelectionReset,
  kMenuSourceTypeLast = kMenuSourceAdjustSelectionReset
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_MENU_SOURCE_TYPE_H_
