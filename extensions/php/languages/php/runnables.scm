(
    ; Class that follow the naming convention of PHPUnit test classes
    ; and that doesn't have the abstract modifier
    ; and have a method that follow the naming convention of PHPUnit test methods
    ; and the method is public
    (class_declaration
        modifier: (_)? @modifier
        (#not-eq? @modifier "abstract")
        name: (_) @name
        (#match? @name ".*Test$")
        body: (declaration_list
            (method_declaration
                (visibility_modifier)? @visibility
                (#eq? @visibility "public")
                name: (_) @run
                (#match? @run "^test.*")
            )
        )
    )
) @phpunit-test

; Class that follow the naming convention of PHPUnit test classes
; and that doesn't have the abstract modifier
; and have a method that has the @test annotation
; and the method is public
(
    (class_declaration
        modifier: (_)? @modifier
        (#not-eq? @modifier "abstract")
        name: (_) @name
        (#match? @name ".*Test$")
        body: (declaration_list
            ((comment) @comment
                (#match? @comment ".*@test\\b.*")
            .
            (method_declaration
                (visibility_modifier)? @visibility
                (#eq? @visibility "public")
                name: (_) @run
                (#not-match? @run "^test.*")
            ))
        )
    )
) @phpunit-test

; Class that follow the naming convention of PHPUnit test classes
; and that doesn't have the abstract modifier
(
    (class_declaration
        modifier: (_)? @modifier
        (#not-eq? @modifier "abstract")
        name: (_) @run
        (#match? @run ".*Test$")
    )
) @phpunit-test

; Add support for Pest runnable (non chainable method)
; Function expression that has `it`, `test` or `describe` as the function name
; And does not have chained methods
(
    (expression_statement
        (function_call_expression
            function: (_) @name
            (#any-of? @name "it" "test" "describe")
            arguments: (arguments
                (argument
                    (encapsed_string (string_value) @run)
                )
            )
        )
    )
) @pest-test

; Add support for Pest runnable (chainable method)
; Function expression that has `it`, `test` or `describe` as the function name
; And has chained methods
(
    (_
        object: (function_call_expression
            function: (_) @name
            (#any-of? @name "it" "test" "describe")
            arguments: (arguments
                (argument
                    (encapsed_string (string_value) @run)
                )
            )
        )
    )
) @pest-test
