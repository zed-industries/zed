This a folder containing tests for `--depfile` parameter.
Each test is in a subfolder and defines a minimum CMake project,
which uses cbindgen to generate Rust bindings and the `--depfile`
parameter to determine when to regenerate.
The outer test can the build the project, assert that rebuilding does not regenerate the
bindings, and then assert that touching the files involved does trigger rebuilding.

The test project must contain an `expectations` folder, containing a file `dependencies`.
This `dependencies` should list all files that should be listed as dependencies in the generated
depfile. The paths should be relative to the project folder (i.e. to the folder containing 
`expectations`).
