#ifndef XML_DICT_H_PRIVATE__
#define XML_DICT_H_PRIVATE__

#include <libxml/dict.h>

/*
 * Values are ANDed with 0xFFFFFFFF to support platforms where
 * unsigned is larger than 32 bits. With 32-bit unsigned values,
 * modern compilers should optimize the operation away.
 */

#define HASH_ROL(x,n) ((x) << (n) | ((x) & 0xFFFFFFFF) >> (32 - (n)))
#define HASH_ROR(x,n) (((x) & 0xFFFFFFFF) >> (n) | (x) << (32 - (n)))

/*
 * GoodOAAT: One of a smallest non-multiplicative One-At-a-Time functions
 * that passes SMHasher.
 *
 * Author: Sokolov Yura aka funny-falcon
 */

#define HASH_INIT(h1, h2, seed) \
    do { \
        h1 = seed ^ 0x3b00; \
        h2 = HASH_ROL(seed, 15); \
    } while (0)

#define HASH_UPDATE(h1, h2, ch) \
    do { \
        h1 += ch; \
        h1 += h1 << 3; \
        h2 += h1; \
        h2 = HASH_ROL(h2, 7); \
        h2 += h2 << 2; \
    } while (0)

/* Result is in h2 */
#define HASH_FINISH(h1, h2) \
    do { \
        h1 ^= h2; \
        h1 += HASH_ROL(h2, 14); \
        h2 ^= h1; h2 += HASH_ROR(h1, 6); \
        h1 ^= h2; h1 += HASH_ROL(h2, 5); \
        h2 ^= h1; h2 += HASH_ROR(h1, 8); \
        h2 &= 0xFFFFFFFF; \
    } while (0)

typedef struct {
    unsigned hashValue;
    const xmlChar *name;
} xmlHashedString;

XML_HIDDEN void
xmlInitDictInternal(void);
XML_HIDDEN void
xmlCleanupDictInternal(void);

XML_HIDDEN unsigned
xmlDictComputeHash(const xmlDict *dict, const xmlChar *string);
XML_HIDDEN unsigned
xmlDictCombineHash(unsigned v1, unsigned v2);
XML_HIDDEN xmlHashedString
xmlDictLookupHashed(xmlDictPtr dict, const xmlChar *name, int len);

XML_HIDDEN void
xmlInitRandom(void);
XML_HIDDEN void
xmlCleanupRandom(void);
XML_HIDDEN unsigned
xmlGlobalRandom(void);
XML_HIDDEN unsigned
xmlRandom(void);

#endif /* XML_DICT_H_PRIVATE__ */
