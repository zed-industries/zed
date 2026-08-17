// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_TASK_SCHEDULER_TASK_TRAITS_ANDROID_H_
#define BASE_ANDROID_TASK_SCHEDULER_TASK_TRAITS_ANDROID_H_

// Enum for the TaskTraits types exposed to Java.
//
// A Java counterpart will be generated for this enum.
// GENERATED_JAVA_ENUM_PACKAGE: org.chromium.base.task
enum TaskTraits {
  THREAD_POOL_TRAITS_START = 0,
  // This task will only be scheduled when machine resources are available. Once
  // running, it may be descheduled if higher priority work arrives (in this
  // process or another) and its running on a non-critical thread. This is the
  // lowest possible priority.
  BEST_EFFORT = THREAD_POOL_TRAITS_START,
  // This is a lowest-priority task which may block, for example non-urgent
  // logging or deletion of temporary files as clean-up.
  BEST_EFFORT_MAY_BLOCK = THREAD_POOL_TRAITS_START + 1,
  // This task affects UI or responsiveness of future user interactions. It is
  // not an immediate response to a user interaction. Most tasks are likely to
  // have this priority.
  // Examples:
  // - Updating the UI to reflect progress on a long task.
  // - Loading data that might be shown in the UI after a future user
  //   interaction.
  USER_VISIBLE = THREAD_POOL_TRAITS_START + 2,
  // USER_VISIBLE + may block.
  USER_VISIBLE_MAY_BLOCK = THREAD_POOL_TRAITS_START + 3,
  // This task affects UI immediately after a user interaction.
  // Example: Generating data shown in the UI immediately after a click.
  USER_BLOCKING = THREAD_POOL_TRAITS_START + 4,
  // USER_BLOCKING + may block.
  USER_BLOCKING_MAY_BLOCK = THREAD_POOL_TRAITS_START + 5,
  THREAD_POOL_TRAITS_END = USER_BLOCKING_MAY_BLOCK,
  UI_TRAITS_START = THREAD_POOL_TRAITS_END + 1,
  UI_BEST_EFFORT = UI_TRAITS_START,
  UI_USER_VISIBLE = UI_TRAITS_START + 1,
  UI_USER_BLOCKING = UI_TRAITS_START + 2,
  UI_DEFAULT = UI_USER_VISIBLE,
  UI_TRAITS_END = UI_USER_BLOCKING
};

#endif  // BASE_ANDROID_TASK_SCHEDULER_TASK_TRAITS_ANDROID_H_
