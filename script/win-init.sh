#!/usr/bin/env bash
#
# win-init.sh -- check (and install) the Windows toolchain needed to build Zed.
#
# Verifies every dependency listed in docs/src/development/windows.md, and
# installs the missing Visual Studio components via vs_installer.
#
# The checks themselves are read-only and run fine unelevated; only the fixes
# need administrator rights, and each of those is launched through a UAC prompt
# rather than requiring you to start an elevated shell yourself.
#
# Run from Git Bash:
#   script/win-init.sh              # check, then install what is missing
#   script/win-init.sh --check-only # report only, change nothing
#
# Companion scripts: script/win-setenv.cmd (load build env),
#                    script/win-build.sh (release build).

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

CHECK_ONLY=0

usage() {
  sed -n '3,18p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
}

for arg in "$@"; do
  case "$arg" in
    --check-only | -c) CHECK_ONLY=1 ;;
    --help | -h)
      usage
      exit 0
      ;;
    *)
      echo "win-init: unknown option: $arg" >&2
      usage >&2
      exit 2
      ;;
  esac
done

# --- output helpers ----------------------------------------------------------

UNRESOLVED=()
REBOOT_REQUIRED=0

info() { printf '[win-init] %s\n' "$*"; }
ok() { printf '[win-init]   OK      %s\n' "$*"; }
bad() { printf '[win-init]   MISSING %s\n' "$*"; }
warn() { printf '[win-init]   WARN    %s\n' "$*"; }

unresolved() { UNRESOLVED+=("$1"); }

# Runs a PowerShell script body elevated, via a UAC prompt, and propagates its
# exit code. Writing the body to a temp file avoids the quoting minefield of
# nesting a command string inside Start-Process -ArgumentList.
#
# NOTE: Start-Process joins -ArgumentList entries with spaces and does NOT quote
# them, so any entry containing a space must carry its own embedded quotes or
# the callee sees it split into several arguments. Hence the \"...\" around the
# path below (and around --installPath in the vs_installer call further down).
run_elevated_ps() {
  local body="$1" tmp win_tmp rc=0
  tmp="$(mktemp -t win-init-XXXXXX.ps1)"
  printf '%s\n' "$body" >"$tmp"
  win_tmp="$(cygpath -w "$tmp")"

  powershell.exe -NoProfile -ExecutionPolicy Bypass -Command \
    "\$p = Start-Process -FilePath 'powershell.exe' -Verb RunAs -Wait -PassThru -ArgumentList '-NoProfile','-ExecutionPolicy','Bypass','-File','\"$win_tmp\"'; exit \$p.ExitCode" ||
    rc=$?

  rm -f "$tmp"
  return $rc
}

# --- environment discovery ---------------------------------------------------

PROGRAM_FILES_X86="$(cygpath -u "${PROGRAMFILES__x86_:-C:\\Program Files (x86)}" 2>/dev/null || echo "/c/Program Files (x86)")"
[ -d "$PROGRAM_FILES_X86" ] || PROGRAM_FILES_X86="/c/Program Files (x86)"

VSWHERE="$PROGRAM_FILES_X86/Microsoft Visual Studio/Installer/vswhere.exe"
VS_INSTALLER="$PROGRAM_FILES_X86/Microsoft Visual Studio/Installer/vs_installer.exe"

case "$(uname -m)" in
  x86_64) SPECTRE_LIB_DIR="x64" SPECTRE_ID_SUFFIX="x86.x64" ;;
  aarch64 | arm64) SPECTRE_LIB_DIR="arm64" SPECTRE_ID_SUFFIX="ARM64" ;;
  *)
    echo "win-init: unsupported host architecture: $(uname -m)" >&2
    exit 1
    ;;
esac

info "Repository: $REPO_ROOT"
info "Host architecture: $(uname -m)"
echo

# --- 1. rustup ---------------------------------------------------------------

info "Rust toolchain"
if command -v rustup >/dev/null 2>&1; then
  ok "rustup ($(rustup --version 2>/dev/null | head -1))"
  # rust-toolchain.toml pins the channel, so rustup installs it on first use.
  ok "toolchain pinned by rust-toolchain.toml ($(sed -n 's/^channel = "\(.*\)"/\1/p' rust-toolchain.toml))"
else
  bad "rustup not found"
  unresolved "Install rustup from https://www.rust-lang.org/tools/install (or: winget install Rustlang.Rustup)"
fi
echo

# --- 2. Visual Studio / Build Tools ------------------------------------------

info "Visual Studio C++ toolchain"
VSINSTALL=""
if [ ! -f "$VSWHERE" ]; then
  bad "vswhere.exe not found at $VSWHERE"
  unresolved "Install Visual Studio or the C++ Build Tools: https://visualstudio.microsoft.com/visual-cpp-build-tools/"
else
  VSINSTALL="$("$VSWHERE" -latest -products '*' \
    -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 \
    -property installationPath 2>/dev/null | tr -d '\r' | head -1)"
  if [ -z "$VSINSTALL" ]; then
    bad "no installation with Microsoft.VisualStudio.Component.VC.Tools.x86.x64"
    unresolved "Install the 'Desktop development with C++' workload"
  else
    ok "installation: $VSINSTALL"
  fi
fi

VSINSTALL_UNIX=""
TOOLSET=""
if [ -n "$VSINSTALL" ]; then
  VSINSTALL_UNIX="$(cygpath -u "$VSINSTALL")"
  # The authoritative default toolset, i.e. the one vcvars64/cl.exe will use.
  TOOLSET_FILE="$VSINSTALL_UNIX/VC/Auxiliary/Build/Microsoft.VCToolsVersion.default.txt"
  if [ -f "$TOOLSET_FILE" ]; then
    TOOLSET="$(tr -d ' \r\n' <"$TOOLSET_FILE")"
    ok "default MSVC toolset: $TOOLSET"
  else
    warn "could not read $TOOLSET_FILE"
  fi
fi
echo

# --- 3. Spectre-mitigated libs -----------------------------------------------
#
# This is the one that bites hardest: the `msvc_spectre_libs` crate (pulled in
# by microsoft/python-environment-tools -> crates/languages -> zed) enables its
# `error` feature, so its build script *panics* rather than warns when the libs
# are absent. Its build.rs resolves cl.exe and then tests
#   <cl.exe dir>\..\..\..\..\lib\spectre\{arch}
# so we check that exact path -- an installer that exits 0 is not proof.

info "Spectre-mitigated CRT libs"
SPECTRE_PATH=""
spectre_present() {
  [ -n "$SPECTRE_PATH" ] && [ -d "$SPECTRE_PATH" ]
}

if [ -n "$VSINSTALL_UNIX" ] && [ -n "$TOOLSET" ]; then
  SPECTRE_PATH="$VSINSTALL_UNIX/VC/Tools/MSVC/$TOOLSET/lib/spectre/$SPECTRE_LIB_DIR"
fi

if [ -z "$SPECTRE_PATH" ]; then
  warn "skipped (no Visual Studio installation to check)"
elif spectre_present; then
  ok "$SPECTRE_PATH"
else
  bad "$SPECTRE_PATH"

  # Resolve the toolset-pinned component id from the installer catalog, e.g.
  # toolset 14.44.35207 -> Microsoft.VisualStudio.Component.VC.14.44.17.14.x86.x64.Spectre
  # The floating "latest" id below comes from docs/src/development/windows.md.
  SPECTRE_IDS=("Microsoft.VisualStudio.Component.VC.Runtimes.${SPECTRE_ID_SUFFIX}.Spectre")
  TOOLSET_MM="$(printf '%s' "$TOOLSET" | cut -d. -f1,2)"
  if [ -n "$TOOLSET_MM" ]; then
    PROGRAM_DATA="$(cygpath -u "${PROGRAMDATA:-C:\\ProgramData}" 2>/dev/null || echo /c/ProgramData)"
    for catalog in "$PROGRAM_DATA"/Microsoft/VisualStudio/Packages/_Instances/*/catalog.json; do
      [ -f "$catalog" ] || continue
      pinned="$(grep -o "Microsoft\.VisualStudio\.Component\.VC\.${TOOLSET_MM//./\\.}\.[0-9]\+\.[0-9]\+\.${SPECTRE_ID_SUFFIX//./\\.}\.Spectre" "$catalog" | sort -u | head -1 || true)"
      if [ -n "$pinned" ]; then
        SPECTRE_IDS+=("$pinned")
        break
      fi
    done
  fi
  info "  components to add: ${SPECTRE_IDS[*]}"

  if [ "$CHECK_ONLY" -eq 1 ]; then
    unresolved "Spectre libs missing; re-run without --check-only to install"
  elif [ ! -f "$VS_INSTALLER" ]; then
    bad "vs_installer.exe not found at $VS_INSTALLER"
    unresolved "Add these components manually via the Visual Studio Installer: ${SPECTRE_IDS[*]}"
  else
    add_args=""
    for id in "${SPECTRE_IDS[@]}"; do
      add_args="$add_args, '--add', '$id'"
    done

    info "  launching the Visual Studio Installer (a UAC prompt will appear)..."
    info "  this can take several minutes; the installer runs quietly"
    # --installPath carries spaces, so it needs embedded quotes -- see the note
    # on run_elevated_ps. Without them the installer receives "C:\Program" and
    # fails with "no installed product matches the given parameters".
    ps_body="\$ErrorActionPreference = 'Stop'
\$p = Start-Process -FilePath '$(cygpath -w "$VS_INSTALLER")' -Wait -PassThru -ArgumentList @(
  'modify', '--installPath', '\"$VSINSTALL\"'${add_args}, '--quiet', '--norestart'
)
exit \$p.ExitCode"

    rc=0
    run_elevated_ps "$ps_body" || rc=$?
    case "$rc" in
      0) info "  installer finished" ;;
      3010)
        info "  installer finished; a reboot is required"
        REBOOT_REQUIRED=1
        ;;
      1602) warn "installer was cancelled at the UAC or installer prompt" ;;
      *)
        warn "installer exited with code $rc"
        warn "diagnose with the newest log: %TEMP%\\dd_installer_*.log"
        ;;
    esac

    # Verify by path, not by exit code.
    if spectre_present; then
      ok "$SPECTRE_PATH"
    else
      bad "still missing after install: $SPECTRE_PATH"
      unresolved "Open the Visual Studio Installer -> Modify -> Individual components and tick
             'C++ ... Spectre-mitigated libs (Latest)' for your toolset, then re-run this script.
             Expected path: $SPECTRE_PATH"
    fi
  fi
fi
echo

# --- 4. Windows SDK ----------------------------------------------------------

info "Windows SDK"
SDK_MIN="10.0.20348.0"
SDK_INCLUDE="$PROGRAM_FILES_X86/Windows Kits/10/Include"
if [ ! -d "$SDK_INCLUDE" ]; then
  bad "no Windows 10/11 SDK found under $SDK_INCLUDE"
  unresolved "Install the Windows SDK: https://developer.microsoft.com/windows/downloads/windows-sdk/"
else
  SDK_BEST="$(find "$SDK_INCLUDE" -maxdepth 1 -mindepth 1 -type d -printf '%f\n' 2>/dev/null |
    grep -E '^10\.[0-9.]+$' | sort -V | tail -1 || true)"
  if [ -z "$SDK_BEST" ]; then
    bad "no versioned SDK directory under $SDK_INCLUDE"
    unresolved "Install Windows 10 SDK $SDK_MIN or newer"
  elif [ "$(printf '%s\n%s\n' "$SDK_MIN" "$SDK_BEST" | sort -V | head -1)" = "$SDK_MIN" ]; then
    ok "$SDK_BEST (>= $SDK_MIN)"
  else
    bad "$SDK_BEST is older than the required $SDK_MIN"
    unresolved "Install Windows 10 SDK $SDK_MIN or newer"
  fi
fi
echo

# --- 5. CMake ----------------------------------------------------------------

info "CMake (required by wasmtime-c-api-impl)"
if command -v cmake >/dev/null 2>&1; then
  ok "$(cmake --version 2>/dev/null | head -1)"
elif [ -n "$VSINSTALL_UNIX" ] &&
  [ -x "$VSINSTALL_UNIX/Common7/IDE/CommonExtensions/Microsoft/CMake/CMake/bin/cmake.exe" ]; then
  ok "bundled with Visual Studio; script/win-setenv.cmd will put it on PATH"
else
  bad "cmake not on PATH and not bundled with Visual Studio"
  unresolved "Install CMake (https://cmake.org/download) or add the Visual Studio CMake component"
fi
echo

# --- 6. Long path support ----------------------------------------------------
#
# Without these, fetching the `pet` git dependency fails with
# "path too long ... class=Filesystem (30)".
# See docs/src/development/windows.md#build-fails-path-too-long

info "Long path support"
GIT_LONGPATHS="$(git config --system --get core.longpaths 2>/dev/null | tr -d '\r' || true)"
REG_LONGPATHS="$(reg query 'HKLM\SYSTEM\CurrentControlSet\Control\FileSystem' //v LongPathsEnabled 2>/dev/null |
  grep -o '0x[0-9a-fA-F]*' | head -1 || true)"

NEED_GIT_LONGPATHS=0
NEED_REG_LONGPATHS=0

if [ "$GIT_LONGPATHS" = "true" ]; then
  ok "git core.longpaths (system) = true"
else
  bad "git core.longpaths (system) is not set to true"
  NEED_GIT_LONGPATHS=1
fi

if [ "$REG_LONGPATHS" = "0x1" ]; then
  ok "HKLM ... FileSystem\\LongPathsEnabled = 1"
else
  bad "HKLM ... FileSystem\\LongPathsEnabled is not 1"
  NEED_REG_LONGPATHS=1
fi

if [ "$NEED_GIT_LONGPATHS" -eq 1 ] || [ "$NEED_REG_LONGPATHS" -eq 1 ]; then
  if [ "$CHECK_ONLY" -eq 1 ]; then
    unresolved "Long path support not enabled; re-run without --check-only to enable"
  else
    ps_lines="\$ErrorActionPreference = 'Stop'"
    if [ "$NEED_GIT_LONGPATHS" -eq 1 ]; then
      # Call git by absolute path: Git Bash resolves git to /mingw64/bin/git,
      # a directory that is not normally on the Windows PATH, so a bare `git`
      # in the elevated session may not resolve at all. And check $LASTEXITCODE
      # explicitly -- $ErrorActionPreference does not trap native command
      # failures in Windows PowerShell, so a failure here would otherwise be
      # swallowed by the final `exit 0`.
      GIT_EXE="$(command -v git)"
      [ -f "$GIT_EXE.exe" ] && GIT_EXE="$GIT_EXE.exe"
      ps_lines="$ps_lines
& '$(cygpath -w "$GIT_EXE")' config --system core.longpaths true
if (\$LASTEXITCODE -ne 0) { exit \$LASTEXITCODE }"
    fi
    [ "$NEED_REG_LONGPATHS" -eq 1 ] &&
      ps_lines="$ps_lines
New-ItemProperty -Path 'HKLM:\\SYSTEM\\CurrentControlSet\\Control\\FileSystem' -Name 'LongPathsEnabled' -Value 1 -PropertyType DWORD -Force | Out-Null"
    ps_lines="$ps_lines
exit 0"

    info "  enabling long path support (a UAC prompt will appear)..."
    rc=0
    run_elevated_ps "$ps_lines" || rc=$?
    [ "$rc" -eq 0 ] || warn "elevated step exited with code $rc"

    # Re-read the real state rather than trusting the exit code.
    still_missing=0
    if [ "$NEED_GIT_LONGPATHS" -eq 1 ]; then
      if [ "$(git config --system --get core.longpaths 2>/dev/null | tr -d '\r' || true)" = "true" ]; then
        ok "git core.longpaths (system) = true"
      else
        bad "git core.longpaths (system) still not set"
        still_missing=1
      fi
    fi
    if [ "$NEED_REG_LONGPATHS" -eq 1 ]; then
      if reg query 'HKLM\SYSTEM\CurrentControlSet\Control\FileSystem' //v LongPathsEnabled 2>/dev/null |
        grep -q '0x1'; then
        ok "HKLM ... FileSystem\\LongPathsEnabled = 1"
        REBOOT_REQUIRED=1
      else
        bad "HKLM ... FileSystem\\LongPathsEnabled still not 1"
        still_missing=1
      fi
    fi
    [ "$still_missing" -eq 1 ] &&
      unresolved "Enable long paths manually, see docs/src/development/windows.md#build-fails-path-too-long"
  fi
fi
echo

# --- 7. RUSTFLAGS ------------------------------------------------------------

info "Environment sanity"
if [ -n "${RUSTFLAGS:-}" ]; then
  bad "RUSTFLAGS is set to '$RUSTFLAGS'"
  unresolved "Unset RUSTFLAGS; it overrides .cargo/config.toml and breaks the build.
             See docs/src/development/windows.md#setting-rustflags-env-var-breaks-builds"
else
  ok "RUSTFLAGS is not set"
fi
echo

# --- summary -----------------------------------------------------------------

if [ "${#UNRESOLVED[@]}" -eq 0 ]; then
  info "All checks passed."
  if [ "$REBOOT_REQUIRED" -eq 1 ]; then
    echo
    info "A REBOOT IS REQUIRED before the changes above take effect."
    exit 0
  fi
  info "Next: script/win-build.sh"
  exit 0
fi

info "${#UNRESOLVED[@]} item(s) still need attention:"
for item in "${UNRESOLVED[@]}"; do
  printf '[win-init]   - %s\n' "$item"
done
if [ "$REBOOT_REQUIRED" -eq 1 ]; then
  echo
  info "A REBOOT IS REQUIRED before some of the changes above take effect."
fi
exit 1
