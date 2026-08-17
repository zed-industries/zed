/* ----------------------------------------------------------------------- *
 *
 *   Copyright 1996-2020 The NASM Authors - All Rights Reserved
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

#ifndef PECOFF_H
#define PECOFF_H

/*
 * Microsoft Portable Executable and Common Object
 * File Format Specification
 *
 * Revision 8.1 – February 15, 2008
 */

/*
 * Machine types
 */
#define IMAGE_FILE_MACHINE_UNKNOWN      0x0000
#define IMAGE_FILE_MACHINE_AM33         0x01d3
#define IMAGE_FILE_MACHINE_AMD64        0x8664
#define IMAGE_FILE_MACHINE_EBC          0x0ebc
#define IMAGE_FILE_MACHINE_M32R         0x9041
#define IMAGE_FILE_MACHINE_ALPHA        0x0184
#define IMAGE_FILE_MACHINE_ARM          0x01c0
#define IMAGE_FILE_MACHINE_ALPHA64      0x0284
#define IMAGE_FILE_MACHINE_I386         0x014c
#define IMAGE_FILE_MACHINE_IA64         0x0200
#define IMAGE_FILE_MACHINE_M68K         0x0268
#define IMAGE_FILE_MACHINE_MIPS16       0x0266
#define IMAGE_FILE_MACHINE_MIPSFPU      0x0366
#define IMAGE_FILE_MACHINE_MIPSFPU16    0x0466
#define IMAGE_FILE_MACHINE_POWERPC      0x01f0
#define IMAGE_FILE_MACHINE_POWERPCFP    0x01f1
#define IMAGE_FILE_MACHINE_R3000        0x0162
#define IMAGE_FILE_MACHINE_R4000        0x0166
#define IMAGE_FILE_MACHINE_R10000       0x0168
#define IMAGE_FILE_MACHINE_SH3          0x01a2
#define IMAGE_FILE_MACHINE_SH3DSP       0x01a3
#define IMAGE_FILE_MACHINE_SH4          0x01a6
#define IMAGE_FILE_MACHINE_SH5          0x01a8
#define IMAGE_FILE_MACHINE_THUMB        0x01c2
#define IMAGE_FILE_MACHINE_WCEMIPSV2    0x0169
#define IMAGE_FILE_MACHINE_MASK         0xffff

/*
 * Characteristics
 */
#define IMAGE_FILE_RELOCS_STRIPPED              0x0001
#define IMAGE_FILE_EXECUTABLE_IMAGE             0x0002
#define IMAGE_FILE_LINE_NUMS_STRIPPED           0x0004
#define IMAGE_FILE_LOCAL_SYMS_STRIPPED          0x0008
#define IMAGE_FILE_AGGRESSIVE_WS_TRIM           0x0010
#define IMAGE_FILE_LARGE_ADDRESS_AWARE          0x0020
#define IMAGE_FILE_BYTES_REVERSED_LO            0x0080
#define IMAGE_FILE_32BIT_MACHINE                0x0100
#define IMAGE_FILE_DEBUG_STRIPPED               0x0200
#define IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP      0x0400
#define IMAGE_FILE_NET_RUN_FROM_SWAP            0x0800
#define IMAGE_FILE_SYSTEM                       0x1000
#define IMAGE_FILE_DLL                          0x2000
#define IMAGE_FILE_UP_SYSTEM_ONLY               0x4000
#define IMAGE_FILE_BYTES_REVERSED_HI            0x8000

/*
 * Windows subsystem
 */
#define IMAGE_SUBSYSTEM_UNKNOWN                 0
#define IMAGE_SUBSYSTEM_NATIVE                  1
#define IMAGE_SUBSYSTEM_WINDOWS_GUI             2
#define IMAGE_SUBSYSTEM_WINDOWS_CUI             3
#define IMAGE_SUBSYSTEM_POSIX_CUI               7
#define IMAGE_SUBSYSTEM_WINDOWS_CE_GUI          9
#define IMAGE_SUBSYSTEM_EFI_APPLICATION         10
#define IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER 11
#define IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER      12
#define IMAGE_SUBSYSTEM_EFI_ROM                 13
#define IMAGE_SUBSYSTEM_XBOX                    14

/*
 * DLL characteristics
 */
#define IMAGE_DLL_CHARACTERISTICS_DYNAMIC_BASE          0x0040
#define IMAGE_DLL_CHARACTERISTICS_FORCE_INTEGRITY       0x0080
#define IMAGE_DLL_CHARACTERISTICS_NX_COMPAT             0x0100
#define IMAGE_DLLCHARACTERISTICS_NO_ISOLATION           0x0200
#define IMAGE_DLLCHARACTERISTICS_NO_SEH                 0x0400
#define IMAGE_DLLCHARACTERISTICS_NO_BIND                0x0800
#define IMAGE_DLLCHARACTERISTICS_WDM_DRIVER             0x2000
#define IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE  0x8000

/*
 * Section flags
 */
#define IMAGE_SCN_TYPE_REG                      0x00000000
#define IMAGE_SCN_TYPE_DSECT                    0x00000001
#define IMAGE_SCN_TYPE_NOLOAD                   0x00000002
#define IMAGE_SCN_TYPE_GROUP                    0x00000004
#define IMAGE_SCN_TYPE_NO_PAD                   0x00000008
#define IMAGE_SCN_TYPE_COPY                     0x00000010

#define IMAGE_SCN_CNT_CODE                      0x00000020
#define IMAGE_SCN_CNT_INITIALIZED_DATA          0x00000040
#define IMAGE_SCN_CNT_UNINITIALIZED_DATA        0x00000080

#define IMAGE_SCN_LNK_OTHER                     0x00000100
#define IMAGE_SCN_LNK_INFO                      0x00000200
#define IMAGE_SCN_TYPE_OVER                     0x00000400
#define IMAGE_SCN_LNK_REMOVE                    0x00000800
#define IMAGE_SCN_LNK_COMDAT                    0x00001000

#define IMAGE_SCN_MAX_RELOC			0xffff

#define IMAGE_SCN_MEM_FARDATA                   0x00008000
#define IMAGE_SCN_MEM_PURGEABLE                 0x00020000
#define IMAGE_SCN_MEM_16BIT                     0x00020000
#define IMAGE_SCN_MEM_LOCKED                    0x00040000
#define IMAGE_SCN_MEM_PRELOAD                   0x00080000

#define IMAGE_SCN_ALIGN_1BYTES                  0x00100000
#define IMAGE_SCN_ALIGN_2BYTES                  0x00200000
#define IMAGE_SCN_ALIGN_4BYTES                  0x00300000
#define IMAGE_SCN_ALIGN_8BYTES                  0x00400000
#define IMAGE_SCN_ALIGN_16BYTES                 0x00500000
#define IMAGE_SCN_ALIGN_32BYTES                 0x00600000
#define IMAGE_SCN_ALIGN_64BYTES                 0x00700000
#define IMAGE_SCN_ALIGN_128BYTES                0x00800000
#define IMAGE_SCN_ALIGN_256BYTES                0x00900000
#define IMAGE_SCN_ALIGN_512BYTES                0x00a00000
#define IMAGE_SCN_ALIGN_1024BYTES               0x00b00000
#define IMAGE_SCN_ALIGN_2048BYTES               0x00c00000
#define IMAGE_SCN_ALIGN_4096BYTES               0x00d00000
#define IMAGE_SCN_ALIGN_8192BYTES               0x00e00000
#define IMAGE_SCN_ALIGN_MASK                    0x00f00000

#define IMAGE_SCN_LNK_NRELOC_OVFL               0x01000000
#define IMAGE_SCN_MEM_DISCARDABLE               0x02000000
#define IMAGE_SCN_MEM_NOT_CACHED                0x04000000
#define IMAGE_SCN_MEM_NOT_PAGED                 0x08000000
#define IMAGE_SCN_MEM_SHARED                    0x10000000
#define IMAGE_SCN_MEM_EXECUTE                   0x20000000
#define IMAGE_SCN_MEM_READ                      0x40000000
#define IMAGE_SCN_MEM_WRITE                     0x80000000

/*
 * Relocation type x86-64
 */
#define IMAGE_REL_AMD64_ABSOLUTE        0x0000
#define IMAGE_REL_AMD64_ADDR64          0x0001
#define IMAGE_REL_AMD64_ADDR32          0x0002
#define IMAGE_REL_AMD64_ADDR32NB        0x0003
#define IMAGE_REL_AMD64_REL32           0x0004
#define IMAGE_REL_AMD64_REL32_1         0x0005
#define IMAGE_REL_AMD64_REL32_2         0x0006
#define IMAGE_REL_AMD64_REL32_3         0x0007
#define IMAGE_REL_AMD64_REL32_4         0x0008
#define IMAGE_REL_AMD64_REL32_5         0x0009
#define IMAGE_REL_AMD64_SECTION         0x000a
#define IMAGE_REL_AMD64_SECREL          0x000b
#define IMAGE_REL_AMD64_SECREL7         0x000c
#define IMAGE_REL_AMD64_TOKEN           0x000d
#define IMAGE_REL_AMD64_SREL32          0x000e
#define IMAGE_REL_AMD64_PAIR            0x000f
#define IMAGE_REL_AMD64_SSPAN32         0x0010

/*
 * Relocation types i386
 */
#define IMAGE_REL_I386_ABSOLUTE         0x0000
#define IMAGE_REL_I386_DIR16            0x0001
#define IMAGE_REL_I386_REL16            0x0002
#define IMAGE_REL_I386_DIR32            0x0006
#define IMAGE_REL_I386_DIR32NB          0x0007
#define IMAGE_REL_I386_SEG12            0x0009
#define IMAGE_REL_I386_SECTION          0x000a
#define IMAGE_REL_I386_SECREL           0x000b
#define IMAGE_REL_I386_TOKEN            0x000c
#define IMAGE_REL_I386_SECREL7          0x000d
#define IMAGE_REL_I386_REL32            0x0014

/*
 * Relocation types ARM
 */
#define IMAGE_REL_ARM_ABSOLUTE          0x0000
#define IMAGE_REL_ARM_ADDR32            0x0001
#define IMAGE_REL_ARM_ADDR32NB          0x0002
#define IMAGE_REL_ARM_BRANCH24          0x0003
#define IMAGE_REL_ARM_BRANCH11          0x0004
#define IMAGE_REL_ARM_SECTION           0x000e
#define IMAGE_REL_ARM_SECREL            0x000f

/*
 * Relocation types Hitachi SuperH
 */
#define IMAGE_REL_SH3_ABSOLUTE          0x0000
#define IMAGE_REL_SH3_DIRECT16          0x0001
#define IMAGE_REL_SH3_DIRECT32          0x0002
#define IMAGE_REL_SH3_DIRECT8           0x0003
#define IMAGE_REL_SH3_DIRECT8_WORD      0x0004
#define IMAGE_REL_SH3_DIRECT8_LONG      0x0005
#define IMAGE_REL_SH3_DIRECT4           0x0006
#define IMAGE_REL_SH3_DIRECT4_WORD      0x0007
#define IMAGE_REL_SH3_DIRECT4_LONG      0x0008
#define IMAGE_REL_SH3_PCREL8_WORD       0x0009
#define IMAGE_REL_SH3_PCREL8_LONG       0x000a
#define IMAGE_REL_SH3_PCREL12_WORD      0x000b
#define IMAGE_REL_SH3_STARTOF_SECTION   0x000c
#define IMAGE_REL_SH3_SIZEOF_SECTION    0x000d
#define IMAGE_REL_SH3_SECTION           0x000e
#define IMAGE_REL_SH3_SECREL            0x000f
#define IMAGE_REL_SH3_DIRECT32_NB       0x0010
#define IMAGE_REL_SH3_GPREL4_LONG       0x0011
#define IMAGE_REL_SH3_TOKEN             0x0012
#define IMAGE_REL_SHM_PCRELPT           0x0013
#define IMAGE_REL_SHM_REFLO             0x0014
#define IMAGE_REL_SHM_REFHALF           0x0015
#define IMAGE_REL_SHM_RELLO             0x0016
#define IMAGE_REL_SHM_RELHALF           0x0017
#define IMAGE_REL_SHM_PAIR              0x0018
#define IMAGE_REL_SHM_NOMODE            0x8000

/*
 * Relocation types IBM PowerPC processors
 */
#define IMAGE_REL_PPC_ABSOLUTE          0x0000
#define IMAGE_REL_PPC_ADDR64            0x0001
#define IMAGE_REL_PPC_ADDR32            0x0002
#define IMAGE_REL_PPC_ADDR24            0x0003
#define IMAGE_REL_PPC_ADDR16            0x0004
#define IMAGE_REL_PPC_ADDR14            0x0005
#define IMAGE_REL_PPC_REL24             0x0006
#define IMAGE_REL_PPC_REL14             0x0007
#define IMAGE_REL_PPC_ADDR32NB          0x000a
#define IMAGE_REL_PPC_SECREL            0x000b
#define IMAGE_REL_PPC_SECTION           0x000c
#define IMAGE_REL_PPC_SECREL16          0x000f
#define IMAGE_REL_PPC_REFHI             0x0010
#define IMAGE_REL_PPC_REFLO             0x0011
#define IMAGE_REL_PPC_PAIR              0x0012
#define IMAGE_REL_PPC_SECRELLO          0x0013
#define IMAGE_REL_PPC_GPREL             0x0015
#define IMAGE_REL_PPC_TOKEN             0x0016

/*
 * Relocation types Intel Itanium processor family (IPF)
 */
#define IMAGE_REL_IA64_ABSOLUTE         0x0000
#define IMAGE_REL_IA64_IMM14            0x0001
#define IMAGE_REL_IA64_IMM22            0x0002
#define IMAGE_REL_IA64_IMM64            0x0003
#define IMAGE_REL_IA64_DIR32            0x0004
#define IMAGE_REL_IA64_DIR64            0x0005
#define IMAGE_REL_IA64_PCREL21B         0x0006
#define IMAGE_REL_IA64_PCREL21M         0x0007
#define IMAGE_REL_IA64_PCREL21F         0x0008
#define IMAGE_REL_IA64_GPREL22          0x0009
#define IMAGE_REL_IA64_LTOFF22          0x000a
#define IMAGE_REL_IA64_SECTION          0x000b
#define IMAGE_REL_IA64_SECREL22         0x000c
#define IMAGE_REL_IA64_SECREL64I        0x000d
#define IMAGE_REL_IA64_SECREL32         0x000e
#define IMAGE_REL_IA64_DIR32NB          0x0010
#define IMAGE_REL_IA64_SREL14           0x0011
#define IMAGE_REL_IA64_SREL22           0x0012
#define IMAGE_REL_IA64_SREL32           0x0013
#define IMAGE_REL_IA64_UREL32           0x0014
#define IMAGE_REL_IA64_PCREL60X         0x0015
#define IMAGE_REL_IA64_PCREL 60B        0x0016
#define IMAGE_REL_IA64_PCREL60F         0x0017
#define IMAGE_REL_IA64_PCREL60I         0x0018
#define IMAGE_REL_IA64_PCREL60M         0x0019
#define IMAGE_REL_IA64_IMMGPREL64       0x001a
#define IMAGE_REL_IA64_TOKEN            0x001b
#define IMAGE_REL_IA64_GPREL32          0x001c
#define IMAGE_REL_IA64_ADDEND           0x001f

/*
 * Relocation types MIPS Processors
 */
#define IMAGE_REL_MIPS_ABSOLUTE         0x0000
#define IMAGE_REL_MIPS_REFHALF          0x0001
#define IMAGE_REL_MIPS_REFWORD          0x0002
#define IMAGE_REL_MIPS_JMPADDR          0x0003
#define IMAGE_REL_MIPS_REFHI            0x0004
#define IMAGE_REL_MIPS_REFLO            0x0005
#define IMAGE_REL_MIPS_GPREL            0x0006
#define IMAGE_REL_MIPS_LITERAL          0x0007
#define IMAGE_REL_MIPS_SECTION          0x000a
#define IMAGE_REL_MIPS_SECREL           0x000b
#define IMAGE_REL_MIPS_SECRELLO         0x000c
#define IMAGE_REL_MIPS_SECRELHI         0x000d
#define IMAGE_REL_MIPS_JMPADDR16        0x0010
#define IMAGE_REL_MIPS_REFWORDNB        0x0022
#define IMAGE_REL_MIPS_PAIR             0x0025

/*
 * Relocation types Mitsubishi M32R
 */
#define IMAGE_REL_M32R_ABSOLUTE         0x0000
#define IMAGE_REL_M32R_ADDR32           0x0001
#define IMAGE_REL_M32R_ADDR32NB         0x0002
#define IMAGE_REL_M32R_ADDR24           0x0003
#define IMAGE_REL_M32R_GPREL16          0x0004
#define IMAGE_REL_M32R_PCREL24          0x0005
#define IMAGE_REL_M32R_PCREL16          0x0006
#define IMAGE_REL_M32R_PCREL8           0x0007
#define IMAGE_REL_M32R_REFHALF          0x0008
#define IMAGE_REL_M32R_REFHI            0x0009
#define IMAGE_REL_M32R_REFLO            0x000a
#define IMAGE_REL_M32R_PAIR             0x000b
#define IMAGE_REL_M32R_SECTION          0x000c
#define IMAGE_REL_M32R_SECREL           0x000d
#define IMAGE_REL_M32R_TOKEN            0x000e

/*
 * Section number values
 */
#define IMAGE_SYM_UNDEFINED      0
#define IMAGE_SYM_ABSOLUTE      -1
#define IMAGE_SYM_DEBUG         -2

/*
 * Type representation
 */
#define IMAGE_SYM_TYPE_NULL             0
#define IMAGE_SYM_TYPE_VOID             1
#define IMAGE_SYM_TYPE_CHAR             2
#define IMAGE_SYM_TYPE_SHORT            3
#define IMAGE_SYM_TYPE_INT              4
#define IMAGE_SYM_TYPE_LONG             5
#define IMAGE_SYM_TYPE_FLOAT            6
#define IMAGE_SYM_TYPE_DOUBLE           7
#define IMAGE_SYM_TYPE_STRUCT           8
#define IMAGE_SYM_TYPE_UNION            9
#define IMAGE_SYM_TYPE_ENUM             10
#define IMAGE_SYM_TYPE_MOE              11
#define IMAGE_SYM_TYPE_BYTE             12
#define IMAGE_SYM_TYPE_WORD             13
#define IMAGE_SYM_TYPE_UINT             14
#define IMAGE_SYM_TYPE_DWORD            15

#define IMAGE_SYM_DTYPE_NULL            0
#define IMAGE_SYM_DTYPE_POINTER         1
#define IMAGE_SYM_DTYPE_FUNCTION        2
#define IMAGE_SYM_DTYPE_ARRAY           3

/*
 * Storage class
 */
#define IMAGE_SYM_CLASS_END_OF_FUNCTION         -1
#define IMAGE_SYM_CLASS_NULL                    0
#define IMAGE_SYM_CLASS_AUTOMATIC               1
#define IMAGE_SYM_CLASS_EXTERNAL                2
#define IMAGE_SYM_CLASS_STATIC                  3
#define IMAGE_SYM_CLAS S_REGISTER               4
#define IMAGE_SYM_CLASS_EXTERNAL_DEF            5
#define IMAGE_SYM_CLASS_LABEL                   6
#define IMAGE_SYM_CLASS_UNDEFINED_LABEL         7
#define IMAGE_SYM_CLASS_MEMBER_OF_STRUCT        8
#define IMAGE_SYM_CLASS_ARGUMENT                9
#define IMAGE_SYM_CLASS_STRUCT_TAG              10
#define IMAGE_SYM_CLASS_MEMBER_OF_UNION         11
#define IMAGE_SYM_CLASS_UNION_TAG               12
#define IMAGE_SYM_CLASS_TYPE_DEFINITION         13
#define IMAGE_SYM_CLASS_UNDEFINED_STATIC        14
#define IMAGE_SYM_CLASS_ENUM_TAG                15
#define IMAGE_SYM_CLASS_MEMBER_OF_ENUM          16
#define IMAGE_SYM_CLASS_REGISTER_PARAM          17
#define IMAGE_SYM_CLASS_BIT_FIELD               18
#define IMAGE_SYM_CLASS_BLOCK                   100
#define IMAGE_SYM_CLASS_FUNCTION                101
#define IMAGE_SYM_CLASS_END_OF_STRUCT           102
#define IMAGE_SYM_CLASS_FILE                    103
#define IMAGE_SYM_CLASS_SECTION                 104
#define IMAGE_SYM_CLASS_WEAK_EXTERNAL           105
#define IMAGE_SYM_CLASS_CLR_TOKEN               107

/*
 * COMDAT sections
 */
#define IMAGE_COMDAT_SELECT_NODUPLICATES        1
#define IMAGE_COMDAT_SELECT_ANY                 2
#define IMAGE_COMDAT_SELECT_SAME_SIZE           3
#define IMAGE_COMDAT_SELECT_EXACT_MATCH         4
#define IMAGE_COMDAT_SELECT_ASSOCIATIVE         5
#define IMAGE_COMDAT_SELECT_LARGEST             6

/*
 * Attribute certificate table
 */
#define WIN_CERT_REVISION_1_0           0x0100
#define WIN_CERT_REVISION_2_0           0x0200
#define WIN_CERT_TYPE_X509              0x0001
#define WIN_CERT_TYPE_PKCS_SIGNED_DATA  0x0002
#define WIN_CERT_TYPE_RESERVED_1        0x0003
#define WIN_CERT_TYPE_TS_STACK_SIGNED   0x0004

/*
 * Debug type
 */
#define IMAGE_DEBUG_TYPE_UNKNOWN        0
#define IMAGE_DEBUG_TYPE_COFF           1
#define IMAGE_DEBUG_TYPE_CODEVIEW       2
#define IMAGE_DEBUG_TYPE_FPO            3
#define IMAGE_DEBUG_TYPE_MISC           4
#define IMAGE_DEBUG_TYPE_EXCEPTION      5
#define IMAGE_DEBUG_TYPE_FIXUP          6
#define IMAGE_DEBUG_TYPE_OMAP_TO_SRC    7
#define IMAGE_DEBUG_TYPE_OMAP_FROM_SRC  8
#define IMAGE_DEBUG_TYP E_BORLAND       9
#define IMAGE_DEBUG_TYPE_RESERVED10     10
#define IMAGE_DEBUG_TYPE_CLSID          11

/*
 * Base relocation types
 */
#define IMAGE_REL_BASED_ABSOLUTE        0
#define IMAGE_REL_BASED_HIGH            1
#define IMAGE_REL_BASED_LOW             2
#define IMAGE_REL_BASED_HIGHLOW         3
#define IMAGE_REL_BASED_HIGHADJ         4
#define IMAGE_REL_BASED_MIPS_JMPADDR    5
#define IMAGE_REL_BASED_MIPS_JMPADDR16  9
#define IMAGE_REL_BASED_DIR64           10

/*
 * TLS callback functions
 */
#define DLL_PROCESS_ATTACH      1
#define DLL_THREAD_ATTACH       2
#define DLL_THREAD_DETACH       3
#define DLL_PROCESS_DETACH      0

/*
 * Import Type
 */
#define IMPORT_CODE     0
#define IMPORT_DATA     1
#define IMPORT_CONST    2

/*
 * Import name type
 */
#define IMPORT_ORDINAL          0
#define IMPORT_NAME             1
#define IMPORT_NAME_NOPREFIX    2
#define IMPORT_NAME_UNDECORATE  3

struct coff_Section {
    struct SAA *data;
    uint32_t len;
    int nrelocs;
    int32_t index;
    struct coff_Reloc *head, **tail;
    uint32_t flags;             /* section flags */
    uint32_t align_flags;       /* user-specified alignment flags */
    uint32_t sectalign_flags;   /* minimum alignment from sectalign */
    char *name;
    int32_t namepos;            /* Offset of name into the strings table */
    int32_t pos, relpos;
    int64_t pass_last_seen;

    /* comdat-related members */
    char *comdat_name;
    uint32_t checksum;          /* set only for comdat sections */
    int8_t comdat_selection;
    int8_t comdat_symbol;       /* is the "comdat name" in symbol table? */
    int32_t comdat_associated;  /* associated section for selection==5 */
};

struct coff_Reloc {
    struct coff_Reloc *next;
    int32_t address;            /* relative to _start_ of section */
    int32_t symbol;             /* symbol number */
    enum {
        SECT_SYMBOLS,
        ABS_SYMBOL,
        REAL_SYMBOLS
    } symbase;                  /* relocation for symbol number :) */
    int16_t type;
};

struct coff_Symbol {
    char name[9];
    int32_t strpos;             /* string table position of name */
    int32_t value;              /* address, or COMMON variable size */
    int section;                /* section number where it's defined
                                 * - in COFF codes, not NASM codes */
    bool is_global;             /* is it a global symbol or not? */
    int16_t type;               /* 0 - notype, 0x20 - function */
    int32_t namlen;             /* full name length */
};

struct coff_DebugInfo {
    int32_t segto;
    int32_t seg;
    uint64_t size;
    struct coff_Section *section;
};

extern struct coff_Section **coff_sects;
extern int coff_nsects;
extern struct SAA *coff_syms;
extern uint32_t coff_nsyms;
extern struct SAA *coff_strs;
extern bool win32, win64;

extern char coff_infile[FILENAME_MAX];
extern char coff_outfile[FILENAME_MAX];

extern int coff_make_section(char *name, uint32_t flags);


#endif /* PECOFF_H */
