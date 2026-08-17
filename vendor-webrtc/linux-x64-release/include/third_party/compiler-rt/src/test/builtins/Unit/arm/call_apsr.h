#ifndef CALL_APSR_H
#define CALL_APSR_H

#if __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__

union cpsr {
  struct {
    uint32_t filler : 28;
    uint32_t v : 1;
    uint32_t c : 1;
    uint32_t z : 1;
    uint32_t n : 1;
  } flags;
  uint32_t value;
};

#else

union cpsr {
  struct {
    uint32_t n : 1;
    uint32_t z : 1;
    uint32_t c : 1;
    uint32_t v : 1;
    uint32_t filler : 28;
  } flags;
  uint32_t value;
};

#endif

__attribute__((noinline, pcs("aapcs"))) static uint32_t call_apsr_f(float a, float b,
                                                                    __attribute__((pcs("aapcs"))) void (*fn)(float, float)) {
  uint32_t result;
  fn(a, b);
  asm volatile("mrs %0, apsr"
               : "=r"(result));
  return result;
}

__attribute__((noinline, pcs("aapcs"))) static uint32_t call_apsr_d(double a, double b,
                                                                    __attribute__((pcs("aapcs"))) void (*fn)(double, double)) {
  uint32_t result;
  fn(a, b);
  asm volatile("mrs %0, apsr"
               : "=r"(result));
  return result;
}

#endif // CALL_APSR_H
