
/* Spew out a long sequence of the byte 251.  When fed to bzip2
   versions 1.0.0 or 1.0.1, causes it to die with internal error
   1007 in blocksort.c.  This assertion misses an extremely rare
   case, which is fixed in this version (1.0.2) and above.
*/

/* ------------------------------------------------------------------
   This file is part of bzip2/libbzip2, a program and library for
   lossless, block-sorting data compression.

   bzip2/libbzip2 version 1.0.8 of 13 July 2019
   Copyright (C) 1996-2019 Julian Seward <jseward@acm.org>

   Please read the WARNING, DISCLAIMER and PATENTS sections in the 
   README file.

   This program is released under the terms of the license contained
   in the file LICENSE.
   ------------------------------------------------------------------ */


#include <stdio.h>

int main ()
{
   int i;
   for (i = 0; i < 48500000 ; i++)
     putchar(251);
   return 0;
}
