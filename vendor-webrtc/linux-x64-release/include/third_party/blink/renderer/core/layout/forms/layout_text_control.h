/*
 * Copyright (C) 2006, 2007 Apple Inc. All rights reserved.
 *           (C) 2008 Torch Mobile Inc. All rights reserved.
 *               (http://www.torchmobile.com/)
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FORMS_LAYOUT_TEXT_CONTROL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FORMS_LAYOUT_TEXT_CONTROL_H_

#include "third_party/blink/renderer/platform/geometry/physical_offset.h"

namespace blink {

class ComputedStyle;
class Font;
class HTMLElement;
class HitTestLocation;
class HitTestResult;
class LayoutBox;

namespace layout_text_control {

void StyleDidChange(HTMLElement* inner_editor,
                    const ComputedStyle* old_style,
                    const ComputedStyle& new_style);
int ScrollbarThickness(const LayoutBox& box);
float GetAvgCharWidth(const ComputedStyle& style);
bool HasValidAvgCharWidth(const Font& font);

void HitInnerEditorElement(const LayoutBox& box,
                           HTMLElement& inner_editor,
                           HitTestResult&,
                           const HitTestLocation&,
                           const PhysicalOffset& accumulated_offset);

}  // namespace layout_text_control

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FORMS_LAYOUT_TEXT_CONTROL_H_
