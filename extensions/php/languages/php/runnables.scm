; Class that follow the naming convention of PHPUnit test classes
; and that doesn't have the abstract modifier
; and have a method that follow the naming convention of PHPUnit test methods
; and the method is public
(
    (class_declaration
        modifier: (_)? @_modifier
        (#not-eq? @_modifier "abstract")
        name: (_) @_name
        (#match? @_name ".*Test$")
        body: (declaration_list
            (method_declaration
                (visibility_modifier)? @_visibility
                (#eq? @_visibility "public")
                name: (_) @run
                (#match? @run "^test.*")
            )
        )
    ) @phpunit-test
    (#set! tag phpunit-test)
)

; Class that follow the naming convention of PHPUnit test classes
; and that doesn't have the abstract modifier
; and have a method that has the @test annotation
; and the method is public
(
    (class_declaration
        modifier: (_)? @_modifier
        (#not-eq? @_modifier "abstract")
        name: (_) @_name
        (#match? @_name ".*Test$")
        body: (declaration_list
            ((comment) @_comment
                (#match? @_comment ".*@test\\b.*")
            .
            (method_declaration
                (visibility_modifier)? @_visibility
                (#eq? @_visibility "public")
                name: (_) @run
                (#not-match? @run "^test.*")
            ))
        )
    ) @phpunit-test
    (#set! tag phpunit-test)
)

; Class that follow the naming convention of PHPUnit test classes
; and that doesn't have the abstract modifier
; and have a method that has the #[Test] attribute
; and the method is public
(
    (class_declaration
        modifier: (_)? @_modifier
        (#not-eq? @_modifier "abstract")
        name: (_) @_name
        (#match? @_name ".*Test$")
        body: (declaration_list
            (method_declaration
                (attribute_list
                    (attribute_group
                        (attribute (name) @_attribute)
                    )
                )
                (#eq? @_attribute "Test")
                (visibility_modifier)? @_visibility
                (#eq? @_visibility "public")
                name: (_) @run
                (#not-match? @run "^test.*")
            )
        )
    ) @phpunit-test
    (#set! tag phpunit-test)
)

; Class that follow the naming convention of PHPUnit test classes
; and that doesn't have the abstract modifier
(
    (class_declaration
        modifier: (_)? @_modifier
        (#not-eq? @_modifier "abstract")
        name: (_) @run
        (#match? @run ".*Test$")
    ) @phpunit-test
    (#set! tag phpunit-test)
)

; Add support for Pest runnable
; Function expression that has `it`, `test` or `describe` as the function name
(
    (function_call_expression
        function: (_) @_name
        (#any-of? @_name "it" "test" "describe")
        arguments: (arguments
            .
            (argument
                [
                  (encapsed_string (string_value) @run)
                  (string (string_value) @run)
                ]
            )
        )
    ) @pest-test
    (#set! tag pest-test)
)
