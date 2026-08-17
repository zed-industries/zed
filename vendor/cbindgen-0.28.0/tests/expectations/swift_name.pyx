#define CF_SWIFT_NAME(_name) __attribute__((swift_name(#_name)))

from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct Opaque:
    pass

  ctypedef struct SelfTypeTestStruct:
    uint8_t times;

  ctypedef struct PointerToOpaque:
    Opaque *ptr;

  void rust_print_hello_world();

  void SelfTypeTestStruct_should_exist_ref(const SelfTypeTestStruct *self);

  void SelfTypeTestStruct_should_exist_ref_mut(SelfTypeTestStruct *self);

  void SelfTypeTestStruct_should_not_exist_box(SelfTypeTestStruct *self);

  SelfTypeTestStruct *SelfTypeTestStruct_should_not_exist_return_box();

  void SelfTypeTestStruct_should_exist_annotated_self(SelfTypeTestStruct self);

  void SelfTypeTestStruct_should_exist_annotated_mut_self(SelfTypeTestStruct self);

  void SelfTypeTestStruct_should_exist_annotated_by_name(SelfTypeTestStruct self);

  void SelfTypeTestStruct_should_exist_annotated_mut_by_name(SelfTypeTestStruct self);

  void SelfTypeTestStruct_should_exist_unannotated(SelfTypeTestStruct self);

  void SelfTypeTestStruct_should_exist_mut_unannotated(SelfTypeTestStruct self);

  void free_function_should_exist_ref(const SelfTypeTestStruct *test_struct);

  void free_function_should_exist_ref_mut(SelfTypeTestStruct *test_struct);

  void unnamed_argument(SelfTypeTestStruct*);

  void free_function_should_not_exist_box(SelfTypeTestStruct *boxed);

  void free_function_should_exist_annotated_by_name(SelfTypeTestStruct test_struct);

  void free_function_should_exist_annotated_mut_by_name(SelfTypeTestStruct test_struct);

  PointerToOpaque PointerToOpaque_create(uint8_t times);

  void PointerToOpaque_sayHello(PointerToOpaque self);
