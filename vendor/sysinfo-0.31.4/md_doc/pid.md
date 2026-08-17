Process ID.

Can be used as an integer type by simple casting. For example:

```
use sysinfo::Pid;

// 0's type will be different depending on the platform!
let p = Pid::from(0);

// For something more "general":
let p = Pid::from_u32(0);
let i: u32 = p.as_u32();
```

On glibc systems this is a glibc [`pid_t`](https://www.gnu.org/software/libc/manual/html_node/Process-Identification.html).

On Windows systems this is a [`usize` and represents a windows process identifier](https://docs.microsoft.com/en-us/windows/win32/procthread/process-handles-and-identifiers).

On unsupported systems, this is also a `usize`.
