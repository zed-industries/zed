Returns the list of the supported signals on this system (used by
[`Process::kill_with`][crate::Process::kill_with]).

```
use sysinfo::{System, SUPPORTED_SIGNALS};

println!("supported signals: {:?}", SUPPORTED_SIGNALS);
```
