#define MUST_USE_FUNC __attribute__((warn_unused_result))
#define MUST_USE_STRUCT __attribute__((warn_unused))
#define MUST_USE_ENUM /* nothing */


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef enum:
    Owned_i32,
    None_i32,
  ctypedef uint8_t MaybeOwnedPtr_i32_Tag;

  ctypedef struct MaybeOwnedPtr_i32:
    MaybeOwnedPtr_i32_Tag tag;
    int32_t *owned;

  ctypedef struct OwnedPtr_i32:
    int32_t *ptr;

  MaybeOwnedPtr_i32 maybe_consume(OwnedPtr_i32 input);
