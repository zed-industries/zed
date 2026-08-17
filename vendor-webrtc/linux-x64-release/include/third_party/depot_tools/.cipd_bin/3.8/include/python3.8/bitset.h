
#ifndef Py_BITSET_H
#define Py_BITSET_H
#ifdef __cplusplus
extern "C" {
#endif

/* Bitset interface */

#define BYTE            char
typedef BYTE *bitset;

#define testbit(ss, ibit) (((ss)[BIT2BYTE(ibit)] & BIT2MASK(ibit)) != 0)

#define BITSPERBYTE     (8*sizeof(BYTE))
#define BIT2BYTE(ibit)  ((ibit) / BITSPERBYTE)
#define BIT2SHIFT(ibit) ((ibit) % BITSPERBYTE)
#define BIT2MASK(ibit)  (1 << BIT2SHIFT(ibit))

#ifdef __cplusplus
}
#endif
#endif /* !Py_BITSET_H */
