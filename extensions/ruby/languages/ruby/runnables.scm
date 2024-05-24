; Adapted from the following sources:
; Minitest: https://github.com/zidhuss/neotest-minitest/blob/main/lua/neotest-minitest/init.lua
; RSpec: https://github.com/olimorris/neotest-rspec/blob/main/lua/neotest-rspec/init.lua

; Minitest
;; Rails unit tests
(
    (class
        name: [
          (constant) @run
          (scope_resolution scope: (constant) name: (constant) @run)
        ]
        (superclass (scope_resolution) @superclass (#match? @superclass "(::IntegrationTest|::TestCase|::SystemTestCase)$"))
    ) @minitest-test
    (#set! tag minitest-test)
)

(
    (call
        method: (identifier) @run (#eq? @run "test")
        arguments: (argument_list (string (string_content) @name))
    ) @minitest-test
    (#set! tag minitest-test)
)

; Methods that begin with test_
(
    (method
        name: (identifier) @run (#match? @run "^test_")
    ) @minitest-test
    (#set! tag minitest-test)
)

; System tests that inherit from ApplicationSystemTestCase
(
    (class
        name: (constant) @run (superclass) @superclass (#match? @superclass "(ApplicationSystemTestCase)$")
    ) @minitest-test
    (#set! tag minitest-test)
)

; RSpec

; Example groups with literals
(
    (call
       method: (identifier) @run (#any-of? @run "describe" "context")
       arguments: (argument_list . (_) @name)
    ) @rspec-test
    (#set! tag rspec-test)
)

; Examples
(
    (call
        method: (identifier) @run (#any-of? @run "it" "its" "specify")
        arguments: (argument_list (string (string_content) @name))
    ) @rspec-test
    (#set! tag rspec-test)
)

; Examples (one-liner syntax)
(
    (call
        method: (identifier) @run (#any-of? @run "it" "its" "specify")
        block: (_) @name
        !arguments
    ) @rspec-test
    (#set! tag rspec-test)
)
