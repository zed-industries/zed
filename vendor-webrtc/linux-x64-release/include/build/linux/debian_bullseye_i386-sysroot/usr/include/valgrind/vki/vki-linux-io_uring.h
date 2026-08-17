/*
   This file is part of Valgrind, a dynamic binary instrumentation framework.

   Copyright (C) 2019 Bart Van Assche <bvanassche@acm.org>

   This program is free software; you can redistribute it and/or
   modify it under the terms of the GNU General Public License as
   published by the Free Software Foundation; either version 2 of the
   License, or (at your option) any later version.

   This program is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, see <http://www.gnu.org/licenses/>.

   The GNU General Public License is contained in the file COPYING.
*/

#ifndef _VKI_IO_URING_H_
#define _VKI_IO_URING_H_

// Derived from linux-5.2/include/uapi/linux/io_uring.h */

/*
 * IO submission data structure (Submission Queue Entry)
 */
struct vki_io_uring_sqe {
	__vki_u8	opcode;		/* type of operation for this sqe */
	__vki_u8	flags;		/* IOSQE_ flags */
	__vki_u16	ioprio;		/* ioprio for the request */
	__vki_s32	fd;		/* file descriptor to do IO on */
	__vki_u64	off;		/* offset into file */
	__vki_u64	addr;		/* pointer to buffer or iovecs */
	__vki_u32	len;		/* buffer size or number of iovecs */
	union {
		int		rw_flags;
		__vki_u32	fsync_flags;
		__vki_u16	poll_events;
		__vki_u32	sync_range_flags;
		__vki_u32	msg_flags;
	};
	__vki_u64	user_data;	/* data to be passed back at completion time */
	union {
		__vki_u16	buf_index;	/* index into fixed buffers, if used */
		__vki_u64	__pad2[3];
	};
};

/*
 * sqe->flags
 */
#define VKI_IOSQE_FIXED_FILE	(1U << 0)	/* use fixed fileset */
#define VKI_IOSQE_IO_DRAIN	(1U << 1)	/* issue after inflight IO */
#define VKI_IOSQE_IO_LINK	(1U << 2)	/* links next sqe */

/*
 * io_uring_setup() flags
 */
#define VKI_IORING_SETUP_IOPOLL	(1U << 0)	/* io_context is polled */
#define VKI_IORING_SETUP_SQPOLL	(1U << 1)	/* SQ poll thread */
#define VKI_IORING_SETUP_SQ_AFF	(1U << 2)	/* sq_thread_cpu is valid */

#define VKI_IORING_OP_NOP		0
#define VKI_IORING_OP_READV		1
#define VKI_IORING_OP_WRITEV	2
#define VKI_IORING_OP_FSYNC		3
#define VKI_IORING_OP_READ_FIXED	4
#define VKI_IORING_OP_WRITE_FIXED	5
#define VKI_IORING_OP_POLL_ADD	6
#define VKI_IORING_OP_POLL_REMOVE	7
#define VKI_IORING_OP_SYNC_FILE_RANGE	8
#define VKI_IORING_OP_SENDMSG	9
#define VKI_IORING_OP_RECVMSG	10

/*
 * sqe->fsync_flags
 */
#define VKI_IORING_FSYNC_DATASYNC	(1U << 0)

/*
 * IO completion data structure (Completion Queue Entry)
 */
struct vki_io_uring_cqe {
	__vki_u64	user_data;	/* sqe->data submission passed back */
	__vki_s32	res;		/* result code for this event */
	__vki_u32	flags;
};

/*
 * Magic offsets for the application to mmap the data it needs
 */
#define VKI_IORING_OFF_SQ_RING		0ULL
#define VKI_IORING_OFF_CQ_RING		0x8000000ULL
#define VKI_IORING_OFF_SQES		0x10000000ULL

/*
 * Filled with the offset for mmap(2)
 */
struct vki_io_sqring_offsets {
	__vki_u32 head;
	__vki_u32 tail;
	__vki_u32 ring_mask;
	__vki_u32 ring_entries;
	__vki_u32 flags;
	__vki_u32 dropped;
	__vki_u32 array;
	__vki_u32 resv1;
	__vki_u64 resv2;
};

/*
 * sq_ring->flags
 */
#define VKI_IORING_SQ_NEED_WAKEUP	(1U << 0) /* needs io_uring_enter wakeup */

struct vki_io_cqring_offsets {
	__vki_u32 head;
	__vki_u32 tail;
	__vki_u32 ring_mask;
	__vki_u32 ring_entries;
	__vki_u32 overflow;
	__vki_u32 cqes;
	__vki_u64 resv[2];
};

/*
 * io_uring_enter(2) flags
 */
#define VKI_IORING_ENTER_GETEVENTS	(1U << 0)
#define VKI_IORING_ENTER_SQ_WAKEUP	(1U << 1)

/*
 * Passed in for io_uring_setup(2). Copied back with updated info on success
 */
struct vki_io_uring_params {
	__vki_u32 sq_entries;
	__vki_u32 cq_entries;
	__vki_u32 flags;
	__vki_u32 sq_thread_cpu;
	__vki_u32 sq_thread_idle;
	__vki_u32 resv[5];
	struct vki_io_sqring_offsets sq_off;
	struct vki_io_cqring_offsets cq_off;
};

/*
 * io_uring_register(2) opcodes and arguments
 */
#define VKI_IORING_REGISTER_BUFFERS	0
#define VKI_IORING_UNREGISTER_BUFFERS	1
#define VKI_IORING_REGISTER_FILES	2
#define VKI_IORING_UNREGISTER_FILES	3
#define VKI_IORING_REGISTER_EVENTFD	4
#define VKI_IORING_UNREGISTER_EVENTFD	5

#endif
