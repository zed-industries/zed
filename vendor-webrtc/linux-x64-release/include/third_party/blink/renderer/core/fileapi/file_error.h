/*
 * Copyright (C) 2010 Google Inc.  All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_ERROR_H_

#include "base/files/file.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class DOMException;
class ExceptionState;

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
//
// LINT.IfChange(FileErrorCode)
enum class FileErrorCode {
  kOK = 0,
  kNotFoundErr = 1,
  kSecurityErr = 2,
  kAbortErr = 3,
  kNotReadableErr = 4,
  kEncodingErr = 5,
  kNoModificationAllowedErr = 6,
  kInvalidStateErr = 7,
  kSyntaxErr = 8,
  kInvalidModificationErr = 9,
  kQuotaExceededErr = 10,
  kTypeMismatchErr = 11,
  kPathExistsErr = 12,
  kMaxValue = kPathExistsErr
};
// LINT.ThenChange(//tools/metrics/histograms/metadata/storage/enums.xml:FileErrorCode)

namespace file_error {

CORE_EXPORT extern const char kAbortErrorMessage[];
CORE_EXPORT extern const char kEncodingErrorMessage[];
CORE_EXPORT extern const char kInvalidStateErrorMessage[];
CORE_EXPORT extern const char kNoModificationAllowedErrorMessage[];
CORE_EXPORT extern const char kNotFoundErrorMessage[];
CORE_EXPORT extern const char kNotReadableErrorMessage[];
CORE_EXPORT extern const char kPathExistsErrorMessage[];
CORE_EXPORT extern const char kQuotaExceededErrorMessage[];
CORE_EXPORT extern const char kSecurityErrorMessage[];
CORE_EXPORT extern const char kSyntaxErrorMessage[];
CORE_EXPORT extern const char kTypeMismatchErrorMessage[];

CORE_EXPORT void ThrowDOMException(ExceptionState&,
                                   FileErrorCode,
                                   String message = String());
CORE_EXPORT void ThrowDOMException(ExceptionState& exception_state,
                                   base::File::Error error,
                                   String message = String());
CORE_EXPORT DOMException* CreateDOMException(FileErrorCode);
CORE_EXPORT DOMException* CreateDOMException(base::File::Error);

}  // namespace file_error

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_ERROR_H_
