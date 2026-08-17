// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_CONTEXT_MENU_DATA_EDIT_FLAGS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_CONTEXT_MENU_DATA_EDIT_FLAGS_H_

namespace blink {

enum ContextMenuDataEditFlags {
  kCanDoNone = 0x0,
  kCanUndo = 0x1,
  kCanRedo = 0x2,
  kCanCut = 0x4,
  kCanCopy = 0x8,
  kCanPaste = 0x10,
  kCanDelete = 0x20,
  kCanSelectAll = 0x40,
  kCanTranslate = 0x80,
  kCanEditRichly = 0x100,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_CONTEXT_MENU_DATA_EDIT_FLAGS_H_
