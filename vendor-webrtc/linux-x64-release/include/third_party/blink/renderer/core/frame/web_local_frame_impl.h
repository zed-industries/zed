/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WEB_LOCAL_FRAME_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WEB_LOCAL_FRAME_IMPL_H_

#include <memory>
#include <set>
#include <utility>
#include <vector>

#include "base/dcheck_is_on.h"
#include "base/observer_list.h"
#include "base/task/single_thread_task_runner.h"
#include "base/types/pass_key.h"
#include "build/build_config.h"
#include "mojo/public/cpp/bindings/pending_associated_receiver.h"
#include "mojo/public/cpp/bindings/pending_associated_remote.h"
#include "services/network/public/mojom/web_sandbox_flags.mojom-blink.h"
#include "third_party/blink/public/common/context_menu_data/context_menu_data.h"
#include "third_party/blink/public/common/features.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/back_forward_cache_not_restored_reasons.mojom.h"
#include "third_party/blink/public/mojom/blob/blob_url_store.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/context_menu/context_menu.mojom-blink.h"
#include "third_party/blink/public/mojom/devtools/devtools_agent.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/devtools/inspector_issue.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/fenced_frame/fenced_frame.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/find_in_page.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/lifecycle.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/tree_scope_type.mojom-blink.h"
#include "third_party/blink/public/mojom/input/input_handler.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/lcp_critical_path_predictor/lcp_critical_path_predictor.mojom-blink.h"
#include "third_party/blink/public/mojom/page/widget.mojom-blink.h"
#include "third_party/blink/public/mojom/script/script_evaluation_params.mojom-blink-forward.h"
#include "third_party/blink/public/platform/web_content_settings_client.h"
#include "third_party/blink/public/platform/web_file_system_type.h"
#include "third_party/blink/public/web/web_history_commit_type.h"
#include "third_party/blink/public/web/web_local_frame.h"
#include "third_party/blink/public/web/web_local_frame_observer.h"
#include "third_party/blink/public/web/web_navigation_control.h"
#include "third_party/blink/public/web/web_performance_metrics_for_nested_contexts.h"
#include "third_party/blink/public/web/web_performance_metrics_for_reporting.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/exported/web_input_method_controller_impl.h"
#include "third_party/blink/renderer/core/frame/local_frame.h"
#include "third_party/blink/renderer/core/frame/web_remote_frame_impl.h"
#include "third_party/blink/renderer/platform/heap/self_keep_alive.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class ChromePrintContext;
class FindInPage;
class HTMLFencedFrameElement;
class LocalFrameClientImpl;
class ResourceError;
class ScrollableArea;
class StorageKey;
class TextFinder;
class WebAssociatedURLLoader;
class WebAutofillClient;
class WebContentSettingsClient;
class WebDevToolsAgentImpl;
class WebFrameWidgetImpl;
class WebLocalFrameClient;
class WebNode;
class WebPerformanceMetricsForNestedContexts;
class WebPerformanceMetricsForReporting;
class WebRemoteFrameImpl;
class WebSpellCheckPanelHostClient;
class WebView;
class WebViewImpl;
class WindowAgentFactory;
enum class WebFrameLoadType;
struct ContextMenuData;
struct WebAssociatedURLLoaderOptions;
struct WebPrintParams;

// Implementation of WebFrame, note that this is a reference counted object.
class CORE_EXPORT WebLocalFrameImpl final
    : public GarbageCollected<WebLocalFrameImpl>,
      public WebNavigationControl {
 public:
  // WebFrame overrides:
  void Close(DetachReason detach_reason) override;
  WebView* View() const override;
  v8::Local<v8::Object> GlobalProxy(v8::Isolate* isolate) const override;
  bool IsLoading() const override;

  // WebLocalFrame overrides:
  WebLocalFrameImpl* CreateLocalChild(
      mojom::blink::TreeScopeType,
      WebLocalFrameClient*,
      InterfaceRegistry*,
      const LocalFrameToken& frame_token) override;
  WebLocalFrameClient* Client() const override { return client_; }
  void SetAutofillClient(WebAutofillClient*) override;
  WebAutofillClient* AutofillClient() override;
  void SetContentCaptureClient(WebContentCaptureClient*) override;
  WebContentCaptureClient* ContentCaptureClient() const override;
  BrowserInterfaceBrokerProxy& GetBrowserInterfaceBroker() override;
  WebDocument GetDocument() const override;
  WebString AssignedName() const override;
  ui::AXTreeID GetAXTreeID() const override;
  void SetName(const WebString&) override;
  bool IsProvisional() const override;
  WebLocalFrameImpl* LocalRoot() override;
  WebFrameWidget* FrameWidget() const override;
  WebFrame* FindFrameByName(const WebString& name) override;
  void SetEmbeddingToken(
      const base::UnguessableToken& embedding_token) override;
  const std::optional<base::UnguessableToken>& GetEmbeddingToken()
      const override;
  bool IsInFencedFrameTree() const override;
  void SendPings(const WebURL& destination_url) override;
  void SendAttributionSrc(const std::optional<Impression>&,
                          bool did_navigate) override;
  void StartReload(WebFrameLoadType) override;
  void ClearActiveFindMatchForTesting() override;
  void EnableViewSourceMode(bool enable) override;
  bool IsViewSourceModeEnabled() const override;
  WebDocumentLoader* GetDocumentLoader() const override;
  void SetReferrerForRequest(WebURLRequest&, const WebURL& referrer) override;
  bool IsNavigationScheduledWithin(base::TimeDelta interval) const override;
  void BlinkFeatureUsageReport(blink::mojom::WebFeature feature) override;
  WebPrintPageDescription GetPageDescription(uint32_t page_index) override;
  void ExecuteScript(const WebScriptSource&) override;
  void ExecuteScriptInIsolatedWorld(
      int32_t world_id,
      const WebScriptSource&,
      BackForwardCacheAware back_forward_cache_aware) override;
  [[nodiscard]] v8::Local<v8::Value> ExecuteScriptInIsolatedWorldAndReturnValue(
      int32_t world_id,
      const WebScriptSource&,
      BackForwardCacheAware back_forward_cache_aware) override;
  void ClearIsolatedWorldCSPForTesting(int32_t world_id) override;
  v8::Local<v8::Value> ExecuteScriptAndReturnValue(
      const WebScriptSource&) override;
  // Call the function with the given receiver and arguments
  v8::MaybeLocal<v8::Value> ExecuteMethodAndReturnValue(
      v8::Local<v8::Function>,
      v8::Local<v8::Value>,
      int argc,
      v8::Local<v8::Value> argv[]) override;
  v8::MaybeLocal<v8::Value> CallFunctionEvenIfScriptDisabled(
      v8::Local<v8::Function>,
      v8::Local<v8::Value>,
      int argc,
      v8::Local<v8::Value> argv[]) override;
  v8::Local<v8::Context> MainWorldScriptContext() const override;
  int32_t GetScriptContextWorldId(
      v8::Local<v8::Context> script_context) const override;
  v8::Local<v8::Context> GetScriptContextFromWorldId(
      v8::Isolate* isolate,
      int world_id) const override;
  void RequestExecuteV8Function(v8::Local<v8::Context>,
                                v8::Local<v8::Function>,
                                v8::Local<v8::Value> receiver,
                                int argc,
                                v8::Local<v8::Value> argv[],
                                WebScriptExecutionCallback) override;
  void RequestExecuteScript(int32_t world_id,
                            base::span<const WebScriptSource> sources,
                            mojom::blink::UserActivationOption,
                            mojom::blink::EvaluationTiming,
                            mojom::blink::LoadEventBlockingOption,
                            WebScriptExecutionCallback,
                            BackForwardCacheAware back_forward_cache_aware,
                            mojom::blink::WantResultOption,
                            mojom::blink::PromiseResultOption) override;
  bool IsInspectorConnected() override;
  void Alert(const WebString& message) override;
  bool Confirm(const WebString& message) override;
  WebString Prompt(const WebString& message,
                   const WebString& default_value) override;
  void GenerateInterventionReport(const WebString& message_id,
                                  const WebString& message) override;
  void UnmarkText() override;
  bool HasMarkedText() const override;
  WebRange MarkedRange() const override;
  bool FirstRectForCharacterRange(uint32_t location,
                                  uint32_t length,
                                  gfx::Rect&) const override;
  bool ExecuteCommand(const WebString&) override;
  bool ExecuteCommand(const WebString&, const WebString& value) override;
  bool IsCommandEnabled(const WebString&) const override;
  bool SelectionTextDirection(base::i18n::TextDirection& start,
                              base::i18n::TextDirection& end) const override;
  bool IsSelectionAnchorFirst() const override;
  void SetTextDirectionForTesting(base::i18n::TextDirection direction) override;
  bool HasSelection() const override;
  WebRange SelectionRange() const override;
  WebString SelectionAsText() const override;
  WebString SelectionAsMarkup() const override;
  void TextSelectionChanged(const WebString& selection_text,
                            uint32_t offset,
                            const gfx::Range& range) override;
  bool SelectAroundCaret(mojom::blink::SelectionGranularity granularity,
                         bool should_show_handle,
                         bool should_show_context_menu);
  EphemeralRange GetWordSelectionRangeAroundCaret() const;
  void SelectRange(const gfx::Point& base, const gfx::Point& extent) override;
  void SelectRange(const WebRange&,
                   HandleVisibilityBehavior,
                   blink::mojom::SelectionMenuBehavior,
                   SelectionSetFocusBehavior) override;
  WebString RangeAsText(const WebRange&) override;
  void MoveRangeSelection(
      const gfx::Point& base,
      const gfx::Point& extent,
      WebFrame::TextGranularity = kCharacterGranularity) override;
  void MoveCaretSelection(const gfx::Point&) override;

#if BUILDFLAG(IS_IOS)
  void StartAutoscrollForSelectionToPoint(const gfx::PointF&);
  void StopAutoscroll();
#endif  // BUILDFLAG(IS_IOS)

  bool SetEditableSelectionOffsets(int start, int end) override;
  bool AddImeTextSpansToExistingText(
      const std::vector<ui::ImeTextSpan>& ime_text_spans,
      unsigned text_start,
      unsigned text_end) override;
  bool ClearImeTextSpansByType(ui::ImeTextSpan::Type type,
                               unsigned text_start,
                               unsigned text_end) override;
  bool SetCompositionFromExistingText(
      int composition_start,
      int composition_end,
      const std::vector<ui::ImeTextSpan>& ime_text_spans) override;
  void ExtendSelectionAndDelete(int before, int after) override;
  void ExtendSelectionAndReplace(int before,
                                 int after,
                                 const WebString& replacement_text) override;
  void MoveRangeSelectionExtent(const gfx::Point&) override;
  void ReplaceSelection(const WebString&) override;
  void DeleteSurroundingText(int before, int after) override;
  void DeleteSurroundingTextInCodePoints(int before, int after) override;
  void SetTextCheckClient(WebTextCheckClient*) override;
  void SetSpellCheckPanelHostClient(WebSpellCheckPanelHostClient*) override;
  WebSpellCheckPanelHostClient* SpellCheckPanelHostClient() const override {
    return spell_check_panel_host_client_;
  }
  void ReplaceMisspelledRange(const WebString&) override;
  void RemoveSpellingMarkers() override;
  void RemoveSpellingMarkersUnderWords(
      const std::vector<WebString>& words) override;
  WebContentSettingsClient* GetContentSettingsClient() const override;
  void SetContentSettingsClient(WebContentSettingsClient*) override;
  const mojom::RendererContentSettingsPtr& GetContentSettings() const override;
  void ReloadImage(const WebNode&) override;
  bool IsAllowedToDownload() const override;
  bool IsCrossOriginToOutermostMainFrame() const override;
  bool FindForTesting(int identifier,
                      const WebString& search_text,
                      bool match_case,
                      bool forward,
                      bool force,
                      bool new_session,
                      bool wrap_within_frame,
                      bool async) override;
  void SetTickmarks(const WebElement& target,
                    const std::vector<gfx::Rect>& tickmarks) override;
  WebNode ContextMenuImageNode() const override;
  WebNode ContextMenuNode() const override;
  void CopyImageAtForTesting(const gfx::Point&) override;
  void ShowContextMenuFromExternal(
      const UntrustworthyContextMenuParams& params,
      CrossVariantMojoAssociatedRemote<
          mojom::blink::ContextMenuClientInterfaceBase> context_menu_client)
      override;
  void UsageCountChromeLoadTimes(const WebString& metric) override;
  void UsageCountChromeCSI(const WebString& metric) override;
  bool DispatchedPagehideAndStillHidden() const override;
  FrameScheduler* Scheduler() const override;
  scheduler::WebAgentGroupScheduler* GetAgentGroupScheduler() const override;
  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner(TaskType) override;
  WebInputMethodController* GetInputMethodController() override;
  std::unique_ptr<WebAssociatedURLLoader> CreateAssociatedURLLoader(
      const WebAssociatedURLLoaderOptions&) override;
  void DeprecatedStopLoading() override;
  gfx::PointF GetScrollOffset() const override;
  bool SetScrollOffset(const gfx::PointF&) override;
  gfx::Size DocumentSize() const override;
  bool HasVisibleContent() const override;
  gfx::Rect VisibleContentRect() const override;
  void DispatchBeforePrintEvent(
      base::WeakPtr<WebPrintClient> print_client) override;
  WebPlugin* GetPluginToPrint(const WebNode& constrain_to_node) override;
  uint32_t PrintBegin(const WebPrintParams&,
                      const WebNode& constrain_to_node) override;
  bool WillPrintSoon() override;
  void PrintPage(uint32_t page_to_print, cc::PaintCanvas*) override;
  void PrintEnd() override;
  void DispatchAfterPrintEvent() override;
  bool GetPrintPresetOptionsForPlugin(const WebNode&,
                                      WebPrintPresetOptions*) override;
  bool CapturePaintPreview(const gfx::Rect& bounds,
                           cc::PaintCanvas* canvas,
                           bool include_linked_destinations,
                           bool skip_accelerated_content) override;
  WebPerformanceMetricsForReporting PerformanceMetricsForReporting()
      const override;
  WebPerformanceMetricsForNestedContexts PerformanceMetricsForNestedContexts()
      const override;
  bool IsAdFrame() const override;
  bool IsAdScriptInStack() const override;
  void SetAdEvidence(const FrameAdEvidence& ad_evidence) override;
  const std::optional<blink::FrameAdEvidence>& AdEvidence() override;
  bool IsFrameCreatedByAdScript() override;
  gfx::Size SpoolSizeInPixelsForTesting(
      const std::vector<uint32_t>& pages) override;
  gfx::Size SpoolSizeInPixelsForTesting(uint32_t page_count) override;
  void PrintPagesForTesting(
      cc::PaintCanvas*,
      const gfx::Size& spool_size_in_pixels,
      const std::vector<uint32_t>* pages = nullptr) override;
  gfx::Rect GetSelectionBoundsRectForTesting() const override;
  gfx::Point GetPositionInViewportForTesting() const override;
  void WasHidden() override;
  void WasShown() override;
  void SetAllowsCrossBrowsingInstanceFrameLookup() override;
  void NotifyUserActivation(
      mojom::blink::UserActivationNotificationType notification_type) override;
  bool HasStickyUserActivation() override;
  bool HasTransientUserActivation() override;
  bool ConsumeTransientUserActivation(UserActivationUpdateSource) override;
  bool LastActivationWasRestricted() const override;
#if BUILDFLAG(IS_WIN)
  WebFontFamilyNames GetWebFontFamilyNames() const override;
#endif
  void SetTargetToCurrentHistoryItem(const WebString& target) override;
  void UpdateCurrentHistoryItem() override;
  PageState CurrentHistoryItemToPageState() override;
  WebHistoryItem GetCurrentHistoryItem() const override;
  void SetLocalStorageArea(
      CrossVariantMojoRemote<mojom::StorageAreaInterfaceBase>
          local_storage_area) override;
  void SetSessionStorageArea(
      CrossVariantMojoRemote<mojom::StorageAreaInterfaceBase>
          session_storage_area) override;
  void AddHitTestOnTouchStartCallback(
      base::RepeatingCallback<void(const WebHitTestResult&)> callback) override;
  void BlockParserForTesting() override;
  void ResumeParserForTesting() override;
  void FlushInputForTesting(base::OnceClosure done_callback) override;
  bool AllowStorageAccessSyncAndNotify(
      WebContentSettingsClient::StorageType storage_type) override;

  // WebNavigationControl overrides:
  bool DispatchBeforeUnloadEvent(bool) override;
  void CommitNavigation(
      std::unique_ptr<WebNavigationParams> navigation_params,
      std::unique_ptr<WebDocumentLoader::ExtraData> extra_data) override;
  blink::mojom::CommitResult CommitSameDocumentNavigation(
      const WebURL&,
      WebFrameLoadType,
      const WebHistoryItem&,
      bool is_client_redirect,
      bool has_transient_user_activation,
      const WebSecurityOrigin& initiator_origin,
      bool is_browser_initiated,
      bool has_ua_visual_transition,
      std::optional<scheduler::TaskAttributionId>
          soft_navigation_heuristics_task_id,
      bool should_skip_screenshot) override;
  void SetIsNotOnInitialEmptyDocument() override;
  bool IsOnInitialEmptyDocument() override;
  void MaybeStartOutermostMainFrameNavigation(
      const std::vector<WebURL>& urls) const override;
  bool WillStartNavigation(const WebNavigationInfo&) override;
  void DidDropNavigation() override;
  void DownloadURL(
      const WebURLRequest& request,
      network::mojom::blink::RedirectMode cross_origin_redirect_behavior,
      CrossVariantMojoRemote<mojom::blink::BlobURLTokenInterfaceBase>
          blob_url_token) override;
  WebFrame* GetProvisionalOwnerFrame() override;

  void SetNotRestoredReasons(
      const mojom::BackForwardCacheNotRestoredReasonsPtr&) override;

  const mojom::blink::BackForwardCacheNotRestoredReasonsPtr&
  GetNotRestoredReasons();

  void SetLCPPHint(
      const mojom::LCPCriticalPathPredictorNavigationTimeHintPtr&) override;

  bool IsFeatureEnabled(
      const network::mojom::PermissionsPolicyFeature&) const override;

  void InitializeCoreFrame(
      Page&,
      FrameOwner*,
      WebFrame* parent,
      WebFrame* previous_sibling,
      FrameInsertType,
      const AtomicString& name,
      WindowAgentFactory*,
      WebFrame* opener,
      const DocumentToken& document_token,
      mojo::PendingRemote<mojom::blink::BrowserInterfaceBroker>
          interface_broker,
      std::unique_ptr<blink::WebPolicyContainer> policy_container,
      const StorageKey& storage_key,
      const KURL& creator_base_url,
      network::mojom::blink::WebSandboxFlags sandbox_flags =
          network::mojom::blink::WebSandboxFlags::kNone);
  LocalFrame* GetFrame() const { return frame_.Get(); }

  void WillBeDetached();
  void WillDetachParent();
  void CollectGarbageForTesting();

  static WebLocalFrameImpl* CreateMainFrame(
      WebView*,
      WebLocalFrameClient*,
      InterfaceRegistry*,
      mojo::PendingRemote<mojom::blink::BrowserInterfaceBroker>,
      const LocalFrameToken& frame_token,
      WebFrame* opener,
      const WebString& name,
      network::mojom::blink::WebSandboxFlags,
      const DocumentToken& document_token,
      std::unique_ptr<WebPolicyContainer>,
      const WebURL& creator_base_url);
  static WebLocalFrameImpl* CreateProvisional(
      WebLocalFrameClient*,
      InterfaceRegistry*,
      mojo::PendingRemote<mojom::blink::BrowserInterfaceBroker>,
      const LocalFrameToken& frame_token,
      WebFrame* previous_frame,
      const FramePolicy&,
      const WebString& name,
      WebView* web_view);

  WebLocalFrameImpl(base::PassKey<WebLocalFrameImpl>,
                    mojom::blink::TreeScopeType,
                    WebLocalFrameClient*,
                    InterfaceRegistry*,
                    const LocalFrameToken& frame_token);
  WebLocalFrameImpl(base::PassKey<WebRemoteFrameImpl>,
                    mojom::blink::TreeScopeType,
                    WebLocalFrameClient*,
                    InterfaceRegistry*,
                    const LocalFrameToken& frame_token);
  ~WebLocalFrameImpl() override;

  LocalFrame* CreateChildFrame(const AtomicString& name,
                               HTMLFrameOwnerElement*);

  RemoteFrame* CreateFencedFrame(
      HTMLFencedFrameElement*,
      mojo::PendingAssociatedReceiver<mojom::blink::FencedFrameOwnerHost>);

  void DidChangeContentsSize(const gfx::Size&);

  bool HasDevToolsOverlays() const;
  void UpdateDevToolsOverlaysPrePaint();
  void PaintDevToolsOverlays(GraphicsContext&);

  void CreateFrameView();

  // Sometimes Blink makes Page/Frame for internal purposes like for SVGImage
  // (see comments in third_party/blink/renderer/core/page/page.h). In that
  // case, such frames are not associated with a WebLocalFrame(Impl).
  // So note that FromFrame may return nullptr even for non-null frames.
  static WebLocalFrameImpl* FromFrame(LocalFrame*);
  static WebLocalFrameImpl* FromFrame(LocalFrame&);
  // TODO(https://crbug.com/1139104): Remove this.
  static std::string GetNullFrameReasonForBug1139104(LocalFrame* frame);

  WebViewImpl* ViewImpl() const;

  LocalFrameView* GetFrameView() const {
    return GetFrame() ? GetFrame()->View() : nullptr;
  }

  void SendOrientationChangeEvent();

  WebDevToolsAgentImpl* DevToolsAgentImpl(bool create_if_necessary);
  void OnDevToolsSessionConnectionChanged(bool attached);

  // Instructs devtools to pause loading of the frame as soon as it's shown
  // until explicit command from the devtools client. May only be called on a
  // local root.
  void WaitForDebuggerWhenShown();

  // When a Find operation ends, we want to set the selection to what was active
  // and set focus to the first focusable node we find (starting with the first
  // node in the matched range and going up the inheritance chain). If we find
  // nothing to focus we focus the first focusable node in the range. This
  // allows us to set focus to a link (when we find text inside a link), which
  // allows us to navigate by pressing Enter after closing the Find box.
  void SetFindEndstateFocusAndSelection();

  void DidCommitLoad();
  void DidDispatchDOMContentLoadedEvent();
  void DidFailLoad(const ResourceError&, WebHistoryCommitType);
  void DidFinish();
  void DidFinishLoadForPrinting();

  void SetClient(WebLocalFrameClient* client) { client_ = client; }

  WebFrameWidgetImpl* FrameWidgetImpl() { return frame_widget_.Get(); }

  WebTextCheckClient* GetTextCheckerClient() const {
    return text_check_client_;
  }

  FindInPage* GetFindInPage() const { return find_in_page_.Get(); }

  TextFinder* GetTextFinder() const;
  // Returns the text finder object if it already exists.
  // Otherwise creates it and then returns.
  TextFinder& EnsureTextFinder();

  // TODO(dcheng): Remove this and make |FrameWidget()| always return something
  // useful.
  WebFrameWidgetImpl* LocalRootFrameWidget();

  // Scroll the focused editable element into the view.
  void ScrollFocusedEditableElementIntoView();
  void ResetHasScrolledFocusedEditableIntoView();

  // Returns true if the frame is focused.
  bool IsFocused() const;

  // Copy the current selection to the pboard.
  void CopyToFindPboard();

  // Center the selection in the viewport.
  void CenterSelection() override;

  // Shows a context menu with commands relevant to a specific element on
  // the given frame. Additional context data and location are supplied.
  void ShowContextMenu(
      mojo::PendingAssociatedRemote<mojom::blink::ContextMenuClient> client,
      const ContextMenuData& data,
      const std::optional<gfx::Point>& host_context_menu_location);

  void Trace(Visitor*) const;

  // Functions to add and remove observers for this object.
  void AddObserver(WebLocalFrameObserver* observer);
  void RemoveObserver(WebLocalFrameObserver* observer);

  void WillSendSubmitEvent(const WebFormElement& form);

 protected:
  // WebLocalFrame protected overrides:
  void AddMessageToConsoleImpl(const WebConsoleMessage&,
                               bool discard_duplicates) override;

  void AddInspectorIssueImpl(mojom::blink::InspectorIssueCode code) override;
  void AddGenericIssueImpl(mojom::blink::GenericIssueErrorType error_type,
                           int violating_node_id) override;
  void AddGenericIssueImpl(mojom::blink::GenericIssueErrorType error_type,
                           int violating_node_id,
                           const WebString& violating_node_attribute) override;

 private:
  friend LocalFrameClientImpl;

  // Sets the local core frame and registers destruction observers.
  void SetCoreFrame(LocalFrame*);

  // Inherited from WebFrame, but intentionally hidden: it never makes sense
  // to call these on a WebLocalFrameImpl.
  bool IsWebLocalFrame() const override;
  WebLocalFrame* ToWebLocalFrame() override;
  const WebLocalFrame* ToWebLocalFrame() const override;
  bool IsWebRemoteFrame() const override;
  WebRemoteFrame* ToWebRemoteFrame() override;
  const WebRemoteFrame* ToWebRemoteFrame() const override;
  void CreateFrameWidgetInternal(
      base::PassKey<WebLocalFrame> pass_key,
      CrossVariantMojoAssociatedRemote<
          mojom::blink::FrameWidgetHostInterfaceBase> mojo_frame_widget_host,
      CrossVariantMojoAssociatedReceiver<mojom::blink::FrameWidgetInterfaceBase>
          mojo_frame_widget,
      CrossVariantMojoAssociatedRemote<mojom::blink::WidgetHostInterfaceBase>
          mojo_widget_host,
      CrossVariantMojoAssociatedReceiver<mojom::blink::WidgetInterfaceBase>
          mojo_widget,
      const viz::FrameSinkId& frame_sink_id,
      bool is_for_nested_main_frame,
      bool is_for_scalable_page,
      bool hidden) override;

  HitTestResult HitTestResultForVisualViewportPos(const gfx::Point&);

  WebPlugin* FocusedPluginIfInputMethodSupported();
  ScrollableArea* LayoutViewport() const;

  // A helper for DispatchBeforePrintEvent() and DispatchAfterPrintEvent().
  void DispatchPrintEventRecursively(const AtomicString& event_type);

  WebPluginContainerImpl* GetPluginToPrintHelper(
      const WebNode& constrain_to_node);

  Node* ContextMenuImageNodeInner() const;
  Node* ContextMenuNodeInner() const;

  void InitializeCoreFrameInternal(
      Page&,
      FrameOwner*,
      WebFrame* parent,
      WebFrame* previous_sibling,
      FrameInsertType,
      const AtomicString& name,
      WindowAgentFactory*,
      WebFrame* opener,
      const DocumentToken& document_token,
      mojo::PendingRemote<mojom::blink::BrowserInterfaceBroker>
          interface_broker,
      std::unique_ptr<PolicyContainer> policy_container,
      const StorageKey& storage_key,
      ukm::SourceId document_ukm_source_id,
      const KURL& creator_base_url,
      network::mojom::blink::WebSandboxFlags sandbox_flags =
          network::mojom::blink::WebSandboxFlags::kNone);

  // This function converts mojom::BackForwardCacheNotRestoredReasonsPtr to
  // mojom::blink::BackForwardCacheNotRestoredReasonsPtr.
  mojom::blink::BackForwardCacheNotRestoredReasonsPtr ConvertNotRestoredReasons(
      const mojom::BackForwardCacheNotRestoredReasonsPtr& reasons_struct);

  // If true, requests compositor warm-up when the page is under prerendering.
  // Please see crbug.com/41496019 for more details.
  bool ShouldWarmUpCompositorOnPrerenderFromThisPoint(
      features::Prerender2WarmUpCompositorTriggerPoint trigger_point);

  WebLocalFrameClient* client_;

  // TODO(dcheng): Inline this field directly rather than going through Member.
  const Member<LocalFrameClientImpl> local_frame_client_;

  // The embedder retains a reference to the WebCore LocalFrame while it is
  // active in the DOM. This reference is released when the frame is removed
  // from the DOM or the entire page is closed.
  Member<LocalFrame> frame_;

  // This is set if the frame is the root of a local frame tree, and requires a
  // widget for layout.
  Member<WebFrameWidgetImpl> frame_widget_;

  Member<WebDevToolsAgentImpl> dev_tools_agent_;

  WebAutofillClient* autofill_client_;

  WebContentCaptureClient* content_capture_client_ = nullptr;

  WebContentSettingsClient* content_settings_client_ = nullptr;

  Member<FindInPage> find_in_page_;

  // Optional weak pointer to the WebPrintClient that initiated printing. Only
  // valid when |is_in_printing_| is true.
  base::WeakPtr<WebPrintClient> print_client_;

  // Valid between calls to BeginPrint() and EndPrint(). Containts the print
  // information. Is used by PrintPage().
  Member<ChromePrintContext> print_context_;

  // Borrowed pointers to Mojo objects.
  InterfaceRegistry* interface_registry_;

  WebInputMethodControllerImpl input_method_controller_;

  WebTextCheckClient* text_check_client_;

  WebSpellCheckPanelHostClient* spell_check_panel_host_client_;

  mojom::BackForwardCacheNotRestoredReasonsPtr not_restored_reasons_;

  // Oilpan: WebLocalFrameImpl must remain alive until close() is called.
  // Accomplish that by keeping a self-referential Persistent<>. It is
  // cleared upon close().
  SelfKeepAlive<WebLocalFrameImpl> self_keep_alive_{this};

#if DCHECK_IS_ON()
  // True if DispatchBeforePrintEvent() was called, and
  // DispatchAfterPrintEvent() is not called yet.
  bool is_in_printing_ = false;
#endif

  // Bookkeeping to suppress redundant scroll and focus requests for an already
  // scrolled and focused editable node.
  bool has_scrolled_focused_editable_node_into_rect_ = false;

  Member<HistoryItem> current_history_item_;

  // All the registered observers.
  base::ObserverList<WebLocalFrameObserver, true> observers_;
};

template <>
struct DowncastTraits<WebLocalFrameImpl> {
  static bool AllowFrom(const WebFrame& frame) {
    return frame.IsWebLocalFrame();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WEB_LOCAL_FRAME_IMPL_H_
