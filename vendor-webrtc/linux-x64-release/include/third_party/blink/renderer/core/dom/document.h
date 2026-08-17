/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 *           (C) 2006 Alexey Proskuryakov (ap@webkit.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2012 Apple Inc. All
 * rights reserved.
 * Copyright (C) 2008, 2009 Torch Mobile Inc. All rights reserved.
 * (http://www.torchmobile.com/)
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies)
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_H_

#include <memory>
#include <optional>

#include "base/check_op.h"
#include "base/containers/enum_set.h"
#include "base/dcheck_is_on.h"
#include "base/gtest_prod_util.h"
#include "base/memory/scoped_refptr.h"
#include "base/time/time.h"
#include "base/timer/elapsed_timer.h"
#include "base/uuid.h"
#include "net/base/schemeful_site.h"
#include "services/metrics/public/cpp/ukm_source_id.h"
#include "services/network/public/mojom/referrer_policy.mojom-blink-forward.h"
#include "services/network/public/mojom/restricted_cookie_manager.mojom-blink-forward.h"
#include "services/network/public/mojom/web_sandbox_flags.mojom-blink-forward.h"
#include "third_party/blink/public/common/metrics/document_update_reason.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/css/preferred_color_scheme.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/color_scheme.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/input/focus_type.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/page/page.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/permissions/permission.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/permissions/permission_status.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/permissions_policy/document_policy_feature.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/scroll/scrollbar_mode.mojom-blink-forward.h"
#include "third_party/blink/public/web/web_form_related_change_type.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/accessibility/axid.h"
#include "third_party/blink/renderer/core/animation/animation_clock.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/media_value_change.h"
#include "third_party/blink/renderer/core/dom/container_node.h"
#include "third_party/blink/renderer/core/dom/create_element_flags.h"
#include "third_party/blink/renderer/core/dom/document_encoding_data.h"
#include "third_party/blink/renderer/core/dom/document_lifecycle.h"
#include "third_party/blink/renderer/core/dom/document_part_root.h"
#include "third_party/blink/renderer/core/dom/document_timing.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/dom/events/event_path.h"
#include "third_party/blink/renderer/core/dom/focus_params.h"
#include "third_party/blink/renderer/core/dom/live_node_list_registry.h"
#include "third_party/blink/renderer/core/dom/node_list_invalidation_type.h"
#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/core/dom/text_link_colors.h"
#include "third_party/blink/renderer/core/dom/tree_scope.h"
#include "third_party/blink/renderer/core/dom/user_action_element_set.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/html/forms/listed_element.h"
#include "third_party/blink/renderer/core/html/parser/parser_synchronization_policy.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap_observer_list.h"
#include "third_party/blink/renderer/platform/instrumentation/use_counter.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value_forward.h"

#if BUILDFLAG(IS_ANDROID)
#include "third_party/blink/public/mojom/facilitated_payments/payment_link_handler.mojom-blink.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#endif

namespace base {
class SingleThreadTaskRunner;
}

namespace cc {
class AnimationTimeline;
}

namespace gfx {
class QuadF;
class RectF;
}  // namespace gfx

namespace mojo {
template <typename Interface>
class PendingRemote;
}  // namespace mojo

namespace ukm {
class UkmRecorder;
}  // namespace ukm

namespace net {
class SiteForCookies;
}  // namespace net

namespace network {
namespace mojom {
enum class CSPDisposition : int32_t;
}  // namespace mojom
}  // namespace network

namespace ui {
class ColorProvider;
}  // namespace ui

namespace blink {

class AXContext;
class AXObjectCache;
class Agent;
class AnchorElementInteractionTracker;
class AnimationClock;
class AriaNotificationOptions;
class Attr;
class BeforeUnloadEventListener;
class CaretPosition;
class CaretPositionFromPointOptions;
class CDATASection;
class CSSStyleSheet;
class CanvasFontCache;
class CharacterData;
class CheckPseudoHasCacheScope;
class ChromeClient;
class Comment;
class ConsoleMessage;
class CookieJar;
class DOMFeaturePolicy;
class DOMImplementation;
class DOMWindow;
class DOMWrapperWorld;
class DisplayLockDocumentState;
class DocumentAnimations;
class DocumentData;
class DocumentFragment;
class DocumentInit;
class DocumentLoader;
class DocumentMarkerController;
class DocumentNameCollection;
class DocumentParser;
class DocumentResourceCoordinator;
class DocumentState;
class DocumentTimeline;
class DocumentType;
class Element;
class ElementDataCache;
class ElementIntersectionObserverData;
class ElementRegistrationOptions;
class Event;
class EventFactoryBase;
class EventListener;
class ExceptionState;
class FocusedElementChangeObserver;
class FontFaceSet;
class FontMatchingMetrics;
class FormController;
class FragmentDirective;
class FrameCallback;
class FrameScheduler;
class HTMLAllCollection;
class HTMLBodyElement;
class HTMLCollection;
class HTMLDialogElement;
class HTMLElement;
class HTMLFrameOwnerElement;
class HTMLHeadElement;
class HTMLImageElement;
class HTMLLinkElement;
class HTMLMetaElement;
class HitTestRequest;
class HttpRefreshScheduler;
class IntersectionObserverController;
class ImportNodeOptions;
class LayoutUpgrade;
class LayoutView;
class LazyLoadImageObserver;
class ListedElement;
class LiveNodeListBase;
class LocalDOMWindow;
class LocalFrame;
class LocalFrameView;
class LocalSVGResource;
class Locale;
class Location;
class MediaQueryListListener;
class MediaQueryMatcher;
class NodeIterator;
class NthIndexCache;
class Page;
class PaintLayerScrollableArea;
class PendingAnimations;
class PendingLinkPreload;
class ProcessingInstruction;
class PropertyRegistry;
class QualifiedName;
class Range;
class RenderBlockingResourceManager;
class ResizeObserver;
class Resource;
class ResourceFetcher;
class RootScrollerController;
class SVGDocumentExtensions;
class SVGUseElement;
class ScriptElementBase;
class ScriptRegexp;
class ScriptRunner;
class ScriptRunnerDelayer;
class ScriptValue;
class ScriptableDocumentParser;
class ScriptedAnimationController;
class ScrollMarkerGroupData;
class SecurityOrigin;
class SelectorQueryCache;
class SerializedScriptValue;
class SetHTMLOptions;
class SetHTMLUnsafeOptions;
class Settings;
class SlotAssignmentEngine;
class StyleEngine;
class StylePropertyMapReadOnly;
class StyleResolver;
class Text;
class TextAutosizer;
class TransformSource;
class TreeWalker;
class TrustedHTML;
class V8DocumentReadyState;
class V8NodeFilter;
class V8UnionStringOrTrustedHTML;
class ViewportData;
class VisitedLinkState;
class WebMouseEvent;
class WorkletAnimationController;
class V8VisibilityState;

template <typename EventType>
class EventWithHitTestResults;

enum class CSSPropertyID;

struct DraggableRegionValue;
struct FocusParams;
struct IconURL;
struct TextDiffRange;
struct WebPrintPageDescription;

using MouseEventWithHitTestResults = EventWithHitTestResults<WebMouseEvent>;

// Specifies a class of document. Values are not mutually exclusive, and can be
// combined using `DocumentClassFlags`.
//
// Remember to keep `kMinValue` and `kMaxValue` up to date.
enum class DocumentClass {
  kHTML,
  kXHTML,
  kImage,
  kPlugin,
  kMedia,
  kSVG,
  kXML,
  kText,

  // For `DocumentClassFlags`.
  kMinValue = kHTML,
  kMaxValue = kText,
};

using DocumentClassFlags = base::
    EnumSet<DocumentClass, DocumentClass::kMinValue, DocumentClass::kMaxValue>;

// A map of IDL attribute name to Element FrozenArray value, for one particular
// element.
// This represents 'cached attr-associated elements' in the HTML specification.
// https://html.spec.whatwg.org/multipage/common-dom-interfaces.html#cached-attr-associated-elements
using CachedAttrAssociatedElementsMap =
    GCedHeapHashMap<QualifiedName, Member<FrozenArray<Element>>>;

// Represents the start and end time of the unload event.
struct UnloadEventTiming {
  bool can_request;
  base::TimeTicks unload_event_start;
  base::TimeTicks unload_event_end;
};

// Used to gather the unload event timing of an unloading document, to be used
// in a new document (if it's same-origin).
struct UnloadEventTimingInfo {
  explicit UnloadEventTimingInfo(
      scoped_refptr<SecurityOrigin> new_document_origin);
  // The origin of the new document that replaces the older document.
  const scoped_refptr<SecurityOrigin> new_document_origin;
  // The unload timing of the old document. This is only set from
  // Document::DispatchUnloadEvents() of the old document. This might not be set
  // if no old document gets unloaded.
  std::optional<UnloadEventTiming> unload_timing;
};

// A document (https://dom.spec.whatwg.org/#concept-document) is the root node
// of a tree of DOM nodes, generally resulting from the parsing of a markup
// (typically, HTML) resource.
//
// A document may or may not have a browsing context
// (https://html.spec.whatwg.org/#browsing-context). A document with a browsing
// context is created by navigation, and has a non-null domWindow(), GetFrame(),
// Loader(), etc., and is visible to the user. It will have a valid
// GetExecutionContext(), which will be equal to domWindow(). If the Document
// constructor receives a DocumentInit created WithDocumentLoader(), it will
// have a browsing context.
// Documents created by all other APIs do not have a browsing context. These
// Documents still have a valid GetExecutionContext() (i.e., the domWindow() of
// the Document in which they were created), so they can still access
// script, but return null for domWindow(), GetFrame() and Loader(). Generally,
// they should not downcast the ExecutionContext to a LocalDOMWindow and access
// the properties of the window directly.
// Finally, unit tests are allowed to create a Document that does not even
// have a valid GetExecutionContext(). This is a lightweight way to test
// properties of the Document and the DOM that do not require script.
class CORE_EXPORT Document : public ContainerNode,
                             public TreeScope,
                             public UseCounter,
                             public Supplementable<Document> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Factory for web-exposed Document constructor. The argument document must be
  // a document instance representing window.document, and it works as the
  // source of ExecutionContext and security origin of the new document.
  // https://dom.spec.whatwg.org/#dom-document-document
  static Document* Create(Document&);

  explicit Document(const DocumentInit& init,
                    DocumentClassFlags flags = DocumentClassFlags());
  ~Document() override;

  // Constructs a Document instance without a subclass for testing.
  static Document* CreateForTest(ExecutionContext& execution_context);

  static Range* CreateRangeAdjustedToTreeScope(const TreeScope&,
                                               const Position&);
  static CaretPosition* CreateCaretPosition(const Position& position);

  static const Position PositionAdjustedToTreeScope(const TreeScope&,
                                                    const Position&);

  // Support JS introspection of frame policy (e.g. permissions policy).
  DOMFeaturePolicy* featurePolicy();

  MediaQueryMatcher& GetMediaQueryMatcher();

  void MediaQueryAffectingValueChanged(MediaValueChange change);

  // SetMediaFeatureEvaluated and WasMediaFeatureEvaluated are used to prevent
  // UKM sampling of CSS media features more than once per document.
  void SetMediaFeatureEvaluated(int feature);
  bool WasMediaFeatureEvaluated(int feature);

  using TreeScope::getElementById;

  bool IsInitialEmptyDocument() const { return is_initial_empty_document_; }
  // Sometimes we permit an initial empty document to cease to be the initial
  // empty document. This is needed for cross-process navigations, where a new
  // LocalFrame needs to be created but the conceptual frame might have had
  // other Documents in a different process. document.open() also causes the
  // document to cease to be the initial empty document.
  void OverrideIsInitialEmptyDocument() { is_initial_empty_document_ = false; }

  bool IsPrerendering() const { return is_prerendering_; }

  bool HasDocumentPictureInPictureWindow() const;

  void SetIsTrackingSoftNavigationHeuristics(bool value) {
    is_tracking_soft_navigation_heuristics_ = value;
  }

  bool IsTrackingSoftNavigationHeuristics() const {
    return is_tracking_soft_navigation_heuristics_;
  }

  network::mojom::ReferrerPolicy GetReferrerPolicy() const;

  bool DocumentPolicyFeatureObserved(
      mojom::blink::DocumentPolicyFeature feature);

  bool CanContainRangeEndPoint() const override { return true; }

  SelectorQueryCache& GetSelectorQueryCache();

  void SetStatePreservingAtomicMoveInProgress(bool value) {
    state_preserving_atomic_move_in_progress_ = value;
  }
  bool StatePreservingAtomicMoveInProgress() const {
    return state_preserving_atomic_move_in_progress_;
  }

  // Focus Management.
  Element* ActiveElement() const;
  bool hasFocus() const;

  // DOM methods & attributes for Document

  DEFINE_ATTRIBUTE_EVENT_LISTENER(beforecopy, kBeforecopy)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(beforecut, kBeforecut)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(beforepaste, kBeforepaste)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(freeze, kFreeze)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(pointerlockchange, kPointerlockchange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(pointerlockerror, kPointerlockerror)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(readystatechange, kReadystatechange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(resume, kResume)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(search, kSearch)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(securitypolicyviolation,
                                  kSecuritypolicyviolation)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(visibilitychange, kVisibilitychange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(prerenderingchange, kPrerenderingchange)

  ViewportData& GetViewportData() const { return *viewport_data_; }

  void SetDoctype(DocumentType*);
  DocumentType* doctype() const { return doc_type_.Get(); }

  DOMImplementation& implementation();

  // Typically, but not guaranteed, to be non-null.
  //
  // ```js
  // document.documentElement.remove();
  // // document.documentElement is now null
  // ```
  Element* documentElement() const { return document_element_.Get(); }

  Location* location() const;

  DocumentFragment* createDocumentFragment();
  Text* createTextNode(const String& data);
  Comment* createComment(const String& data);
  CDATASection* createCDATASection(const String& data, ExceptionState&);
  ProcessingInstruction* createProcessingInstruction(const String& target,
                                                     const String& data,
                                                     ExceptionState&);
  Attr* createAttribute(const AtomicString& name, ExceptionState&);
  Attr* createAttributeNS(const AtomicString& namespace_uri,
                          const AtomicString& qualified_name,
                          ExceptionState&);

  Node* importNode(Node* imported_node,
                   ImportNodeOptions* options,
                   ExceptionState&);
  Node* importNode(Node* imported_node, bool deep, ExceptionState&);

  Node* importNode(Node* imported_node,
                   bool deep,
                   CustomElementRegistry*,
                   ExceptionState&);

  CustomElementRegistry* customElementRegistry() const override;

  // Creates an element without custom element processing.
  Element* CreateRawElement(const QualifiedName&,
                            const CreateElementFlags = CreateElementFlags());

  Range* caretRangeFromPoint(int x, int y);

  // Returns a |CaretPosition| from given point. If the point is inside a shadow
  // tree, then |CaretPosition| only points inside the shadow tree if it's
  // provided in the |shadowRoots| vector in |options| argument.
  // https://drafts.csswg.org/cssom-view/#ref-for-dom-document-caretpositionfrompoint
  CaretPosition* caretPositionFromPoint(
      float x,
      float y,
      const CaretPositionFromPointOptions* options);
  Element* scrollingElement();

  // When calling from C++ code, use this method. scrollingElement() is
  // just for the web IDL implementation.
  //
  // Style/layout-tree needs to be updated before calling this function,
  // otherwise the returned element might be outdated. However, accessing
  // information based on the layout of the previous frame is occasionally
  // the correct behavior [1], hence it's not invalid to call this function
  // while style/layout dirty.
  //
  // [1] https://drafts.csswg.org/scroll-animations-1/#avoiding-cycles
  Element* ScrollingElementNoLayout();

  bool KeyboardFocusableScrollersEnabled();
  bool StandardizedBrowserZoomEnabled() const;

  V8DocumentReadyState readyState() const;

  AtomicString characterSet() const { return Document::EncodingName(); }

  AtomicString EncodingName() const;

  void SetContent(const String&);

  // DOMParser::parseFromString() calls to this. Does the same thing as
  // `setContent()`, but may use the fast path parser.
  void SetContentFromDOMParser(const String&);

  String SuggestedMIMEType() const;
  void SetMimeType(const AtomicString&);
  AtomicString contentType() const;  // DOM 4 document.contentType

  const AtomicString& ContentLanguage() const { return content_language_; }
  void SetContentLanguage(const AtomicString&);

  String xmlEncoding() const { return xml_encoding_; }
  String xmlVersion() const { return xml_version_; }
  enum StandaloneStatus { kStandaloneUnspecified, kStandalone, kNotStandalone };
  bool xmlStandalone() const { return xml_standalone_ == kStandalone; }
  StandaloneStatus XmlStandaloneStatus() const {
    return static_cast<StandaloneStatus>(xml_standalone_);
  }
  bool HasXMLDeclaration() const { return has_xml_declaration_; }

  void SetXMLEncoding(const String& encoding) {
    xml_encoding_ = encoding;
  }  // read-only property, only to be set from XMLDocumentParser
  void setXMLVersion(const String&, ExceptionState&);
  void setXMLStandalone(bool, ExceptionState&);
  void SetHasXMLDeclaration(bool has_xml_declaration) {
    has_xml_declaration_ = has_xml_declaration ? 1 : 0;
  }

  V8VisibilityState visibilityState() const;
  String visibilityStateAsString() const;
  bool IsPageVisible() const;
  bool hidden() const;
  void DidChangeVisibilityState();

  bool prerendering() const;

  uint32_t softNavigations() const;

  bool wasDiscarded() const;
  void SetWasDiscarded(bool);

  // If the document is "prefetch only", it will not be fully contstructed,
  // and should never be displayed. Only a few resources will be loaded and
  // scanned, in order to warm up caches.
  bool IsPrefetchOnly() const;

  Node* adoptNode(Node* source, ExceptionState&);

  HTMLCollection* images();
  HTMLCollection* embeds();
  HTMLCollection* applets();
  HTMLCollection* links();
  HTMLCollection* forms();
  HTMLCollection* anchors();
  HTMLCollection* scripts();
  HTMLAllCollection* all();

  HTMLCollection* WindowNamedItems(const AtomicString& name);
  DocumentNameCollection* DocumentNamedItems(const AtomicString& name);
  HTMLCollection* DocumentAllNamedItems(const AtomicString& name);

  // The unassociated listed elements are listed elements that are not
  // associated to a <form> element. This includes elements inside Shadow DOM.
  const ListedElement::List& UnassociatedListedElements() const;
  void MarkUnassociatedListedElementsDirty();

  // Returns all `HTMLFormElement`s that have no shadow-including
  // `HTMLFormElement` ancestor. Note that the form elements are returned in BFS
  // order.
  const HeapVector<Member<HTMLFormElement>>& GetTopLevelForms();
  // Invalidates the cache for top level form elements.
  void MarkTopLevelFormsDirty();

  // "defaultView" attribute defined in HTML spec.
  DOMWindow* defaultView() const;

  bool IsHTMLDocument() const {
    return document_classes_.Has(DocumentClass::kHTML);
  }
  bool IsXHTMLDocument() const {
    return document_classes_.Has(DocumentClass::kXHTML);
  }
  bool IsXMLDocument() const {
    return document_classes_.Has(DocumentClass::kXML);
  }
  bool IsImageDocument() const {
    return document_classes_.Has(DocumentClass::kImage);
  }
  bool IsSVGDocument() const {
    return document_classes_.Has(DocumentClass::kSVG);
  }
  bool IsPluginDocument() const {
    return document_classes_.Has(DocumentClass::kPlugin);
  }
  bool IsMediaDocument() const {
    return document_classes_.Has(DocumentClass::kMedia);
  }
  bool IsTextDocument() const {
    return document_classes_.Has(DocumentClass::kText);
  }

  bool HasSVGRootNode() const;

  bool IsFrameSet() const;

  bool IsSrcdocDocument() const { return is_srcdoc_document_; }
  bool IsMobileDocument() const { return is_mobile_document_; }

  StyleResolver& GetStyleResolver() const;

  bool IsViewSource() const { return is_view_source_; }
  void SetIsViewSource(bool is_view_source) {
    is_view_source_ = is_view_source;
  }

  virtual bool IsJSONDocument() const { return false; }

  // WebXR DOM Overlay support, cf https://immersive-web.github.io/dom-overlays/
  // True if there's an ongoing "immersive-ar" WebXR session with a DOM Overlay
  // element active. This is needed for applying the :xr-overlay pseudoclass
  // and compositing/paint integration for this mode.
  bool IsXrOverlay() const { return is_xr_overlay_; }
  // Called from modules/xr's XRSystem when DOM Overlay mode starts and ends.
  // This lazy-loads the UA stylesheet and updates the overlay element's
  // pseudoclass.
  void SetIsXrOverlay(bool enabled, Element* overlay_element);

  bool SawElementsInKnownNamespaces() const {
    return saw_elements_in_known_namespaces_;
  }

  bool IsScriptExecutionReady() const {
    return HaveScriptBlockingStylesheetsLoaded();
  }

  bool IsForExternalHandler() const { return is_for_external_handler_; }

  StyleEngine& GetStyleEngine() const {
    DCHECK(style_engine_.Get());
    return *style_engine_.Get();
  }

  mojom::blink::PreferredColorScheme GetPreferredColorScheme() const;

  void ScheduleUseShadowTreeUpdate(SVGUseElement&);
  void UnscheduleUseShadowTreeUpdate(SVGUseElement&);

  void ScheduleSVGResourceInvalidation(LocalSVGResource&);
  void InvalidatePendingSVGResources();

  void EvaluateMediaQueryList();

  FormController& GetFormController();
  DocumentState* GetDocumentState() const;
  void SetStateForNewControls(const Vector<String>&);

  LocalFrameView* View() const;   // can be null
  LocalFrame* GetFrame() const;   // can be null
  Page* GetPage() const;          // can be null
  Settings* GetSettings() const;  // can be null

  float DevicePixelRatio() const;

  Range* createRange();

  NodeIterator* createNodeIterator(Node* root,
                                   unsigned what_to_show,
                                   V8NodeFilter*);
  TreeWalker* createTreeWalker(Node* root,
                               unsigned what_to_show,
                               V8NodeFilter*);

  // Special support for editing
  Text* CreateEditingTextNode(const String&);

  enum class StyleAndLayoutTreeUpdate {
    // Style/layout-tree is not dirty.
    kNone,

    // Style/layout-tree is dirty, and it's possible to understand whether a
    // given element will be affected or not by analyzing its ancestor chain.
    kAnalyzed,

    // Style/layout-tree is dirty, but we cannot decide which specific elements
    // need to have its style or layout tree updated.
    kFull,
  };

  // Looks at various sources that cause style/layout-tree dirtiness,
  // and returns the severity of the needed update.
  //
  // Note that this does not cover "implicit" style/layout-tree dirtiness
  // via layout/container-queries. That is: this function may return kNone,
  // and yet a subsequent layout may need to recalc container-query-dependent
  // styles.
  StyleAndLayoutTreeUpdate CalculateStyleAndLayoutTreeUpdate() const;

  bool NeedsLayoutTreeUpdate() const {
    return CalculateStyleAndLayoutTreeUpdate() !=
           StyleAndLayoutTreeUpdate::kNone;
  }

  // Whether we need layout tree update for this node or not, without
  // considering nodes in display locked subtrees.
  bool NeedsLayoutTreeUpdateForNode(const Node&) const;
  // Whether we need layout tree update for this node or not, including nodes in
  // display locked subtrees.
  bool NeedsLayoutTreeUpdateForNodeIncludingDisplayLocked(const Node&) const;

  // Update ComputedStyles and attach LayoutObjects if necessary. This
  // recursively invokes itself for all ancestor LocalFrames, because style in
  // an ancestor frame can affect style in a child frame. This method is
  // appropriate for cases where we need to ensure that the style for a single
  // Document is up-to-date.
  //
  // A call to UpdateStyleAndLayoutTree may be upgraded [1] to also perform
  // layout. This is because updating the style and layout-tree may depend
  // on layout when container queries are used.
  //
  // Whether or not an upgrade should take place is decide by the
  // provided LayoutUpgrade object.
  //
  // [1] See blink::LayoutUpgrade
  void UpdateStyleAndLayoutTree();
  void UpdateStyleAndLayoutTree(LayoutUpgrade&);

  // Same as UpdateStyleAndLayoutTree, but does not recursively update style in
  // ancestor frames. This method is intended to be used in cases where we can
  // guarantee that ancestor frames already have clean style (e.g., from
  // LocalFrameView::UpdateLifecyclePhases, which is a top-down iteration over
  // the entire LocalFrame tree; or from Document::UpdateStyleAndLayout, which
  // does its own ancestor tree walk).
  void UpdateStyleAndLayoutTreeForThisDocument();

  // `only_cv_auto` is passed to the constructor of
  // DisplayLockUtilities::ScopedForcedUpdate. When set to true, this element
  // won't get a style/layout update if it is inside a content-visibility:hidden
  // subtree.
  void UpdateStyleAndLayoutTreeForElement(const Element*,
                                          DocumentUpdateReason,
                                          bool only_cv_auto = false);
  void UpdateStyleAndLayoutTreeForSubtree(const Element*, DocumentUpdateReason);

  void UpdateStyleAndLayout(DocumentUpdateReason);
  void LayoutUpdated();
  enum RunPostLayoutTasks {
    kRunPostLayoutTasksAsynchronously,
    kRunPostLayoutTasksSynchronously,
  };
  void UpdateStyleAndLayoutForNode(const Node*, DocumentUpdateReason);
  void UpdateStyleAndLayoutForRange(const Range*, DocumentUpdateReason);

  // Ensures that location-based data will be valid for a given node.
  //
  // This will run style and layout if they are currently dirty, and it may also
  // run compositing inputs if the node is in a sticky subtree (as the sticky
  // offset may change the node's position).
  //
  // Due to this you should only call this if you definitely need valid location
  // data, otherwise use one of the |UpdateStyleAndLayout...| methods above.
  void EnsurePaintLocationDataValidForNode(const Node*,
                                           DocumentUpdateReason reason);

  // Gets the description for the specified page. This includes preferred page
  // size and margins in pixels, assuming 96 pixels per inch. Updates layout as
  // needed to get the description.
  WebPrintPageDescription GetPageDescription(uint32_t page_index);

  ResourceFetcher* Fetcher() const { return fetcher_.Get(); }

  void Initialize();
  virtual void Shutdown();

  // If you have a Document, use GetLayoutView() instead which is faster.
  void GetLayoutObject() const = delete;

  LayoutView* GetLayoutView() const { return layout_view_.Get(); }

  // This will return an AXObjectCache only if there's one or more
  // AXContext associated with this document. When all associated
  // AXContexts are deleted, the AXObjectCache will be removed.
  AXObjectCache* ExistingAXObjectCache() const;
  Document& AXObjectCacheOwner() const;
  // If there is an accessibility tree, recompute it and re-serialize it all.
  // This method is useful when something that potentially affects most of the
  // page occurs, such as an inertness change or a fullscreen toggle.
  void RefreshAccessibilityTree() const;

  // to get visually ordered hebrew and arabic pages right
  bool VisuallyOrdered() const { return visually_ordered_; }

  DocumentLoader* Loader() const;

  // This is the DOM API document.open().
  void open(LocalDOMWindow* entered_window, ExceptionState&);
  // This is used internally and does not handle exceptions.
  void open();
  DocumentParser* OpenForNavigation(ParserSynchronizationPolicy,
                                    const AtomicString& mime_type,
                                    const AtomicString& encoding);
  DocumentParser* ImplicitOpen(ParserSynchronizationPolicy);

  // This is the DOM API document.open() implementation.
  // document.open() opens a new window when called with three arguments.
  Document* open(v8::Isolate*,
                 const AtomicString& type,
                 const AtomicString& replace,
                 ExceptionState&);
  DOMWindow* open(v8::Isolate*,
                  const String& url_string,
                  const AtomicString& name,
                  const AtomicString& features,
                  ExceptionState&);
  // This is the DOM API document.close().
  void close(ExceptionState&);
  // This is used internally and does not handle exceptions.
  void close();

  // Corresponds to "9. Abort the active document of browsingContext."
  // https://html.spec.whatwg.org/C/#navigate
  void Abort();

  void CheckCompleted();

  // Dispatches beforeunload into this document. Returns true if the
  // beforeunload handler indicates that it is safe to proceed with an unload,
  // false otherwise.
  //
  // |chrome_client| is used to synchronously get user consent (via a modal
  // javascript dialog) to allow the unload to proceed if the beforeunload
  // handler returns a non-null value, indicating unsaved state. If a
  // null |chrome_client| is provided and the beforeunload returns a non-null
  // value this function will automatically return false, indicating that the
  // unload should not proceed. A null chrome client is set to by the freezing
  // logic, which uses this to determine if a non-empty beforeunload handler
  // is present before allowing discarding to proceed.
  //
  // |is_reload| indicates if the beforeunload is being triggered because of a
  // reload operation, otherwise it is assumed to be a page close or navigation.
  //
  // |did_allow_navigation| is set to reflect the choice made by the user via
  // the modal dialog. The value is meaningless if |auto_cancel|
  // is true, in which case it will always be set to false.
  bool DispatchBeforeUnloadEvent(ChromeClient* chrome_client,
                                 bool is_reload,
                                 bool& did_allow_navigation);

  // Dispatches "pagehide", "visibilitychange" and "unload" events, if not
  // dispatched already. Fills `unload_timing_info` if present.
  void DispatchUnloadEvents(UnloadEventTimingInfo* unload_timing_info);

  void DispatchFreezeEvent();

  enum PageDismissalType {
    kNoDismissal,
    kBeforeUnloadDismissal,
    kPageHideDismissal,
    kUnloadVisibilityChangeDismissal,
    kUnloadDismissal
  };
  PageDismissalType PageDismissalEventBeingDispatched() const;

  void CancelParsing();

  void write(const String& text,
             LocalDOMWindow* entered_window = nullptr,
             ExceptionState& = ASSERT_NO_EXCEPTION);
  void writeln(const String& text,
               LocalDOMWindow* entered_window = nullptr,
               ExceptionState& = ASSERT_NO_EXCEPTION);
  void write(v8::Isolate*, const Vector<String>& text, ExceptionState&);
  void writeln(v8::Isolate*, const Vector<String>& text, ExceptionState&);

  // TrustedHTML variants of the above.
  void write(v8::Isolate*, TrustedHTML*, ExceptionState&);
  void writeln(v8::Isolate*, TrustedHTML*, ExceptionState&);
  void write(v8::Isolate*,
             TrustedHTML*,
             HeapVector<Member<V8UnionStringOrTrustedHTML>>,
             ExceptionState&);
  void writeln(v8::Isolate*,
               TrustedHTML*,
               HeapVector<Member<V8UnionStringOrTrustedHTML>>,
               ExceptionState&);

  // Corresponds to https://html.spec.whatwg.org/#document-write-steps
  //
  // This implements steps 1-5 of the algorithm, and calls
  // write(const String&, LocalDOMWindow*, ExceptionState&) for the remainder.
  void Write(v8::Isolate*,
             TrustedHTML*,
             HeapVector<Member<V8UnionStringOrTrustedHTML>>,
             bool line_feed,
             const char* sink,
             ExceptionState&);

  bool WellFormed() const { return well_formed_; }

  const DocumentToken& Token() const {
    if (!token_.has_value()) {
      token_.emplace();
    }
    return token_.value();
  }

  // Return the document URL, or an empty URL if it's unavailable.
  // This is not an implementation of web-exposed Document.prototype.URL.
  const KURL& Url() const { return url_; }
  void SetURL(const KURL&);

  // Bind the url to document.url, if unavailable bind to about:blank.
  KURL urlForBinding() const;

  // To understand how these concepts relate to one another, please see the
  // comments surrounding their declaration.

  // Document base URL.
  // https://html.spec.whatwg.org/C/#document-base-url
  const KURL& BaseURL() const;
  void SetBaseURLOverride(const KURL&);
  const KURL& BaseURLOverride() const { return base_url_override_; }
  KURL ValidBaseElementURL() const;
  const AtomicString& BaseTarget() const { return base_target_; }
  void ProcessBaseElement();

  // Fallback base URL.
  // https://html.spec.whatwg.org/C/#fallback-base-url
  KURL FallbackBaseURL() const;

  // If we call CompleteURL* during preload, it's possible that we may not
  // have processed any <base> element the document might have
  // (https://crbug.com/331806513), and so we should avoid triggering use counts
  // for resolving relative urls into absolute urls in that case. The following
  // enum allows us to detect calls originating from PreloadRequest.
  // TODO(https://crbug.com/330744612): Remove `CompleteURLPreloadStatus` and
  // related code once the associated issue is ready to be closed.
  enum CompleteURLPreloadStatus { kIsNotPreload, kIsPreload };
  // Creates URL based on passed relative url and this documents base URL.
  // Depending on base URL value it is possible that parent document
  // base URL will be used instead. Uses CompleteURLWithOverride internally.
  KURL CompleteURL(
      const String&,
      const CompleteURLPreloadStatus preload_status = kIsNotPreload) const;
  // Creates URL based on passed relative url and passed base URL override.
  KURL CompleteURLWithOverride(
      const String&,
      const KURL& base_url_override,
      const CompleteURLPreloadStatus preload_status = kIsNotPreload) const;

  // Determines whether a new document should take on the same origin as that of
  // the document which created it.
  static bool ShouldInheritSecurityOriginFromOwner(const KURL&);

  CSSStyleSheet& ElementSheet();

  virtual DocumentParser* CreateParser();
  DocumentParser* Parser() const { return parser_.Get(); }
  ScriptableDocumentParser* GetScriptableDocumentParser() const;

  // FinishingPrinting denotes that the non-printing layout state is being
  // restored.
  enum PrintingState {
    kNotPrinting,
    kBeforePrinting,
    kPrinting,
    kFinishingPrinting
  };
  bool Printing() const { return printing_ == kPrinting; }
  bool BeforePrintingOrPrinting() const {
    return printing_ == kPrinting || printing_ == kBeforePrinting;
  }
  bool FinishingOrIsPrinting() const {
    return printing_ == kPrinting || printing_ == kFinishingPrinting;
  }
  void SetPrinting(PrintingState);
  // Call this if printing is about to begin, so that any unloaded resources
  // (such as lazy-loaded images) necessary for printing are requested and
  // marked as blocking load. Returns whether any resources have started
  // loading as a result.
  bool WillPrintSoon();

  enum PaintPreviewState {
    // A paint preview is not in the process of being captured.
    kNotPaintingPreview = 0,

    // A paint preview is in the process of being captured.
    kPaintingPreview,

    // The same as `kPaintingPreview`, but where appropriate GPU accelerated
    // content should be skipped during painting. This can reduce hangs and
    // memory usage at the expense of a lower fidelity capture.
    kPaintingPreviewSkipAcceleratedContent,
  };
  PaintPreviewState GetPaintPreviewState() const { return paint_preview_; }
  bool IsPrintingOrPaintingPreview() const {
    return Printing() ||
           GetPaintPreviewState() != Document::kNotPaintingPreview;
  }

  enum CompatibilityMode { kQuirksMode, kLimitedQuirksMode, kNoQuirksMode };

  void SetCompatibilityMode(CompatibilityMode);
  CompatibilityMode GetCompatibilityMode() const { return compatibility_mode_; }

  String compatMode() const;

  bool InQuirksMode() const { return compatibility_mode_ == kQuirksMode; }
  bool InLimitedQuirksMode() const {
    return compatibility_mode_ == kLimitedQuirksMode;
  }
  bool InNoQuirksMode() const { return compatibility_mode_ == kNoQuirksMode; }
  bool InLineHeightQuirksMode() const { return !InNoQuirksMode(); }

  // https://html.spec.whatwg.org/C/#documentreadystate
  enum DocumentReadyState { kLoading, kInteractive, kComplete };

  DocumentReadyState GetReadyState() const { return ready_state_; }
  void SetReadyState(DocumentReadyState);
  bool IsLoadCompleted() const;

  bool IsFreezingInProgress() const { return is_freezing_in_progress_; }

  enum ParsingState { kParsing, kInDOMContentLoaded, kFinishedParsing };
  void SetParsingState(ParsingState);
  bool Parsing() const { return parsing_state_ == kParsing; }
  bool HasFinishedParsing() const { return parsing_state_ == kFinishedParsing; }

  bool ShouldScheduleLayout() const;

  TextLinkColors& GetTextLinkColors() { return text_link_colors_; }
  const TextLinkColors& GetTextLinkColors() const { return text_link_colors_; }
  VisitedLinkState& GetVisitedLinkState();

  MouseEventWithHitTestResults PerformMouseEventHitTest(const HitTestRequest&,
                                                        const PhysicalOffset&,
                                                        const WebMouseEvent&);

  void SetHadKeyboardEvent(bool had_keyboard_event) {
    had_keyboard_event_ = had_keyboard_event;
  }
  bool HadKeyboardEvent() const { return had_keyboard_event_; }
  void SetLastFocusType(mojom::blink::FocusType last_focus_type);
  mojom::blink::FocusType LastFocusType() const { return last_focus_type_; }
  bool SetFocusedElement(Element*, const FocusParams&);
  void ClearFocusedElement(bool omit_blur_events = false);
  Element* FocusedElement() const { return focused_element_.Get(); }
  void ClearFocusedElementIfNeeded();
  UserActionElementSet& UserActionElements() { return user_action_elements_; }
  const UserActionElementSet& UserActionElements() const {
    return user_action_elements_;
  }

  CachedAttrAssociatedElementsMap* GetCachedAttrAssociatedElementsMap(Element*);
  void MoveElementCachedAttrAssociatedElementsMapToNewDocument(
      Element*,
      Document& new_document);
  inline bool HasCachedAttrAssociatedElements() const {
    return !element_cached_attr_associated_elements_map_.empty();
  }

  // Returns false if the function fails.  e.g. |pseudo| is not supported.
  bool SetPseudoStateForTesting(Element& element,
                                const String& pseudo,
                                bool matches);
  void EnqueueAutofocusCandidate(Element&);
  bool HasAutofocusCandidates() const;
  void FlushAutofocusCandidates();
  void FinalizeAutofocus();
  Element* GetAutofocusDelegate() const;
  void SetSequentialFocusNavigationStartingPoint(Node*);
  Element* SequentialFocusNavigationStartingPoint(
      mojom::blink::FocusType) const;

  void SetActiveElement(Element*);
  Element* GetActiveElement() const { return active_element_.Get(); }

  void AddFocusedElementChangeObserver(FocusedElementChangeObserver*);
  void RemoveFocusedElementChangeObserver(FocusedElementChangeObserver*);

  Element* HoverElement() const { return hover_element_.Get(); }

  void RemoveFocusedElementOfSubtree(Node&, bool among_children_only = false);
  void HoveredElementDetached(Element&);
  void ActiveChainNodeDetached(Element&);

  // Updates hover and active state of elements in the Document. The
  // |is_active| param specifies whether the active state should be set or
  // unset. |update_active_chain| is used to prevent updates to elements
  // outside the frozen active chain; passing false will only refresh the
  // active state of elements in the existing chain, but not outside of it. The
  // given element is the inner-most element whose state is being modified.
  // Hover is always applied.
  void UpdateHoverActiveState(bool is_active,
                              bool update_active_chain,
                              Element*);

  // Updates for :target (CSS3 selector).
  void SetCSSTarget(Element*);
  Element* CssTarget() const { return css_target_.Get(); }
  void SetSelectorFragmentAnchorCSSTarget(Element*);

  void ScheduleLayoutTreeUpdateIfNeeded();
  bool HasPendingForcedStyleRecalc() const;

  void RegisterNodeList(const LiveNodeListBase*);
  void UnregisterNodeList(const LiveNodeListBase*);
  void RegisterNodeListWithIdNameCache(const LiveNodeListBase*);
  void UnregisterNodeListWithIdNameCache(const LiveNodeListBase*);
  bool ShouldInvalidateNodeListCaches(
      const QualifiedName* attr_name = nullptr) const;
  void InvalidateNodeListCaches(const QualifiedName* attr_name);

  void AttachNodeIterator(NodeIterator*);
  void DetachNodeIterator(NodeIterator*);
  void MoveNodeIteratorsToNewDocument(Node&, Document&);
  inline bool HasNodeIterators() const { return !node_iterators_.empty(); }

  void AttachRange(Range*);
  void DetachRange(Range*);
  inline bool HasRanges() const { return !ranges_.empty(); }

  void DidMoveTreeToNewDocument(const Node& root);
  // nodeChildrenWillBeRemoved is used when removing all node children at once.
  void NodeChildrenWillBeRemoved(ContainerNode&);
  // nodeWillBeRemoved is only safe when removing one node at a time.
  void NodeWillBeRemoved(Node&);
  bool CanAcceptChild(const Node* new_child,
                      const VectorOf<Node>* new_children,
                      const Node* next,
                      const Node* old_child,
                      ExceptionState&) const;

  void DidInsertText(const CharacterData&, unsigned offset, unsigned length);
  void DidRemoveText(const CharacterData&, unsigned offset, unsigned length);
  void DidMergeTextNodes(const Text& merged_node,
                         const Text& node_to_be_removed,
                         unsigned old_length);
  void DidSplitTextNode(const Text& old_node);

  LocalDOMWindow* domWindow() const { return dom_window_.Get(); }

  // Helper functions for forwarding LocalDOMWindow event related tasks to the
  // LocalDOMWindow if it exists.
  void SetWindowAttributeEventListener(const AtomicString& event_type,
                                       EventListener*);
  EventListener* GetWindowAttributeEventListener(
      const AtomicString& event_type);

  static void RegisterEventFactory(std::unique_ptr<EventFactoryBase>);
  static Event* createEvent(ScriptState*,
                            const String& event_type,
                            ExceptionState&);

  // keep track of what types of event listeners are registered, so we don't
  // dispatch events unnecessarily
  enum ListenerType {
    kDOMSubtreeModifiedListener = 1,
    kDOMNodeInsertedListener = 1 << 1,
    kDOMNodeRemovedListener = 1 << 2,
    kDOMNodeRemovedFromDocumentListener = 1 << 3,
    kDOMNodeInsertedIntoDocumentListener = 1 << 4,
    kDOMCharacterDataModifiedListener = 1 << 5,
    kAnimationEndListener = 1 << 6,
    kAnimationStartListener = 1 << 7,
    kAnimationIterationListener = 1 << 8,
    kAnimationCancelListener = 1 << 9,
    kTransitionRunListener = 1 << 10,
    kTransitionStartListener = 1 << 11,
    kTransitionEndListener = 1 << 12,
    kTransitionCancelListener = 1 << 13,
    kScrollListener = 1 << 14,
    kLoadListenerAtCapturePhaseOrAtStyleElement = 1 << 15,
    // 0 bits remaining
    kDOMMutationEventListener =
        kDOMSubtreeModifiedListener | kDOMNodeInsertedListener |
        kDOMNodeRemovedListener | kDOMNodeRemovedFromDocumentListener |
        kDOMNodeInsertedIntoDocumentListener |
        kDOMCharacterDataModifiedListener,
  };

  bool HasListenerType(ListenerType listener_type) const;
  void AddListenerTypeIfNeeded(const AtomicString& event_type, EventTarget&);

  void DidAddEventListeners(uint32_t count);
  void DidRemoveEventListeners(uint32_t count);
  bool HasAnyNodeWithEventListeners() const { return event_listener_counts_; }

  bool HasMutationObserversOfType(MutationType type) const {
    return mutation_observer_types_ & type;
  }
  bool HasMutationObservers() const { return mutation_observer_types_; }
  void AddMutationObserverTypes(MutationType types) {
    mutation_observer_types_ |= types;
  }

  IntersectionObserverController* GetIntersectionObserverController();
  IntersectionObserverController& EnsureIntersectionObserverController();

  // This is used to track IntersectionObservers for which this document is the
  // explicit root. The IntersectionObserverController tracks *all* observers
  // associated with this document; usually that's what you want.
  ElementIntersectionObserverData*
  DocumentExplicitRootIntersectionObserverData() const;
  ElementIntersectionObserverData&
  EnsureDocumentExplicitRootIntersectionObserverData();

  const ScriptRegexp& EnsureEmailRegexp() const;

  // Returns the owning element in the parent document. Returns nullptr if
  // this is the top level document or the owner is remote.
  HTMLFrameOwnerElement* LocalOwner() const;

  void WillChangeFrameOwnerProperties(
      int margin_width,
      int margin_height,
      mojom::blink::ScrollbarMode,
      bool is_display_none,
      mojom::blink::ColorScheme color_scheme,
      mojom::blink::PreferredColorScheme preferred_color_scheme);

  String title() const { return title_; }
  void setTitle(const String&);

  Element* TitleElement() const { return title_element_.Get(); }
  void SetTitleElement(Element*);
  void RemoveTitle(Element* title_element);

  const AtomicString& dir();
  void setDir(const AtomicString&);

  String cookie(ExceptionState&) const;
  void setCookie(const String&, ExceptionState&);
  bool CookiesEnabled() const;

  void SetCookieManager(
      mojo::PendingRemote<network::mojom::blink::RestrictedCookieManager>
          cookie_manager);

  const base::Uuid& base_auction_nonce();

  const AtomicString& referrer() const;

  String domain() const;
  void setDomain(const String& new_domain, ExceptionState&);

  void OverrideLastModified(const AtomicString& modified) {
    override_last_modified_ = modified;
  }
  std::optional<base::Time> lastModifiedTime() const;
  String lastModified() const;

  // The cookieURL is used to query the cookie database for this document's
  // cookies. For example, if the cookie URL is http://example.com, we'll
  // use the non-Secure cookies for example.com when computing
  // document.cookie.
  //
  // Q: How is the cookieURL different from the document's URL?
  // A: The two URLs are the same almost all the time.  However, if one
  //    document inherits the security context of another document, it
  //    inherits its cookieURL but not its URL.
  //
  const KURL& CookieURL() const { return cookie_url_; }

  // Returns null if the document is not attached to a frame.
  scoped_refptr<const SecurityOrigin> TopFrameOrigin() const;

  net::SiteForCookies SiteForCookies() const;

  // Permissions service helper methods to facilitate requesting and checking
  // storage access permissions.
  mojom::blink::PermissionService* GetPermissionService(
      ExecutionContext* execution_context);
  void PermissionServiceConnectionError();

  // Fragment directive API, currently used to feature detect text-fragments.
  // https://wicg.github.io/scroll-to-text-fragment/#feature-detectability
  FragmentDirective& fragmentDirective() const;

  // Sends a query via Mojo to ask whether the user has any private
  // tokens. This can reject on permissions errors (e.g. associating |issuer|
  // with the top-level origin would exceed the top-level origin's limit on the
  // number of associated issuers) or on other internal errors (e.g. the network
  // service is unavailable).
  ScriptPromise<IDLBoolean> hasPrivateToken(ScriptState* script_state,
                                            const String& issuer,
                                            ExceptionState&);

  // Sends a query via Mojo to ask whether the user has a redemption record.
  // This can reject on permissions errors (e.g. associating |issuer| with the
  // top-level origin would exceed the top-level origin's limit on the number of
  // associated issuers) or on other internal errors (e.g. the network service
  // is unavailable).
  ScriptPromise<IDLBoolean> hasRedemptionRecord(ScriptState* script_state,
                                                const String& issuer,
                                                ExceptionState&);

  void ariaNotify(const String& announcement,
                  const AriaNotificationOptions* options);

  // The following implements the rule from HTML 4 for what valid names are.
  // To get this right for all the XML cases, we probably have to improve this
  // or move it and make it sensitive to the type of document.
  static bool IsValidName(const StringView&);

  // The following breaks a qualified name into a prefix and a local name.
  // It also does a validity check, and returns false if the qualified name
  // is invalid.  It also sets ExceptionCode when name is invalid.
  static bool ParseQualifiedName(const AtomicString& qualified_name,
                                 AtomicString& prefix,
                                 AtomicString& local_name,
                                 ExceptionState&);

  // Checks to make sure prefix and namespace do not conflict (per DOM Core 3)
  static bool HasValidNamespaceForElements(const QualifiedName&);
  static bool HasValidNamespaceForAttributes(const QualifiedName&);

  // "body element" as defined by HTML5
  // (https://html.spec.whatwg.org/C/#the-body-element-2).
  // That is, the first body or frameset child of the document element.
  HTMLElement* body() const;

  // "HTML body element" as defined by CSSOM View spec
  // (https://drafts.csswg.org/cssom-view/#the-html-body-element).
  // That is, the first body child of the document element.
  HTMLBodyElement* FirstBodyElement() const;

  void setBody(HTMLElement*, ExceptionState&);
  void WillInsertBody();

  HTMLHeadElement* head() const;

  // Decide which element is to define the viewport's overflow policy.
  Element* ViewportDefiningElement() const;

  DocumentMarkerController& Markers() const { return *markers_; }

  // Support for Javascript execCommand, and related methods
  // See "core/editing/commands/document_exec_command.cc" for implementations.
  bool execCommand(const String& command,
                   bool show_ui,
                   const V8UnionStringOrTrustedHTML* value,
                   ExceptionState&);

  bool execCommand(const String& command,
                   bool show_ui,
                   const String& value,
                   ExceptionState&);

  bool IsRunningExecCommand() const { return is_running_exec_command_; }
  bool queryCommandEnabled(const String& command, ExceptionState&);
  bool queryCommandIndeterm(const String& command, ExceptionState&);
  bool queryCommandState(const String& command, ExceptionState&);
  bool queryCommandSupported(const String& command, ExceptionState&);
  String queryCommandValue(const String& command, ExceptionState&);

  KURL OpenSearchDescriptionURL();

  // designMode support
  bool InDesignMode() const { return design_mode_; }
  String designMode() const;
  void setDesignMode(const String&);

  // The document of the parent frame.
  Document* ParentDocument() const;
  Document& TopDocument() const;

  // Will only return nullptr if the document has Shutdown() or in unit tests.
  // See `execution_context_` for details.
  ExecutionContext* GetExecutionContext() const final;

  // Return the agent.
  Agent& GetAgent() const;

  ScriptRunner* GetScriptRunner() { return script_runner_.Get(); }
  const base::ElapsedTimer& GetStartTime() const { return start_time_; }

  V8HTMLOrSVGScriptElement* currentScriptForBinding() const;
  void PushCurrentScript(ScriptElementBase*);
  void PopCurrentScript(ScriptElementBase*);

  void SetTransformSource(std::unique_ptr<TransformSource>);
  TransformSource* GetTransformSource() const {
    return transform_source_.get();
  }

  void IncDOMTreeVersion() {
    DCHECK(lifecycle_.StateAllowsTreeMutations());
    dom_tree_version_ = ++global_tree_version_;
  }
  uint64_t DomTreeVersion() const { return dom_tree_version_; }

  uint64_t StyleVersion() const { return style_version_; }

  enum PendingSheetLayout {
    kNoLayoutWithPendingSheets,
    kDidLayoutWithPendingSheets,
    kIgnoreLayoutWithPendingSheets
  };

  Vector<IconURL> IconURLs(int icon_types_mask);

  void UpdateThemeColorCache();
  std::optional<Color> ThemeColor();

  // Returns the HTMLLinkElement currently in use for the Web Manifest.
  // Returns null if there is no such element.
  HTMLLinkElement* LinkManifest() const;

  // Returns the HTMLLinkElement holding the canonical URL. Returns null if
  // there is no such element.
  HTMLLinkElement* LinkCanonical() const;

  void SetShouldUpdateSelectionAfterLayout(bool flag) {
    should_update_selection_after_layout_ = flag;
  }
  bool ShouldUpdateSelectionAfterLayout() const {
    return should_update_selection_after_layout_;
  }

  void SendFocusNotification(Element*, mojom::blink::FocusType);

  bool IsDNSPrefetchEnabled() const { return is_dns_prefetch_enabled_; }
  void ParseDNSPrefetchControlHeader(const String&);

  void MarkFirstPaint();
  void OnLargestContentfulPaintUpdated();
  void OnPrepareToStopParsing();
  void FinishedParsing();

  void SetEncodingData(const DocumentEncodingData& new_data);
  const WTF::TextEncoding& Encoding() const {
    return encoding_data_.Encoding();
  }

  bool EncodingWasDetectedHeuristically() const {
    return encoding_data_.WasDetectedHeuristically();
  }
  bool SawDecodingError() const { return encoding_data_.SawDecodingError(); }

  // Draggable regions are set using the "app-region" CSS property.
  void SetDraggableRegionsDirty(bool f) { draggable_regions_dirty_ = f; }
  bool DraggableRegionsDirty() const { return draggable_regions_dirty_; }
  bool HasDraggableRegions() const { return has_draggable_regions_; }
  void SetHasDraggableRegions(bool f) { has_draggable_regions_ = f; }
  const Vector<DraggableRegionValue>& DraggableRegions() const;
  void SetDraggableRegions(const Vector<DraggableRegionValue>&);

  void RemovedEventListener(const AtomicString& event_type,
                            const RegisteredEventListener&) final;
  void RemoveAllEventListeners() final;

  const SVGDocumentExtensions* SvgExtensions() const;
  SVGDocumentExtensions& AccessSVGExtensions();

  bool AllowInlineEventHandler(Node*,
                               EventListener*,
                               const String& context_url,
                               const WTF::OrdinalNumber& context_line);

  void StatePopped(scoped_refptr<SerializedScriptValue>);

  enum LoadEventProgress {
    kLoadEventNotRun,
    kLoadEventInProgress,
    kLoadEventCompleted,
    kBeforeUnloadEventInProgress,
    // Advanced to only if the beforeunload event in this document and
    // subdocuments isn't canceled and will cause an unload. If beforeunload is
    // canceled |load_event_progress_| will revert to its value prior to the
    // beforeunload being dispatched.
    kBeforeUnloadEventHandled,
    kPageHideInProgress,
    kUnloadVisibilityChangeInProgress,
    kUnloadEventInProgress,
    kUnloadEventHandled
  };
  bool LoadEventStillNeeded() const {
    return load_event_progress_ == kLoadEventNotRun;
  }
  bool LoadEventStarted() const {
    return load_event_progress_ == kLoadEventInProgress;
  }
  bool LoadEventFinished() const {
    return load_event_progress_ >= kLoadEventCompleted;
  }
  bool BeforeUnloadStarted() const {
    return load_event_progress_ >= kBeforeUnloadEventInProgress;
  }
  bool ProcessingBeforeUnload() const {
    return load_event_progress_ == kBeforeUnloadEventInProgress;
  }
  bool UnloadStarted() const {
    return load_event_progress_ >= kPageHideInProgress;
  }
  bool UnloadEventInProgress() const {
    return load_event_progress_ == kUnloadEventInProgress;
  }

  void BeforeUnloadDoneWillUnload() {
    load_event_progress_ = kBeforeUnloadEventHandled;
  }

  void SetContainsPlugins() { contains_plugins_ = true; }
  bool ContainsPlugins() const { return contains_plugins_; }

  void EnqueueMoveEvent();
  void EnqueueResizeEvent();
  void EnqueueScrollEventForNode(Node*);
  void EnqueueScrollEndEventForNode(Node*);
  void EnqueueOverscrollEventForNode(Node* target,
                                     double delta_x,
                                     double delta_y);
  void EnqueueDisplayLockActivationTask(base::OnceClosure);
  void EnqueueAnimationFrameTask(base::OnceClosure);
  void EnqueueAnimationFrameEvent(Event*);
  // Only one event for a target/event type combination will be dispatched per
  // frame.
  void EnqueueUniqueAnimationFrameEvent(Event*);
  void EnqueueMediaQueryChangeListeners(
      HeapVector<Member<MediaQueryListListener>>&);
  void EnqueueVisualViewportScrollEvent();
  void EnqueueVisualViewportScrollEndEvent();
  void EnqueueVisualViewportResizeEvent();
  void EnqueueScrollSnapChangeEvent(Node* target,
                                    Member<Node>& block_target,
                                    Member<Node>& inline_target);
  void EnqueueScrollSnapChangingEvent(Node* target,
                                      Member<Node>& block_target,
                                      Member<Node>& inline_target);

  void DispatchEventsForPrinting();

  void exitPointerLock();
  Element* PointerLockElement() const;

  // Used to allow element that loads data without going through a FrameLoader
  // to delay the 'load' event.
  void IncrementLoadEventDelayCount() { ++load_event_delay_count_; }
  void DecrementLoadEventDelayCount();
  void CheckLoadEventSoon();
  bool IsDelayingLoadEvent();
  void LoadPluginsSoon();
  // This calls CheckCompleted() sync and thus can cause JavaScript execution.
  void DecrementLoadEventDelayCountAndCheckLoadEvent();
  // Objects and embeds depend on "being rendered" for delaying the load event.
  // This method makes sure we run a layout tree update before unblocking the
  // load event after such elements have been inserted.
  //
  // Spec:
  //
  // https://html.spec.whatwg.org/multipage/iframe-embed-object.html#the-object-element
  // https://html.spec.whatwg.org/multipage/iframe-embed-object.html#the-embed-element
  void DelayLoadEventUntilLayoutTreeUpdate();

  const DocumentTiming& GetTiming() const { return document_timing_; }

  bool ShouldMarkFontPerformance() const {
    return !IsInitialEmptyDocument() && !IsXMLDocument() &&
           IsInOutermostMainFrame();
  }

  int RequestAnimationFrame(FrameCallback*);
  void CancelAnimationFrame(int id);

  ScriptedAnimationController& GetScriptedAnimationController();

  void InitDNSPrefetch();

  bool IsInDocumentWrite() const { return write_recursion_depth_ > 0; }

  TextAutosizer* GetTextAutosizer();

  ScriptValue registerElement(ScriptState*,
                              const AtomicString& name,
                              const ElementRegistrationOptions*,
                              ExceptionState&);

  void AdjustQuadsForScrollAndAbsoluteZoom(Vector<gfx::QuadF>&,
                                           const LayoutObject&) const;
  void AdjustRectForScrollAndAbsoluteZoom(gfx::RectF&,
                                          const LayoutObject&) const;

  ElementDataCache* GetElementDataCache() { return element_data_cache_.Get(); }

  void DidLoadAllScriptBlockingResources();
  void DidAddPendingParserBlockingStylesheet();
  void DidLoadAllPendingParserBlockingStylesheets();
  void DidRemoveAllPendingStylesheets();

  bool InStyleRecalc() const;

  // Return a Locale for the default locale if the argument is null or empty.
  Locale& GetCachedLocale(const AtomicString& locale = g_null_atom);

  AnimationClock& GetAnimationClock();
  const AnimationClock& GetAnimationClock() const;
  DocumentAnimations& GetDocumentAnimations() const {
    return *document_animations_;
  }
  DocumentTimeline& Timeline() const { return *timeline_; }
  PendingAnimations& GetPendingAnimations() { return *pending_animations_; }
  WorkletAnimationController& GetWorkletAnimationController() {
    return *worklet_animation_controller_;
  }

  void AttachCompositorTimeline(cc::AnimationTimeline*) const;

  enum class TopLayerReason {
    kFullscreen,
    kDialog,
    kPopover,
  };
  void AddToTopLayer(Element*, const Element* before = nullptr);
  void RemoveFromTopLayerImmediately(Element*);
  const HeapVector<Member<Element>>& TopLayerElements() const {
    return top_layer_elements_;
  }
  void ScheduleForTopLayerRemoval(Element*, TopLayerReason);
  void RemoveFinishedTopLayerElements();
  // Returns std::nullopt if the provided element is not scheduled for top
  // layer removal. If it is scheduled for removal, then this returns the reason
  // for the element being in the top layer.
  std::optional<TopLayerReason> IsScheduledForTopLayerRemoval(Element*) const;

  HTMLDialogElement* ActiveModalDialog() const;

  using PopoverStack = HeapVector<Member<HTMLElement>>;
  const PopoverStack& PopoverHintStack() const { return popover_hint_stack_; }
  PopoverStack& PopoverHintStack() { return popover_hint_stack_; }
  bool PopoverHintShowing() const { return !popover_hint_stack_.empty(); }
  PopoverStack& PopoverAutoStack() { return popover_auto_stack_; }
  const PopoverStack& PopoverAutoStack() const { return popover_auto_stack_; }
  bool PopoverAutoShowing() const { return !popover_auto_stack_.empty(); }
  HeapHashSet<Member<HTMLElement>>& AllOpenPopovers() {
    return all_open_popovers_;
  }
  HTMLElement* TopmostPopoverOrHint() const;
  HeapHashSet<Member<HTMLElement>>& PopoversWaitingToHide() {
    return popovers_waiting_to_hide_;
  }
  const HTMLElement* PopoverPointerdownTarget() const {
    return popover_pointerdown_target_.Get();
  }
  void SetPopoverPointerdownTarget(const HTMLElement*);
  std::optional<gfx::PointF> CustomizableSelectMousedownLocation() const {
    return customizable_select_mousedown_location_;
  }
  void SetCustomizableSelectMousedownLocation(std::optional<gfx::PointF>);
  const HTMLDialogElement* DialogPointerdownTarget() const;
  void SetDialogPointerdownTarget(const HTMLDialogElement*);

  HeapLinkedHashSet<Member<HTMLDialogElement>>& AllOpenDialogs() {
    return all_open_dialogs_;
  }

  HeapLinkedHashSet<Member<Element>>& CurrentInterestTargetElements();

  // https://crbug.com/1453291
  // The DOM Parts API:
  // https://github.com/WICG/webcomponents/blob/gh-pages/proposals/DOM-Parts.md.
  DocumentPartRoot& getPartRoot();
  DocumentPartRoot& EnsureDocumentPartRoot();
  bool DOMPartsInUse() const { return document_part_root_ != nullptr; }

  // A non-null template_document_host_ implies that |this| was created by
  // EnsureTemplateDocument().
  bool IsTemplateDocument() const { return template_document_host_ != nullptr; }
  Document& EnsureTemplateDocument();
  Document* TemplateDocumentHost() { return template_document_host_.Get(); }

  // Signals the ChromeClient that a (Form|Listed)Element changed dynamically,
  // passing the changed element as well as the type of the change.
  // TODO(crbug.com/1483242): Fire the signal for elements that become hidden.
  void DidChangeFormRelatedElementDynamically(HTMLElement*,
                                              WebFormRelatedChangeType);

  void AddConsoleMessage(ConsoleMessage* message,
                         bool discard_duplicates = false) const;

  DocumentLifecycle& Lifecycle() { return lifecycle_; }
  const DocumentLifecycle& Lifecycle() const { return lifecycle_; }
  bool IsActive() const { return lifecycle_.IsActive(); }
  bool IsDetached() const {
    return lifecycle_.GetState() >= DocumentLifecycle::kStopping;
  }
  bool IsStopped() const {
    return lifecycle_.GetState() == DocumentLifecycle::kStopped;
  }
  bool InvalidationDisallowed() const;

  enum HttpRefreshType { kHttpRefreshFromHeader, kHttpRefreshFromMetaTag };
  void MaybeHandleHttpRefresh(const String&, HttpRefreshType);
  bool IsHttpRefreshScheduledWithin(base::TimeDelta interval);

  // Marks the Document has having at least one Element which depends
  // on the specified ViewportUnitFlags.
  void AddViewportUnitFlags(unsigned flags) { viewport_unit_flags_ |= flags; }

  bool HasViewportUnits() const { return viewport_unit_flags_; }
  bool HasStaticViewportUnits() const {
    return viewport_unit_flags_ &
           static_cast<unsigned>(ViewportUnitFlag::kStatic);
  }
  bool HasDynamicViewportUnits() const {
    return viewport_unit_flags_ &
           static_cast<unsigned>(ViewportUnitFlag::kDynamic);
  }

  void LayoutViewportWasResized();
  void MarkViewportUnitsDirty();

  // dv*
  void DynamicViewportUnitsChanged();

  void InvalidateStyleAndLayoutForFontUpdates();

  void Trace(Visitor*) const override;

  AtomicString ConvertLocalName(const AtomicString&);

  void PlatformColorsChanged();

  NthIndexCache* GetNthIndexCache() const { return nth_index_cache_; }

  CheckPseudoHasCacheScope* GetCheckPseudoHasCacheScope() const {
    return check_pseudo_has_cache_scope_;
  }
  bool InPseudoHasChecking() const { return in_pseudo_has_checking_; }

  CanvasFontCache* GetCanvasFontCache();

  // Used by unit tests so that all parsing will be synchronous for
  // controlling parsing and chunking precisely.
  static void SetForceSynchronousParsingForTesting(bool);
  static bool ForceSynchronousParsingForTesting();

#if DCHECK_IS_ON()
  void IncrementNodeCount() { node_count_++; }
  void DecrementNodeCount() {
    DCHECK_GT(node_count_, 0);
    node_count_--;
  }
#endif  // DCHECK_IS_ON()

  void SetContainsShadowRoot() { may_contain_shadow_roots_ = true; }

  bool MayContainShadowRoots() const { return may_contain_shadow_roots_; }

  RootScrollerController& GetRootScrollerController() const {
    DCHECK(root_scroller_controller_);
    return *root_scroller_controller_;
  }

  AnchorElementInteractionTracker* GetAnchorElementInteractionTracker() const {
    return anchor_element_interaction_tracker_.Get();
  }

  // Returns true if this document has a frame and it is a main frame.
  // See `Frame::IsMainFrame`.
  bool IsInMainFrame() const;

  // Returns true if this document has a frame and is an outermost main frame.
  // See `Frame::IsOutermostMainFrame`.
  bool IsInOutermostMainFrame() const;

  const PropertyRegistry* GetPropertyRegistry() const {
    return property_registry_.Get();
  }
  PropertyRegistry& EnsurePropertyRegistry();

  // May return nullptr when PerformanceManager instrumentation is disabled,
  // when the Document is inactive or when the document was installed for
  // discarding.
  DocumentResourceCoordinator* GetResourceCoordinator();

  const AtomicString& bgColor() const;
  void setBgColor(const AtomicString&);
  const AtomicString& fgColor() const;
  void setFgColor(const AtomicString&);
  const AtomicString& alinkColor() const;
  void setAlinkColor(const AtomicString&);
  const AtomicString& linkColor() const;
  void setLinkColor(const AtomicString&);
  const AtomicString& vlinkColor() const;
  void setVlinkColor(const AtomicString&);

  void clear() {}

  void captureEvents() {}
  void releaseEvents() {}

  FontFaceSet* fonts();

  ukm::UkmRecorder* UkmRecorder();
  ukm::SourceId UkmSourceID() const;

  // Tracks and reports UKM metrics of the number of attempted font family match
  // attempts (both successful and not successful) by the page. This will return
  // null if the document is stopped.
  FontMatchingMetrics* GetFontMatchingMetrics();

  void MaybeRecordSvgImageProcessingTime(
      int data_change_count,
      base::TimeDelta data_change_elapsed_time) const;

  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner(TaskType);

  StylePropertyMapReadOnly* ComputedStyleMap(Element*);
  void AddComputedStyleMapItem(Element*, StylePropertyMapReadOnly*);
  StylePropertyMapReadOnly* RemoveComputedStyleMapItem(Element*);

  SlotAssignmentEngine& GetSlotAssignmentEngine();

  bool IsSlotAssignmentDirty() const;

#if DCHECK_IS_ON()
  unsigned& SlotAssignmentRecalcForbiddenRecursionDepth() {
    return slot_assignment_recalc_forbidden_recursion_depth_;
  }
  bool IsSlotAssignmentRecalcForbidden() {
    return slot_assignment_recalc_forbidden_recursion_depth_ > 0;
  }
#endif

#if EXPENSIVE_DCHECKS_ARE_ON()
  void AssertLayoutTreeUpdatedAfterLayout();
#endif

  unsigned& FlatTreeTraversalForbiddenRecursionDepth() {
    return flat_tree_traversal_forbidden_recursion_depth_;
  }
  bool IsFlatTreeTraversalForbidden() {
    return flat_tree_traversal_forbidden_recursion_depth_ > 0;
  }

  unsigned& SlotAssignmentRecalcDepth() {
    return slot_assignment_recalc_depth_;
  }
  bool IsInSlotAssignmentRecalc() const {
    // Since we forbid recursive slot assignement recalc, the depth should be
    // <= 1.
    DCHECK_LE(slot_assignment_recalc_depth_, 1u);
    return slot_assignment_recalc_depth_ == 1;
  }

  bool ShouldSuppressMutationEvents() const {
    return suppress_mutation_events_;
  }
  // To be called from MutationEventSuppressionScope.
  void SetSuppressMutationEvents(bool suppress) {
    suppress_mutation_events_ = suppress;
  }

  bool IsVerticalScrollEnforced() const { return is_vertical_scroll_enforced_; }
  bool IsFocusAllowed(FocusTrigger trigger) const;

  LazyLoadImageObserver& EnsureLazyLoadImageObserver();

  void IncrementNumberOfCanvases();
  unsigned GetNumberOfCanvases() const { return num_canvases_; }

  void ProcessJavaScriptUrl(const KURL&, const DOMWrapperWorld* world);

  DisplayLockDocumentState& GetDisplayLockDocumentState() const;

  // Deferred compositor commits are disallowed by default, and are only allowed
  // for html documents fetched via the http family of protocols.
  bool DeferredCompositorCommitIsAllowed() const;
  void SetDeferredCompositorCommitIsAllowed(bool new_value) {
    deferred_compositor_commit_is_allowed_ = new_value;
  }

  // Returns whether the document is inside the scope specified in the Web App
  // Manifest. If the document doesn't run in a context of a Web App or has no
  // associated Web App Manifest, it will return false.
  bool IsInWebAppScope() const;

  void DispatchHandleLoadStart();
  void DispatchHandleLoadComplete();

  bool HaveRenderBlockingStylesheetsLoaded() const;
  bool HaveRenderBlockingResourcesLoaded() const;

  // Sets a beforeunload handler for documents which are embedding plugins. This
  // includes PluginDocument as well as an HTMLDocument which embeds a plugin
  // inside a cross-process frame (MimeHandlerView).
  void SetShowBeforeUnloadDialog(bool show_dialog);

  void ColorSchemeChanged();

  // A new vision deficiency is being emulated through DevTools.
  void VisionDeficiencyChanged();

  // A META element with name=color-scheme was added, removed, or modified.
  // Update the presentation level color-scheme property for the root element.
  void ColorSchemeMetaChanged();

  // A META element with name=supports-reduced-motion was added, removed, or
  // modified. Re-collect the META values.
  void SupportsReducedMotionMetaChanged();

  // Use counter related functions.
  void CountUse(mojom::WebFeature feature) final;
  void CountDeprecation(mojom::WebFeature feature) final;
  void CountUse(mojom::WebFeature feature) const;
  void CountWebDXFeature(mojom::blink::WebDXFeature feature) final;
  void CountWebDXFeature(mojom::blink::WebDXFeature feature) const;
  void CountProperty(CSSPropertyID property_id) const;
  void CountAnimatedProperty(CSSPropertyID property_id) const;
  // Return whether the Feature was previously counted for this document.
  // NOTE: only for use in testing.
  bool IsUseCounted(mojom::WebFeature) const;
  // Return whether the property was previously counted for this document.
  // NOTE: only for use in testing.
  bool IsWebDXFeatureCounted(mojom::blink::WebDXFeature) const;
  // Return whether the property was previously counted for this document.
  // NOTE: only for use in testing.
  bool IsPropertyCounted(CSSPropertyID property) const;
  // Return whether the animated property was previously counted for this
  // document.
  // NOTE: only for use in testing.
  bool IsAnimatedPropertyCounted(CSSPropertyID property) const;
  void ClearUseCounterForTesting(mojom::WebFeature);
  void ClearWebDXFeatureCounterForTesting(mojom::blink::WebDXFeature);

  void UpdateForcedColors();
  bool InForcedColorsMode() const;
  bool InDarkMode();

  const ui::ColorProvider* GetColorProviderForPainting(
      mojom::blink::ColorScheme color_scheme) const;

  // Capture the toggle event during parsing either by HTML parser or XML
  // parser.
  void SetToggleDuringParsing(bool toggle_during_parsing) {
    toggle_during_parsing_ = toggle_during_parsing;
  }
  bool ToggleDuringParsing() { return toggle_during_parsing_; }

  // We setup a dummy document to sanitize clipboard markup before pasting.
  // Sets and indicates whether this is the dummy document.
  void SetIsForMarkupSanitization(bool is_for_sanitization) {
    is_for_markup_sanitization_ = is_for_sanitization;
  }
  bool IsForMarkupSanitization() const { return is_for_markup_sanitization_; }

  bool HasPendingJavaScriptUrlsForTest() {
    return !pending_javascript_urls_.empty();
  }

  void ApplyScrollRestorationLogic();

  void MarkHasFindInPageRequest();
  void MarkHasFindInPageContentVisibilityActiveMatch();
  void MarkHasFindInPageBeforematchExpandedHiddenMatchable();

  void CancelPendingJavaScriptUrls();

  void NotifyUpdateCharacterData(CharacterData* character_data,
                                 const TextDiffRange&);
  void NotifyChangeChildren(const ContainerNode& container,
                            const ContainerNode::ChildrenChange& change);
  void NotifyAttributeChanged(const Element& element,
                              const QualifiedName& name,
                              const AtomicString& old_value,
                              const AtomicString& new_value);

  RenderBlockingResourceManager* GetRenderBlockingResourceManager() {
    return render_blocking_resource_manager_.Get();
  }

  void SetHasRenderBlockingExpectLinkElements(bool flag);

  bool HasRenderBlockingExpectLinkElements() const {
    return has_render_blocking_expect_link_elements_;
  }

  void SetHasFullFrameRateBlockingExpectLinkElements(bool flag);

  bool HasFullFrameRateBlockingExpectLinkElements() const {
    return has_frame_rate_blocking_expect_link_elements_;
  }

  // Whether the document has any pending elements that need to be tracked for
  // full render blocking or full frame rate blocking.
  bool HasPendingExpectLinkElements() const {
    return has_pending_expect_link_elements_;
  }

  void UpdateRenderFrameRate();

  // Called when a previously render-blocking resource is no longer render-
  // blocking, due to it has finished loading or has given up render-blocking.
  void RenderBlockingResourceUnblocked();

  bool RenderingHasBegun() const { return rendering_has_begun_; }
  bool RenderingHadBegunForLastStyleUpdate() const {
    return rendering_had_begun_for_last_style_update_;
  }

  void IncrementImmediateChildFrameCreationCount();
  int GetImmediateChildFrameCreationCount() const;

  enum class DeclarativeShadowRootAllowState : uint8_t {
    kNotSet,
    kAllow,
    kDeny
  };
  DeclarativeShadowRootAllowState GetDeclarativeShadowRootAllowState() const;
  void setAllowDeclarativeShadowRoots(bool val);

  void SetFindInPageActiveMatchNode(Node*);
  const Node* GetFindInPageActiveMatchNode() const;

  void ActivateForPrerendering(
      const mojom::blink::PrerenderPageActivationParams& params);

  void AddWillDispatchPrerenderingchangeCallback(base::OnceClosure);

  void AddPostPrerenderingActivationStep(base::OnceClosure callback);

  class CORE_EXPORT PaintPreviewScope {
    STACK_ALLOCATED();

   public:
    PaintPreviewScope(Document& document, PaintPreviewState state);
    ~PaintPreviewScope();

    PaintPreviewScope(PaintPreviewScope&) = delete;
    PaintPreviewScope& operator=(PaintPreviewScope&) = delete;

   private:
    Document& document_;
  };

  // Does an element in this document have an HTML dir attribute (or its
  // implicit equivalent)?
  bool HasDirAttribute() const { return has_dir_attribute_; }
  void SetHasDirAttribute() { has_dir_attribute_ = true; }

  ResizeObserver& EnsureResizeObserver();

  void ObserveForIntrinsicSize(Element* element);
  void UnobserveForIntrinsicSize(Element* element);

  void ObserveForLazyLoadedAutoSizedImg(HTMLImageElement* img);
  void UnobserveForLazyLoadedAutoSizedImg(HTMLImageElement* img);

  // Returns true if motion should be forcibly reduced in animations on this
  // document. This returns true if all of the following conditions are true:
  // 1. The user prefers reduced motion.
  // 2. The document does not contain a meta tag indicating it supports and uses
  //    prefers-reduced-motion media queries.
  // 3. The ForceReduceMotion feature is enabled.
  // For more details and explanation, see
  // https://github.com/flackr/reduce-motion/blob/main/explainer.md
  bool ShouldForceReduceMotion() const;

  void AddPendingLinkHeaderPreload(const PendingLinkPreload&);

  // Has no effect if the preload is not initiated by link header.
  void RemovePendingLinkHeaderPreloadIfNeeded(const PendingLinkPreload&);

  void WriteIntoTrace(perfetto::TracedValue ctx) const;

  void IncrementIgnoreDestructiveWriteModuleScriptCount() {
    ignore_destructive_write_module_script_count_++;
  }
  unsigned GetIgnoreDestructiveWriteModuleScriptCount() {
    return ignore_destructive_write_module_script_count_;
  }

  void IncrementDataListCount() { ++data_list_count_; }
  void DecrementDataListCount() {
    DCHECK_GT(data_list_count_, 0u);
    --data_list_count_;
  }
  // Returns true if the Document has at least one data-list associated with
  // it.
  bool HasAtLeastOneDataList() const { return data_list_count_; }

  void IncrementDisabledFieldsetCount() { ++disabled_fieldset_count_; }
  void DecrementDisabledFieldsetCount() {
    DCHECK_GT(disabled_fieldset_count_, 0u);
    --disabled_fieldset_count_;
  }
  bool HasAtLeastOneDisabledFieldset() const {
    return disabled_fieldset_count_;
  }

  // Updates application title based to the latest application title meta tag
  // value.
  void UpdateApplicationTitle();

  void ResetAgent(Agent& agent);

  bool SupportsLegacyDOMMutations();

  void EnqueuePageRevealEvent();

  // https://github.com/whatwg/html/pull/9538
  static Document* parseHTMLUnsafe(ExecutionContext* context,
                                   const String& html,
                                   ExceptionState& exception_state);

  // https://wicg.github.io/sanitizer-api/#framework
  //
  // parseHTMLUnsafe uses an overload, so that we can separately enable/disable
  // the |options| parameter. Long-term, the two parseHTMLUnsage methods
  // should be merged.
  static Document* parseHTMLUnsafe(ExecutionContext* context,
                                   const String& html,
                                   SetHTMLUnsafeOptions* options,
                                   ExceptionState& exception_state);
  static Document* parseHTML(ExecutionContext* context,
                             const String& html,
                             SetHTMLOptions* options,
                             ExceptionState& exception_state);

  // Delays execution of pending async scripts until a milestone is reached.
  // Used in conjunction with kDelayAsyncScriptExecution experiment.
  void DelayAsyncScriptExecution();
  void ResumeAsyncScriptExecution();

  // This method should only be called when the document is top-level and it is
  // rendering static media like video or images.
  void SetOverrideSiteForCookiesForCSPMedia(bool value);

  // Flags to determine if LCPP ElementLocator matched during
  // HTML preload scanning.
  void SetLcpElementFoundInHtml(bool found);
  bool IsLcpElementFoundInHtml();

  // Adds/removes an element to the set of elements that need shadow tree
  // creation on the next layout.
  void ScheduleShadowTreeCreation(HTMLInputElement& element);
  void UnscheduleShadowTreeCreation(HTMLInputElement& element);

  // Traverses DOM tree and collects HTMLAnchorElements to closest ancestor
  // element with scroll-marker-contain property.
  void UpdateScrollMarkerGroupRelations();
  void SetNeedsScrollMarkerGroupRelationsUpdate() {
    needs_scroll_marker_contain_relations_update_ = true;
  }

  // Subscribes each ScrollMarkerGroupData to all scrollers
  // that own corresponding scroll marker's scroll target (see
  // scroll_marker_group_to_scrollable_areas_ for details), so that the scroller
  // will notify ScrollMarkerGroupData of updates.
  void UpdateScrollMarkerGroupToScrollableAreasMap();
  void AddScrollMarkerGroup(ScrollMarkerGroupData* scroll_marker_group);
  void RemoveScrollMarkerGroup(ScrollMarkerGroupData* scroll_marker_group);
  void SetNeedsScrollMarkerGroupsMapUpdate() {
    needs_scroll_marker_groups_map_update_ = true;
  }

  void ScheduleSelectionchangeEvent();

  // Reset to false after the event gets callbacked
  void ResetEventQueueStatus(const AtomicString& event_type) override {
    if (event_type == event_type_names::kSelectionchange)
      has_scheduled_selectionchange_event_on_document_ = false;
  }

  // To partition :visited links, we use a triple-key containing <link_url,
  // top_level_site, frame_origin>. In practice, this means we are frequently
  // querying TopFrameOrigin() and constructing a net::SchemefulSite from it.
  // This is a relatively expensive operation, and since a Document may have
  // many HTMLAnchorElements, it is much more efficient to calculate the
  // SchemefulSite once and store that value here for easy access. Since usage
  // of GetCachedTopFrameSite() is scoped only to VisitedLink use cases, we can
  // reasonably cache top-level site without fear of stale results, as it is
  // safe to assume that the top-level site will not change during the
  // Document's lifetime.
  class VisitedLinkPassKey {
   private:
    friend class HTMLAnchorElementBase;
    VisitedLinkPassKey() = default;
    ~VisitedLinkPassKey() = default;
  };
  net::SchemefulSite GetCachedTopFrameSite(VisitedLinkPassKey);

#if BUILDFLAG(IS_ANDROID)
  // This method is invoked when a payment link element is encountered. It
  // passes the payment link back to browser process through the mojo pipe.
  void HandlePaymentLink(const KURL& href);
#endif

 protected:
  void ClearXMLVersion() { xml_version_ = String(); }

  virtual Document* CloneDocumentWithoutChildren() const;

  void LockCompatibilityMode() { compatibility_mode_locked_ = true; }
  ParserSynchronizationPolicy GetParserSynchronizationPolicy() const {
    return parser_sync_policy_;
  }

 private:
  friend class DocumentTest;
  friend class IgnoreDestructiveWriteCountIncrementer;
  friend class ThrowOnDynamicMarkupInsertionCountIncrementer;
  friend class IgnoreOpensDuringUnloadCountIncrementer;
  friend class NthIndexCache;
  friend class CheckPseudoHasCacheScope;
  friend class CanvasRenderingAPIUkmMetricsTest;
  friend class MobileFriendlinessCheckerTest;
  friend class OffscreenCanvasRenderingAPIUkmMetricsTest;
  friend class TapFriendlinessCheckerTest;
  friend class DocumentStorageAccess;
  FRIEND_TEST_ALL_PREFIXES(LazyLoadAutomaticImagesTest,
                           LoadAllImagesIfPrinting);
  FRIEND_TEST_ALL_PREFIXES(FrameFetchContextSubresourceFilterTest,
                           DuringOnFreeze);
  FRIEND_TEST_ALL_PREFIXES(DocumentTest, FindInPageUkm);
  FRIEND_TEST_ALL_PREFIXES(DocumentTest, FindInPageUkmInFrame);
  FRIEND_TEST_ALL_PREFIXES(TextFinderSimTest,
                           BeforeMatchExpandedHiddenMatchableUkm);
  FRIEND_TEST_ALL_PREFIXES(TextFinderSimTest,
                           BeforeMatchExpandedHiddenMatchableUkmNoHandler);
  FRIEND_TEST_ALL_PREFIXES(DictionaryLoadFromHeaderTest,
                           LoadDictionaryFromHeader);
  FRIEND_TEST_ALL_PREFIXES(
      RangeTest,
      ContainerNodeRemovalWithSequentialFocusNavigationStartingPoint);
  FRIEND_TEST_ALL_PREFIXES(HTMLLinkElementTest,
                           PaymentLinkHandledWhenRelAndHrefSetBeforeAppend);
  FRIEND_TEST_ALL_PREFIXES(HTMLLinkElementTest,
                           PaymentLinkHandledWhenHrefAndRelSetBeforeAppend);
  FRIEND_TEST_ALL_PREFIXES(HTMLLinkElementTest,
                           PaymentLinkHandledWhenRelAndHrefSetAfterAppend);
  FRIEND_TEST_ALL_PREFIXES(HTMLLinkElementTest,
                           PaymentLinkHandledWhenHrefAndRelSetAfterAppend);
  FRIEND_TEST_ALL_PREFIXES(HTMLLinkElementTest,
                           PaymentLinkNotHandledWhenRelNotSet);
  FRIEND_TEST_ALL_PREFIXES(HTMLLinkElementTest,
                           PaymentLinkNotHandledWhenHrefNotSet);
  FRIEND_TEST_ALL_PREFIXES(HTMLLinkElementTest,
                           PaymentLinkNotHandledWhenNotAppended);
  FRIEND_TEST_ALL_PREFIXES(HTMLLinkElementSimTest,
                           PaymentLinkNotHandledWhenNotInTheMainFrame);

  // Listed elements that are not associated to a <form> element.
  class UnassociatedListedElementsList {
    DISALLOW_NEW();

   public:
    void MarkDirty();
    const ListedElement::List& Get(const Document& owner);
    void Trace(Visitor*) const;

   private:
    ListedElement::List list_;
    // Set this flag if the stored unassociated listed elements were changed.
    bool dirty_ = false;
  };

  // Helper class to cache the top level <form> elements of a document.
  class TopLevelFormsList {
    DISALLOW_NEW();

   public:
    void MarkDirty();
    const HeapVector<Member<HTMLFormElement>>& Get(Document& owner);
    void Trace(Visitor*) const;

   private:
    HeapVector<Member<HTMLFormElement>> list_;
    bool dirty_ = false;
  };

  friend class AXContext;
  void AddAXContext(AXContext*);
  void RemoveAXContext(AXContext*);
  // Called when the AXMode of an existing AXContext changes.
  void AXContextModeChanged();
  void ClearAXObjectCache();

  bool IsDocumentFragment() const =
      delete;  // This will catch anyone doing an unnecessary check.
  bool IsDocumentNode() const =
      delete;  // This will catch anyone doing an unnecessary check.
  bool IsElementNode() const =
      delete;  // This will catch anyone doing an unnecessary check.

  bool HasPendingVisualUpdate() const {
    return lifecycle_.GetState() == DocumentLifecycle::kVisualUpdatePending;
  }

  // Calls EnsureShadowSubtree() on all Elements added via
  // ScheduleShadowTreeCreation().
  void ProcessScheduledShadowTreeCreationsNow();

  bool ShouldScheduleLayoutTreeUpdate() const;
  void ScheduleLayoutTreeUpdate();

  // See UpdateStyleAndLayoutTreeForThisDocument for an explanation of
  // the "ForThisDocument" suffix.
  //
  // These functions do not take into account dirtiness of parent frames:
  // they are assumed to be clean. If it isn't possible to guarantee
  // clean parent frames, use Needs[Full]LayoutTreeUpdate() instead.
  bool NeedsLayoutTreeUpdateForThisDocument() const {
    return CalculateStyleAndLayoutTreeUpdateForThisDocument() !=
           StyleAndLayoutTreeUpdate::kNone;
  }

  StyleAndLayoutTreeUpdate CalculateStyleAndLayoutTreeUpdateForThisDocument()
      const;
  StyleAndLayoutTreeUpdate CalculateStyleAndLayoutTreeUpdateForParentFrame()
      const;

  void UpdateUseShadowTreesIfNeeded();
  void EvaluateMediaQueryListIfNeeded();

  void UpdateStyleInvalidationIfNeeded();
  void UpdateStyle();
  bool ChildrenCanHaveStyle() const final;

  // Objects and embeds depend on "being rendered" for delaying the load event.
  // This method unblocks the load event after the first layout tree update
  // after parsing finished.
  void UnblockLoadEventAfterLayoutTreeUpdate();

  // ImplicitClose() actually does the work of closing the input stream.
  void ImplicitClose();
  bool ShouldComplete();

  // Returns |true| if both document and its owning frame are still attached.
  // Any of them could be detached during the check, e.g. by calling
  // iframe.remove() from an event handler.
  bool CheckCompletedInternal();

  void DetachParser();

  void BeginLifecycleUpdatesIfRenderingReady();

  void ChildrenChanged(const ChildrenChange&) override;

  String nodeName() const final;
  bool ChildTypeAllowed(NodeType) const final;
  Node* Clone(Document& factory,
              NodeCloningData& data,
              ContainerNode* append_to,
              ExceptionState& append_exception_state) const override;
  void CloneDataFromDocument(const Document&);

  void UpdateTitle(const String&);
  void DispatchDidReceiveTitle();
  void UpdateSelectionAfterLayout();
  void UpdateBaseURL();

  void ExecuteScriptsWaitingForResources();
  void ExecuteJavaScriptUrls();

  enum class MilestoneForDelayedAsyncScript {
    kFirstPaint,
    kFinishedParsing,
    kLcpCandidate,
  };
  void MaybeExecuteDelayedAsyncScripts(MilestoneForDelayedAsyncScript);

  void LoadEventDelayTimerFired(TimerBase*);
  void PluginLoadingTimerFired(TimerBase*);

  void AddListenerType(ListenerType listener_type) {
    listener_types_ |= listener_type;
  }
  void AddMutationEventListenerTypeIfEnabled(ListenerType);

  void ClearFocusedElementTimerFired(TimerBase*);

  bool HaveScriptBlockingStylesheetsLoaded() const;

  void SetHoverElement(Element*);

  using EventFactorySet = HashSet<std::unique_ptr<EventFactoryBase>>;
  static EventFactorySet& EventFactories();

  void SetNthIndexCache(NthIndexCache* nth_index_cache) {
    DCHECK(!nth_index_cache_ || !nth_index_cache);
    nth_index_cache_ = nth_index_cache;
  }

  void SetCheckPseudoHasCacheScope(
      CheckPseudoHasCacheScope* check_pseudo_has_cache_scope) {
    DCHECK(!check_pseudo_has_cache_scope_ || !check_pseudo_has_cache_scope);
    check_pseudo_has_cache_scope_ = check_pseudo_has_cache_scope;
  }

  // See CheckPseudoHasCacheScope constructor.
  void EnterPseudoHasChecking() {
    DCHECK(!in_pseudo_has_checking_);
    in_pseudo_has_checking_ = true;
  }
  void LeavePseudoHasChecking() { in_pseudo_has_checking_ = false; }

  void UpdateActiveState(bool is_active, bool update_active_chain, Element*);
  void UpdateHoverState(Element*);

  const AtomicString& BodyAttributeValue(const QualifiedName&) const;
  void SetBodyAttribute(const QualifiedName&, const AtomicString&);

  void SetFreezingInProgress(bool is_freezing_in_progress) {
    is_freezing_in_progress_ = is_freezing_in_progress;
  }

  void NotifyFocusedElementChanged(Element* old_focused_element,
                                   Element* new_focused_element,
                                   mojom::blink::FocusType focus_type);
  void DisplayNoneChangedForFrame();

  // Handles a connection error to |trust_token_query_answerer_| by rejecting
  // all pending promises created by |hasPrivateToken| and
  // |hasRedemptionRecord|.
  void TrustTokenQueryAnswererConnectionError();

  void RunPostPrerenderingActivationSteps();

  // Fetch the compression dictionary sent in the response header after the
  // document load completes.
  void FetchDictionaryFromLinkHeader();

  void OnWarnUnusedPreloads(Vector<KURL> unused_preloads);

  Resource* GetPendingLinkPreloadForTesting(const KURL&);

  ResizeObserver& GetLazyLoadedAutoSizedImgObserver();

  // Initiates data loading for print that is dependent on style or layout.
  // Returns true if data loading has started.
  bool InitiateStyleOrLayoutDependentLoadForPrint();

  // https://wicg.github.io/sanitizer-api/#framework
  // Common implementation for parseHTML and parseHTMLUnsafe.
  static Document* parseHTMLInternal(ExecutionContext* context,
                                     const String& html,
                                     ExceptionState& exception_state);

  // Mutable because the token is lazily-generated on demand if no token is
  // explicitly set.
  mutable std::optional<DocumentToken> token_;

  // Bitfield used for tracking UKM sampling of media features such that each
  // media feature is sampled only once per document.
  uint64_t evaluated_media_features_ = 0;

  DocumentLifecycle lifecycle_;

  bool is_initial_empty_document_;

  // Track the prerendering state.
  // TODO(crbug.com/1169032): Update the flag on the prerendering activation.
  // Also, we will merge the state into the lifecycle state eventually.
  // TODO(bokan): This should eventually be based on the document loading-mode:
  // https://github.com/jeremyroman/alternate-loading-modes/blob/main/prerendering-state.md#documentprerendering
  bool is_prerendering_;

  // Tracks whether the current document was installed as the result of a
  // discard operation.
  // TODO(crbug.com/391949533): Explore combining this with
  // `is_initial_empty_document_`.
  const bool is_for_discard_;

  // Callbacks to execute upon activation of a prerendered page, just before the
  // prerenderingchange event is dispatched.
  Vector<base::OnceClosure> will_dispatch_prerenderingchange_callbacks_;

  // The callback list for post-prerendering activation step.
  // https://wicg.github.io/nav-speculation/prerendering.html#document-post-prerendering-activation-steps-list
  Vector<base::OnceClosure> post_prerendering_activation_callbacks_;

  bool evaluate_media_queries_on_style_recalc_ = false;

  // If we do ignore the pending stylesheet count, then we need to add a boolean
  // to track that this happened so that we can do a full repaint when the
  // stylesheets do eventually load.
  PendingSheetLayout pending_sheet_layout_ = kNoLayoutWithPendingSheets;

  Member<LocalDOMWindow> dom_window_;

  // For Documents given a dom_window_ at creation that are not Shutdown(),
  // execution_context_ and dom_window_ will be equal and non-null.
  // For Documents given a dom_window_ at creation that are Shutdown(),
  // execution_context_ and dom_window_ will both be nullptr.
  // For Documents not given a dom_window_ at creation, execution_context_
  // will be the LocalDOMWindow where script will execute (which may be nullptr
  // in unit tests).
  Member<ExecutionContext> execution_context_;

  // Documents should always have an agent.
  Member<Agent> agent_;

  Member<ResourceFetcher> fetcher_;
  Member<DocumentParser> parser_;
  Member<HttpRefreshScheduler> http_refresh_scheduler_;

  bool well_formed_ = false;

  bool is_tracking_soft_navigation_heuristics_ = false;

  // Document URLs.
  KURL url_;  // Document.URL: The URL from which this document was retrieved.
  KURL base_url_;  // Node.baseURI: The URL to use when resolving relative URLs.
  KURL base_url_override_;  // An alternative base URL that takes precedence
                            // over base_url_ (but not base_element_url_).

  // Indicates whether all the conditions are met to trigger recording of counts
  // for cases where sandboxed srcdoc documents use their base url to resolve
  // relative urls.
  // Note: mutable since it needs to be reset inside a const function.
  // TODO(https://crbug.com/330744612): Remove this code once we have the data
  // around how often this happens.
  mutable bool should_record_sandboxed_srcdoc_baseurl_metrics_ = false;

  // Used in FallbackBaseURL() to provide the base URL for  about:srcdoc  and
  // about:blank documents, which is the initiator's base URL at the time the
  // navigation was initiated. Separate from the base_url_* fields because the
  // fallback base URL should not take precedence over things like <base>.
  KURL fallback_base_url_;

  KURL base_element_url_;  // The URL set by the <base> element.
  KURL cookie_url_;        // The URL to use for cookie access.

  AtomicString base_target_;

  // Mime-type of the document in case it was cloned or created by XHR.
  AtomicString mime_type_;

  Member<DocumentType> doc_type_;
  Member<DOMImplementation> implementation_;

  Member<CSSStyleSheet> elem_sheet_;

  PrintingState printing_ = kNotPrinting;
  PaintPreviewState paint_preview_ = kNotPaintingPreview;

  CompatibilityMode compatibility_mode_ = kNoQuirksMode;
  // This is cheaper than making setCompatibilityMode virtual.
  bool compatibility_mode_locked_ = false;

  TaskHandle execute_scripts_waiting_for_resources_task_handle_;
  TaskHandle javascript_url_task_handle_;
  class PendingJavascriptUrl final
      : public GarbageCollected<PendingJavascriptUrl> {
   public:
    PendingJavascriptUrl(const KURL& input_url, const DOMWrapperWorld* world);
    ~PendingJavascriptUrl();

    void Trace(Visitor* visitor) const;

    KURL url;
    // The world in which the navigation to |url| initiated. Non-null.
    Member<const DOMWrapperWorld> world;
  };
  HeapVector<Member<PendingJavascriptUrl>> pending_javascript_urls_;

  // https://html.spec.whatwg.org/C/#autofocus-processed-flag
  bool autofocus_processed_flag_ = false;
  mojom::blink::FocusType last_focus_type_;
  bool had_keyboard_event_ = false;
  HeapTaskRunnerTimer<Document> clear_focused_element_timer_;
  // https://html.spec.whatwg.org/C/#autofocus-candidates
  // We implement this as a Vector because its maximum size is typically 1.
  HeapVector<Member<Element>> autofocus_candidates_;
  Member<Element> focused_element_;
  Member<Range> sequential_focus_navigation_starting_point_;
  Member<Element> hover_element_;
  Member<Element> active_element_;
  Member<Element> document_element_;
  UserActionElementSet user_action_elements_;
  Member<RootScrollerController> root_scroller_controller_;
  Member<AnchorElementInteractionTracker> anchor_element_interaction_tracker_;

  HeapHashSet<Member<FocusedElementChangeObserver>>
      focused_element_change_observers_;

  double overscroll_accumulated_delta_x_ = 0;
  double overscroll_accumulated_delta_y_ = 0;

  uint64_t dom_tree_version_;
  static uint64_t global_tree_version_;

  uint64_t style_version_ = 0;

  HeapHashSet<WeakMember<NodeIterator>> node_iterators_;
  using AttachedRangeSet = HeapHashSet<WeakMember<Range>>;
  AttachedRangeSet ranges_;

  uint16_t listener_types_ = 0;

  // Used to record the counts of event listeners added from the nodes in the
  // document.
  uint32_t event_listener_counts_ = 0;

  MutationObserverOptions mutation_observer_types_ = 0;

  Member<ElementIntersectionObserverData>
      document_explicit_root_intersection_observer_data_;

  Member<StyleEngine> style_engine_;

  Member<FormController> form_controller_;

  TextLinkColors text_link_colors_;
  Member<VisitedLinkState> visited_link_state_;

  bool visually_ordered_ = false;

  using ElementComputedStyleMap =
      HeapHashMap<WeakMember<Element>, Member<StylePropertyMapReadOnly>>;
  ElementComputedStyleMap element_computed_style_map_;

  DocumentReadyState ready_state_;
  ParsingState parsing_state_ = kFinishedParsing;

  bool is_dns_prefetch_enabled_;
  bool have_explicitly_disabled_dns_prefetch_;

  // TODO(crbug.com/40511450): Remove once PPAPI is gone.
  bool contains_plugins_ = false;

  bool has_render_blocking_expect_link_elements_ = false;

  bool has_frame_rate_blocking_expect_link_elements_ = false;

  bool has_pending_expect_link_elements_ = false;

  // Set to true whenever shadow root is attached to document. Does not
  // get reset if all roots are removed.
  bool may_contain_shadow_roots_ = false;

  // https://html.spec.whatwg.org/C/dynamic-markup-insertion.html#ignore-destructive-writes-counter
  unsigned ignore_destructive_write_count_ = 0;
  // https://html.spec.whatwg.org/C/dynamic-markup-insertion.html#throw-on-dynamic-markup-insertion-counter
  unsigned throw_on_dynamic_markup_insertion_count_ = 0;
  // https://html.spec.whatwg.org/C/dynamic-markup-insertion.html#ignore-opens-during-unload-counter
  unsigned ignore_opens_during_unload_count_ = 0;

  bool ignore_opens_and_writes_for_abort_ = false;

  String title_;
  String raw_title_;
  Member<Element> title_element_;

  Vector<AXContext*> ax_contexts_;
  Member<AXObjectCache> ax_object_cache_;
  Member<DocumentMarkerController> markers_;

  bool should_update_selection_after_layout_ = false;

  WeakMember<Element> css_target_;
  bool css_target_is_selector_fragment_ = false;

  bool was_discarded_ = false;

  LoadEventProgress load_event_progress_ = kLoadEventCompleted;

  bool is_freezing_in_progress_ = false;

  base::ElapsedTimer start_time_;

  Member<ScriptRunner> script_runner_;
  Member<ScriptRunnerDelayer> script_runner_delayer_;

  HeapVector<Member<ScriptElementBase>> current_script_stack_;

  std::unique_ptr<TransformSource> transform_source_;

  String xml_encoding_;
  String xml_version_{"1.0"};
  unsigned xml_standalone_ : 2 = kStandaloneUnspecified;
  unsigned has_xml_declaration_ : 1 = 0;
  // See enum ViewportUnitFlags.
  unsigned viewport_unit_flags_ : kViewportUnitFlagBits = 0;

  AtomicString content_language_;

  DocumentEncodingData encoding_data_;

  bool design_mode_ = false;
  bool is_running_exec_command_ = false;

  HeapHashSet<WeakMember<const LiveNodeListBase>>
      lists_invalidated_at_document_;
  LiveNodeListRegistry node_lists_;

  Member<SVGDocumentExtensions> svg_extensions_;

  Vector<DraggableRegionValue> draggable_regions_;
  bool has_draggable_regions_ = false;
  bool draggable_regions_dirty_ = false;

  Member<SelectorQueryCache> selector_query_cache_;

  // It is safe to keep a raw, untraced pointer to this stack-allocated
  // cache object: it is set upon the cache object being allocated on
  // the stack and cleared upon leaving its allocated scope. Hence it
  // is acceptable not to trace it -- should a conservative GC occur,
  // the cache object's references will be traced by a stack walk.
  GC_PLUGIN_IGNORE("https://crbug.com/461878")
  NthIndexCache* nth_index_cache_ = nullptr;

  // This is an untraced pointer to the cache-scoped object that is first
  // allocated on the stack. It is set upon the first object being allocated
  // on the stack, and cleared upon leaving its allocated scope. The object's
  // references will be traced by a stack walk.
  GC_PLUGIN_IGNORE("https://crbug.com/669058")
  CheckPseudoHasCacheScope* check_pseudo_has_cache_scope_ = nullptr;

  bool in_pseudo_has_checking_ = false;

  DocumentClassFlags document_classes_;

  bool is_view_source_ = false;
  bool is_xr_overlay_ = false;
  bool saw_elements_in_known_namespaces_ = false;
  bool is_srcdoc_document_;
  bool is_mobile_document_ = false;

  Member<LayoutView> layout_view_;

  // The last element in |top_layer_elements_| is topmost in the top layer
  // stack and is thus the one that will be visually on top.
  HeapVector<Member<Element>> top_layer_elements_;

  // top_layer_elements_pending_removal_ is a list of elements which will be
  // removed from top_layer_elements_ when overlay computes to none. Each
  // element also has a "reason" for being in the top layer which corresponds to
  // the API which caused the element to enter the top layer in the first place.
  // TODO(http://crbug.com/1472330): This data structure is a Vector in order to
  // preserve ordering, but ideally it would be a map so that we could key into
  // it with an Element and access the TopLayerReason. However, there is no
  // ordered map oilpan data structure, so some methods that access this will be
  // O(n) instead of O(1).
  class TopLayerPendingRemoval
      : public GarbageCollected<TopLayerPendingRemoval> {
   public:
    TopLayerPendingRemoval(Element* new_element, TopLayerReason new_reason)
        : element(new_element), reason(new_reason) {}
    Member<Element> element;
    TopLayerReason reason;
    void Trace(Visitor* visitor) const { visitor->Trace(element); }
  };
  VectorOf<TopLayerPendingRemoval> top_layer_elements_pending_removal_;

  // The stack of currently-displayed popover elements that descend from a root
  // `popover=auto` element. Elements in the stack go from earliest
  // (bottom-most) to latest (top-most). Note that `popover=hint` elements can
  // exist in this stack, but there will never be a `popover=auto` that comes
  // after that in the stack.
  HeapVector<Member<HTMLElement>> popover_auto_stack_;
  // The stack of currently-displayed `popover=hint` elements. Ordering in the
  // stack is the same as for `popover_auto_stack_`. This stack will only ever
  // contain `popover=hint` elements, and nothing else.
  HeapVector<Member<HTMLElement>> popover_hint_stack_;
  // The popover (if any) that received the most recent pointerdown event.
  Member<const HTMLElement> popover_pointerdown_target_;
  // The mouse location for the mousedown that opened the select, if any.
  std::optional<gfx::PointF> customizable_select_mousedown_location_;
  // The dialog (if any) that received the most recent pointerdown event. This
  // is distinct from popover_pointerdown_target_ because the same pointer
  // action could trigger light dismiss on a containing popover and not a
  // containing dialog, or vice versa. This will be nullptr for a click on
  // the ::backdrop pseudo element for a dialog.
  Member<const HTMLDialogElement> dialog_pointerdown_target_;
  // A set of popovers for which hidePopover() has been called, but animations
  // are still running.
  HeapHashSet<Member<HTMLElement>> popovers_waiting_to_hide_;
  // A set of all open popovers, of all types.
  HeapHashSet<Member<HTMLElement>> all_open_popovers_;

  // The ordered list of currently-open dialogs, in order they were opened.
  HeapLinkedHashSet<Member<HTMLDialogElement>> all_open_dialogs_;

  // The current list of elements in the document that have interest (in the
  // `interesttarget` sense). This is used to "lose" interest if the keyboard
  // activation key (ESC) or other actions should cause a loss of interest.
  // This collection holds the *invokers* (the elements with `interesttarget`)
  // and not the *targets* of those invokers.
  HeapLinkedHashSet<Member<Element>> current_interest_target_elements_;

  Member<DocumentPartRoot> document_part_root_;

  int load_event_delay_count_ = 0;

  // Objects and embeds depend on "being rendered" for delaying the load event.
  // This is a document-wide flag saying that we have incremented the
  // load_event_delay_count_ to wait for the next layout tree update. On the
  // next layout tree update, the counter will be decremented and this flag will
  // be set to false. If any of the objects/embeds started to fetch a blocking
  // resource, they would have incremented the delay count during the layout
  // tree update and further blocked the load event.
  bool delay_load_event_until_layout_tree_update_ = false;

  HeapTaskRunnerTimer<Document> load_event_delay_timer_;
  HeapTaskRunnerTimer<Document> plugin_loading_timer_;

  DocumentTiming document_timing_;
  Member<MediaQueryMatcher> media_query_matcher_;
  bool write_recursion_is_too_deep_ = false;
  unsigned write_recursion_depth_ = 0;

  Member<ScriptedAnimationController> scripted_animation_controller_;
  Member<TextAutosizer> text_autosizer_;

  void ElementDataCacheClearTimerFired(TimerBase*);
  HeapTaskRunnerTimer<Document> element_data_cache_clear_timer_;

  Member<ElementDataCache> element_data_cache_;

  using LocaleIdentifierToLocaleMap =
      HashMap<AtomicString, std::unique_ptr<Locale>>;
  LocaleIdentifierToLocaleMap locale_cache_;

  Member<DocumentAnimations> document_animations_;
  Member<DocumentTimeline> timeline_;
  Member<PendingAnimations> pending_animations_;
  Member<WorkletAnimationController> worklet_animation_controller_;
  AnimationClock animation_clock_;

  Member<Document> template_document_;
  Member<Document> template_document_host_;

  HeapHashSet<Member<SVGUseElement>> use_elements_needing_update_;
  // SVG resources ("resource elements") for which NotifyContentChanged() needs
  // to be called to notify any clients about a change in layout attachment
  // state. Should be populated during layout detach or style recalc, and be
  // empty before and after those operations.
  HeapHashSet<Member<LocalSVGResource>> svg_resources_needing_invalidation_;

  ParserSynchronizationPolicy parser_sync_policy_ = kAllowDeferredParsing;

  Member<CanvasFontCache> canvas_font_cache_;

  Member<IntersectionObserverController> intersection_observer_controller_;

#if DCHECK_IS_ON()
  int node_count_ = 0;
#endif

  Member<PropertyRegistry> property_registry_;

  UnassociatedListedElementsList unassociated_listed_elements_;

  TopLevelFormsList top_level_forms_;

  // |ukm_recorder_| and |source_id_| will allow objects that are part of
  // the document to record UKM.
  std::unique_ptr<ukm::UkmRecorder> ukm_recorder_;
  const int64_t ukm_source_id_;

  // Tracks and reports metrics of attempted font match attempts (both
  // successful and not successful) by the page.
  std::unique_ptr<FontMatchingMetrics> font_matching_metrics_;

#if DCHECK_IS_ON()
  unsigned slot_assignment_recalc_forbidden_recursion_depth_ = 0;
#endif
  unsigned slot_assignment_recalc_depth_ = 0;
  unsigned flat_tree_traversal_forbidden_recursion_depth_ = 0;
  bool suppress_mutation_events_ = false;

  Member<DOMFeaturePolicy> policy_;

  Member<SlotAssignmentEngine> slot_assignment_engine_;

  // TODO(tkent): Should it be moved to LocalFrame or LocalFrameView?
  Member<ViewportData> viewport_data_;

  // This is set through permissions policy 'vertical-scroll'.
  bool is_vertical_scroll_enforced_ = false;

  // The number of canvas elements on the document
  unsigned num_canvases_ = 0;

  bool deferred_compositor_commit_is_allowed_ = false;

  // True when the document was created (in DomImplementation) for specific MIME
  // types that are handled externally. The document in this case is the
  // counterpart to a PluginDocument except that it contains a FrameView as
  // opposed to a PluginView.
  bool is_for_external_handler_;

  Member<LazyLoadImageObserver> lazy_load_image_observer_;

  // Tracks which document policies have already been parsed, so as not to
  // count them multiple times. The size of this vector is 0 until
  // `DocumentPolicyFeatureObserved` is called.
  Vector<bool> parsed_document_policies_;

  AtomicString override_last_modified_;

  // When the document contains MimeHandlerView, this variable might hold a
  // beforeunload handler. This will be set by the blink embedder when
  // necessary.
  Member<BeforeUnloadEventListener>
      mime_handler_view_before_unload_event_listener_;

  // Used to communicate state associated with resource management to the
  // embedder.
  std::unique_ptr<DocumentResourceCoordinator> resource_coordinator_;

  // Used for document.cookie. May be null.
  Member<CookieJar> cookie_jar_;

  // Seed for all PAAPI Auction Nonces generated for this document.
  base::Uuid base_auction_nonce_;

  bool toggle_during_parsing_ = false;

  bool is_for_markup_sanitization_ = false;

  Member<FragmentDirective> fragment_directive_;

  HeapHashMap<WeakMember<Element>, Member<CachedAttrAssociatedElementsMap>>
      element_cached_attr_associated_elements_map_;

  Member<DisplayLockDocumentState> display_lock_document_state_;

  bool in_forced_colors_mode_;

  bool applying_scroll_restoration_logic_ = false;

  // Records find-in-page metrics, which are sent to UKM on shutdown.
  bool had_find_in_page_request_ = false;
  bool had_find_in_page_render_subtree_active_match_ = false;
  bool had_find_in_page_beforematch_expanded_hidden_matchable_ = false;

  bool has_dir_attribute_ = false;

  // True if the developer supplied a media query indicating that
  // the site has support for reduced motion.
  bool supports_reduced_motion_ = false;

  // Indicate whether there is one scheduled selectionchange event.
  bool has_scheduled_selectionchange_event_on_document_ = false;

  Member<RenderBlockingResourceManager> render_blocking_resource_manager_;

  // Record if the previous UpdateStyleAndLayoutTreeForThisDocument() happened
  // while RenderingHasBegun() returned true.
  // UpdateStyleAndLayoutTreeForThisDocument() can happen while render-blocking.
  // For instance a forced update from devtools queries. If rendering_had_begun
  // is false we should not
  bool rendering_had_begun_for_last_style_update_ = false;

  bool rendering_has_begun_ = false;

  DeclarativeShadowRootAllowState declarative_shadow_root_allow_state_ =
      DeclarativeShadowRootAllowState::kNotSet;

  WeakMember<Node> find_in_page_active_match_node_;

  Member<DocumentData> data_;

  // List of meta[name=theme-color] elements cached used when getting theme
  // color.
  HeapVector<Member<HTMLMetaElement>> meta_theme_color_elements_;

  Member<ResizeObserver> intrinsic_size_observer_;

  // Watches lazy loaded auto sized img elements for resizes.
  Member<ResizeObserver> lazy_loaded_auto_sized_img_observer_;

  // Whether any resource loads that block printing are happening.
  bool loading_for_print_ = false;

  // Document owns pending preloads, prefetches and modulepreloads initiated by
  // link header so that they won't be incidentally GC-ed and cancelled.
  HeapHashSet<Member<const PendingLinkPreload>> pending_link_header_preloads_;

  // This is incremented when a module script is evaluated.
  // http://crbug.com/1079044
  unsigned ignore_destructive_write_module_script_count_ = 0;

  // Number of data-list elements in this document.
  unsigned data_list_count_ = 0;

  // Number of disabled <fieldset> elements in this document.
  unsigned disabled_fieldset_count_ = 0;

  // If legacy DOM Mutation event listeners are supported by the embedder.
  std::optional<bool> legacy_dom_mutations_supported_;

  // True if the document has scroll marker groups that need to be
  // recalculated due to e.g. a new element with scroll-marker-contain
  // property was added or removed, hence it can now be a container
  // for some html anchor scroll marker elements of other container.
  bool needs_scroll_marker_contain_relations_update_ = false;
  // True if the document has elements with scroll-marker-contain property
  // and some html anchor scroll marker elements. It is a signal to update a
  // map between scroll marker groups and scrollable areas to subscribe scroll
  // marker groups to scrollable areas changes.
  bool needs_scroll_marker_groups_map_update_ = false;
  // Every element with scroll-marker-contain property set collects
  // HTMLAnchorElements as scroll markers inside its ScrollMarkerGroupData.
  // This is the map of ScrollMarkerGroupData to all scrollers that is the
  // closest scroller to scroll marker's scroll target (e.g. scroll marker is <a
  // href="#target"> then scroll target is some element with id="target" and
  // scroller is closest ancestor scroller of scroll target).
  // It's needed to subscribe ScrollMarkerGroupData to changes in scrollers.
  HeapHashMap<Member<ScrollMarkerGroupData>,
              HeapHashSet<Member<PaintLayerScrollableArea>>>
      scroll_marker_group_to_scrollable_areas_;

  // For rendering media URLs in a top-level context that use the
  // Content-Security-Policy header to sandbox their content. This causes
  // access-controlled media to not load when it is the top-level URL when
  // third-party cookie blocking is enabled.
  bool override_site_for_cookies_for_csp_media_ = false;

  // See description in ScheduleShadowTreeCreation().
  HeapHashSet<Member<HTMLInputElement>> elements_needing_shadow_tree_;

  // See https://github.com/whatwg/dom/issues/1255 and
  // https://crbug.com/40150299. This flag is consulted via its getter, by any
  // code in the Node insertion/removal path that's interested in NOT resetting
  // certain state, when the insertion is triggered via the state-preserving
  // atomic move API (so far, `Node#moveBefore()`).
  bool state_preserving_atomic_move_in_progress_ = false;

  // See VisitedLinkPassKey class description.
  std::optional<net::SchemefulSite> cached_top_frame_site_for_visited_links_ =
      std::nullopt;

#if BUILDFLAG(IS_ANDROID)
  HeapMojoRemote<payments::facilitated::mojom::blink::PaymentLinkHandler>
      payment_link_handler_{nullptr};

  // If a payment link is handled before.
  bool payment_link_handled_ = false;
#endif

  // If you want to add new data members to blink::Document, please reconsider
  // if the members really should be in blink::Document.  document.h is a very
  // popular header, and the size of document.h affects build time
  // significantly.
  //
  // If a new data member doesn't make sense in inactive documents, such as
  // documents created by DOMImplementation/DOMParser, the member should not be
  // in blink::Document.  It should be in a per-Frame class like
  // blink::LocalDOMWindow and blink::LocalFrame.
  //
  // If you need to add new data members to blink::Document and it requires new
  // #includes, add them to blink::DocumentData instead.
};

extern template class CORE_EXTERN_TEMPLATE_EXPORT Supplement<Document>;

inline void Document::ScheduleLayoutTreeUpdateIfNeeded() {
  // Inline early out to avoid the function calls below.
  if (HasPendingVisualUpdate())
    return;
  if (ShouldScheduleLayoutTreeUpdate() && NeedsLayoutTreeUpdate())
    ScheduleLayoutTreeUpdate();
}

// This is needed to avoid ambiguous overloads with the Node and TreeScope
// versions.
DEFINE_COMPARISON_OPERATORS_WITH_REFERENCES(Document)

// Put these methods here, because they require the Document definition, but we
// really want to inline them.

inline bool Node::IsDocumentNode() const {
  return this == GetDocument();
}

Node* EventTargetNodeForDocument(Document*);

template <>
struct DowncastTraits<Document> {
  static bool AllowFrom(const Node& node) { return node.IsDocumentNode(); }
};

}  // namespace blink

#ifndef NDEBUG
// Outside the blink namespace for ease of invocation from gdb.
CORE_EXPORT void ShowLiveDocumentInstances();
#endif

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_H_
