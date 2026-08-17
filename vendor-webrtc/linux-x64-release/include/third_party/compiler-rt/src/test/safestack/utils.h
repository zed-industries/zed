#ifndef UTILS_H
#define UTILS_H

static inline void break_optimization(void *arg) {
  __asm__ __volatile__("" : : "r" (arg) : "memory");
}

#endif
