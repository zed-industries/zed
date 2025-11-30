How to use our internal tools to profile and keep Zed fast.

# Flamechart/CPU profiling

See what the CPU spends the most time on. Strongly recommend you use
[samply](https://github.com/mstange/samply). It opens an interactive profile in
the browser (specifically a local instance of [firefox_profiler](https://profiler.firefox.com/)).

See [samply](https://github.com/mstange/samply)'s README on how to install and run.

The profile.json does not contain any symbols. Firefox profiler can add the local symbols to the profile for for. To do that hit the upload local profile button in the top right corner.

<img width="851" height="613" alt="image" src="https://github.com/user-attachments/assets/cbef2b51-0442-4ee9-bc5c-95f6ccf9be2c" />

# Task/Async profiling

Get a profile of the zed foreground executor and background executors. Check if
anything is blocking the foreground too long or taking too much (clock) time in
the background.

The profiler always runs in the background. You can save a trace from its UI or
look at the results live.

## Setup/Building the importer:

- Clone the repo at git@github.com:zed-industries/tracy.git on v0.12.2 branch
- `cd profiler && mkdir build && cd build`
- Run cmake to generate build files: `cmake -G Ninja -DCMAKE_BUILD_TYPE=Release ..`
- Build the importer: `ninja`
- Run the impoter on the trace file: `./tracy-import-miniprofiler /path/to/trace.miniprof /path/to/output.tracy`
- Open the trace in tracy:
  - If you're on windows download the v0.12.2 version from the releases on the upstream repo
  - If you're on other platforms open it on the website: https://tracy.nereid.pl/ (the version might mismatch so your luck might vary, we need to host our own ideally..)

## To Save a Trace:

- Run the action: `zed open performance profiler`
- Hit the save button. This opens a save dialog or if that fails to open the trace gets saved in your working directory.
- Convert the profile so it can be imported in tracy using the importer: `./tracy-import-miniprofiler <path to performance_profile.miniprof> output.tracy`
- Go to <https://tracy.nereid.pl/> hit the 'power button' in the top left and then open saved trace.
- Now zoom in to see the tasks and how long they took

# Warn if function is slow

```rust
let _timer = zlog::time!("my_function_name").warn_if_gt(std::time::Duration::from_millis(100));
```
