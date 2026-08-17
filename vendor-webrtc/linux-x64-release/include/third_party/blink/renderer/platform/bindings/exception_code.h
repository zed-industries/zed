// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_EXCEPTION_CODE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_EXCEPTION_CODE_H_

namespace blink {

// DOMException uses |unsigned short| for exception codes.
// https://webidl.spec.whatwg.org/#idl-DOMException
// In our DOM implementation we use |int| instead, and use different numerical
// ranges for different types of exceptions (not limited to DOMException), so
// that an exception of any type can be expressed with a single integer.
//
// Zero value in ExceptionCode means no exception being thrown.
using ExceptionCode = int;

// DOMException's error code
// https://webidl.spec.whatwg.org/#idl-DOMException-error-names
enum class DOMExceptionCode : ExceptionCode {
  // DOMExceptions with the legacy error code.

  // Zero value is used for representing no exception.
  kNoError = 0,

  // The minimum value of the legacy error code of DOMException defined in
  // Web IDL.
  // https://webidl.spec.whatwg.org/#idl-DOMException
  kLegacyErrorCodeMin = 1,

  kIndexSizeError = 1,  // Deprecated. Use ECMAScript RangeError instead.
  // DOMStringSizeError (= 2) is deprecated and no longer supported.
  kHierarchyRequestError = 3,
  kWrongDocumentError = 4,
  kInvalidCharacterError = 5,
  // NoDataAllowedError (= 6) is deprecated and no longer supported.
  kNoModificationAllowedError = 7,
  kNotFoundError = 8,
  kNotSupportedError = 9,
  kInUseAttributeError = 10,  // Historical. Only used in setAttributeNode etc
                              // which have been removed from the DOM specs.
  kInvalidStateError = 11,
  // Web IDL 2.7.1 Error names
  // https://webidl.spec.whatwg.org/#idl-DOMException-error-names
  // Note: Don't confuse the "SyntaxError" DOMException defined here with
  // ECMAScript's SyntaxError. "SyntaxError" DOMException is used to report
  // parsing errors in web APIs, for example when parsing selectors, while
  // the ECMAScript SyntaxError is reserved for the ECMAScript parser.
  kSyntaxError = 12,
  kInvalidModificationError = 13,
  kNamespaceError = 14,
  // kInvalidAccessError is deprecated. Use ECMAScript TypeError for invalid
  // arguments, |kNotSupportedError| for unsupported operations, and
  // |kNotAllowedError| for denied requests instead.
  kInvalidAccessError = 15,  // Deprecated.
  // ValidationError (= 16) is deprecated and no longer supported.
  kTypeMismatchError = 17,  // Deprecated. Use ECMAScript TypeError instead.
  // SecurityError should be thrown with ExceptionState::ThrowSecurityError
  // with careful consideration about the message that is observable by web
  // author. Avoid using this error code unless it's really SecurityError.
  //
  // "NotAllowedError" is often a better choice because the error represetnts
  // "The request is not allowed by the user agent or the platform in the
  // current context, possibly because the user denied permission."
  // https://webidl.spec.whatwg.org/#idl-DOMException-error-names
  kSecurityError = 18,
  kNetworkError = 19,
  kAbortError = 20,
  kURLMismatchError = 21,
  kQuotaExceededError = 22,
  kTimeoutError = 23,
  kInvalidNodeTypeError = 24,
  kDataCloneError = 25,

  // The maximum value of the legacy error code of DOMException defined in
  // Web IDL.
  // https://webidl.spec.whatwg.org/#idl-DOMException
  kLegacyErrorCodeMax = 25,

  // DOMExceptions without the legacy error code.
  kEncodingError,
  kNotReadableError,
  kUnknownError,
  kConstraintError,
  kDataError,
  kTransactionInactiveError,
  kReadOnlyError,
  kVersionError,
  kOperationError,
  kNotAllowedError,
  kOptOutError,

  // The rest of entries are defined out of scope of Web IDL.

  // DOMError (obsolete, not DOMException) defined in File system (obsolete).
  // https://www.w3.org/TR/2012/WD-file-system-api-20120417/
  kPathExistsError,

  // Push API
  //
  // PermissionDeniedError (obsolete) was replaced with NotAllowedError in the
  // standard.
  // https://github.com/WICG/BackgroundSync/issues/124
  kPermissionDeniedError,

  // Serial API - https://wicg.github.io/serial
  kBreakError,
  kBufferOverrunError,
  kFramingError,
  kParityError,

  // WebTransport - https://w3c.github.io/webtransport/
  kWebTransportError,

  // Smart Card API
  // https://wicg.github.io/web-smart-card/#smartcarderror-interface
  kSmartCardError,

  // WebGPU https://www.w3.org/TR/webgpu/
  kGPUPipelineError,

  // Media Capture and Streams API
  // https://w3c.github.io/mediacapture-main/#overconstrainederror-interface
  kOverconstrainedError,

  // FedCM API
  // https://fedidcg.github.io/FedCM/#browser-api-identity-credential-error-interface
  kIdentityCredentialError,

  // WebSocketStream - https://websocket.spec.whatwg.org/
  kWebSocketError,

  kNumOfCodes,
};

inline bool IsDOMExceptionCode(ExceptionCode exception_code) {
  return static_cast<ExceptionCode>(1) <= exception_code &&
         exception_code <
             static_cast<ExceptionCode>(DOMExceptionCode::kNumOfCodes);
}

// Exception codes that correspond to ECMAScript Error objects and
// other errors derived from ECMAScript's NativeError class like
// WebAssembly errors.
// https://tc39.github.io/ecma262/#sec-error-objects
enum class ESErrorType : ExceptionCode {
  kError = 1000,    // ECMAScript Error object
  kRangeError,      // ECMAScript RangeError object
  kReferenceError,  // ECMAScript ReferenceError object
  // Note that ECMAScript SyntaxError object is different from DOMException's
  // SyntaxError. See also the comment at |DOMExceptionCode::kSyntaxError|.
  kSyntaxError,       // ECMAScript SyntaxError object
  kTypeError,         // ECMAScript TypeError object
  kWasmCompileError,  // WebAssembly.CompileError object
  kWasmLinkError,     // WebAssembly.LinkError object
  kWasmRuntimeError,  // WebAssembly.RuntimeError object
};

// Exception codes used only inside ExceptionState implementation.
enum class InternalExceptionType : ExceptionCode {
  // An exception code that is used when rethrowing a v8::Value as an
  // exception where there is no way to determine an exception code.
  kRethrownException = 2000,
};

// Upcast from DOMExceptionCode to ExceptionCode as ExceptionCode is considered
// as an union of DOMExceptionCode and ESErrorType.
// Downcast should be performed with explicit static_cast with a range check.
inline ExceptionCode ToExceptionCode(DOMExceptionCode exception_code) {
  return static_cast<ExceptionCode>(exception_code);
}

// Upcast from ESErrorType to ExceptionCode as ExceptionCode is considered
// as an union of DOMExceptionCode and ESErrorType.
// Downcast should be performed with explicit static_cast with a range check.
inline ExceptionCode ToExceptionCode(ESErrorType exception_code) {
  return static_cast<ExceptionCode>(exception_code);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_EXCEPTION_CODE_H_
