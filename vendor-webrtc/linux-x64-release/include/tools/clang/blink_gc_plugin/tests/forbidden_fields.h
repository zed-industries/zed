// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_BLINK_GC_PLUGIN_TESTS_FORBIDDEN_FIELDS_H_
#define TOOLS_CLANG_BLINK_GC_PLUGIN_TESTS_FORBIDDEN_FIELDS_H_

#include "heap/stubs.h"

namespace blink {

template <typename T>
class TaskRunnerTimer {};

class SecondLevelPartObject {
 private:
  TaskRunnerTimer<SecondLevelPartObject> timer_;
};

class FirstLevelPartObject {
 private:
  SecondLevelPartObject obj_;
  std::map<int, SecondLevelPartObject> map_of_embedded;
};

class HeapObject : public GarbageCollected<HeapObject> {
 private:
  FirstLevelPartObject obj_;
};

class AnotherHeapObject : public GarbageCollected<AnotherHeapObject> {
 private:
  TaskRunnerTimer<AnotherHeapObject> timer_;
  Vector<TaskRunnerTimer<AnotherHeapObject>> vec_of_timers_;
  Vector<SecondLevelPartObject> vec_of_embedded_of_timers;
  TaskRunnerTimer<AnotherHeapObject> array_of_bad_typ_e[2];
  SecondLevelPartObject array_of_embedded_object_[2];
  std::vector<TaskRunnerTimer<AnotherHeapObject>> std_vec_of_timers_;
  std::optional<TaskRunnerTimer<AnotherHeapObject>> optional_timer_;
  std::optional<TaskRunnerTimer<AnotherHeapObject>> optional_timer2_;
  std::optional<SecondLevelPartObject> optional_embedded_object_;
  std::optional<SecondLevelPartObject> optional_embedded_object2_;
  absl::variant<SecondLevelPartObject> variant_embedded_object_;
  std::variant<SecondLevelPartObject> variant_embedded_object2_;
  std::unique_ptr<TaskRunnerTimer<AnotherHeapObject>> unique_ptr_timer_;
  TaskRunnerTimer<AnotherHeapObject>* raw_ptr_timer_;
  scoped_refptr<TaskRunnerTimer<AnotherHeapObject>> scoped_refptr_timer_;
  base::WeakPtr<TaskRunnerTimer<AnotherHeapObject>> weak_ptr_timer_;
};

}  // namespace blink

#endif /* TOOLS_CLANG_BLINK_GC_PLUGIN_TESTS_FORBIDDEN_FIELDS_H_ */
