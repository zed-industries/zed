#ifndef MD5_H
#define MD5_H

#include "compiler.h"

#define MD5_HASHBYTES 16

typedef struct MD5Context {
	uint32_t buf[4];
	uint32_t bits[2];
	unsigned char in[64];
} MD5_CTX;

extern void   MD5Init(MD5_CTX *context);
extern void   MD5Update(MD5_CTX *context, unsigned char const *buf,
	       unsigned len);
extern void   MD5Final(unsigned char digest[MD5_HASHBYTES], MD5_CTX *context);
extern void   MD5Transform(uint32_t buf[4], uint32_t const in[16]);
extern char * MD5End(MD5_CTX *, char *);

#endif /* !MD5_H */
