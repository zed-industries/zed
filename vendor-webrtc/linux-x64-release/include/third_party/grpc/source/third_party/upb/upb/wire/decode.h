// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// upb_decode: parsing into a upb_Message using a upb_MiniTable.

#ifndef UPB_WIRE_DECODE_H_
#define UPB_WIRE_DECODE_H_

#include <stddef.h>
#include <stdint.h>

#include "upb/mem/arena.h"
#include "upb/message/message.h"
#include "upb/mini_table/extension_registry.h"
#include "upb/mini_table/message.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

enum {
  /* If set, strings will alias the input buffer instead of copying into the
   * arena. */
  kUpb_DecodeOption_AliasString = 1,

  /* If set, the parse will return failure if any message is missing any
   * required fields when the message data ends.  The parse will still continue,
   * and the failure will only be reported at the end.
   *
   * IMPORTANT CAVEATS:
   *
   * 1. This can throw a false positive failure if an incomplete message is seen
   *    on the wire but is later completed when the sub-message occurs again.
   *    For this reason, a second pass is required to verify a failure, to be
   *    truly robust.
   *
   * 2. This can return a false success if you are decoding into a message that
   *    already has some sub-message fields present.  If the sub-message does
   *    not occur in the binary payload, we will never visit it and discover the
   *    incomplete sub-message.  For this reason, this check is only useful for
   *    implementing ParseFromString() semantics.  For MergeFromString(), a
   *    post-parse validation step will always be necessary. */
  kUpb_DecodeOption_CheckRequired = 2,

  /* EXPERIMENTAL:
   *
   * If set, the parser will allow parsing of sub-message fields that were not
   * previously linked using upb_MiniTable_SetSubMessage().  The data will be
   * parsed into an internal "empty" message type that cannot be accessed
   * directly, but can be later promoted into the true message type if the
   * sub-message fields are linked at a later time.
   *
   * Users should set this option if they intend to perform dynamic tree shaking
   * and promoting using the interfaces in message/promote.h.  If this option is
   * enabled, it is important that the resulting messages are only accessed by
   * code that is aware of promotion rules:
   *
   * 1. Message pointers in upb_Message, upb_Array, and upb_Map are represented
   *    by a tagged pointer upb_TaggedMessagePointer.  The tag indicates whether
   *    the message uses the internal "empty" type.
   *
   * 2. Any code *reading* these message pointers must test whether the "empty"
   *    tag bit is set, using the interfaces in mini_table/types.h.  However
   *    writing of message pointers should always use plain upb_Message*, since
   *    users are not allowed to create "empty" messages.
   *
   * 3. It is always safe to test whether a field is present or test the array
   *    length; these interfaces will reflect that empty messages are present,
   *    even though their data cannot be accessed without promoting first.
   *
   * 4. If a message pointer is indeed tagged as empty, the message may not be
   *    accessed directly, only promoted through the interfaces in
   *    message/promote.h.
   *
   * 5. Tagged/empty messages may never be created by the user.  They may only
   *    be created by the parser or the message-copying logic in message/copy.h.
   */
  kUpb_DecodeOption_ExperimentalAllowUnlinked = 4,

  /* EXPERIMENTAL:
   *
   * If set, decoding will enforce UTF-8 validation for string fields, even for
   * proto2 or fields with `features.utf8_validation = NONE`. Normally, only
   * proto3 string fields will be validated for UTF-8. Decoding will return
   * kUpb_DecodeStatus_BadUtf8 for non-UTF-8 strings, which is the same behavior
   * as non-UTF-8 proto3 string fields.
   */
  kUpb_DecodeOption_AlwaysValidateUtf8 = 8,
};

UPB_INLINE uint32_t upb_DecodeOptions_MaxDepth(uint16_t depth) {
  return (uint32_t)depth << 16;
}

UPB_INLINE uint16_t upb_DecodeOptions_GetMaxDepth(uint32_t options) {
  return options >> 16;
}

// Enforce an upper bound on recursion depth.
UPB_INLINE int upb_Decode_LimitDepth(uint32_t decode_options, uint32_t limit) {
  uint32_t max_depth = upb_DecodeOptions_GetMaxDepth(decode_options);
  if (max_depth > limit) max_depth = limit;
  return upb_DecodeOptions_MaxDepth(max_depth) | (decode_options & 0xffff);
}

// LINT.IfChange
typedef enum {
  kUpb_DecodeStatus_Ok = 0,
  kUpb_DecodeStatus_Malformed = 1,    // Wire format was corrupt
  kUpb_DecodeStatus_OutOfMemory = 2,  // Arena alloc failed
  kUpb_DecodeStatus_BadUtf8 = 3,      // String field had bad UTF-8
  kUpb_DecodeStatus_MaxDepthExceeded =
      4,  // Exceeded upb_DecodeOptions_MaxDepth

  // kUpb_DecodeOption_CheckRequired failed (see above), but the parse otherwise
  // succeeded.
  kUpb_DecodeStatus_MissingRequired = 5,

  // Unlinked sub-message field was present, but
  // kUpb_DecodeOptions_ExperimentalAllowUnlinked was not specified in the list
  // of options.
  kUpb_DecodeStatus_UnlinkedSubMessage = 6,
} upb_DecodeStatus;
// LINT.ThenChange(//depot/google3/third_party/protobuf/rust/upb.rs:decode_status)

UPB_API upb_DecodeStatus upb_Decode(const char* buf, size_t size,
                                    upb_Message* msg, const upb_MiniTable* mt,
                                    const upb_ExtensionRegistry* extreg,
                                    int options, upb_Arena* arena);

// Same as upb_Decode but with a varint-encoded length prepended.
// On success 'num_bytes_read' will be set to the how many bytes were read,
// on failure the contents of num_bytes_read is undefined.
UPB_API upb_DecodeStatus upb_DecodeLengthPrefixed(
    const char* buf, size_t size, upb_Message* msg, size_t* num_bytes_read,
    const upb_MiniTable* mt, const upb_ExtensionRegistry* extreg, int options,
    upb_Arena* arena);

// Utility function for wrapper languages to get an error string from a
// upb_DecodeStatus.
UPB_API const char* upb_DecodeStatus_String(upb_DecodeStatus status);
#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_WIRE_DECODE_H_ */
