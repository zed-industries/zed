# `wasmtime-unwinder`

This crate implements an unwind info format, stack walking, and
unwinding for Wasmtime. It includes logic that:

- Can walk the Wasmstack and visit each frame;
- Can find exception handlers using an efficient format serialized
  from Cranelift compilation metadata that can be mapped and used
  in-place from disk;
- Provides a "throw" helper that, when called from host code that has
  been invoked from Wasmcode, can find a handler; and a "resume" stub
  that can be invoked to transfer control to the corresponding
  handler.
