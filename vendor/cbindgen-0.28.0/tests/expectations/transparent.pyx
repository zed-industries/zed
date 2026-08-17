from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct DummyStruct:
    pass

  ctypedef struct EnumWithAssociatedConstantInImpl:
    pass

  ctypedef DummyStruct TransparentComplexWrappingStructTuple;

  ctypedef uint32_t TransparentPrimitiveWrappingStructTuple;

  ctypedef DummyStruct TransparentComplexWrappingStructure;

  ctypedef uint32_t TransparentPrimitiveWrappingStructure;

  ctypedef DummyStruct TransparentComplexWrapper_i32;

  ctypedef uint32_t TransparentPrimitiveWrapper_i32;

  ctypedef uint32_t TransparentPrimitiveWithAssociatedConstants;
  const TransparentPrimitiveWithAssociatedConstants TransparentPrimitiveWithAssociatedConstants_ZERO # = 0
  const TransparentPrimitiveWithAssociatedConstants TransparentPrimitiveWithAssociatedConstants_ONE # = 1

  ctypedef struct TransparentEmptyStructure:
    pass

  const TransparentPrimitiveWrappingStructure EnumWithAssociatedConstantInImpl_TEN # = 10

  void root(TransparentComplexWrappingStructTuple a,
            TransparentPrimitiveWrappingStructTuple b,
            TransparentComplexWrappingStructure c,
            TransparentPrimitiveWrappingStructure d,
            TransparentComplexWrapper_i32 e,
            TransparentPrimitiveWrapper_i32 f,
            TransparentPrimitiveWithAssociatedConstants g,
            TransparentEmptyStructure h,
            EnumWithAssociatedConstantInImpl i);
