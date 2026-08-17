/*-
 * Copyright 2003,2004 Colin Percival
 * All rights reserved
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted providing that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
 * IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 *
 * Changelog:
 * 2005-04-26 - Define the header as a C structure, add a CRC32 checksum to
 *              the header, and make all the types 32-bit.
 *                --Benjamin Smedberg <benjamin@smedbergs.us>
 */

#ifndef bspatch_h__
#define bspatch_h__

#define OK 0
#define MEM_ERROR 1
// #define IO_ERROR 2  // Use READ_ERROR or WRITE_ERROR instead
#define USAGE_ERROR 3
#define CRC_ERROR 4
#define PARSE_ERROR 5
#define READ_ERROR 6
#define WRITE_ERROR 7
#define UNEXPECTED_ERROR 8

typedef struct MBSPatchHeader_ {
  /* "MBDIFF10" */
  char tag[8];
  
  /* Length of the file to be patched */
  unsigned int slen;

  /* CRC32 of the file to be patched */
  unsigned int scrc32;

  /* Length of the result file */
  unsigned int dlen;

  /* Length of the control block in bytes */
  unsigned int cblen;

  /* Length of the diff block in bytes */
  unsigned int difflen;

  /* Length of the extra block in bytes */
  unsigned int extralen;

  /* Control block (MBSPatchTriple[]) */
  /* Diff block (binary data) */
  /* Extra block (binary data) */
} MBSPatchHeader;

/**
 * Read the header of a patch file into the MBSPatchHeader structure.
 *
 * @param fd Must have been opened for reading, and be at the beginning
 *           of the file.
 */
int MBS_ReadHeader(int fd, MBSPatchHeader *header);

/**
 * Apply a patch. This method does not validate the checksum of the original
 * file: client code should validate the checksum before calling this method.
 *
 * @param patchfd Must have been processed by MBS_ReadHeader
 * @param fbuffer The original file read into a memory buffer of length
 *                header->slen.
 * @param filefd  Must have been opened for writing. Should be truncated
 *                to header->dlen if it is an existing file. The offset
 *                should be at the beginning of the file.
 */
int MBS_ApplyPatch(const MBSPatchHeader *header, int patchfd,
                   unsigned char *fbuffer, int filefd);

typedef struct MBSPatchTriple_ {
  unsigned int x; /* add x bytes from oldfile to x bytes from the diff block */
  unsigned int y; /* copy y bytes from the extra block */
  int z; /* seek forwards in oldfile by z bytes */
} MBSPatchTriple;

/**
 * Apply the given patch file to a given source file. This method validates 
 * the CRC of the original file stored in the patch file, before applying the 
 * patch to it.
 */
int ApplyBinaryPatch(const wchar_t *old_file, const wchar_t *patch_file,
                     const wchar_t *new_file);

/**
  * Calculates Crc of the given buffer by calling CRC method in LZMA SDK
  */
int CalculateCrc(const unsigned char *buf, int size);
#endif  // bspatch_h__
