from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef const char *Str;

  cdef struct HashTable_Str__c_char__false:
    uintptr_t num_buckets;
    uintptr_t capacity;
    uint8_t *occupied;
    Str *keys;
    char *vals;

  ctypedef HashTable_Str__c_char__false MySet;

  ctypedef void (*SetCallback)(Str key);

  cdef struct HashTable_Str__u64__true:
    uintptr_t num_buckets;
    uintptr_t capacity;
    uint8_t *occupied;
    Str *keys;
    uint64_t *vals;

  ctypedef void (*MapCallback)(Str key, uint64_t val);

  MySet *new_set();

  void set_for_each(const MySet *set, SetCallback callback);

  HashTable_Str__u64__true *new_map();

  void map_for_each(const HashTable_Str__u64__true *map, MapCallback callback);
