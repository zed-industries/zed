// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_SHAPE_RESULT_BLOBERIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_SHAPE_RESULT_BLOBERIZER_H_

#include "cc/paint/node_id.h"
#include "third_party/blink/renderer/platform/fonts/canvas_rotation_in_vertical.h"
#include "third_party/blink/renderer/platform/fonts/glyph.h"
#include "third_party/blink/renderer/platform/fonts/shaping/shape_result_buffer.h"
#include "third_party/blink/renderer/platform/fonts/simple_font_data.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/skia/include/core/SkTextBlob.h"
#include "ui/gfx/geometry/point_f.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace cc {
class PaintCanvas;
class PaintFlags;
}  // namespace cc

namespace blink {

class FontDescription;
class PlainTextNode;
struct TextRunPaintInfo;

class PLATFORM_EXPORT ShapeResultBloberizer {
  STACK_ALLOCATED();

 public:
  enum class Type { kNormal, kTextIntercepts, kEmitText };

  struct FillGlyphsNG;
  struct FillTextEmphasisGlyphsNG;

  struct FillGlyphs;
  struct FillTextEmphasisGlyphs;

  explicit ShapeResultBloberizer(const FontDescription&, Type);
  ShapeResultBloberizer(const ShapeResultBloberizer&) = delete;
  ShapeResultBloberizer& operator=(const ShapeResultBloberizer&) = delete;

  struct BlobInfo {
    BlobInfo(sk_sp<SkTextBlob> b, CanvasRotationInVertical r)
        : blob(std::move(b)), rotation(r) {}
    sk_sp<SkTextBlob> blob;
    CanvasRotationInVertical rotation;
  };
  using BlobBuffer = Vector<BlobInfo, 16>;

  const BlobBuffer& Blobs();
  float Advance() const { return advance_; }

 private:
  friend class ShapeResultBloberizerTestInfo;

  void Add(Glyph glyph,
           const SimpleFontData* font_data,
           CanvasRotationInVertical canvas_rotation,
           float h_offset,
           unsigned character_index) {
    // cannot mix x-only/xy offsets
    DCHECK(!HasPendingVerticalOffsets());

    if (font_data != pending_font_data_ ||
        canvas_rotation != pending_canvas_rotation_) [[unlikely]] {
      CommitPendingRun();
      pending_font_data_ = font_data;
      pending_canvas_rotation_ = canvas_rotation;
      DCHECK(!IsCanvasRotationInVerticalUpright(canvas_rotation))
          << static_cast<int>(canvas_rotation);
    }

    pending_glyphs_.push_back(glyph);
    pending_offsets_.push_back(h_offset);
    if (!current_text_.IsNull()) [[unlikely]] {
      DVLOG(5) << "  Appending glyph " << glyph << " with start index "
               << character_index;
      current_character_indexes_.push_back(character_index);
    }
  }

  void Add(Glyph glyph,
           const SimpleFontData* font_data,
           CanvasRotationInVertical canvas_rotation,
           const gfx::Vector2dF& offset,
           unsigned character_index) {
    // cannot mix x-only/xy offsets
    DCHECK(pending_glyphs_.empty() || HasPendingVerticalOffsets());

    if (font_data != pending_font_data_ ||
        canvas_rotation != pending_canvas_rotation_) [[unlikely]] {
      CommitPendingRun();
      pending_font_data_ = font_data;
      pending_canvas_rotation_ = canvas_rotation;
      const auto& metrics = font_data->GetFontMetrics();
      pending_vertical_baseline_x_offset_ =
          !IsCanvasRotationInVerticalUpright(canvas_rotation)
              ? 0
              : metrics.FloatAscent() - metrics.FloatAscent(kCentralBaseline);
    }

    pending_glyphs_.push_back(glyph);
    pending_offsets_.push_back(offset.x() +
                               pending_vertical_baseline_x_offset_);
    pending_offsets_.push_back(offset.y());
    if (!current_text_.IsNull()) [[unlikely]] {
      DVLOG(5) << "  Appending glyph " << glyph << " with start index "
               << character_index;
      current_character_indexes_.push_back(character_index);
    }
  }

  // Whether the FillFastHorizontalGlyphs or AddFastHorizontalGlyphToBloberizer
  // can be used. Only applies for full runs with no vertical offsets, no text
  // intercepts, and not emitting text.
  bool CanUseFastPath(unsigned from,
                      unsigned to,
                      unsigned length,
                      bool has_vertical_offsets);
  bool CanUseFastPath(unsigned from, unsigned to, const ShapeResultView*);
  float FillFastHorizontalGlyphs(const ShapeResultBuffer&, TextDirection);
  float FillFastHorizontalGlyphs(const ShapeResult*, float advance = 0);
  static void AddFastHorizontalGlyphToBloberizer(void* context,
                                                 unsigned,
                                                 Glyph,
                                                 gfx::Vector2dF glyph_offset,
                                                 float advance,
                                                 bool is_horizontal,
                                                 CanvasRotationInVertical,
                                                 const SimpleFontData*);

  float FillGlyphsForResult(const ShapeResult*,
                            const StringView&,
                            unsigned from,
                            unsigned to,
                            float initial_advance,
                            unsigned run_offset);
  static void AddGlyphToBloberizer(void* context,
                                   unsigned character_index,
                                   Glyph,
                                   gfx::Vector2dF glyph_offset,
                                   float advance,
                                   bool is_horizontal,
                                   CanvasRotationInVertical,
                                   const SimpleFontData*);

  void AddEmphasisMark(const GlyphData& emphasis_data,
                       CanvasRotationInVertical canvas_rotation,
                       gfx::PointF glyph_center,
                       float mid_glyph_offset);
  static void AddEmphasisMarkToBloberizer(
      void* context,
      unsigned character_index,
      float advance_so_far,
      unsigned graphemes_in_cluster,
      float cluster_advance,
      CanvasRotationInVertical canvas_rotation);

  bool IsSkipInkException(const StringView& text, unsigned character_index);

  void SetText(const StringView& text,
               unsigned from,
               unsigned to,
               base::span<const unsigned> cluster_starts);
  void CommitText();
  void CommitPendingRun();
  void CommitPendingBlob();

  bool HasPendingVerticalOffsets() const;

  const FontDescription& font_description_;
  const Type type_;

  // Current text blob state.
  SkTextBlobBuilder builder_;
  CanvasRotationInVertical builder_rotation_ =
      CanvasRotationInVertical::kRegular;
  size_t builder_run_count_ = 0;

  // Current run state.
  const SimpleFontData* pending_font_data_ = nullptr;
  CanvasRotationInVertical pending_canvas_rotation_ =
      CanvasRotationInVertical::kRegular;
  Vector<Glyph, 1024> pending_glyphs_;
  Vector<float, 1024> pending_offsets_;

  // Reserve a small amount of space for the common case when printing.
  // Allowing this class to grow larger than ~7k impacts user perf.
  Vector<uint8_t, 64> pending_utf8_;
  Vector<uint32_t, 64> pending_utf8_character_indexes_;
  Vector<unsigned, 64> current_character_indexes_;
  Vector<unsigned, 64> cluster_ends_;
  unsigned cluster_ends_offset_ = 0;
  StringView current_text_;

  float pending_vertical_baseline_x_offset_ = 0;

  // Constructed blobs.
  BlobBuffer blobs_;
  float advance_ = 0;
};

struct PLATFORM_EXPORT ShapeResultBloberizer::FillGlyphsNG
    : public ShapeResultBloberizer {
  FillGlyphsNG(const FontDescription&,
               const StringView&,
               unsigned from,
               unsigned to,
               const ShapeResultView*,
               Type);
};
struct PLATFORM_EXPORT ShapeResultBloberizer::FillTextEmphasisGlyphsNG
    : public ShapeResultBloberizer {
  FillTextEmphasisGlyphsNG(const FontDescription&,
                           const StringView&,
                           unsigned from,
                           unsigned to,
                           const ShapeResultView*,
                           const GlyphData& emphasis_data);
};

struct PLATFORM_EXPORT ShapeResultBloberizer::FillGlyphs
    : public ShapeResultBloberizer {
  FillGlyphs(const FontDescription&,
             const TextRunPaintInfo&,
             const ShapeResultBuffer&,
             Type);
  FillGlyphs(const FontDescription& font_description,
             const PlainTextNode& node,
             Type type);

 private:
  template <typename ShapeList>
  void FillGlyphsSlow(StringView text,
                      TextDirection direction,
                      const ShapeList& list,
                      unsigned from,
                      unsigned to);
};

void DrawTextBlobs(const ShapeResultBloberizer::BlobBuffer& blobs,
                   cc::PaintCanvas& canvas,
                   const gfx::PointF& point,
                   const cc::PaintFlags& flags,
                   cc::NodeId node_id = cc::kInvalidNodeId);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_SHAPE_RESULT_BLOBERIZER_H_
