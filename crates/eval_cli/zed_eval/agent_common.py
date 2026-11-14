from __future__ import annotations

import json
import os
import shlex
from collections.abc import Callable
from pathlib import Path
from typing import Any

PROVIDER_ENV_MAP = {
    "anthropic": "ANTHROPIC_API_KEY",
    "openai": "OPENAI_API_KEY",
    "google": "GEMINI_API_KEY",
    "gemini": "GEMINI_API_KEY",
    "deepseek": "DEEPSEEK_API_KEY",
    "mistral": "MISTRAL_API_KEY",
}


def provider_api_env(
    model_name: str | None,
    get_env: Callable[[str], str | None] = os.environ.get,
) -> dict[str, str]:
    env: dict[str, str] = {}
    if not model_name or "/" not in model_name:
        return env

    provider = model_name.split("/", 1)[0]
    env_var = PROVIDER_ENV_MAP.get(provider) or (
        f"{provider}_API_KEY".upper().replace("-", "_")
    )
    api_key = get_env(env_var)
    if api_key:
        env[env_var] = api_key
    return env


def add_openai_compatible_provider_env(
    env: dict[str, str], providers_json: str | None
) -> None:
    if providers_json:
        env["ZED_OPENAI_COMPATIBLE_PROVIDERS"] = providers_json


def add_anthropic_available_models_env(
    env: dict[str, str], models_json: str | None
) -> None:
    if models_json:
        env["ZED_ANTHROPIC_AVAILABLE_MODELS"] = models_json


def add_zed_eval_env(
    env: dict[str, str], extra_env: dict[str, str], *, exclude: set[str] | None = None
) -> None:
    exclude = exclude or set()
    for key, value in extra_env.items():
        if key.startswith("ZED_EVAL_") and key not in exclude:
            env[key] = value


async def detect_workdir(
    environment: Any,
    exec_as_agent: Callable[..., Any],
    get_env: Callable[[str], str | None],
    error_message: str,
) -> str:
    override = get_env("EVAL_CLI_WORKDIR")
    if override:
        return override

    result = await exec_as_agent(
        environment,
        command=(
            "for d in /app /testbed /repo; do "
            '  if [ -d "$d/.git" ]; then echo "$d"; exit 0; fi; '
            "done; "
            "find / -maxdepth 3 -name .git -type d 2>/dev/null "
            '| head -1 | sed "s|/.git$||"'
        ),
    )
    workdir = (result.stdout or "").strip()
    if workdir:
        return workdir

    result = await exec_as_agent(
        environment,
        command=(
            "for d in /app /testbed /repo /root /home; do "
            '  if [ -d "$d" ]; then echo "$d"; exit 0; fi; '
            "done; pwd"
        ),
    )
    workdir = (result.stdout or "").strip()
    if workdir:
        return workdir
    raise RuntimeError(error_message)


def populate_context_from_result(logs_dir: Path, context: Any, logger: Any) -> None:
    result_data = None
    for json_file in logs_dir.rglob("result.json"):
        try:
            result_data = json.loads(json_file.read_text())
            break
        except (json.JSONDecodeError, OSError):
            continue

    if result_data is None:
        logger.warning("Could not find or parse result.json from eval-cli")
        return

    if result_data.get("input_tokens") is not None:
        context.n_input_tokens = result_data["input_tokens"]
    if result_data.get("output_tokens") is not None:
        context.n_output_tokens = result_data["output_tokens"]
    if result_data.get("cache_read_input_tokens") is not None:
        context.n_cache_tokens = result_data["cache_read_input_tokens"]
    if isinstance(result_data.get("step_count"), int):
        context.n_agent_steps = result_data["step_count"]

    context.metadata = {
        "status": result_data.get("status"),
        "duration_secs": result_data.get("duration_secs"),
        "model": result_data.get("model"),
        "tool_call_count": result_data.get("tool_call_count"),
    }


def eval_cli_with_log_command(
    parts: list[str],
    log_path: str,
    *,
    timeout_message: str | None = None,
    line_buffered: bool = False,
) -> str:
    """Run eval-cli, tee output, and preserve eval-cli's exit status.

    POSIX shells return the last command's status for a pipeline, so `cmd | tee`
    would otherwise hide eval-cli failures whenever `tee` succeeds.
    """
    status_file = "/tmp/zed-eval-eval-cli-status"
    quoted_status_file = shlex.quote(status_file)
    quoted_log_path = shlex.quote(log_path)
    timeout_handler = 'if [ "$ec" -eq 2 ]; then ec=0; fi; '
    if timeout_message:
        timeout_handler = (
            f'if [ "$ec" -eq 2 ]; then echo {shlex.quote(timeout_message)}; ec=0; fi; '
        )
    tee_command = f"tee {quoted_log_path}"
    if line_buffered:
        tee_command = (
            "if command -v stdbuf >/dev/null 2>&1; "
            f"then stdbuf -oL tee {quoted_log_path}; "
            f"else tee {quoted_log_path}; fi"
        )
    return (
        f'status_file={quoted_status_file}; rm -f "$status_file"; '
        "( "
        + " ".join(parts)
        + "; ec=$?; "
        + timeout_handler
        + 'printf "%s\\n" "$ec" > "$status_file"; '
        + 'exit "$ec" ) 2>&1 | '
        + tee_command
        + '; ec=1; if [ -s "$status_file" ]; then read ec < "$status_file"; fi; '
        + 'rm -f "$status_file"; exit "$ec"'
    )


def patch_command(agent_dir: str) -> str:
    patch_path = shlex.quote(f"{agent_dir}/patch.diff")
    return (
        "if git rev-parse --git-dir >/dev/null 2>&1; then "
        "git add -A && "
        "if git rev-parse --verify HEAD >/dev/null 2>&1; then "
        f"git diff --cached HEAD -- > {patch_path} && "
        f'echo "Patch size: $(wc -c < {patch_path}) bytes"; '
        "else "
        'echo "Git repo has no valid HEAD, skipping patch generation"; '
        "fi; "
        "else "
        'echo "No git repo found, skipping patch generation"; '
        "fi"
    )
