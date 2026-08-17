// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUNCTIONAL_DISALLOW_UNRETAINED_H_
#define BASE_FUNCTIONAL_DISALLOW_UNRETAINED_H_

// IMPORTANT: this is currently experimental. Use with caution, as the
// interaction with various base APIs is still unstable and subject to change.
//
// Types can opt to forbid use of `Unretained()`, et cetera by using this macro
// to annotate their class definition:
//
// class Dangerous {
//   DISALLOW_UNRETAINED();
//  public:
//
//   ...
//
//   void PostAsyncWork() {
//     // Will not compile.
//     task_runner_->PostTask(
//         FROM_HERE,
//         base::BindOnce(&Dangerous::OnAsyncWorkDone, base::Unretained(this)));
//   }
//
//   void OnAsyncWorkDone() {
//     ...
//   }
// };
//
// A type that disallows use of base::Unretained() can still be used with
// callback:
//
// - If a type is only used on one sequence (e.g. `content::RenderFrameHostImpl`
//   may only be used on the UI thread), embed a `base::WeakPtrFactory<T>` and
//   either use:
//
//   - `GetSafeRef()` to bind a `SafeRef<T>` if `this` must *always* still be
//     alive when the callback is invoked, e.g. binding Mojo reply callbacks
//     when making Mojo calls through a `mojo::Remote` owned by `this`.
//
//   - `GetWeakPtr()` to bind a `WeakPtr<T>` if the lifetimes are unclear, e.g.
//     a task posted to main UI task runner, and a strong lifetime assertion is
//     not possible.
//
//   - Note 1: use `WeakPtr<T>` only when appropriate. `WeakPtr<T>` makes it
//     harder to reason about lifetimes; while it is necessary and appropriate
//     in many places, using it unnecessarily makes it hard to understand when
//     one object is guaranteed to outlive another.
//
//   - Note 2: whether `GetSafeRef()` or `GetWeakPtr()` is used, include
//     comments to explain the assumptions behind the selection. Though these
//     comments may become inaccurate over time, they are still valuable
//     to helping when reading unfamiliar code.
//
// - If a type is used on multiple sequences, make it refcounted and either bind
//   a `scoped_refptr<t>` or use `base::RetainedRef()`.
//
// - Consider if callbacks are needed at all; using abstractions like
//   `base::SequenceBound<T>` make it much easier to manage cross-sequence
//   lifetimes and avoid the need to write `base::Unretained()` at all.
#define DISALLOW_UNRETAINED()                                        \
 public:                                                             \
  using DisallowBaseUnretainedMarker [[maybe_unused]] = void;        \
                                                                     \
 private:                                                            \
  /* No-op statement so use of this macro can be followed by `;`. */ \
  static_assert(true)

#endif  // BASE_FUNCTIONAL_DISALLOW_UNRETAINED_H_
