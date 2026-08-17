# Changelog

## unreleased

- FEATURE: [macOS] add `Config::with_fsevent_latency` to configure FSEvents stream latency [#930]
- FIX: [windows] emit a Remove event when a watched directory is deleted, matching inotify and FSEvents
- FIX: [windows] surface `ReadDirectoryChangesW` read-start failures [#935]
- FEATURE: [windows] report created file/folder kinds when they can be determined [#935]
- CHANGE: [macOS] improve FSEvents callback performance by avoiding unnecessary allocations and repeated handler locking
- PERF: [kqueue] avoid filesystem walks for recursive kqueue unwatch

[#930]: https://github.com/notify-rs/notify/pull/930
[#935]: https://github.com/notify-rs/notify/issues/935

## notify 9.0.0-rc.4 (2026-05-02)

- CHANGE: preserve watched path representation in `Event.paths` and `Watcher::watched_paths`; relative watch paths now produce relative event paths consistently across backends [#453] [#740]
- FIX: [kqueue] stop reporting arbitrary existing child paths for non-recursive directory write events [#644]
- FIX: replace an existing watch when `watch` is called again for the same path, avoiding duplicate FSEvent paths and leaked Windows watch handles [#708]
- DOCS: define `Watcher::watch` replacement behavior for an existing backend-resolved path [#708]

## notify 9.0.0-rc.3 (2026-04-16)

- CHANGE: raise MSRV to 1.88
- FEATURE: add `Watcher::watched_paths` to list active watches as `(PathBuf, RecursiveMode)` pairs across supported backends
- FIX: [windows] normalize emitted event paths to follow the watched path separator style and trim leading separators; add `Config::with_windows_path_separator_style` for explicit control [#375]
- FIX: [windows] make `unwatch()` wait until the watch is fully removed so later filesystem changes do not leak events [#730]
- FIX: [macOS] annotate FSEvents clone-related events with `info = "is: clone"` [#465]
- FIX: avoid panicking in `unwatch` when internal mutexes are poisoned
- CHANGE: add `#[must_use]` annotations to builder, constructor, and getter-style APIs such as `Config`, `PathOp`, and `Error`

[#375]: https://github.com/notify-rs/notify/issues/375
[#465]: https://github.com/notify-rs/notify/issues/465
[#730]: https://github.com/notify-rs/notify/issues/730
[#739]: https://github.com/notify-rs/notify/issues/739
[#453]: https://github.com/notify-rs/notify/issues/453
[#740]: https://github.com/notify-rs/notify/issues/740
[#644]: https://github.com/notify-rs/notify/issues/644
[#708]: https://github.com/notify-rs/notify/issues/708

## notify 9.0.0-rc.2 (2026-02-14)

- FEATURE: remove `Watcher::paths_mut` and introduce `update_paths` [#705]
- FEATURE: impl `EventHandler` for `futures::channel::mpsc::UnboundedSender` and `tokio::sync::mpsc::UnboundedSender` behind the `futures` and `tokio` feature flags [#767]
- FIX: [macOS] prevent handler panicking in the FSEvents callback panics [#790]
- FIX: [macOS] prevent panicking when path contains non UTF-8 chars [#790]
- FIX: [linux] fix a TOCTOU issue when adding a watcher [#792]
- FIX: [linux] surface inotify `UNMOUNT` events to users and clean up affected watches [#627]
- FIX: [macOS] remove `FSEventsPurgeEventsForDeviceUpToEventId` call [#795]

[#627]: https://github.com/notify-rs/notify/issues/627
[#705]: https://github.com/notify-rs/notify/pull/705
[#767]: https://github.com/notify-rs/notify/pull/767
[#790]: https://github.com/notify-rs/notify/pull/790
[#792]: https://github.com/notify-rs/notify/pull/792
[#795]: https://github.com/notify-rs/notify/pull/795

## notify 9.0.0-rc.1 (2026-01-25)

> [!IMPORTANT]
> The MSRV policy has been changed since this release.
> Check out README for details.

- CHANGE: raise MSRV to 1.85 **breaking**
- CHANGE: [macOS] replace `fsevent-sys` with `objc2-core-foundation` and `objc2-core-services` [#726]
- FEATURE: add `EventKindMask` for filtering filesystem events [#736]
- FIX: Fix the bug that `FsEventWatcher` crashes when dealing with empty path [#718]
- FIX: Fix the bug that `INotifyWatcher` keeps watching deleted paths [#720]
- FIX: Fixed ordering where `FsEventWatcher` emitted `Remove` events non-terminally [#747]
- FIX: [macOS] throw `FsEventWatcher` stream start error properly [#733]

[#718]: https://github.com/notify-rs/notify/pull/718
[#720]: https://github.com/notify-rs/notify/pull/720
[#726]: https://github.com/notify-rs/notify/pull/726
[#733]: https://github.com/notify-rs/notify/pull/733
[#736]: https://github.com/notify-rs/notify/pull/736
[#747]: https://github.com/notify-rs/notify/pull/747

## notify 8.2.0 (2025-08-03)
- FEATURE: notify user if inotify's `max_user_watches` has been reached [#698]
- FIX: `INotifyWatcher` ignore events with unknown watch descriptors (instead of `EventMask::Q_OVERFLOW`) [#700]

[#698]: https://github.com/notify-rs/notify/pull/698
[#700]: https://github.com/notify-rs/notify/pull/700

## notify 8.1.0 (2025-07-03)
- FEATURE: added support for the [`flume`](https://docs.rs/flume) crate
- FIX: kqueue-backend: do not double unwatch top-level directory when recursively unwatching [#683]
- FIX: Return the crate error `PathNotFound` instead bubbling up the std::io error [#685]
- FIX: fix server hangs when trashing folders on Windows [#674]

## notify 8.0.0 (2025-01-10)

- CHANGE: update notify-types to version 2.0.0
- CHANGE: raise MSRV to 1.77 **breaking**
- FEATURE: add config option to disable following symbolic links [#635]
- FIX: unaligned access to FILE_NOTIFY_INFORMATION [#647] **breaking**

[#635]: https://github.com/notify-rs/notify/pull/635
[#647]: https://github.com/notify-rs/notify/pull/647

## notify 7.0.0 (2024-10-25)

- CHANGE: raise MSRV to 1.72 [#569] [#610] **breaking**
- CHANGE: move event type to notify-types crate [#559]
- CHANGE: flatten serialization of events and use camelCase [#558]
- CHANGE: remove internal use of crossbeam channels [#569] [#610]
- CHANGE: rename feature `crossbeam` to `crossbeam-channel` and disable it by default [#610] **breaking**
- CHANGE: upgrade mio to 1.0 [#623]
- CHANGE: add log statements [#499]
- FIX: prevent UB with illegal instruction for the windows backend [#604] [#607]
- FIX: on Linux report deleted directories correctly [#545]
- FIX: on Linux report access open events [#612]
- FEATURE: enable kqueue on iOS [#533]
- MISC: various minor doc updates and fixes [#535] [#536] [#543] [#565] [#592] [#595]
- MISC: update inotify to 0.10 [#547]

[#499]: https://github.com/notify-rs/notify/pull/499
[#533]: https://github.com/notify-rs/notify/pull/533
[#535]: https://github.com/notify-rs/notify/pull/535
[#536]: https://github.com/notify-rs/notify/pull/536
[#543]: https://github.com/notify-rs/notify/pull/543
[#545]: https://github.com/notify-rs/notify/pull/545
[#547]: https://github.com/notify-rs/notify/pull/547
[#558]: https://github.com/notify-rs/notify/pull/558
[#559]: https://github.com/notify-rs/notify/pull/559
[#565]: https://github.com/notify-rs/notify/pull/565
[#569]: https://github.com/notify-rs/notify/pull/569
[#592]: https://github.com/notify-rs/notify/pull/592
[#595]: https://github.com/notify-rs/notify/pull/595
[#604]: https://github.com/notify-rs/notify/pull/604
[#607]: https://github.com/notify-rs/notify/pull/607
[#610]: https://github.com/notify-rs/notify/pull/610
[#612]: https://github.com/notify-rs/notify/pull/612
[#623]: https://github.com/notify-rs/notify/pull/623

## notify 6.1.1 (2023-08-21)

- CHANGE: remove serde binary experiment opt-out after it got removed [#530]

[#530]: https://github.com/notify-rs/notify/pull/530

## notify 6.1.0 (2023-08-18)

- CHANGE: opt-out of the serde binary experiment by restricting it to < 1.0.172 [#528]
- CHANGE: license changed to only CC0-1.0 [#520]
- CHANGE: use logging [#499]
- CHANGE: upgrade windows-sys to 0.48 [#479]
- CHANGE: bump filetime to 0.2.22 [#521]
- FEATURE: support manual polling of PollWatcher and disabling automatic polling [#524]
- FEATURE: support listening to the initial pollwatcher file scan [#507]
- FIX: fix moved folders not being watched on linux [#498]
- FIX: fixup potential future double free on windows [#517]
- FIX: require bitflags only on macos and upgrade the crate [#505]
- DOCS: add more known issues, typos and cleanup examples [#523] [#502] [#522]

[#524]: https://github.com/notify-rs/notify/pull/524
[#523]: https://github.com/notify-rs/notify/pull/523
[#502]: https://github.com/notify-rs/notify/pull/502
[#522]: https://github.com/notify-rs/notify/pull/522
[#479]: https://github.com/notify-rs/notify/pull/479
[#521]: https://github.com/notify-rs/notify/pull/521
[#517]: https://github.com/notify-rs/notify/pull/517
[#507]: https://github.com/notify-rs/notify/pull/507
[#499]: https://github.com/notify-rs/notify/pull/499
[#505]: https://github.com/notify-rs/notify/pull/505
[#498]: https://github.com/notify-rs/notify/pull/498
[#520]: https://github.com/notify-rs/notify/pull/520
[#528]: https://github.com/notify-rs/notify/pull/528

## notify 6.0.1 (2023-06-16)

- DOCS: fix swapped debouncer-full / -mini links in the readme/crates.io [4be6bde]

[4be6bde]: https://github.com/notify-rs/notify/commit/4be6bdef4fa7b260a3a56a11212ac074f8e39b39

## notify 6.0.0 (2023-05-17)

- CHANGE: files and directories moved into a watch folder on Linux will now be reported as `rename to` events instead of `create` events [#480]
- CHANGE: on Linux `rename from` events will be emitted immediately without starting a new thread [#480]
- CHANGE: raise MSRV to 1.60 [#480]

[#480]: https://github.com/notify-rs/notify/pull/480

## notify 5.2.0 (2023-05-17)

- CHANGE: implement `Copy` for `EventKind` and `ModifyKind` [#481]

[#481]: https://github.com/notify-rs/notify/pull/481

## notify 5.1.0 (2023-01-15)

- CHANGE: switch from winapi to windows-sys [#457]
- FIX: kqueue-backend: batch file-watching together to improve performance [#454]
- DOCS: include license file in crate again [#461]
- DOCS: typo and examples fixups

[#454]: https://github.com/notify-rs/notify/pull/454
[#461]: https://github.com/notify-rs/notify/pull/461
[#457]: https://github.com/notify-rs/notify/pull/457

## notify 5.0.0 (2022-08-28)

For a list of changes when upgrading from v4 see [UPGRADING_V4_TO_V5.md](../docs/UPGRADING_V4_TO_V5.md).

Differences to 5.0.0-pre.16:

- FIX: update minimum walkdir version to 2.2.2 [#432]
- CHANGE: add `need_rescan` function to `Event`, allowing easier detection when a rescan is required [#435]
- FIX: debouncer-mini: change crossbeam feature to `crossbeam`, to allow passthrough with notify re-exports [#429]
- DOCS: improve v5-to-v5 upgrade docs [#431]
- DOCS: file back v4 changelog into main [#437]
- DOCS: cleanups and link fixes

[#431]: https://github.com/notify-rs/notify/pull/431
[#432]: https://github.com/notify-rs/notify/pull/432
[#437]: https://github.com/notify-rs/notify/pull/437
[#435]: https://github.com/notify-rs/notify/pull/435
[#429]: https://github.com/notify-rs/notify/pull/429

## 5.0.0-pre.16 (2022-08-12)

- CHANGE: require config for watcher creation and unify config [#426]
- CHANGE: fsevent: use RenameMode::Any for renaming events [#371]
- FEATURE: re-add debouncer as new crate and fixup CI [#286]
- FEATURE: allow disabling crossbeam-channel dependency [#425]
- FIX: PollWatcher panic after delete-and-recreate [#406]
- MISC: rework pollwatcher internally [#409]
- DOCS: cleanup all docs towards v5 [#395]

[#395]: https://github.com/notify-rs/notify/pull/395
[#406]: https://github.com/notify-rs/notify/pull/406
[#409]: https://github.com/notify-rs/notify/pull/409
[#425]: https://github.com/notify-rs/notify/pull/425
[#286]: https://github.com/notify-rs/notify/pull/286
[#426]: https://github.com/notify-rs/notify/pull/426
[#371]: https://github.com/notify-rs/notify/pull/371

## 5.0.0-pre.15 (2022-04-30)

- CHANGE: raise MSRV to 1.56! [#396] and [#402]
- FEATURE: add support for pseudo filesystems like sysfs/procfs [#396]
- FIX: Fix builds on (Free)BSD due to changes in kqueue fix release [#399]

[#396]: https://github.com/notify-rs/notify/pull/396
[#399]: https://github.com/notify-rs/notify/pull/399
[#402]: https://github.com/notify-rs/notify/pull/402

## 5.0.0-pre.14 (2022-03-13)

- CHANGE: upgrade mio to 0.8 [#386]
- CHANGE: PollWatcher: unify signature of new and with_delay  [#360]
- CHANGE: emit EventKind::Modify on kqueue write event [#370]
- CHANGE: use RenameMode::Any for renaming events [#371]
- CHANGE: name all threads spawned by notify [#383]
- FEATURE: Add Watcher::kind() [#364]
- FEATURE: Add more Debug/Copy trait impls [#377] [#378]
- FIX: Fix selection of RecommendedWatcher for macos_kqueue feature  [#362]
- FIX: Turn possible panic into an error in FSEvents backend when file is deleted rapidly [#369]
- FIX: lqueue: emit Create Events and watch all files in a directory [#372]
- FIX: inotify: don't panic on shutdown [#373]

[#386]: https://github.com/notify-rs/notify/pull/386
[#360]: https://github.com/notify-rs/notify/pull/360
[#370]: https://github.com/notify-rs/notify/pull/370
[#371]: https://github.com/notify-rs/notify/pull/371
[#383]: https://github.com/notify-rs/notify/pull/383
[#364]: https://github.com/notify-rs/notify/pull/364
[#377]: https://github.com/notify-rs/notify/pull/377
[#378]: https://github.com/notify-rs/notify/pull/378
[#362]: https://github.com/notify-rs/notify/pull/362
[#369]: https://github.com/notify-rs/notify/pull/369
[#372]: https://github.com/notify-rs/notify/pull/372
[#373]: https://github.com/notify-rs/notify/pull/373


## 5.0.0-pre.13 (2021-09-07)

- Fix: Add path information to inotify and kqueue watch/unwatch errors  [#354]
- Fix: Delete dbg call from kqueue.rs  [#357]

[#354]: https://github.com/notify-rs/notify/pull/354
[#357]: https://github.com/notify-rs/notify/pull/357

## 5.0.0-pre.12 (2021-08-12)

- CHANGE: Move creation of watcher into trait [#345]
- CHANGE: Add EventHandler trait to replace EventFn [#346]
- FIX: Fix build failure on x86_64-unknown-netbsd [#347]

[#345]: https://github.com/notify-rs/notify/pull/345
[#346]: https://github.com/notify-rs/notify/pull/346
[#347]: https://github.com/notify-rs/notify/pull/347

## 5.0.0-pre.11 (2021-07-22)

- FEATURE: Add `Kqueue` backend for use on BSD [#335]
- CHANGE: Change EventFn to take FnMut [#333]
- CHANGE: Make `Watcher` object safe [#336]
- FIX: Join thread in `fseven` on shutdown [#337]
- FIX: Only check for ENOSPC on inotify_add_watch in `inotify` [#330]
- FIX: Free context when stream is deallocated in `fsevent` [#329]
- DOCS: Fix missing comma in docs [#340]

[#333]: https://github.com/notify-rs/notify/pull/333
[#336]: https://github.com/notify-rs/notify/pull/336
[#340]: https://github.com/notify-rs/notify/pull/340
[#337]: https://github.com/notify-rs/notify/pull/337
[#335]: https://github.com/notify-rs/notify/pull/335
[#330]: https://github.com/notify-rs/notify/pull/330
[#329]: https://github.com/notify-rs/notify/pull/329

## 5.0.0-pre.10 (2021-06-04)

- FIX: Make StreamContextInfo `Send` to fix soundness issue [#325]

[#325]: https://github.com/notify-rs/notify/pull/325

## 5.0.0-pre.9 (2021-05-21)

- DEPS: Upgrade fsevent-sys dependency to 4.0 [#322]
- CHANGE: Remove dependency on `fsevent`. [#313]
- FIX: Correct the return type for `CFRunLoopIsWaiting` to be `Boolean` [#319]
- CHANGE: Hide fsevent::{CFRunLoopIsWaiting,callback}, fix clippy lint warnings [#312]
- FIX: Fix some clippy lints [#320]

[#319]: https://github.com/notify-rs/notify/pull/319
[#313]: https://github.com/notify-rs/notify/pull/313
[#312]: https://github.com/notify-rs/notify/pull/312
[#320]: https://github.com/notify-rs/notify/pull/320
[#322]: https://github.com/notify-rs/notify/pull/322

## 4.0.17 (2021-05-13)

- FIX: Don't crash on macos when creating & deleting folders in rapid succession [#303]

[#303]: https://github.com/notify-rs/notify/pull/303

## 5.0.0-pre.8 (2021-05-12)

- HOTFIX: Fix breaking change in fsevent-sys in minor version destroying builds [#316]
- FIX: Don't crash on macos when creating & deleting folders in rapid succession [#302]
- FIX: Remove `anymap`, and replace event attributes with an opaque type. [#306]

[#302]: https://github.com/notify-rs/notify/pull/302
[#306]: https://github.com/notify-rs/notify/pull/306
[#316]: https://github.com/notify-rs/notify/pull/316

## 5.0.0-pre.7 (2021-04-15)

- FIX: Display proper error message when reaching inotify limits on linux [#285]
- FIX: Fix leaks on Windows [#298]

[#285]: https://github.com/notify-rs/notify/pull/285
[#298]: https://github.com/notify-rs/notify/pull/298

## 5.0.0-pre.6 (2021-02-20)

- FIX: Handle interrupted system call errors from mio [#281]

[#281]: https://github.com/notify-rs/notify/pull/281

## 5.0.0-pre.5 (2021-01-28)

- RUSTC: Push the minimum version to 1.47.0 [#280]
- DEPS: Update `inotify` to 0.9 [#280]
- DEPS: Update `mio` to 0.7 and remove `mio-extras` [#278]
- FIX: Report events promptly on Linux, even when many occur in rapid succession. [#268]

[#280]: https://github.com/notify-rs/notify/pull/280
[#278]: https://github.com/notify-rs/notify/pull/278

## 5.0.0-pre.4 (2020-10-31)

- CHANGE: Avoid stating the watched path for non-recursive watches with inotify [#256]
- DOCS: Fix broken link in crate documentation [#260]

[#256]: https://github.com/notify-rs/notify/pull/256
[#260]: https://github.com/notify-rs/notify/pull/260

## 5.0.0-pre.3 (2020-06-22)

- DEPS: Removed unused chashmap dependency [#242]

[#242]: https://github.com/notify-rs/notify/pull/242

## 4.0.16 (2021-04-14)

- FIX: Report events promptly on Linux, even when many occur in rapid succession. [#268]
- FIX: Fix leaks on Windows and debounce module. [#288]
- FIX: Display proper error message when reaching inotify limits on linux. [#290]

[#268]: https://github.com/notify-rs/notify/pull/268
[#288]: https://github.com/notify-rs/notify/pull/288
[#290]: https://github.com/notify-rs/notify/pull/290

## 5.0.0-pre.2 (2020-01-07)

- (Temporary): Remove event debouncing.
- (Temporary): Remove tests.
- CHANGE: Rewrite immediate events to use new system.
- CHANGE: Remove `Sender`s from watcher API in favour of `EventFn` [#214]
- DEPS: Update inotify to 0.8. [#234]
- DEPS: Update crossbeam-channel to 0.4.
- DEPS: \[macOS\] Update fsevent to 2.0.1 and fsevent-sys to 3.0.0.

[#214]: https://github.com/notify-rs/notify/pull/214
[#234]: https://github.com/notify-rs/notify/pull/234

## 4.0.15 (2020-01-07)

- DEPS: Update inotify to 0.7.
- DEPS(DEV): Replace tempdir with tempfile since tempdir is deprecated.
- DEPS: Update winapi to 0.3 and remove kernel32-sys. [#232]

[#232]: https://github.com/notify-rs/notify/pull/232

## 5.0.0-pre.1 (2019-06-30)

_(no changes, just a new release because the old one failed to publish properly)_

## 5.0.0-pre.0 (2019-06-22)

- **yanked 2019-06-30**
- RUSTC: Push the minimum version to 1.36.0 [#201]
- RUSTC: Switch the crate to Rust 2018.
- FIX: Implement `Sync` for PollWatcher to support FreeBSD. [#197]
- FEATURE: Add new runtime configuration system.
- FEATURE: Add `Ongoing` events (optional, configured at runtime). [#146], [#183]
- FEATURE: Bring in new event system from `next` branch. [#187]
- FEATURE: Allow multiple watchers to send to the same channel. [`2a035c86`]
- CHANGE: Switch to crossbeam channel. [#160]
- CHANGE: Rename `Chmod` to `Metadata`. [#179], [#180], previously [#112], [#161]
- CHANGE: Remove `DebouncedEvent` event classification. [#187]
- DEPS: \[Linux\] Upgrade inotify to 0.7. [#184]
- DEPS: \[macOS\] Upgrade fsevent to 0.4. [#195]
- DEPS: Upgrade filetime to 0.2.6.
- META: Rename `v4-legacy` branch to `main`, to further clarify status and prepare for a breaking release.
- DOCS: Change `v5` to `Next Generation Notify` to allow for a breaking release.
- DOCS: Add rust-analyzer to Readme showcase.
- DOCS: Add github issue / PR templates and funding.

[#112]: https://github.com/notify-rs/notify/issues/112
[#146]: https://github.com/notify-rs/notify/issues/146
[#160]: https://github.com/notify-rs/notify/issues/160
[#161]: https://github.com/notify-rs/notify/issues/161
[#179]: https://github.com/notify-rs/notify/issues/179
[#180]: https://github.com/notify-rs/notify/issues/180
[#183]: https://github.com/notify-rs/notify/issues/183
[#184]: https://github.com/notify-rs/notify/issues/184
[#187]: https://github.com/notify-rs/notify/issues/187
[#195]: https://github.com/notify-rs/notify/issues/195
[#197]: https://github.com/notify-rs/notify/issues/197
[#201]: https://github.com/notify-rs/notify/issues/201
[`2a035c86`]: https://github.com/notify-rs/notify/commit/2a035c86c5f12aeee635a827c1f458211ca923ca

## 4.0.15 (2020)

- DEPS: Update winapi to 0.3.8 and remove kernel32-sys. [#232]
- META: The project maintainers are changed from @passcod to notify-rs.

[#232]: https://github.com/notify-rs/notify/pull/232

## 4.0.14 (2019-10-17)

- FIX: Fix deadlock in debouncer. [#210]

[#210]: https://github.com/notify-rs/notify/pull/210

## 4.0.13 (2019-09-01)

- FIX: Undo filetime pin. [#202], [`22e40f5e`]
- META: Project is abandoned.

[#202]: https://github.com/notify-rs/notify/issues/202
[`22e40f5e`]: https://github.com/notify-rs/notify/commit/22e40f5e4cb2a23528f169fc92015f935edc1c55

## 4.0.12 (2019-05-22)

- FIX: Implement `Sync` for PollWatcher to support FreeBSD. [#198]
- DEPS: Peg filetime to 1.2.5 to maintain rustc 1.26.1 compatibility. [#199]

[#198]: https://github.com/notify-rs/notify/issues/198
[#199]: https://github.com/notify-rs/notify/issues/199

## 4.0.11 (2019-05-08)

- DEPS: \[macOS\] Upgrade fsevent to 0.4. [#196]

[#196]: https://github.com/notify-rs/notify/issues/196

## 4.0.10 (2019-03-07)

- FIX: Panic caused by a clock race. [#182]
- DOCS: Add xi to Readme showcase. [`e6f09441`]

[#182]: https://github.com/notify-rs/notify/issues/182
[`e6f09441`]: https://github.com/notify-rs/notify/commit/e6f0944165551fa2ed9ad70e3e11d8b14186fc0a

## 4.0.9 (2019-02-09)

- FIX: High CPU usage in some conditions when using debouncing. [#177], [#178], coming from [rust-analyzer/#556]

[#177]: https://github.com/notify-rs/notify/issues/177
[#178]: https://github.com/notify-rs/notify/issues/178
[rust-analyzer/#556]: https://github.com/rust-analyzer/rust-analyzer/issues/556

## 4.0.8 (2019-02-06)

- DOCS: Mention hotwatch as alternative API. [#175], [`34775f26`]
- DEPS: \[Linux\] Disable `stream` feature for inotify. [#176], [`e729e279`]
- DOCS: Add dates to releases in changelog. [`cc621398`]
- DOCS: Backfill changelog: 4.0.2 to 4.0.7. [`6457f697`]
- DOCS: Backfill changelog: 0.0.1 to 2.6.0. [`d34e6ee7`]

[#175]: https://github.com/notify-rs/notify/issues/175
[`34775f26`]: https://github.com/notify-rs/notify/commit/34775f2695ec236fabc79f2c938e12e4cd54047b
[#176]: https://github.com/notify-rs/notify/issues/176
[`e729e279`]: https://github.com/notify-rs/notify/commit/e729e279f0721c4a5729e725a7cd5e4d761efb58
[`cc621398`]: https://github.com/notify-rs/notify/commit/cc621398e56e2257daf5816e8c2bb01ca79e8ddb
[`6457f697`]: https://github.com/notify-rs/notify/commit/6457f6975a9171483d531fcdafb956d2ee334d55
[`d34e6ee7`]: https://github.com/notify-rs/notify/commit/d34e6ee70df9b4905cbd04fe1a2b5770a9d2a4d4


## 4.0.7 (2019-01-23)

- DOCS: Document unexpected behaviour around watching a tree root. [#165], [#166]
- DOCS: Remove v2 documentation. [`8310b2cc`]
- TESTS: Change how tests are skipped. [`0b4c8400`]
- DOCS: Add timetrack to Readme showcase. [#167]
- META: Change commit message style: commits are now prefixed by a `[topic]`.
- FIX: Make sure debounced watcher terminates. [#170]
- FIX: \[Linux\] Remove thread wake-up on timeout (introduced in 4.0.5 by error). [#174]
- FIX: Restore compatibility with Rust before 1.30.0. [`eab75118`]
- META: Enforce compatibility with Rust 1.26.1 via CI. [`50924cd6`]
- META: Add maintenance status badge. [`ecd686ba`]
- DOCS: Freeze v4 branch (2018-10-05) [`8310b2cc`] — and subsequently unfreeze it. (2019-01-19) [`20c40f99`], [`c00da47c`]

[#165]: https://github.com/notify-rs/notify/issues/165
[#166]: https://github.com/notify-rs/notify/issues/166
[`8310b2cc`]: https://github.com/notify-rs/notify/commit/8310b2ccf68382548914df6ffeaf45248565b9fb
[`0b4c8400`]: https://github.com/notify-rs/notify/commit/0b4c840091f5b3ebd3262d7109308828800dc976
[#167]: https://github.com/notify-rs/notify/issues/167
[#170]: https://github.com/notify-rs/notify/issues/170
[#174]: https://github.com/notify-rs/notify/issues/174
[`eab75118`]: https://github.com/notify-rs/notify/commit/eab75118464dc5d0d48dce31ab7a8e07d7e68d80
[`50924cd6`]: https://github.com/notify-rs/notify/commit/50924cd676c8bce877634e32260ef3872f2feccb
[`ecd686ba`]: https://github.com/notify-rs/notify/commit/ecd686bab604442c315c114e536bdc310a9413b1
[`20c40f99`]: https://github.com/notify-rs/notify/commit/20c40f99ad042fba5abf36f65e9ee598562744d8
[`c00da47c`]: https://github.com/notify-rs/notify/commit/c00da47ce63815972ef7c4bafd3b8c2c11b8b0de


## 4.0.6 (2018-08-30)

- FIX: Add some consts to restore semver compatibility. [`6d4f1ab9`]

[`6d4f1ab9`]: https://github.com/notify-rs/notify/commit/6d4f1ab9af76ecfc856f573a3f5584ddcfe017df


## 4.0.5 (2018-08-29)

- DEPS: Update winapi (0.3), mio (0.6), inotify (0.6), filetime (0.2), bitflags (1.0). [#162]
- SEMVER BREAK: The bitflags upgrade introduced a breaking change to the API.

[#162]: https://github.com/notify-rs/notify/issues/162


## 4.0.4 (2018-08-06)

- Derive various traits for `RecursiveMode`. [#148]
- DOCS: Add docket to Readme showcase. [#154]
- DOCS: [Rename OS X to macOS](https://www.wired.com/2016/06/apple-os-x-dead-long-live-macos/). [#156]
- FIX: \[FreeBSD / Poll\] Release the lock while the thread sleeps (was causing random hangs). [#159]

[#148]: https://github.com/notify-rs/notify/issues/148
[#154]: https://github.com/notify-rs/notify/issues/154
[#156]: https://github.com/notify-rs/notify/issues/156
[#159]: https://github.com/notify-rs/notify/issues/159


## 4.0.3 (2017-11-26)

- FIX: \[macOS\] Concurrency-related FSEvent crash. [#132]
- FIX: \[macOS\] Deadlock due to race in FsEventWatcher. [#118], [#134]
- DEPS: Update walkdir to 2.0. [`fbffef24`]

[#118]: https://github.com/notify-rs/notify/issues/118
[#132]: https://github.com/notify-rs/notify/issues/132
[#134]: https://github.com/notify-rs/notify/issues/134
[`fbffef24`]: https://github.com/notify-rs/notify/commit/fbffef244726aae6e8a98e33ecb77a66274db91b


## 4.0.2 (2017-11-03)

- FIX: Suppress events for files which have been moved and deleted if a new file in the original location is created quickly when using the debounced interface (eg. while safe-saving files) [#129]

[#129]: https://github.com/notify-rs/notify/issues/129


## 4.0.1 (2017-03-25)

- FIX: \[Linux\] Detect moves if two connected move events are split between two mio polls


## 4.0.0 (2017-02-07)

- CHANGE: \[Linux\] Update dependency to inotify 0.3.0.
- FIX: \[macOS\] `.watch()` panics on macOS when the target doesn't exist. [#105]

[#105]: https://github.com/notify-rs/notify/issues/105

## (start work on vNext) (2016-12-29)

## 3.0.1 (2016-12-05)

- FIX: \[macOS\] Fix multiple panics in debounce module related to move events. [#99], [#100], [#101]

[#99]: https://github.com/notify-rs/notify/issues/99
[#100]: https://github.com/notify-rs/notify/issues/100
[#101]: https://github.com/notify-rs/notify/issues/101


## 3.0.0 (2016-10-30)

- FIX: \[Windows\] Fix watching files on Windows using relative paths. [#90]
- FEATURE: Add debounced event notification interface. [#63]
- FEATURE: \[Polling\] Implement `CREATE` and `DELETE` events for PollWatcher. [#88]
- FEATURE: \[Polling\] Add `::with_delay_ms()` constructor. [#88]
- FIX: \[macOS\] Report `ITEM_CHANGE_OWNER` as `CHMOD` events. [#93]
- FIX: \[Linux\] Emit `CLOSE_WRITE` events. [#93]
- FEATURE: Allow recursion mode to be changed. [#60], [#61] **breaking**
- FEATURE: Track move events using a cookie.
- FEATURE: \[macOS\] Return an error when trying to unwatch non-existing file or directory.
- CHANGE: \[Linux\] Remove `IGNORED` event. **breaking**
- CHANGE: \[Linux\] Provide absolute paths even if the watch was created with a relative path.

[#60]: https://github.com/notify-rs/notify/issues/60
[#61]: https://github.com/notify-rs/notify/issues/61
[#63]: https://github.com/notify-rs/notify/issues/63
[#88]: https://github.com/notify-rs/notify/issues/88
[#90]: https://github.com/notify-rs/notify/issues/90
[#93]: https://github.com/notify-rs/notify/issues/93


## 2.6.3 (2016-08-05)

- FIX: \[macOS\] Bump `fsevents` version. [#91]

[#91]: https://github.com/notify-rs/notify/issues/91


## 2.6.2 (2016-07-05)

- FEATURE: \[macOS\] Implement Send and Sync for FsWatcher. [#82]
- FEATURE: \[Windows\] Implement Send and Sync for ReadDirectoryChangesWatcher. [#82]
- DOCS: Add example to monitor a given file or directory. [#77]

[#77]: https://github.com/notify-rs/notify/issues/77
[#82]: https://github.com/notify-rs/notify/issues/82


## 2.6.1 (2016-06-09)

- FIX: \[Linux\] Only register _directories_ for watching. [#74]
- DOCS: Update Readme example code. [`ccfb54be`]

[`ccfb54be`]: https://github.com/notify-rs/notify/commit/ccfb54bed3df7c4c5e0058566f10232e92b526a4
[#74]: https://github.com/notify-rs/notify/issues/74


## 2.6.0 (2016-06-06)

- Fix clippy lints. [#57]
- Run formatter. [#59]
- Fix warnings. [#58]
- FEATURE: Add `Result` alias. [#72]
- DEPS: Update inotify (0.2). [#49], [#70]
- DOCS: Write API docs. [#65]
- FEATURE: \[Linux\] Add op::IGNORED. [#73]
- Simplify Cargo.toml. [#75]
- FEATURE: Implement std::Error for our Error type. [#71]

[#57]: https://github.com/notify-rs/notify/issues/57
[#59]: https://github.com/notify-rs/notify/issues/59
[#58]: https://github.com/notify-rs/notify/issues/58
[#72]: https://github.com/notify-rs/notify/issues/72
[#49]: https://github.com/notify-rs/notify/issues/49
[#70]: https://github.com/notify-rs/notify/issues/70
[#65]: https://github.com/notify-rs/notify/issues/65
[#73]: https://github.com/notify-rs/notify/issues/73
[#75]: https://github.com/notify-rs/notify/issues/75
[#71]: https://github.com/notify-rs/notify/issues/71


## 2.5.5 (2016-03-27)

- DOCS: Explain an FSEvent limitation. [#51]
- DOCS: Clean up example code. [#52]
- RELENG: \[macOS\] Support i686. [#54]
- FEATURE: Implement `Display` on `Error`. [#56]

[#51]: https://github.com/notify-rs/notify/issues/51
[#52]: https://github.com/notify-rs/notify/issues/52
[#54]: https://github.com/notify-rs/notify/issues/54
[#56]: https://github.com/notify-rs/notify/issues/56


## 2.5.4 (2016-01-23)

- META: Remove all `*` specifiers to comply with Crates.io policy. [`9f44843f`]

[`9f44843f`]: https://github.com/notify-rs/notify/commit/9f44843f2e70b2e1fcb3ef8b0834692fe75a99a6


## 2.5.3 (2015-12-25)

- RELENG: \[Linux\] Support i686. [#46]

[#46]: https://github.com/notify-rs/notify/issues/46


## 2.5.2 (2015-12-21)

- META: Fix AppVeyor build. [#43], [#45]
- FIX: \[Linux\] Use a mio loop instead of handmade. [#40]
- DEPS: Replace walker by walkdir (0.1). [#44]

[#43]: https://github.com/notify-rs/notify/issues/43
[#45]: https://github.com/notify-rs/notify/issues/45
[#40]: https://github.com/notify-rs/notify/issues/40
[#44]: https://github.com/notify-rs/notify/issues/44


## 2.5.1 (2015-12-05)

- META: Update Code of Conduct. [`c963bdf0`]
- RELENG: Support musl. [#42]

[`c963bdf0`]: https://github.com/notify-rs/notify/commit/c963bdf0a94d951f5d11ca2a691eeb42746e721b
[#42]: https://github.com/notify-rs/notify/issues/42


## 2.5.0 (2015-11-29)

- FEATURE: Add Windows backend. [#39]
- META: Add AppVeyor CI. [`304473c3`]

[`304473c3`]: https://github.com/notify-rs/notify/commit/304473c32a76ec60bbcb20a1d673fa7c5879767d
[#39]: https://github.com/notify-rs/notify/issues/39


## 2.4.1 (2015-11-06)

- FIX: \[macOS\] Race condition in FSEvent. [#33]


## 2.4.0 (2015-10-25)

- FIX: \[macOS\] Stop segfault when watcher is moved. [#33], [#35]
- FEATURE: Add `::with_delay` to poll. [#34]

[#33]: https://github.com/notify-rs/notify/issues/33
[#35]: https://github.com/notify-rs/notify/issues/35
[#34]: https://github.com/notify-rs/notify/issues/34


## 2.3.3 (2015-10-08)

- FIX: Comply with Rust RFC 1214, adding `Sized` bound to trait. [#32]

[#32]: https://github.com/notify-rs/notify/issues/32


## 2.3.2 (2015-09-08)

- META: Use Travis CI Linux containers, macOS builds. [`7081297d`]
- FIX: \[macOS\] Symlinks and broken tests. [#27]

[`7081297d`]: https://github.com/notify-rs/notify/commit/7081297de6c557484e4cc7fbf8b2837a7d408870
[#27]: https://github.com/notify-rs/notify/issues/27


## 2.3.1 (2015-08-24)

- FIX: Move paths instead of borrowing. [`3340d740`]

[`3340d740`]: https://github.com/notify-rs/notify/commit/3340d7401230be5f5ed59956b29f8db3a1c12d1c


## 2.3.0 (2015-07-29)

- FEATURE: Use `AsRef<Path>` instead of `Path` in signatures. [#25]

[#25]: https://github.com/notify-rs/notify/issues/25


## 2.2.0 (2015-07-12)

- FEATURE: \[Linux\] Support watching single file. [#22]
- META: Change release commit message style to be just the version, instead of "Cut ${version}".

[#22]: https://github.com/notify-rs/notify/issues/22


## 2.1.0 (2015-06-26)

- META: Add Code of Conduct. [`4b88f7d9`]
- FEATURE: Restore Poll backend. [#12]
- FIX: \[Linux\] Inverse op::WRITE and op::REMOVE. [#18]
- DEPS: \[macOS\] Use fsevent (0.2). [#14]

[`4b88f7d9`]: https://github.com/notify-rs/notify/commit/4b88f7d9fcc4c41bb942c46c792f64afc848db2c
[#12]: https://github.com/notify-rs/notify/issues/12
[#18]: https://github.com/notify-rs/notify/issues/18
[#14]: https://github.com/notify-rs/notify/issues/14


## 2.0.0 (2015-06-09)

- RUST: 1.0 is out! Use stable, migrate to walker crate. [`0b127a38`]
- FEATURE: \[macOS\] FSEvent backend. [#13]
- BREAKING: Remove Poll backend. [`92936460`]

[`0b127a38`]: https://github.com/notify-rs/notify/commit/0b127a383072b0136bb44f74d5580abae01e7627
[`92936460`]: https://github.com/notify-rs/notify/commit/92936460070d4dd44090fc9e3b4c4150c2ef434c
[#13]: https://github.com/notify-rs/notify/issues/13


## 1.2.2 (2015-05-09)

- RUST: Update to latest, migrate to bitflags crate. [#11]

[#11]: https://github.com/notify-rs/notify/issues/11


## 1.2.1 (2015-04-03)

- RUST: Update to latest, thread upgrades. [#9]

[#9]: https://github.com/notify-rs/notify/issues/9


## 1.2.0 (2015-03-05)

- FEATURE: Provide full path to file that caused an event. [#8]

[#8]: https://github.com/notify-rs/notify/issues/8


## 1.1.3 (2015-02-18)

- META: Add Travis CI. [`2c865803`]
- DOCS: Build using Rust-CI. [`9c7dd960`]
- RUST: Update to latest, upgrade to new IO. [#5]
- FIX: \[Linux\] Keep watching when there are no events received. [#6]

[`2c865803`]: https://github.com/notify-rs/notify/commit/2c865803ba9d04227661e3cd320732da22526634
[`9c7dd960`]: https://github.com/notify-rs/notify/commit/9c7dd960f3f4e635046b6a1dd4b847c69dfd4f94
[#5]: https://github.com/notify-rs/notify/issues/5
[#6]: https://github.com/notify-rs/notify/issues/6


## 1.1.2 (2015-02-03)

- RUST: Update to latest, using `old_io`. [`116af0c4`]

[`116af0c4`]: https://github.com/notify-rs/notify/commit/116af0c4e00b5d5b268a9d69bba772cc1f2f67fa


## 1.1.1 (2015-01-06)

- RUST: Update to latest and fix changes to channels. [`327075c2`]

[`327075c2`]: https://github.com/notify-rs/notify/commit/327075c2ecd1d0c4123e9979aeebde4248015ef6


## 1.1.0 (2015-01-06)

- FEATURE: \[Linux\] Recursive watch. [#2]

[#2]: https://github.com/notify-rs/notify/issues/2


## 1.0.5 (2015-01-03)

- RELENG: Publish on crates.io. [`6f7d38a9`]
- RUST: Update to latest. [`fd78d0e4`]
- FIX: \[Linux\] Stop panic when inotify backend is dropped. [`6f7d38a9`]

[`6f7d38a9`]: https://github.com/notify-rs/notify/commit/6f7d38a94aead30c2f059bb6bea8bdcda542d4af
[`fd78d0e4`]: https://github.com/notify-rs/notify/commit/fd78d0e4b988d92d1be81e05e1812432ad476149


## 1.0.4 (2014-12-23)

- RUST: Update to latest. [`d45c954c`]

[`d45c954c`]: https://github.com/notify-rs/notify/commit/d45c954c03f2d4d948e252abdcf7136a139b378b


## 1.0.3 (2014-12-23)

- DEPS: Update inotify (0.1). [`fcda20bb`]

[`fcda20bb`]: https://github.com/notify-rs/notify/commit/fcda20bb17e9b2b30645350c57d9bfe2ce9b78d9


## 1.0.2 (2014-12-21)

- FIX: Build on non-Linux platforms. [`55c8d7b0`]

[`55c8d7b0`]: https://github.com/notify-rs/notify/commit/55c8d7b0bb7ae767b3032ccb57571de5d506e842


## 1.0.1 (2014-12-20)

- DOCS: Add example code to Readme. [`f14e83f3`]
- META: Add cargo metadata. [`f14e83f3`]

[`f14e83f3`]: https://github.com/notify-rs/notify/commit/f14e83f33df33f362f151525bce768911933d0dd


## 1.0.0 (2014-12-20)

- FEATURE: Invent Notify.
- FEATURE: \[Linux\] inotify backend. [`d4b61fd2`]
- FEATURE: Poll backend. [`b24a5339`]
- FEATURE: Recommended watcher selection mechanism. [`a9c60ebf`]

[`d4b61fd2`]: https://github.com/notify-rs/notify/commit/d4b61fd29c82880c473f20a2d7119977817530e0
[`b24a5339`]: https://github.com/notify-rs/notify/commit/b24a5339680792cbd7b4f25ad7ec23a04b2eba57
[`a9c60ebf`]: https://github.com/notify-rs/notify/commit/a9c60ebf0fcc630bd745dc7e5106a24311c5f1bf


## 0.0.1 (2014-12-11)

- Empty release (no code)
