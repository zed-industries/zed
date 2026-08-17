/* SPDX-License-Identifier: LGPL-2.1-or-later */
/*
 * libmount.h - libmount API
 *
 * This file is part of libmount from util-linux project.
 *
 * Copyright (C) 2008-2018 Karel Zak <kzak@redhat.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

#ifndef _LIBMOUNT_MOUNT_H
#define _LIBMOUNT_MOUNT_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdio.h>
#include <mntent.h>
#include <sys/types.h>

/* Make sure libc MS_* definitions are used by default. Note that MS_* flags
 * may be already defined by linux/fs.h or another file -- in this case we
 * don't want to include sys/mount.h at all to avoid collisions.
 */
#if defined(__linux__) && !defined(MS_RDONLY)
# include <sys/mount.h>
#endif

#define LIBMOUNT_VERSION   "2.36.1"
#define LIBMOUNT_MAJOR_VERSION   2
#define LIBMOUNT_MINOR_VERSION   36
#define LIBMOUNT_PATCH_VERSION   1

/**
 * libmnt_cache:
 *
 * Stores canonicalized paths and evaluated tags
 */
struct libmnt_cache;

/**
 * libmnt_lock:
 *
 * Stores information about the locked file (e.g. /etc/mtab)
 */
struct libmnt_lock;

/**
 * libmnt_iter:
 *
 * Generic iterator (stores state about lists)
 */
struct libmnt_iter;

/**
 * libmnt_optmap:
 * @name: option name[=type] where type is printf-like type specifier")
 * @id: option ID or MS_* flags (e.g MS_RDONLY)
 * @mask: MNT_{NOMTAB,INVERT,...} mask
 *
 * Mount options description (map)
 */
struct libmnt_optmap
{
	const char	*name;
	int		id;
	int		mask;
};

/*
 * mount options map masks
 */
#define MNT_INVERT	(1 << 1) /* invert the mountflag */
#define MNT_NOMTAB	(1 << 2) /* skip in the mtab option string */
#define MNT_PREFIX	(1 << 3) /* prefix used for some options (e.g. "x-foo") */
#define MNT_NOHLPS	(1 << 4) /* don't add the option to mount.<type> helpers command line */

/**
 * libmnt_fs:
 *
 * Parsed fstab/mtab/mountinfo entry
 */
struct libmnt_fs;

/**
 * libmnt_table:
 *
 * List of struct libmnt_fs entries (parsed fstab/mtab/mountinfo)
 */
struct libmnt_table;

/**
 * libmnt_update
 *
 * /etc/mtab or utab update description
 */
struct libmnt_update;

/**
 * libmnt_context
 *
 * Mount/umount status
 */
struct libmnt_context;

/**
 * libmnt_monitor
 *
 * Mount tables monitor
 */
struct libmnt_monitor;

/**
 * libmnt_tabdiff:
 *
 * Stores mountinfo state
 */
struct libmnt_tabdiff;

/**
 * libmnt_ns:
 *
 * Describes mount namespace
 */
struct libmnt_ns;

/*
 * Actions
 */
enum {
	MNT_ACT_MOUNT = 1,
	MNT_ACT_UMOUNT
};

/*
 * Errors -- by default libmount returns -errno for generic errors (ENOMEM,
 * EINVAL, ...) and for mount(2) errors, but for some specific operations it
 * returns private error codes. Note that maximum system errno value should be
 * 4095 on UNIXes.
 *
 * See also mnt_context_get_syscall_errno() and mnt_context_get_helper_status().
 */
/**
 * MNT_ERR_NOFSTAB:
 *
 * not found required entry in fstab
 */
#define MNT_ERR_NOFSTAB      5000
/**
 * MNT_ERR_NOFSTYPE:
 *
 * failed to detect filesystem type
 */
#define MNT_ERR_NOFSTYPE     5001
/**
 * MNT_ERR_NOSOURCE:
 *
 * required mount source undefined
 */
#define MNT_ERR_NOSOURCE     5002
/**
 * MNT_ERR_LOOPDEV:
 *
 * loopdev setup failed, errno set by libc
 */
#define MNT_ERR_LOOPDEV      5003
/**
 * MNT_ERR_MOUNTOPT:
 *
 * failed to parse/use userspace mount options
 */
#define MNT_ERR_MOUNTOPT     5004
/**
 * MNT_ERR_APPLYFLAGS:
 *
 * failed to apply MS_PROPAGATION flags
 */
#define MNT_ERR_APPLYFLAGS   5005
/**
 * MNT_ERR_AMBIFS:
 *
 * libblkid detected more filesystems on the device
 */
#define MNT_ERR_AMBIFS       5006
/**
 * MNT_ERR_LOOPOVERLAP:
 *
 * detected overlapping loop device that cannot be re-used
 */
#define MNT_ERR_LOOPOVERLAP 5007
/**
 * MNT_ERR_LOCK:
 *
 * failed to lock mtab/utab or so.
 */
#define MNT_ERR_LOCK         5008
/**
 * MNT_ERR_NAMESPACE:
 *
 * failed to switch namespace
 */
#define MNT_ERR_NAMESPACE    5009


/*
 * Overall return codes -- based on mount(8) and umount(8) return codes.
 * See mnt_context_get_excode() for more details.
 */

/**
 * MNT_EX_SUCCESS:
 *
 * [u]mount(8) exit code: no errors
 */
#define MNT_EX_SUCCESS	0

/**
 * MNT_EX_USAGE:
 *
 * [u]mount(8) exit code: incorrect invocation or permission
 */
#define MNT_EX_USAGE	1

/**
 * MNT_EX_SYSERR:
 *
 * [u]mount(8) exit code: out of memory, cannot fork, ...
 */

#define MNT_EX_SYSERR	2

/**
 * MNT_EX_SOFTWARE:
 *
 * [u]mount(8) exit code: internal mount bug or wrong version
 */
#define MNT_EX_SOFTWARE	4

/**
 * MNT_EX_USER:
 *
 * [u]mount(8) exit code: user interrupt
 */
#define MNT_EX_USER	8

/**
 * MNT_EX_FILEIO:
 *
 * [u]mount(8) exit code: problems writing, locking, ... mtab/utab
 */
#define MNT_EX_FILEIO	16

/**
 * MNT_EX_FAIL:
 *
 * [u]mount(8) exit code: mount failure
 */
#define MNT_EX_FAIL	32

/**
 * MNT_EX_SOMEOK:
 *
 * [u]mount(8) exit code: some mount succeeded; usually when executed with
 * --all options. Never returned by libmount.
 */
#define MNT_EX_SOMEOK	64



#ifndef __GNUC_PREREQ
# if defined __GNUC__ && defined __GNUC_MINOR__
#  define __GNUC_PREREQ(maj, min)  ((__GNUC__ << 16) + __GNUC_MINOR__ >= ((maj) << 16) + (min))
# else
#  define __GNUC_PREREQ(maj, min) 0
# endif
#endif

#ifndef __ul_attribute__
# if __GNUC_PREREQ (3, 4)
#  define __ul_attribute__(_a_) __attribute__(_a_)
# else
#  define __ul_attribute__(_a_)
# endif
#endif


/* init.c */
extern void mnt_init_debug(int mask);

/* version.c */
extern int mnt_parse_version_string(const char *ver_string);
extern int mnt_get_library_version(const char **ver_string);
extern int mnt_get_library_features(const char ***features);

/* utils.c */
extern char *mnt_mangle(const char *str)
			__ul_attribute__((warn_unused_result));
extern char *mnt_unmangle(const char *str)
			__ul_attribute__((warn_unused_result));

extern int mnt_tag_is_valid(const char *tag);
extern int mnt_fstype_is_netfs(const char *type);
extern int mnt_fstype_is_pseudofs(const char *type);

extern int mnt_match_fstype(const char *type, const char *pattern)
			__ul_attribute__((warn_unused_result));
extern int mnt_match_options(const char *optstr, const char *pattern)
			__ul_attribute__((warn_unused_result));
extern const char *mnt_get_fstab_path(void);
extern const char *mnt_get_swaps_path(void);
extern const char *mnt_get_mtab_path(void);
extern int mnt_has_regular_mtab(const char **mtab, int *writable);
extern char *mnt_get_mountpoint(const char *path)
			__ul_attribute__((warn_unused_result));
extern int mnt_guess_system_root(dev_t devno, struct libmnt_cache *cache, char **path)
			__ul_attribute__((nonnull(3)));

/* cache.c */
extern struct libmnt_cache *mnt_new_cache(void)
			__ul_attribute__((warn_unused_result));
extern void mnt_free_cache(struct libmnt_cache *cache);

extern void mnt_ref_cache(struct libmnt_cache *cache);
extern void mnt_unref_cache(struct libmnt_cache *cache);

extern int mnt_cache_set_targets(struct libmnt_cache *cache,
				struct libmnt_table *mtab);
extern int mnt_cache_read_tags(struct libmnt_cache *cache, const char *devname);

extern int mnt_cache_device_has_tag(struct libmnt_cache *cache,
				const char *devname,
                                const char *token,
				const char *value);

extern char *mnt_cache_find_tag_value(struct libmnt_cache *cache,
				const char *devname, const char *token);

extern char *mnt_get_fstype(const char *devname, int *ambi,
			    struct libmnt_cache *cache)
			__ul_attribute__((warn_unused_result));
extern char *mnt_resolve_path(const char *path, struct libmnt_cache *cache)
			__ul_attribute__((warn_unused_result));
extern char *mnt_resolve_target(const char *path, struct libmnt_cache *cache)
			__ul_attribute__((warn_unused_result));
extern char *mnt_resolve_tag(const char *token, const char *value,
			     struct libmnt_cache *cache)
			__ul_attribute__((warn_unused_result));
extern char *mnt_resolve_spec(const char *spec, struct libmnt_cache *cache)
			__ul_attribute__((warn_unused_result));
extern char *mnt_pretty_path(const char *path, struct libmnt_cache *cache)
			__ul_attribute__((warn_unused_result));

/* optstr.c */
extern int mnt_optstr_next_option(char **optstr, char **name, size_t *namesz,
				char **value, size_t *valuesz);
extern int mnt_optstr_append_option(char **optstr, const char *name,
				const char *value);
extern int mnt_optstr_prepend_option(char **optstr, const char *name,
				const char *value);

extern int mnt_optstr_get_option(const char *optstr, const char *name,
				char **value, size_t *valsz);
extern int mnt_optstr_set_option(char **optstr, const char *name,
				const char *value);
extern int mnt_optstr_remove_option(char **optstr, const char *name);
extern int mnt_optstr_deduplicate_option(char **optstr, const char *name);

extern int mnt_split_optstr(const char *optstr,
			    char **user, char **vfs, char **fs,
			    int ignore_user, int ignore_vfs);

extern int mnt_optstr_get_options(const char *optstr, char **subset,
                            const struct libmnt_optmap *map, int ignore);

extern int mnt_optstr_get_flags(const char *optstr, unsigned long *flags,
				const struct libmnt_optmap *map);

extern int mnt_optstr_apply_flags(char **optstr, unsigned long flags,
                                const struct libmnt_optmap *map);

/* iter.c */
enum {

	MNT_ITER_FORWARD = 0,
	MNT_ITER_BACKWARD
};
extern struct libmnt_iter *mnt_new_iter(int direction)
			__ul_attribute__((warn_unused_result));
extern void mnt_free_iter(struct libmnt_iter *itr);

extern void mnt_reset_iter(struct libmnt_iter *itr, int direction)
			__ul_attribute__((nonnull));
extern int mnt_iter_get_direction(struct libmnt_iter *itr)
			__ul_attribute__((nonnull));

/* optmap.c */
enum {
	MNT_LINUX_MAP = 1,
	MNT_USERSPACE_MAP
};
extern const struct libmnt_optmap *mnt_get_builtin_optmap(int id);

/* lock.c */
extern struct libmnt_lock *mnt_new_lock(const char *datafile, pid_t id)
			__ul_attribute__((warn_unused_result));
extern void mnt_free_lock(struct libmnt_lock *ml);

extern void mnt_unlock_file(struct libmnt_lock *ml);
extern int mnt_lock_file(struct libmnt_lock *ml);
extern int mnt_lock_block_signals(struct libmnt_lock *ml, int enable);

/* fs.c */
extern struct libmnt_fs *mnt_new_fs(void)
			__ul_attribute__((warn_unused_result));
extern void mnt_free_fs(struct libmnt_fs *fs);
extern void mnt_ref_fs(struct libmnt_fs *fs);
extern void mnt_unref_fs(struct libmnt_fs *fs);

extern void mnt_reset_fs(struct libmnt_fs *fs);
extern struct libmnt_fs *mnt_copy_fs(struct libmnt_fs *dest,
				     const struct libmnt_fs *src)
			__ul_attribute__((warn_unused_result));
extern void *mnt_fs_get_userdata(struct libmnt_fs *fs);
extern int mnt_fs_set_userdata(struct libmnt_fs *fs, void *data);
extern const char *mnt_fs_get_source(struct libmnt_fs *fs);
extern int mnt_fs_set_source(struct libmnt_fs *fs, const char *source);
extern const char *mnt_fs_get_srcpath(struct libmnt_fs *fs);
extern int mnt_fs_get_table(struct libmnt_fs *fs, struct libmnt_table **tb);

extern int mnt_fs_get_tag(struct libmnt_fs *fs, const char **name,
			  const char **value);
extern const char *mnt_fs_get_target(struct libmnt_fs *fs);
extern int mnt_fs_set_target(struct libmnt_fs *fs, const char *tgt);
extern const char *mnt_fs_get_fstype(struct libmnt_fs *fs);
extern int mnt_fs_set_fstype(struct libmnt_fs *fs, const char *fstype);

extern int mnt_fs_streq_srcpath(struct libmnt_fs *fs, const char *path)
			__ul_attribute__((warn_unused_result));
extern int mnt_fs_streq_target(struct libmnt_fs *fs, const char *path)
			__ul_attribute__((warn_unused_result));

extern char *mnt_fs_strdup_options(struct libmnt_fs *fs)
			__ul_attribute__((warn_unused_result));
extern const char *mnt_fs_get_options(struct libmnt_fs *fs)
			__ul_attribute__((warn_unused_result));
extern const char *mnt_fs_get_optional_fields(struct libmnt_fs *fs)
			__ul_attribute__((warn_unused_result));
extern int mnt_fs_get_propagation(struct libmnt_fs *fs, unsigned long *flags);

extern int mnt_fs_set_options(struct libmnt_fs *fs, const char *optstr);
extern int mnt_fs_append_options(struct libmnt_fs *fs, const char *optstr);
extern int mnt_fs_prepend_options(struct libmnt_fs *fs, const char *optstr);

extern int mnt_fs_get_option(struct libmnt_fs *fs, const char *name,
				char **value, size_t *valsz);

extern const char *mnt_fs_get_fs_options(struct libmnt_fs *fs);
extern const char *mnt_fs_get_vfs_options(struct libmnt_fs *fs);
extern const char *mnt_fs_get_user_options(struct libmnt_fs *fs);

extern const char *mnt_fs_get_attributes(struct libmnt_fs *fs);
extern int mnt_fs_set_attributes(struct libmnt_fs *fs, const char *optstr);
extern int mnt_fs_get_attribute(struct libmnt_fs *fs, const char *name,
				char **value, size_t *valsz);
extern int mnt_fs_append_attributes(struct libmnt_fs *fs, const char *optstr);
extern int mnt_fs_prepend_attributes(struct libmnt_fs *fs, const char *optstr);

extern int mnt_fs_get_freq(struct libmnt_fs *fs);
extern int mnt_fs_set_freq(struct libmnt_fs *fs, int freq);
extern int mnt_fs_get_passno(struct libmnt_fs *fs);
extern int mnt_fs_set_passno(struct libmnt_fs *fs, int passno);
extern const char *mnt_fs_get_root(struct libmnt_fs *fs);
extern int mnt_fs_set_root(struct libmnt_fs *fs, const char *path);
extern const char *mnt_fs_get_bindsrc(struct libmnt_fs *fs);
extern int mnt_fs_set_bindsrc(struct libmnt_fs *fs, const char *src);
extern int mnt_fs_get_id(struct libmnt_fs *fs);
extern int mnt_fs_get_parent_id(struct libmnt_fs *fs);
extern dev_t mnt_fs_get_devno(struct libmnt_fs *fs);
extern pid_t mnt_fs_get_tid(struct libmnt_fs *fs);

extern const char *mnt_fs_get_swaptype(struct libmnt_fs *fs);
extern off_t mnt_fs_get_size(struct libmnt_fs *fs);
extern off_t mnt_fs_get_usedsize(struct libmnt_fs *fs);
extern int mnt_fs_get_priority(struct libmnt_fs *fs);
extern int mnt_fs_set_priority(struct libmnt_fs *fs, int prio);

extern const char *mnt_fs_get_comment(struct libmnt_fs *fs);
extern int mnt_fs_set_comment(struct libmnt_fs *fs, const char *comm);
extern int mnt_fs_append_comment(struct libmnt_fs *fs, const char *comm);

extern int mnt_fs_match_target(struct libmnt_fs *fs, const char *target,
			       struct libmnt_cache *cache);
extern int mnt_fs_match_source(struct libmnt_fs *fs, const char *source,
			       struct libmnt_cache *cache);
extern int mnt_fs_match_fstype(struct libmnt_fs *fs, const char *types);
extern int mnt_fs_match_options(struct libmnt_fs *fs, const char *options);
extern int mnt_fs_print_debug(struct libmnt_fs *fs, FILE *file);

extern int mnt_fs_is_kernel(struct libmnt_fs *fs);
extern int mnt_fs_is_swaparea(struct libmnt_fs *fs);
extern int mnt_fs_is_netfs(struct libmnt_fs *fs);
extern int mnt_fs_is_pseudofs(struct libmnt_fs *fs);

extern void mnt_free_mntent(struct mntent *mnt);
extern int mnt_fs_to_mntent(struct libmnt_fs *fs, struct mntent **mnt);

/* tab-parse.c */
extern struct libmnt_table *mnt_new_table_from_file(const char *filename)
			__ul_attribute__((warn_unused_result));
extern struct libmnt_table *mnt_new_table_from_dir(const char *dirname)
			__ul_attribute__((warn_unused_result));
extern int mnt_table_parse_stream(struct libmnt_table *tb, FILE *f,
				  const char *filename);
extern int mnt_table_parse_file(struct libmnt_table *tb, const char *filename);
extern int mnt_table_parse_dir(struct libmnt_table *tb, const char *dirname);

extern int mnt_table_parse_fstab(struct libmnt_table *tb, const char *filename);
extern int mnt_table_parse_swaps(struct libmnt_table *tb, const char *filename);
extern int mnt_table_parse_mtab(struct libmnt_table *tb, const char *filename);
extern int mnt_table_set_parser_errcb(struct libmnt_table *tb,
                int (*cb)(struct libmnt_table *tb, const char *filename, int line));

/* tab.c */
extern struct libmnt_table *mnt_new_table(void)
			__ul_attribute__((warn_unused_result));
extern void mnt_free_table(struct libmnt_table *tb);

extern void mnt_ref_table(struct libmnt_table *tb);
extern void mnt_unref_table(struct libmnt_table *tb);

extern int mnt_reset_table(struct libmnt_table *tb);
extern int mnt_table_get_nents(struct libmnt_table *tb);
extern int mnt_table_is_empty(struct libmnt_table *tb);

extern int mnt_table_set_userdata(struct libmnt_table *tb, void *data);
extern void *mnt_table_get_userdata(struct libmnt_table *tb);

extern void mnt_table_enable_comments(struct libmnt_table *tb, int enable);
extern int mnt_table_with_comments(struct libmnt_table *tb);
extern const char *mnt_table_get_intro_comment(struct libmnt_table *tb);
extern int mnt_table_set_intro_comment(struct libmnt_table *tb, const char *comm);
extern int mnt_table_append_intro_comment(struct libmnt_table *tb, const char *comm);
extern int mnt_table_set_trailing_comment(struct libmnt_table *tb, const char *comm);
extern const char *mnt_table_get_trailing_comment(struct libmnt_table *tb);
extern int mnt_table_append_trailing_comment(struct libmnt_table *tb, const char *comm);

extern int mnt_table_set_cache(struct libmnt_table *tb, struct libmnt_cache *mpc);
extern struct libmnt_cache *mnt_table_get_cache(struct libmnt_table *tb);
extern int mnt_table_add_fs(struct libmnt_table *tb, struct libmnt_fs *fs);
extern int mnt_table_find_fs(struct libmnt_table *tb, struct libmnt_fs *fs);
extern int mnt_table_insert_fs(struct libmnt_table *tb, int before,
				struct libmnt_fs *pos, struct libmnt_fs *fs);
extern int mnt_table_move_fs(struct libmnt_table *src, struct libmnt_table *dst,
                      int before, struct libmnt_fs *pos, struct libmnt_fs *fs);
extern int mnt_table_remove_fs(struct libmnt_table *tb, struct libmnt_fs *fs);
extern int mnt_table_first_fs(struct libmnt_table *tb, struct libmnt_fs **fs);
extern int mnt_table_last_fs(struct libmnt_table *tb, struct libmnt_fs **fs);
extern int mnt_table_next_fs(struct libmnt_table *tb, struct libmnt_iter *itr,
			     struct libmnt_fs **fs);
extern int mnt_table_next_child_fs(struct libmnt_table *tb, struct libmnt_iter *itr,
	                        struct libmnt_fs *parent, struct libmnt_fs **chld);
extern int mnt_table_get_root_fs(struct libmnt_table *tb, struct libmnt_fs **root);
extern int mnt_table_set_iter(struct libmnt_table *tb, struct libmnt_iter *itr,
			      struct libmnt_fs *fs);

enum {
	MNT_UNIQ_FORWARD  = (1 << 1),	/* default is backward */
	MNT_UNIQ_KEEPTREE = (1 << 2)
};
extern int mnt_table_uniq_fs(struct libmnt_table *tb, int flags,
				int (*cmp)(struct libmnt_table *,
					   struct libmnt_fs *,
					   struct libmnt_fs *));

extern struct libmnt_fs *mnt_table_find_mountpoint(struct libmnt_table *tb,
				const char *path, int direction);
extern struct libmnt_fs *mnt_table_find_target(struct libmnt_table *tb,
				const char *path, int direction);
extern struct libmnt_fs *mnt_table_find_srcpath(struct libmnt_table *tb,
				const char *path, int direction);
extern struct libmnt_fs *mnt_table_find_tag(struct libmnt_table *tb, const char *tag,
				const char *val, int direction);
extern struct libmnt_fs *mnt_table_find_target_with_option(struct libmnt_table *tb, const char *path,
			const char *option, const char *val, int direction);
extern struct libmnt_fs *mnt_table_find_source(struct libmnt_table *tb,
				const char *source, int direction);
extern struct libmnt_fs *mnt_table_find_pair(struct libmnt_table *tb,
				const char *source,
				const char *target, int direction);
extern struct libmnt_fs *mnt_table_find_devno(struct libmnt_table *tb,
				dev_t devno, int direction);

extern int mnt_table_find_next_fs(struct libmnt_table *tb,
			struct libmnt_iter *itr,
			int (*match_func)(struct libmnt_fs *, void *),
			void *userdata,
		        struct libmnt_fs **fs);

extern int mnt_table_is_fs_mounted(struct libmnt_table *tb, struct libmnt_fs *fstab_fs);

/* tab_update.c */
extern struct libmnt_update *mnt_new_update(void)
			__ul_attribute__((warn_unused_result));
extern void mnt_free_update(struct libmnt_update *upd);

extern int mnt_table_replace_file(struct libmnt_table *tb, const char *filename);
extern int mnt_table_write_file(struct libmnt_table *tb, FILE *file);

extern int mnt_update_is_ready(struct libmnt_update *upd);
extern int mnt_update_set_fs(struct libmnt_update *upd, unsigned long mountflags,
	                      const char *target, struct libmnt_fs *fs);
extern int mnt_update_table(struct libmnt_update *upd, struct libmnt_lock *lc);
extern unsigned long mnt_update_get_mflags(struct libmnt_update *upd);
extern int mnt_update_force_rdonly(struct libmnt_update *upd, int rdonly);
extern const char *mnt_update_get_filename(struct libmnt_update *upd);
extern struct libmnt_fs *mnt_update_get_fs(struct libmnt_update *upd);

/* tab_diff.c */
enum {
	MNT_TABDIFF_MOUNT = 1,
	MNT_TABDIFF_UMOUNT,
	MNT_TABDIFF_MOVE,
	MNT_TABDIFF_REMOUNT,
	MNT_TABDIFF_PROPAGATION,	/* not implemented yet (TODO) */
};

extern struct libmnt_tabdiff *mnt_new_tabdiff(void)
			__ul_attribute__((warn_unused_result));
extern void mnt_free_tabdiff(struct libmnt_tabdiff *df);

extern int mnt_diff_tables(struct libmnt_tabdiff *df,
			   struct libmnt_table *old_tab,
			   struct libmnt_table *new_tab);

extern int mnt_tabdiff_next_change(struct libmnt_tabdiff *df,
				   struct libmnt_iter *itr,
				   struct libmnt_fs **old_fs,
				   struct libmnt_fs **new_fs,
				   int *oper);

/* monitor.c */
enum {
	MNT_MONITOR_TYPE_USERSPACE = 1,	/* userspace mount options */
	MNT_MONITOR_TYPE_KERNEL		/* kernel mount table */
};

extern struct libmnt_monitor *mnt_new_monitor(void);
extern void mnt_ref_monitor(struct libmnt_monitor *mn);
extern void mnt_unref_monitor(struct libmnt_monitor *mn);

extern int mnt_monitor_enable_kernel(struct libmnt_monitor *mn, int enable);
extern int mnt_monitor_enable_userspace(struct libmnt_monitor *mn,
				int enable, const char *filename);

extern int mnt_monitor_get_fd(struct libmnt_monitor *mn);
extern int mnt_monitor_close_fd(struct libmnt_monitor *mn);
extern int mnt_monitor_wait(struct libmnt_monitor *mn, int timeout);

extern int mnt_monitor_next_change(struct libmnt_monitor *mn,
			     const char **filename, int *type);
extern int mnt_monitor_event_cleanup(struct libmnt_monitor *mn);


/* context.c */

/*
 * Mode for mount options from fstab (or mtab), see mnt_context_set_optsmode().
 */
enum {
	MNT_OMODE_IGNORE  = (1 << 1),	/* ignore mtab/fstab options */
	MNT_OMODE_APPEND  = (1 << 2),	/* append mtab/fstab options to existing options */
	MNT_OMODE_PREPEND = (1 << 3),	/* prepend mtab/fstab options to existing options */
	MNT_OMODE_REPLACE = (1 << 4),	/* replace existing options with options from mtab/fstab */

	MNT_OMODE_FORCE   = (1 << 5),   /* always read mtab/fstab options */

	MNT_OMODE_FSTAB   = (1 << 10),	/* read from fstab */
	MNT_OMODE_MTAB    = (1 << 11),	/* read from mtab if fstab not enabled or failed */
	MNT_OMODE_NOTAB   = (1 << 12),	/* do not read fstab/mtab at all */

	/* default */
	MNT_OMODE_AUTO   = (MNT_OMODE_PREPEND | MNT_OMODE_FSTAB | MNT_OMODE_MTAB),
	/* non-root users */
	MNT_OMODE_USER   = (MNT_OMODE_REPLACE | MNT_OMODE_FORCE | MNT_OMODE_FSTAB)
};

extern struct libmnt_context *mnt_new_context(void)
			__ul_attribute__((warn_unused_result));
extern void mnt_free_context(struct libmnt_context *cxt);

extern int mnt_reset_context(struct libmnt_context *cxt);
extern int mnt_context_is_restricted(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));
extern int mnt_context_force_unrestricted(struct libmnt_context *cxt);

extern int mnt_context_init_helper(struct libmnt_context *cxt,
				   int action, int flags);
extern int mnt_context_helper_setopt(struct libmnt_context *cxt, int c, char *arg);

extern int mnt_context_set_optsmode(struct libmnt_context *cxt, int mode);
extern int mnt_context_disable_canonicalize(struct libmnt_context *cxt, int disable);
extern int mnt_context_enable_lazy(struct libmnt_context *cxt, int enable);
extern int mnt_context_enable_rdonly_umount(struct libmnt_context *cxt, int enable);
extern int mnt_context_enable_rwonly_mount(struct libmnt_context *cxt, int enable);
extern int mnt_context_disable_helpers(struct libmnt_context *cxt, int disable);
extern int mnt_context_enable_sloppy(struct libmnt_context *cxt, int enable);
extern int mnt_context_enable_fake(struct libmnt_context *cxt, int enable);
extern int mnt_context_disable_mtab(struct libmnt_context *cxt, int disable);
extern int mnt_context_enable_force(struct libmnt_context *cxt, int enable);
extern int mnt_context_enable_verbose(struct libmnt_context *cxt, int enable);
extern int mnt_context_enable_loopdel(struct libmnt_context *cxt, int enable);
extern int mnt_context_enable_fork(struct libmnt_context *cxt, int enable);
extern int mnt_context_disable_swapmatch(struct libmnt_context *cxt, int disable);

extern int mnt_context_get_optsmode(struct libmnt_context *cxt);

extern int mnt_context_is_lazy(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));
extern int mnt_context_is_rdonly_umount(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));
extern int mnt_context_is_rwonly_mount(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));
extern int mnt_context_is_sloppy(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));
extern int mnt_context_is_fake(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));
extern int mnt_context_is_nomtab(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));
extern int mnt_context_is_force(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));
extern int mnt_context_is_verbose(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));
extern int mnt_context_is_loopdel(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));
extern int mnt_context_is_nohelpers(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));
extern int mnt_context_is_nocanonicalize(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));
extern int mnt_context_is_swapmatch(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));
extern int mnt_context_forced_rdonly(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));

extern int mnt_context_is_fork(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));
extern int mnt_context_is_parent(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));
extern int mnt_context_is_child(struct libmnt_context *cxt)
			__ul_attribute__((nonnull));

extern int mnt_context_wait_for_children(struct libmnt_context *cxt,
                                  int *nchildren, int *nerrs);

extern int mnt_context_is_fs_mounted(struct libmnt_context *cxt,
                              struct libmnt_fs *fs, int *mounted);
extern int mnt_context_set_fs(struct libmnt_context *cxt, struct libmnt_fs *fs);
extern struct libmnt_fs *mnt_context_get_fs(struct libmnt_context *cxt);

extern int mnt_context_set_source(struct libmnt_context *cxt, const char *source);
extern int mnt_context_set_target(struct libmnt_context *cxt, const char *target);
extern int mnt_context_set_fstype(struct libmnt_context *cxt, const char *fstype);
extern int mnt_context_set_target_prefix(struct libmnt_context *cxt, const char *path);

extern const char *mnt_context_get_source(struct libmnt_context *cxt);
extern const char *mnt_context_get_target(struct libmnt_context *cxt);
extern const char *mnt_context_get_fstype(struct libmnt_context *cxt);
extern const char *mnt_context_get_target_prefix(struct libmnt_context *cxt);

extern void *mnt_context_get_mtab_userdata(struct libmnt_context *cxt);
extern void *mnt_context_get_fstab_userdata(struct libmnt_context *cxt);
extern void *mnt_context_get_fs_userdata(struct libmnt_context *cxt);

extern int mnt_context_set_options(struct libmnt_context *cxt, const char *optstr);
extern int mnt_context_append_options(struct libmnt_context *cxt, const char *optstr);

extern const char *mnt_context_get_options(struct libmnt_context *cxt);

extern int mnt_context_set_fstype_pattern(struct libmnt_context *cxt, const char *pattern);
extern int mnt_context_set_options_pattern(struct libmnt_context *cxt, const char *pattern);

extern int mnt_context_set_passwd_cb(struct libmnt_context *cxt,
			      char *(*get)(struct libmnt_context *),
			      void (*release)(struct libmnt_context *, char *))
			__ul_attribute__((deprecated));

extern int mnt_context_set_tables_errcb(struct libmnt_context *cxt,
        int (*cb)(struct libmnt_table *tb, const char *filename, int line));
extern int mnt_context_set_fstab(struct libmnt_context *cxt,
				 struct libmnt_table *tb);
extern int mnt_context_get_fstab(struct libmnt_context *cxt,
				 struct libmnt_table **tb);

extern int mnt_context_get_mtab(struct libmnt_context *cxt,
				struct libmnt_table **tb);
extern int mnt_context_get_table(struct libmnt_context *cxt,
				const char *filename,
				struct libmnt_table **tb);
extern int mnt_context_set_cache(struct libmnt_context *cxt,
				 struct libmnt_cache *cache);
extern struct libmnt_cache *mnt_context_get_cache(struct libmnt_context *cxt);
extern struct libmnt_lock *mnt_context_get_lock(struct libmnt_context *cxt);
extern int mnt_context_set_mflags(struct libmnt_context *cxt,
				      unsigned long flags);
extern int mnt_context_get_mflags(struct libmnt_context *cxt,
				      unsigned long *flags);
extern int mnt_context_set_user_mflags(struct libmnt_context *cxt,
						unsigned long flags);
extern int mnt_context_get_user_mflags(struct libmnt_context *cxt,
						unsigned long *flags);

extern int mnt_context_set_mountdata(struct libmnt_context *cxt, void *data);
extern int mnt_context_apply_fstab(struct libmnt_context *cxt);

extern int mnt_context_reset_status(struct libmnt_context *cxt);
extern int mnt_context_get_status(struct libmnt_context *cxt);

extern int mnt_context_helper_executed(struct libmnt_context *cxt);
extern int mnt_context_get_helper_status(struct libmnt_context *cxt);

extern int mnt_context_syscall_called(struct libmnt_context *cxt);

extern int mnt_context_get_syscall_errno(struct libmnt_context *cxt);

extern int mnt_context_strerror(struct libmnt_context *cxt, char *buf,
				size_t bufsiz)
				__ul_attribute__((deprecated));

extern int mnt_context_get_excode(struct libmnt_context *cxt,
                        int rc, char *buf, size_t bufsz);

extern int mnt_context_set_target_ns(struct libmnt_context *cxt, const char *path);
extern struct libmnt_ns *mnt_context_get_target_ns(struct libmnt_context *cxt);
extern struct libmnt_ns *mnt_context_get_origin_ns(struct libmnt_context *cxt);
extern struct libmnt_ns *mnt_context_switch_ns(struct libmnt_context *cxt, struct libmnt_ns *ns);
extern struct libmnt_ns *mnt_context_switch_origin_ns(struct libmnt_context *cxt);
extern struct libmnt_ns *mnt_context_switch_target_ns(struct libmnt_context *cxt);


/* context_mount.c */
extern int mnt_context_mount(struct libmnt_context *cxt);
extern int mnt_context_umount(struct libmnt_context *cxt);
extern int mnt_context_next_mount(struct libmnt_context *cxt,
				struct libmnt_iter *itr,
				struct libmnt_fs **fs,
				int *mntrc, int *ignored);

extern int mnt_context_next_remount(struct libmnt_context *cxt,
                           struct libmnt_iter *itr,
                           struct libmnt_fs **fs,
                           int *mntrc,
                           int *ignored);

extern int mnt_context_prepare_mount(struct libmnt_context *cxt)
			__ul_attribute__((warn_unused_result));
extern int mnt_context_do_mount(struct libmnt_context *cxt);
extern int mnt_context_finalize_mount(struct libmnt_context *cxt);

/* context_umount.c */
extern int mnt_context_find_umount_fs(struct libmnt_context *cxt,
			       const char *tgt,
			       struct libmnt_fs **pfs);
extern int mnt_context_next_umount(struct libmnt_context *cxt,
				struct libmnt_iter *itr,
				struct libmnt_fs **fs,
				int *mntrc, int *ignored);

extern int mnt_context_prepare_umount(struct libmnt_context *cxt)
			__ul_attribute__((warn_unused_result));
extern int mnt_context_do_umount(struct libmnt_context *cxt);
extern int mnt_context_finalize_umount(struct libmnt_context *cxt);

extern int mnt_context_tab_applied(struct libmnt_context *cxt);
extern int mnt_context_set_syscall_status(struct libmnt_context *cxt, int status);

/*
 * mount(8) userspace options masks (MNT_MAP_USERSPACE map)
 */
#define MNT_MS_NOAUTO	(1 << 2)
#define MNT_MS_USER	(1 << 3)
#define MNT_MS_USERS	(1 << 4)
#define MNT_MS_OWNER	(1 << 5)
#define MNT_MS_GROUP	(1 << 6)
#define MNT_MS_NETDEV	(1 << 7)
#define MNT_MS_COMMENT  (1 << 8)
#define MNT_MS_LOOP     (1 << 9)
#define MNT_MS_NOFAIL   (1 << 10)
#define MNT_MS_UHELPER  (1 << 11)
#define MNT_MS_HELPER   (1 << 12)
#define MNT_MS_XCOMMENT (1 << 13)
#define MNT_MS_OFFSET   (1 << 14)
#define MNT_MS_SIZELIMIT (1 << 15)
#define MNT_MS_ENCRYPTION (1 << 16)
#define MNT_MS_XFSTABCOMM (1 << 17)
#define MNT_MS_HASH_DEVICE (1 << 18)
#define MNT_MS_ROOT_HASH (1 << 19)
#define MNT_MS_HASH_OFFSET (1 << 20)
#define MNT_MS_ROOT_HASH_FILE (1 << 21)
#define MNT_MS_FEC_DEVICE (1 << 22)
#define MNT_MS_FEC_OFFSET (1 << 23)
#define MNT_MS_FEC_ROOTS (1 << 24)
#define MNT_MS_ROOT_HASH_SIG (1 << 25)

/*
 * mount(2) MS_* masks (MNT_MAP_LINUX map)
 */
#ifndef MS_RDONLY
#define MS_RDONLY	 1	/* Mount read-only */
#endif
#ifndef MS_NOSUID
#define MS_NOSUID	 2	/* Ignore suid and sgid bits */
#endif
#ifndef MS_NODEV
#define MS_NODEV	 4	/* Disallow access to device special files */
#endif
#ifndef MS_NOEXEC
#define MS_NOEXEC	 8	/* Disallow program execution */
#endif
#ifndef MS_SYNCHRONOUS
#define MS_SYNCHRONOUS	16	/* Writes are synced at once */
#endif
#ifndef MS_REMOUNT
#define MS_REMOUNT	32	/* Alter flags of a mounted FS */
#endif
#ifndef MS_MANDLOCK
#define MS_MANDLOCK	64	/* Allow mandatory locks on an FS */
#endif
#ifndef MS_DIRSYNC
#define MS_DIRSYNC	128	/* Directory modifications are synchronous */
#endif
#ifndef MS_NOSYMFOLLOW
#define MS_NOSYMFOLLOW	256	/* Don't follow symlinks */
#endif
#ifndef MS_NOATIME
#define MS_NOATIME	0x400	/* 1024: Do not update access times. */
#endif
#ifndef MS_NODIRATIME
#define MS_NODIRATIME   0x800	/* 2048: Don't update directory access times */
#endif
#ifndef MS_BIND
#define	MS_BIND		0x1000	/* 4096: Mount existing tree elsewhere as well */
#endif
#ifndef MS_MOVE
#define MS_MOVE		0x2000	/* 8192: Atomically move the tree */
#endif
#ifndef MS_REC
#define MS_REC		0x4000	/* 16384: Recursive loopback */
#endif
#ifndef MS_SILENT
#define MS_SILENT	0x8000	/* 32768: Don't emit certain kernel messages */
#endif
#ifndef MS_UNBINDABLE
#define MS_UNBINDABLE	(1<<17)	/* 131072: Make unbindable */
#endif
#ifndef MS_PRIVATE
#define MS_PRIVATE	(1<<18)	/* 262144: Make private */
#endif
#ifndef MS_SLAVE
#define MS_SLAVE	(1<<19)	/* 524288: Make slave */
#endif
#ifndef MS_SHARED
#define MS_SHARED	(1<<20)	/* 1048576: Make shared */
#endif
#ifndef MS_RELATIME
#define MS_RELATIME	(1<<21) /* 2097152: Update atime relative to mtime/ctime */
#endif
#ifndef MS_I_VERSION
#define MS_I_VERSION	(1<<23)	/* Update the inode I_version field */
#endif
#ifndef MS_STRICTATIME
#define MS_STRICTATIME	(1<<24) /* Always perform atime updates */
#endif
#ifndef MS_LAZYTIME
#define MS_LAZYTIME     (1<<25) /* Update the on-disk [acm]times lazily */
#endif


/*
 * Magic mount flag number. Had to be or-ed to the flag values.  Deprecated and
 * no more used since libmount v2.33; required for Linux <= 2.4.
 */
#ifndef MS_MGC_VAL
#define MS_MGC_VAL 0xC0ED0000	/* magic flag number to indicate "new" flags */
#endif
#ifndef MS_MGC_MSK
#define MS_MGC_MSK 0xffff0000	/* magic flag number mask */
#endif


/* Shared-subtree options */
#define MS_PROPAGATION  (MS_SHARED|MS_SLAVE|MS_UNBINDABLE|MS_PRIVATE)

/* Options that we make ordinary users have by default.  */
#define MS_SECURE	(MS_NOEXEC|MS_NOSUID|MS_NODEV)

/* Options that we make owner-mounted devices have by default */
#define MS_OWNERSECURE	(MS_NOSUID|MS_NODEV)

#ifdef __cplusplus
}
#endif

#endif /* _LIBMOUNT_MOUNT_H */
