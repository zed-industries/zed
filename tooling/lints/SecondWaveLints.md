# Second-wave lint proposals

This document collects candidate lints to add to the `lints` dylint crate.
Each lint follows the conventions established by `notify_in_render`,
`entity_update_in_render`, `shared_string_from_str_literal`, and
`async_block_without_await`: a single module, a positive `ui` test, a
negative `ui` test, simple reporting via `span_lint`, and no
machine-applicable fix.

## Terms

- *late lint*: a pass that runs after type checking, so it can ask
  `cx.typeck_results().expr_ty(...)` for the type of any expression.
- *HIR*: the high-level intermediate representation, the post-parse,
  post-name-resolution tree the existing lints already walk.
- *let-binding*: a `let` statement, distinct from an expression statement
  that drops its value.

## Faster

### 1. `format_single_argument`
- **Pattern.** A `format!` call whose format string is a single bare
  placeholder, with no width, precision, or fill.
  ```rust
  let s = format!("{}", x);
  let t = format!("{x}");
  ```
- **Instead.**
  ```rust
  let s = x.to_string();
  ```
- **Cost.** `format!` allocates a `String` and invokes the formatting
  machinery; `x.to_string()` is direct and often inlined.
- **Detection.** Match `ExprKind::Call` to the `format` macro expansion;
  check that the format string is exactly `"{}"` or `"{ident}"` and there
  is exactly one argument.

### 2. `collect_then_into_iter`
- **Pattern.** An iterator chain that calls `collect` into a `Vec` (or
  `HashMap`) and then immediately walks the collection again with
  `into_iter`/`iter`/`iter_mut`.
  ```rust
  for item in iter.map(transform).collect::<Vec<_>>().into_iter() {
      // ...
  }
  ```
- **Instead.**
  ```rust
  for item in iter.map(transform) {
      // ...
  }
  ```
- **Cost.** Materializes a `Vec` and immediately consumes it.
- **Detection.** A method call `into_iter`/`iter`/`iter_mut` whose
  receiver is a method call `collect` returning `Vec` or `HashMap`.

### 3. `len_in_loop_condition`
- **Pattern.** A loop whose bound is a `len()` call on a collection that
  is not mutated in the body.
  ```rust
  for i in 0..v.len() {
      // ...
  }
  ```
- **Instead.**
  ```rust
  let len = v.len();
  for i in 0..len {
      // ...
  }
  ```
- **Cost.** `len()` is recomputed every iteration on types where it is
  not free, for example `BTreeMap`.
- **Detection.** Limit to types where `len` is non-trivial; skip if the
  loop body contains a method call on `v` whose receiver type permits
  mutation.

### 4. `arc_clone_then_deref`
- **Pattern.** A clone of an `Arc` or `Rc` that is immediately
  dereferenced or method-called and then dropped.
  ```rust
  let value = (*arc.clone()).field;
  arc.clone().method(&payload);
  ```
- **Instead.**
  ```rust
  let value = arc.field;
  arc.method(&payload);
  ```
- **Cost.** Each `Arc::clone` is an atomic increment and decrement.
- **Detection.** Match a method call `clone` on a value of type
  `Arc<_>`/`Rc<_>` whose result is the receiver of a deref or another
  method call and is not stored.

### 5. `vec_with_known_capacity`
- **Pattern.** A `Vec::new()` initializer followed by a contiguous run
  of unconditional `push` calls.
  ```rust
  let mut v = Vec::new();
  v.push(a);
  v.push(b);
  v.push(c);
  ```
- **Instead.**
  ```rust
  let v = vec![a, b, c];
  // or, when each element is computed separately:
  let mut v = Vec::with_capacity(3);
  v.push(a);
  v.push(b);
  v.push(c);
  ```
- **Cost.** Each push past capacity reallocates.
- **Detection.** In a block, find a `let mut v = Vec::new()` followed by
  a contiguous run of `v.push(...)` statements with no intervening
  control flow. Flag if the run length is at least three. Restrict to
  function bodies, not closures, to keep it simple.

### 6. `box_then_arc`
- **Pattern.** An `Arc::new` (or `Arc::from`, `Rc::new`) call whose
  argument is a `Box::new` call.
  ```rust
  let shared = Arc::new(Box::new(x));
  ```
- **Instead.**
  ```rust
  let shared: Arc<T> = Arc::new(x);
  ```
- **Cost.** Two heap allocations where one suffices.
- **Detection.** A call to `Arc::new`/`Arc::from`/`Rc::new` whose
  argument is a call to `Box::new`.

### 7. `string_push_str_literal_one_char`
- **Pattern.** A `push_str` call on `String` whose argument is a string
  literal containing exactly one `char`.
  ```rust
  s.push_str("x");
  ```
- **Instead.**
  ```rust
  s.push('x');
  ```
- **Cost.** `push_str` traverses the slice; `push` writes one `char`
  directly.
- **Detection.** Method `push_str` on `String`, argument is a string
  literal of length one in bytes and UTF-8 codepoints.

### 8. `owned_string_into_shared`
- **Pattern.** A `String::from(<literal>)`, `<literal>.to_string()`, or
  `<literal>.to_owned()` call whose result is immediately converted
  with `.into()` into a refcounted string type such as `SharedString`,
  `Arc<str>`, or `Rc<str>`.
  ```rust
  let label: SharedString = String::from("foo").into();
  let key: Arc<str> = "foo".to_string().into();
  ```
- **Instead.**
  ```rust
  let label = SharedString::new_static("foo");
  let key: Arc<str> = Arc::from("foo");
  ```
- **Cost.** Two heap allocations and two copies of the literal where
  one is enough: `String::from` allocates a `String` and copies the
  bytes, and the `.into()` conversion into an `Arc<str>`-backed type
  allocates an `Arc<str>` and copies the bytes again. For string
  literals the destination can be built directly from `'static` data
  with no allocation at all.
- **Detection.** A method call `into` whose receiver is a call to
  `String::from`, or a method call `to_string`/`to_owned`, where the
  inner call's argument or receiver is a string literal. Resolve the
  `.into()` target type via `cx.typeck_results().expr_ty(...)` and
  restrict to `SharedString`, `Arc<str>`, `Rc<str>`, and `Cow<'_, str>`
  to keep the signal high. Overlaps with
  `shared_string_from_str_literal` only when the target is
  `SharedString`; the cases where the target is `Arc<str>` or `Rc<str>`
  are not covered by the existing lint.

## Less memory

### 9. `string_field_in_pub_struct`
- **Pattern.** A `String` field on a `Clone` type that already has at
  least one `SharedString` field, indicating that the cheap-clone
  representation was used inconsistently.
  ```rust
  #[derive(Clone)]
  struct Item {
      label: SharedString,
      detail: String,
  }
  ```
- **Instead.**
  ```rust
  #[derive(Clone)]
  struct Item {
      label: SharedString,
      detail: SharedString,
  }
  ```
- **Cost.** Each clone copies bytes; `SharedString` and `Arc<str>`
  clones bump a refcount.
- **Detection.** Conservative version: flag only `String` fields in
  types that also have at least one `SharedString` field, signalling
  inconsistency. Mark this as *experimental* and ship it off by default.

### 10. `boxed_closure_called_once`
- **Pattern.** A `Box<dyn Fn(...)>`/`Box<dyn FnMut(...)>` stored in a
  field that is read out with `.take()` and called exactly once.
  ```rust
  struct Job {
      callback: Option<Box<dyn Fn(Result) + Send>>,
  }

  impl Job {
      fn finish(&mut self, result: Result) {
          if let Some(callback) = self.callback.take() {
              callback(result);
          }
      }
  }
  ```
- **Instead.**
  ```rust
  struct Job {
      callback: Option<Box<dyn FnOnce(Result) + Send>>,
  }
  ```
- **Cost.** `FnOnce` allows the captured environment to be moved;
  `Fn`/`FnMut` forces it to be kept by reference, often via `Arc`.
- **Detection.** Hard to do precisely. Skip unless we want a tight,
  false-positive-prone pass.

### 11. `redundant_to_owned`
- **Pattern.** A `to_owned`/`to_string` call whose result is immediately
  borrowed back with `as_str`/`as_ref`/`borrow`.
  ```rust
  let borrowed: &str = original.to_owned().as_str();
  ```
- **Instead.**
  ```rust
  let borrowed: &str = original;
  ```
- **Cost.** Allocates a `String` only to borrow it back.
- **Detection.** Method call whose receiver is `.to_owned()` or
  `.to_string()` and whose name is `as_str`/`as_ref`/`borrow`.

### 12. `vec_of_arc_clone_in_collect`
- **Pattern.** An iterator chain that maps `&Arc<_>`/`&Rc<_>` through a
  closure body of exactly `x.clone()` and collects the result.
  ```rust
  let owned: Vec<Arc<T>> = xs.iter().map(|x| x.clone()).collect();
  ```
- **Instead.**
  ```rust
  // When the source slice is no longer needed:
  let owned: Vec<Arc<T>> = xs;
  // When borrowing without consuming:
  for x in &xs {
      consume(x);
  }
  ```
- **Cost.** Bumps a refcount per element. Often `xs.to_vec()` or
  `xs.iter().cloned().collect()` is intentional, but the
  `.map(|x| x.clone())` form is a common copy-paste.
- **Detection.** A `map` whose closure body is exactly `x.clone()` and
  whose receiver iterator yields `&Arc<_>` or `&Rc<_>`. Already covered
  partially by clippy's `redundant_closure`; restrict to the `Arc`/`Rc`
  case for high signal.

## More correct

### 13. `subscription_not_stored`
- **Pattern.** A `cx.subscribe`/`cx.observe` call whose returned
  `Subscription` is dropped at the end of the statement.
  ```rust
  impl View {
      fn new(other: &Entity<Other>, cx: &mut Context<Self>) -> Self {
          cx.subscribe(other, |this, other, event, cx| {
              // ...
          });
          Self { /* no field for the subscription */ }
      }
  }
  ```
- **Instead.**
  ```rust
  struct View {
      _subscriptions: Vec<Subscription>,
  }

  impl View {
      fn new(other: &Entity<Other>, cx: &mut Context<Self>) -> Self {
          let subscription = cx.subscribe(other, |this, other, event, cx| {
              // ...
          });
          Self { _subscriptions: vec![subscription] }
      }
  }
  ```
- **Cost.** The subscription is unregistered immediately, so the
  callback never fires.
- **Detection.** A `MethodCall` named
  `subscribe`/`observe`/`observe_release`/`subscribe_in`/`observe_in` on
  a receiver of type `gpui::Context<_>` or `&mut gpui::App`. Flag when
  the parent HIR node is a `StmtKind::Semi` or a block tail with `()`
  type.

### 14. `task_dropped_immediately`
- **Pattern.** A `Task` returned from `cx.spawn`/`cx.background_spawn`/
  `cx.spawn_in` that is dropped without `.detach()`,
  `.detach_and_log_err(...)`, `.await`, or being stored.
  ```rust
  cx.spawn(async move |this, cx| {
      // ...
  });
  ```
- **Instead.**
  ```rust
  cx.spawn(async move |this, cx| {
      // ...
  })
  .detach();

  // Or, when the task should be cancellable:
  self.pending = cx.spawn(async move |this, cx| { /* ... */ });
  ```
- **Cost.** The task is cancelled at end of scope, often invisibly.
- **Detection.** Same shape as the previous lint. The receiver type
  check (`gpui::Context`, `gpui::App`, `gpui::AsyncApp`,
  `gpui::AsyncWindowContext`) gives high precision. Skip if the parent
  expression is `.detach`, `.detach_and_log_err`, `.await`, or a `let`
  binding.

### 15. `detach_on_fallible_task`
- **Pattern.** A `.detach()` call on a `Task<Result<_, _>>`.
  ```rust
  cx.spawn(async move |this, cx| -> Result<()> {
      // ...
      Ok(())
  })
  .detach();
  ```
- **Instead.**
  ```rust
  cx.spawn(async move |this, cx| -> Result<()> {
      // ...
      Ok(())
  })
  .detach_and_log_err(cx);
  ```
- **Cost.** Errors from the task are silently discarded.
- **Detection.** Method call `detach` whose receiver type is
  `gpui::Task<T>` with `T = Result<_, _>`. Reuse the result-detection
  helper from `render_helpers.rs`.

### 16. `let_underscore_on_fallible`
- **Pattern.** A `let _ = expr;` binding whose `expr` is a method call
  returning `anyhow::Result<_>` or `core::result::Result<_, _>`.
  ```rust
  let _ = client.request(payload).await?;
  ```
- **Instead.**
  ```rust
  client.request(payload).await?;
  // or, when ignoring the error is intentional:
  client.request(payload).await.log_err();
  ```
- **Cost.** This is the one explicitly called out in the project
  `.rules`. Standard clippy has `let_underscore_must_use`, but it is too
  noisy; a Zed-specific version that only flags `Result` from method
  calls returning anyhow errors would be useful.
- **Detection.** Match a `let` statement with pattern `_` and an
  initializer of type `anyhow::Result<_>` or `core::result::Result<_,
  _>` whose initializer is a method call (not a literal `Ok(())` and so
  on).

### 17. `mutex_guard_held_across_await`
- **Pattern.** A `MutexGuard` (from `parking_lot` or `std::sync`) whose
  scope contains an `.await`.
  ```rust
  let guard = state.lock();
  let result = remote.fetch(&*guard).await?;
  ```
- **Instead.**
  ```rust
  let snapshot = {
      let guard = state.lock();
      guard.clone()
  };
  let result = remote.fetch(snapshot).await?;
  ```
- **Cost.** Deadlocks; also blocks the GPUI foreground thread.
- **Detection.** Walk a function body. For each `let g = mutex.lock()`,
  check whether the same block contains an `.await` before `g` is
  dropped (last use). Existing clippy lint `await_holding_lock` covers
  this for `std::sync::Mutex`. Extend to `parking_lot` and to GPUI's own
  locks if any.

### 18. `smol_timer_in_test`
- **Pattern.** A `smol::Timer::after` call inside an `#[gpui::test]`
  function or a function whose first parameter is
  `&mut TestAppContext`.
  ```rust
  #[gpui::test]
  async fn test_thing(cx: &mut TestAppContext) {
      smol::Timer::after(Duration::from_millis(50)).await;
      cx.run_until_parked();
  }
  ```
- **Instead.**
  ```rust
  #[gpui::test]
  async fn test_thing(cx: &mut TestAppContext) {
      cx.background_executor.timer(Duration::from_millis(50)).await;
      cx.run_until_parked();
  }
  ```
- **Cost.** Stated in the project `.rules`: timers not tracked by the
  GPUI dispatcher break `run_until_parked`.
- **Detection.** Path expression resolving to `smol::Timer::after`, in a
  function whose attributes contain `gpui::test` or whose signature
  mentions `TestAppContext`/`VisualTestContext`.

### 19. `outer_cx_in_async_closure`
- **Pattern.** A path expression named `cx` inside a `cx.spawn` or
  `entity.update` closure that resolves to an outer binding rather than
  the closure's own `cx` parameter.
  ```rust
  cx.spawn(async move |this, cx| {
      // Bug: `cx` here refers to the captured outer cx, not the parameter.
      this.update(&mut outer_cx, |this, cx| this.refresh(cx))?;
  })
  .detach();
  ```
- **Instead.**
  ```rust
  cx.spawn(async move |this, cx| {
      // Use the inner `cx` for all entity updates:
      this.update(cx, |this, cx| {
          this.refresh(cx);
      })?;
  })
  .detach();
  ```
- **Cost.** The project `.rules` warn that this leads to multiple-borrow
  panics.
- **Detection.** Find closures whose first or second parameter is named
  `cx`. Walk the body. Flag any path expression resolving to a binding
  named `cx` whose `HirId` lies outside the closure. Refine by checking
  the type is `&mut Context<_>` or similar.

### 20. `recursive_entity_update`
- **Pattern.** A nested `entity.update(...)` call whose receiver
  resolves to the same local binding as an enclosing `update`.
  ```rust
  entity.update(cx, |this, cx| {
      this.first_step(cx);
      entity.update(cx, |this, cx| this.second_step(cx));
  });
  ```
- **Instead.**
  ```rust
  entity.update(cx, |this, cx| {
      this.first_step(cx);
      this.second_step(cx);
  });
  ```
- **Cost.** Panics at runtime.
- **Detection.** When matching a method call `update` on a
  `gpui::Entity<_>` receiver, walk the closure body for another `update`
  whose receiver resolves to the same local. Use
  `clippy_utils::path_to_local` or similar.

### 21. `dispatch_action_unboxed`
- **Pattern.** A `window.dispatch_action` call whose first argument is a
  bare action value rather than a `Box<dyn Action>`.
  ```rust
  window.dispatch_action(SomeAction, cx);
  ```
- **Instead.**
  ```rust
  window.dispatch_action(SomeAction.boxed_clone(), cx);
  ```
- **Cost.** Compiles only when the API takes `impl Action`; mixing
  usages causes inconsistent dispatch behaviour.
- **Detection.** Method call `dispatch_action` whose first argument's
  type is not `Box<dyn gpui::Action>`. Skip if it is `&` or already
  boxed.

### 22. `set_text_in_render`
- **Pattern.** A named mutator like `set_text`, `set_placeholder_text`,
  or `set_content` called directly inside a `Render::render` body.
  ```rust
  impl Render for InputField {
      fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
          self.editor.update(cx, |editor, cx| editor.set_text(&self.value, cx));
          div().child(self.editor.clone())
      }
  }
  ```
- **Instead.**
  ```rust
  // Mutate state outside of render (e.g. in an event handler):
  fn on_input_change(&mut self, value: &str, _: &mut Window, cx: &mut Context<Self>) {
      self.editor.update(cx, |editor, cx| editor.set_text(value, cx));
  }

  // The render method only reads:
  fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
      div().child(self.editor.clone())
  }
  ```
- **Cost.** Same family as `entity_update_in_render`, but for direct
  method calls rather than `update`.
- **Detection.** Method call whose name is in a small allowlist
  (`set_text`, `set_placeholder_text`, `set_content`, ...) and is
  `is_directly_in_render_method`. The allowlist keeps it simple; we
  accept missing other mutators.

## Less prone to errors

### 23. `unwrap_in_non_test_code`
- **Pattern.** A `.unwrap()` or `.expect(...)` call outside
  `#[cfg(test)]` modules and `#[gpui::test]` functions.
  ```rust
  fn load(path: &Path) -> Settings {
      let value = maybe_value.unwrap();
      parse(value).expect("settings must parse")
  }
  ```
- **Instead.**
  ```rust
  let value = maybe_value.context("missing value")?;
  ```
- **Cost.** Panics in production code.
- **Detection.** Method call `unwrap`/`expect` whose enclosing function
  or module does not have a `cfg(test)` attribute and is not in a file
  under a `tests/` directory.

### 24. `slice_index_without_get`
- **Pattern.** A `[i]` index expression on a slice or `Vec` where `i` is
  not a literal constant.
  ```rust
  let item = xs[i];
  ```
- **Instead.**
  ```rust
  let Some(item) = xs.get(i) else {
      return;
  };
  ```
- **Cost.** Panics on out-of-bounds; the `.rules` flag this.
- **Detection.** An index expression whose receiver type is a slice or
  `Vec` and whose index is not a literal.

### 25. `float_equality`
- **Pattern.** An equality or inequality comparison whose operands are
  both `f32` or both `f64`.
  ```rust
  if a == b {
      // ...
  }
  ```
- **Instead.**
  ```rust
  const EPSILON: f64 = 1e-9;
  if (a - b).abs() < EPSILON {
      // ...
  }
  ```
- **Cost.** Floating-point arithmetic produces values that compare
  unequal even when the underlying real-valued result is the same.
- **Detection.** A binary expression with `==` or `!=` and both operand
  types `f32`/`f64`. Standard clippy already has `float_cmp`; we ship
  our own only if we want a stricter variant.

### 26. `std_mutex_in_workspace`
- **Pattern.** Use of `std::sync::Mutex` or `std::sync::RwLock` in a
  workspace crate.
  ```rust
  use std::sync::Mutex;

  let state = Mutex::new(State::default());
  let guard = state.lock().unwrap();
  ```
- **Instead.**
  ```rust
  use parking_lot::Mutex;

  let state = Mutex::new(State::default());
  let guard = state.lock();
  ```
- **Cost.** Zed uses `parking_lot` throughout for performance and
  uniformity.
- **Detection.** Path expression resolving to `std::sync::Mutex::new`
  and so on. Skip in `tooling/`.

### 27. `panic_macro_in_non_test_code`
- **Pattern.** A `panic!`, `todo!`, or `unimplemented!` invocation in a
  non-test function or module.
  ```rust
  fn handle(state: &State) {
      if !state.is_supported() {
          panic!("unsupported state: {state:?}");
      }
  }
  ```
- **Instead.**
  ```rust
  return Err(anyhow!("unsupported state: {state:?}"));
  ```
- **Cost.** Aborts the process instead of surfacing the error to the
  caller.
- **Detection.** A call expression with the macro span; rule out
  test-marked functions and modules.

## Suggested first batch

Of the above, five align most directly with the project `.rules` and
have low false-positive rates:

1. `subscription_not_stored`
2. `task_dropped_immediately`
3. `detach_on_fallible_task`
4. `smol_timer_in_test`
5. `set_text_in_render`

These are all bugs the `.rules` already warn humans about, which is
evidence that the patterns recur and that the rules are specific enough
to act on. Each fits `LintRULES.md`: a single module, a positive `ui`
test, a negative `ui` test, simple reporting, and no machine-applicable
fix.
