; Flutter main
(
    (
        (import_or_export
            (library_import
                (import_specification
                    ("import"
                        (configurable_uri
                            (uri
                                (string_literal) @import
                                (#match? @import "package:flutter/(material|widgets|cupertino).dart")
                                (#not-match? @import "package:flutter_test/flutter_test.dart")
                                (#not-match? @import "package:test/test.dart")
        ))))))
        (
            (function_signature
                name: (_) @run
            )
            (#eq? @run "main")
        )
        (#set! tag flutter-main)
    )
)

; Flutter test main
(
    (
        (import_or_export
            (library_import
                (import_specification
                    ("import"
                        (configurable_uri
                            (uri
                                (string_literal) @import
                                (#match? @import "package:flutter_test/flutter_test.dart")
        ))))))
        (
            (function_signature
                name: (_) @run
            )
            (#eq? @run "main")
        )
        (#set! tag flutter-test-main)
    )
)
