"""Build the harness command (Harbor or Pier) for a benchmark run request.

The command is driven by the self-describing `benchmark` block embedded in a run
request (see `benchmarks.benchmark_metadata`), so it works for any registered
benchmark without a separate experiment registry lookup.
"""

from __future__ import annotations

import json
from typing import Any

from . import benchmarks, config


def _benchmark_block(run_request: dict[str, Any]) -> dict[str, Any]:
    block = run_request.get("benchmark")
    if not isinstance(block, dict):
        raise ValueError("run request is missing a 'benchmark' block")
    return block


def dataset_args(benchmark: dict[str, Any]) -> list[str]:
    dataset = benchmark.get("dataset") or {}
    kind = dataset.get("kind")
    if kind == benchmarks.DATASET_REGISTRY:
        name = dataset.get("name")
        if not name:
            raise ValueError("registry dataset requires a name")
        return ["-d", name]
    if kind in (benchmarks.DATASET_PATH, benchmarks.DATASET_PIER_PATH):
        data_dir = dataset.get("data_dir")
        if not data_dir:
            raise ValueError("path dataset requires data_dir")
        return ["-p", dataset_path(benchmark)]
    raise ValueError(f"unsupported dataset kind: {kind}")


def dataset_clone_dir(benchmark: dict[str, Any]) -> str:
    """Where the controller clones the dataset repo for path datasets."""
    return f"/tmp/datasets/{benchmark['id']}"


def dataset_path(benchmark: dict[str, Any]) -> str:
    """Repo-relative task directory inside the cloned dataset repo."""
    dataset = benchmark.get("dataset") or {}
    return f"{dataset_clone_dir(benchmark)}/{dataset.get('data_dir')}"


def harness_binary(benchmark: dict[str, Any]) -> str:
    harness = benchmark.get("harness")
    if harness not in (benchmarks.HARNESS_HARBOR, benchmarks.HARNESS_PIER):
        raise ValueError(f"unsupported harness: {harness}")
    return harness


def eval_cli_timeout(run_request: dict[str, Any], benchmark: dict[str, Any]) -> int:
    return int(
        run_request.get("eval_cli_timeout")
        or benchmark.get("default_timeout_secs")
        or config.DEFAULT_SANDBOX_TIMEOUT_SECS
    )


def build_harness_command(run_request: dict[str, Any], jobs_dir: str) -> list[str]:
    benchmark = _benchmark_block(run_request)
    build_id = run_request.get("build_id")
    if not build_id:
        raise ValueError("zed benchmarks require build_id")
    volume_name = run_request["volume_name"]
    api_secret_name = run_request["api_secret_name"]
    run_id = run_request["run_id"]
    agent_model = run_request.get("agent_model") or config.DEFAULT_MODEL
    n_concurrent = int(run_request.get("n_concurrent") or config.DEFAULT_N_CONCURRENT)
    sandbox_timeout_secs = int(
        run_request.get("sandbox_timeout_secs") or config.DEFAULT_SANDBOX_TIMEOUT_SECS
    )
    sandbox_idle_timeout_secs = int(
        run_request.get("sandbox_idle_timeout_secs")
        or config.DEFAULT_SANDBOX_IDLE_TIMEOUT_SECS
    )
    override_cpus = run_request.get("override_cpus") or config.DEFAULT_OVERRIDE_CPUS
    override_memory_mb = (
        run_request.get("override_memory_mb") or config.DEFAULT_OVERRIDE_MEMORY_MB
    )

    secret_names = [api_secret_name]
    for extra_secret_name in run_request.get("extra_api_secrets") or []:
        if extra_secret_name not in secret_names:
            secret_names.append(extra_secret_name)

    command = [
        harness_binary(benchmark),
        "run",
        *dataset_args(benchmark),
        "-m",
        agent_model,
        "--env",
        "modal",
        "--n-concurrent",
        str(n_concurrent),
        "--job-name",
        run_id,
        "--jobs-dir",
        jobs_dir,
        "--ek",
        f"volumes={json.dumps({'/data': volume_name})}",
        "--ek",
        f"secrets={json.dumps(secret_names)}",
        "--ek",
        f"sandbox_timeout_secs={sandbox_timeout_secs}",
        "--ek",
        f"sandbox_idle_timeout_secs={sandbox_idle_timeout_secs}",
        # Harbor and Pier have different installed-agent interfaces, so each gets
        # its own thin agent shell (the sanctioned cross-framework exception).
        "--agent-import-path",
        (
            "zed_eval.pier_agent:ZedPierAgent"
            if harness_binary(benchmark) == benchmarks.HARNESS_PIER
            else "zed_eval.agent:ZedAgent"
        ),
        "--ae",
        f"EVAL_CLI_CONTAINER_PATH=/data/builds/{build_id}/eval-cli",
        "--ae",
        f"EVAL_CLI_TIMEOUT={eval_cli_timeout(run_request, benchmark)}",
    ]
    # Omit --override-cpus/--override-memory-mb unless explicitly set, so Harbor
    # applies each task's declared cpus/memory. Overriding below the declared
    # values OOM-kills memory-heavy tasks (SIGKILL / exit 137).
    if override_cpus is not None:
        command += ["--override-cpus", str(int(override_cpus))]
    if override_memory_mb is not None:
        command += ["--override-memory-mb", str(int(override_memory_mb))]

    openai_compatible_provider_json = run_request.get("openai_compatible_provider_json")
    if openai_compatible_provider_json:
        command.extend(
            [
                "--ae",
                f"ZED_OPENAI_COMPATIBLE_PROVIDERS={openai_compatible_provider_json}",
            ]
        )
    anthropic_available_models_json = run_request.get("anthropic_available_models_json")
    if anthropic_available_models_json:
        command.extend(
            [
                "--ae",
                f"ZED_ANTHROPIC_AVAILABLE_MODELS={anthropic_available_models_json}",
            ]
        )

    # NOTE: Pier's network allowlist for air-gapped DeepSWE tasks is declared by
    # the agent (`network_allowlist()` on a Pier-native agent class), not via a
    # CLI flag — see benchmarks.AGENT_API_HOSTS and the DeepSWE agent. There is
    # no `--agent-allow-host` option on `pier run`.
    if benchmark.get("needs_judge"):
        judge = config.get_judge(run_request["judge_preset"])
        judge_model = run_request.get("judge_model") or judge.model
        command.extend(config.judge_verifier_args(judge, judge_model))

    for key, value in (benchmark.get("env") or {}).items():
        command.extend(["--ae", f"{key}={value}"])
    for key, value in (run_request.get("extra_env") or {}).items():
        command.extend(["--ae", f"{key}={value}"])

    for task_name in run_request.get("task_names") or []:
        command.extend(["--include-task-name", task_name])

    n_tasks = run_request.get("n_tasks")
    if n_tasks is not None:
        command.extend(["--n-tasks", str(n_tasks)])

    command.extend(run_request.get("extra_harbor_args") or [])
    return command


def run_metadata(run_request: dict[str, Any]) -> dict[str, Any]:
    benchmark = _benchmark_block(run_request)
    metadata: dict[str, Any] = {
        "run_id": run_request["run_id"],
        "namespace": run_request["namespace"],
        "benchmark": benchmark,
        "suite_id": run_request.get("suite_id"),
        "harness": benchmark.get("harness"),
        "scoring": benchmark.get("scoring"),
        "build_id": run_request.get("build_id"),
        "agent_model": run_request.get("agent_model") or config.DEFAULT_MODEL,
        "orchestration": run_request.get("orchestration")
        or config.orchestration_info(),
        "build_toolchain": run_request.get("build_toolchain"),
        "volume_name": run_request["volume_name"],
        "api_secret_name": run_request["api_secret_name"],
        "extra_api_secrets": run_request.get("extra_api_secrets") or [],
        "openai_compatible_provider_json": run_request.get(
            "openai_compatible_provider_json"
        ),
        "anthropic_available_models_json": run_request.get(
            "anthropic_available_models_json"
        ),
        "n_concurrent": run_request.get("n_concurrent") or config.DEFAULT_N_CONCURRENT,
        "override_cpus": run_request.get("override_cpus")
        or config.DEFAULT_OVERRIDE_CPUS,
        "override_memory_mb": run_request.get("override_memory_mb")
        or config.DEFAULT_OVERRIDE_MEMORY_MB,
        "eval_cli_timeout": eval_cli_timeout(run_request, benchmark),
        "sandbox_timeout_secs": run_request.get("sandbox_timeout_secs")
        or config.DEFAULT_SANDBOX_TIMEOUT_SECS,
        "sandbox_idle_timeout_secs": run_request.get("sandbox_idle_timeout_secs")
        or config.DEFAULT_SANDBOX_IDLE_TIMEOUT_SECS,
        "task_count": len(run_request.get("task_names") or []),
        "n_tasks": run_request.get("n_tasks"),
    }
    if benchmark.get("needs_judge"):
        judge = config.get_judge(run_request["judge_preset"])
        metadata["judge_preset"] = run_request["judge_preset"]
        metadata["judge_model"] = run_request.get("judge_model") or judge.model
        metadata["judge_upstream"] = judge.upstream
        metadata["judge_auth_env"] = judge.auth_env
        metadata["judge_eval_base_url"] = config.EVAL_BASE_URL_IN_SANDBOX
    return metadata
