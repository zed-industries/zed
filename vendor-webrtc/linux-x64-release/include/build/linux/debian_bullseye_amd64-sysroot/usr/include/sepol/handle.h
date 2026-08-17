#ifndef _SEPOL_HANDLE_H_
#define _SEPOL_HANDLE_H_

#ifdef __cplusplus
extern "C" {
#endif

struct sepol_handle;
typedef struct sepol_handle sepol_handle_t;

/* Create and return a sepol handle. */
sepol_handle_t *sepol_handle_create(void);

/* Get whether or not dontaudits will be disabled, same values as
 * specified by set_disable_dontaudit. This value reflects the state
 * your system will be set to upon commit, not necessarily its
 * current state.*/
int sepol_get_disable_dontaudit(sepol_handle_t * sh);

/* Set whether or not to disable dontaudits, 0 is default and does 
 * not disable dontaudits, 1 disables them */
void sepol_set_disable_dontaudit(sepol_handle_t * sh, int disable_dontaudit);

/* Set whether module_expand() should consume the base policy passed in.
 * This should reduce the amount of memory required to expand the policy. */
void sepol_set_expand_consume_base(sepol_handle_t * sh, int consume_base);

/* Destroy a sepol handle. */
void sepol_handle_destroy(sepol_handle_t *);

/* Get whether or not needless unused branch of tunables would be preserved */
int sepol_get_preserve_tunables(sepol_handle_t * sh);

/* Set whether or not to preserve the needless unused branch of tunables,
 * 0 is default and discard such branch, 1 preserves them */
void sepol_set_preserve_tunables(sepol_handle_t * sh, int preserve_tunables);

#ifdef __cplusplus
}
#endif

#endif
