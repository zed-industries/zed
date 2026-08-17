(module
  (import "foo:dep/the-name" "a" (func))
  (import "foo:foo/the-name" "a" (func))
  (func (export "foo:foo/the-name#a"))
)
