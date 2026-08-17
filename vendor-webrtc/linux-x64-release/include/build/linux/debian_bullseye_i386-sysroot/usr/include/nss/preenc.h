/* -*- Mode: C; tab-width: 4; indent-tabs-mode: nil -*- */

/*
 * Fortezza support is removed.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* Fortezza support is removed.
 * This file remains so that old programs will continue to compile,
 * But this functionality is no longer supported or implemented.
 */

#include "seccomon.h"
#include "prio.h"

typedef struct PEHeaderStr PEHeader;

#define PE_MIME_TYPE "application/pre-encrypted"

typedef struct PEFortezzaHeaderStr PEFortezzaHeader;
typedef struct PEFortezzaGeneratedHeaderStr PEFortezzaGeneratedHeader;
typedef struct PEFixedKeyHeaderStr PEFixedKeyHeader;
typedef struct PERSAKeyHeaderStr PERSAKeyHeader;

struct PEFortezzaHeaderStr {
    unsigned char key[12];
    unsigned char iv[24];
    unsigned char hash[20];
    unsigned char serial[8];
};

struct PEFortezzaGeneratedHeaderStr {
    unsigned char key[12];
    unsigned char iv[24];
    unsigned char hash[20];
    unsigned char Ra[128];
    unsigned char Y[128];
};

struct PEFixedKeyHeaderStr {
    unsigned char pkcs11Mech[4];
    unsigned char labelLen[2];
    unsigned char keyIDLen[2];
    unsigned char ivLen[2];
    unsigned char keyLen[2];
    unsigned char data[1];
};

struct PERSAKeyHeaderStr {
    unsigned char pkcs11Mech[4];
    unsigned char issuerLen[2];
    unsigned char serialLen[2];
    unsigned char ivLen[2];
    unsigned char keyLen[2];
    unsigned char data[1];
};

#define PEFIXED_Label(header) (header->data)
#define PEFIXED_KeyID(header) (&header->data[GetInt2(header->labelLen)])
#define PEFIXED_IV(header) (&header->data[GetInt2(header->labelLen) + \
                                          GetInt2(header->keyIDLen)])
#define PEFIXED_Key(header) (&header->data[GetInt2(header->labelLen) + \
                                           GetInt2(header->keyIDLen) + \
                                           GetInt2(header->keyLen)])
#define PERSA_Issuer(header) (header->data)
#define PERSA_Serial(header) (&header->data[GetInt2(header->issuerLen)])
#define PERSA_IV(header) (&header->data[GetInt2(header->issuerLen) + \
                                        GetInt2(header->serialLen)])
#define PERSA_Key(header) (&header->data[GetInt2(header->issuerLen) + \
                                         GetInt2(header->serialLen) + \
                                         GetInt2(header->keyLen)])
struct PEHeaderStr {
    unsigned char magic[2];
    unsigned char len[2];
    unsigned char type[2];
    unsigned char version[2];
    union {
        PEFortezzaHeader fortezza;
        PEFortezzaGeneratedHeader g_fortezza;
        PEFixedKeyHeader fixed;
        PERSAKeyHeader rsa;
    } u;
};

#define PE_CRYPT_INTRO_LEN 8
#define PE_INTRO_LEN 4
#define PE_BASE_HEADER_LEN 8

#define PRE_BLOCK_SIZE 8

#define GetInt2(c) ((c[0] << 8) | c[1])
#define GetInt4(c) (((unsigned long)c[0] << 24) | ((unsigned long)c[1] << 16) | \
                    ((unsigned long)c[2] << 8) | ((unsigned long)c[3]))
#define PutInt2(c, i) ((c[1] = (i)&0xff), (c[0] = ((i) >> 8) & 0xff))
#define PutInt4(c, i) ((c[0] = ((i) >> 24) & 0xff), (c[1] = ((i) >> 16) & 0xff), \
                       (c[2] = ((i) >> 8) & 0xff), (c[3] = (i)&0xff))

#define PRE_MAGIC 0xc0de
#define PRE_VERSION 0x1010
#define PRE_FORTEZZA_FILE 0x00ff
#define PRE_FORTEZZA_STREAM 0x00f5
#define PRE_FORTEZZA_GEN_STREAM 0x00f6
#define PRE_FIXED_FILE 0x000f
#define PRE_RSA_FILE 0x001f
#define PRE_FIXED_STREAM 0x0005

PEHeader *SSL_PreencryptedStreamToFile(PRFileDesc *fd, PEHeader *,
                                       int *headerSize);

PEHeader *SSL_PreencryptedFileToStream(PRFileDesc *fd, PEHeader *,
                                       int *headerSize);
