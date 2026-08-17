// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_VIDEO_FRAME_IMAGE_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_VIDEO_FRAME_IMAGE_UTIL_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "components/viz/common/resources/shared_image_format.h"
#include "media/base/video_transformation.h"
#include "third_party/blink/renderer/platform/graphics/image_orientation.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/skia/include/core/SkAlphaType.h"
#include "ui/gfx/color_space.h"
#include "ui/gfx/geometry/rect.h"
#include "ui/gfx/geometry/size.h"

// Note: Don't include "media/base/video_frame.h" here without good reason,
// since it includes a lot of non-blink types which can pollute the namespace.
namespace media {
class PaintCanvasVideoRenderer;
class VideoFrame;
}  // namespace media

namespace viz {
class RasterContextProvider;
}

namespace cc {
class PaintCanvas;
class PaintFlags;
}  // namespace cc

namespace blink {
class CanvasResourceProvider;
class StaticBitmapImage;

// Converts a media orientation into a blink one or vice versa.
PLATFORM_EXPORT ImageOrientationEnum
VideoTransformationToImageOrientation(media::VideoTransformation transform);
PLATFORM_EXPORT media::VideoTransformation
ImageOrientationToVideoTransformation(ImageOrientationEnum orientation);

// Returns true if CreateImageFromVideoFrame() expects to create an
// AcceleratedStaticBitmapImage. Note: This may be overridden if a software
// |resource_provider| is given to CreateImageFromVideoFrame().
PLATFORM_EXPORT bool WillCreateAcceleratedImagesFromVideoFrame(
    const media::VideoFrame* frame);

// Returns a StaticBitmapImage for the given frame. Accelerated images will be
// preferred if possible. A zero copy mechanism will be preferred if possible
// unless |allow_zero_copy_images| is false.
//
// |video_renderer| may optionally be provided in cases where the same frame may
// end up repeatedly converted.
//
// Likewise |resource_provider| may be provided to prevent thrashing when this
// method is called with high frequency.
//
// The default resource provider size is the frame's visible size. The default
// |dest_rect| is the visible size aligned to the origin. Callers may choose to
// provide their own |resource_provider| and |dest_rect| for rendering to the
// frame's natural size.
//
// When an external |resource_provider| is provided a |dest_rect| may also be
// provided to control where in the canvas the VideoFrame will be drawn. A
// non-empty |dest_rect| will disable zero copy image support.
//
// If |prefer_tagged_orientation| is true, CreateImageFromVideoFrame() will just
// tag the StaticBitmapImage with the correct orientation ("soft flip") instead
// of drawing the frame with the correct orientation ("hard flip").
//
// If `reinterpret_video_as_srgb` true, then the video will be reinterpreted as
// being originally having been in sRGB.
//
// Returns nullptr if a StaticBitmapImage can't be created.
PLATFORM_EXPORT scoped_refptr<StaticBitmapImage> CreateImageFromVideoFrame(
    scoped_refptr<media::VideoFrame> frame,
    bool allow_zero_copy_images = true,
    CanvasResourceProvider* resource_provider = nullptr,
    media::PaintCanvasVideoRenderer* video_renderer = nullptr,
    const gfx::Rect& dest_rect = gfx::Rect(),
    bool prefer_tagged_orientation = true,
    bool reinterpret_video_as_srgb = false);

// Similar to the above, but just skips creating the StaticBitmapImage from the
// CanvasResourceProvider. Returns true if the frame could be drawn or false
// otherwise. Note: In certain failure modes a black frame will be drawn.
//
// |video_renderer| may optionally be provided in cases where the same frame may
// end up repeatedly drawn.
//
// A |raster_context_provider| is required to convert texture backed frames.
//
// If |ignore_video_transformation| is true, the media::VideoTransformation on
// the |frame| will be ignored.
//
// If `reinterpret_video_as_srgb` true, then the video will be reinterpreted as
// being originally having been in sRGB.
PLATFORM_EXPORT bool DrawVideoFrameIntoResourceProvider(
    scoped_refptr<media::VideoFrame> frame,
    CanvasResourceProvider* resource_provider,
    viz::RasterContextProvider* raster_context_provider,
    const gfx::Rect& dest_rect,
    media::PaintCanvasVideoRenderer* video_renderer = nullptr,
    bool ignore_video_transformation = false,
    bool reinterpret_video_as_srgb = false);

PLATFORM_EXPORT void DrawVideoFrameIntoCanvas(
    scoped_refptr<media::VideoFrame> frame,
    cc::PaintCanvas* canvas,
    cc::PaintFlags& flags,
    bool ignore_video_transformation = false);

// Extract a RasterContextProvider from the current SharedGpuContext.
PLATFORM_EXPORT scoped_refptr<viz::RasterContextProvider>
GetRasterContextProvider();

// Creates a CanvasResourceProvider which is appropriate for drawing VideoFrame
// objects into. Some callers to CreateImageFromVideoFrame() may choose to cache
// their resource providers. If |raster_context_provider| is null a software
// resource provider will be returned.
PLATFORM_EXPORT std::unique_ptr<CanvasResourceProvider>
CreateResourceProviderForVideoFrame(
    gfx::Size size,
    viz::SharedImageFormat format,
    SkAlphaType alpha_type,
    const gfx::ColorSpace& color_space,
    viz::RasterContextProvider* raster_context_provider);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_VIDEO_FRAME_IMAGE_UTIL_H_
