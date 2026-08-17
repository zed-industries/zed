#ifndef _SEPOL_IBPKEY_RECORD_H_
#define _SEPOL_IBPKEY_RECORD_H_

#include <stddef.h>
#include <stdint.h>
#include <sepol/context_record.h>
#include <sepol/handle.h>

#define INET6_ADDRLEN 16

#ifdef __cplusplus
extern "C" {
#endif

struct sepol_ibpkey;
struct sepol_ibpkey_key;
typedef struct sepol_ibpkey sepol_ibpkey_t;
typedef struct sepol_ibpkey_key sepol_ibpkey_key_t;

extern int sepol_ibpkey_compare(const sepol_ibpkey_t *ibpkey,
				const sepol_ibpkey_key_t *key);

extern int sepol_ibpkey_compare2(const sepol_ibpkey_t *ibpkey,
				 const sepol_ibpkey_t *ibpkey2);

extern int sepol_ibpkey_key_create(sepol_handle_t *handle,
				   const char *subnet_prefix,
				   int low, int high,
				   sepol_ibpkey_key_t **key_ptr);

extern void sepol_ibpkey_key_unpack(const sepol_ibpkey_key_t *key,
				    uint64_t *subnet_prefix,
				    int *low, int *high);

extern int sepol_ibpkey_key_extract(sepol_handle_t *handle,
				    const sepol_ibpkey_t *ibpkey,
				    sepol_ibpkey_key_t **key_ptr);

extern void sepol_ibpkey_key_free(sepol_ibpkey_key_t *key);

extern int sepol_ibpkey_get_low(const sepol_ibpkey_t *ibpkey);

extern int sepol_ibpkey_get_high(const sepol_ibpkey_t *ibpkey);

extern void sepol_ibpkey_set_pkey(sepol_ibpkey_t *ibpkey, int pkey_num);

extern void sepol_ibpkey_set_range(sepol_ibpkey_t *ibpkey, int low, int high);

extern int sepol_ibpkey_get_subnet_prefix(sepol_handle_t *handle,
					  const sepol_ibpkey_t *ibpkey,
					  char **subnet_prefix);

extern uint64_t sepol_ibpkey_get_subnet_prefix_bytes(const sepol_ibpkey_t *ibpkey);

extern int sepol_ibpkey_set_subnet_prefix(sepol_handle_t *handle,
					  sepol_ibpkey_t *ibpkey,
					  const char *subnet_prefix);

extern void sepol_ibpkey_set_subnet_prefix_bytes(sepol_ibpkey_t *ibpkey,
						 uint64_t subnet_prefix);

extern sepol_context_t *sepol_ibpkey_get_con(const sepol_ibpkey_t *ibpkey);

extern int sepol_ibpkey_set_con(sepol_handle_t *handle,
				sepol_ibpkey_t *ibpkey, sepol_context_t *con);

extern int sepol_ibpkey_create(sepol_handle_t *handle, sepol_ibpkey_t **ibpkey_ptr);

extern int sepol_ibpkey_clone(sepol_handle_t *handle,
			      const sepol_ibpkey_t *ibpkey,
			      sepol_ibpkey_t **ibpkey_ptr);

extern void sepol_ibpkey_free(sepol_ibpkey_t *ibpkey);


#ifdef __cplusplus
}
#endif

#endif
