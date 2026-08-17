/* See LICENSE file for copyright and license details. */
#ifndef UTF_H
#define UTF_H

#include <stddef.h>
#include <stdint.h>

#define RUNE_C(x) INT32_C(x)
typedef int32_t Rune;

#define UTFmax 6 /* maximum bytes per rune */

#define Runeself 0x80              /* rune and utf are equal (<) */
#define Runemax RUNE_C(0x7FFFFFFF) /* maximum rune value */
#define Runeerror ((Rune)-1)       /* decoding error in utf */

#ifdef __cplusplus
extern "C" {
#endif

int charntorune(Rune*, const char*, size_t);
int chartorune(Rune*, const char*);
int fullrune(const char*, size_t);
int runelen(const Rune);
size_t runenlen(const Rune*, size_t);
int runetochar(char*, const Rune*);
size_t utflen(const char*);
size_t utfnlen(const char*, size_t);
char* utfrune(const char*, Rune);
char* utfrrune(const char*, Rune);
char* utfutf(const char*, const char*);
char* utfecpy(char*, char*, const char*);

int isalnumrune(Rune);
int isalpharune(Rune);
int isblankrune(Rune);
int iscntrlrune(Rune);
int isdigitrune(Rune);
int isgraphrune(Rune);
int islowerrune(Rune);
int isprintrune(Rune);
int ispunctrune(Rune);
int isspacerune(Rune);
int istitlerune(Rune);
int isupperrune(Rune);
int isvalidrune(Rune);
int isxdigitrune(Rune);
Rune tolowerrune(Rune);
Rune toupperrune(Rune);

Rune* runestrcat(Rune*, const Rune*);
Rune* runestrncat(Rune*, const Rune*, size_t);
int runestrcmp(const Rune*, const Rune*);
int runestrncmp(const Rune*, const Rune*, size_t);
Rune* runestrcpy(Rune*, const Rune*);
Rune* runestrncpy(Rune*, const Rune*, size_t);
size_t runestrlen(const Rune*);
Rune* runestrchr(const Rune*, Rune);
Rune* runestrrchr(const Rune*, Rune);
Rune* runestrdup(const Rune*);
Rune* runestrstr(const Rune*, const Rune*);
Rune* runestrecpy(Rune*, Rune*, Rune*);

extern const unsigned char utftab[64];

#ifdef __cplusplus
}
#endif

#endif
