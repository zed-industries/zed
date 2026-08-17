# Version 2.4.1

- Update to events-listener 4.0

# Version 2.4.0

- Update to async-io 2.0

# Version 2.3.1

- Update blocking dependency

# Version 2.3.0

- Switch back to edition 2021 and MSRV 1.59, dropping num-cups dependency

# Version 2.2.0

- Revert back to edition 2018 and MSRV 1.49

# Version 2.1.0

- Switch from async-mutex to async-lock
- Switch from num-cpus to Use `std::thread::available_parallelism`
- Update MSRV to 1.59
- Update to edition 2021

# Version 2.0.4

- Return concrete type Task from `spawn_blocking`

# Version 2.0.3

- Documentation updates

# Version 2.0.2

- Documentation updates

# Version 2.0.1

- fix build without default features

# Version 2.0.0

- add tokio 1.0 integration
- rework configuration
- add a way to update the number of threads at runtime within configured bounds

# Version 1.4.3

- switch to multi threaded tokio schedulers when enabled

# Version 1.4.2

- Drop an Arc

# Version 1.4.1

- switch back to manual implementation for tokio02 integration

# Version 1.4.0

- add tokio03 integration

# Version 1.3.0

- use async-compat for tokio02 integration

# Version 1.2.1

- tokio02 fix

# Version 1.2.0

- Add tokio02 feature

# Version 1.1.1

- Update `async-executor`.

# Version 1.1.0

- Update async-executor

# Version 1.0.2

- Do not run global tasks in `block_on()`

# Version 1.0.1

- Update dependencies

# Version 1.0.0

- Update dependencies
- Make async-io support optional

# Version 0.2.3

- Change license to MIT or Apache-2.0

# Version 0.2.2

- Reexport `async_executor::Task`

# Version 0.2.1

- Make sure we spawn at least one thread

# Version 0.2.0

- Rename `run` to `block_on` and drop `'static` requirement
- Add `GlobalExecutorConfig::with_thread_name`

# Version 0.1.4

- Add init functions

# Version 0.1.3

- `run`: do not require `Future` to be `Send`

# Version 0.1.2

- Adjust dependencies

# Versio 0.1.1

- Fix the number of spawned threads

# Version 0.1.0

- Initial release
