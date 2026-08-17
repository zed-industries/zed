cd single-crate
rd /s /q proptest-regressions
cargo test >cargo.txt
cargo clean >nul
if not exist proptest-regressions/submodule/code.txt goto fail

cd ..\workspace
rd /s /q proptest-regressions
cargo test --all >cargo.txt
cargo clean >nul
if not exist member/proptest-regressions/submodule/code.txt goto fail
cd ..

echo All persistence files written to correct location.
echo PASS
exit /b

:fail
echo Persistence file not in expected location. FS:
dir /s
echo Cargo output:
type cargo.txt
exit /b 1
