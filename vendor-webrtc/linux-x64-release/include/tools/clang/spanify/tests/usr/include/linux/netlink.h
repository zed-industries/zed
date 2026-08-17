// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#ifndef TOOLS_CLANG_SPANIFY_TESTS_USR_INCLUDE_LINUX_NETLINK_H_
#define TOOLS_CLANG_SPANIFY_TESTS_USR_INCLUDE_LINUX_NETLINK_H_

#include <cstdint>

/**
 * struct nlmsghdr - fixed format metadata header of Netlink messages
 * @nlmsg_len:   Length of message including header
 * @nlmsg_type:  Message content type
 * @nlmsg_flags: Additional flags
 * @nlmsg_seq:   Sequence number
 * @nlmsg_pid:   Sending process port ID
 */
struct nlmsghdr {
  uint32_t nlmsg_len;
  uint16_t nlmsg_type;
  uint16_t nlmsg_flags;
  uint32_t nlmsg_seq;
  uint32_t nlmsg_pid;
};

#define NLMSG_ALIGNTO 4U
#define NLMSG_ALIGN(len) (((len) + NLMSG_ALIGNTO - 1) & ~(NLMSG_ALIGNTO - 1))
#define NLMSG_HDRLEN ((int)NLMSG_ALIGN(sizeof(struct nlmsghdr)))
#define NLMSG_DATA(nlh) ((void*)(((char*)nlh) + NLMSG_HDRLEN))

#endif  // TOOLS_CLANG_SPANIFY_TESTS_USR_INCLUDE_LINUX_NETLINK_H_
