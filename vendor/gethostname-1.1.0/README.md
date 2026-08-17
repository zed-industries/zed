# gethostname.rs

[![Current release](https://img.shields.io/crates/v/gethostname.svg)][crates]
[![Documentation](https://docs.rs/gethostname/badge.svg)][docs]

[gethostname()][ghn] for all platforms.

```rust
use gethostname::gethostname;

println!("Hostname: {:?}", gethostname());
```

This crate is stable and will not see many changes.

[crates]: https://crates.io/crates/gethostname
[docs]: https://docs.rs/gethostname
[ghn]: http://pubs.opengroup.org/onlinepubs/9699919799/functions/gethostname.html

## License

Copyright Sebastian Wiesner <sebastian@swsnr.de>

Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at <http://www.apache.org/licenses/LICENSE-2.0>.

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
