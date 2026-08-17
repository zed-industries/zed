#if 0
''' '
#endif
#if defined(CBINDGEN_STYLE_TYPE)
/* ANONYMOUS STRUCTS DO NOT SUPPORT FORWARD DECLARATIONS!
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

  cdef struct StructInfo:
    const TypeInfo *const *fields;
    uintptr_t num_fields;

  cdef enum TypeData_Tag:
    Primitive,
    Struct,

  cdef struct TypeData:
    TypeData_Tag tag;
    StructInfo struct_;

  cdef struct TypeInfo:
    TypeData data;

  void root(TypeInfo x);

#if 0
''' '
#endif
#if defined(CBINDGEN_STYLE_TYPE)
*/
#endif
#if 0
' '''
#endif
