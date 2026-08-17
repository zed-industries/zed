#ifndef _SEPOL_NODE_RECORD_H_
#define _SEPOL_NODE_RECORD_H_

#include <stddef.h>
#include <sepol/context_record.h>
#include <sepol/handle.h>

#ifdef __cplusplus
extern "C" {
#endif

struct sepol_node;
struct sepol_node_key;
typedef struct sepol_node sepol_node_t;
typedef struct sepol_node_key sepol_node_key_t;

#define SEPOL_PROTO_IP4 0
#define SEPOL_PROTO_IP6 1

/* Key */
extern int sepol_node_compare(const sepol_node_t * node,
			      const sepol_node_key_t * key);

extern int sepol_node_compare2(const sepol_node_t * node,
			       const sepol_node_t * node2);

extern int sepol_node_key_create(sepol_handle_t * handle,
				 const char *addr,
				 const char *mask,
				 int proto, sepol_node_key_t ** key_ptr);

extern void sepol_node_key_unpack(const sepol_node_key_t * key,
				  const char **addr,
				  const char **mask, int *proto);

extern int sepol_node_key_extract(sepol_handle_t * handle,
				  const sepol_node_t * node,
				  sepol_node_key_t ** key_ptr);

extern void sepol_node_key_free(sepol_node_key_t * key);

/* Address */
extern int sepol_node_get_addr(sepol_handle_t * handle,
			       const sepol_node_t * node, char **addr);

extern int sepol_node_get_addr_bytes(sepol_handle_t * handle,
				     const sepol_node_t * node,
				     char **addr, size_t * addr_sz);

extern int sepol_node_set_addr(sepol_handle_t * handle,
			       sepol_node_t * node,
			       int proto, const char *addr);

extern int sepol_node_set_addr_bytes(sepol_handle_t * handle,
				     sepol_node_t * node,
				     const char *addr, size_t addr_sz);

/* Netmask */
extern int sepol_node_get_mask(sepol_handle_t * handle,
			       const sepol_node_t * node, char **mask);

extern int sepol_node_get_mask_bytes(sepol_handle_t * handle,
				     const sepol_node_t * node,
				     char **mask, size_t * mask_sz);

extern int sepol_node_set_mask(sepol_handle_t * handle,
			       sepol_node_t * node,
			       int proto, const char *mask);

extern int sepol_node_set_mask_bytes(sepol_handle_t * handle,
				     sepol_node_t * node,
				     const char *mask, size_t mask_sz);

/* Protocol */
extern int sepol_node_get_proto(const sepol_node_t * node);

extern void sepol_node_set_proto(sepol_node_t * node, int proto);

extern const char *sepol_node_get_proto_str(int proto);

/* Context */
extern sepol_context_t *sepol_node_get_con(const sepol_node_t * node);

extern int sepol_node_set_con(sepol_handle_t * handle,
			      sepol_node_t * node, sepol_context_t * con);

/* Create/Clone/Destroy */
extern int sepol_node_create(sepol_handle_t * handle, sepol_node_t ** node_ptr);

extern int sepol_node_clone(sepol_handle_t * handle,
			    const sepol_node_t * node,
			    sepol_node_t ** node_ptr);

extern void sepol_node_free(sepol_node_t * node);

#ifdef __cplusplus
}
#endif

#endif
