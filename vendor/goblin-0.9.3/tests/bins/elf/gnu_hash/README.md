# How to generate hello.so file

With 64-bit gcc:

```bash
% gcc -o hello.so helloworld.c -Wl,--as-needed -shared -fPIC
% readelf --dyn-syms hello.so

Symbol table '.dynsym' contains 13 entries:
   Num:    Value          Size Type    Bind   Vis      Ndx Name
     0: 0000000000000000     0 NOTYPE  LOCAL  DEFAULT  UND
     1: 0000000000000000     0 NOTYPE  WEAK   DEFAULT  UND _ITM_deregisterTMCloneTable
     2: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND printf@GLIBC_2.2.5 (2)
     3: 0000000000000000     0 NOTYPE  WEAK   DEFAULT  UND __gmon_start__
     4: 0000000000000000     0 NOTYPE  WEAK   DEFAULT  UND _ITM_registerTMCloneTable
     5: 0000000000000000     0 FUNC    WEAK   DEFAULT  UND __cxa_finalize@GLIBC_2.2.5 (2)
     6: 0000000000201030     0 NOTYPE  GLOBAL DEFAULT   22 _edata
     7: 000000000000065a    33 FUNC    GLOBAL DEFAULT   12 helloWorld
     8: 0000000000201038     0 NOTYPE  GLOBAL DEFAULT   23 _end
     9: 0000000000201030     0 NOTYPE  GLOBAL DEFAULT   23 __bss_start
    10: 000000000000067b    43 FUNC    GLOBAL DEFAULT   12 main
    11: 0000000000000520     0 FUNC    GLOBAL DEFAULT    9 _init
    12: 00000000000006a8     0 FUNC    GLOBAL DEFAULT   13 _fini

% readelf --section-headers hello.so
There are 26 section headers, starting at offset 0x1140:

Section Headers:
  [Nr] Name              Type             Address           Offset
       Size              EntSize          Flags  Link  Info  Align
  [ 0]                   NULL             0000000000000000  00000000
       0000000000000000  0000000000000000           0     0     0
  [ 1] .note.gnu.build-i NOTE             00000000000001c8  000001c8
       0000000000000024  0000000000000000   A       0     0     4
  [ 2] .gnu.hash         GNU_HASH         00000000000001f0  000001f0
       0000000000000040  0000000000000000   A       3     0     8
  [ 3] .dynsym           DYNSYM           0000000000000230  00000230
       0000000000000138  0000000000000018   A       4     1     8
  [ 4] .dynstr           STRTAB           0000000000000368  00000368
       00000000000000a6  0000000000000000   A       0     0     1
  [ 5] .gnu.version      VERSYM           000000000000040e  0000040e
       000000000000001a  0000000000000002   A       3     0     2
  [ 6] .gnu.version_r    VERNEED          0000000000000428  00000428
       0000000000000020  0000000000000000   A       4     1     8
  [ 7] .rela.dyn         RELA             0000000000000448  00000448
       00000000000000a8  0000000000000018   A       3     0     8
  [ 8] .rela.plt         RELA             00000000000004f0  000004f0
       0000000000000030  0000000000000018  AI       3    21     8
  [ 9] .init             PROGBITS         0000000000000520  00000520
       0000000000000017  0000000000000000  AX       0     0     4
  [10] .plt              PROGBITS         0000000000000540  00000540
       0000000000000030  0000000000000010  AX       0     0     16
  [11] .plt.got          PROGBITS         0000000000000570  00000570
       0000000000000008  0000000000000008  AX       0     0     8
  [12] .text             PROGBITS         0000000000000580  00000580
       0000000000000126  0000000000000000  AX       0     0     16
  [13] .fini             PROGBITS         00000000000006a8  000006a8
       0000000000000009  0000000000000000  AX       0     0     4
  [14] .rodata           PROGBITS         00000000000006b1  000006b1
       0000000000000010  0000000000000000   A       0     0     1
  [15] .eh_frame_hdr     PROGBITS         00000000000006c4  000006c4
       000000000000002c  0000000000000000   A       0     0     4
  [16] .eh_frame         PROGBITS         00000000000006f0  000006f0
       000000000000009c  0000000000000000   A       0     0     8
  [17] .init_array       INIT_ARRAY       0000000000200e10  00000e10
       0000000000000008  0000000000000008  WA       0     0     8
  [18] .fini_array       FINI_ARRAY       0000000000200e18  00000e18
       0000000000000008  0000000000000008  WA       0     0     8
  [19] .dynamic          DYNAMIC          0000000000200e20  00000e20
       00000000000001c0  0000000000000010  WA       4     0     8
  [20] .got              PROGBITS         0000000000200fe0  00000fe0
       0000000000000020  0000000000000008  WA       0     0     8
  [21] .got.plt          PROGBITS         0000000000201000  00001000
       0000000000000028  0000000000000008  WA       0     0     8
  [22] .data             PROGBITS         0000000000201028  00001028
       0000000000000008  0000000000000000  WA       0     0     8
  [23] .bss              NOBITS           0000000000201030  00001030
       0000000000000008  0000000000000000  WA       0     0     1
  [24] .comment          PROGBITS         0000000000000000  00001030
       000000000000002a  0000000000000001  MS       0     0     1
  [25] .shstrtab         STRTAB           0000000000000000  0000105a
       00000000000000e1  0000000000000000           0     0     1
Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), I (info),
  L (link order), O (extra OS processing required), G (group), T (TLS),
  C (compressed), x (unknown), o (OS specific), E (exclude),
  l (large), p (processor specific)
```

Or in 32-bit mode (one might need to install `gcc-multilib` on Ubuntu):

```bash
% gcc -o hello.so helloworld.c -Wl,--as-needed -shared -fPIC
% readelf --dyn-syms hello32.so

Symbol table '.dynsym' contains 13 entries:
   Num:    Value  Size Type    Bind   Vis      Ndx Name
     0: 00000000     0 NOTYPE  LOCAL  DEFAULT  UND
     1: 00000000     0 NOTYPE  WEAK   DEFAULT  UND _ITM_deregisterTMCloneTable
     2: 00000000     0 FUNC    GLOBAL DEFAULT  UND printf@GLIBC_2.0 (2)
     3: 00000000     0 FUNC    WEAK   DEFAULT  UND __cxa_finalize@GLIBC_2.1.3 (3)
     4: 00000000     0 NOTYPE  WEAK   DEFAULT  UND __gmon_start__
     5: 00000000     0 NOTYPE  WEAK   DEFAULT  UND _ITM_registerTMCloneTable
     6: 00002018     0 NOTYPE  GLOBAL DEFAULT   22 _edata
     7: 000004ed    49 FUNC    GLOBAL DEFAULT   12 helloWorld
     8: 0000201c     0 NOTYPE  GLOBAL DEFAULT   23 _end
     9: 00002018     0 NOTYPE  GLOBAL DEFAULT   23 __bss_start
    10: 0000051e    66 FUNC    GLOBAL DEFAULT   12 main
    11: 0000038c     0 FUNC    GLOBAL DEFAULT    9 _init
    12: 00000564     0 FUNC    GLOBAL DEFAULT   13 _fini

```
