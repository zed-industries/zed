// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_TRACE_EVENTS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_TRACE_EVENTS_H_

#include <memory>
#include <optional>

#include "base/containers/span_or_size.h"
#include "base/trace_event/trace_event.h"
#include "third_party/blink/renderer/bindings/core/v8/script_streamer.h"
#include "third_party/blink/renderer/core/animation/compositor_animations.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/core_probe_sink.h"
#include "third_party/blink/renderer/core/css/css_selector.h"
#include "third_party/blink/renderer/core/loader/frame_loader_types.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/instrumentation/tracing/traced_value.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_load_priority.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread.h"
#include "third_party/blink/renderer/platform/scheduler/public/web_scheduling_priority.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "v8/include/v8.h"

namespace base {
class UnguessableToken;
}

namespace gfx {
class Rect;
class RectF;
}

namespace v8 {
class Function;
}  // namespace v8

namespace WTF {
class TextPosition;
}

namespace blink {
class Animation;
class CSSStyleSheetResource;
class ContainerNode;
class Document;
class DocumentLoader;
class Element;
class EncodedFormData;
class Event;
class MessageEvent;
class ExecutionContext;
class HitTestLocation;
class HitTestRequest;
class HitTestResult;
class ImageResourceContent;
class InvalidationSet;
class KURL;
class LayoutImage;
class LayoutObject;
struct LayoutObjectWithDepth;
class LocalFrame;
class LocalFrameView;
class Node;
class QualifiedName;
enum class RenderBlockingBehavior : uint8_t;
class Resource;
class ResourceError;
struct ResourceLoaderOptions;
class ResourceRequest;
class ResourceRequestHead;
class ResourceResponse;
class StyleChangeReasonForTracing;
class StyleImage;
class XMLHttpRequest;
enum class ResourceType : uint8_t;
enum StyleChangeType : uint32_t;

namespace probe {
class CallFunction;
class ExecuteScript;
class ParseHTML;
}  // namespace probe

class CORE_EXPORT InspectorTraceEvents
    : public GarbageCollected<InspectorTraceEvents> {
 public:
  InspectorTraceEvents() = default;
  InspectorTraceEvents(const InspectorTraceEvents&) = delete;
  InspectorTraceEvents& operator=(const InspectorTraceEvents&) = delete;

  static uint64_t GetNextSampleTraceId();

  void WillSendRequest(ExecutionContext*,
                       DocumentLoader*,
                       const KURL& fetch_context_url,
                       const ResourceRequest&,
                       const ResourceResponse& redirect_response,
                       const ResourceLoaderOptions&,
                       ResourceType,
                       RenderBlockingBehavior,
                       base::TimeTicks timestamp);
  void WillSendNavigationRequest(uint64_t identifier,
                                 DocumentLoader*,
                                 const KURL&,
                                 const AtomicString& http_method,
                                 EncodedFormData*);
  void DidReceiveResourceResponse(uint64_t identifier,
                                  DocumentLoader*,
                                  const ResourceResponse&,
                                  const Resource*);
  void DidReceiveData(uint64_t identifier,
                      DocumentLoader*,
                      base::SpanOrSize<const char> encoded_data);
  void DidFinishLoading(uint64_t identifier,
                        DocumentLoader*,
                        base::TimeTicks monotonic_finish_time,
                        int64_t encoded_data_length,
                        int64_t decoded_body_length);
  void DidFailLoading(
      CoreProbeSink* sink,
      uint64_t identifier,
      DocumentLoader*,
      const ResourceError&,
      const base::UnguessableToken& devtools_frame_or_worker_token);
  void MarkResourceAsCached(DocumentLoader* loader, uint64_t identifier);

  void Will(const probe::ExecuteScript&);
  void Did(const probe::ExecuteScript&);

  void Will(const probe::ParseHTML&);
  void Did(const probe::ParseHTML&);

  void Will(const probe::CallFunction&);
  void Did(const probe::CallFunction&);

  void PaintTiming(Document*, const char* name, double timestamp);

  void FrameStartedLoading(LocalFrame*);

  void Trace(Visitor*) const {}
};

// Helper macros for emitting devtools.timeline events, taking the name of the
// event (e.g. "MyEvent"), function name for writing event metadata (usually
// my_event::Data) and the parameters to pass to the function (except the first
// perfetto::TracedValue param, which will be appended by this macro.
#define DEVTOOLS_TIMELINE_TRACE_EVENT_INSTANT_WITH_CATEGORIES(           \
    categories, event_name, function_name, ...)                          \
  TRACE_EVENT_INSTANT1(categories, event_name, TRACE_EVENT_SCOPE_THREAD, \
                       "data", [&](perfetto::TracedValue ctx) {          \
                         function_name(std::move(ctx), __VA_ARGS__);     \
                       })

#define DEVTOOLS_TIMELINE_TRACE_EVENT_WITH_CATEGORIES(categories, event_name, \
                                                      function_name, ...)     \
  TRACE_EVENT1(categories, event_name, "data",                                \
               [&](perfetto::TracedValue ctx) {                               \
                 function_name(std::move(ctx), __VA_ARGS__);                  \
               })

#define DEVTOOLS_TIMELINE_TRACE_EVENT_INSTANT(...)                           \
  DEVTOOLS_TIMELINE_TRACE_EVENT_INSTANT_WITH_CATEGORIES("devtools.timeline", \
                                                        __VA_ARGS__)

#define DEVTOOLS_TIMELINE_TRACE_EVENT(...)                           \
  DEVTOOLS_TIMELINE_TRACE_EVENT_WITH_CATEGORIES("devtools.timeline", \
                                                __VA_ARGS__)

namespace inspector_layout_event {
void BeginData(perfetto::TracedValue context, LocalFrameView*);
void EndData(perfetto::TracedValue context,
             const HeapVector<LayoutObjectWithDepth>&);
}  // namespace inspector_layout_event

namespace inspector_schedule_style_invalidation_tracking_event {
extern const char kAttribute[];
extern const char kClass[];
extern const char kId[];
extern const char kPseudo[];
extern const char kRuleSet[];

void AttributeChange(perfetto::TracedValue context,
                     Element&,
                     const InvalidationSet&,
                     const QualifiedName&);
void ClassChange(perfetto::TracedValue context,
                 Element&,
                 const InvalidationSet&,
                 const AtomicString&);
void IdChange(perfetto::TracedValue context,
              Element&,
              const InvalidationSet&,
              const AtomicString&);
void PseudoChange(perfetto::TracedValue context,
                  Element&,
                  const InvalidationSet&,
                  CSSSelector::PseudoType);
}  // namespace inspector_schedule_style_invalidation_tracking_event

#define TRACE_SCHEDULE_STYLE_INVALIDATION(element, invalidationSet,        \
                                          changeType, ...)                 \
  DEVTOOLS_TIMELINE_TRACE_EVENT_INSTANT_WITH_CATEGORIES(                   \
      TRACE_DISABLED_BY_DEFAULT("devtools.timeline.invalidationTracking"), \
      "ScheduleStyleInvalidationTracking",                                 \
      inspector_schedule_style_invalidation_tracking_event::changeType,    \
      (element), (invalidationSet), ##__VA_ARGS__);

namespace inspector_style_recalc_invalidation_tracking_event {
void Data(perfetto::TracedValue context,
          Node*,
          StyleChangeType,
          const StyleChangeReasonForTracing&);
}

namespace inspector_style_resolver_resolve_style_event {
void Data(perfetto::TracedValue context, Element*, PseudoId);
}

String DescendantInvalidationSetToIdString(const InvalidationSet&);

namespace inspector_style_invalidator_invalidate_event {
extern const char kElementHasPendingInvalidationList[];
extern const char kInvalidateCustomPseudo[];
extern const char kInvalidationSetInvalidatesSelf[];
extern const char kInvalidationSetInvalidatesSubtree[];
extern const char kInvalidationSetMatchedAttribute[];
extern const char kInvalidationSetMatchedClass[];
extern const char kInvalidationSetMatchedId[];
extern const char kInvalidationSetMatchedTagName[];
extern const char kInvalidationSetMatchedPart[];
extern const char kInvalidationSetInvalidatesTreeCounting[];

void Data(perfetto::TracedValue context, Element&, const char* reason);
void SelectorPart(perfetto::TracedValue context,
                  Element&,
                  const char* reason,
                  const InvalidationSet&,
                  const AtomicString&);
void InvalidationList(perfetto::TracedValue context,
                      ContainerNode&,
                      const Vector<scoped_refptr<InvalidationSet>>&);
}  // namespace inspector_style_invalidator_invalidate_event

#define TRACE_STYLE_INVALIDATOR_INVALIDATION(element, reason)              \
  DEVTOOLS_TIMELINE_TRACE_EVENT_INSTANT_WITH_CATEGORIES(                   \
      TRACE_DISABLED_BY_DEFAULT("devtools.timeline.invalidationTracking"), \
      "StyleInvalidatorInvalidationTracking",                              \
      inspector_style_invalidator_invalidate_event::Data, (element),       \
      (inspector_style_invalidator_invalidate_event::reason))

#define TRACE_STYLE_INVALIDATOR_INVALIDATION_SELECTORPART(                   \
    element, reason, invalidationSet, singleSelectorPart)                    \
  DEVTOOLS_TIMELINE_TRACE_EVENT_INSTANT_WITH_CATEGORIES(                     \
      TRACE_DISABLED_BY_DEFAULT("devtools.timeline.invalidationTracking"),   \
      "StyleInvalidatorInvalidationTracking",                                \
      inspector_style_invalidator_invalidate_event::SelectorPart, (element), \
      (inspector_style_invalidator_invalidate_event::reason),                \
      (invalidationSet), (singleSelectorPart))

#define TRACE_STYLE_INVALIDATOR_INVALIDATION_SET(element, reason, \
                                                 invalidationSet) \
  TRACE_STYLE_INVALIDATOR_INVALIDATION_SELECTORPART(              \
      element, reason, invalidationSet, g_empty_atom)

// From a web developer's perspective: what caused this layout? This is strictly
// for tracing. Blink logic must not depend on these.
namespace layout_invalidation_reason {
extern const char kUnknown[];
extern const char kSizeChanged[];
extern const char kAncestorMoved[];
extern const char kStyleChange[];
extern const char kDomChanged[];
extern const char kTextChanged[];
extern const char kPrintingChanged[];
extern const char kPaintPreview[];
extern const char kAttributeChanged[];
extern const char kColumnsChanged[];
extern const char kChildAnonymousBlockChanged[];
extern const char kAnonymousBlockChange[];
extern const char kFontsChanged[];
extern const char kFullscreen[];
extern const char kChildChanged[];
extern const char kListValueChange[];
extern const char kListStyleTypeChange[];
extern const char kCounterStyleChange[];
extern const char kImageChanged[];
extern const char kSliderValueChanged[];
extern const char kAncestorMarginCollapsing[];
extern const char kFieldsetChanged[];
extern const char kTextAutosizing[];
extern const char kSvgResourceInvalidated[];
extern const char kFloatDescendantChanged[];
extern const char kCountersChanged[];
extern const char kGridChanged[];
extern const char kMenuOptionsChanged[];
extern const char kRemovedFromLayout[];
extern const char kAddedToLayout[];
extern const char kTableChanged[];
extern const char kPaddingChanged[];
extern const char kTextControlChanged[];
// FIXME: This is too generic, we should be able to split out transform and
// size related invalidations.
extern const char kSvgChanged[];
extern const char kScrollbarChanged[];
extern const char kDisplayLock[];
extern const char kDevtools[];
extern const char kAnchorPositioning[];
extern const char kScrollMarkersChanged[];
extern const char kOutOfFlowAlignmentChanged[];
}  // namespace layout_invalidation_reason

// LayoutInvalidationReasonForTracing is strictly for tracing. Blink logic must
// not depend on this value.
typedef const char LayoutInvalidationReasonForTracing[];

namespace inspector_layout_invalidation_tracking_event {
CORE_EXPORT
void Data(perfetto::TracedValue context,
          const LayoutObject*,
          LayoutInvalidationReasonForTracing);
}

namespace inspector_change_resource_priority_event {
void Data(perfetto::TracedValue context,
          DocumentLoader*,
          uint64_t identifier,
          const ResourceLoadPriority&);
}

namespace inspector_send_request_event {
void Data(perfetto::TracedValue context,
          ExecutionContext* execution_context,
          DocumentLoader*,
          uint64_t identifier,
          LocalFrame*,
          const ResourceRequest&,
          ResourceType resource_type,
          RenderBlockingBehavior,
          const ResourceLoaderOptions&);
}

namespace inspector_change_render_blocking_behavior_event {
void Data(perfetto::TracedValue context,
          DocumentLoader*,
          uint64_t identifier,
          const ResourceRequestHead&,
          RenderBlockingBehavior);
}

namespace inspector_send_navigation_request_event {
void Data(perfetto::TracedValue context,
          DocumentLoader*,
          uint64_t identifier,
          LocalFrame*,
          const KURL&,
          const AtomicString& http_method);
}

namespace inspector_receive_response_event {
void Data(perfetto::TracedValue context,
          DocumentLoader*,
          uint64_t identifier,
          LocalFrame*,
          const ResourceResponse&);
}

namespace inspector_receive_data_event {
void Data(perfetto::TracedValue context,
          DocumentLoader*,
          uint64_t identifier,
          LocalFrame*,
          uint64_t encoded_data_length);
}

namespace inspector_resource_finish_event {
void Data(perfetto::TracedValue context,
          DocumentLoader*,
          uint64_t identifier,
          base::TimeTicks finish_time,
          bool did_fail,
          int64_t encoded_data_length,
          int64_t decoded_body_length);
}

namespace inspector_mark_resource_cached_event {
void Data(perfetto::TracedValue context, DocumentLoader*, uint64_t identifier);
}

namespace inspector_timer_install_event {
CORE_EXPORT void Data(perfetto::TracedValue context,
                      ExecutionContext*,
                      int timer_id,
                      base::TimeDelta timeout,
                      bool single_shot);
}

namespace inspector_timer_remove_event {
CORE_EXPORT
void Data(perfetto::TracedValue context, ExecutionContext*, int timer_id);
}

namespace inspector_timer_fire_event {
CORE_EXPORT
void Data(perfetto::TracedValue context, ExecutionContext*, int timer_id);
}

namespace inspector_idle_callback_request_event {
void Data(perfetto::TracedValue context,
          ExecutionContext*,
          int id,
          double timeout);
}

namespace inspector_idle_callback_cancel_event {
void Data(perfetto::TracedValue context, ExecutionContext*, int id);
}

namespace inspector_idle_callback_fire_event {
void Data(perfetto::TracedValue context,
          ExecutionContext*,
          int id,
          double allotted_milliseconds,
          bool timed_out);
}

namespace inspector_animation_frame_event {
void Data(perfetto::TracedValue context, ExecutionContext*, int callback_id);
}

namespace inspector_parse_author_style_sheet_event {
void Data(perfetto::TracedValue context, const CSSStyleSheetResource*);
}

namespace inspector_xhr_ready_state_change_event {
void Data(perfetto::TracedValue context, ExecutionContext*, XMLHttpRequest*);
}

namespace inspector_xhr_load_event {
void Data(perfetto::TracedValue context, ExecutionContext*, XMLHttpRequest*);
}

namespace inspector_paint_event {
void Data(perfetto::TracedValue context,
          LocalFrame*,
          const LayoutObject*,
          const gfx::Rect& contents_cull_rect);
}

namespace inspector_paint_image_event {
void Data(perfetto::TracedValue context,
          const LayoutImage&,
          const gfx::RectF& src_rect,
          const gfx::RectF& dest_rect);
void Data(perfetto::TracedValue context,
          const LayoutObject&,
          const StyleImage&);
void Data(perfetto::TracedValue context,
          Node*,
          const StyleImage&,
          const gfx::RectF& src_rect,
          const gfx::RectF& dest_rect);
void Data(perfetto::TracedValue context,
          const LayoutObject*,
          const ImageResourceContent&);
}  // namespace inspector_paint_image_event

namespace inspector_commit_load_event {
void Data(perfetto::TracedValue context, LocalFrame*);
}

namespace inspector_layerize_event {
void Data(perfetto::TracedValue context, LocalFrame*);
}

namespace inspector_mark_load_event {
void Data(perfetto::TracedValue context, LocalFrame*);
}

namespace inspector_scroll_layer_event {
void Data(perfetto::TracedValue context, LayoutObject*);
}

namespace inspector_pre_paint_event {
void Data(perfetto::TracedValue context, LocalFrame*);
}

namespace inspector_evaluate_script_event {
void Data(perfetto::TracedValue context,
          v8::Isolate*,
          LocalFrame*,
          const String& url,
          const WTF::TextPosition&);
}

namespace inspector_target_rundown_event {

void Data(perfetto::TracedValue context,
          ExecutionContext* execution_context,
          v8::Isolate* isolate,
          ScriptState* script_state,
          int scriptId);
}

namespace inspector_parse_script_event {
void Data(perfetto::TracedValue context,
          uint64_t identifier,
          const String& url);
}

namespace inspector_deserialize_script_event {
void Data(perfetto::TracedValue context,
          uint64_t identifier,
          const String& url);
}

namespace inspector_compile_script_event {

struct V8ConsumeCacheResult {
  V8ConsumeCacheResult(int cache_size, bool rejected, bool full);
  int cache_size;
  bool rejected;
  bool full;
};

void Data(perfetto::TracedValue context,
          const String& url,
          const WTF::TextPosition&,
          std::optional<V8ConsumeCacheResult>,
          bool eager,
          bool streamed,
          ScriptStreamer::NotStreamingReason);
}  // namespace inspector_compile_script_event

namespace inspector_produce_script_cache_event {
void Data(perfetto::TracedValue context,
          const String& url,
          const WTF::TextPosition&,
          int cache_size);
}

namespace inspector_function_call_event {
void Data(perfetto::TracedValue context,
          ExecutionContext*,
          const v8::Local<v8::Function>&);
}

namespace inspector_update_counters_event {
void Data(perfetto::TracedValue context, v8::Isolate* isolate);
}

namespace inspector_invalidate_layout_event {
void Data(perfetto::TracedValue context, LocalFrame*, DOMNodeId);
}

namespace inspector_recalculate_styles_event {
void Data(perfetto::TracedValue context, LocalFrame*);
}

namespace inspector_event_dispatch_event {
void Data(perfetto::TracedValue context, const Event&, v8::Isolate*);
}

namespace inspector_time_stamp_event {
void Data(perfetto::TracedValue context,
          ExecutionContext*,
          const String& message,
          const v8::LocalVector<v8::Value>& args);
}

namespace inspector_tracing_session_id_for_worker_event {
void Data(perfetto::TracedValue context,
          const base::UnguessableToken& worker_devtools_token,
          const base::UnguessableToken& parent_devtools_token,
          const KURL& url,
          PlatformThreadId worker_thread_id);
}

namespace inspector_tracing_started_in_frame {
void Data(perfetto::TracedValue context, const String& session_id, LocalFrame*);
}

namespace inspector_set_layer_tree_id {
void Data(perfetto::TracedValue context, LocalFrame* local_root);
}

namespace inspector_dom_stats {
void Data(perfetto::TracedValue context, LocalFrame* local_root);
}

namespace inspector_animation_event {
void Data(perfetto::TracedValue context, const Animation&);
}

namespace inspector_animation_state_event {
void Data(perfetto::TracedValue context, const Animation&);
}

namespace inspector_animation_compositor_event {
void Data(perfetto::TracedValue context,
          blink::CompositorAnimations::FailureReasons failure_reasons,
          const blink::PropertyHandleSet& unsupported_properties_for_tracing);
}

namespace inspector_hit_test_event {
void EndData(perfetto::TracedValue context,
             const HitTestRequest&,
             const HitTestLocation&,
             const HitTestResult&);
}

namespace inspector_async_task {
void Data(perfetto::TracedValue context, const StringView&);
}

namespace inspector_schedule_post_message_event {
void Data(perfetto::TracedValue context,
          ExecutionContext* execution_context,
          uint64_t trace_id);
}

namespace inspector_handle_post_message_event {
void Data(perfetto::TracedValue context,
          ExecutionContext* execution_context,
          const MessageEvent& event);
}

namespace inspector_scheduler_schedule_event {
void Data(perfetto::TracedValue trace_context,
          ExecutionContext* execution_context,
          uint64_t task_id,
          WebSchedulingPriority priority,
          std::optional<double> delay = std::nullopt);
}

namespace inspector_scheduler_run_event {
void Data(perfetto::TracedValue trace_context,
          ExecutionContext* execution_context,
          uint64_t task_id,
          WebSchedulingPriority priority,
          std::optional<double> delay = std::nullopt);
}

namespace inspector_scheduler_abort_event {
void Data(perfetto::TracedValue trace_context,
          ExecutionContext* execution_context,
          uint64_t task_id);
}

CORE_EXPORT String ToHexString(const void* p);
CORE_EXPORT void SetCallStack(v8::Isolate* isolate,
                              perfetto::TracedDictionary&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_TRACE_EVENTS_H_
