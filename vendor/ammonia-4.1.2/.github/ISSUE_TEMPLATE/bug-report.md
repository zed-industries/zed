---
name: Bug Report
about: Create a report to help us improve
title: ''
labels: ''
assignees: ''

---

- [ ] If you used a custom Builder configuration, please supply it here

    ```rust
    Builder::default()
        .link_rel(None)
        .url_relative(UrlRelative::PassThrough)
    ```

  - [ ] Please supply a reduced version of the *input* HTML that you used with Ammonia

    ```html
    fill <html> in here
    ```

  - [ ] If ammonia produced unexpected HTML

      - [ ] Try opening the *input* HTML directly in a WHATWG web browser (like Safari, Edge, Chrome, or Firefox) without sanitizing it.

        If the browser parses it the same way Ammonia does, then it's not a bug.

      - [ ] Supply the reduced *output* HTML here

        ```html
        output <html> here
        ```

  - [ ] If Ammonia panics, please provide a backtrace

    ```text
    $ RUST_BACKTRACE=1 ./test
    thread 'main' panicked at test.rs:1:17:
    explicit panic
    stack backtrace:
       0: std::panicking::begin_panic
       1: test::other_fn
       2: test::main
       3: core::ops::function::FnOnce::call_once
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    ```
