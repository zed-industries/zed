#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROFILE_ROOT="${ZED_DEBUG_PROFILE_ROOT:-/tmp/zed-cursor-tail-profile}"
DEFAULT_PROFILE="silk"

usage() {
  cat <<'EOF'
Usage:
  ./start-debug.sh [profile] [-- <zed args>]

Profiles:
  classic  Shader-like defaults from cursor_tail.glsl.
  silk     Smooth and elegant profile.
  comet    Long, airy trail with subtle teal tint.
  swift    Fast, snappy response profile.
  custom   Uses environment variables below.

Custom profile environment variables:
  ZED_CURSOR_TAIL_DURATION_MS
  ZED_CURSOR_TAIL_MIN_DISTANCE_MULTIPLIER
  ZED_CURSOR_TAIL_MAX_LENGTH_MULTIPLIER
  ZED_CURSOR_TAIL_OPACITY
  ZED_CURSOR_TAIL_COLOR (hex like #9ED8FF, or empty for null)

Examples:
  ./start-debug.sh
  ./start-debug.sh classic
  ./start-debug.sh comet -- /Users/you/project
  ZED_DEBUG_PROFILE_ROOT=/tmp/zed-dev ./start-debug.sh silk
EOF
}

copy_tree() {
  local source_dir="$1"
  local destination_dir="$2"
  mkdir -p "$destination_dir"
  if command -v rsync >/dev/null 2>&1; then
    rsync -a "${source_dir}/" "${destination_dir}/"
  else
    cp -R "${source_dir}/." "${destination_dir}/"
  fi
}

sanitize_settings_json() {
  local source_file="$1"
  local destination_file="$2"
  perl -0pe 's{/\*.*?\*/}{}gs; s{^\s*//.*$}{}mg; s/,\s*([}\]])/$1/g' "$source_file" > "$destination_file"
}

profile="${DEFAULT_PROFILE}"

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  usage
  exit 0
fi

if [[ "${1:-}" == "--list-profiles" ]]; then
  printf '%s\n' "classic" "silk" "comet" "swift" "custom"
  exit 0
fi

if [[ $# -gt 0 && "${1}" != "--" && "${1}" != -* ]]; then
  profile="$1"
  shift
fi

if [[ "${1:-}" == "--" ]]; then
  shift
fi

case "$profile" in
  classic)
    cursor_tail_json='{"enabled":true,"profile":"classic","duration_ms":90,"minimum_distance_multiplier":1.5,"max_length_multiplier":6.0,"opacity":1.0,"color":null}'
    ;;
  silk)
    cursor_tail_json='{"enabled":true,"profile":"silk","duration_ms":96,"minimum_distance_multiplier":1.0,"max_length_multiplier":8.0,"opacity":0.78,"color":"#9ED8FF"}'
    ;;
  comet)
    cursor_tail_json='{"enabled":true,"profile":"comet","duration_ms":130,"minimum_distance_multiplier":0.75,"max_length_multiplier":11.0,"opacity":0.64,"color":"#8BF6D5"}'
    ;;
  swift)
    cursor_tail_json='{"enabled":true,"profile":"swift","duration_ms":64,"minimum_distance_multiplier":1.25,"max_length_multiplier":5.3,"opacity":0.92,"color":"#FFD28F"}'
    ;;
  custom)
    custom_duration_ms="${ZED_CURSOR_TAIL_DURATION_MS:-90}"
    custom_minimum_distance_multiplier="${ZED_CURSOR_TAIL_MIN_DISTANCE_MULTIPLIER:-1.5}"
    custom_max_length_multiplier="${ZED_CURSOR_TAIL_MAX_LENGTH_MULTIPLIER:-6.0}"
    custom_opacity="${ZED_CURSOR_TAIL_OPACITY:-1.0}"
    custom_color="${ZED_CURSOR_TAIL_COLOR:-}"
    if [[ -n "$custom_color" ]]; then
      custom_color_json="\"${custom_color}\""
    else
      custom_color_json="null"
    fi
    cursor_tail_json="{\"enabled\":true,\"profile\":\"custom\",\"duration_ms\":${custom_duration_ms},\"minimum_distance_multiplier\":${custom_minimum_distance_multiplier},\"max_length_multiplier\":${custom_max_length_multiplier},\"opacity\":${custom_opacity},\"color\":${custom_color_json}}"
    ;;
  *)
    echo "Unknown profile: $profile" >&2
    usage >&2
    exit 1
    ;;
esac

profile_dir="${PROFILE_ROOT}/${profile}"
profile_config_dir="${profile_dir}/config"
settings_file="${profile_config_dir}/settings.json"

rm -rf "$profile_dir"
mkdir -p "$profile_config_dir"

if [[ -d "$HOME/.config/zed" ]]; then
  copy_tree "$HOME/.config/zed" "$profile_config_dir"
elif [[ -d "$HOME/.zed" ]]; then
  copy_tree "$HOME/.zed" "$profile_config_dir"
fi

if [[ -d "$HOME/Library/Application Support/Zed" ]]; then
  copy_tree "$HOME/Library/Application Support/Zed" "$profile_dir"
fi

if [[ -f "$settings_file" ]]; then
  sanitized_settings_file="$(mktemp)"
  sanitize_settings_json "$settings_file" "$sanitized_settings_file"
  if jq -e . "$sanitized_settings_file" >/dev/null 2>&1; then
    settings_tmp="$(mktemp)"
    jq \
      --argjson cursor_tail "$cursor_tail_json" \
      '.cursor_shape = "block" | .cursor_tail = $cursor_tail' \
      "$sanitized_settings_file" > "$settings_tmp"
    mv "$settings_tmp" "$settings_file"
  else
    cp "$settings_file" "${settings_file}.original"
    cat > "$settings_file" <<EOF
{
  "cursor_shape": "block",
  "cursor_tail": ${cursor_tail_json}
}
EOF
  fi
  rm -f "$sanitized_settings_file"
else
  cat > "$settings_file" <<EOF
{
  "cursor_shape": "block",
  "cursor_tail": ${cursor_tail_json}
}
EOF
fi

zed_binary="${ZED_DEBUG_BIN:-${SCRIPT_DIR}/target/debug/zed}"
if [[ ! -x "$zed_binary" ]]; then
  echo "Building Zed debug binary with gpui/runtime_shaders..."
  (cd "$SCRIPT_DIR" && cargo build -p zed --features gpui/runtime_shaders)
fi

echo "Launching profile '${profile}' with isolated data dir: ${profile_dir}"
echo "Applied cursor-tail preset to: ${settings_file}"
exec "$zed_binary" --user-data-dir "$profile_dir" "$@"
