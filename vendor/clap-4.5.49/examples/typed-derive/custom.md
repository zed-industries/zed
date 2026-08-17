Help:
```console
$ typed-derive custom --help
Usage: typed-derive custom [OPTIONS]

Options:
      --target-version <TARGET_VERSION>
          Hand-implement `TypedValueParser`

          Possible values:
          - major: Increase the major version (x.0.0)
          - minor: Increase the minor version (x.y.0)
          - patch: Increase the patch version (x.y.z)

  -h, --help
          Print help (see a summary with '-h')

```

Defines (key-value pairs)
```console
$ typed-derive custom --target-version major
Custom(CustomParser { target_version: Some(Relative(Major)) })

$ typed-derive custom --target-version 10.0.0
Custom(CustomParser { target_version: Some(Absolute(Version { major: 10, minor: 0, patch: 0 })) })

$ typed-derive custom --target-version 10
? failed
error: invalid value '10' for '--target-version <TARGET_VERSION>': unexpected end of input while parsing major version number

For more information, try '--help'.

$ typed-derive custom --target-version blue
? failed
error: invalid value 'blue' for '--target-version <TARGET_VERSION>': unexpected character 'b' while parsing major version number

For more information, try '--help'.

```
