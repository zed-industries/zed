#if 0
''' '
#endif

#ifdef __cplusplus
template <typename T>
using Box = T*;
#endif

#if 0
' '''
#endif


from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef enum:
    a1 # = 0,
    a2 # = 2,
    a3,
    a4 # = 5,
  ctypedef uint64_t A;

  cdef enum:
    b1 # = 0,
    b2 # = 2,
    b3,
    b4 # = 5,
  ctypedef uint32_t B;

  cdef enum:
    c1 # = 0,
    c2 # = 2,
    c3,
    c4 # = 5,
  ctypedef uint16_t C;

  cdef enum:
    d1 # = 0,
    d2 # = 2,
    d3,
    d4 # = 5,
  ctypedef uint8_t D;

  cdef enum:
    e1 # = 0,
    e2 # = 2,
    e3,
    e4 # = 5,
  ctypedef uintptr_t E;

  cdef enum:
    f1 # = 0,
    f2 # = 2,
    f3,
    f4 # = 5,
  ctypedef intptr_t F;

  cdef enum L:
    l1,
    l2,
    l3,
    l4,

  cdef enum:
    m1 # = -1,
    m2 # = 0,
    m3 # = 1,
  ctypedef int8_t M;

  cdef enum N:
    n1,
    n2,
    n3,
    n4,

  cdef enum:
    o1,
    o2,
    o3,
    o4,
  ctypedef int8_t O;

  cdef struct J:
    pass

  cdef struct K:
    pass

  cdef struct Opaque:
    pass

  cdef enum:
    Foo,
    Bar,
    Baz,
  ctypedef uint8_t G_Tag;

  cdef struct Bar_Body:
    G_Tag tag;
    uint8_t x;
    int16_t y;

  cdef union G:
    G_Tag tag;
    int16_t foo;
    Bar_Body bar;

  cdef enum H_Tag:
    H_Foo,
    H_Bar,
    H_Baz,

  cdef struct H_Bar_Body:
    uint8_t x;
    int16_t y;

  cdef struct H:
    H_Tag tag;
    int16_t foo;
    H_Bar_Body bar;

  cdef enum:
    ExI_Foo,
    ExI_Bar,
    ExI_Baz,
  ctypedef uint8_t ExI_Tag;

  cdef struct ExI_Bar_Body:
    uint8_t x;
    int16_t y;

  cdef struct ExI:
    ExI_Tag tag;
    int16_t foo;
    ExI_Bar_Body bar;

  cdef enum:
    P0,
    P1,
  ctypedef uint8_t P_Tag;

  cdef struct P1_Body:
    uint8_t _0;
    uint8_t _1;
    uint8_t _2;

  cdef struct P:
    P_Tag tag;
    uint8_t p0;
    P1_Body p1;

  cdef enum Q_Tag:
    Ok,
    Err,

  cdef struct Q:
    Q_Tag tag;
    uint32_t *ok;
    uint32_t err;

  cdef enum R_Tag:
    IRFoo,
    IRBar,
    IRBaz,

  cdef struct IRBar_Body:
    uint8_t x;
    int16_t y;

  cdef struct R:
    R_Tag tag;
    int16_t IRFoo;
    IRBar_Body IRBar;

  void root(Opaque *opaque,
            A a,
            B b,
            C c,
            D d,
            E e,
            F f,
            G g,
            H h,
            ExI i,
            J j,
            K k,
            L l,
            M m,
            N n,
            O o,
            P p,
            Q q,
            R r);

#if 0
''' '
#endif

#include <stddef.h>
#include "testing-helpers.h"
static_assert(offsetof(CBINDGEN_STRUCT(P), tag) == 0, "unexpected offset for tag");
static_assert(offsetof(CBINDGEN_STRUCT(P), p0) == 1, "unexpected offset for p0");
static_assert(offsetof(CBINDGEN_STRUCT(P), p0) == 1, "unexpected offset for p1");
static_assert(sizeof(CBINDGEN_STRUCT(P)) == 4, "unexpected size for P");

#if 0
' '''
#endif
