/*
 * Copyright (C) 2008 Nikolas Zimmermann <zimmermann@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_SCRIPT_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_SCRIPT_LOADER_H_

#include "third_party/blink/public/mojom/script/script_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/container_node.h"
#include "third_party/blink/renderer/core/html/cross_origin_attribute.h"
#include "third_party/blink/renderer/core/script/pending_script.h"
#include "third_party/blink/renderer/core/script/script_scheduling_type.h"
#include "third_party/blink/renderer/core/speculation_rules/speculation_rule_set.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_finish_observer.h"
#include "third_party/blink/renderer/platform/loader/fetch/script_fetch_options.h"
#include "third_party/blink/renderer/platform/wtf/text/text_encoding.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Resource;
class ResourceFetcher;
class ScriptElementBase;
class Script;
class ScriptResource;
class ScriptWebBundle;
class Modulator;

class CORE_EXPORT ScriptLoader final : public ResourceFinishObserver,
                                       public NameClient {
 public:
  ScriptLoader(ScriptElementBase*, const CreateElementFlags);
  ~ScriptLoader() override;
  void Trace(Visitor*) const override;
  const char* NameInHeapSnapshot() const override { return "ScriptLoader"; }
  String DebugName() const override { return "ScriptLoader"; }

  // Script type at the time of #prepare-the-script-element. Import maps are
  // included here but not in `mojom::blink::ScriptType` because import maps are
  // handled differently from ordinal scripts after PrepareScript().
  enum class ScriptTypeAtPrepare : uint8_t {
    kClassic,
    kModule,
    kImportMap,
    kSpeculationRules,
    kWebBundle,
    kInvalid
  };

  static ScriptTypeAtPrepare GetScriptTypeAtPrepare(
      const String& type_attribute_value,
      const String& language_attribute_value);

  static bool BlockForNoModule(ScriptTypeAtPrepare, bool nomodule);

  static network::mojom::CredentialsMode ModuleScriptCredentialsMode(
      CrossOriginAttributeValue);

  // Indicate whether the caller of `PrepareScript()` allows
  // `kParserBlockingInline` scripts. This matches with the spec:
  //
  // <spec href="https://html.spec.whatwg.org/C/#prepare-the-script-element"
  // step="32.2">... either the parser that created el is an XML parser or it's
  // an HTML parser whose script nesting level is not greater than one,
  // ...</spec>
  enum ParserBlockingInlineOption { kDeny, kAllow };

  // https://html.spec.whatwg.org/C/#prepare-the-script-element
  // Returns a `PendingScript` when the script is to be managed by the parser
  // and the parser should perform some remaining steps of
  // `#prepare-the-script-element`. Otherwise returns a `nullptr`, i.e.
  // - when called from non-parser call sites,
  // - when no scripts are to be evaluated, or
  // - when the script is handled by ScriptLoader/ScriptRunner, not by parsers.
  [[nodiscard]] PendingScript* PrepareScript(
      ParserBlockingInlineOption,
      const TextPosition& script_start_position);

  bool IsParserInserted() const { return parser_inserted_; }
  bool AlreadyStarted() const { return already_started_; }
  bool IsForceAsync() const { return force_async_; }
  ScriptTypeAtPrepare GetScriptType() const { return script_type_; }

  // Helper functions used by our parent classes.
  void DidNotifySubtreeInsertionsToDocument();
  void ChildrenChanged(const ContainerNode::ChildrenChange&);
  void HandleSourceAttribute(const String& source_url);
  void HandleAsyncAttribute();
  void Removed();
  void DocumentBaseURLChanged();

 private:
  // ResourceFinishObserver. This should be used only for managing
  // `resource_keep_alive_` lifetime and shouldn't be used for script
  // evaluation.
  void NotifyFinished() override;

  // Helpers that implement parts of `PrepareScript()` that should be called
  // only from `PrepareScript()`.
  bool IsScriptForEventSupported() const;
  PendingScript* TakePendingScript(ScriptSchedulingType);
  // Get the effective script text (after Trusted Types checking).
  String GetScriptText() const;
  void FetchModuleScriptTree(const KURL&,
                             ResourceFetcher*,
                             Modulator*,
                             const ScriptFetchOptions&);
  // Calculate ScriptSchedulingType per spec (#prepare-the-script-element Steps
  // 31-32), before any intervention applied.
  ScriptSchedulingType GetScriptSchedulingTypePerSpec(
      Document& element_document,
      ParserBlockingInlineOption) const;
  // Methods to add/remove a SpeculationRuleSet in DocumentSpeculationRules.
  // Only used for <script type="speculationrules">.
  void AddSpeculationRuleSet(SpeculationRuleSet::Source* source);
  SpeculationRuleSet* RemoveSpeculationRuleSet();

  Member<ScriptElementBase> element_;

  // <spec href="https://html.spec.whatwg.org/C/#parser-document">... initially
  // null. It is set by the HTML parser and the XML parser on script elements
  // they insert, ...</spec>
  //
  // We use a WeakMember here because we're keeping the parser-inserted
  // information separately from the parser document, so ScriptLoader doesn't
  // need to keep the parser document alive.
  WeakMember<Document> parser_document_;

  // A PendingScript is first created in PrepareScript() and stored in
  // |prepared_pending_script_|.
  // Later, TakePendingScript() is called, and its caller holds a reference
  // to the PendingScript instead and |prepared_pending_script_| is cleared.
  Member<PendingScript> prepared_pending_script_;

  // This is used only to keep the ScriptResource of a classic script alive
  // and thus to keep it on MemoryCache, even after script execution, as long
  // as ScriptLoader is alive. crbug.com/778799
  Member<Resource> resource_keep_alive_;

  // This is created only for <script type=webbundle>, representing a webbundle
  // mapping rule and its loader.
  Member<ScriptWebBundle> script_web_bundle_;

  // Speculation rule set registered by this script, if applicable.
  Member<SpeculationRuleSet> speculation_rule_set_;

  // https://html.spec.whatwg.org/C/#script-processing-model
  // "A script element has several associated pieces of state.":

  // <spec href="https://html.spec.whatwg.org/C/#already-started">... initially
  // false.</spec>
  bool already_started_ = false;

  // <spec href="https://html.spec.whatwg.org/C/#parser-inserted">... script
  // elements with non-null parser documents are known as
  // parser-inserted.</spec>
  //
  // Note that we don't actually implement "parser inserted" in terms of a
  // non-null |parser_document_| like the spec, because it is possible for
  // |CreateElementFlags::created_by_parser_| to be true even when
  // |CreateElementFlags::parser_document_| is null. Therefore, we have to
  // store this information separately.
  bool parser_inserted_ = false;

  // <spec href="https://html.spec.whatwg.org/C/#script-force-async">...
  // initially true. ...</spec>
  bool force_async_ = true;

  // <spec href="https://html.spec.whatwg.org/C/#concept-script-type">...
  // initially null. It is determined when the element is prepared, based on the
  // type attribute of the element at that time.</spec>
  ScriptTypeAtPrepare script_type_ = ScriptTypeAtPrepare::kInvalid;

  // <spec href="https://html.spec.whatwg.org/C/#concept-script-external">
  // ... initially false. It is determined when the script is prepared, based on
  // the src attribute of the element at that time.</spec>
  bool is_external_script_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_SCRIPT_LOADER_H_
