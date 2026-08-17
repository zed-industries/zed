// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LOCAL_FRAME_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LOCAL_FRAME_H_

#include <memory>
#include <optional>
#include <set>

#include "base/containers/span.h"
#include "base/functional/callback.h"
#include "base/i18n/rtl.h"
#include "base/memory/weak_ptr.h"
#include "base/types/pass_key.h"
#include "base/unguessable_token.h"
#include "build/build_config.h"
#include "components/viz/common/surfaces/frame_sink_id.h"
#include "services/network/public/mojom/permissions_policy/permissions_policy_feature.mojom-shared.h"
#include "services/network/public/mojom/web_sandbox_flags.mojom-shared.h"
#include "third_party/blink/public/common/context_menu_data/untrustworthy_context_menu_params.h"
#include "third_party/blink/public/common/frame/frame_ad_evidence.h"
#include "third_party/blink/public/common/frame/user_activation_update_source.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/back_forward_cache_not_restored_reasons.mojom-forward.h"
#include "third_party/blink/public/mojom/blob/blob_url_store.mojom-shared.h"
#include "third_party/blink/public/mojom/browser_interface_broker.mojom-shared.h"
#include "third_party/blink/public/mojom/commit_result/commit_result.mojom-shared.h"
#include "third_party/blink/public/mojom/context_menu/context_menu.mojom-shared.h"
#include "third_party/blink/public/mojom/devtools/devtools_agent.mojom-shared.h"
#include "third_party/blink/public/mojom/devtools/inspector_issue.mojom-shared.h"
#include "third_party/blink/public/mojom/dom_storage/storage_area.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/lifecycle.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/media_player_action.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/user_activation_notification_type.mojom-shared.h"
#include "third_party/blink/public/mojom/lcp_critical_path_predictor/lcp_critical_path_predictor.mojom-forward.h"
#include "third_party/blink/public/mojom/navigation/navigation_params.mojom-shared.h"
#include "third_party/blink/public/mojom/navigation/renderer_content_settings.mojom.h"
#include "third_party/blink/public/mojom/page/widget.mojom-shared.h"
#include "third_party/blink/public/mojom/script/script_evaluation_params.mojom-shared.h"
#include "third_party/blink/public/mojom/selection_menu/selection_menu_behavior.mojom-shared.h"
#include "third_party/blink/public/mojom/use_counter/metrics/web_feature.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/task_type.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_content_settings_client.h"
#include "third_party/blink/public/platform/web_url_error.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/public/web/web_document.h"
#include "third_party/blink/public/web/web_document_loader.h"
#include "third_party/blink/public/web/web_frame.h"
#include "third_party/blink/public/web/web_frame_load_type.h"
#include "third_party/blink/public/web/web_history_item.h"
#include "third_party/blink/public/web/web_script_execution_callback.h"
#include "ui/accessibility/ax_tree_id.h"
#include "ui/base/ime/ime_text_span.h"
#include "ui/gfx/range/range.h"
#include "v8/include/v8-forward.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace cc {
class PaintCanvas;
}  // namespace cc

namespace gfx {
class Point;
class PointF;
}  // namespace gfx

namespace ui {
struct ImeTextSpan;
}  // namespace ui

namespace blink {

namespace scheduler {
class WebAgentGroupScheduler;
}  // namespace scheduler

class BrowserInterfaceBrokerProxy;
class FrameScheduler;
class InterfaceRegistry;
class PageState;
class WebAssociatedURLLoader;
class WebAutofillClient;
class WebContentCaptureClient;
class WebContentSettingsClient;
class WebLocalFrameClient;
class WebFrameWidget;
class WebHistoryItem;
class WebHitTestResult;
class WebInputMethodController;
class WebPerformanceMetricsForReporting;
class WebPerformanceMetricsForNestedContexts;
class WebPlugin;
class WebPrintClient;
class WebRange;
class WebSpellCheckPanelHostClient;
class WebString;
class WebTextCheckClient;
class WebURL;
class WebView;
struct FramePolicy;
struct Impression;
struct WebAssociatedURLLoaderOptions;
struct WebConsoleMessage;
struct WebIsolatedWorldInfo;
struct WebPolicyContainer;
struct WebPrintPageDescription;
struct WebPrintParams;
struct WebPrintPresetOptions;
struct WebScriptSource;

#if BUILDFLAG(IS_WIN)
struct WebFontFamilyNames;
#endif

namespace mojom {
enum class TreeScopeType;
}

// Interface for interacting with in process frames. This contains methods that
// require interacting with a frame's document.
// FIXME: Move lots of methods from WebFrame in here.
class BLINK_EXPORT WebLocalFrame : public WebFrame {
 public:
  // Creates a main local frame for the WebView. Can only be invoked when no
  // main frame exists yet. Call Close() to release the returned frame.
  // WebLocalFrameClient may not be null.
  // TODO(dcheng): The argument order should be more consistent with
  // CreateLocalChild() and CreateRemoteChild() in WebRemoteFrame... but it's so
  // painful...
  static WebLocalFrame* CreateMainFrame(
      WebView*,
      WebLocalFrameClient*,
      blink::InterfaceRegistry*,
      CrossVariantMojoRemote<mojom::BrowserInterfaceBrokerInterfaceBase>,
      const LocalFrameToken& frame_token,
      const DocumentToken& document_token,
      std::unique_ptr<blink::WebPolicyContainer> policy_container,
      WebFrame* opener = nullptr,
      const WebString& name = WebString(),
      network::mojom::WebSandboxFlags = network::mojom::WebSandboxFlags::kNone,
      const WebURL& base_url = WebURL());

  // Used to create a provisional local frame. Currently, it's possible for a
  // provisional navigation not to commit (i.e. it might turn into a download),
  // but this can only be determined by actually trying to load it. The loading
  // process depends on having a corresponding LocalFrame in Blink to hold all
  // the pending state.
  //
  // When a provisional frame is first created, it is only partially attached to
  // the frame tree. This means that though a provisional frame might have a
  // frame owner, the frame owner's content frame does not point back at the
  // provisional frame. Similarly, though a provisional frame may have a parent
  // frame pointer, the parent frame's children list will not contain the
  // provisional frame. Thus, a provisional frame is invisible to the rest of
  // Blink unless the navigation commits and the provisional frame is fully
  // attached to the frame tree by calling Swap(). It swaps with the
  // |previous_web_frame|.
  //
  // |name| should either match the name of the frame that might be replaced, or
  // be an empty string (e.g. if the browsing context name needs to be cleared
  // due to Cross-Origin Opener Policy).
  //
  // Otherwise, if the load should not commit, call Detach() to discard the
  // frame.
  static WebLocalFrame* CreateProvisional(
      WebLocalFrameClient*,
      InterfaceRegistry*,
      CrossVariantMojoRemote<mojom::BrowserInterfaceBrokerInterfaceBase>,
      const LocalFrameToken& frame_token,
      WebFrame* previous_web_frame,
      const FramePolicy&,
      const WebString& name,
      WebView* web_view);

  // Creates a new local child of this frame. Similar to the other methods that
  // create frames, the returned frame should be freed by calling Close() when
  // it's no longer needed.
  virtual WebLocalFrame* CreateLocalChild(
      mojom::TreeScopeType,
      WebLocalFrameClient*,
      InterfaceRegistry*,
      const LocalFrameToken& frame_token) = 0;

  // Returns the WebFrame associated with the current V8 context. This
  // function can return 0 if the context is associated with a Document that
  // is not currently being displayed in a Frame.
  static WebLocalFrame* FrameForCurrentContext();

  // Returns the frame corresponding to the given context. This can return 0
  // if the context is detached from the frame, or if the context doesn't
  // correspond to a frame (e.g., workers).
  static WebLocalFrame* FrameForContext(v8::Local<v8::Context>);

  // Returns the frame associated with the |frame_token|.
  static WebLocalFrame* FromFrameToken(const LocalFrameToken& frame_token);

  virtual WebLocalFrameClient* Client() const = 0;

  // Initialization ---------------------------------------------------------

  virtual void SetAutofillClient(WebAutofillClient*) = 0;
  virtual WebAutofillClient* AutofillClient() = 0;

  virtual void SetContentCaptureClient(WebContentCaptureClient*) = 0;
  virtual WebContentCaptureClient* ContentCaptureClient() const = 0;

  // Basic properties ---------------------------------------------------

  virtual BrowserInterfaceBrokerProxy& GetBrowserInterfaceBroker() = 0;

  LocalFrameToken GetLocalFrameToken() const {
    return GetFrameToken().GetAs<LocalFrameToken>();
  }

  virtual WebDocument GetDocument() const = 0;

  // The name of this frame. If no name is given, empty string is returned.
  virtual WebString AssignedName() const = 0;

  // Sets the name of this frame.
  virtual void SetName(const WebString&) = 0;

  // Returns the AXTreeID associated to the current frame. It is tied to the
  // frame's associated EmbeddingToken, and so it will only be a valid one after
  // the first time that document has been loaded, and will change whenever the
  // loaded document changes (e.g. frame navigated to a different document).
  virtual ui::AXTreeID GetAXTreeID() const = 0;

  // Sets BackForwardCache NotRestoredReasons for the current frame.
  virtual void SetNotRestoredReasons(
      const mojom::BackForwardCacheNotRestoredReasonsPtr&) = 0;

  // Sets LCP Critical Path Detector hint for the current frame that was
  // available at the navigation commit timing.
  virtual void SetLCPPHint(
      const mojom::LCPCriticalPathPredictorNavigationTimeHintPtr&) = 0;

  // Tests whether the policy-controlled feature is enabled in this frame.
  virtual bool IsFeatureEnabled(
      const network::mojom::PermissionsPolicyFeature&) const = 0;

  // Hierarchy ----------------------------------------------------------

  // Returns true if the current frame is a provisional frame.
  // TODO(https://crbug.com/578349): provisional frames are a hack that should
  // be removed.
  virtual bool IsProvisional() const = 0;

  // Get the highest-level LocalFrame in this frame's in-process subtree.
  virtual WebLocalFrame* LocalRoot() = 0;

  // Returns the WebFrameWidget associated with this frame if there is one or
  // nullptr otherwise.
  // TODO(dcheng): The behavior of this will be changing to always return a
  // WebFrameWidget. Use IsLocalRoot() if it's important to tell if a frame is a
  // local root.
  virtual WebFrameWidget* FrameWidget() const = 0;

  // Creates and returns an associated FrameWidget for this frame. The frame
  // must be a LocalRoot. The WebLocalFrame maintins ownership of the
  // WebFrameWidget that was created.
  WebFrameWidget* InitializeFrameWidget(
      CrossVariantMojoAssociatedRemote<mojom::FrameWidgetHostInterfaceBase>
          frame_widget_host,
      CrossVariantMojoAssociatedReceiver<mojom::FrameWidgetInterfaceBase>
          frame_widget,
      CrossVariantMojoAssociatedRemote<mojom::WidgetHostInterfaceBase>
          widget_host,
      CrossVariantMojoAssociatedReceiver<mojom::WidgetInterfaceBase> widget,
      const viz::FrameSinkId& frame_sink_id,
      bool is_for_nested_main_frame = false,
      bool is_for_scalable_page = true,
      bool hidden = false);

  // Returns the frame identified by the given name.  This method supports
  // pseudo-names like _self, _top, and _blank and otherwise performs the same
  // kind of lookup what |window.open(..., name)| would in Javascript.
  virtual WebFrame* FindFrameByName(const WebString& name) = 0;

  // Sets an embedding token for the document in this frame. This token is
  // propagated to the remote parent of this frame (via the browser) such
  // that it can uniquely refer to the document in this frame.
  virtual void SetEmbeddingToken(
      const base::UnguessableToken& embedding_token) = 0;

  // Returns the embedding token for this frame or nullopt if the frame hasn't
  // committed a navigation. This token changes when a new document is committed
  // in this WebLocalFrame.
  virtual const std::optional<base::UnguessableToken>& GetEmbeddingToken()
      const = 0;

  // "Returns true if the frame the document belongs to, or any of its ancestor
  // nodes (within the frame tree) is a fenced frame. See
  // blink::Frame::IsInFencedFrameTree() for more details.
  virtual bool IsInFencedFrameTree() const = 0;

  // Navigation Ping --------------------------------------------------------

  virtual void SendPings(const WebURL& destination_url) = 0;

  virtual void SendAttributionSrc(const std::optional<Impression>&,
                                  bool did_navigate) = 0;

  // Navigation ----------------------------------------------------------

  // Start reloading the current document.
  // Note: StartReload() will be deprecated, use StartNavigation() instead.
  virtual void StartReload(WebFrameLoadType) = 0;

  // View-source rendering mode.  Set this before loading an URL to cause
  // it to be rendered in view-source mode.
  virtual void EnableViewSourceMode(bool) = 0;
  virtual bool IsViewSourceModeEnabled() const = 0;

  // Returns the document loader that is currently loaded.
  virtual WebDocumentLoader* GetDocumentLoader() const = 0;

  // Sets the referrer for the given request to be the specified URL or
  // if that is null, then it sets the referrer to the referrer that the
  // frame would use for subresources.  NOTE: This method also filters
  // out invalid referrers (e.g., it is invalid to send a HTTPS URL as
  // the referrer for a HTTP request).
  virtual void SetReferrerForRequest(WebURLRequest&, const WebURL&) = 0;

  // The frame should handle the request as a download.
  // If the request is for a blob: URL, a BlobURLToken should be provided
  // as |blob_url_token| to ensure the correct blob gets downloaded.
  virtual void DownloadURL(
      const WebURLRequest& request,
      network::mojom::RedirectMode cross_origin_redirect_behavior,
      CrossVariantMojoRemote<mojom::BlobURLTokenInterfaceBase>
          blob_url_token) = 0;

  // If `this` is a provisional frame, returns the "owner" frame, i.e. the frame
  // that would be replaced if a navigation commits in `this`. Otherwise,
  // returns nullptr.
  virtual WebFrame* GetProvisionalOwnerFrame() = 0;

  // Navigation State -------------------------------------------------------

  // Returns true if there is a pending redirect or location change
  // within specified interval. This could be caused by:
  // * an HTTP Refresh header
  // * an X-Frame-Options header
  // * the respective http-equiv meta tags
  // * window.location value being mutated
  // * CSP policy block
  // * reload
  // * form submission
  virtual bool IsNavigationScheduledWithin(base::TimeDelta interval) const = 0;

  virtual void BlinkFeatureUsageReport(blink::mojom::WebFeature feature) = 0;

  // CSS3 Paged Media ----------------------------------------------------

  // Gets the description for the specified page. This includes preferred page
  // size and margins in pixels, assuming 96 pixels per inch. The size and
  // margins must be initialized to the default values that are used if auto is
  // specified.
  //
  // This function must be called after having called PrintBegin() at some
  // point, and before PrintEnd() is called.
  virtual WebPrintPageDescription GetPageDescription(uint32_t page_index) = 0;

  // Scripting --------------------------------------------------------------

  // The following methods execute script within the frame synchronously, even
  // if script execution is suspended (e.g. due to a devtools breakpoint).
  // Prefer the RequestExecute*() methods below for any script that should
  // respect script being suspended.

  // Executes script in the context of the current page.
  virtual void ExecuteScript(const WebScriptSource&) = 0;

  // Executes JavaScript in a new world associated with the web frame.
  // The script gets its own global scope and its own prototypes for
  // intrinsic JavaScript objects (String, Array, and so-on). It also
  // gets its own wrappers for all DOM nodes and DOM constructors.
  //
  // `world_id` must be > kMainDOMWorldId and < kEmbedderWorldIdLimit (a
  // high number used internally).
  virtual void ExecuteScriptInIsolatedWorld(int32_t world_id,
                                            const WebScriptSource&,
                                            BackForwardCacheAware) = 0;

  // `world_id` must be > kMainDOMWorldId and < kEmbedderWorldIdLimit (a
  // high number used internally).
  [[nodiscard]] virtual v8::Local<v8::Value>
  ExecuteScriptInIsolatedWorldAndReturnValue(int32_t world_id,
                                             const WebScriptSource&,
                                             BackForwardCacheAware) = 0;

  // Clears the isolated world CSP stored for |world_id| by this frame's
  // Document.
  virtual void ClearIsolatedWorldCSPForTesting(int32_t world_id) = 0;

  // Executes script in the context of the current page and returns the value
  // that the script evaluated to.
  virtual v8::Local<v8::Value> ExecuteScriptAndReturnValue(
      const WebScriptSource&) = 0;

  // Call the function with the given receiver and arguments
  virtual v8::MaybeLocal<v8::Value> ExecuteMethodAndReturnValue(
      v8::Local<v8::Function>,
      v8::Local<v8::Value>,
      int argc,
      v8::Local<v8::Value> argv[]) = 0;

  // Call the function with the given receiver and arguments, bypassing
  // canExecute().
  virtual v8::MaybeLocal<v8::Value> CallFunctionEvenIfScriptDisabled(
      v8::Local<v8::Function>,
      v8::Local<v8::Value>,
      int argc,
      v8::Local<v8::Value> argv[]) = 0;

  // Returns the V8 context for associated with the main world and this
  // frame. There can be many V8 contexts associated with this frame, one for
  // each isolated world and one for the main world. If you don't know what
  // the "main world" or an "isolated world" is, then you probably shouldn't
  // be calling this API.
  virtual v8::Local<v8::Context> MainWorldScriptContext() const = 0;

  // Returns the world ID associated with |script_context|.
  virtual int32_t GetScriptContextWorldId(
      v8::Local<v8::Context> script_context) const = 0;
  virtual v8::Local<v8::Context> GetScriptContextFromWorldId(
      v8::Isolate* isolate,
      int world_id) const = 0;

  // The following RequestExecute*() functions execute script within the frame,
  // but respect script suspension (e.g. from devtools), allowing the script to
  // (potentially) finish executing asynchronously, at which point the
  // `WebScriptExecutionCallback` will be triggered. These methods are preferred
  // when the injected script should respect suspension (e.g., for script
  // inserted on behalf of a developer).

  // Requests execution of the given function, but allowing for script
  // suspension and asynchronous execution.
  virtual void RequestExecuteV8Function(v8::Local<v8::Context>,
                                        v8::Local<v8::Function>,
                                        v8::Local<v8::Value> receiver,
                                        int argc,
                                        v8::Local<v8::Value> argv[],
                                        WebScriptExecutionCallback) = 0;

  // Executes the script in the main world of the page.
  // Use kMainDOMWorldId to execute in the main world; otherwise,
  // `world_id` must be a positive integer and less than kEmbedderWorldIdLimit.
  virtual void RequestExecuteScript(int32_t world_id,
                                    base::span<const WebScriptSource> sources,
                                    mojom::UserActivationOption,
                                    mojom::EvaluationTiming,
                                    mojom::LoadEventBlockingOption,
                                    WebScriptExecutionCallback,
                                    BackForwardCacheAware,
                                    mojom::WantResultOption,
                                    mojom::PromiseResultOption) = 0;

  // Returns if devtools is connected to the frame.
  virtual bool IsInspectorConnected() = 0;

  // Logs to the console associated with this frame. If |discard_duplicates| is
  // set, the message will only be added if it is unique (i.e. has not been
  // added to the console previously from this page).
  void AddMessageToConsole(const WebConsoleMessage& message,
                           bool discard_duplicates = false) {
    AddMessageToConsoleImpl(message, discard_duplicates);
  }

  void AddInspectorIssue(mojom::InspectorIssueCode code) {
    AddInspectorIssueImpl(code);
  }

  void AddGenericIssue(mojom::GenericIssueErrorType error_type,
                       int violating_node_id,
                       const WebString& violating_node_attribute) {
    AddGenericIssueImpl(error_type, violating_node_id,
                        violating_node_attribute);
  }

  void AddGenericIssue(mojom::GenericIssueErrorType error_type,
                       int violating_node_id) {
    AddGenericIssueImpl(error_type, violating_node_id);
  }

  // Expose modal dialog methods to avoid having to go through JavaScript.
  virtual void Alert(const WebString& message) = 0;
  virtual bool Confirm(const WebString& message) = 0;
  virtual WebString Prompt(const WebString& message,
                           const WebString& default_value) = 0;

  // Generates an intervention report, which will be routed to the Reporting API
  // and any ReportingObservers. It will also emit the intervention message to
  // the console.
  virtual void GenerateInterventionReport(const WebString& message_id,
                                          const WebString& message) = 0;

  // Editing -------------------------------------------------------------
  virtual void UnmarkText() = 0;
  virtual bool HasMarkedText() const = 0;

  virtual WebRange MarkedRange() const = 0;

  // Returns the text range rectangle in the viepwort coordinate space.
  virtual bool FirstRectForCharacterRange(uint32_t location,
                                          uint32_t length,
                                          gfx::Rect&) const = 0;

  // Supports commands like Undo, Redo, Cut, Copy, Paste, SelectAll,
  // Unselect, etc. See EditorCommand.cpp for the full list of supported
  // commands.
  virtual bool ExecuteCommand(const WebString&) = 0;
  virtual bool ExecuteCommand(const WebString&, const WebString& value) = 0;
  virtual bool IsCommandEnabled(const WebString&) const = 0;

  // Returns the text direction at the start and end bounds of the current
  // selection.  If the selection range is empty, it returns false.
  virtual bool SelectionTextDirection(base::i18n::TextDirection& start,
                                      base::i18n::TextDirection& end) const = 0;
  // Returns true if the selection range is nonempty and its anchor is first
  // (i.e its anchor is its start).
  virtual bool IsSelectionAnchorFirst() const = 0;
  // Changes the text direction of the selected input node.
  virtual void SetTextDirectionForTesting(
      base::i18n::TextDirection direction) = 0;

  // Selection -----------------------------------------------------------
  virtual void CenterSelection() = 0;

  virtual bool HasSelection() const = 0;

  virtual WebRange SelectionRange() const = 0;

  virtual WebString SelectionAsText() const = 0;
  virtual WebString SelectionAsMarkup() const = 0;

  virtual void TextSelectionChanged(const WebString& selection_text,
                                    uint32_t offset,
                                    const gfx::Range& range) = 0;

  // DEPRECATED: Use moveRangeSelection.
  virtual void SelectRange(const gfx::Point& base,
                           const gfx::Point& extent) = 0;

  enum HandleVisibilityBehavior {
    // Hide handle(s) in the new selection.
    kHideSelectionHandle,
    // Show handle(s) in the new selection.
    kShowSelectionHandle,
    // Keep the current handle visibility.
    kPreserveHandleVisibility,
  };

  enum SelectionSetFocusBehavior {
    // Set Focus in the new selection.
    kSelectionSetFocus,
    // Not set focus in the new selection.
    kSelectionDoNotSetFocus,
  };

  virtual void SelectRange(const WebRange&,
                           HandleVisibilityBehavior,
                           mojom::SelectionMenuBehavior,
                           SelectionSetFocusBehavior) = 0;

  virtual WebString RangeAsText(const WebRange&) = 0;

  // Move the current selection to the provided viewport point/points. If the
  // current selection is editable, the new selection will be restricted to
  // the root editable element.
  // |TextGranularity| represents character wrapping granularity. If
  // WordGranularity is set, WebFrame extends selection to wrap word.
  virtual void MoveRangeSelection(
      const gfx::Point& base,
      const gfx::Point& extent,
      WebFrame::TextGranularity = kCharacterGranularity) = 0;
  virtual void MoveCaretSelection(const gfx::Point&) = 0;

  virtual bool SetEditableSelectionOffsets(int start, int end) = 0;
  virtual bool AddImeTextSpansToExistingText(
      const std::vector<ui::ImeTextSpan>& ime_text_spans,
      unsigned text_start,
      unsigned text_end) = 0;
  virtual bool ClearImeTextSpansByType(ui::ImeTextSpan::Type type,
                                       unsigned text_start,
                                       unsigned text_end) = 0;
  virtual bool SetCompositionFromExistingText(
      int composition_start,
      int composition_end,
      const std::vector<ui::ImeTextSpan>& ime_text_spans) = 0;
  virtual void ExtendSelectionAndDelete(int before, int after) = 0;
  virtual void ExtendSelectionAndReplace(int before,
                                         int after,
                                         const WebString& replacement_text) = 0;

  // Moves the selection extent point. This function does not allow the
  // selection to collapse. If the new extent is set to the same position as
  // the current base, this function will do nothing.
  virtual void MoveRangeSelectionExtent(const gfx::Point&) = 0;
  // Replaces the selection with the input string.
  virtual void ReplaceSelection(const WebString&) = 0;
  // Deletes text before and after the current cursor position, excluding the
  // selection. The lengths are supplied in UTF-16 Code Unit, not in code points
  // or in glyphs.
  virtual void DeleteSurroundingText(int before, int after) = 0;
  // A variant of deleteSurroundingText(int, int). Major differences are:
  // 1. The lengths are supplied in code points, not in UTF-16 Code Unit or in
  // glyphs.
  // 2. This method does nothing if there are one or more invalid surrogate
  // pairs in the requested range.
  virtual void DeleteSurroundingTextInCodePoints(int before, int after) = 0;

  // Spell-checking support -------------------------------------------------
  virtual void SetTextCheckClient(WebTextCheckClient*) = 0;
  virtual void SetSpellCheckPanelHostClient(WebSpellCheckPanelHostClient*) = 0;
  virtual WebSpellCheckPanelHostClient* SpellCheckPanelHostClient() const = 0;
  virtual void ReplaceMisspelledRange(const WebString&) = 0;
  virtual void RemoveSpellingMarkers() = 0;
  virtual void RemoveSpellingMarkersUnderWords(
      const std::vector<WebString>& words) = 0;

  // Content Settings -------------------------------------------------------

  virtual WebContentSettingsClient* GetContentSettingsClient() const = 0;
  virtual void SetContentSettingsClient(WebContentSettingsClient*) = 0;

  virtual const mojom::RendererContentSettingsPtr& GetContentSettings()
      const = 0;

  // Image reload -----------------------------------------------------------

  // If the provided node is an image that failed to load, reload it.
  virtual void ReloadImage(const WebNode&) = 0;

  // Iframe sandbox ---------------------------------------------------------

  // Returns false if this frame, or any parent frame is sandboxed and does not
  // have the flag "allow-downloads" set.
  virtual bool IsAllowedToDownload() const = 0;

  // Returns true if a frame is a subframe or an embedded main frame and it is
  // cross-origin with respect to the outermost main frame.
  virtual bool IsCrossOriginToOutermostMainFrame() const = 0;

  // Find-in-page -----------------------------------------------------------

  // Searches a frame for a given string. Only used for testing.
  //
  // If a match is found, this function will select it (scrolling down to
  // make it visible if needed) and fill in selectionRect with the
  // location of where the match was found (in window coordinates).
  //
  // If no match is found, this function clears all tickmarks and
  // highlighting.
  //
  // Returns true if the search string was found, false otherwise.
  virtual bool FindForTesting(int identifier,
                              const WebString& search_text,
                              bool match_case,
                              bool forward,
                              bool new_session,
                              bool force,
                              bool wrap_within_frame,
                              bool async) = 0;

  // Set the tickmarks for the frame and a given `target` element in the frame.
  // If `target` is non-null, use its layout box for scrolling. If `target` is
  // null, use the root layout object for the document in the frame.
  // This will override the default tickmarks generated by find results for the
  // given layout object. If this is called with an empty array, the default
  // behavior will be restored.
  virtual void SetTickmarks(const WebElement& target,
                            const std::vector<gfx::Rect>& tickmarks) = 0;

  // Context menu -----------------------------------------------------------

  // Returns the node that the context menu opened over.
  virtual WebNode ContextMenuImageNode() const = 0;
  virtual WebNode ContextMenuNode() const = 0;

  // Copy to the clipboard the image located at a particular point in visual
  // viewport coordinates.
  virtual void CopyImageAtForTesting(const gfx::Point&) = 0;

  // Shows a context menu with the given information from an external context
  // menu request. The given client will be called with the result.
  //
  // The request ID will be returned by this function. This is passed to the
  // client functions for identification.
  //
  // If the client is destroyed, CancelContextMenu() should be called with the
  // request ID returned by this function.
  //
  // Note: if you end up having clients outliving the WebLocalFrame, we should
  // add a CancelContextMenuCallback function that takes a request id.
  virtual void ShowContextMenuFromExternal(
      const UntrustworthyContextMenuParams& params,
      CrossVariantMojoAssociatedRemote<
          blink::mojom::ContextMenuClientInterfaceBase>
          context_menu_client) = 0;

  // Events --------------------------------------------------------------

  // Usage count for chrome.loadtimes deprecation.
  // This will be removed following the deprecation. See: crbug.com/621512
  virtual void UsageCountChromeLoadTimes(const WebString& metric) = 0;

  // Usage count for chrome.csi deprecation.
  // This will be removed following the deprecation. See: crbug.com/113048
  virtual void UsageCountChromeCSI(const WebString& metric) = 0;

  // Whether we've dispatched "pagehide" on the current document in this frame
  // previously, and haven't dispatched the "pageshow" event after the last time
  // we dispatched "pagehide". This means that we've navigated away from the
  // document and it's still hidden (possibly preserved in the back-forward
  // cache, or unloaded).
  virtual bool DispatchedPagehideAndStillHidden() const = 0;

  // Scheduling ---------------------------------------------------------------

  // Returns FrameScheduler
  virtual FrameScheduler* Scheduler() const = 0;

  // Returns AgentGroupScheduler
  virtual scheduler::WebAgentGroupScheduler* GetAgentGroupScheduler() const = 0;

  // Task queues --------------------------------------------------------------

  // Returns frame-specific task runner to run tasks of this type on.
  // They have the same lifetime as the frame.
  virtual scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner(
      TaskType) = 0;

  // Returns the WebInputMethodController associated with this local frame.
  virtual WebInputMethodController* GetInputMethodController() = 0;

  // Loading ------------------------------------------------------------------

  // Returns an AssociatedURLLoader that is associated with this frame.  The
  // loader will, for example, be cancelled when StopLoading is called.
  //
  // FIXME: StopLoading does not yet cancel an associated loader!!
  virtual std::unique_ptr<WebAssociatedURLLoader> CreateAssociatedURLLoader(
      const WebAssociatedURLLoaderOptions&) = 0;

  // This API is deprecated and only required by PepperURLLoaderHost::Close()
  // and PepperPluginInstanceImpl::HandleDocumentLoad() and so it should not be
  // used on a regular basis.
  virtual void DeprecatedStopLoading() = 0;

  // Geometry -----------------------------------------------------------------

  // NOTE: These routines do not force page layout so their results may
  // not be accurate if the page layout is out-of-date.

  // The scroll offset from the top-left corner of the frame in pixels.
  // Note: This is actually corresponds to "scroll position" instead of
  // "scroll offset" in blink renderer. We use the term "scroll offset" here
  // because it is the term used throughout Chrome (except for blink renderer)
  // where there is no concept of scroll origin.
  // See renderer/core/scroll/scroll_area.h for details.
  virtual gfx::PointF GetScrollOffset() const = 0;
  // Returns true if the scroll offset was set successfully.
  virtual bool SetScrollOffset(const gfx::PointF&) = 0;

  // The size of the document in this frame.
  virtual gfx::Size DocumentSize() const = 0;

  // Returns true if the contents (minus scrollbars) has non-zero area.
  virtual bool HasVisibleContent() const = 0;

  // Returns the visible content rect (minus scrollbars), relative to the
  // document.
  virtual gfx::Rect VisibleContentRect() const = 0;

  // Printing ------------------------------------------------------------

  // Dispatch |beforeprint| event, and execute event handlers. They might detach
  // this frame from the owner WebView.
  // This function should be called before pairs of PrintBegin() and PrintEnd().
  // |print_client| is an optional weak pointer to the caller.
  virtual void DispatchBeforePrintEvent(
      base::WeakPtr<WebPrintClient> print_client) = 0;

  // Get the plugin to print, if any. The |constrain_to_node| parameter is the
  // same as the one for PrintBegin() below.
  virtual WebPlugin* GetPluginToPrint(const WebNode& constrain_to_node) = 0;

  // Reformats the WebFrame for printing. WebPrintParams specifies the printable
  // content size, paper size, printable area size, printer DPI and print
  // scaling option. If |constrain_to_node| is specified, then only the given
  // node is printed (for now only plugins are supported), instead of the entire
  // frame.
  // Returns the number of pages that can be printed at the given page size.
  virtual uint32_t PrintBegin(const WebPrintParams& print_params,
                              const WebNode& constrain_to_node) = 0;

  // Called when printing has been requested, but has not yet begun. This
  // gives the document an opportunity to load any new resources needed for
  // printing. It returns whether any resources will need to load.
  virtual bool WillPrintSoon() = 0;

  // Prints one page.
  virtual void PrintPage(uint32_t page_index, cc::PaintCanvas*) = 0;

  // Reformats the WebFrame for screen display.
  virtual void PrintEnd() = 0;

  // Dispatch |afterprint| event, and execute event handlers. They might detach
  // this frame from the owner WebView.
  // This function should be called after pairs of PrintBegin() and PrintEnd().
  virtual void DispatchAfterPrintEvent() = 0;

  // Returns true on success and sets the out parameter to the print preset
  // options for the document.
  virtual bool GetPrintPresetOptionsForPlugin(const WebNode&,
                                              WebPrintPresetOptions*) = 0;

  // Paint Preview ------------------------------------------------------------

  // Captures a full frame paint preview of the WebFrame including subframes. If
  // |include_linked_destinations| is true, the capture will include annotations
  // about linked destinations within the document. If
  // |skip_accelerated_content| is true, the capture will omit GPU accelerated
  // content where applicable. Currently, this setting replaces video frames
  // with a poster or empty space.
  virtual bool CapturePaintPreview(const gfx::Rect& bounds,
                                   cc::PaintCanvas* canvas,
                                   bool include_linked_destinations,
                                   bool skip_accelerated_content) = 0;

  // Performance --------------------------------------------------------

  virtual WebPerformanceMetricsForReporting PerformanceMetricsForReporting()
      const = 0;
  virtual WebPerformanceMetricsForNestedContexts
  PerformanceMetricsForNestedContexts() const = 0;

  // Ad Tagging ---------------------------------------------------------

  // True if the frame is thought (heuristically) to be created for
  // advertising purposes.
  bool IsAdFrame() const override = 0;

  // See blink::LocalFrame::SetAdEvidence()
  virtual void SetAdEvidence(const blink::FrameAdEvidence& ad_evidence) = 0;

  // See blink::LocalFrame::AdEvidence()
  virtual const std::optional<blink::FrameAdEvidence>& AdEvidence() = 0;

  // This is used to check if a script tagged as an ad is currently on the v8
  // stack. This is the same method used to compute the below bit which will
  // persist.
  virtual bool IsAdScriptInStack() const = 0;

  // True iff a script tagged as an ad was on the v8 stack when the frame was
  // created. This is not currently propagated when a frame navigates
  // cross-origin.
  virtual bool IsFrameCreatedByAdScript() = 0;

  // User activation -----------------------------------------------------------

  // See |blink::LocalFrame::NotifyUserActivation()|.
  virtual void NotifyUserActivation(
      mojom::UserActivationNotificationType notification_type) = 0;

  // See |blink::Frame::HasStickyUserActivation()|.
  virtual bool HasStickyUserActivation() = 0;

  // See |blink::Frame::HasTransientUserActivation()|.
  virtual bool HasTransientUserActivation() = 0;

  // See |blink::LocalFrame::ConsumeTransientUserActivation()|.
  virtual bool ConsumeTransientUserActivation(
      UserActivationUpdateSource update_source =
          UserActivationUpdateSource::kRenderer) = 0;

  // See |blink::Frame::LastActivationWasRestricted()|.
  virtual bool LastActivationWasRestricted() const = 0;

  // Fonts --------------------------------------------------------------------

#if BUILDFLAG(IS_WIN)
  // Returns the font family names currently used.
  virtual WebFontFamilyNames GetWebFontFamilyNames() const = 0;
#endif

  // Testing ------------------------------------------------------------------

  // Get the total spool size (the bounding box of all the pages placed after
  // oneanother vertically), when printing for testing.
  virtual gfx::Size SpoolSizeInPixelsForTesting(
      const std::vector<uint32_t>& pages) = 0;
  virtual gfx::Size SpoolSizeInPixelsForTesting(uint32_t page_count) = 0;

  // Prints the given pages of the frame into the canvas, with page boundaries
  // drawn as one pixel wide blue lines. By default, all pages are printed. This
  // method exists to support web tests.
  virtual void PrintPagesForTesting(
      cc::PaintCanvas*,
      const gfx::Size& spool_size_in_pixels,
      const std::vector<uint32_t>* pages = nullptr) = 0;

  // Returns the bounds rect for current selection. If selection is performed
  // on transformed text, the rect will still bound the selection but will
  // not be transformed itself. If no selection is present, the rect will be
  // empty ((0,0), (0,0)).
  virtual gfx::Rect GetSelectionBoundsRectForTesting() const = 0;

  // Returns the position of the frame's origin relative to the viewport (ie the
  // local root).
  virtual gfx::Point GetPositionInViewportForTesting() const = 0;

  virtual void WasHidden() = 0;
  virtual void WasShown() = 0;

  // Grants ability to lookup a named frame via the FindFrame
  // WebLocalFrameClient API. Enhanced binding security checks that check the
  // agent cluster will be enabled for windows that do not have this permission.
  // This should only be used for extensions and the webview tag.
  virtual void SetAllowsCrossBrowsingInstanceFrameLookup() = 0;

  virtual void SetTargetToCurrentHistoryItem(const WebString& target) = 0;
  virtual void UpdateCurrentHistoryItem() = 0;
  virtual PageState CurrentHistoryItemToPageState() = 0;
  virtual WebHistoryItem GetCurrentHistoryItem() const = 0;
  // Reset TextFinder state for the web test runner in between two tests.
  virtual void ClearActiveFindMatchForTesting() = 0;

  // Sets a local storage area which can be used for this frame. This storage
  // area is ignored if a cached storage area already exists for the storage
  // key.
  virtual void SetLocalStorageArea(
      CrossVariantMojoRemote<mojom::StorageAreaInterfaceBase>
          local_storage_area) = 0;

  // Sets a session storage area which can be used for this frame. This storage
  // area is ignored if a cached storage area already exists for the storage
  // key and namespace.
  virtual void SetSessionStorageArea(
      CrossVariantMojoRemote<mojom::StorageAreaInterfaceBase>
          session_storage_area) = 0;

  // Android WebView requires notification of hit tests from blink. It requires
  // hit tests on touchstart. So this method installs a passive event listener
  // on touchstart and does a GestureTap hit test providing the results to the
  // callback.
  virtual void AddHitTestOnTouchStartCallback(
      base::RepeatingCallback<void(const blink::WebHitTestResult&)>
          callback) = 0;

  // Used to block and resume parsing of the current document in the frame.
  virtual void BlockParserForTesting() {}
  virtual void ResumeParserForTesting() {}

  // Processes all pending input in the widget associated with this frame.
  // This is an asynchronous operation since it processes the compositor queue
  // as well. The passed closure is invoked when queues of both threads have
  // been processed.
  virtual void FlushInputForTesting(base::OnceClosure) {}

  virtual bool AllowStorageAccessSyncAndNotify(
      WebContentSettingsClient::StorageType storage_type) = 0;

 protected:
  explicit WebLocalFrame(mojom::TreeScopeType scope,
                         const LocalFrameToken& frame_token)
      : WebFrame(scope, frame_token) {}

  // Inherited from WebFrame, but intentionally hidden: it never makes sense
  // to directly call these on a WebLocalFrame.
  bool IsWebLocalFrame() const override = 0;
  WebLocalFrame* ToWebLocalFrame() override = 0;
  bool IsWebRemoteFrame() const override = 0;
  WebRemoteFrame* ToWebRemoteFrame() override = 0;

  virtual void AddMessageToConsoleImpl(const WebConsoleMessage&,
                                       bool discard_duplicates) = 0;
  virtual void AddInspectorIssueImpl(blink::mojom::InspectorIssueCode code) = 0;
  virtual void AddGenericIssueImpl(
      blink::mojom::GenericIssueErrorType error_type,
      int violating_node_id) = 0;
  virtual void AddGenericIssueImpl(
      blink::mojom::GenericIssueErrorType error_type,
      int violating_node_id,
      const WebString& violating_node_attribute) = 0;
  virtual void CreateFrameWidgetInternal(
      base::PassKey<WebLocalFrame> pass_key,
      CrossVariantMojoAssociatedRemote<mojom::FrameWidgetHostInterfaceBase>
          frame_widget_host,
      CrossVariantMojoAssociatedReceiver<mojom::FrameWidgetInterfaceBase>
          frame_widget,
      CrossVariantMojoAssociatedRemote<mojom::WidgetHostInterfaceBase>
          widget_host,
      CrossVariantMojoAssociatedReceiver<mojom::WidgetInterfaceBase> widget,
      const viz::FrameSinkId& frame_sink_id,
      bool is_for_nested_main_frame,
      bool is_for_scalable_page,
      bool hidden) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LOCAL_FRAME_H_
