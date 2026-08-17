#if __LP64__
# define SANITIZER_WORDSIZE 64
#else
# define SANITIZER_WORDSIZE 32
#endif

// This is a simplified version of GetMaxVirtualAddress function.
unsigned long SystemVMA () {
#if SANITIZER_WORDSIZE == 64
  unsigned long vma = (unsigned long)__builtin_frame_address(0);
  return SANITIZER_WORDSIZE - __builtin_clzll(vma);
#else
  return SANITIZER_WORDSIZE;
#endif
}
