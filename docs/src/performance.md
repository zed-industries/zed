How to use our internal tools to profile and keep Zed fast.

# Rough quick CPU profiling (Flamechart)

See what the CPU spends the most time on. Strongly recommend you use
[samply](https://github.com/mstange/samply). It opens an interactive profile in
the browser (specifically a local instance of [firefox_profiler](https://profiler.firefox.com/)).

See [samply](https://github.com/mstange/samply)'s README on how to install and run.

The profile.json does not contain any symbols. Firefox profiler can add the local symbols to the profile for for. To do that hit the upload local profile button in the top right corner.

<img width="851" height="613" alt="image" src="https://github.com/user-attachments/assets/cbef2b51-0442-4ee9-bc5c-95f6ccf9be2c" />

# In depth CPU profiling (Tracing)

See how long each annotated function call took and its arguments (if
configured).

Annotate any function you need appear in the profile with instrument. For more
details see
[tracing-instrument](https://docs.rs/tracing/latest/tracing/attr.instrument.html):

```rust
#[instrument(skip_all)]
fn should_appear_in_profile(kitty: Cat) {
    sleep(QUITE_LONG)
}
```

Then either compile Zed with `ZTRACING=1 cargo r --features tracy --release`. The release build is optional but highly recommended as like every program Zeds performance characteristics change dramatically with optimizations. You do not want to chase slowdowns that do not exist in release.

## One time Setup/Building the profiler:

Download the profiler:
[linux x86_64](https://zed-tracy-import-miniprofiler.nyc3.digitaloceanspaces.com/tracy-profiler-linux-x86_64)
[macos aarch64](https://zed-tracy-import-miniprofiler.nyc3.digitaloceanspaces.com/tracy-profiler-0.13.0-macos-aarch64)

### Alternative: Building it yourself

- Clone the repo at git@github.com:wolfpld/tracy.git
- `cd profiler && mkdir build && cd build`
- Run cmake to generate build files: `cmake -G Ninja -DCMAKE_BUILD_TYPE=Release ..`
- Build the profiler: `ninja`
- [Optional] move the profiler somewhere nice like ~/.local/bin on linux

## Usage

Open the profiler (tracy-profiler), you should see zed in the list of `Discovered clients` click it.
<img width="392" height="287" alt="image" src="https://github.com/user-attachments/assets/b6f06fc3-6b25-41c7-ade9-558cc93d6033" />

To find functions that take a long time follow this image:
<img width="888" height="1159" alt="image" src="https://github.com/user-attachments/assets/77087617-f53a-4331-863d-e59f8a5b6f0b" />

# Task/Async profiling

Get a profile of the zed foreground executor and background executors. Check if
anything is blocking the foreground too long or taking too much (clock) time in
the background.

The profiler always runs in the background. You can save a trace from its UI or
look at the results live.

## Setup/Building the importer:

Download the importer
[linux x86_64](https://zed-tracy-import-miniprofiler.nyc3.digitaloceanspaces.com/tracy-import-miniprofiler-linux-x86_64)
[mac aarch64](https://zed-tracy-import-miniprofiler.nyc3.digitaloceanspaces.com/tracy-import-miniprofiler-macos-aarch64)

### Alternative: Building it yourself

- Clone the repo at git@github.com:zed-industries/tracy.git on v0.12.2 branch
- `cd import && mkdir build && cd build`
- Run cmake to generate build files: `cmake -G Ninja -DCMAKE_BUILD_TYPE=Release ..`
- Build the importer: `ninja`
- Run the importer on the trace file: `./tracy-import-miniprofiler /path/to/trace.miniprof /path/to/output.tracy`
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
