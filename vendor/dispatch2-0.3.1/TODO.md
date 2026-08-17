# TODO

- Fully document all ffi APIs
- CI test on macOS
- CI test on Linux using https://github.com/apple/swift-corelibs-libdispatch
- CI test on Windows using https://github.com/apple/swift-corelibs-libdispatch
- Safe wrapper for ``dispatch_source_*`` + ``set_target_queue/activate/suspend/resume`` for it
- Safe wrapper for ``dispatch_data_*``
- Safe wrapper for ``dispatch_get_context/dispatch_set_context`` (quite impossible without big overhead => wrap dispatch object destructor to release the boxed value)
- All blocks related bindings and ``dispatch_block_*`` functions with compat with ``block2`` on Apple platforms.
- Integrate conversion from SystemTime to dispatch_time_t via dispatch_walltime and safe APIs using that.
- `dispatch/introspection.h`


NOTES:
- suspended object must not be released
  - Closure?
  - Resume drop guard (that holds a reference count, ensuring not releasing even if leaked)
  - Activation: "Releasing the last reference count on an inactive object is undefined."
- Rename `DispatchQoS` etc. to fit Swift naming

- Dispatch blocks
