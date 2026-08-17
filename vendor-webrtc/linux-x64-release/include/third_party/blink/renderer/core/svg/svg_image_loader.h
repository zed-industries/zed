/*
 * Copyright (C) 2006 Alexander Kellett <lypanov@kde.org>
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_IMAGE_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_IMAGE_LOADER_H_

#include "third_party/blink/renderer/core/loader/image_loader.h"

namespace blink {

class SVGImageElement;

class SVGImageLoader final : public ImageLoader {
 public:
  explicit SVGImageLoader(SVGImageElement*);

 private:
  void DispatchLoadEvent() override;
  void DispatchErrorEvent() override;
  String DebugName() const override { return "SVGImageLoader"; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_IMAGE_LOADER_H_
