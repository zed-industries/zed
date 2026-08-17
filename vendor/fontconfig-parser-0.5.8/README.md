# fontconfig-parser

This crate provide parsing fontconfig file but not yet complete all features

see <https://www.freedesktop.org/software/fontconfig/fontconfig-user.html> for more detail infomation of fontconfig file

## Example

```rust
use fontconfig_parser::FontConfig;

let mut config = FontConfig::default();

config.merge_config("/etc/fonts/fonts.conf").unwrap();
```

License: MIT
