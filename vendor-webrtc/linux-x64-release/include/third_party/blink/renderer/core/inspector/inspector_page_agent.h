/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_PAGE_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_PAGE_AGENT_H_

#include <optional>

#include "third_party/blink/public/mojom/loader/same_document_navigation_type.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/ad_tracker.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/protocol/page.h"
#include "third_party/blink/renderer/core/loader/frame_loader_types.h"
#include "third_party/blink/renderer/core/page/chrome_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8-inspector.h"

namespace blink {

class GenericFontFamilySettings;

namespace probe {
class RecalculateStyle;
class UpdateLayout;
}  // namespace probe

class Resource;
class Document;
class DocumentLoader;
enum class FrameDetachType;
class InspectedFrames;
class InspectorResourceContentLoader;
class LocalFrame;
class ClassicScript;
enum class ResourceType : uint8_t;

class CORE_EXPORT InspectorPageAgent final
    : public InspectorBaseAgent<protocol::Page::Metainfo> {
 public:
  class Client {
   public:
    virtual ~Client() = default;
    virtual void PageLayoutInvalidated(bool resized) {}
    virtual void WaitForDebugger() {}
    virtual bool IsPausedForNewWindow() { return false; }
  };

  enum ResourceType {
    kDocumentResource,
    kStylesheetResource,
    kImageResource,
    kFontResource,
    kMediaResource,
    kScriptResource,
    kTextTrackResource,
    kXHRResource,
    kFetchResource,
    kEventSourceResource,
    kWebSocketResource,
    kManifestResource,
    kSignedExchangeResource,
    kPingResource,
    kOtherResource
  };

  static bool CachedResourceContent(const Resource*,
                                    String* result,
                                    bool* base64_encoded,
                                    bool* was_cached);
  static bool SegmentedBufferContent(const SegmentedBuffer*,
                                     const String& mime_type,
                                     const String& text_encoding_name,
                                     String* result,
                                     bool* base64_encoded);

  static String ResourceTypeJson(ResourceType);
  static ResourceType ToResourceType(const blink::ResourceType);
  static String CachedResourceTypeJson(const Resource&);

  InspectorPageAgent(InspectedFrames*,
                     Client*,
                     InspectorResourceContentLoader*,
                     v8_inspector::V8InspectorSession*,
                     const String& script_to_evaluate_on_load);
  InspectorPageAgent(const InspectorPageAgent&) = delete;
  InspectorPageAgent& operator=(const InspectorPageAgent&) = delete;

  // Page API for frontend
  protocol::Response enable(
      std::optional<bool> enable_file_chooser_opened_event) override;
  protocol::Response disable() override;
  protocol::Response addScriptToEvaluateOnLoad(const String& script_source,
                                               String* identifier) override;
  protocol::Response removeScriptToEvaluateOnLoad(
      const String& identifier) override;
  protocol::Response addScriptToEvaluateOnNewDocument(
      const String& source,
      std::optional<String> world_name,
      std::optional<bool> include_command_line_api,
      std::optional<bool> runImmediately,
      String* identifier) override;
  protocol::Response removeScriptToEvaluateOnNewDocument(
      const String& identifier) override;
  protocol::Response setLifecycleEventsEnabled(bool) override;
  protocol::Response reload(std::optional<bool> bypass_cache,
                            std::optional<String> script_to_evaluate_on_load,
                            std::optional<String> loader_id) override;
  protocol::Response stopLoading() override;
  protocol::Response setAdBlockingEnabled(bool) override;
  protocol::Response getResourceTree(
      std::unique_ptr<protocol::Page::FrameResourceTree>* frame_tree) override;
  protocol::Response getFrameTree(
      std::unique_ptr<protocol::Page::FrameTree>*) override;
  void getResourceContent(const String& frame_id,
                          const String& url,
                          std::unique_ptr<GetResourceContentCallback>) override;
  protocol::Response getAdScriptAncestryIds(
      const String& frame_id,
      std::unique_ptr<protocol::Array<protocol::Page::AdScriptId>>*
          out_ad_script_ancestry) override;
  void searchInResource(const String& frame_id,
                        const String& url,
                        const String& query,
                        std::optional<bool> case_sensitive,
                        std::optional<bool> is_regex,
                        std::unique_ptr<SearchInResourceCallback>) override;
  protocol::Response setDocumentContent(const String& frame_id,
                                        const String& html) override;
  protocol::Response setBypassCSP(bool enabled) override;

  protocol::Response getPermissionsPolicyState(
      const String& frame_id,
      std::unique_ptr<
          protocol::Array<protocol::Page::PermissionsPolicyFeatureState>>*)
      override;
  protocol::Response getOriginTrials(
      const String& frame_id,
      std::unique_ptr<protocol::Array<protocol::Page::OriginTrial>>*) override;

  protocol::Response startScreencast(
      std::optional<String> format,
      std::optional<int> quality,
      std::optional<int> max_width,
      std::optional<int> max_height,
      std::optional<int> every_nth_frame) override;
  protocol::Response stopScreencast() override;
  protocol::Response getLayoutMetrics(
      std::unique_ptr<protocol::Page::LayoutViewport>* out_layout_viewport,
      std::unique_ptr<protocol::Page::VisualViewport>* out_visual_viewport,
      std::unique_ptr<protocol::DOM::Rect>* out_content_size,
      std::unique_ptr<protocol::Page::LayoutViewport>* out_css_layout_viewport,
      std::unique_ptr<protocol::Page::VisualViewport>* out_css_visual_viewport,
      std::unique_ptr<protocol::DOM::Rect>* out_css_content_size) override;
  void createIsolatedWorld(
      const String& frame_id,
      std::optional<String> world_name,
      std::optional<bool> grant_universal_access,
      std::unique_ptr<CreateIsolatedWorldCallback>) override;
  protocol::Response setFontFamilies(
      std::unique_ptr<protocol::Page::FontFamilies>,
      std::unique_ptr<protocol::Array<protocol::Page::ScriptFontFamilies>>
          forScripts) override;
  protocol::Response setFontSizes(
      std::unique_ptr<protocol::Page::FontSizes>) override;
  protocol::Response generateTestReport(const String& message,
                                        std::optional<String> group) override;

  protocol::Response produceCompilationCache(
      std::unique_ptr<protocol::Array<protocol::Page::CompilationCacheParams>>
          scripts) override;
  protocol::Response addCompilationCache(const String& url,
                                         const protocol::Binary& data) override;
  protocol::Response clearCompilationCache() override;
  protocol::Response waitForDebugger() override;
  protocol::Response setInterceptFileChooserDialog(
      bool enabled,
      std::optional<bool> cancel) override;

  // InspectorInstrumentation API
  void DidCreateMainWorldContext(LocalFrame*);
  void DidNavigateWithinDocument(LocalFrame*,
                                 mojom::blink::SameDocumentNavigationType);
  void DomContentLoadedEventFired(LocalFrame*);
  void LoadEventFired(LocalFrame*);
  void WillCommitLoad(LocalFrame*, DocumentLoader*);
  void DidRestoreFromBackForwardCache(LocalFrame*);
  void DidOpenDocument(LocalFrame*, DocumentLoader*);
  void FrameAttachedToParent(
      LocalFrame*,
      const Vector<AdScriptIdentifier>& ad_script_ancestry);
  void FrameDetachedFromParent(LocalFrame*, FrameDetachType);
  void FrameSubtreeWillBeDetached(Frame* frame);
  void FrameStoppedLoading(LocalFrame*);
  void FrameRequestedNavigation(Frame* target_frame,
                                const KURL&,
                                ClientNavigationReason,
                                NavigationPolicy);
  void FrameScheduledNavigation(LocalFrame*,
                                const KURL&,
                                base::TimeDelta delay,
                                ClientNavigationReason);
  void FrameClearedScheduledNavigation(LocalFrame*);
  void WillRunJavaScriptDialog();
  void DidRunJavaScriptDialog();
  void DidResizeMainFrame();
  void DidChangeViewport();
  void LifecycleEvent(LocalFrame*,
                      DocumentLoader*,
                      const char* name,
                      double timestamp);
  void PaintTiming(Document*, const char* name, double timestamp);
  void Will(const probe::UpdateLayout&);
  void Did(const probe::UpdateLayout&);
  void Will(const probe::RecalculateStyle&);
  void Did(const probe::RecalculateStyle&);
  void WindowOpen(const KURL&,
                  const AtomicString&,
                  const WebWindowFeatures&,
                  bool);
  void ApplyCompilationModeOverride(const ClassicScript&,
                                    v8::ScriptCompiler::CachedData**,
                                    v8::ScriptCompiler::CompileOptions*);
  void DidProduceCompilationCache(const ClassicScript&,
                                  v8::Local<v8::Script> script);
  void FileChooserOpened(LocalFrame* frame,
                         HTMLInputElement* element,
                         bool multiple,
                         bool* suppressed,
                         bool* canceled);

  // Inspector Controller API
  void Restore() override;
  bool ScreencastEnabled();

  void Trace(Visitor*) const override;
  void Dispose() override;

 private:
  struct IsolatedWorldRequest {
    IsolatedWorldRequest() = delete;
    IsolatedWorldRequest(String world_name,
                         bool grant_universal_access,
                         std::unique_ptr<CreateIsolatedWorldCallback> callback)
        : world_name(world_name),
          grant_universal_access(grant_universal_access),
          callback(std::move(callback)) {}

    const String world_name;
    const bool grant_universal_access;
    std::unique_ptr<CreateIsolatedWorldCallback> callback;
  };

  void GetResourceContentAfterResourcesContentLoaded(
      const String& frame_id,
      const String& url,
      std::unique_ptr<GetResourceContentCallback>);
  void SearchContentAfterResourcesContentLoaded(
      const String& frame_id,
      const String& url,
      const String& query,
      bool case_sensitive,
      bool is_regex,
      std::unique_ptr<SearchInResourceCallback>);
  DOMWrapperWorld* EnsureDOMWrapperWorld(LocalFrame* frame,
                                         const String& world_name,
                                         bool grant_universal_access);

  static KURL UrlWithoutFragment(const KURL&);

  void PageLayoutInvalidated(bool resized);

  protocol::Response setFontFamilies(
      GenericFontFamilySettings& family_settings,
      const protocol::Array<protocol::Page::ScriptFontFamilies>& forScripts);
  std::unique_ptr<protocol::Page::Frame> BuildObjectForFrame(LocalFrame*);
  std::unique_ptr<protocol::Page::FrameTree> BuildObjectForFrameTree(
      LocalFrame*);
  std::unique_ptr<protocol::Page::FrameResourceTree> BuildObjectForResourceTree(
      LocalFrame*);
  void CreateIsolatedWorldImpl(LocalFrame& frame,
                               String world_name,
                               bool grant_universal_access,
                               std::unique_ptr<CreateIsolatedWorldCallback>);
  void EvaluateScriptOnNewDocument(LocalFrame&,
                                   const String& script_identifier);

  Member<InspectedFrames> inspected_frames_;
  HashMap<String, protocol::Binary> compilation_cache_;
  // TODO(caseq): should this be stored as InspectorAgentState::StringMap
  // instead? Current use cases do not require this, but we might eventually
  // reconsider. Value is true iff eager compilation requested.
  HashMap<String, bool> requested_compilation_cache_;

  HeapHashMap<WeakMember<LocalFrame>, Vector<IsolatedWorldRequest>>
      pending_isolated_worlds_;
  using FrameIsolatedWorlds = GCedHeapHashMap<String, Member<DOMWrapperWorld>>;
  HeapHashMap<WeakMember<LocalFrame>, Member<FrameIsolatedWorlds>>
      isolated_worlds_;
  HashMap<String, Vector<AdScriptIdentifier>> frame_ad_script_ancestry_;
  v8_inspector::V8InspectorSession* v8_session_;
  Client* client_;
  Member<InspectorResourceContentLoader> inspector_resource_content_loader_;
  int resource_content_loader_client_id_;
  InspectorAgentState::Boolean suppress_file_chooser_;
  InspectorAgentState::Boolean cancel_file_chooser_;
  InspectorAgentState::Boolean enabled_;
  InspectorAgentState::Boolean enable_file_chooser_opened_event_;
  InspectorAgentState::Boolean screencast_enabled_;
  InspectorAgentState::Boolean lifecycle_events_enabled_;
  InspectorAgentState::Boolean bypass_csp_enabled_;
  InspectorAgentState::StringMap scripts_to_evaluate_on_load_;
  InspectorAgentState::StringMap worlds_to_evaluate_on_load_;
  InspectorAgentState::BooleanMap
      include_command_line_api_for_scripts_to_evaluate_on_load_;
  InspectorAgentState::Integer standard_font_size_;
  InspectorAgentState::Integer fixed_font_size_;
  InspectorAgentState::Bytes script_font_families_cbor_;
  String script_injection_on_load_once_;
  String pending_script_injection_on_load_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_PAGE_AGENT_H_
