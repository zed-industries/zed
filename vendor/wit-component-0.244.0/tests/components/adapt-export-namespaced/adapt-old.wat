(module
  (import "__main_module__" "the_entrypoint" (func $entry))
  (export "foo:foo/new#entrypoint" (func $entry))
)
