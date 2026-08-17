/*
 * Copyright (C) 2006, 2007, 2008 Apple Computer, Inc.  All rights reserved.
 * Copyright (C) 2007 Alp Toker <alp@atoker.com>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_IMAGE_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_IMAGE_SOURCE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/flush_reason.h"
#include "third_party/blink/renderer/platform/graphics/image_orientation.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class Image;

enum SourceImageStatus {
  kNormalSourceImageStatus,
  kUndecodableSourceImageStatus,     // Image element with a 'broken' image
  kZeroSizeCanvasSourceImageStatus,  // Source is a canvas with width or height
                                     // of zero
  kZeroSizeImageSourceStatus,    // Image element with width or height of zero
  kIncompleteSourceImageStatus,  // Image element with no source media
  kInvalidSourceImageStatus,
  kLayersOpenInCanvasSource,  // Source is a canvas with open layers
};

class CORE_EXPORT CanvasImageSource {
 public:
  virtual scoped_refptr<Image> GetSourceImageForCanvas(FlushReason,
                                                       SourceImageStatus*,
                                                       const gfx::SizeF&) = 0;

  // IMPORTANT: Result must be independent of whether destinationContext is
  // already tainted because this function may be used to determine whether
  // a CanvasPattern is "origin clean", and that pattern may be used on
  // another canvas, which may not be already tainted.
  virtual bool WouldTaintOrigin() const = 0;

  virtual bool IsCSSImageValue() const { return false; }
  virtual bool IsImageElement() const { return false; }
  virtual bool IsVideoElement() const { return false; }
  virtual bool IsCanvasElement() const { return false; }
  virtual bool IsSVGSource() const { return false; }
  virtual bool IsImageBitmap() const { return false; }
  virtual bool IsOffscreenCanvas() const { return false; }
  virtual bool IsVideoFrame() const { return false; }

  // TODO(crbug.com/dawn/1197369): Implement check
  // the usability of the image argument.
  // Ref the spec here:
  //  https://html.spec.whatwg.org/multipage/canvas.html#check-the-usability-of-the-image-argument
  virtual bool IsNeutered() const { return false; }

  // Spec about placeholder context:
  // https://html.spec.whatwg.org/multipage/canvas.html#offscreencanvas-placeholder
  virtual bool IsPlaceholder() const { return false; }

  virtual gfx::SizeF ElementSize(const gfx::SizeF& default_object_size,
                                 const RespectImageOrientationEnum) const = 0;
  virtual gfx::SizeF DefaultDestinationSize(
      const gfx::SizeF& default_object_size,
      const RespectImageOrientationEnum respect_orientation) const {
    return ElementSize(default_object_size, respect_orientation);
  }
  virtual const KURL& SourceURL() const { return BlankURL(); }
  virtual bool IsOpaque() const { return false; }
  virtual bool IsAccelerated() const = 0;

 protected:
  virtual ~CanvasImageSource() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_IMAGE_SOURCE_H_
