/*
 * This file is part of the theme implementation for form controls in WebCore.
 *
 * Copyright (C) 2005, 2006, 2007, 2008, 2009, 2010, 2012 Apple Computer, Inc.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_THEME_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_THEME_PAINTER_H_

#include "third_party/blink/renderer/platform/theme_types.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace gfx {
class Rect;
}

namespace blink {

class ComputedStyle;
class Document;
class Element;
class LayoutObject;
class Node;
struct PaintInfo;

class ThemePainter {
  DISALLOW_NEW();

 public:
  explicit ThemePainter();

  // This method is called to paint the widget as a background of the
  // LayoutObject.  A widget's foreground, e.g., the text of a button, is always
  // rendered by the engine itself.  The boolean return value indicates whether
  // the CSS border/background should also be painted.
  bool Paint(const LayoutObject&, const PaintInfo&, const gfx::Rect&);
  bool PaintBorderOnly(const Node*,
                       const ComputedStyle&,
                       const PaintInfo&,
                       const gfx::Rect&);
  bool PaintDecorations(const Node*,
                        const Document&,
                        const ComputedStyle&,
                        const PaintInfo&,
                        const gfx::Rect&);

  virtual bool PaintCapsLockIndicator(const LayoutObject&,
                                      const PaintInfo&,
                                      const gfx::Rect&) {
    return false;
  }
  void PaintSliderTicks(const LayoutObject&,
                        const PaintInfo&,
                        const gfx::Rect&);

 protected:
  virtual bool PaintCheckbox(const Element&,
                             const Document&,
                             const ComputedStyle&,
                             const PaintInfo&,
                             const gfx::Rect&) {
    return true;
  }
  virtual bool PaintRadio(const Element&,
                          const Document&,
                          const ComputedStyle&,
                          const PaintInfo&,
                          const gfx::Rect&) {
    return true;
  }
  virtual bool PaintButton(const Element&,
                           const Document&,
                           const ComputedStyle&,
                           const PaintInfo&,
                           const gfx::Rect&) {
    return true;
  }
  virtual bool PaintInnerSpinButton(const Element&,
                                    const ComputedStyle&,
                                    const PaintInfo&,
                                    const gfx::Rect&) {
    return true;
  }
  virtual bool PaintTextField(const Element&,
                              const ComputedStyle&,
                              const PaintInfo&,
                              const gfx::Rect&) {
    return true;
  }
  virtual bool PaintTextArea(const Element&,
                             const ComputedStyle&,
                             const PaintInfo&,
                             const gfx::Rect&) {
    return true;
  }
  virtual bool PaintMenuList(const Element&,
                             const Document&,
                             const ComputedStyle&,
                             const PaintInfo&,
                             const gfx::Rect&) {
    return true;
  }
  virtual bool PaintMenuListButton(const Element& element,
                                   const Document&,
                                   const ComputedStyle&,
                                   const PaintInfo&,
                                   const gfx::Rect&) {
    return true;
  }
  virtual bool PaintProgressBar(const Element& element,
                                const LayoutObject&,
                                const PaintInfo&,
                                const gfx::Rect&,
                                const ComputedStyle&) {
    return true;
  }
  virtual bool PaintSliderTrack(const Element& element,
                                const LayoutObject&,
                                const PaintInfo&,
                                const gfx::Rect&,
                                const ComputedStyle&) {
    return true;
  }
  virtual bool PaintSliderThumb(const Element&,
                                const ComputedStyle&,
                                const PaintInfo&,
                                const gfx::Rect&) {
    return true;
  }
  virtual bool PaintSearchField(const Element&,
                                const ComputedStyle&,
                                const PaintInfo&,
                                const gfx::Rect&) {
    return true;
  }
  virtual bool PaintSearchFieldCancelButton(const LayoutObject&,
                                            const PaintInfo&,
                                            const gfx::Rect&) {
    return true;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_THEME_PAINTER_H_
