(
    ; Class that follow the naming convention of PHPUnit test classes
    ; and that doesn't have the abstract modifier
    ; and have a method that follow the naming convention of PHPUnit test methods
    ; and the method is public
    (class_declaration
        modifier: (_)? @modifier
        (#not-eq? @modifier "abstract")
        name: (_) @name
        (#match? @name ".*Test")
        body: (declaration_list
            (method_declaration
                (visibility_modifier)? @visibility
                (#eq? @visibility "public")
                name: (_) @run
                (#match? @run "test.*")
            )
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
        (#match? @run ".*Test")
    )
) @phpunit-test
