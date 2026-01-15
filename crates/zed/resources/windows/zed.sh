#!/usr/bin/env sh

if [ "$ZED_WSL_DEBUG_INFO" = true ]; then
	set -x
fi

if command -v realpath >/dev/null 2>&1; then
	ZED_PATH="$(dirname "$(realpath "$0")")"
elif command -v readlink >/dev/null 2>&1; then
	ZED_PATH="$(dirname "$(readlink -f "$0")")"
else
	ZED_PATH="$(cd "$(dirname "$0")" && pwd)"
fi

ZED_EXE="$ZED_PATH/zed.exe"
if [ ! -f "$ZED_EXE" ]; then
	echo "Zed CLI not found at $ZED_EXE. Reinstall Zed or run zed.exe directly." >&2
	exit 127
fi

IN_WSL=false
if [ -n "$WSL_DISTRO_NAME" ]; then
	# $WSL_DISTRO_NAME is available since WSL builds 18362, also for WSL2
	IN_WSL=true
fi

if [ $IN_WSL = true ]; then
	if [ -f /proc/sys/fs/binfmt_misc/WSLInterop ]; then
		if ! grep -qi enabled /proc/sys/fs/binfmt_misc/WSLInterop; then
			echo "WSL interop is disabled. Enable it to launch Windows apps." >&2
			exit 126
		fi
	fi
	WSL_USER="$USER"
	if [ -z "$WSL_USER" ]; then
		WSL_USER="$USERNAME"
	fi
	if [ -n "$WSL_USER" ]; then
		"$ZED_EXE" --wsl "$WSL_USER@$WSL_DISTRO_NAME" "$@"
	else
		"$ZED_EXE" --wsl "$WSL_DISTRO_NAME" "$@"
	fi
	exit $?
else
	"$ZED_EXE" "$@"
	exit $?
fi
