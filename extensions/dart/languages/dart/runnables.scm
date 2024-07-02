; Flutter main
(
    (
        (import_or_export
            (library_import
                (import_specification
                    ("import"
                        (configurable_uri
                            (uri
                                (string_literal) @_import
                                (#match? @_import "package:flutter/(material|widgets|cupertino).dart")
                                (#not-match? @_import "package:flutter_test/flutter_test.dart")
                                (#not-match? @_import "package:test/test.dart")
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
                                (string_literal) @_import
                                (#match? @_import "package:flutter_test/flutter_test.dart")
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
