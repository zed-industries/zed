; Adapted from the following sources:
; Minitest: https://github.com/zidhuss/neotest-minitest/blob/main/lua/neotest-minitest/init.lua
; RSpec: https://github.com/olimorris/neotest-rspec/blob/main/lua/neotest-rspec/init.lua

; Tests that inherit from a specific class
(
    (class
        name: [
          (constant) @run
          (scope_resolution scope: (constant) name: (constant) @run)
        ]
        (superclass (scope_resolution) @superclass (#match? @superclass "(::IntegrationTest|::TestCase|::SystemTestCase|Minitest::Test|TLDR)$"))
    ) @_ruby-test
    (#set! tag ruby-test)
)

(
    (call
        method: (identifier) @run (#eq? @run "test")
        arguments: (argument_list (string (string_content) @_name))
    ) @_ruby-test
    (#set! tag ruby-test)
)

; Methods that begin with test_
(
    (method
        name: (identifier) @run (#match? @run "^test_")
    ) @_ruby-test
    (#set! tag ruby-test)
)

; System tests that inherit from ApplicationSystemTestCase
(
    (class
        name: (constant) @run (superclass) @superclass (#match? @superclass "(ApplicationSystemTestCase)$")
    ) @_ruby-test
    (#set! tag ruby-test)
)

; Examples
(
    (call
       method: (identifier) @run (#any-of? @run "describe" "context" "it" "its" "specify")
       arguments: (argument_list . (_) @_name)
    ) @_ruby-test
    (#set! tag ruby-test)
)

; Examples (one-liner syntax)
(
    (call
        method: (identifier) @run (#any-of? @run "it" "its" "specify")
        block: (_) @_name
        !arguments
    ) @_ruby-test
    (#set! tag ruby-test)
)
