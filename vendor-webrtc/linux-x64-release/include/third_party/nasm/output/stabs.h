/* ----------------------------------------------------------------------- *
 *
 *   Copyright 1996-2018 The NASM Authors - All Rights Reserved
 *   See the file AUTHORS included with the NASM distribution for
 *   the specific copyright holders.
 *
 *   Redistribution and use in source and binary forms, with or without
 *   modification, are permitted provided that the following
 *   conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above
 *     copyright notice, this list of conditions and the following
 *     disclaimer in the documentation and/or other materials provided
 *     with the distribution.
 *
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND
 *     CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
 *     INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *     MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 *     DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
 *     CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *     SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 *     NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 *     HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 *     CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
 *     OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
 *     EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * ----------------------------------------------------------------------- */

#ifndef STABS_H_
#define STABS_H_

#include "nctype.h"

#include "compiler.h"
#include "nasmlib.h"
#include "nasm.h"

/* offsets */
enum stab_offsets {
	STAB_strdxoff	= 0,
	STAB_typeoff	= 4,
	STAB_otheroff	= 5,
	STAB_descoff	= 6,
	STAB_valoff	= 8,
	STAB_stabsize	= 12
};

/* stab/non-stab types */
enum stab_types {
	N_UNDF		= 0x00, /* Undefined symbol */
	N_EXT		= 0x01, /* External symbol */
	N_ABS		= 0x02, /* Absolute symbol */
	N_ABS_EXT	= 0x03, /* Absolute external symbol */
	N_TEXT		= 0x04, /* Symbol in text segment */
	N_TEXT_EXT	= 0x05, /* Symbol in external text segment */
	N_DATA		= 0x06,
	N_DATA_EXT	= 0x07,
	N_BSS		= 0x08,
	N_BSS_EXT	= 0x09,
	N_INDR		= 0x0a,
	N_FN_SEQ	= 0x0c, /* N_FN from Sequent compilers */
	N_WEAKU		= 0x0d, /* Weak undefined symbol */
	N_WEAKA		= 0x0e, /* Weak absolute symbl */
	N_WEAKT		= 0x0f, /* Weak text symbol */
	N_WEAKD		= 0x10, /* Weak data symbol */
	N_WEAKB		= 0x11, /* Weak bss symbol */
	N_COMM		= 0x12, /* Common symbol */
	N_SETA		= 0x14, /* Absolute set element symbol */
	N_SETA_EXT	= 0x15,
	N_SETT		= 0x16, /* Text set element symbol */
	N_SETT_EXT	= 0x17,
	N_SETD		= 0x18, /* Data set element symbol */
	N_SETD_EXT	= 0x19,
	N_SETB		= 0x1a, /* BSS set element symbol */
	N_SETB_EXT	= 0x1b,
	N_SETV		= 0x1c, /* Pointer to set vector in data area */
	N_SETV_EXT	= 0x1d,
	N_WARNING	= 0x1e, /* Warning symbol */
	N_FN		= 0x1f, /* Filename of .o file */
	N_GSYM		= 0x20, /* Global variable */
	N_FNAME		= 0x22, /* Function name for BSD Fortran */
	N_FUN		= 0x24, /* Function name or text segment variable for C */
	N_STSYM		= 0x26, /* Data-segment variable with internal linkage */
	N_LCSYM		= 0x28, /* BSS-segment variable with internal linkage */
	N_MAIN		= 0x2a, /* Name of main routine */
	N_ROSYM		= 0x2c, /* Read-only data symbols */
	N_BNSYM		= 0x2e, /* The beginning of a relocatable function block */
	N_PC		= 0x30, /* Global symbol in Pascal */
	N_NSYMS		= 0x32, /* Number of symbols */
	N_NOMAP		= 0x34, /* No DST map for sym */
	N_OBJ		= 0x38, /* Like N_SO, but for the object file */
	N_OPT		= 0x3c, /* Options for the debugger */
	N_RSYM		= 0x40, /* Register variable */
	N_M2C		= 0x42, /* Modula-2 compilation unit */
	N_SLINE		= 0x44, /* Line number in text segment */
	N_DSLINE	= 0x46, /* Line number in data segment */
	N_BSLINE	= 0x48, /* Line number in bss segment */
	N_BROWS		= 0x48, /* Sun's source-code browser stabs */
	N_DEFD		= 0x4a, /* GNU Modula-2 definition module dependency */
	N_FLINE		= 0x4c, /* Function start/body/end line numbers */
	N_ENSYM		= 0x4e, /* This tells the end of a relocatable function */
	N_EHDECL	= 0x50, /* GNU C++ exception variable */
	N_MOD2		= 0x50, /* Modula2 info "for imc" */
	N_CATCH		= 0x54, /* GNU C++ `catch' clause */
	N_SSYM		= 0x60, /* Structure or union element */
	N_ENDM		= 0x62, /* Last stab emitted for module */
	N_SO		= 0x64, /* ID for main source file */
	N_OSO		= 0x66, /* Apple: This is the stab that associated the .o file  */
	N_ALIAS		= 0x6c, /* SunPro F77: Name of alias */
	N_LSYM		= 0x80, /* Automatic variable in the stack */
	N_BINCL		= 0x82, /* Beginning of an include file */
	N_SOL		= 0x84, /* ID for sub-source file */
	N_PSYM		= 0xa0, /* Parameter variable */
	N_EINCL		= 0xa2, /* End of an include file */
	N_ENTRY		= 0xa4, /* Alternate entry point */
	N_LBRAC		= 0xc0, /* Beginning of lexical block */
	N_EXCL		= 0xc2, /* Place holder for deleted include file */
	N_SCOPE		= 0xc4, /* Modula-2 scope information */
	N_PATCH		= 0xd0, /* Solaris2: Patch Run Time Checker */
	N_RBRAC		= 0xe0, /* End of a lexical block */
	N_BCOMM		= 0xe2, /* Begin named common block */
	N_ECOMM		= 0xe4, /* End named common block */
	N_ECOML		= 0xe8, /* Member of a common block */
	N_WITH		= 0xea, /* Solaris2: Pascal "with" statement */
	N_NBTEXT	= 0xf0,
	N_NBDATA	= 0xf2,
	N_NBBSS		= 0xf4,
	N_NBSTS		= 0xf6,
	N_NBLCS		= 0xf8,
	N_LENG		= 0xfe  /* Second symbol entry whih a length-value for the preceding entry */
};

enum stab_source_file {
	N_SO_AS		= 0x01,
	N_SO_C		= 0x02,
	N_SO_ANSI_C	= 0x03,
	N_SO_CC		= 0x04,
	N_SO_FORTRAN	= 0x05,
	N_SO_PASCAL	= 0x06,
	N_SO_FORTRAN90	= 0x07,
	N_SO_OBJC	= 0x32,
	N_SO_OBJCPLUS	= 0x33
};

#endif /* STABS_H_ */
