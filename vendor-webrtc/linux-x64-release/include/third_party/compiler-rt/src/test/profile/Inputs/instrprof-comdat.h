// Template instantiations are placed into comdat sections. Check that
// coverage data from different instantiations are mapped back to the correct
// source regions.

template <class T> class FOO {
public:
  FOO() : t(0) {}

  T DoIt(T ti);

private:
  T t;
};

template <class T> T FOO<T>::DoIt(T ti) { // HEADER: [[@LINE]]|  2|template
  for (T I = 0; I < ti; I++) {            // HEADER: [[@LINE]]| 22|  for (T
    t += I;                               // HEADER: [[@LINE]]| 20|    t += I;
    if (I > ti / 2)                       // HEADER: [[@LINE]]| 20|    if (I > ti
      t -= 1;                             // HEADER: [[@LINE]]|  8|      t -= 1;
  }                                       // HEADER: [[@LINE]]| 10|  }
                                          // HEADER: [[@LINE]]|   |
  return t;                               // HEADER: [[@LINE]]|  1|  return t;
}
