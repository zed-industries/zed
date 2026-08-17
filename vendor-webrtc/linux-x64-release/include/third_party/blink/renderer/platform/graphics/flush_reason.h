// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FLUSH_REASON_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FLUSH_REASON_H_

namespace blink {

// Reasons for requesting that recorded PaintOps be flushed. Used in code
// loosely related to 2d canvas rendering contexts.
enum class FlushReason {
  // This enum is used by a histogram. Do not change item values.

  // Use at call sites that never require flushing recorded paint ops
  // For example when requesting WebGL or WebGPU snapshots. Does not
  // impede vector printing.
  kNone = 0,

  // Used in C++ unit tests
  kTesting = 1,

  // Call site may be flushing paint ops, but they're for a use case
  // unrelated to Canvas rendering contexts. Does not impede vector printing.
  kNon2DCanvas = 2,

  // Canvas contents were cleared. This makes the canvas vector printable
  // again.
  kClear = 3,

  // The canvas content is being swapped-out because its tab is hidden.
  // Should not happen while printing.
  kHibernating = 4,

  // `OffscreenCanvas::commit` was called.
  // Should not happen while printing.
  kOffscreenCanvasCommit_OBSOLETE = 5,

  // `OffscreenCanvas` dispatched a frame to the compositor as part of the
  // regular animation frame presentation flow.
  // Should not happen while printing.
  kOffscreenCanvasPushFrame = 6,

  // createImageBitmap() was called with the canvas as its argument.
  // Should not happen while printing.
  kCreateImageBitmap = 7,

  // The `getImageData` API method was called on the canvas's 2d context.
  // This inhibits vector printing.
  kGetImageData = 8,

  // A paint op was recorded that referenced a volatile source image and
  // therefore the recording needed to be flush immediately before the
  // source image contents could be overwritten. For example, a video frame.
  // This inhibits vector printing.
  kVolatileSourceImage = 9,

  // The canvas element dispatched a frame to the compositor
  // This inhibits vector printing.
  kCanvasPushFrame = 10,

  // The canvas element dispatched a frame to the compositor while printing
  // was in progress.
  // This does not prevent vector printing as long as the current frame is
  // clear.
  kCanvasPushFrameWhilePrinting = 11,

  // Direct write access to the pixel buffer (e.g. `putImageData`)
  // This inhibits vector printing.
  kWritePixels = 12,

  // To blob was called on the canvas.
  // This inhibits vector printing.
  kToBlob = 13,

  // A `VideoFrame` object was created with the canvas as an image source
  // This inhibits vector printing.
  kCreateVideoFrame = 14,

  // The canvas was used as a source image in a call to
  // `CanvasRenderingContext2D.drawImage`.
  // This inhibits vector printing.
  kDrawImage = 15,

  // The canvas is observed by a `CanvasDrawListener`. This typically means
  // that canvas contents are being streamed to a WebRTC video stream.
  // This inhibits vector printing.
  kDrawListener = 16,

  // The canvas contents were painted to its parent content layer, this
  // is the non-composited presentation code path.
  // This should never happen while printing.
  kPaint = 17,

  // Canvas contents were transferred to an `ImageBitmap`. This does not
  // inhibit vector printing since it effectively clears the canvas.
  kTransfer = 18,

  // The canvas is being printed.
  kPrinting = 19,

  // The canvas was loaded as a WebGPU external image.
  // This inhibits vector printing.
  kWebGPUExternalImage = 20,

  // The canvas contents were copied to an SkBitmap.
  // This inhibits vector printing.
  kCopyToSkBitmap = 21,

  // The canvas was uploaded to a WebGL texture.
  // This inhibits vector printing.
  kWebGLTexImage = 22,

  // The canvas was used as a source in a call to
  // `CanvasRenderingContext2D.createPattern`.
  // This inhibits vector printing.
  kCreatePattern = 23,

  // The canvas contents were copied to the clipboard.
  // This inhibits vector printing.
  kClipboard = 24,

  // The canvas's recorded ops had a reference to an image whose contents
  // were about to change.
  // This inhibits vector printing.
  kSourceImageWillChange = 25,

  // The canvas was uploade to a WebGPU texture.
  // This inhibits vector printing.
  kWebGPUTexture = 26,

  // The HTMLCanvasElement.toDataURL method was called on the canvas.
  kToDataURL = 27,

  // The canvas's layer bridge was replaced. This happens when switching
  // between GPU and CPU rendering.
  // This inhibits vector printing.
  kReplaceLayerBridge = 28,

  // The auto-flush heuristic kicked-in. Should not happen while
  // printing.
  kRecordingLimitExceeded = 29,

  // The canvas was used as a source image in a call to
  // `CanvasRenderingContext2D.drawMesh`.
  // This inhibits vector printing.
  kDrawMesh = 30,

  kMaxValue = kDrawMesh,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FLUSH_REASON_H_
