This is the minimum interval time used internally by `sysinfo` to refresh the CPU time.

⚠️ This value differs from one OS to another.

Why is this constant even needed?

If refreshed too often, the CPU usage of processes will be `0` whereas on Linux it'll
always be the maximum value (`number of CPUs * 100`).
