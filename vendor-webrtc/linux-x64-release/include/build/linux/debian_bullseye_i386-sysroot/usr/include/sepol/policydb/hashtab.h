/* Author : Stephen Smalley, <sds@tycho.nsa.gov> */

/* FLASK */

/*
 * A hash table (hashtab) maintains associations between
 * key values and datum values.  The type of the key values 
 * and the type of the datum values is arbitrary.  The
 * functions for hash computation and key comparison are
 * provided by the creator of the table.
 */

#ifndef _SEPOL_POLICYDB_HASHTAB_H_
#define _SEPOL_POLICYDB_HASHTAB_H_

#include <sepol/errcodes.h>

#include <stdint.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef char *hashtab_key_t;	/* generic key type */
typedef const char *const_hashtab_key_t;	/* constant generic key type */
typedef void *hashtab_datum_t;	/* generic datum type */

typedef struct hashtab_node *hashtab_ptr_t;

typedef struct hashtab_node {
	hashtab_key_t key;
	hashtab_datum_t datum;
	hashtab_ptr_t next;
} hashtab_node_t;

typedef struct hashtab_val {
	hashtab_ptr_t *htable;	/* hash table */
	unsigned int size;	/* number of slots in hash table */
	uint32_t nel;		/* number of elements in hash table */
	unsigned int (*hash_value) (struct hashtab_val * h, const_hashtab_key_t key);	/* hash function */
	int (*keycmp) (struct hashtab_val * h, const_hashtab_key_t key1, const_hashtab_key_t key2);	/* key comparison function */
} hashtab_val_t;

typedef hashtab_val_t *hashtab_t;

/*
   Creates a new hash table with the specified characteristics.

   Returns NULL if insufficient space is available or
   the new hash table otherwise.
 */
extern hashtab_t hashtab_create(unsigned int (*hash_value) (hashtab_t h,
							    const_hashtab_key_t
							    key),
				int (*keycmp) (hashtab_t h,
					       const_hashtab_key_t key1,
					       const_hashtab_key_t key2),
				unsigned int size);
/*
   Inserts the specified (key, datum) pair into the specified hash table.

   Returns SEPOL_ENOMEM if insufficient space is available or
   SEPOL_EEXIST  if there is already an entry with the same key or
   SEPOL_OK otherwise.
 */
extern int hashtab_insert(hashtab_t h, hashtab_key_t k, hashtab_datum_t d);

/*
   Removes the entry with the specified key from the hash table.
   Applies the specified destroy function to (key,datum,args) for
   the entry.

   Returns SEPOL_ENOENT if no entry has the specified key or
   SEPOL_OK otherwise.
 */
extern int hashtab_remove(hashtab_t h, hashtab_key_t k,
			  void (*destroy) (hashtab_key_t k,
					   hashtab_datum_t d,
					   void *args), void *args);

/*
   Searches for the entry with the specified key in the hash table.

   Returns NULL if no entry has the specified key or
   the datum of the entry otherwise.
 */
extern hashtab_datum_t hashtab_search(hashtab_t h, const_hashtab_key_t k);

/*
   Destroys the specified hash table.
 */
extern void hashtab_destroy(hashtab_t h);

/*
   Applies the specified apply function to (key,datum,args)
   for each entry in the specified hash table.

   The order in which the function is applied to the entries
   is dependent upon the internal structure of the hash table.

   If apply returns a non-zero status, then hashtab_map will cease
   iterating through the hash table and will propagate the error
   return to its caller.
 */
extern int hashtab_map(hashtab_t h,
		       int (*apply) (hashtab_key_t k,
				     hashtab_datum_t d,
				     void *args), void *args);

extern void hashtab_hash_eval(hashtab_t h, char *tag);

#ifdef __cplusplus
}
#endif

#endif
