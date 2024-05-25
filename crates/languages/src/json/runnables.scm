; Add support package.json script runnable

(
    (document
        (object
            (pair
                key: (string
                    (string_content) @name
                    (#eq? @name "scripts")
                )
                value: (object
                    (pair
                        key: (string (string_content) @run)
                        value: (string (string_content))
                    )
                )
            )
        )
    ) @package-script
    (#set! tag package-script)
)
