// Prevents the compiler from optimizing everything away.
template <class T> void DoNotOptimize(const T &var) {
  asm volatile("" : "+m"(const_cast<T &>(var)));
}
