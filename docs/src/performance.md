---
title: Rough quick CPU profiling (Flamechart)
description: "Performance profiling and optimization for Zed development."
---

How to use our internal tools to profile and keep Zed fast.

# Rough quick CPU profiling (Flamechart)

See what the CPU spends the most time on. Strongly recommend you use
[samply](https://github.com/mstange/samply). It opens an interactive profile in
the browser (specifically a local instance of [firefox_profiler](https://profiler.firefox.com/)).

See [samply](https://github.com/mstange/samply)'s README on how to install and run.

The profile.json does not contain any symbols. Firefox profiler can add the local symbols to the profile for for. To do that hit the upload local profile button in the top right corner.

<img width="851" height="auto" alt="image" src="https://github.com/user-attachments/assets/cbef2b51-0442-4ee9-bc5c-95f6ccf9be2c" style="display: block; margin: 0 auto;" />

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

<img width="392" height="auto" alt="image" src="https://github.com/user-attachments/assets/b6f06fc3-6b25-41c7-ade9-558cc93d6033" style="display: block; margin: 0 auto;"/>

Tracy is an incredibly powerful profiler which can do a lot however it's UI is not that friendly. This is not the place for an in depth guide to Tracy, I do however want to highlight one particular workflow that is helpful when figuring out why a piece of code is _sometimes_ slow.

Here are the steps:

1. Click the flamechart button at the top.

<img width="1815" height="auto" alt="Click flamechart" src="https://github.com/user-attachments/assets/9b488c60-90fa-4013-a663-f4e35ea753d2" />

2. Click on a function that takes a lot of time.

<img width="2001" height="auto" alt="Click snapshot" src="https://github.com/user-attachments/assets/ddb838ed-2c83-4dba-a750-b8a2d4ac6202" />

3. Expand the list of function calls by clicking on main thread.

<img width="2313" height="auto" alt="Click main thread" src="https://github.com/user-attachments/assets/465dd883-9d3c-4384-a396-fce68b872d1a" />

4. Filter that list to the slower calls then click on one of the slow calls in the list

<img width="2264" height="auto" alt="Select the tail calls in the histogram to filter down the list of calls then click on one call" src="https://github.com/user-attachments/assets/a8fddc7c-f40a-4f11-a648-ca7cc193ff6f" />

5. Click zoom to zone to go to that specific function call in the timeline

<img width="1822" height="auto" alt="Click zoom to zone" src="https://github.com/user-attachments/assets/3391664d-7297-41d4-be17-ac9b2e2c85d1" />

6. Scroll to zoom in and see more detail about the callers

<img width="1964" height="auto" alt="Scroll to zoom in" src="https://github.com/user-attachments/assets/625c2bf4-a68d-40c4-becb-ade16bc9a8bc" />

7. Click on a caller to to get statistics on _it_.

<img width="1888" height="auto" alt="Click on any of the zones to get statistics" src="https://github.com/user-attachments/assets/7e578825-2b63-4b7f-88f7-0cb16b8a3387" />

While normally the blue bars in the Tracy timeline correspond to function calls they can time any part of a codebase. In the example below we have added an extra span "for block in edits" and added metadata to it: the block_height. You can do that like this:

```rust
let span = ztracing::debug_span!("for block in edits", block_height = block.height());
let _enter = span.enter(); // span guard, when this is dropped the span ends (and its duration is recorded)
```

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
- Run the importer on the trace file: `./tracy-import-miniprofiler /path/to/trace.miniprof.json /path/to/output.tracy`
- Open the trace in tracy:
  - If you're on windows download the v0.12.2 version from the releases on the upstream repo
  - If you're on other platforms open it on the website: https://tracy.nereid.pl/ (the version might mismatch so your luck might vary, we need to host our own ideally..)

## To Save a Trace:

- Run the action: `zed open performance profiler`
- Hit the save button. This opens a save dialog or if that fails to open the trace gets saved in your working directory.
- Convert the profile so it can be imported in tracy using the importer: `./tracy-import-miniprofiler <path to performance_profile.miniprof.json> output.tracy`
- Go to <https://tracy.nereid.pl/> hit the 'power button' in the top left and then open saved trace.
- Now zoom in to see the tasks and how long they took

# Warn if function is slow

```rust
let _timer = zlog::time!("my_function_name").warn_if_gt(std::time::Duration::from_millis(100));
```
