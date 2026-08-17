//===-- Spawn file actions  -------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SPAWN_FILE_ACTIONS_H
#define LLVM_LIBC_SRC_SPAWN_FILE_ACTIONS_H

#include "src/__support/macros/config.h"
#include <spawn.h> // For mode_t
#include <stdint.h>

namespace LIBC_NAMESPACE_DECL {

struct BaseSpawnFileAction {
  enum ActionType {
    OPEN = 111,
    CLOSE = 222,
    DUP2 = 333,
  };

  ActionType type;
  BaseSpawnFileAction *next;

  static void add_action(posix_spawn_file_actions_t *actions,
                         BaseSpawnFileAction *act) {
    if (actions->__back != nullptr) {
      auto *back = reinterpret_cast<BaseSpawnFileAction *>(actions->__back);
      back->next = act;
      actions->__back = act;
    } else {
      // First action is being added.
      actions->__front = actions->__back = act;
    }
  }

protected:
  explicit BaseSpawnFileAction(ActionType t) : type(t), next(nullptr) {}
};

struct SpawnFileOpenAction : public BaseSpawnFileAction {
  const char *path;
  int fd;
  int oflag;
  mode_t mode;

  SpawnFileOpenAction(const char *p, int fdesc, int flags, mode_t m)
      : BaseSpawnFileAction(BaseSpawnFileAction::OPEN), path(p), fd(fdesc),
        oflag(flags), mode(m) {}
};

struct SpawnFileCloseAction : public BaseSpawnFileAction {
  int fd;

  SpawnFileCloseAction(int fdesc)
      : BaseSpawnFileAction(BaseSpawnFileAction::CLOSE), fd(fdesc) {}
};

struct SpawnFileDup2Action : public BaseSpawnFileAction {
  int fd;
  int newfd;

  SpawnFileDup2Action(int fdesc, int new_fdesc)
      : BaseSpawnFileAction(BaseSpawnFileAction::DUP2), fd(fdesc),
        newfd(new_fdesc) {}
};

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SPAWN_FILE_ACTIONS_H
