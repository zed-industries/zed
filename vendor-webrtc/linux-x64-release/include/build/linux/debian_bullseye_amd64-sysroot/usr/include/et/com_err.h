/*
 * Header file for common error description library.
 *
 * Copyright 1988, Student Information Processing Board of the
 * Massachusetts Institute of Technology.
 *
 * For copyright and distribution info, see the documentation supplied
 * with this package.
 */

#if !defined(__COM_ERR_H) && !defined(__COM_ERR_H__)

#ifdef __GNUC__
#define COM_ERR_ATTR(x) __attribute__(x)
#else
#define COM_ERR_ATTR(x)
#endif

#include <stddef.h>
#include <stdarg.h>

typedef long errcode_t;

struct error_table {
	char const * const * msgs;
	long base;
	int n_msgs;
};
struct et_list;

extern void com_err (const char *, long, const char *, ...)
	COM_ERR_ATTR((format(printf, 3, 4)));

extern void com_err_va (const char *whoami, errcode_t code, const char *fmt,
		 va_list args)
	COM_ERR_ATTR((format(printf, 3, 0)));

extern char const *error_message (long);
extern void (*com_err_hook) (const char *, long, const char *, va_list);
extern void (*set_com_err_hook (void (*) (const char *, long,
					  const char *, va_list)))
	(const char *, long, const char *, va_list);
extern void (*reset_com_err_hook (void)) (const char *, long,
					  const char *, va_list);
extern int init_error_table(const char * const *msgs, long base, int count);
extern char *(*set_com_err_gettext (char *(*) (const char *)))
	(const char *);

extern errcode_t add_error_table(const struct error_table * et);
extern errcode_t remove_error_table(const struct error_table * et);
extern void add_to_error_table(struct et_list *new_table);

/* Provided for Heimdall compatibility */
extern const char *com_right(struct et_list *list, long code);
extern const char *com_right_r(struct et_list *list, long code, char *str, size_t len);
extern void initialize_error_table_r(struct et_list **list,
				     const char **messages,
				     int num_errors,
				     long base);
extern void free_error_table(struct et_list *et);

/* Provided for compatibility with other com_err libraries */
extern int et_list_lock(void);
extern int et_list_unlock(void);

#define __COM_ERR_H
#define __COM_ERR_H__
#endif /* !defined(__COM_ERR_H) && !defined(__COM_ERR_H__)*/
