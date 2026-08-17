/*
 * Copyright (C) 2004, 2006, 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_CURSORS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_CURSORS_H_

#include "third_party/blink/renderer/platform/platform_export.h"

// To avoid conflicts with the CreateWindow macro from the Windows SDK...
#undef CopyCursor

namespace ui {
class Cursor;
}

namespace blink {

PLATFORM_EXPORT const ui::Cursor& PointerCursor();
PLATFORM_EXPORT const ui::Cursor& CrossCursor();
PLATFORM_EXPORT const ui::Cursor& HandCursor();
PLATFORM_EXPORT const ui::Cursor& MoveCursor();
PLATFORM_EXPORT const ui::Cursor& IBeamCursor();
PLATFORM_EXPORT const ui::Cursor& WaitCursor();
PLATFORM_EXPORT const ui::Cursor& HelpCursor();
PLATFORM_EXPORT const ui::Cursor& EastResizeCursor();
PLATFORM_EXPORT const ui::Cursor& NorthResizeCursor();
PLATFORM_EXPORT const ui::Cursor& NorthEastResizeCursor();
PLATFORM_EXPORT const ui::Cursor& NorthWestResizeCursor();
PLATFORM_EXPORT const ui::Cursor& SouthResizeCursor();
PLATFORM_EXPORT const ui::Cursor& SouthEastResizeCursor();
PLATFORM_EXPORT const ui::Cursor& SouthWestResizeCursor();
PLATFORM_EXPORT const ui::Cursor& WestResizeCursor();
PLATFORM_EXPORT const ui::Cursor& NorthSouthResizeCursor();
PLATFORM_EXPORT const ui::Cursor& EastWestResizeCursor();
PLATFORM_EXPORT const ui::Cursor& NorthEastSouthWestResizeCursor();
PLATFORM_EXPORT const ui::Cursor& NorthWestSouthEastResizeCursor();
PLATFORM_EXPORT const ui::Cursor& ColumnResizeCursor();
PLATFORM_EXPORT const ui::Cursor& RowResizeCursor();
PLATFORM_EXPORT const ui::Cursor& MiddlePanningCursor();
PLATFORM_EXPORT const ui::Cursor& MiddlePanningVerticalCursor();
PLATFORM_EXPORT const ui::Cursor& MiddlePanningHorizontalCursor();
PLATFORM_EXPORT const ui::Cursor& EastPanningCursor();
PLATFORM_EXPORT const ui::Cursor& NorthPanningCursor();
PLATFORM_EXPORT const ui::Cursor& NorthEastPanningCursor();
PLATFORM_EXPORT const ui::Cursor& NorthWestPanningCursor();
PLATFORM_EXPORT const ui::Cursor& SouthPanningCursor();
PLATFORM_EXPORT const ui::Cursor& SouthEastPanningCursor();
PLATFORM_EXPORT const ui::Cursor& SouthWestPanningCursor();
PLATFORM_EXPORT const ui::Cursor& WestPanningCursor();
PLATFORM_EXPORT const ui::Cursor& VerticalTextCursor();
PLATFORM_EXPORT const ui::Cursor& CellCursor();
PLATFORM_EXPORT const ui::Cursor& ContextMenuCursor();
PLATFORM_EXPORT const ui::Cursor& NoDropCursor();
PLATFORM_EXPORT const ui::Cursor& NotAllowedCursor();
PLATFORM_EXPORT const ui::Cursor& ProgressCursor();
PLATFORM_EXPORT const ui::Cursor& AliasCursor();
PLATFORM_EXPORT const ui::Cursor& ZoomInCursor();
PLATFORM_EXPORT const ui::Cursor& ZoomOutCursor();
PLATFORM_EXPORT const ui::Cursor& CopyCursor();
PLATFORM_EXPORT const ui::Cursor& NoneCursor();
PLATFORM_EXPORT const ui::Cursor& GrabCursor();
PLATFORM_EXPORT const ui::Cursor& GrabbingCursor();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_CURSORS_H_
