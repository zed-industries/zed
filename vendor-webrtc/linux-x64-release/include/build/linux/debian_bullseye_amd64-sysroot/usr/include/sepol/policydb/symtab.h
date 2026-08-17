
/* Author : Stephen Smalley, <sds@tycho.nsa.gov> */

/* FLASK */

/*
 * A symbol table (symtab) maintains associations between symbol
 * strings and datum values.  The type of the datum values
 * is arbitrary.  The symbol table type is implemented
 * using the hash table type (hashtab).
 */

#ifndef _SEPOL_POLICYDB_SYMTAB_H_
#define _SEPOL_POLICYDB_SYMTAB_H_

#include <sepol/policydb/hashtab.h>

#ifdef __cplusplus
extern "C" {
#endif

/* The symtab_datum struct stores the common information for
 * all symtab datums. It should the first element in every
 * struct that will be used in a symtab to allow the specific
 * datum types to be freely cast to this type.
 *
 * The values start at 1 - 0 is never a valid value.
 */
typedef struct symtab_datum {
	uint32_t value;
} symtab_datum_t;

typedef struct {
	hashtab_t table;	/* hash table (keyed on a string) */
	uint32_t nprim;		/* number of primary names in table */
} symtab_t;

extern int symtab_init(symtab_t *, unsigned int size);
extern void symtab_destroy(symtab_t *);

#ifdef __cplusplus
}
#endif

#endif				/* _SYMTAB_H_ */

/* FLASK */
