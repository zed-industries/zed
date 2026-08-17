// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_REMOTE_FONT_FACE_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_REMOTE_FONT_FACE_SOURCE_H_

#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/core/css/css_font_face_source.h"
#include "third_party/blink/renderer/core/execution_context/security_context.h"
#include "third_party/blink/renderer/core/loader/resource/font_resource.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CSSFontFace;
class Document;
class FontSelector;
class FontCustomPlatformData;

class RemoteFontFaceSource final : public CSSFontFaceSource,
                                   public FontResourceClient {
 public:
  enum Phase : uint8_t {
    kNoLimitExceeded,
    kShortLimitExceeded,
    kLongLimitExceeded
  };

  RemoteFontFaceSource(CSSFontFace*,
                       FontSelector*,
                       FontDisplay,
                       scoped_refptr<base::SingleThreadTaskRunner>);
  ~RemoteFontFaceSource() override;

  bool IsLoading() const override;
  bool IsLoaded() const override;
  bool IsValid() const override;

  String GetURL() const override { return url_; }

  bool IsPendingDataUrl() const override;

  const FontCustomPlatformData* GetCustomPlaftormData() const override {
    return custom_font_data_.Get();
  }

  void BeginLoadIfNeeded() override;
  void SetDisplay(FontDisplay) override;

  void NotifyFinished(Resource*) override;
  void FontLoadShortLimitExceeded(FontResource*) override;
  void FontLoadLongLimitExceeded(FontResource*) override;
  String DebugName() const override { return "RemoteFontFaceSource"; }

  bool IsInBlockPeriod() const override { return period_ == kBlockPeriod; }
  bool IsInFailurePeriod() const override { return period_ == kFailurePeriod; }

  // For UMA reporting and 'font-display: optional' period control.
  void PaintRequested() override;

  // For UMA reporting
  bool HadBlankText() override { return histograms_.HadBlankText(); }

  void Trace(Visitor*) const override;

 protected:
  const SimpleFontData* CreateFontData(
      const FontDescription&,
      const FontSelectionCapabilities&) override;
  const SimpleFontData* CreateLoadingFallbackFontData(const FontDescription&);

 private:
  // Periods of the Font Display Timeline.
  // https://drafts.csswg.org/css-fonts-4/#font-display-timeline
  // Note that kNotApplicablePeriod is an implementation detail indicating that
  // the font is loaded from memory cache synchronously, and hence, made
  // immediately available. As we never need to use a fallback for it, using
  // other DisplayPeriod values seem artificial. So we use a special value.
  enum DisplayPeriod : uint8_t {
    kBlockPeriod,
    kSwapPeriod,
    kFailurePeriod,
    kNotApplicablePeriod
  };

  class FontLoadHistograms {
    DISALLOW_NEW();

   public:
    // Should not change following order in CacheHitMetrics to be used for
    // metrics values.
    enum class CacheHitMetrics {
      kMiss,
      kDiskHit,
      kDataUrl,
      kMemoryHit,
      kMaxValue = kMemoryHit,
    };
    enum DataSource {
      kFromUnknown,
      kFromDataURL,
      kFromMemoryCache,
      kFromDiskCache,
      kFromNetwork
    };

    FontLoadHistograms()
        : blank_paint_time_recorded_(false),
          is_long_limit_exceeded_(false),
          data_source_(kFromUnknown) {}
    void LoadStarted();
    void FallbackFontPainted(DisplayPeriod);
    void LongLimitExceeded();
    void RecordFallbackTime();
    void RecordRemoteFont(const FontResource*);
    bool HadBlankText() { return !blank_paint_time_.is_null(); }
    DataSource GetDataSource() { return data_source_; }
    void MaySetDataSource(DataSource);

    static DataSource DataSourceForLoadFinish(const FontResource* font) {
      if (font->Url().ProtocolIsData()) {
        return kFromDataURL;
      }
      return font->GetResponse().WasCached() ? kFromDiskCache : kFromNetwork;
    }

   private:
    void RecordLoadTimeHistogram(const FontResource*, base::TimeDelta duration);
    CacheHitMetrics DataSourceMetricsValue();
    base::TimeTicks load_start_time_;
    base::TimeTicks blank_paint_time_;
    // |blank_paint_time_recorded_| is used to prevent
    // WebFont.BlankTextShownTime to be reported incorrectly when the web font
    // fallbacks immediately. See https://crbug.com/591304
    bool blank_paint_time_recorded_;
    bool is_long_limit_exceeded_;
    DataSource data_source_;
  };

  Document* GetDocument() const;

  DisplayPeriod ComputeFontDisplayAutoPeriod() const;
  bool NeedsInterventionToAlignWithLCPGoal() const;

  DisplayPeriod ComputePeriod() const;
  bool UpdatePeriod() override;
  bool ShouldTriggerWebFontsIntervention();
  bool IsLowPriorityLoadingAllowedForRemoteFont() const override;

  // Our owning font face.
  Member<CSSFontFace> face_;
  Member<FontSelector> font_selector_;

  // |nullptr| if font is not loaded or failed to decode.
  Member<const FontCustomPlatformData> custom_font_data_;
  // |nullptr| if font is not loaded or failed to decode.
  String url_;

  FontLoadHistograms histograms_;

  FontDisplay display_;
  Phase phase_;
  DisplayPeriod period_;
  bool is_intervention_triggered_;
  bool finished_before_document_rendering_begin_;

  // Indicates whether FontData has been requested for painting while the font
  // is still being loaded, in which case we will paint with a fallback font. If
  // true, and later if we would switch to the web font after it loads, there
  // will be a layout shifting. Therefore, we don't need to worry about layout
  // shifting when it's false.
  bool paint_requested_while_pending_;

  bool finished_before_lcp_limit_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_REMOTE_FONT_FACE_SOURCE_H_
