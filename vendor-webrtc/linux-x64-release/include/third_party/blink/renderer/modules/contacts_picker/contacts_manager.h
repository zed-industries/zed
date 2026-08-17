// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CONTACTS_PICKER_CONTACTS_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CONTACTS_PICKER_CONTACTS_MANAGER_H_

#include "third_party/blink/public/mojom/contacts/contacts_manager.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_contact_property.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_contacts_select_options.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {
class ContactInfo;
class ExceptionState;
class Navigator;
class ScriptState;
class V8ContactProperty;

// Represents an the ContactManager, providing access to Contacts.
class ContactsManager final : public ScriptWrappable,
                              public Supplement<Navigator> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];
  // Web Exposed as navigator.contacts
  static ContactsManager* contacts(Navigator& navigator);

  explicit ContactsManager(Navigator& navigator);
  ~ContactsManager() override;

  // Web-exposed function defined in the IDL file.
  ScriptPromise<IDLSequence<ContactInfo>> select(
      ScriptState* script_state,
      const Vector<V8ContactProperty>& properties,
      ContactsSelectOptions* options,
      ExceptionState& exception_state);
  ScriptPromise<IDLSequence<V8ContactProperty>> getProperties(
      ScriptState* script_state);

  void Trace(Visitor*) const override;

 private:
  mojom::blink::ContactsManager* GetContactsManager(ScriptState* script_state);

  void OnContactsSelected(
      ScriptPromiseResolver<IDLSequence<ContactInfo>>* resolver,
      std::optional<Vector<mojom::blink::ContactInfoPtr>> contacts);

  const Vector<V8ContactProperty>& GetProperties(ScriptState* script_state);

  // Created lazily.
  HeapMojoRemote<mojom::blink::ContactsManager> contacts_manager_;
  bool contact_picker_in_use_ = false;
  Vector<V8ContactProperty> properties_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CONTACTS_PICKER_CONTACTS_MANAGER_H_
