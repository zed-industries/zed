/* Copyright (C) 1997-2015 Free Software Foundation, Inc.
   This file is part of the GNU C Library.
   Contributed by Thorsten Kukuk <kukuk@suse.de>, 1997.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <http://www.gnu.org/licenses/>.  */

#ifndef	__RPCSVC_NISLIB_H__
#define	__RPCSVC_NISLIB_H__

#ifdef  __cplusplus
extern "C" {
#endif

typedef const char *const_nis_name;

/* nis_names: These functions are used to locate and manipulate all NIS+
 * objects except the NIS+ entry objects.
 *
 * nis_lookup (name, flags) resolves a NIS+ name and returns a copy of
 *                          that object  from a NIS+ server.
 *    const nis_name name: name of the object to be resolved
 *    unsigned int flags: logically ORing zero or more flags (FOLLOW_LINKS,
 *                          HARD_LOOKUP, [NO_CACHE], MASTER_ONLY, EXPAND_NAME)
 *
 * nis_add (name, obj) adds objects to the NIS+ namespace.
 *    const nis_name name: fully qualified NIS+ name.
 *    const nis_object *obj: object members zo_name and zo_domain will be
 *                           constructed from name.
 *
 * nis_remove (name, obj) removes objects from the NIS+ namespace.
 *    const nis_name name: fully qualified NIS+ name.
 *    const nis_object *obj: if not NULL, it is assumed to point to a copy
 *                           of the object being removed. In this case, if
 *                           the object on the server does not have the same
 *                           object identifier as the  object  being  passed,
 *                           the operation will fail with the NIS_NOTSAMEOBJ
 *                           error.
 *
 * nis_modify (name, obj) can change specific attributes of an object
 *                        that already exists in the namespace.
 */
extern nis_result *nis_lookup (const_nis_name name, unsigned int flags)
    ;
extern nis_result *nis_add (const_nis_name name, const nis_object *obj)
    ;
extern nis_result *nis_remove (const_nis_name name,
			       const nis_object *obj);
extern nis_result *nis_modify (const_nis_name name,
			       const nis_object *obj);

/* nis_tables: These functions are used to search and modify NIS+ tables.
 *
 * nis_list (table_name, flags, callback(table_name, obj, userdata), userdata)
 *           search a table in the NIS+ namespace.
 *    const nis_name table_name: indexed name ([xx=yy],table.dir)
 *    unsigned int flags: logically ORing one or more flags (FOLLOW_LINKS,
 *                      [FOLLOW_PATH], HARD_LOOKUP, [ALL_RESULTS], [NO_CACHE],
 *                      MASTER_ONLY, EXPAND_NAME, RETURN_RESULT)
 *    callback(): callback is an optional pointer to a function that will
 *                process the ENTRY type objects that are returned from the
 *                search. If this pointer is NULL, then all entries that match
 *                the search criteria are returned in the nis_result structure,
 *                otherwise  this  function  will  be  called once for each
 *                entry returned.
 *    void *userdata: passed to callback function along with the returned
 *                    entry object.
 *
 * nis_add_entry (table_name, obj, flags) will add the NIS+ object to the
 *                                        NIS+ table_name.
 *    const nis_name table_name
 *    const nis_object *obj
 *    unsigned int flags: 0, ADD_OVERWRITE, RETURN_RESULT
 *
 * nis_modify_entry (name, obj, flags) modifies an object identified by name.
 *    const nis_name name: object identifier
 *    const nis_object *obj: should point to an entry with the EN_MODIFIED
 *                           flag set in each column that contains new
 *                           information.
 *    unsigned int flags: 0, MOD_SAMEOBJ, RETURN_RESULT
 *
 * nis_remove_entry (table_name, obj, flags) removes a set of entries
 *                                 identified by table_name from the table.
 *    const nis_name table_name: indexed NIS+ name
 *    const nis_object *obj: if obj is non-null, it is presumed to point to
 *                           a cached copy of the entry. When the removal is
 *                           attempted, and the object that would be removed
 *                           is not the same as the cached object pointed to
 *                           by object then the operation will fail with an
 *                           NIS_NOTSAMEOBJ error
 *    unsigned int flags: 0, REM_MULTIPLE
 *
 * nis_first_entry (table_name) fetches entries from a table one at a time.
 *    const nis_name table_name
 *
 * nis_next_entry (table_name, cookie) retrieves the "next" entry from a
 *                                     table specified by table_name.
 *    const nis_name table_name:
 *    const netobj *cookie: The value of cookie from the nis_result structure
 *                          form the previous call.
 */
extern nis_result *nis_list (const_nis_name __name, unsigned int __flags,
			     int (*__callback)(const_nis_name __table_name,
					       const nis_object *__obj,
					       const void *__userdata),
			     const void *__userdata);
extern nis_result *nis_add_entry (const_nis_name __table_name,
				  const nis_object *__obj,
				  unsigned int __flags);
extern nis_result *nis_modify_entry (const_nis_name __name,
				     const nis_object *__obj,
				     unsigned int __flags);
extern nis_result *nis_remove_entry (const_nis_name __table_name,
				     const nis_object *__obj,
				     unsigned int __flags);
extern nis_result *nis_first_entry (const_nis_name __table_name);
extern nis_result *nis_next_entry (const_nis_name __table_name,
				   const netobj *__cookie);
/*
** nis_server
*/
extern nis_error nis_mkdir (const_nis_name __dirname,
			    const nis_server *__machine);
extern nis_error nis_rmdir (const_nis_name __dirname,
			    const nis_server *__machine);
extern nis_error nis_servstate (const nis_server *__machine,
				const nis_tag *__tags, int __numtags,
				nis_tag **__result);
extern nis_error nis_stats (const nis_server *__machine,
			    const nis_tag *__tags, int __numtags,
			    nis_tag **__result);
extern void nis_freetags (nis_tag *__tags, int __numtags);
extern nis_server **nis_getservlist (const_nis_name __dirname);
extern void nis_freeservlist (nis_server **__machines);

/*
** nis_subr
*/
extern nis_name nis_leaf_of (const_nis_name __name);
extern nis_name nis_leaf_of_r (const_nis_name __name, char *__buffer,
			       size_t __buflen);
extern nis_name nis_name_of (const_nis_name __name);
extern nis_name nis_name_of_r (const_nis_name __name, char *__buffer,
			       size_t __buflen);
extern nis_name nis_domain_of (const_nis_name __name);
extern nis_name nis_domain_of_r (const_nis_name __name, char *__buffer,
				 size_t __buflen);
extern nis_name *nis_getnames (const_nis_name __name);
extern void nis_freenames (nis_name *__namelist);
extern name_pos nis_dir_cmp (const_nis_name __n1, const_nis_name __n2);
extern nis_object *nis_clone_object (const nis_object *__src,
				     nis_object *__dest);
extern void nis_destroy_object (nis_object *__obj);
extern void nis_print_object (const nis_object *__obj);

/*
** nis_local_names
*/
extern nis_name nis_local_group (void);
extern nis_name nis_local_directory (void);
extern nis_name nis_local_principal (void);
extern nis_name nis_local_host (void);

/*
** nis_error
*/
extern const char *nis_sperrno (const nis_error __status);
extern void nis_perror (const nis_error __status, const char *__label);
extern void nis_lerror (const nis_error __status, const char *__label);
extern char *nis_sperror (const nis_error status, const char *__label);
extern char *nis_sperror_r (const nis_error __status, const char *__label,
			    char *__buffer, size_t __buflen);
/*
** nis_groups
*/
extern bool_t nis_ismember (const_nis_name __principal,
			    const_nis_name __group);
extern nis_error nis_addmember (const_nis_name __member,
				const_nis_name __group);
extern nis_error nis_removemember (const_nis_name __member,
				   const_nis_name __group);
extern nis_error nis_creategroup (const_nis_name __group,
				  unsigned int __flags);
extern nis_error nis_destroygroup (const_nis_name __group);
extern void nis_print_group_entry (const_nis_name __group);
extern nis_error nis_verifygroup (const_nis_name __group);

/*
** nis_ping
*/
extern void nis_ping (const_nis_name __dirname, uint32_t __utime,
		      const nis_object *__dirobj);
extern nis_result *nis_checkpoint (const_nis_name __dirname);

/*
** nis_print (XXX INTERNAL FUNCTIONS, SHOULD NOT BE USED !!)
*/
extern void nis_print_result (const nis_result *__result);
extern void nis_print_rights (unsigned int __rights);
extern void nis_print_directory (const directory_obj *__dirobj);
extern void nis_print_group (const group_obj *__grpobj);
extern void nis_print_table (const table_obj *__tblobj);
extern void nis_print_link (const link_obj *__lnkobj);
extern void nis_print_entry (const entry_obj *__enobj);

/*
** nis_file (XXX INTERNAL FUNCTIONS, SHOULD NOT BE USED !!)
*/
extern directory_obj *readColdStartFile (void);
extern bool_t writeColdStartFile (const directory_obj *__dirobj);
extern nis_object *nis_read_obj (const char *__obj);
extern bool_t nis_write_obj (const char *__file, const nis_object *__obj);

/*
** nis_clone - (XXX INTERNAL FUNCTIONS, SHOULD NOT BE USED !!)
*/
extern directory_obj *nis_clone_directory (const directory_obj *__src,
					   directory_obj *__dest);
extern nis_result *nis_clone_result (const nis_result *__src,
				     nis_result *__dest);

/* nis_free - nis_freeresult */
extern void nis_freeresult (nis_result *__result);
/* (XXX THE FOLLOWING ARE INTERNAL FUNCTIONS, SHOULD NOT BE USED !!) */
extern void nis_free_request (ib_request *__req);
extern void nis_free_directory (directory_obj *__dirobj);
extern void nis_free_object (nis_object *__obj);

/* (XXX INTERNAL FUNCTIONS, SHOULD NOT BE USED !!) */
extern nis_name __nis_default_owner (char *);
extern nis_name __nis_default_group (char *);
extern uint32_t __nis_default_ttl (char *);
extern unsigned int __nis_default_access (char *, unsigned int);
extern fd_result *__nis_finddirectory (directory_obj *, const_nis_name);
extern void __free_fdresult (fd_result *);
extern uint32_t __nis_hash (const void *__keyarg, size_t __len);

/* NIS+ cache locking */
extern int __nis_lock_cache (void);
extern int __nis_unlock_cache (void);

/* (XXX INTERNAL FUNCTIONS, ONLY FOR rpc.nisd AND glibc !!) */
#if defined (NIS_INTERNAL)

struct dir_binding
{
  CLIENT *clnt;                  /* RPC CLIENT handle */
  nis_server *server_val;        /* List of servers */
  unsigned int server_len;       /* # of servers */
  unsigned int server_used;      /* Which server we are bind in the moment ? */
  unsigned int current_ep;       /* Which endpoint of the server are in use? */
  unsigned int trys;             /* How many server have we tried ? */
  unsigned int class;            /* From which class is server_val ? */
  bool_t master_only;            /* Is only binded to the master */
  bool_t use_auth;               /* Do we use AUTH ? */
  bool_t use_udp;                /* Do we use UDP ? */
  struct sockaddr_in addr;       /* Server's IP address */
  int socket;                    /* Server's local socket */
};
typedef struct dir_binding dir_binding;

extern nis_error __nisbind_create (dir_binding *, const nis_server *,
				   unsigned int, unsigned int, unsigned int,
				   unsigned int);
extern nis_error __nisbind_connect (dir_binding *);
extern nis_error __nisbind_next (dir_binding *);
extern void __nisbind_destroy (dir_binding *);
extern nis_error __nisfind_server (const_nis_name, int, directory_obj **,
				   dir_binding *, unsigned int);

#endif

#ifdef  __cplusplus
}
#endif

#endif	/* __RPCSVC_NISLIB_H__ */
