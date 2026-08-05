@echo off
rem ============================================================================
rem win-setenv.cmd -- load the MSVC build environment needed to build Zed.
rem
rem Discovers the Visual Studio / Build Tools installation with vswhere (never a
rem hardcoded path -- Community, Professional, Enterprise and BuildTools all
rem live in different directories), calls vcvars64.bat, and then sets the few
rem Zed-specific variables documented in docs/src/development/windows.md.
rem
rem This script deliberately does NOT use `setlocal`: its entire purpose is to
rem mutate the environment of the calling cmd session.
rem
rem Usage:
rem   From a cmd prompt:  script\win-setenv.cmd
rem   Chained:            call script\win-setenv.cmd && cargo build --release
rem
rem Companion scripts: script\win-init.sh (one-time toolchain install),
rem                    script\win-build.sh (release build).
rem ============================================================================

rem --- Refuse to run if RUSTFLAGS is set -------------------------------------
rem A RUSTFLAGS env var overrides the `rustflags` in .cargo/config.toml, which
rem for this repo carries `--cfg windows_slim_errors` and
rem `-C target-feature=+crt-static`. Losing those produces linker failures and
rem other hard-to-diagnose errors.
rem See docs/src/development/windows.md#setting-rustflags-env-var-breaks-builds
if defined RUSTFLAGS goto :err_rustflags

rem --- Locate vswhere --------------------------------------------------------
set "_ZED_VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
if not exist "%_ZED_VSWHERE%" goto :err_no_vswhere

set "_ZED_VSINSTALL="
for /f "usebackq tokens=*" %%i in (`"%_ZED_VSWHERE%" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do set "_ZED_VSINSTALL=%%i"
if not defined _ZED_VSINSTALL goto :err_no_vs

rem --- Load the native x64 MSVC environment ---------------------------------
rem vcvars64.bat is preferred over VsDevCmd.bat / Launch-VsDevShell.ps1: it is
rem the direct native-x64 entry point and does not depend on PowerShell being
rem present, which matters for a BuildTools-only installation.
if defined VSCMD_VER goto :vs_already_loaded

set "_ZED_VCVARS=%_ZED_VSINSTALL%\VC\Auxiliary\Build\vcvars64.bat"
if not exist "%_ZED_VCVARS%" goto :err_no_vcvars

echo [win-setenv] Visual Studio: %_ZED_VSINSTALL%
rem NOTE: on a BuildTools-only installation vcvarsall.bat prints
rem   'vswhere.exe' is not recognized as an internal or external command
rem to stderr, because one of its internal probes invokes vswhere bare and the
rem Installer directory is not on PATH. It falls back correctly and still
rem returns 0 with a fully initialized environment -- the warning is Microsoft's,
rem not ours, and is safe to ignore.
call "%_ZED_VCVARS%"
if errorlevel 1 goto :err_vcvars
goto :after_vs

:vs_already_loaded
echo [win-setenv] MSVC environment already loaded (VSCMD_VER=%VSCMD_VER%); skipping vcvars64.

:after_vs

rem --- ZED_RC_TOOLKIT_PATH ---------------------------------------------------
rem Only needed when rc.exe is not already resolvable; some security policies
rem leave the Windows SDK bin directory off PATH.
rem See docs/src/development/windows.md#invalid-rc-path-selected
where rc.exe >nul 2>&1
if not errorlevel 1 goto :rc_done
if not defined WindowsSdkDir goto :warn_rc
if not defined WindowsSDKVersion goto :warn_rc
rem Both WindowsSdkDir and WindowsSDKVersion carry a trailing backslash.
set "ZED_RC_TOOLKIT_PATH=%WindowsSdkDir%bin\%WindowsSDKVersion%x64"
if not exist "%ZED_RC_TOOLKIT_PATH%\rc.exe" goto :warn_rc_clear
echo [win-setenv] ZED_RC_TOOLKIT_PATH=%ZED_RC_TOOLKIT_PATH%
goto :rc_done

:warn_rc_clear
set "ZED_RC_TOOLKIT_PATH="
:warn_rc
echo [win-setenv] WARNING: rc.exe not found and ZED_RC_TOOLKIT_PATH could not be derived.
echo [win-setenv]          See docs/src/development/windows.md#invalid-rc-path-selected
:rc_done

rem --- CMake -----------------------------------------------------------------
rem Required by wasmtime-c-api-impl. vcvars64 does not put the Visual Studio
rem copy of CMake on PATH, so fall back to it when there is no cmake already.
where cmake >nul 2>&1
if not errorlevel 1 goto :cmake_done
set "_ZED_CMAKE_BIN=%_ZED_VSINSTALL%\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin"
if not exist "%_ZED_CMAKE_BIN%\cmake.exe" goto :warn_cmake
set "PATH=%_ZED_CMAKE_BIN%;%PATH%"
echo [win-setenv] Added CMake to PATH: %_ZED_CMAKE_BIN%
goto :cmake_done

:warn_cmake
echo [win-setenv] WARNING: cmake not found. Install it, or add the Visual Studio
echo [win-setenv]          CMake component. Run script/win-init.sh to check.
:cmake_done

rem --- Done ------------------------------------------------------------------
set "_ZED_VSWHERE="
set "_ZED_VSINSTALL="
set "_ZED_VCVARS="
set "_ZED_CMAKE_BIN="
echo [win-setenv] Environment ready.
exit /b 0

rem --- Failure paths ---------------------------------------------------------
:err_rustflags
echo [win-setenv] ERROR: RUSTFLAGS is set to "%RUSTFLAGS%".
echo [win-setenv]        It overrides .cargo/config.toml and breaks the build.
echo [win-setenv]        Unset it ^(set "RUSTFLAGS="^) and add your flags to
echo [win-setenv]        .cargo/config.toml instead. See
echo [win-setenv]        docs/src/development/windows.md#setting-rustflags-env-var-breaks-builds
exit /b 1

:err_no_vswhere
echo [win-setenv] ERROR: vswhere.exe not found at:
echo [win-setenv]        %_ZED_VSWHERE%
echo [win-setenv]        Install Visual Studio or the C++ Build Tools, then run
echo [win-setenv]        script/win-init.sh
exit /b 1

:err_no_vs
echo [win-setenv] ERROR: no Visual Studio installation with the component
echo [win-setenv]        Microsoft.VisualStudio.Component.VC.Tools.x86.x64 was found.
echo [win-setenv]        Run script/win-init.sh to install what is missing.
exit /b 1

:err_no_vcvars
echo [win-setenv] ERROR: vcvars64.bat not found at:
echo [win-setenv]        %_ZED_VCVARS%
exit /b 1

:err_vcvars
echo [win-setenv] ERROR: vcvars64.bat failed.
exit /b 1
