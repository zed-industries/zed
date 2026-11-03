# Independent async work

This change lets Zed start independent file reads and completion-detail requests together. It limits active work to 16 navigation results, 16 completion items, or 16 file-comparison pairs. Each comparison pair can read two files. Result order stays the same where the caller needs it.

## Measurements

The tests run the actual Zed functions with a controlled 40 ms delay for each independent read or server request.

| Operation | Test input | Earlier implementation | Overlapping work |
| --- | --- | --- | --- |
| Go to definition | Three unopened files and one repeated result | 120 ms | 40 ms |
| Find references | Three unopened files and one repeated result | 120 ms | 40 ms |
| Completion details | Three server requests | 120 ms | 40 ms |
| One file comparison | Two file reads | 80 ms | 40 ms |
| Three comparisons | Six file reads | 240 ms | 40 ms |

The earlier implementation measurements came from the original checkout at `957673ad158277b26c5e702fa9765c61efa7d40f`. This PR carries the changes to `8a47b35109` and preserves its newer language-server capability checks. Single-file comparison code now lives in `git_ui_core`.

These numbers measure waiting under controlled conditions. They do not establish a whole-application speed change. Remote completion requests use the same scheduling code. Remote network performance was not measured. The limit of 16 bounds resource use. It is not claimed to be the best value for every machine.

## Behavior checks

The tests check result order, text ranges, repeated file results, language-server reuse, release of temporary file entries, and invalid definition targets. They also check that repeated completion indexes do not produce extra requests. Twenty completion items finish in two 40 ms intervals, with exactly 16 requests active at once. One failed completion request does not prevent successful items from updating.

File-comparison tests check old and new file contents. The multiple-file test also checks file order and the shared parent folder.

## Earlier full validation

The original checkout passed 425 tests. Four tests were already ignored. Its required code check and formatting checks passed. Parallel project test runs stalled in file-system notification tests on both the original and changed implementations. Running the full suite one test at a time passed.

A clean development build of that checkout also passed five checks in the running app:

- Go to Definition opened the correct function.
- Find All References showed the expected matches across three files.
- Both completion suggestions showed the correct signatures and documentation. The accepted suggestion produced valid Rust code.
- A single-file comparison showed the correct old value, new value, and added line.
- A three-file comparison showed all files in order with the correct changed values and added lines.

The earlier built app included separate local debugger changes. Those debugger changes are outside this PR. The earlier full-suite and built-app results do not establish validation of the port to current `main`.

## Current-main validation

The current-main port passed 748 tests across the project, Git interface, and shared Git interface packages. Four tests were already ignored. Tests ran one at a time inside each suite. The required `./script/clippy` check passed for all three affected packages, with all targets and all features. Its dependency check also passed. Formatting checks and `git diff --check` passed.

Seven timing and behavior tests also passed with 100 task execution orders each, for 700 runs. The port caught and fixed a repeated-file race: results for the same file address now share one file-open operation. This keeps repeated results attached to the same open copy of the file.

The focused command is:

```sh
cargo test -p project -p git_ui -p git_ui_core --features project/test-support concurrently -- --nocapture
```

The sweep uses `ITERATIONS=100`. Current `main` no longer has the old workspace-only definition-conversion argument, so that earlier test was not carried forward.

## Scan scope

The original text scan covered 1,807 Rust files and 22,569 lines containing `.await`. Manual review followed editing, navigation, search, folder scanning, Git, terminal, extension, and project-opening paths.

Many waits already overlap work. Some loops wait on tasks that started earlier. Other waits keep file writes, shared Git commands, text edits, terminal events, or save prompts in the required order.

Further candidates remain in startup path checks, project skill loading, extension asset loading, and task-menu preparation. Their actual time cost was not established. This PR changes the five cases with measured evidence.
