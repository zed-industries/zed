// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_STACK_H
#define OPENSSL_HEADER_STACK_H

#include <openssl/base.h>   // IWYU pragma: export

#if defined(__cplusplus)
extern "C" {
#endif


// A stack, in OpenSSL, is an array of pointers. They are the most commonly
// used collection object.
//
// This file defines macros for type-safe use of the stack functions. A stack
// type is named like |STACK_OF(FOO)| and is accessed with functions named
// like |sk_FOO_*|. Note the stack will typically contain /pointers/ to |FOO|.
//
// The |DECLARE_STACK_OF| macro makes |STACK_OF(FOO)| available, and
// |DEFINE_STACK_OF| makes the corresponding functions available.


// Defining stacks.

// STACK_OF expands to the stack type for |type|.
#define STACK_OF(type) struct stack_st_##type

// DECLARE_STACK_OF declares the |STACK_OF(type)| type. It does not make the
// corresponding |sk_type_*| functions available. This macro should be used in
// files which only need the type.
#define DECLARE_STACK_OF(type) STACK_OF(type);

// DEFINE_NAMED_STACK_OF defines |STACK_OF(name)| to be a stack whose elements
// are |type| *. This macro makes the |sk_name_*| functions available.
//
// It is not necessary to use |DECLARE_STACK_OF| in files which use this macro.
#define DEFINE_NAMED_STACK_OF(name, type)                    \
  BORINGSSL_DEFINE_STACK_OF_IMPL(name, type *, const type *) \
  BORINGSSL_DEFINE_STACK_TRAITS(name, type, false)

// DEFINE_STACK_OF defines |STACK_OF(type)| to be a stack whose elements are
// |type| *. This macro makes the |sk_type_*| functions available.
//
// It is not necessary to use |DECLARE_STACK_OF| in files which use this macro.
#define DEFINE_STACK_OF(type) DEFINE_NAMED_STACK_OF(type, type)

// DEFINE_CONST_STACK_OF defines |STACK_OF(type)| to be a stack whose elements
// are const |type| *. This macro makes the |sk_type_*| functions available.
//
// It is not necessary to use |DECLARE_STACK_OF| in files which use this macro.
#define DEFINE_CONST_STACK_OF(type)                                \
  BORINGSSL_DEFINE_STACK_OF_IMPL(type, const type *, const type *) \
  BORINGSSL_DEFINE_STACK_TRAITS(type, const type, true)


// Using stacks.
//
// After the |DEFINE_STACK_OF| macro is used, the following functions are
// available.

#if 0  // Sample

// sk_SAMPLE_free_func is a callback to free an element in a stack.
typedef void (*sk_SAMPLE_free_func)(SAMPLE *);

// sk_SAMPLE_copy_func is a callback to copy an element in a stack. It should
// return the copy or NULL on error.
typedef SAMPLE *(*sk_SAMPLE_copy_func)(const SAMPLE *);

// sk_SAMPLE_cmp_func is a callback to compare |*a| to |*b|. It should return a
// value < 0, 0, or > 0 if |*a| is less than, equal to, or greater than |*b|,
// respectively.  Note the extra indirection - the function is given a pointer
// to a pointer to the element. This is the |qsort|/|bsearch| comparison
// function applied to an array of |SAMPLE*|.
typedef int (*sk_SAMPLE_cmp_func)(const SAMPLE *const *a,
                                  const SAMPLE *const *b);

// sk_SAMPLE_new creates a new, empty stack with the given comparison function,
// which may be NULL. It returns the new stack or NULL on allocation failure.
STACK_OF(SAMPLE) *sk_SAMPLE_new(sk_SAMPLE_cmp_func comp);

// sk_SAMPLE_new_null creates a new, empty stack. It returns the new stack or
// NULL on allocation failure.
STACK_OF(SAMPLE) *sk_SAMPLE_new_null(void);

// sk_SAMPLE_num returns the number of elements in |sk|. It is safe to cast this
// value to |int|. |sk| is guaranteed to have at most |INT_MAX| elements. If
// |sk| is NULL, it is treated as the empty list and this function returns zero.
size_t sk_SAMPLE_num(const STACK_OF(SAMPLE) *sk);

// sk_SAMPLE_zero resets |sk| to the empty state but does nothing to free the
// individual elements themselves.
void sk_SAMPLE_zero(STACK_OF(SAMPLE) *sk);

// sk_SAMPLE_value returns the |i|th pointer in |sk|, or NULL if |i| is out of
// range. If |sk| is NULL, it is treated as an empty list and the function
// returns NULL.
SAMPLE *sk_SAMPLE_value(const STACK_OF(SAMPLE) *sk, size_t i);

// sk_SAMPLE_set sets the |i|th pointer in |sk| to |p| and returns |p|. If |i|
// is out of range, it returns NULL.
SAMPLE *sk_SAMPLE_set(STACK_OF(SAMPLE) *sk, size_t i, SAMPLE *p);

// sk_SAMPLE_free frees |sk|, but does nothing to free the individual elements.
// Use |sk_SAMPLE_pop_free| to also free the elements.
void sk_SAMPLE_free(STACK_OF(SAMPLE) *sk);

// sk_SAMPLE_pop_free calls |free_func| on each element in |sk| and then
// frees the stack itself.
void sk_SAMPLE_pop_free(STACK_OF(SAMPLE) *sk, sk_SAMPLE_free_func free_func);

// sk_SAMPLE_insert inserts |p| into the stack at index |where|, moving existing
// elements if needed. It returns the length of the new stack, or zero on
// error.
size_t sk_SAMPLE_insert(STACK_OF(SAMPLE) *sk, SAMPLE *p, size_t where);

// sk_SAMPLE_delete removes the pointer at index |where|, moving other elements
// down if needed. It returns the removed pointer, or NULL if |where| is out of
// range.
SAMPLE *sk_SAMPLE_delete(STACK_OF(SAMPLE) *sk, size_t where);

// sk_SAMPLE_delete_ptr removes, at most, one instance of |p| from |sk| based on
// pointer equality. If an instance of |p| is found then |p| is returned,
// otherwise it returns NULL.
SAMPLE *sk_SAMPLE_delete_ptr(STACK_OF(SAMPLE) *sk, const SAMPLE *p);

// sk_SAMPLE_delete_if_func is the callback function for |sk_SAMPLE_delete_if|.
// It should return one to remove |p| and zero to keep it.
typedef int (*sk_SAMPLE_delete_if_func)(SAMPLE *p, void *data);

// sk_SAMPLE_delete_if calls |func| with each element of |sk| and removes the
// entries where |func| returned one. This function does not free or return
// removed pointers so, if |sk| owns its contents, |func| should release the
// pointers prior to returning one.
void sk_SAMPLE_delete_if(STACK_OF(SAMPLE) *sk, sk_SAMPLE_delete_if_func func,
                         void *data);

// sk_SAMPLE_find find the first value in |sk| equal to |p|. |sk|'s comparison
// function determines equality, or pointer equality if |sk| has no comparison
// function.
//
// If the stack is sorted (see |sk_SAMPLE_sort|), this function uses a binary
// search. Otherwise it performs a linear search. If it finds a matching
// element, it writes the index to |*out_index| (if |out_index| is not NULL) and
// returns one. Otherwise, it returns zero. If |sk| is NULL, it is treated as
// the empty list and the function returns zero.
//
// Note this differs from OpenSSL. The type signature is slightly different, and
// OpenSSL's version will implicitly sort |sk| if it has a comparison function
// defined.
int sk_SAMPLE_find(const STACK_OF(SAMPLE) *sk, size_t *out_index,
                   const SAMPLE *p);

// sk_SAMPLE_shift removes and returns the first element in |sk|, or NULL if
// |sk| is empty.
SAMPLE *sk_SAMPLE_shift(STACK_OF(SAMPLE) *sk);

// sk_SAMPLE_push appends |p| to |sk| and returns the length of the new stack,
// or 0 on allocation failure.
size_t sk_SAMPLE_push(STACK_OF(SAMPLE) *sk, SAMPLE *p);

// sk_SAMPLE_pop removes and returns the last element of |sk|, or NULL if |sk|
// is empty.
SAMPLE *sk_SAMPLE_pop(STACK_OF(SAMPLE) *sk);

// sk_SAMPLE_dup performs a shallow copy of a stack and returns the new stack,
// or NULL on error. Use |sk_SAMPLE_deep_copy| to also copy the elements.
STACK_OF(SAMPLE) *sk_SAMPLE_dup(const STACK_OF(SAMPLE) *sk);

// sk_SAMPLE_sort sorts the elements of |sk| into ascending order based on the
// comparison function. The stack maintains a "sorted" flag and sorting an
// already sorted stack is a no-op.
void sk_SAMPLE_sort(STACK_OF(SAMPLE) *sk);

// sk_SAMPLE_is_sorted returns one if |sk| is known to be sorted and zero
// otherwise.
int sk_SAMPLE_is_sorted(const STACK_OF(SAMPLE) *sk);

// sk_SAMPLE_set_cmp_func sets the comparison function to be used by |sk| and
// returns the previous one.
sk_SAMPLE_cmp_func sk_SAMPLE_set_cmp_func(STACK_OF(SAMPLE) *sk,
                                          sk_SAMPLE_cmp_func comp);

// sk_SAMPLE_deep_copy performs a copy of |sk| and of each of the non-NULL
// elements in |sk| by using |copy_func|. If an error occurs, it calls
// |free_func| to free any copies already made and returns NULL.
STACK_OF(SAMPLE) *sk_SAMPLE_deep_copy(const STACK_OF(SAMPLE) *sk,
                                      sk_SAMPLE_copy_func copy_func,
                                      sk_SAMPLE_free_func free_func);

#endif  // Sample


// Private functions.
//
// The |sk_*| functions generated above are implemented internally using the
// type-erased functions below. Callers should use the typed wrappers instead.
// When using the type-erased functions, callers are responsible for ensuring
// the underlying types are correct. Casting pointers to the wrong types will
// result in memory errors.

// OPENSSL_sk_free_func is a function that frees an element in a stack. Note its
// actual type is void (*)(T *) for some T. Low-level |sk_*| functions will be
// passed a type-specific wrapper to call it correctly.
typedef void (*OPENSSL_sk_free_func)(void *ptr);

// OPENSSL_sk_copy_func is a function that copies an element in a stack. Note
// its actual type is T *(*)(const T *) for some T. Low-level |sk_*| functions
// will be passed a type-specific wrapper to call it correctly.
typedef void *(*OPENSSL_sk_copy_func)(const void *ptr);

// OPENSSL_sk_cmp_func is a comparison function that returns a value < 0, 0 or >
// 0 if |*a| is less than, equal to or greater than |*b|, respectively.  Note
// the extra indirection - the function is given a pointer to a pointer to the
// element. This differs from the usual qsort/bsearch comparison function.
//
// Note its actual type is |int (*)(const T *const *a, const T *const *b)|.
// Low-level |sk_*| functions will be passed a type-specific wrapper to call it
// correctly.
typedef int (*OPENSSL_sk_cmp_func)(const void *const *a, const void *const *b);

// OPENSSL_sk_delete_if_func is the generic version of
// |sk_SAMPLE_delete_if_func|.
typedef int (*OPENSSL_sk_delete_if_func)(void *obj, void *data);

// The following function types call the above type-erased signatures with the
// true types.
typedef void (*OPENSSL_sk_call_free_func)(OPENSSL_sk_free_func, void *);
typedef void *(*OPENSSL_sk_call_copy_func)(OPENSSL_sk_copy_func, const void *);
typedef int (*OPENSSL_sk_call_cmp_func)(OPENSSL_sk_cmp_func, const void *,
                                        const void *);
typedef int (*OPENSSL_sk_call_delete_if_func)(OPENSSL_sk_delete_if_func, void *,
                                              void *);

// An OPENSSL_STACK contains an array of pointers. It is not designed to be used
// directly, rather the wrapper macros should be used.
typedef struct stack_st OPENSSL_STACK;

// The following are raw stack functions. They implement the corresponding typed
// |sk_SAMPLE_*| functions generated by |DEFINE_STACK_OF|. Callers shouldn't be
// using them. Rather, callers should use the typed functions.
OPENSSL_EXPORT OPENSSL_STACK *OPENSSL_sk_new(OPENSSL_sk_cmp_func comp);
OPENSSL_EXPORT OPENSSL_STACK *OPENSSL_sk_new_null(void);
OPENSSL_EXPORT size_t OPENSSL_sk_num(const OPENSSL_STACK *sk);
OPENSSL_EXPORT void OPENSSL_sk_zero(OPENSSL_STACK *sk);
OPENSSL_EXPORT void *OPENSSL_sk_value(const OPENSSL_STACK *sk, size_t i);
OPENSSL_EXPORT void *OPENSSL_sk_set(OPENSSL_STACK *sk, size_t i, void *p);
OPENSSL_EXPORT void OPENSSL_sk_free(OPENSSL_STACK *sk);
OPENSSL_EXPORT void OPENSSL_sk_pop_free_ex(
    OPENSSL_STACK *sk, OPENSSL_sk_call_free_func call_free_func,
    OPENSSL_sk_free_func free_func);
OPENSSL_EXPORT size_t OPENSSL_sk_insert(OPENSSL_STACK *sk, void *p,
                                        size_t where);
OPENSSL_EXPORT void *OPENSSL_sk_delete(OPENSSL_STACK *sk, size_t where);
OPENSSL_EXPORT void *OPENSSL_sk_delete_ptr(OPENSSL_STACK *sk, const void *p);
OPENSSL_EXPORT void OPENSSL_sk_delete_if(
    OPENSSL_STACK *sk, OPENSSL_sk_call_delete_if_func call_func,
    OPENSSL_sk_delete_if_func func, void *data);
OPENSSL_EXPORT int OPENSSL_sk_find(const OPENSSL_STACK *sk, size_t *out_index,
                                   const void *p,
                                   OPENSSL_sk_call_cmp_func call_cmp_func);
OPENSSL_EXPORT void *OPENSSL_sk_shift(OPENSSL_STACK *sk);
OPENSSL_EXPORT size_t OPENSSL_sk_push(OPENSSL_STACK *sk, void *p);
OPENSSL_EXPORT void *OPENSSL_sk_pop(OPENSSL_STACK *sk);
OPENSSL_EXPORT OPENSSL_STACK *OPENSSL_sk_dup(const OPENSSL_STACK *sk);
OPENSSL_EXPORT void OPENSSL_sk_sort(OPENSSL_STACK *sk,
                                    OPENSSL_sk_call_cmp_func call_cmp_func);
OPENSSL_EXPORT int OPENSSL_sk_is_sorted(const OPENSSL_STACK *sk);
OPENSSL_EXPORT OPENSSL_sk_cmp_func
OPENSSL_sk_set_cmp_func(OPENSSL_STACK *sk, OPENSSL_sk_cmp_func comp);
OPENSSL_EXPORT OPENSSL_STACK *OPENSSL_sk_deep_copy(
    const OPENSSL_STACK *sk, OPENSSL_sk_call_copy_func call_copy_func,
    OPENSSL_sk_copy_func copy_func, OPENSSL_sk_call_free_func call_free_func,
    OPENSSL_sk_free_func free_func);


// Deprecated private functions (hidden).
//
// TODO(crbug.com/boringssl/499): Migrate callers to the typed wrappers, or at
// least the new names and remove the old ones.
//
// TODO(b/290792019, b/290785937): Ideally these would at least be inline
// functions, so we do not squat the symbols.

typedef OPENSSL_STACK _STACK;

// The following functions call the corresponding |OPENSSL_sk_*| function.
OPENSSL_EXPORT OPENSSL_DEPRECATED OPENSSL_STACK *sk_new_null(void);
OPENSSL_EXPORT OPENSSL_DEPRECATED size_t sk_num(const OPENSSL_STACK *sk);
OPENSSL_EXPORT OPENSSL_DEPRECATED void *sk_value(const OPENSSL_STACK *sk,
                                                 size_t i);
OPENSSL_EXPORT OPENSSL_DEPRECATED void sk_free(OPENSSL_STACK *sk);
OPENSSL_EXPORT OPENSSL_DEPRECATED size_t sk_push(OPENSSL_STACK *sk, void *p);
OPENSSL_EXPORT OPENSSL_DEPRECATED void *sk_pop(OPENSSL_STACK *sk);

// sk_pop_free_ex calls |OPENSSL_sk_pop_free_ex|.
//
// TODO(b/291994116): Remove this.
OPENSSL_EXPORT OPENSSL_DEPRECATED void sk_pop_free_ex(
    OPENSSL_STACK *sk, OPENSSL_sk_call_free_func call_free_func,
    OPENSSL_sk_free_func free_func);

// sk_pop_free behaves like |OPENSSL_sk_pop_free_ex| but performs an invalid
// function pointer cast. It exists because some existing callers called
// |sk_pop_free| directly.
//
// TODO(davidben): Migrate callers to bssl::UniquePtr and remove this.
OPENSSL_EXPORT OPENSSL_DEPRECATED void sk_pop_free(
    OPENSSL_STACK *sk, OPENSSL_sk_free_func free_func);


#if !defined(BORINGSSL_NO_CXX)
extern "C++" {
BSSL_NAMESPACE_BEGIN
namespace internal {
template <typename T>
struct StackTraits {};
}
BSSL_NAMESPACE_END
}

#define BORINGSSL_DEFINE_STACK_TRAITS(name, type, is_const) \
  extern "C++" {                                            \
  BSSL_NAMESPACE_BEGIN                                      \
  namespace internal {                                      \
  template <>                                               \
  struct StackTraits<STACK_OF(name)> {                      \
    static constexpr bool kIsStack = true;                  \
    using Type = type;                                      \
    static constexpr bool kIsConst = is_const;              \
  };                                                        \
  }                                                         \
  BSSL_NAMESPACE_END                                        \
  }

#else
#define BORINGSSL_DEFINE_STACK_TRAITS(name, type, is_const)
#endif

#define BORINGSSL_DEFINE_STACK_OF_IMPL(name, ptrtype, constptrtype)            \
  /* We disable MSVC C4191 in this macro, which warns when pointers are cast   \
   * to the wrong type. While the cast itself is valid, it is often a bug      \
   * because calling it through the cast is UB. However, we never actually     \
   * call functions as |OPENSSL_sk_cmp_func|. The type is just a type-erased   \
   * function pointer. (C does not guarantee function pointers fit in          \
   * |void*|, and GCC will warn on this.) Thus we just disable the false       \
   * positive warning. */                                                      \
  OPENSSL_MSVC_PRAGMA(warning(push))                                           \
  OPENSSL_MSVC_PRAGMA(warning(disable : 4191))                                 \
  OPENSSL_CLANG_PRAGMA("clang diagnostic push")                                \
  OPENSSL_CLANG_PRAGMA("clang diagnostic ignored \"-Wunknown-warning-option\"") \
  OPENSSL_CLANG_PRAGMA("clang diagnostic ignored \"-Wcast-function-type-strict\"") \
                                                                               \
  DECLARE_STACK_OF(name)                                                       \
                                                                               \
  typedef void (*sk_##name##_free_func)(ptrtype);                              \
  typedef ptrtype (*sk_##name##_copy_func)(constptrtype);                      \
  typedef int (*sk_##name##_cmp_func)(constptrtype const *,                    \
                                      constptrtype const *);                   \
  typedef int (*sk_##name##_delete_if_func)(ptrtype, void *);                  \
                                                                               \
  OPENSSL_INLINE void sk_##name##_call_free_func(                              \
      OPENSSL_sk_free_func free_func, void *ptr) {                             \
    ((sk_##name##_free_func)free_func)((ptrtype)ptr);                          \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE void *sk_##name##_call_copy_func(                             \
      OPENSSL_sk_copy_func copy_func, const void *ptr) {                       \
    return (void *)((sk_##name##_copy_func)copy_func)((constptrtype)ptr);      \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE int sk_##name##_call_cmp_func(OPENSSL_sk_cmp_func cmp_func,   \
                                               const void *a, const void *b) { \
    constptrtype a_ptr = (constptrtype)a;                                      \
    constptrtype b_ptr = (constptrtype)b;                                      \
    /* |cmp_func| expects an extra layer of pointers to match qsort. */        \
    return ((sk_##name##_cmp_func)cmp_func)(&a_ptr, &b_ptr);                   \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE int sk_##name##_call_delete_if_func(                          \
      OPENSSL_sk_delete_if_func func, void *obj, void *data) {                 \
    return ((sk_##name##_delete_if_func)func)((ptrtype)obj, data);             \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE STACK_OF(name) *sk_##name##_new(sk_##name##_cmp_func comp) {  \
    return (STACK_OF(name) *)OPENSSL_sk_new((OPENSSL_sk_cmp_func)comp);        \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE STACK_OF(name) *sk_##name##_new_null(void) {                  \
    return (STACK_OF(name) *)OPENSSL_sk_new_null();                            \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE size_t sk_##name##_num(const STACK_OF(name) *sk) {            \
    return OPENSSL_sk_num((const OPENSSL_STACK *)sk);                          \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE void sk_##name##_zero(STACK_OF(name) *sk) {                   \
    OPENSSL_sk_zero((OPENSSL_STACK *)sk);                                      \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE ptrtype sk_##name##_value(const STACK_OF(name) *sk,           \
                                           size_t i) {                         \
    return (ptrtype)OPENSSL_sk_value((const OPENSSL_STACK *)sk, i);            \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE ptrtype sk_##name##_set(STACK_OF(name) *sk, size_t i,         \
                                         ptrtype p) {                          \
    return (ptrtype)OPENSSL_sk_set((OPENSSL_STACK *)sk, i, (void *)p);         \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE void sk_##name##_free(STACK_OF(name) *sk) {                   \
    OPENSSL_sk_free((OPENSSL_STACK *)sk);                                      \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE void sk_##name##_pop_free(STACK_OF(name) *sk,                 \
                                           sk_##name##_free_func free_func) {  \
    OPENSSL_sk_pop_free_ex((OPENSSL_STACK *)sk, sk_##name##_call_free_func,    \
                           (OPENSSL_sk_free_func)free_func);                   \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE size_t sk_##name##_insert(STACK_OF(name) *sk, ptrtype p,      \
                                           size_t where) {                     \
    return OPENSSL_sk_insert((OPENSSL_STACK *)sk, (void *)p, where);           \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE ptrtype sk_##name##_delete(STACK_OF(name) *sk,                \
                                            size_t where) {                    \
    return (ptrtype)OPENSSL_sk_delete((OPENSSL_STACK *)sk, where);             \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE ptrtype sk_##name##_delete_ptr(STACK_OF(name) *sk,            \
                                                constptrtype p) {              \
    return (ptrtype)OPENSSL_sk_delete_ptr((OPENSSL_STACK *)sk,                 \
                                          (const void *)p);                    \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE void sk_##name##_delete_if(                                   \
      STACK_OF(name) *sk, sk_##name##_delete_if_func func, void *data) {       \
    OPENSSL_sk_delete_if((OPENSSL_STACK *)sk, sk_##name##_call_delete_if_func, \
                         (OPENSSL_sk_delete_if_func)func, data);               \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE int sk_##name##_find(const STACK_OF(name) *sk,                \
                                      size_t *out_index, constptrtype p) {     \
    return OPENSSL_sk_find((const OPENSSL_STACK *)sk, out_index,               \
                           (const void *)p, sk_##name##_call_cmp_func);        \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE ptrtype sk_##name##_shift(STACK_OF(name) *sk) {               \
    return (ptrtype)OPENSSL_sk_shift((OPENSSL_STACK *)sk);                     \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE size_t sk_##name##_push(STACK_OF(name) *sk, ptrtype p) {      \
    return OPENSSL_sk_push((OPENSSL_STACK *)sk, (void *)p);                    \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE ptrtype sk_##name##_pop(STACK_OF(name) *sk) {                 \
    return (ptrtype)OPENSSL_sk_pop((OPENSSL_STACK *)sk);                       \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE STACK_OF(name) *sk_##name##_dup(const STACK_OF(name) *sk) {   \
    return (STACK_OF(name) *)OPENSSL_sk_dup((const OPENSSL_STACK *)sk);        \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE void sk_##name##_sort(STACK_OF(name) *sk) {                   \
    OPENSSL_sk_sort((OPENSSL_STACK *)sk, sk_##name##_call_cmp_func);           \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE int sk_##name##_is_sorted(const STACK_OF(name) *sk) {         \
    return OPENSSL_sk_is_sorted((const OPENSSL_STACK *)sk);                    \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE sk_##name##_cmp_func sk_##name##_set_cmp_func(                \
      STACK_OF(name) *sk, sk_##name##_cmp_func comp) {                         \
    return (sk_##name##_cmp_func)OPENSSL_sk_set_cmp_func(                      \
        (OPENSSL_STACK *)sk, (OPENSSL_sk_cmp_func)comp);                       \
  }                                                                            \
                                                                               \
  OPENSSL_INLINE STACK_OF(name) *sk_##name##_deep_copy(                        \
      const STACK_OF(name) *sk, sk_##name##_copy_func copy_func,               \
      sk_##name##_free_func free_func) {                                       \
    return (STACK_OF(name) *)OPENSSL_sk_deep_copy(                             \
        (const OPENSSL_STACK *)sk, sk_##name##_call_copy_func,                 \
        (OPENSSL_sk_copy_func)copy_func, sk_##name##_call_free_func,           \
        (OPENSSL_sk_free_func)free_func);                                      \
  }                                                                            \
                                                                               \
  OPENSSL_CLANG_PRAGMA("clang diagnostic pop")                                 \
  OPENSSL_MSVC_PRAGMA(warning(pop))


// Built-in stacks.

typedef char *OPENSSL_STRING;

DEFINE_STACK_OF(void)
DEFINE_NAMED_STACK_OF(OPENSSL_STRING, char)


#if defined(__cplusplus)
}  // extern C
#endif

#if !defined(BORINGSSL_NO_CXX)
extern "C++" {

#include <type_traits>

BSSL_NAMESPACE_BEGIN

namespace internal {

// Stacks defined with |DEFINE_CONST_STACK_OF| are freed with |sk_free|.
template <typename Stack>
struct DeleterImpl<Stack, std::enable_if_t<StackTraits<Stack>::kIsConst>> {
  static void Free(Stack *sk) {
    OPENSSL_sk_free(reinterpret_cast<OPENSSL_STACK *>(sk));
  }
};

// Stacks defined with |DEFINE_STACK_OF| are freed with |sk_pop_free| and the
// corresponding type's deleter.
template <typename Stack>
struct DeleterImpl<Stack, std::enable_if_t<!StackTraits<Stack>::kIsConst>> {
  static void Free(Stack *sk) {
    // sk_FOO_pop_free is defined by macros and bound by name, so we cannot
    // access it from C++ here.
    using Type = typename StackTraits<Stack>::Type;
    OPENSSL_sk_pop_free_ex(
        reinterpret_cast<OPENSSL_STACK *>(sk),
        [](OPENSSL_sk_free_func /* unused */, void *ptr) {
          DeleterImpl<Type>::Free(reinterpret_cast<Type *>(ptr));
        },
        nullptr);
  }
};

template <typename Stack>
class StackIteratorImpl {
 public:
  using Type = typename StackTraits<Stack>::Type;
  // Iterators must be default-constructable.
  StackIteratorImpl() : sk_(nullptr), idx_(0) {}
  StackIteratorImpl(const Stack *sk, size_t idx) : sk_(sk), idx_(idx) {}

  bool operator==(StackIteratorImpl other) const {
    return sk_ == other.sk_ && idx_ == other.idx_;
  }
  bool operator!=(StackIteratorImpl other) const {
    return !(*this == other);
  }

  Type *operator*() const {
    return reinterpret_cast<Type *>(
        OPENSSL_sk_value(reinterpret_cast<const OPENSSL_STACK *>(sk_), idx_));
  }

  StackIteratorImpl &operator++(/* prefix */) {
    idx_++;
    return *this;
  }

  StackIteratorImpl operator++(int /* postfix */) {
    StackIteratorImpl copy(*this);
    ++(*this);
    return copy;
  }

 private:
  const Stack *sk_;
  size_t idx_;
};

template <typename Stack>
using StackIterator =
    std::enable_if_t<StackTraits<Stack>::kIsStack, StackIteratorImpl<Stack>>;

}  // namespace internal

// PushToStack pushes |elem| to |sk|. It returns true on success and false on
// allocation failure.
template <typename Stack>
inline std::enable_if_t<!internal::StackTraits<Stack>::kIsConst, bool>
PushToStack(Stack *sk,
            UniquePtr<typename internal::StackTraits<Stack>::Type> elem) {
  if (!OPENSSL_sk_push(reinterpret_cast<OPENSSL_STACK *>(sk), elem.get())) {
    return false;
  }
  // OPENSSL_sk_push takes ownership on success.
  elem.release();
  return true;
}

BSSL_NAMESPACE_END

// Define begin() and end() for stack types so C++ range for loops work.
template <typename Stack>
inline bssl::internal::StackIterator<Stack> begin(const Stack *sk) {
  return bssl::internal::StackIterator<Stack>(sk, 0);
}

template <typename Stack>
inline bssl::internal::StackIterator<Stack> end(const Stack *sk) {
  return bssl::internal::StackIterator<Stack>(
      sk, OPENSSL_sk_num(reinterpret_cast<const OPENSSL_STACK *>(sk)));
}

}  // extern C++
#endif

#endif  // OPENSSL_HEADER_STACK_H
