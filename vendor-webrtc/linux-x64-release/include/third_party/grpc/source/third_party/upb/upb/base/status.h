// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_BASE_STATUS_H_
#define UPB_BASE_STATUS_H_

#include <stdarg.h>

// Must be last.
#include "upb/port/def.inc"

#define _kUpb_Status_MaxMessage 511

typedef struct {
  bool ok;
  char msg[_kUpb_Status_MaxMessage];  // Error message; NULL-terminated.
} upb_Status;

#ifdef __cplusplus
extern "C" {
#endif

UPB_API const char* upb_Status_ErrorMessage(const upb_Status* status);
UPB_API bool upb_Status_IsOk(const upb_Status* status);

// These are no-op if |status| is NULL.
UPB_API void upb_Status_Clear(upb_Status* status);
void upb_Status_SetErrorMessage(upb_Status* status, const char* msg);
void upb_Status_SetErrorFormat(upb_Status* status, const char* fmt, ...)
    UPB_PRINTF(2, 3);
void upb_Status_VSetErrorFormat(upb_Status* status, const char* fmt,
                                va_list args) UPB_PRINTF(2, 0);
void upb_Status_VAppendErrorFormat(upb_Status* status, const char* fmt,
                                   va_list args) UPB_PRINTF(2, 0);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_BASE_STATUS_H_ */
