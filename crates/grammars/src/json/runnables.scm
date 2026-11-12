; Add support `package.json` and `composer.json` script runnable
((document
  (object
    (pair
      key: (string
        (string_content) @_name
        (#eq? @_name "scripts"))
      value: (object
        (pair
          key: (string) @run) @run_item))))
  (#set! tag package-script)
  (#set! tag composer-script))
