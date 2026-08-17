#ifndef UTILS_H
#define UTILS_H

inline void break_optimization(void *arg) {
  __asm__ __volatile__("" : : "r" (arg) : "memory");
}

// Tests will instantiate this class to pad out bit sets to test out the
// various ways we can represent the bit set (32-bit inline, 64-bit inline,
// memory). Instantiating this class will trigger the instantiation of I
// templates with I virtual tables for classes deriving from T, I-2 of which
// will be of size sizeof(void*) * 5, 1 of which will be of size sizeof(void*)
// * 3, and 1 of which will be of size sizeof(void*) * 9. (Under the MS ABI
// each virtual table will be sizeof(void*) bytes smaller). Each category
// of virtual tables is aligned to a different power of 2, precluding the
// all-ones optimization. As a result, the bit vector for the base class will
// need to contain at least I*2 entries to accommodate all the derived virtual
// tables.
template <typename T, unsigned I>
struct Deriver : T {
  Deriver() {
    break_optimization(new Deriver<T, I-1>);
  }
  virtual void f() {}
  virtual void g() {}
  virtual void h() {}
};

template <typename T>
struct Deriver<T, 0> : T {
  virtual void f() {}
  void g() {}
};

template <typename T>
struct Deriver<T, 1> : T {
  Deriver() {
    break_optimization(new Deriver<T, 0>);
  }
  virtual void f() {}
  virtual void g() {}
  virtual void h() {}
  virtual void i() {}
  virtual void j() {}
  virtual void k() {}
  virtual void l() {}
};

// Instantiate enough classes to force CFI checks for type T to use bit
// vectors of size 32 (if B32 defined), 64 (if B64 defined) or >64 (if BM
// defined).
template <typename T>
void create_derivers() {
#ifdef B32
  break_optimization(new Deriver<T, 10>);
#endif

#ifdef B64
  break_optimization(new Deriver<T, 25>);
#endif

#ifdef BM
  break_optimization(new Deriver<T, 40>);
#endif
}

#endif
