/*
  This assembly file is a cleaned (and less ABI-compliant) version of GCC
  output of the following
  C program:

  static const int x[10] = {2, 4, 6, 8, 10, 12, 14, 16, 18, 20};
  static const int y[10] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
  static const int z = 1;

  int f(uint64_t i) {
    return x[i] + y[i];
  }

  int g(int64_t i) {
    return f(i + z);
  }
*/

#if defined(__ELF__)
.section  .rodata
  .type  Lx, %object
  .size  Lx, 40
#elif defined(__APPLE__)
.const_data
#endif
  .align  3
Lx:
  .word  2
  .word  4
  .word  6
  .word  8
  .word  10
  .word  12
  .word  14
  .word  16
  .word  18
  .word  20

#if defined(__ELF__)
  .type  Ly, %object
  .size  Ly, 40
#endif
  .align  3
Ly:
  .word  1
  .word  2
  .word  3
  .word  4
  .word  5
  .word  6
  .word  7
  .word  8
  .word  9
  .word  10

#if defined(__ELF__)
  .type  Lz, %object
  .size  Lz, 4
#endif
  .align  3
Lz:
  .word  1

.text
  .align  2
#if defined(__ELF__)
  .type  f, %function
#endif

f:
  mov x3, x0
#if defined(__ELF__)
  adrp  x10, Lx
  add  x10, x10, :lo12:Lx
#else
  adrp  x10, Lx@PAGE
  add  x10, x10, Lx@PAGEOFF
#endif
  mov x1, x3
  ldr  w1, [x10, x1, lsl 2]
#if defined(__ELF__)
  adrp  x11, Ly
  add  x11, x11, :lo12:Ly
#else
  adrp  x11, Ly@PAGE
  add  x11, x11, Ly@PAGEOFF
#endif
  mov x2, x3
  ldr  w0, [x11, x2, lsl 2]
  add  w0, w1, w0
  ret

#if defined(__ELF__)
  .type  g, %function
#endif
g:
#if defined(__ELF__)
  adrp  x10, Lz
  add  x10, x10, :lo12:Lz
#else
  adrp  x10, Lz@PAGE
  add  x10, x10, Lz@PAGEOFF
#endif
  ldr w1, [x10]
  add x0, x1, x0
  b f
