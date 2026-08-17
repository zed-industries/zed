#if 0
DEF PLATFORM_UNIX = 0
DEF PLATFORM_WIN = 0
DEF X11 = 0
DEF M_32 = 0
#endif


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  IF (PLATFORM_WIN or M_32):
    cdef enum:
      A,
      B,
      C,
    ctypedef uint32_t BarType;

  IF (PLATFORM_UNIX and X11):
    cdef enum:
      A,
      B,
      C,
    ctypedef uint32_t FooType;

  IF (PLATFORM_UNIX and X11):
    ctypedef struct FooHandle:
      FooType ty;
      int32_t x;
      float y;

  cdef enum:
    C1,
    C2,
    C3,
    C5,
  ctypedef uint8_t C_Tag;

  ctypedef struct C5_Body:
    C_Tag tag;
    int32_t int_;

  ctypedef union C:
    C_Tag tag;
    C5_Body c5;

  IF (PLATFORM_WIN or M_32):
    ctypedef struct BarHandle:
      BarType ty;
      int32_t x;
      float y;

  ctypedef struct ConditionalField:
    int32_t field;

  ctypedef struct Normal:
    int32_t x;
    float y;

  IF PLATFORM_WIN:
    extern int32_t global_array_with_different_sizes[2];

  IF PLATFORM_UNIX:
    extern int32_t global_array_with_different_sizes[1];

  IF (PLATFORM_UNIX and X11):
    void root(FooHandle a, C c);

  IF (PLATFORM_WIN or M_32):
    void root(BarHandle a, C c);

  void cond(ConditionalField a);

  IF PLATFORM_WIN:
    extern int32_t foo();

  IF PLATFORM_WIN:
    extern void bar(Normal a);
