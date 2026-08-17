/*
 * Copyright (c) 2013, Opera Software ASA. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of Opera Software ASA nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_CUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_CUE_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_align_setting.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/media/html_media_element.h"
#include "third_party/blink/renderer/core/html/track/text_track_cue.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/point_f.h"

namespace blink {

class Document;
class ExecutionContext;
class V8UnionAutoKeywordOrDouble;
class VTTCueBox;
class VTTRegion;
class VTTScanner;
class V8DirectionSetting;

using AlignSetting = V8AlignSetting::Enum;
using VTTRegionMap = HeapHashMap<String, Member<VTTRegion>>;

struct VTTDisplayParameters {
  STACK_ALLOCATED();

 public:
  VTTDisplayParameters();

  gfx::PointF position;
  double size;
  CSSValueID direction;
  CSSValueID text_align;
  CSSValueID writing_mode;
  double snap_to_lines_position;
};

class VTTCueBackgroundBox final : public HTMLDivElement {
 public:
  explicit VTTCueBackgroundBox(Document&);
  bool IsVTTCueBackgroundBox() const override { return true; }
  void SetTrack(TextTrack*);
  void Trace(Visitor*) const override;

  const TextTrack* GetTrack() const { return track_.Get(); }

 private:
  void DidRecalcStyle(const StyleRecalcChange) override;

  Member<TextTrack> track_;
};

template <>
struct DowncastTraits<VTTCueBackgroundBox> {
  static bool AllowFrom(const Node& node) {
    return node.IsElementNode() && To<Element>(node).IsVTTCueBackgroundBox();
  }
  static bool AllowFrom(const Element& element) {
    return element.IsVTTCueBackgroundBox();
  }
};

class CORE_EXPORT VTTCue final : public TextTrackCue {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static VTTCue* Create(Document& document,
                        double start_time,
                        double end_time,
                        const String& text) {
    return MakeGarbageCollected<VTTCue>(document, start_time, end_time, text);
  }

  VTTCue(Document&, double start_time, double end_time, const String& text);
  ~VTTCue() override;

  VTTRegion* region() const { return region_.Get(); }
  void setRegion(VTTRegion*);

  V8DirectionSetting vertical() const;
  void setVertical(const V8DirectionSetting&);

  bool snapToLines() const { return snap_to_lines_; }
  void setSnapToLines(bool);

  V8UnionAutoKeywordOrDouble* line() const;
  void setLine(const V8UnionAutoKeywordOrDouble* position);

  V8UnionAutoKeywordOrDouble* position() const;
  void setPosition(const V8UnionAutoKeywordOrDouble* position,
                   ExceptionState& exception_state);

  double size() const { return cue_size_; }
  void setSize(double, ExceptionState&);

  V8AlignSetting align() const;
  void setAlign(const V8AlignSetting&);

  const String& text() const { return text_; }
  void setText(const String&);

  void ParseSettings(const VTTRegionMap*, const String&);

  // Applies CSS override style from user settings.
  void ApplyUserOverrideCSSProperties();

  DocumentFragment* getCueAsHTML();

  // Handles the entrance and exit of cues for description-tagged tracks.
  // OnEnter begins speaking the cue. OnExit pauses the video to let the
  // description finish, if the cue is still being spoken at the specified end
  // time.
  void OnEnter(HTMLMediaElement& video) override;
  void OnExit(HTMLMediaElement& video) override;

  void UpdateDisplay(HTMLDivElement& container) override;

  void UpdatePastAndFutureNodes(double movie_time) override;

  std::optional<double> GetNextIntraCueTime(double movie_time) const override;

  void RemoveDisplayTree(RemovalNotification) override;

  double CalculateComputedLinePosition() const;

  enum class WritingDirection {
    kHorizontal = 0,
    kVerticalGrowingLeft,
    kVerticalGrowingRight,
    kMaxValue = kVerticalGrowingRight
  };
  WritingDirection GetWritingDirection() const { return writing_direction_; }

  ExecutionContext* GetExecutionContext() const override;

#ifndef NDEBUG
  String ToString() const override;
#endif

  void Trace(Visitor*) const override;

 private:
  Document& GetDocument() const;

  VTTCueBox* GetDisplayTree();

  void CueDidChange(
      CueMutationAffectsOrder = kCueMutationDoesNotAffectOrder) override;

  void CreateVTTNodeTree();
  void CopyVTTNodeToDOMTree(ContainerNode* vtt_node, ContainerNode* root);

  bool LineIsAuto() const;
  bool TextPositionIsAuto() const;

  VTTDisplayParameters CalculateDisplayParameters() const;
  double CalculateComputedTextPosition() const;
  AlignSetting CalculateComputedCueAlignment() const;

  enum class CueSetting {
    kNone,
    kVertical,
    kLine,
    kPosition,
    kSize,
    kAlign,
    kRegionId
  };
  CueSetting SettingName(VTTScanner&) const;

  String text_;
  double line_position_;
  double text_position_;
  double cue_size_;
  WritingDirection writing_direction_;
  AlignSetting cue_alignment_ = AlignSetting::kCenter;

  Member<VTTRegion> region_;
  Member<DocumentFragment> vtt_node_tree_;
  Member<VTTCueBackgroundBox> cue_background_box_;
  Member<VTTCueBox> display_tree_;

  bool snap_to_lines_ : 1;
  bool display_tree_should_change_ : 1;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_CUE_H_
