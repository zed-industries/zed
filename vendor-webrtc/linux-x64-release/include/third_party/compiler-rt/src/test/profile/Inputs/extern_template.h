template <typename T> struct Test {
  Test() : M(10) {}
  void doIt(int N) { // CHECK: [[@LINE]]| 2|  void doIt
    if (N > 10) {    // CHECK: [[@LINE]]| 2|    if (N > 10) {
      M += 2;        // CHECK: [[@LINE]]| 1|      M += 2;
    } else           // CHECK: [[@LINE]]| 1|    } else
      M -= 2;        // CHECK: [[@LINE]]| 1|      M -= 2;
  }
  T M;
};

#ifdef USE
extern template struct Test<int>;
#endif
#ifdef DEF
template struct Test<int>;
#endif
