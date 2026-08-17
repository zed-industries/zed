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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_DOM_DEBUGGER_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_DOM_DEBUGGER_AGENT_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_event_listener_info.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/csp/content_security_policy_violation_type.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/inspector_dom_agent.h"
#include "third_party/blink/renderer/core/inspector/protocol/dom_debugger.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8-inspector.h"

namespace blink {

class Element;
class InspectorDOMAgent;
class Node;

namespace probe {
class UserCallback;
}  // namespace probe

namespace protocol {
class DictionaryValue;
}

class CORE_EXPORT InspectorDOMDebuggerAgent final
    : public InspectorBaseAgent<protocol::DOMDebugger::Metainfo>,
      public InspectorDOMAgent::DOMListener {
 public:
  static void EventListenersInfoForTarget(v8::Isolate*,
                                          v8::Local<v8::Value>,
                                          V8EventListenerInfoList* listeners);

  InspectorDOMDebuggerAgent(v8::Isolate*,
                            InspectorDOMAgent*,
                            v8_inspector::V8InspectorSession*);
  InspectorDOMDebuggerAgent(const InspectorDOMDebuggerAgent&) = delete;
  InspectorDOMDebuggerAgent& operator=(const InspectorDOMDebuggerAgent&) =
      delete;
  ~InspectorDOMDebuggerAgent() override;
  void Trace(Visitor*) const override;

  protocol::Response setBreakOnCSPViolation(
      std::unique_ptr<protocol::Array<String>> violationTypes) override;
  // DOMDebugger API for frontend
  protocol::Response setDOMBreakpoint(int node_id, const String& type) override;
  protocol::Response removeDOMBreakpoint(int node_id,
                                         const String& type) override;
  protocol::Response setEventListenerBreakpoint(
      const String& event_name,
      std::optional<String> target_name) override;
  protocol::Response removeEventListenerBreakpoint(
      const String& event_name,
      std::optional<String> target_name) override;
  protocol::Response setXHRBreakpoint(const String& url) override;
  protocol::Response removeXHRBreakpoint(const String& url) override;
  protocol::Response getEventListeners(
      const String& object_id,
      std::optional<int> depth,
      std::optional<bool> pierce,
      std::unique_ptr<protocol::Array<protocol::DOMDebugger::EventListener>>*
          listeners) override;

  // InspectorInstrumentation API
  void WillInsertDOMNode(Node* parent);
  void DidInvalidateStyleAttr(Node*);
  void DidInsertDOMNode(Node*);
  void CharacterDataModified(CharacterData*);
  void DidRemoveDOMNode(Node*);
  void WillModifyDOMAttr(Element*, const AtomicString&, const AtomicString&);
  void WillSendXMLHttpOrFetchNetworkRequest(const String& url);
  void ScriptExecutionBlockedByCSP(const String& directive_text);
  void Will(const probe::UserCallback&);
  void Did(const probe::UserCallback&);
  void OnContentSecurityPolicyViolation(
      const ContentSecurityPolicyViolationType);

  protocol::Response disable() override;
  void Restore() override;
  void DidCommitLoadForLocalFrame(LocalFrame*) override;

  static void CollectEventListeners(v8::Isolate*,
                                    EventTarget*,
                                    v8::Local<v8::Value> target_wrapper,
                                    Node* target_node,
                                    bool report_for_all_contexts,
                                    V8EventListenerInfoList* event_information);

  std::unique_ptr<protocol::Array<protocol::DOMDebugger::EventListener>>
  BuildObjectsForEventListeners(
      const V8EventListenerInfoList&,
      v8::Local<v8::Context>,
      const v8_inspector::StringView& object_group_id);

 private:
  String MatchXHRBreakpoints(const String& url) const;

  static void EventListenersInfoForTarget(
      v8::Isolate*,
      v8::Local<v8::Value>,
      int depth,
      bool pierce,
      InspectorDOMAgent::IncludeWhitespaceEnum include_whitespace,
      V8EventListenerInfoList* listeners);
  void CancelNativeBreakpoint();
  void PauseOnNativeEventIfNeeded(
      std::unique_ptr<protocol::DictionaryValue> event_data,
      bool synchronous);
  std::unique_ptr<protocol::DictionaryValue> PreparePauseOnNativeEventData(
      const String& event_name,
      const String& target_name);
  void BreakProgramOnDOMEvent(Node* target,
                              int breakpoint_type,
                              bool insertion);
  void UpdateSubtreeBreakpoints(Node*, uint32_t root_mask, bool set);
  bool HasBreakpoint(Node*, int type) const;
  // Returns value if node is in `dom_breakpoints_`, otherwise zero.
  uint32_t FindBreakpointMask(Node*) const;
  protocol::Response SetBreakpoint(const String& event_name,
                                   const String& target_name);
  protocol::Response RemoveBreakpoint(const String& event_name,
                                      const String& target_name);

  void DidAddBreakpoint();
  void DidRemoveBreakpoint();
  void SetEnabled(bool);

  // InspectorDOMAgent::DOMListener implementation
  void DidAddDocument(Document*) override;
  void WillRemoveDOMNode(Node*) override;
  void DidModifyDOMAttr(Element*) override;

  std::unique_ptr<protocol::DOMDebugger::EventListener>
  BuildObjectForEventListener(v8::Local<v8::Context>,
                              const V8EventListenerInfo&,
                              const v8_inspector::StringView& object_group_id);

  v8::Isolate* isolate_;
  Member<InspectorDOMAgent> dom_agent_;
  v8_inspector::V8InspectorSession* v8_session_;
  HeapHashMap<Member<Node>, uint32_t> dom_breakpoints_;
  InspectorAgentState::Boolean enabled_;
  InspectorAgentState::Boolean pause_on_all_xhrs_;
  InspectorAgentState::BooleanMap xhr_breakpoints_;
  InspectorAgentState::BooleanMap event_listener_breakpoints_;
  InspectorAgentState::BooleanMap csp_violation_breakpoints_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_DOM_DEBUGGER_AGENT_H_
