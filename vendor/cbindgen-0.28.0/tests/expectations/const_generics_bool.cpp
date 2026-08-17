#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T = void>
struct MaybeUninit;

using Str = const char*;

template<typename K, typename V, bool IS_MAP>
struct HashTable {
  uintptr_t num_buckets;
  uintptr_t capacity;
  uint8_t *occupied;
  MaybeUninit<K> *keys;
  MaybeUninit<V> *vals;
};

using MySet = HashTable<Str, char, false>;

using SetCallback = void(*)(Str key);

using MapCallback = void(*)(Str key, uint64_t val);

extern "C" {

MySet *new_set();

void set_for_each(const MySet *set, SetCallback callback);

HashTable<Str, uint64_t, true> *new_map();

void map_for_each(const HashTable<Str, uint64_t, true> *map, MapCallback callback);

}  // extern "C"
