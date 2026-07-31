from __future__ import annotations

import shlex
from dataclasses import dataclass

DEFAULT_APP_NAME = "agent-evals"
DEFAULT_VOLUME_NAME = "agent-evals"
# Modal secret holding the LLM provider keys (ANTHROPIC_API_KEY, BASETEN_API_KEY,
# ...) mounted into trial sandboxes.
DEFAULT_LLM_PROVIDERS_SECRET_NAME = "agent-evals-llm-providers"
# Modal secret holding the controller's Modal token (MODAL_TOKEN_ID/SECRET).
DEFAULT_MODAL_TOKEN_SECRET_NAME = "agent-evals-modal-token"
HARBOR_VERSION = "0.15.0"
MODAL_VERSION = "1.5.0"
CONTROLLER_IMAGE_RECIPE_VERSION = "agent-evals-controller-image-v1"
DEFAULT_MODEL = "anthropic/claude-sonnet-4-6"
DEFAULT_JUDGE_PRESET = "auto"
BASETEN_API_URL = "https://inference.baseten.co/v1"
BASETEN_PROVIDER_ID = "baseten"
BASETEN_DEFAULT_MAX_TOKENS = 262_144
BASETEN_DEFAULT_MAX_OUTPUT_TOKENS = 65_536

# Kept modest on purpose: the agent model's API key is the real bottleneck,
# not Modal capacity. Snickerdoodle-EAP is capped at 2M ITPM / 400K OTPM, and
# ~150 concurrent agent loops (3 jobs x 50) blew past that. One job at this
# concurrency stays well under the ceiling; raise only if you've confirmed the
# model's rate limits have headroom.
DEFAULT_N_CONCURRENT = 25
DEFAULT_SANDBOX_TIMEOUT_SECS = 14_400
DEFAULT_SANDBOX_IDLE_TIMEOUT_SECS = 300
# No per-task resource override by default. When these are None the
# --override-cpus/--override-memory-mb flags are omitted entirely, so Harbor
# applies each task's declared cpus/memory (SWE-Atlas tasks declare 16 CPU /
# 16 GB). Overriding *below* the declared values OOM-kills memory-heavy tasks
# (SIGKILL / exit 137) and, per the resource policy, changes the experimental
# conditions. Pass --override-cpus/--override-memory-mb to opt into smaller
# sandboxes deliberately.
DEFAULT_OVERRIDE_CPUS = None
DEFAULT_OVERRIDE_MEMORY_MB = None
EVAL_BASE_URL_IN_SANDBOX = "http://127.0.0.1:8089/v1"
JUDGE_PROXY_VERIFIER_IMPORT_PATH = "zed_eval.verifier:ZedJudgeProxyVerifier"


@dataclass(frozen=True)
class JudgeConfig:
    model: str
    upstream: str
    auth_env: str
    # Floor for the judge's max_completion_tokens, applied by the in-sandbox
    # judge shim (ZED_JUDGE_MAX_TOKENS). Set for Baseten judges so the runtime
    # judge matches the offline calibration (8192); left None for the
    # opus/leaderboard path so it stays leaderboard-faithful at the verifier's
    # hardcoded 2048.
    max_tokens: int | None = None


def judge_verifier_args(judge: JudgeConfig, judge_model: str) -> list[str]:
    args = [
        "--verifier-import-path",
        JUDGE_PROXY_VERIFIER_IMPORT_PATH,
        "--ve",
        f"EVAL_MODEL={judge_model}",
        "--ve",
        f"EVAL_BASE_URL={EVAL_BASE_URL_IN_SANDBOX}",
        "--ve",
        "EVAL_API_KEY=unused-by-agent-evals-proxy",
        "--ve",
        f"ZED_JUDGE_UPSTREAM={judge.upstream}",
        "--ve",
        f"ZED_JUDGE_AUTH_ENV={judge.auth_env}",
    ]
    if judge.max_tokens is not None:
        args.extend(["--ve", f"ZED_JUDGE_MAX_TOKENS={judge.max_tokens}"])
    return args


def redacted_command(command: list[str]) -> str:
    return shlex.join(
        "EVAL_API_KEY=<redacted>" if argument.startswith("EVAL_API_KEY=") else argument
        for argument in command
    )


JUDGES: dict[str, JudgeConfig] = {
    "leaderboard": JudgeConfig(
        model="claude-opus-4-5-20251101",
        upstream="https://api.anthropic.com/v1",
        auth_env="ANTHROPIC_API_KEY",
    ),
    "opus-leaderboard": JudgeConfig(
        model="claude-opus-4-5-20251101",
        upstream="https://api.anthropic.com/v1",
        auth_env="ANTHROPIC_API_KEY",
    ),
    "deepseek-v4-pro": JudgeConfig(
        model="deepseek-ai/DeepSeek-V4-Pro",
        upstream=BASETEN_API_URL,
        auth_env="BASETEN_API_KEY",
        max_tokens=8192,
    ),
    "deepseek": JudgeConfig(
        model="deepseek-ai/DeepSeek-V4-Pro",
        upstream=BASETEN_API_URL,
        auth_env="BASETEN_API_KEY",
        max_tokens=8192,
    ),
    "kimi-k2.7-code": JudgeConfig(
        model="moonshotai/Kimi-K2.7-Code",
        upstream=BASETEN_API_URL,
        auth_env="BASETEN_API_KEY",
        max_tokens=8192,
    ),
    "kimi": JudgeConfig(
        model="moonshotai/Kimi-K2.7-Code",
        upstream=BASETEN_API_URL,
        auth_env="BASETEN_API_KEY",
        max_tokens=8192,
    ),
    "gpt55": JudgeConfig(
        model="gpt-5.5-2026-04-23",
        upstream="https://api.openai.com/v1",
        auth_env="OPENAI_API_KEY",
    ),
}


MODEL_PRESETS: dict[str, str] = {
    "sonnet-4.6": DEFAULT_MODEL,
    "claude-sonnet-4.6": DEFAULT_MODEL,
    "sonnet-4.6-latest": "anthropic/claude-sonnet-4-6-latest",
    "opus-4.5": "anthropic/claude-opus-4-5",
    "baseten:kimi-k2.7-code": "baseten/moonshotai/Kimi-K2.7-Code",
    "baseten:kimi": "baseten/moonshotai/Kimi-K2.7-Code",
    "baseten:deepseek-v4-pro": "baseten/deepseek-ai/DeepSeek-V4-Pro",
    "baseten:deepseek": "baseten/deepseek-ai/DeepSeek-V4-Pro",
}


def orchestration_info() -> dict[str, str]:
    return {
        "controller_image_recipe_version": CONTROLLER_IMAGE_RECIPE_VERSION,
        "harbor_version": HARBOR_VERSION,
        "modal_client_version": MODAL_VERSION,
    }


def get_judge(name: str) -> JudgeConfig:
    try:
        return JUDGES[name]
    except KeyError as error:
        valid = ", ".join(sorted(JUDGES))
        raise ValueError(f"unknown judge preset '{name}' (valid: {valid})") from error


def canonical_part(part: str) -> str:
    normalized = part.strip().lower()
    if normalized in ("qna", "rf", "tw"):
        return normalized
    valid = ", ".join(("qna", "rf", "tw"))
    raise ValueError(f"unknown SWE-Atlas part '{part}' (valid: {valid})")


def resolve_model_preset(model: str) -> str:
    return MODEL_PRESETS.get(model, model)
