/*
    Copyright (C) 1998 Lars Knoll (knoll@mpi-hd.mpg.de)
    Copyright (C) 2001 Dirk Mueller <mueller@kde.org>
    Copyright (C) 2006 Samuel Weinig (sam.weinig@gmail.com)
    Copyright (C) 2004, 2005, 2006, 2007 Apple Inc. All rights reserved.

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Library General Public
    License as published by the Free Software Foundation; either
    version 2 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Library General Public License for more details.

    You should have received a copy of the GNU Library General Public License
    along with this library; see the file COPYING.LIB.  If not, write to
    the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
    Boston, MA 02110-1301, USA.
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_IMAGE_RESOURCE_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_IMAGE_RESOURCE_OBSERVER_H_

#include "third_party/blink/public/mojom/webpreferences/web_preferences.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/style/style_image.h"
#include "third_party/blink/renderer/platform/graphics/graphics_context_types.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_priority.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class ImageResourceContent;

class CORE_EXPORT ImageResourceObserver : public GarbageCollectedMixin {
 public:
  // Used to notify the observers whether the invalidation resulting from an
  // image change notification can be deferred. In cases where the image is
  // changing as a result of an animation, its performant to avoid continuous
  // invalidations of offscreen content.
  // Note that the observer can ignore kYes and perform an immediate
  // invalidation, but kNo must be strictly enforced, i.e., if specified the
  // invalidation can not be deferred.
  enum class CanDeferInvalidation { kYes, kNo };

  virtual ~ImageResourceObserver() = default;

  // Called whenever a frame of an image changes, either because we got more
  // data from the network or because we are animating.
  virtual void ImageChanged(ImageResourceContent*, CanDeferInvalidation) {}

  // Sub-classes that have an associated image need to override this function
  // to get notified of any image change.
  virtual void ImageChanged(WrappedImagePtr, CanDeferInvalidation) {}

  // Called just after imageChanged() if all image data is received or errored.
  // TODO(hiroshige): Merge imageNotifyFinished() into imageChanged().
  virtual void ImageNotifyFinished(ImageResourceContent*) {}

  // Called when the image has fully removed this observer from its set of
  // observers.
  virtual void NotifyImageFullyRemoved(ImageResourceContent*) {}

  // Called to find out if this client wants to actually display the image. Used
  // to tell when we can halt animation. Content nodes that hold image refs for
  // example would not render the image, but LayoutImages would (assuming they
  // have visibility: visible and their layout tree isn't hidden e.g., in the
  // b/f cache or in a background tab).
  //
  // An implementation of this method is not allowed to add or remove
  // ImageResource observers.
  virtual bool WillRenderImage() { return false; }

  // Called to get imageAnimation policy from settings. An implementation of
  // this method is not allowed to add or remove ImageResource observers.
  virtual bool GetImageAnimationPolicy(mojom::blink::ImageAnimationPolicy&) {
    return false;
  }

  // Return the observer's requested resource priority. An implementation of
  // this method is not allowed to add or remove ImageResource observers.
  virtual ResourcePriority ComputeResourcePriority() const {
    return ResourcePriority();
  }
  virtual bool CanBeSpeculativelyDecoded() const { return true; }
  virtual gfx::Size GetSpeculativeDecodeSize() const { return gfx::Size(); }
  virtual InterpolationQuality GetSpeculativeDecodeQuality() const {
    return kInterpolationNone;
  }

  // Name for debugging, e.g. shown in memory-infra.
  virtual WTF::String DebugName() const = 0;

  static bool IsExpectedType(ImageResourceObserver*) { return true; }

  void Trace(Visitor*) const override {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_IMAGE_RESOURCE_OBSERVER_H_
