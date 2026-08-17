// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TRUSTEDTYPES_TRUSTED_TYPES_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TRUSTEDTYPES_TRUSTED_TYPES_UTIL_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/script/script_element_base.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class ExecutionContext;
class QualifiedName;
class ScriptValue;
class ScriptState;
class V8UnionStringOrTrustedScript;
class V8UnionStringLegacyNullToEmptyStringOrTrustedScript;

enum class SpecificTrustedType {
  kNone,
  kHTML,
  kScript,
  kScriptURL,
};

// Perform Trusted Type checks, with the IDL union types as input. All of these
// will call String& versions below to do the heavy lifting.
[[nodiscard]] CORE_EXPORT String
TrustedTypesCheckFor(SpecificTrustedType type,
                     const V8TrustedType* trusted,
                     const ExecutionContext* execution_context,
                     const char* interface_name,
                     const char* property_name,
                     ExceptionState& exception_state);
[[nodiscard]] CORE_EXPORT String
TrustedTypesCheckForScript(const V8UnionStringOrTrustedScript* value,
                           const ExecutionContext* execution_context,
                           const char* interface_name,
                           const char* property_name,
                           ExceptionState& exception_state);
[[nodiscard]] CORE_EXPORT String TrustedTypesCheckForScript(
    const V8UnionStringLegacyNullToEmptyStringOrTrustedScript* value,
    const ExecutionContext* execution_context,
    const char* interface_name,
    const char* property_name,
    ExceptionState& exception_state);

// Perform Trusted Type checks, for a dynamically or statically determined
// type.
// Returns the effective value (which may have been modified by the "default"
// policy.
[[nodiscard]] String TrustedTypesCheckFor(SpecificTrustedType,
                                          String,
                                          const ExecutionContext*,
                                          const char* interface_name,
                                          const char* property_name,
                                          ExceptionState&);
[[nodiscard]] CORE_EXPORT String
TrustedTypesCheckForHTML(const String&,
                         const ExecutionContext*,
                         const char* interface_name,
                         const char* property_name,
                         ExceptionState&);
[[nodiscard]] CORE_EXPORT String
TrustedTypesCheckForScript(const String&,
                           const ExecutionContext*,
                           const char* interface_name,
                           const char* property_name,
                           ExceptionState&);
[[nodiscard]] CORE_EXPORT String
TrustedTypesCheckForScriptURL(const String&,
                              const ExecutionContext*,
                              const char* interface_name,
                              const char* property_name,
                              ExceptionState&);

// Functionally equivalent to TrustedTypesCheckForScript(const String&, ...),
// but with setup & error handling suitable for the asynchronous execution
// cases.
String TrustedTypesCheckForJavascriptURLinNavigation(const String&,
                                                     ExecutionContext*);
CORE_EXPORT String GetStringForScriptExecution(const String&,
                                               ScriptElementBase::Type,
                                               ExecutionContext*);

// Functionally equivalent to TrustedTypesCheckForHTML(const String&, ...),
// but with separate enable flag and use counter, to ensure this won't break
// existing sites before enabling it in full.
[[nodiscard]] CORE_EXPORT String
TrustedTypesCheckForExecCommand(const String&,
                                const ExecutionContext*,
                                ExceptionState&);

// Determine whether a Trusted Types check is needed in this execution context.
//
// Note: All methods above handle this internally and will return success if a
// check is not required. However, in cases where not-required doesn't
// immediately imply "okay" this method can be used.
// Example: To determine whether 'eval' may pass, one needs to also take CSP
// into account.
CORE_EXPORT bool RequireTrustedTypesCheck(const ExecutionContext*);

// Determine whether an attribute is considered an event handler by Trusted
// Types.
//
// Note: This is different from Element::IsEventHandlerAttribute, because
// Element only needs this distinction for built-in attributes, but not for
// user-defined property names. But Trusted Types needs this for any built-in or
// user-defined attribute/property, and thus must check against a list of known
// event handlers.
bool IsTrustedTypesEventHandlerAttribute(const QualifiedName&);

// Return a string, if the passed-in script value is a literal. With "literal"
// meaning it passes the checks for TrustedType's fromLiteral definition.
//
// If an error occurs, this will return a null-String.
//
// Spec:
// https://w3c.github.io/trusted-types/dist/spec/#check-templatedness-algorithm
String GetTrustedTypesLiteral(const ScriptValue&, ScriptState*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TRUSTEDTYPES_TRUSTED_TYPES_UTIL_H_
