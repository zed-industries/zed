/* Header file for mounting/unmount Linux filesystems.
   Copyright (C) 1996-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

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
   <https://www.gnu.org/licenses/>.  */

/* This is taken from /usr/include/linux/fs.h.  */

#ifndef _SYS_MOUNT_H
#define _SYS_MOUNT_H	1

#include <features.h>
#include <sys/ioctl.h>

#define BLOCK_SIZE	1024
#define BLOCK_SIZE_BITS	10


/* These are the fs-independent mount-flags: up to 16 flags are
   supported  */
enum
{
  MS_RDONLY = 1,		/* Mount read-only.  */
#define MS_RDONLY	MS_RDONLY
  MS_NOSUID = 2,		/* Ignore suid and sgid bits.  */
#define MS_NOSUID	MS_NOSUID
  MS_NODEV = 4,			/* Disallow access to device special files.  */
#define MS_NODEV	MS_NODEV
  MS_NOEXEC = 8,		/* Disallow program execution.  */
#define MS_NOEXEC	MS_NOEXEC
  MS_SYNCHRONOUS = 16,		/* Writes are synced at once.  */
#define MS_SYNCHRONOUS	MS_SYNCHRONOUS
  MS_REMOUNT = 32,		/* Alter flags of a mounted FS.  */
#define MS_REMOUNT	MS_REMOUNT
  MS_MANDLOCK = 64,		/* Allow mandatory locks on an FS.  */
#define MS_MANDLOCK	MS_MANDLOCK
  MS_DIRSYNC = 128,		/* Directory modifications are synchronous.  */
#define MS_DIRSYNC	MS_DIRSYNC
  MS_NOATIME = 1024,		/* Do not update access times.  */
#define MS_NOATIME	MS_NOATIME
  MS_NODIRATIME = 2048,		/* Do not update directory access times.  */
#define MS_NODIRATIME	MS_NODIRATIME
  MS_BIND = 4096,		/* Bind directory at different place.  */
#define MS_BIND		MS_BIND
  MS_MOVE = 8192,
#define MS_MOVE		MS_MOVE
  MS_REC = 16384,
#define MS_REC		MS_REC
  MS_SILENT = 32768,
#define MS_SILENT	MS_SILENT
  MS_POSIXACL = 1 << 16,	/* VFS does not apply the umask.  */
#define MS_POSIXACL	MS_POSIXACL
  MS_UNBINDABLE = 1 << 17,	/* Change to unbindable.  */
#define MS_UNBINDABLE	MS_UNBINDABLE
  MS_PRIVATE = 1 << 18,		/* Change to private.  */
#define MS_PRIVATE	MS_PRIVATE
  MS_SLAVE = 1 << 19,		/* Change to slave.  */
#define MS_SLAVE	MS_SLAVE
  MS_SHARED = 1 << 20,		/* Change to shared.  */
#define MS_SHARED	MS_SHARED
  MS_RELATIME = 1 << 21,	/* Update atime relative to mtime/ctime.  */
#define MS_RELATIME	MS_RELATIME
  MS_KERNMOUNT = 1 << 22,	/* This is a kern_mount call.  */
#define MS_KERNMOUNT	MS_KERNMOUNT
  MS_I_VERSION =  1 << 23,	/* Update inode I_version field.  */
#define MS_I_VERSION	MS_I_VERSION
  MS_STRICTATIME = 1 << 24,	/* Always perform atime updates.  */
#define MS_STRICTATIME	MS_STRICTATIME
  MS_LAZYTIME = 1 << 25,	/* Update the on-disk [acm]times lazily.  */
#define MS_LAZYTIME	MS_LAZYTIME
  MS_ACTIVE = 1 << 30,
#define MS_ACTIVE	MS_ACTIVE
  MS_NOUSER = 1 << 31
#define MS_NOUSER	MS_NOUSER
};

/* Flags that can be altered by MS_REMOUNT  */
#define MS_RMT_MASK (MS_RDONLY|MS_SYNCHRONOUS|MS_MANDLOCK|MS_I_VERSION \
		     |MS_LAZYTIME)


/* Magic mount flag number. Has to be or-ed to the flag values.  */

#define MS_MGC_VAL 0xc0ed0000	/* Magic flag number to indicate "new" flags */
#define MS_MGC_MSK 0xffff0000	/* Magic flag number mask */


/* The read-only stuff doesn't really belong here, but any other place
   is probably as bad and I don't want to create yet another include
   file.  */

#define BLKROSET   _IO(0x12, 93) /* Set device read-only (0 = read-write).  */
#define BLKROGET   _IO(0x12, 94) /* Get read-only status (0 = read_write).  */
#define BLKRRPART  _IO(0x12, 95) /* Re-read partition table.  */
#define BLKGETSIZE _IO(0x12, 96) /* Return device size.  */
#define BLKFLSBUF  _IO(0x12, 97) /* Flush buffer cache.  */
#define BLKRASET   _IO(0x12, 98) /* Set read ahead for block device.  */
#define BLKRAGET   _IO(0x12, 99) /* Get current read ahead setting.  */
#define BLKFRASET  _IO(0x12,100) /* Set filesystem read-ahead.  */
#define BLKFRAGET  _IO(0x12,101) /* Get filesystem read-ahead.  */
#define BLKSECTSET _IO(0x12,102) /* Set max sectors per request.  */
#define BLKSECTGET _IO(0x12,103) /* Get max sectors per request.  */
#define BLKSSZGET  _IO(0x12,104) /* Get block device sector size.  */
#define BLKBSZGET  _IOR(0x12,112,size_t)
#define BLKBSZSET  _IOW(0x12,113,size_t)
#define BLKGETSIZE64 _IOR(0x12,114,size_t) /* return device size.  */


/* Possible value for FLAGS parameter of `umount2'.  */
enum
{
  MNT_FORCE = 1,		/* Force unmounting.  */
#define MNT_FORCE MNT_FORCE
  MNT_DETACH = 2,		/* Just detach from the tree.  */
#define MNT_DETACH MNT_DETACH
  MNT_EXPIRE = 4,		/* Mark for expiry.  */
#define MNT_EXPIRE MNT_EXPIRE
  UMOUNT_NOFOLLOW = 8		/* Don't follow symlink on umount.  */
#define UMOUNT_NOFOLLOW UMOUNT_NOFOLLOW
};


__BEGIN_DECLS

/* Mount a filesystem.  */
extern int mount (const char *__special_file, const char *__dir,
		  const char *__fstype, unsigned long int __rwflag,
		  const void *__data) __THROW;

/* Unmount a filesystem.  */
extern int umount (const char *__special_file) __THROW;

/* Unmount a filesystem.  Force unmounting if FLAGS is set to MNT_FORCE.  */
extern int umount2 (const char *__special_file, int __flags) __THROW;

__END_DECLS

#endif /* _SYS_MOUNT_H */
