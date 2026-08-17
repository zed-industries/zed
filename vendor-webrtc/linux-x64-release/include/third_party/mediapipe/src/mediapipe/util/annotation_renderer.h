// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_UTIL_ANNOTATION_RENDERER_H_
#define MEDIAPIPE_UTIL_ANNOTATION_RENDERER_H_

#include <string>

#include "mediapipe/framework/port/opencv_core_inc.h"
#include "mediapipe/framework/port/opencv_imgproc_inc.h"
#include "mediapipe/util/render_data.pb.h"

namespace mediapipe {

// The renderer library for rendering data on images.
//
// Example usage:
//
// AnnotationRenderer renderer;
//
// std::unique_ptr<cv::Mat> mat_image(new cv::Mat(kImageHeight, kImageWidth,
//                                 CV_8UC3));
//
// renderer.AdoptImage(mat_image.get());
//
// RenderData render_data_0;
// <FILL RENDER_DATA_0 WITH ANNOTATIONS>
//
// renderer.RenderDataOnImage(render_data_0);
//
// RenderData render_data_1;
// <FILL RENDER_DATA_1 WITH ANNOTATIONS>
//
// renderer.RenderDataOnImage(render_data_1);
//
// UseRenderedImage(mat_image.get());
class AnnotationRenderer {
 public:
  explicit AnnotationRenderer() {}

  explicit AnnotationRenderer(const cv::Mat& mat_image)
      : image_width_(mat_image.cols),
        image_height_(mat_image.rows),
        mat_image_(mat_image.clone()) {}

  // Renders the image with the input render data.
  void RenderDataOnImage(const RenderData& render_data);

  // Resets the renderer with a new image. Does not own input_image. input_image
  // must not be modified by caller during rendering.
  void AdoptImage(cv::Mat* input_image);

  // Gets image dimensions.
  int GetImageWidth() const;
  int GetImageHeight() const;

  // Sets whether text should be rendered upside down. This is default to false
  // and text is rendered assuming the underlying image has its origin at the
  // top-left corner. Set it to true if the image origin is at the bottom-left
  // corner.
  void SetFlipTextVertically(bool flip);

  // For GPU rendering optimization in AnnotationOverlayCalculator.
  // Scale all incoming coordinates,sizes,thickness,etc. by this amount.
  // Should be in the range (0-1].
  // See 'gpu_scale_factor' in annotation_overlay_calculator.proto
  void SetScaleFactor(float scale_factor);
  float GetScaleFactor() { return scale_factor_; }

 private:
  // Draws a rectangle on the image as described in the annotation.
  void DrawRectangle(const RenderAnnotation& annotation);

  // Draws a filled rectangle on the image as described in the annotation.
  void DrawFilledRectangle(const RenderAnnotation& annotation);

  // Draws an oval on the image as described in the annotation.
  void DrawOval(const RenderAnnotation& annotation);

  // Draws a filled oval on the image as described in the annotation.
  void DrawFilledOval(const RenderAnnotation& annotation);

  // Draws an arrow on the image as described in the annotation.
  void DrawArrow(const RenderAnnotation& annotation);

  // Draws a point on the image as described in the annotation.
  void DrawPoint(const RenderAnnotation& annotation);
  void DrawPoint(const RenderAnnotation::Point& point,
                 const RenderAnnotation& annotation);

  // Draws scribbles on the image as described in the annotation.
  void DrawScribble(const RenderAnnotation& annotation);

  // Draws a line segment on the image as described in the annotation.
  void DrawLine(const RenderAnnotation& annotation);

  // Draws a 2-tone line segment on the image as described in the annotation.
  void DrawGradientLine(const RenderAnnotation& annotation);

  // Draws a text on the image as described in the annotation.
  void DrawText(const RenderAnnotation& annotation);

  // Draws a rounded rectangle on the image as described in the annotation.
  void DrawRoundedRectangle(const RenderAnnotation& annotation);

  // Draws a filled rounded rectangle on the image as described in the
  // annotation.
  void DrawFilledRoundedRectangle(const RenderAnnotation& annotation);

  // Helper function for drawing a rectangle with rounded corners. The
  // parameters are the same as in the OpenCV function rectangle().
  // corner_radius: A positive int value defining the radius of the round
  // corners.
  void DrawRoundedRectangle(cv::Mat src, cv::Point top_left,
                            cv::Point bottom_right,
                            const cv::Scalar& line_color, int thickness = 1,
                            int line_type = 8, int corner_radius = 0);

  // Computes the font scale from font_face, size and thickness.
  double ComputeFontScale(int font_face, int font_size, int thickness);

  // Width and Height of the image (in pixels).
  int image_width_ = -1;
  int image_height_ = -1;

  // The image for rendering.
  cv::Mat mat_image_;

  // See SetFlipTextVertically(bool).
  bool flip_text_vertically_ = false;

  // See SetScaleFactor(float)
  float scale_factor_ = 1.0;
};
}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_ANNOTATION_RENDERER_H_
